//! 🚦️ Edition-specific catalogue cases checked against independently verified reference fixtures.

use semio_s_artifact_norm_en1991_checks::catalogue::{Case, En1991Cases, case_input, evaluate_design_case, evaluate_outcomes};
use semio_s_plugin_norm_compliance::{AnnexChoice, design_case};

#[test]
fn rail_aerodynamic_factors_match_the_published_surface_rules() {
    let fixture = include_str!("../../⚖️checks/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/rail-aerodynamics.tsv");
    for row in fixture.lines().skip(1) {
        let [entry, inputs, result, expected] = row.split('\t').collect::<Vec<_>>().try_into().unwrap();
        let mut case = Case::new("rail-aerodynamic", AnnexChoice::En);
        for input in inputs.split(',') {
            let (key, value) = input.split_once('=').unwrap();
            case = match value.parse::<f64>() {
                Ok(number) => case.with_n(key, number),
                Err(_) => case.with_s(key, value),
            };
        }
        let outcomes = evaluate_outcomes(entry, &case);
        if expected == "-" {
            assert!(outcomes.is_err(), "{row}");
            continue;
        }
        let outcomes = outcomes.unwrap();
        let calculated = outcomes.iter().find(|outcome| outcome.key == result).unwrap();
        let expected: f64 = expected.parse().unwrap();
        assert!((calculated.value - expected).abs() <= 1e-10, "{row}");
    }
}

#[test]
fn rail_fatigue_annex_d_cases_match_the_reference_fixture() {
    use semio_s_plugin_norm_compliance::CheckStatus;

    for row in include_str!("../../⚖️checks/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/🚆️.tsv").lines().skip(1) {
        let [entry, inputs, key, expected, capacity] =
            row.split('\t').collect::<Vec<_>>().try_into().unwrap();
        let metadata = design_case::entry::<En1991Cases>(entry).unwrap();
        let mut case = design_case::seed::<En1991Cases>(&metadata, "rail-fatigue");
        for input in inputs.split(',') {
            let (key, value) = input.split_once('=').unwrap();
            case.values = match value.parse::<f64>() {
                Ok(number) => case.values.with_number(key, number),
                Err(_) => case.values.with_text(key, value),
            };
        }

        let outcomes = evaluate_outcomes(entry, &case_input(AnnexChoice::En, &case));
        if key == "invalid" {
            assert!(outcomes.is_err(), "{row}");
            continue;
        }
        let outcomes = outcomes.unwrap();
        let outcome = outcomes.iter().find(|outcome| outcome.key == key).unwrap();
        let expected: f64 = expected.parse().unwrap();
        assert!((outcome.value - expected).abs() <= 1e-11 * expected.abs().max(1.0), "{row}");
        if capacity != "-" {
            let capacity: f64 = capacity.parse().unwrap();
            assert_eq!(outcome.capacity, capacity, "{row}");
            let results = evaluate_design_case(&case, AnnexChoice::En);
            let check = results.iter().find(|result| result.id.contains(".stress_range.")).unwrap();
            let expected_status = if expected <= capacity { CheckStatus::Pass } else { CheckStatus::Fail };
            assert_eq!(check.status, expected_status, "{row}");
        }
    }
}

#[test]
fn high_speed_load_model_a_matches_the_ten_normative_train_types() {
    for row in include_str!("../../⚖️checks/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/hslm-a.tsv").lines().skip(1) {
        let mut fields = row.split('\t');
        let id = fields.next().unwrap();
        let case = Case::new("high-speed-train", AnnexChoice::En).with_s("rail_hslm_a_train", id);
        let results = evaluate_outcomes("E2.22", &case).unwrap();
        let quantity = |key| results.iter().find(|result| result.key == key).unwrap().value;
        let [coaches, coach, bogie, force_kn, count, rear] = fields
            .map(|column| column.parse::<f64>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
        assert_eq!(quantity("intermediate_coaches"), coaches, "{id}");
        assert_eq!(quantity("coach_length"), coach, "{id}");
        assert_eq!(quantity("bogie_spacing"), bogie, "{id}");
        assert_eq!(quantity("axle_force"), force_kn * 1000.0, "{id}");
        let axles: Vec<_> = results.iter().filter(|result| result.key.starts_with("axle_at_"))
            .map(|result| result.value).collect();
        assert_eq!(axles.len(), count as usize, "{id}");
        assert_eq!(&axles[..6], &[0.0, 3.0, 14.0, 17.0, 20.525, 20.525 + bogie], "{id}");
        assert_eq!(*axles.last().unwrap(), rear, "{id}");
        assert!((axles[6] - (18.7625 + coach - bogie / 2.0)).abs() < 1e-12, "{id}");
        assert!(axles.windows(2).all(|pair| pair[0] < pair[1]), "{id}");
        assert!(axles.iter().zip(axles.iter().rev()).all(|(front, back)| (front + back - rear).abs() < 1e-10), "{id}");
    }
}

#[test]
fn high_speed_load_model_b_matches_the_published_span_labels() {
    for row in include_str!("../../⚖️checks/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/hslm-b.tsv").lines().skip(1) {
        let [span, given_count, given_spacing, count, spacing, rear] =
            row.split('\t').collect::<Vec<_>>().try_into().unwrap();
        let mut case = Case::new("uniform-train", AnnexChoice::En).with_n("rail_hslm_b_span", span.parse().unwrap());
        for (key, value) in [("rail_hslm_b_count", given_count), ("rail_hslm_b_spacing", given_spacing)] {
            if value != "-" {
                case = case.with_n(key, value.parse().unwrap());
            }
        }
        let results = evaluate_outcomes("E2.22.1", &case);
        if count == "-" {
            assert!(results.is_err(), "{row}");
            continue;
        }
        let results = results.unwrap();
        let quantity = |key| results.iter().find(|result| result.key == key).unwrap().value;
        let count: usize = count.parse().unwrap();
        assert_eq!(quantity("axle_count"), count as f64, "{row}");
        assert_eq!(quantity("axle_spacing"), spacing.parse::<f64>().unwrap(), "{row}");
        assert_eq!(quantity("axle_force"), 170_000.0, "{row}");
        assert_eq!(quantity(&format!("axle_at_{count}")), rear.parse::<f64>().unwrap(), "{row}");
    }
}

#[test]
fn the_load_model_selection_matches_table_6_4() {
    for row in include_str!("../../⚖️checks/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/hslm-selection.tsv").lines().skip(1) {
        let [span, structure, a, a_eligible, b] = row.split('\t').collect::<Vec<_>>().try_into().unwrap();
        let case = Case::new("high-speed-model", AnnexChoice::En)
            .with_n("span", span.parse().unwrap()).with_s("rail_hslm_structure", structure);
        let results = evaluate_outcomes("E2.22.2", &case).unwrap();
        for (key, expected) in [("a_all", a), ("a_one_eligible", a_eligible), ("b", b)] {
            let outcome = results.iter().find(|result| result.key == key).unwrap();
            assert_eq!(outcome.value, expected.parse::<f64>().unwrap(), "{row}");
        }
    }
}

#[test]
fn crane_classification_matches_all_table_b_1_rows() {
    for row in include_str!("../../⚖️checks/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/crane-classes.tsv").lines().skip(1) {
        let [item, hc_min, hc_max, s_min, s_max] = row.split('\t').collect::<Vec<_>>().try_into().unwrap();
        let case = Case::new("crane", AnnexChoice::En).with_s("crane_duty", item);
        let outcomes = evaluate_outcomes("E3.06", &case).unwrap();
        for (key, expected) in [("hc_min", hc_min), ("hc_max", hc_max), ("s_min", s_min), ("s_max", s_max)] {
            let outcome = outcomes.iter().find(|outcome| outcome.key == key).unwrap();
            assert_eq!(outcome.value, expected.parse::<f64>().unwrap(), "{row}");
        }
    }
    let unknown = Case::new("crane", AnnexChoice::En).with_s("crane_duty", "29");
    assert!(evaluate_outcomes("E3.06", &unknown).is_err());
}

#[test]
fn silo_wall_cooling_pressure_matches_equation_5_101() {
    for row in include_str!("../../⚖️checks/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/silo-thermal.tsv").lines().skip(1) {
        let [source, delta, nu, measured, weight, base_stress, esu, ct, pressure] =
            row.split('\t').collect::<Vec<_>>().try_into().unwrap();
        let mut case = Case::new("silo", AnnexChoice::En)
            .with_s("silo_modulus_source", source)
            .with_n("dc", 8.0)
            .with_n("silo_wall_thickness", 0.008)
            .with_n("silo_wall_expansion", 12.0)
            .with_n("silo_wall_modulus", 200_000_000_000.0);
        for (key, value) in [
            ("silo_delta_cooling", delta), ("silo_solid_poisson", nu),
            ("silo_unloading_modulus", measured), ("silo_solid_weight", weight),
            ("silo_base_filling_stress", base_stress),
        ] {
            if value != "-" {
                case = case.with_n(key, value.parse().unwrap());
            }
        }
        let result = evaluate_outcomes("E4.06", &case);
        if pressure == "-" {
            assert!(result.is_err(), "{row}");
            continue;
        }
        let outcomes = result.unwrap();
        for (key, expected) in [("unloading_modulus", esu), ("temperature_multiplier", ct), ("wall_pressure", pressure)] {
            let outcome = outcomes.iter().find(|outcome| outcome.key == key).unwrap();
            let expected: f64 = expected.parse().unwrap();
            assert!((outcome.value - expected).abs() <= 1e-11 * expected.abs().max(1.0), "{row}");
        }
    }
}

#[test]
fn tank_pressure_cases_match_annex_b_actions() {
    for row in include_str!("../../⚖️checks/📇️catalogue/🗂️cases/🧪️tests/🔬️unit/tank-actions.tsv").lines().skip(1) {
        let [entry, situation, first, second, third, expected] = row.split('\t').collect::<Vec<_>>().try_into().unwrap();
        let number = |value: &str| value.parse::<f64>().unwrap();
        let case = Case::new("tank", AnnexChoice::En);
        let (case, key) = if entry == "E4.09" {
            (case.with_s("tank_wind_enclosure", situation).with_n("tank_wind_qe", number(first))
                .with_n("tank_wind_qi", number(second)).with_n("tank_wind_cpe", number(third)), "net_wind")
        } else {
            (case.with_s("tank_pressure_situation", situation).with_n("tank_pressure_min", number(first))
                .with_n("tank_pressure_max", number(second)), "pressure_range")
        };
        let outcomes = evaluate_outcomes(entry, &case);
        if expected == "-" {
            assert!(outcomes.is_err(), "{row}");
        } else {
            let outcomes = outcomes.unwrap();
            let outcome = outcomes.iter().find(|outcome| outcome.key == key).unwrap();
            assert!((outcome.value - number(expected)).abs() <= 1e-10, "{row}");
        }
    }
}
