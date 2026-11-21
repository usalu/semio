use super::*;

#[test]
fn gmodg_pressure_test_fixtures() {
    for row in include_str!("pressure-tests.tsv").lines().skip(1) {
        let cells: Vec<_> = row.split('\t').collect();
        let bool_at = |index: usize| cells[index] == "1";
        let number = |index: usize| cells[index].parse::<f64>().unwrap();
        let basis = match cells[7] {
            "volume" => AirflowBasis::Volume,
            "envelope_area" => AirflowBasis::EnvelopeArea { envelope_area_m2: number(8) },
            value => panic!("unknown basis {value}"),
        };
        let scope = match cells[9] {
            "whole_building" => TestScope::WholeBuilding,
            "non_residential_zone" => TestScope::NonResidentialZone {
                non_residential: bool_at(10),
                only_selected_zones: bool_at(11),
                different_requirements: bool_at(12),
            },
            "exterior_unit_sample" => TestScope::ExteriorUnitSample {
                identical_units: bool_at(13),
                exterior_access_only: bool_at(14),
                annex_nb_confirmed: bool_at(15),
            },
            value => panic!("unknown scope {value}"),
        };
        let test = PressureTest {
            before_completion: bool_at(1), annex_na_confirmed: bool_at(2), conditioned_volume_m3: number(3),
            mechanical_ventilation: bool_at(4), pressurisation_m3_h: number(5), depressurisation_m3_h: number(6), basis, scope,
        };
        match evaluate_pressure_test(test) {
            Ok(outcome) => {
                assert_eq!(cells[16], "ok", "{}", cells[0]);
                assert_eq!(outcome.limit_m3_h, number(17), "{}", cells[0]);
                assert_eq!(outcome.pressurisation_passes, bool_at(18), "{}", cells[0]);
                assert_eq!(outcome.depressurisation_passes, bool_at(19), "{}", cells[0]);
                assert_eq!(outcome.eligible(), bool_at(18) && bool_at(19), "{}", cells[0]);
            }
            Err(gap) => assert_eq!(format!("{gap:?}"), cells[16], "{}", cells[0]),
        }
    }
}
