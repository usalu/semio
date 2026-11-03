//! 📐️ Language-agnostic worked-example vectors of the EN 1999 compliance tests: one JSON file, evaluated by the Rust clause functions here and by the independent Python oracle. See [`super`].

use super::*;
use semio_s_artifact_norm_en1999_checks::{joints, part_1_4_1_5};

fn vectors_path() -> PathBuf {
    family_any_dir().join("🔮️oracles").join("⚖️compliance").join("📐️worked-examples").join("🔣️.json")
}

fn num(v: &serde_json::Value) -> f64 { v.as_f64().expect("number") }

fn evaluate_rust(name: &str, a: &[serde_json::Value]) -> f64 {
    match name {
        "rho_c" => part_1_1::rho_c(num(&a[0]), a[1].as_bool().unwrap(), a[2].as_bool().unwrap(), a[3].as_bool().unwrap()),
        "n_cr" => part_1_1::n_cr(na_de::E_PA, num(&a[0]), num(&a[1])),
        "chi" => part_1_1::chi_from_lambda(num(&a[0]), num(&a[1]), num(&a[2])),
        "m_cr" => part_1_1::m_cr_ltb(num(&a[0]), na_de::E_PA, num(&a[1]), num(&a[2]), num(&a[3]), num(&a[4])),
        "bolt_shear" => joints::shear_resistance(a[0].as_str().unwrap(), num(&a[1]), num(&a[2])),
        "bolt_bearing" => joints::bearing_resistance(num(&a[0]), num(&a[1]), num(&a[2]), num(&a[3]), num(&a[4]), num(&a[5]), num(&a[6]), num(&a[7])),
        "bolt_tension" => joints::tension_resistance(a[0].as_str().unwrap(), num(&a[1]), num(&a[2])),
        "delta_sigma_d" => part_1_3::delta_sigma_d(num(&a[0]), num(&a[1])),
        "delta_sigma_l" => part_1_3::delta_sigma_l(num(&a[0]), num(&a[1]), num(&a[2])),
        "cycles" => part_1_3::cycles_to_failure(num(&a[0]), num(&a[1]), num(&a[2]), num(&a[3])),
        "damage" => part_1_3::damage(num(&a[0]), num(&a[1]), num(&a[2]), num(&a[3]), num(&a[4]), num(&a[5]), num(&a[6])),
        "fire_step" => part_1_2::unprotected_temperature_c(num(&a[0]), num(&a[1]), num(&a[2])) - num(&a[0]),
        "sigma_x_rcr" => part_1_4_1_5::sigma_x_rcr(num(&a[0]), num(&a[1]), num(&a[2])),
        "haz_width" => part_1_1::haz_width(num(&a[0])),
        "shear_rho" => part_1_1::shear_buckling_rho(num(&a[0]), num(&a[1]), num(&a[2]), num(&a[3])),
        "haz_width_for" => {
            let mut el = semio_s_artifact_norm_en1999_model::PlateElement { id: "e".into(), width: 0.1, thickness: num(&a[0]), outstand: true, welded: true, weld_position: 0.0, weld_process: a[1].as_str().unwrap().into(), weld_t_junction: a[2].as_bool().unwrap(), weld_direction: semio_s_artifact_norm_en1999_model::WeldDirection::Longitudinal, edge_stiffener_depth: 0.0, edge_stiffener_thickness: 0.0 };
            el.welded = true;
            part_1_1::haz_width_for(&el)
        }
        "patch_rd" => part_1_1::patch_load_resistance(num(&a[0]), num(&a[1]), num(&a[2]), num(&a[3]), num(&a[4]), num(&a[5]), num(&a[6]), 1.1),
        "stress_area" => joints::stress_area(num(&a[0])),
        other => panic!("unknown vector function {other}"),
    }
}

#[test]
fn rust_matches_every_worked_example_vector() {
    let doc: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(vectors_path()).expect("vectors")).expect("json");
    let vectors = doc["vectors"].as_array().expect("vectors array");
    assert!(vectors.len() >= 19);
    for v in vectors {
        let got = evaluate_rust(v["fn"].as_str().unwrap(), v["args"].as_array().unwrap());
        let expected = num(&v["expected"]);
        assert!((got - expected).abs() <= num(&v["relTol"]) * expected.abs().max(1e-12), "{}: expected {expected} got {got}", v["id"]);
    }
}

#[test]
fn python_oracle_matches_every_worked_example_vector() {
    let output = semio_s_plugin_norm_compliance::python_oracle::run(oracle_script(), &[std::ffi::OsStr::new("--vectors"), vectors_path().as_os_str()], None);
    assert!(output.status.success(), "oracle vectors failed: {}", String::from_utf8_lossy(&output.stdout));
}
