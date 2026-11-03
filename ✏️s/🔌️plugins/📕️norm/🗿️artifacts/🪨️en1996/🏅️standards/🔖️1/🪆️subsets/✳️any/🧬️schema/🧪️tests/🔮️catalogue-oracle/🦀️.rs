//! 🔮️ Catalogue oracle: the independent Python oracle (scipy, shapely, numpy, independently keyed tables) recomputes the catalogue cases and the wall utilisations; nothing touches the filesystem, the data goes through stdin and stdout.

use super::catalogue_tests::fixture_cases;
use crate::checks::masonry::{evaluate_building, evaluate_outcomes, Case};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus};
use std::ffi::OsString;

fn artifact_file(relative: &str) -> std::path::PathBuf {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        let candidate = dir.join(relative);
        if candidate.exists() {
            return candidate;
        }
        assert!(dir.pop(), "no ancestor of {} contains {relative}", env!("CARGO_MANIFEST_DIR"));
    }
}

fn oracle_script() -> std::path::PathBuf {
    artifact_file("🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/⚖️compliance/🐍️.py")
}

fn run(input: serde_json::Value) -> serde_json::Value {
    let args: Vec<OsString> = Vec::new();
    let refs: Vec<&std::ffi::OsStr> = args.iter().map(|a| a.as_os_str()).collect();
    let out = semio_s_plugin_norm_compliance::python_oracle::run_ok(oracle_script(), &refs, Some(input.to_string().as_bytes()));
    serde_json::from_slice(&out.stdout).unwrap_or_else(|e| panic!("oracle output: {e}: {}", String::from_utf8_lossy(&out.stdout)))
}

fn case_json(index: usize, entry: &str, c: &Case) -> serde_json::Value {
    serde_json::json!({
        "id": format!("{index}"),
        "entry": entry,
        "annex": if c.annex == AnnexChoice::De { "de" } else { "en" },
        "num": c.num,
        "text": c.text,
        "vec": c.vec,
    })
}

#[test]
fn python_catalogue_oracle_matches_rust() {
    let cases = fixture_cases();
    let payload = serde_json::json!({"mode": "catalogue", "cases": cases.iter().enumerate().map(|(i, (e, c))| case_json(i, e, c)).collect::<Vec<_>>()});
    let answer = run(payload);
    assert_eq!(answer["ok"], true);
    let mut compared = 0;
    for (i, (entry, case)) in cases.iter().enumerate() {
        let Some(expected) = answer["results"][format!("{i}")].as_object() else { continue };
        let outcomes = evaluate_outcomes(entry, case).expect("outcomes");
        for (key, want) in expected {
            let want = want.as_f64().expect("number");
            let o = outcomes.iter().find(|o| o.key == key).unwrap_or_else(|| panic!("{entry}: oracle key {key} missing in Rust"));
            let got = o.capacity;
            let numerical = matches!(*entry, "M-31" | "M-42");
            let tol = if numerical { 3e-3 } else { 1e-7 };
            assert!((got - want).abs() <= tol * want.abs().max(1e-12), "{entry}/{key} case {}: rust {got}, oracle {want}", case.id);
            compared += 1;
        }
    }
    assert!(compared >= 45, "only {compared} oracle comparisons");
}

#[test]
fn python_snapshot_oracle_matches_wall_utilisations() {
    for (name, doc) in [
        ("compliant", En1996Snapshot::compliant_clay_wall()),
        ("openings", En1996Snapshot::opening_wall_example()),
        ("noncompliant", En1996Snapshot::noncompliant_multi_fail()),
        ("reinforced", En1996Snapshot::reinforced_wall_example()),
    ] {
        let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.reliability_class, doc.storeys, &doc.walls);
        let answer = run(serde_json::json!({"mode": "snapshot", "snapshot": doc}));
        assert_eq!(answer["ok"], true, "{name}");
        let mut compared = 0;
        for wall in &doc.walls {
            let expected = &answer["expected"][wall.id.as_str()];
            let rust_max = |prefix: &str, exclude: &str| {
                report.checks.iter().filter(|c| c.id.starts_with(prefix) && !c.id.contains(exclude) && c.id.contains(&wall.id) && c.status != CheckStatus::NotApplicable).map(|c| c.utilization).fold(None, |a: Option<f64>, u| Some(a.map_or(u, |x| x.max(u))))
            };
            for (key, prefix, exclude) in [("compression", "en1996.6.1.2.compression.", ".mid."), ("mid", "en1996.6.1.2.compression.mid.", "~"), ("slenderness", "en1996.5.5.1.slenderness.", "~"), ("simplified", "en1996.3.simplified.", "~")] {
                let (Some(rust), Some(oracle)) = (rust_max(prefix, exclude), expected[key].as_f64()) else { continue };
                assert!((rust - oracle).abs() <= 1e-6 * oracle.abs().max(1e-9) + 1e-9, "{name}/{}/{key}: rust {rust}, oracle {oracle}", wall.id);
                compared += 1;
            }
        }
        assert!(compared >= 3, "{name}: only {compared} comparisons");
    }
}

#[test]
fn python_geometry_oracle_matches_the_arch_the_relocation_and_the_yield_pattern() {
    use crate::checks::masonry::numerical_models::{arch_block_depth, arch_thrust_line, relocation, yield_line_pattern, yield_line_pressure, Edges, Support};
    let arches = [(0.24, 2.6, 3.0e6), (0.175, 3.1, 4.5e6), (0.365, 4.0, 2.2e6)];
    let moves: [(&[f64], &[bool], &[f64]); 4] = [
        (&[0.0, 1.0, 5.7, 9.0, 12.0], &[false, true, true, true, false], &[4.0, 4.0, 4.0, 4.0]),
        (&[0.0, 1.0, 2.5, 9.0], &[false, true, true, false], &[4.0, 4.0, 4.0]),
        (&[0.0, 3.0, 9.0], &[false, true, false], &[4.0, 4.0]),
        (&[0.0, 0.4, 3.1, 6.2], &[false, false, true, false], &[1.2, 2.4, 2.4]),
    ];
    let panels = [("SSSS", 4.0, 4.0, 1000.0, 1000.0), ("SSSS", 6.0, 3.0, 800.0, 300.0), ("SSFF", 2.0, 3.0, 700.0, 200.0), ("SSSF", 3.5, 2.7, 650.0, 250.0)];
    let mut cases = Vec::new();
    for (i, (t, span, fd)) in arches.iter().enumerate() {
        cases.push(serde_json::json!({"id": format!("arch{i}"), "kind": "arch", "t_m": t, "span_m": span, "fd_pa": fd, "samples": 8}));
    }
    for (i, (at, movable, limits)) in moves.iter().enumerate() {
        cases.push(serde_json::json!({"id": format!("move{i}"), "kind": "relocation", "at": at, "movable": movable, "limits": limits}));
    }
    for (i, (code, l, h, m1, m2)) in panels.iter().enumerate() {
        cases.push(serde_json::json!({"id": format!("yield{i}"), "kind": "yield", "edges": code, "l_m": l, "h_m": h, "m1": m1, "m2": m2}));
    }
    let answer = run(serde_json::json!({"mode": "geometry", "cases": cases}));
    assert_eq!(answer["ok"], true);
    for (i, (t, span, fd)) in arches.iter().enumerate() {
        let want = &answer["results"][format!("arch{i}")];
        assert!((arch_block_depth(*t) - want["block"].as_f64().unwrap()).abs() < 1e-9, "block depth {i}");
        let line = arch_thrust_line(*t, *span, 8);
        for (got, want) in line.iter().zip(want["points"].as_array().unwrap()) {
            assert!((got.0 - want[0].as_f64().unwrap()).abs() < 1e-9 && (got.1 - want[1].as_f64().unwrap()).abs() < 1e-9, "thrust line {i}");
        }
        let q = crate::checks::masonry::numerical_models::arching_pressure(*fd, *t, *span);
        assert!((q - want["q"].as_f64().unwrap()).abs() <= 1e-9 * q, "the three-hinge statics reproduce the catalogue capacity {i}");
        assert!((q - want["q_catalogue"].as_f64().unwrap()).abs() <= 1e-9 * q);
    }
    for (i, (at, movable, limits)) in moves.iter().enumerate() {
        let points: Vec<Support> = at.iter().zip(movable.iter()).map(|(&at, &movable)| Support { at, movable }).collect();
        let got = relocation(&points, limits);
        let want = &answer["results"][format!("move{i}")];
        match got {
            Some((index, to)) => {
                assert_eq!(want["index"].as_u64(), Some(index as u64), "move {i}");
                assert!((to - want["at"].as_f64().unwrap()).abs() < 1e-9, "move {i}: {to}");
            }
            None => assert!(want.is_null(), "move {i}: oracle {want}"),
        }
    }
    for (i, (code, l, h, m1, m2)) in panels.iter().enumerate() {
        let edges = Edges::parse(code).unwrap();
        let pattern = yield_line_pattern(edges, *l, *h, *m1, *m2);
        let want = answer["results"][format!("yield{i}")]["pressure"].as_f64().unwrap();
        assert!((pattern.pressure - want).abs() <= 3e-3 * want, "yield {i}: rust {} oracle {want}", pattern.pressure);
        assert!((pattern.pressure - yield_line_pressure(edges, *l, *h, *m1, *m2)).abs() <= 1e-12 * want);
        assert!(!pattern.lines(*l, *h).is_empty());
    }
}

#[test]
fn python_oracle_matches_the_annex_e_structure_its_stand_in_the_chase_limit_and_the_joint_move() {
    use crate::checks::masonry::annex_e::{class_code, classes, interpolate_cells, standin_alpha1, AnnexECell};
    use crate::checks::masonry::numerical_models::{relocation, Edges, Support};
    use crate::checks::masonry::{chase_limits, detailing_checks};
    use semio_s_artifact_norm_en1996_model::ChaseKind;
    let codes = ["SSSS", "XSSF", "FXSX", "SFXS", "FFSS", "XXXX"];
    let synthetic = [(0.5, 0.5, 1.0), (0.5, 1.0, 3.0), (0.5, 1.5, 4.0), (1.0, 0.5, 2.0), (1.0, 1.0, 6.5), (1.0, 1.5, 7.0)];
    let points = [(0.5, 0.75), (0.75, 1.0), (1.0, 1.5), (0.4, 1.0), (0.75, 1.7)];
    let mut cases = vec![serde_json::json!({"id": "classes", "kind": "annex-e-classes", "codes": codes})];
    for (i, (mu, hl)) in points.iter().enumerate() {
        cases.push(serde_json::json!({"id": format!("grid{i}"), "kind": "annex-e-grid", "cells": synthetic, "mu": mu, "hl": hl}));
    }
    for (i, (code, mu, hl)) in [("SSSS", 1.0, 1.0), ("SSSF", 0.7, 0.6), ("XXSS", 0.5, 1.4)].iter().enumerate() {
        cases.push(serde_json::json!({"id": format!("standin{i}"), "kind": "annex-e-standin", "edges": code, "mu": mu, "hl": hl}));
    }
    let chases = [("vertical", ChaseKind::Vertical, 0.06, 1.0, 0.5), ("vertical", ChaseKind::Vertical, 0.115, 1.0, 2.0), ("vertical", ChaseKind::Vertical, 0.24, 1.0, 0.5), ("vertical", ChaseKind::Vertical, 0.24, 1.0, 2.0), ("recess", ChaseKind::Recess, 0.365, 0.4, 0.5), ("horizontal", ChaseKind::Horizontal, 0.175, 1.0, 0.0), ("horizontal", ChaseKind::Horizontal, 0.365, 2.0, 0.0)];
    for (i, (orientation, _, t, length, top)) in chases.iter().enumerate() {
        cases.push(serde_json::json!({"id": format!("chase{i}"), "kind": "chase-limit", "orientation": orientation, "t_m": t, "length_m": length, "top_m": top, "storey_m": 2.75}));
    }
    let at = [0.0, 8.0, 26.0, 30.0];
    cases.push(serde_json::json!({"id": "joints", "kind": "relocation", "at": at, "movable": [false, true, true, false], "limits": [12.0, 12.0, 12.0]}));
    let answer = run(serde_json::json!({"mode": "geometry", "cases": cases}));
    assert_eq!(answer["ok"], true);
    let want: Vec<String> = answer["results"]["classes"]["classes"].as_array().unwrap().iter().map(|c| c.as_str().unwrap().to_owned()).collect();
    assert_eq!(classes(), want, "the 36 edge-condition classes");
    for code in codes {
        assert_eq!(class_code(Edges::parse(code).unwrap()), answer["results"]["classes"]["canonical"][code].as_str().unwrap(), "canonical class of {code}");
    }
    let cells: Vec<AnnexECell> = synthetic.iter().map(|&(mu, h_over_l, alpha1)| AnnexECell { mu, h_over_l, alpha1 }).collect();
    for (i, (mu, hl)) in points.iter().enumerate() {
        let want = &answer["results"][format!("grid{i}")]["value"];
        match interpolate_cells(&cells, *mu, *hl) {
            Some(got) => assert!((got - want.as_f64().unwrap()).abs() < 1e-9, "grid {i}: {got}"),
            None => assert!(want.is_null(), "grid {i}: oracle {want}"),
        }
    }
    for (i, (code, mu, hl)) in [("SSSS", 1.0, 1.0), ("SSSF", 0.7, 0.6), ("XXSS", 0.5, 1.4)].iter().enumerate() {
        let got = standin_alpha1(Edges::parse(code).unwrap(), *mu, *hl);
        let want = answer["results"][format!("standin{i}")]["alpha"].as_f64().unwrap();
        assert!((got - want).abs() <= 3e-3 * want, "stand-in {i}: rust {got} oracle {want}");
    }
    for (i, (_, kind, t, length, top)) in chases.iter().enumerate() {
        let want = &answer["results"][format!("chase{i}")];
        match chase_limits(*kind, *t, *length, *top, 2.75, AnnexChoice::De) {
            Some(got) => {
                assert!((got.depth_m - want["limit"].as_f64().unwrap()).abs() < 1e-12, "chase {i} depth");
                assert_eq!(got.width_m.is_some(), want["width"].is_number(), "chase {i} width");
                if let Some(w) = got.width_m {
                    assert!((w - want["width"].as_f64().unwrap()).abs() < 1e-12, "chase {i} width");
                }
            }
            None => assert!(want["limit"].is_null(), "chase {i}"),
        }
    }
    let supports: Vec<Support> = at.iter().zip([false, true, true, false]).map(|(&at, movable)| Support { at, movable }).collect();
    let got = relocation(&supports, &[12.0, 12.0, 12.0]);
    let want = &answer["results"]["joints"];
    assert_eq!(got.map(|(index, _)| index as u64), want["index"].as_u64());
    assert!((got.unwrap().1 - want["at"].as_f64().unwrap()).abs() < 1e-9);
    assert!(detailing_checks(AnnexChoice::De, &[], &[], &[]).checks.is_empty());
}

#[test]
fn python_oracle_matches_the_computed_remedies_of_confined_masonry_prestress_ties_and_anchors() {
    use crate::checks::masonry::ROW_KEYS;
    use semio_s_plugin_norm_compliance::LocalizedCopy;
    let doc = |path: &str| Case::new("t", AnnexChoice::En).bound("w", path, LocalizedCopy::new("W", "W"));
    let rows = |case: Case, positions: &[f64]| case.with_v("pos_m", positions).with_paths(positions.iter().enumerate().map(|(i, _)| (ROW_KEYS[i], format!("walls[id=w].r[id=r{i}].positionM"))));
    let m43 = rows(doc("walls[id=w].confined[id=r0].asM2"), &[0.5, 6.0]).with_n("col_a_m2", 0.018).with_n("col_min_m", 0.3).with_n("col_as_m2", 1e-3).with_n("col_spacing_m", 5.5).with_n("l_m", 9.0).with_path("col_a_side_m", "walls[id=w].confined[id=r0].depthM").with_n("col_a_side_m", 0.12).with_n("col_a_other_m", 0.15);
    let m51 = rows(doc("walls[id=w].ties[id=r0].capacityN"), &[1.0, 1.8, 5.4]).with_n("w_ed_pa", 1000.0).with_n("h_trib_m", 1.0).with_n("s_anchor_m", 3.6).with_n("anchor_rk_n", 4500.0).with_n("l_m", 6.0);
    let m44 = doc("walls[id=w].prestress[id=r0].forceN").with_path("t_m", "walls[id=w].thicknessM").with_n("t_m", 0.2).with_n("p0_n", 500e3).with_n("ap_m2", 1e-3).with_n("fpk_pa", 1.86e9).with_n("ep_pa", 195e9).with_n("e_pa", 5e9).with_n("a_m2", 0.2).with_n("n_ed_n", 100e3).with_n("fd_pa", 1.5e6);
    let m50 = doc("walls[id=w].ties[id=r0].capacityN").with_path("tie_rd_n", "walls[id=w].ties[id=r0].capacityN").with_n("n_per_m2", 3.0).with_n("cavity_m", 0.1).with_n("w_ed_pa", 900.0).with_n("t1_m", 0.1).with_n("t2_m", 0.1).with_n("tie_rd_n", 100.0);
    let request = |id: &str, kind: &str, c: &Case| serde_json::json!({"id": id, "kind": kind, "num": c.num, "pos_m": c.vec.get("pos_m").cloned().unwrap_or_default()});
    let answer = run(serde_json::json!({"mode": "geometry", "cases": [request("m43", "remedy-m43", &m43), request("m51", "remedy-m51", &m51), request("m44", "remedy-m44", &m44), request("m50", "remedy-m50", &m50)]}));
    let required = |entry: &str, case: &Case, outcome: &str, key: &str| {
        let fix = evaluate_outcomes(entry, case).unwrap().into_iter().find(|o| o.key == outcome).and_then(|o| o.fix).unwrap_or_else(|| panic!("{entry}/{outcome} has no fix"));
        assert_eq!(fix.key, key);
        fix.required
    };
    let close = |got: f64, want: f64| assert!((got - want).abs() < 1e-9, "rust {got}, oracle {want}");
    close(required("M-43", &m43, "col-area", "col_a_side_m"), answer["results"]["m43"]["side"].as_f64().unwrap());
    let moved = &answer["results"]["m43"]["move"];
    close(required("M-43", &m43, "col-spacing", ROW_KEYS[moved["index"].as_u64().unwrap() as usize - 1]), moved["at"].as_f64().unwrap());
    let moved = &answer["results"]["m51"]["move"];
    close(required("M-51", &m51, "anchor", ROW_KEYS[moved["index"].as_u64().unwrap() as usize - 1]), moved["at"].as_f64().unwrap());
    close(required("M-44", &m44, "compression", "t_m"), answer["results"]["m44"]["thickness"].as_f64().unwrap());
    close(required("M-50", &m50, "tie-force", "tie_rd_n"), answer["results"]["m50"]["resistance"].as_f64().unwrap());
}
