//! 🏷️ V-028, V-030 to V-033, V-063 and V-120 against the Python oracle `🧪️tests/🏷️section-class/🧮️reference/🐍️.py` (closed forms from the standard, area from `sectionproperties`).

use crate::clauses::classification::*;
use crate::clauses::resistance::{av_rolled_i, eta_shear, v_pl_rd};
use crate::clauses::stability::{interaction_annex_a, AnnexAInput, InteractionInput};

fn close(a: f64, b: f64, rel: f64, what: &str) {
    assert!((a - b).abs() <= rel * b.abs().max(1e-12), "{what}: rust {a} vs oracle {b}");
}

fn oracle() -> serde_json::Value {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let script = loop {
        let candidate = dir.join("🧪️tests/🏷️section-class/🧮️reference/🐍️.py");
        if candidate.is_file() {
            break candidate;
        }
        assert!(dir.pop(), "oracle script not found above the manifest dir");
    };
    let out = semio_s_plugin_norm_compliance::python_oracle::command().arg(&script).output().expect("spawn oracle");
    assert!(out.status.success(), "oracle failed: {}", String::from_utf8_lossy(&out.stderr));
    let text = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(&text[text.find('{').expect("json")..=text.rfind('}').expect("json end")]).expect("parse oracle")
}

fn class_of(o: &serde_json::Value, key: &str) -> u8 {
    o[key].as_u64().unwrap_or_else(|| panic!("oracle key {key}")) as u8
}

#[test]
fn v028_v030_v031_v033_table_5_2_classes_match_the_oracle() {
    let o = oracle();
    let eps = epsilon(355.0);
    close(eps, o["epsilon_355"].as_f64().unwrap(), 1e-12, "epsilon");
    let g = Geometry { h: 300.0, b: 150.0, tw: 7.1, tf: 10.7, r: 15.0, weld_throat: 0.0 };
    let c = classify_i_section(&g, 355.0, 2.0e6, 1.0, true);
    close(c.c_web / g.tw, o["web_ct"].as_f64().unwrap(), 1e-12, "web c/t");
    close(c.c_flange / g.tf, o["flange_ct"].as_f64().unwrap(), 1e-12, "flange c/t");
    assert_eq!(internal_part_class(c.c_web / g.tw, eps, 1.0, 1.0), class_of(&o, "class_web_compression"));
    assert_eq!(internal_part_class(c.c_web / g.tw, eps, -1.0, 0.5), class_of(&o, "class_web_bending"));
    assert_eq!(outstand_class(c.c_flange / g.tf, eps, 1.0, 1.0, true), class_of(&o, "class_flange"));
    assert_eq!(outstand_class(10.0, eps, 1.0, 1.0, true), class_of(&o, "class_plate_outstand_10"));
    assert_eq!(outstand_class(12.0, eps, 1.0, 1.0, true), class_of(&o, "class_plate_outstand_12"));
    assert_eq!((c.flange, c.web, c.section), (class_of(&o, "class_flange"), class_of(&o, "class_web_compression"), class_of(&o, "class_section_compression")));
    assert_eq!(c.section, c.flange.max(c.web));
}

#[test]
fn v032_circular_and_angle_classes_match_the_oracle() {
    let o = oracle();
    let eps = epsilon(355.0);
    assert_eq!(chs_class(139.7 / 5.0, eps), class_of(&o, "class_chs_139_5"));
    assert_eq!(chs_class(273.0 / 6.3, eps), class_of(&o, "class_chs_273_63"));
    assert_eq!(chs_class(508.0 / 5.0, eps), class_of(&o, "class_chs_508_5"));
    assert_eq!(angle_class(100.0, 100.0, 10.0, 1.0), class_of(&o, "class_angle_s235"));
    assert_eq!(angle_class(100.0, 100.0, 10.0, eps), class_of(&o, "class_angle_s355"));
}

#[test]
fn v063_plastic_shear_resistance_matches_the_oracle() {
    let o = oracle();
    let hw = 300.0 - 2.0 * 10.7;
    let av = av_rolled_i(5381.0, 150.0, 10.7, 7.1, 15.0, hw, eta_shear(355.0));
    close(av, o["a_v"].as_f64().unwrap(), 5e-4, "A_v");
    close(v_pl_rd(av, 355.0, 1.0), o["v_pl_rd"].as_f64().unwrap(), 5e-4, "V_pl,Rd");
}

#[test]
fn v120_annex_a_interaction_factors_match_the_oracle() {
    let a = oracle()["annex_a"].clone();
    let f = |key: &str| a[key].as_f64().unwrap_or_else(|| panic!("oracle key {key}"));
    let (area, fy, wpl_y, wpl_z) = (5381.0, 355.0, 628.4e3, 125.2e3);
    let base = InteractionInput { n_ed: 400e3, my_ed: 100e6, mz_ed: 0.0, n_rk: area * fy, my_rk: wpl_y * fy, mz_rk: wpl_z * fy, chi_y: f("chi_y"), chi_z: f("chi_z"), chi_lt: 1.0, lambda_y: f("lambda_y"), lambda_z: f("lambda_z"), cmy: 0.9, cmz: 0.9, cm_lt: 1.0, class: 1, susceptible_torsion: false, rhs: false, gm1: 1.0, delta_my: 0.0, delta_mz: 0.0 };
    let input = AnnexAInput { base, ncr_y: f("ncr_y"), ncr_z: f("ncr_z"), ncr_t: 1.0e15, ncr_tf: 1.0e15, wel_y: 557e3, wpl_y, wel_z: 80.5e3, wpl_z, mpl_y_rd: wpl_y * fy, mpl_z_rd: wpl_z * fy, it: 20.12e4, iy: 8356e4, a_area: area, c1: 1.0, lambda_0: 0.0, cmy0: 0.9, cmz0: 0.9 };
    let r = interaction_annex_a(&input);
    for (name, value) in [("kyy", r.kyy), ("kyz", r.kyz), ("kzy", r.kzy), ("kzz", r.kzz), ("eta_61", r.eta_61), ("eta_62", r.eta_62)] {
        close(value, f(name), 1e-9, name);
    }
}
