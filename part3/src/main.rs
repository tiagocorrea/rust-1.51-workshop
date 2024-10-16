enum CitySize {
    Town,       // approximate residents: 1_000
    City,       // approximate residents: 10_000
    Metropolis, // approximate residents: 1_000_000
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

impl City {
    fn new(city_size: CitySize, is_coastal: bool) -> City {
        let (description, residents) = match city_size {
            CitySize::Town => {
                let residents = 1_000;

                (
                    format!("a *town* of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::City => {
                let residents = 10_000;

                (
                    format!("a *city* of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::Metropolis => {
                let residents = 1_000_000;
                (
                    format!("a *metropolis* of approximately {} residents", residents),
                    residents,
                )
            }
        };

        City {
            description,
            residents,
            is_coastal,
        }
    }
}

fn main() {
    for city_size in [CitySize::Town, CitySize::City, CitySize::Metropolis] {
        let city = City::new(city_size, true);
        println!("This city is {}", city.description);

        if city.residents > 100_000 {
            println!("Wow!");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{City, CitySize};

    #[test]
    fn test_city_new() {
        let cases = [
            (
                CitySize::Town,
                true,
                "a *town* of approximately 1000 residents",
            ),
            (
                CitySize::City,
                false,
                "a *city* of approximately 10000 residents",
            ),
            (
                CitySize::Metropolis,
                true,
                "a *metropolis* of approximately 1000000 residents",
            ),
        ];
        for (city_size, is_coastal, description) in cases {
            let city = City::new(city_size, is_coastal);

            assert_eq!(city.description, description);
            assert_eq!(city.is_coastal, is_coastal);
        }
    }
}
