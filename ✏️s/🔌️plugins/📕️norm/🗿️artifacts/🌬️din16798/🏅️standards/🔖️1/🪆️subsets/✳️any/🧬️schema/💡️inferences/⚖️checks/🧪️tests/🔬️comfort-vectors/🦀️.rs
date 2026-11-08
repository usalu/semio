//! 🧪️ The language-agnostic comfort vectors (`🧪️tests/🌡️indoor-comfort/🥒️.feature`): every number was written by the independent Python oracle and is reproduced here.

use super::*;
use serde_json::Value;

const VECTORS: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🌡️comfort-vectors/🔣️.json");

fn section(name: &str) -> Vec<Value> {
    serde_json::from_str::<Value>(VECTORS).expect("the committed vectors parse")[name].as_array().cloned().unwrap_or_else(|| panic!("section {name}"))
}

fn number(row: &Value, key: &str) -> f64 {
    row[key].as_f64().unwrap_or_else(|| panic!("{key} of {row}"))
}

fn category(row: &Value) -> ComfortCategory {
    [ComfortCategory::I, ComfortCategory::II, ComfortCategory::III, ComfortCategory::IV][row["category"].as_u64().expect("category") as usize - 1]
}

fn close(actual: f64, expected: f64, tolerance: f64, what: &str) {
    assert!((actual - expected).abs() <= tolerance, "{what}: {actual} against {expected}");
}

#[test]
fn the_predicted_mean_vote_and_dissatisfied_equal_the_oracle() {
    for row in section("pmv") {
        let pmv = part_1::pmv_iso7730(number(&row, "t_op"), number(&row, "rh"), number(&row, "v"), number(&row, "met"), number(&row, "clo"));
        let expected = number(&row, "pmv");
        if expected.abs() < 3.0 {
            close(pmv, expected, 1e-6, &format!("PMV of {row}"));
            close(part_1::ppd_from_pmv(pmv), number(&row, "ppd"), 1e-4, &format!("PPD of {row}"));
        }
    }
}

#[test]
fn the_predicted_mean_vote_reproduces_the_published_iso_7730_table() {
    for row in section("iso7730_table_d1") {
        close(part_1::pmv_iso7730(number(&row, "t_op"), number(&row, "rh"), number(&row, "v"), number(&row, "met"), number(&row, "clo")), number(&row, "pmv"), 0.011, &format!("{row}"));
    }
}

#[test]
fn the_adaptive_bands_equal_the_oracle() {
    for row in section("adaptive") {
        let theta = number(&row, "theta_rm");
        let centre = part_1::adaptive_comfort_temperature_c(theta);
        let half = part_1::adaptive_band_k(category(&row));
        close(centre, number(&row, "centre"), 1e-9, "comfort temperature");
        close(centre - half, number(&row, "lower"), 1e-9, "lower limit");
        close(centre + half, number(&row, "upper"), 1e-9, "upper limit");
    }
}

#[test]
fn the_ventilation_components_equal_the_oracle() {
    for row in section("ventilation") {
        let cat = category(&row);
        let pollution = parse_pollution(row["pollution"].as_str().expect("pollution"));
        let persons = number(&row, "occupants") * part_1::outdoor_air_per_person_l_s(cat) * 3.6;
        let building = number(&row, "area") * part_1::outdoor_air_per_area_l_s_m2(cat, pollution) * 3.6;
        close(persons, number(&row, "persons"), 1e-9, "persons component");
        close(building, number(&row, "building"), 1e-9, "building component");
        close(part_1::required_outdoor_air_m3_h(number(&row, "occupants") as u32, number(&row, "area"), cat, pollution), persons + building, 1e-9, "required outdoor air");
    }
}

#[test]
fn the_co2_limits_equal_the_oracle() {
    for row in section("co2") {
        let annex = annex_params::AnnexParams::for_choice(if row["annex"] == "de" { AnnexChoice::De } else { AnnexChoice::En });
        let usage = parse_usage(row["usage"].as_str().expect("usage"));
        close(part_1::co2_limit_above_outdoor_ppm(&annex, usage, category(&row), number(&row, "outdoor")), number(&row, "limit_above"), 1e-9, &format!("{row}"));
    }
}

fn oracle_vote(t_op: f64, rh: f64, v: f64, met: f64, clo: f64) -> f64 {
    let row = section("pmv").into_iter().find(|row| [("t_op", t_op), ("rh", rh), ("v", v), ("met", met), ("clo", clo)].iter().all(|(key, value)| (number(row, key) - value).abs() < 1e-12)).unwrap_or_else(|| panic!("no oracle vote for {t_op} {rh} {v} {met} {clo}"));
    number(&row, "pmv")
}

#[test]
fn the_pmv_checks_of_the_example_documents_report_the_oracle_vote() {
    use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
    for document in [Din16798Snapshot::compliant_office(), Din16798Snapshot::residential_method3(), Din16798Snapshot::noncompliant_office()] {
        let report = check_full_environment(&document);
        for zone in &document.zones {
            let limit = part_1::pmv_limit(parse_category(&zone.comfort_category));
            for (season, t_op, clo) in [("summer", zone.t_op_summer_c, zone.clothing_clo), ("winter", zone.t_op_winter_c, 1.0)] {
                let check = report.checks.iter().find(|c| c.id == format!("din16798-1.pmv.{season}.{}", zone.id)).unwrap_or_else(|| panic!("the {season} PMV check of {}", zone.id));
                close(check.utilization, oracle_vote(t_op, zone.rh_percent, zone.air_speed_m_s, zone.metabolic_rate_met, clo).abs() / limit, 1e-6, &format!("{season} PMV utilisation of {}", zone.id));
            }
        }
    }
}
