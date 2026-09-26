use super::*;
use crate::document::CheckStatus;
use crate::standards::v1::subsets::any::schema::NaDe;

#[semio_framework_async_macros::async_test]
async fn evaluate_default_produces_checks() {
    let doc = En1990Snapshot::default();
    let report = evaluate(&doc);
    assert!(!report.checks.is_empty(), "default document must produce checks");
    assert!(report.checks.iter().any(|c| c.id.contains("6.10") || c.clause.section.contains("6.10")));
}

#[semio_framework_async_macros::async_test]
async fn default_subject_complies() {
    let report = evaluate(&En1990Snapshot::default());
    let fails: Vec<_> = report.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| c.id.clone()).collect();
    assert!(fails.is_empty(), "default must comply, fails={fails:?}");
    assert!(report.complies());
}

#[semio_framework_async_macros::async_test]
async fn high_consequence_office_fails_with_multiple_checks() {
    let doc = crate::standards::v1::subsets::any::examples::high_consequence_office::reference_snapshot();
    let report = evaluate(&doc);
    let fails: Vec<_> = report.checks.iter().filter(|c| c.status == CheckStatus::Fail).collect();
    assert!(fails.len() >= 2, "expected ≥2 fails, got {} {:?}", fails.len(), fails.iter().map(|c| &c.id).collect::<Vec<_>>());
    assert!(!report.complies());
    assert!(fails.iter().all(|c| !c.remedies.is_empty()), "every Fail needs a remedy");
}

#[semio_framework_async_macros::async_test]
async fn high_consequence_office_dsl_asset_decodes_and_fails() {
    let text = crate::standards::v1::subsets::any::examples::high_consequence_office::PRIMARY_TEXT;
    let doc = crate::standards::v1::subsets::any::schema::snapshot::decode_en1990_dsl(text).expect("dsl");
    assert_eq!(doc.consequence_class, 3);
    let report = evaluate(&doc);
    assert!(report.summary.fail >= 2, "fail_count={}", report.summary.fail);
}

#[semio_framework_async_macros::async_test]
async fn default_office_dsl_asset_decodes_and_complies() {
    let text = include_str!("../../../../🖼️assets/🏢️default-office/🏢️default-office/🗣️.dsl.semio");
    let doc = crate::standards::v1::subsets::any::schema::snapshot::decode_en1990_dsl(text).expect("dsl");
    assert!(evaluate(&doc).complies());
}

#[semio_framework_async_macros::async_test]
async fn evaluate_marks_seismic_not_applicable_when_no_a_ek() {
    let doc = En1990Snapshot::default();
    assert!(doc.seismics.is_empty());
    let report = evaluate(&doc);
    let seismic = report.checks.iter().find(|c| c.clause.section == "6.12b" || c.id.contains("6.12b")).expect("6.12b check present on default");
    assert_eq!(seismic.status, CheckStatus::NotApplicable, "empty seismics must yield NotApplicable, got {:?}", seismic.status);
}

#[semio_framework_async_macros::async_test]
async fn dangling_effect_action_id_fails_referential_integrity() {
    let mut doc = En1990Snapshot::default();
    doc.effects[0].action_id = format!("{}-x", doc.effects[0].action_id);
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.effects") && c.id.contains("actionId") && c.status == CheckStatus::Fail));
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn dangling_bridge_sls_member_id_fails_referential_integrity() {
    let mut doc = crate::standards::v1::subsets::any::examples::road_bridge_compliant::reference_snapshot();
    doc.bridge_sls[0].member_id = format!("{}-x", doc.bridge_sls[0].member_id);
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.bridgeSls") && c.id.contains("memberId") && c.status == CheckStatus::Fail));
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn duplicate_member_id_fails_integrity() {
    let mut doc = En1990Snapshot::default();
    let twin = doc.members[0].clone();
    doc.members.push(twin);
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.id.contains("integrity.duplicate.members") && c.status == CheckStatus::Fail));
    assert!(!report.complies());
}


#[semio_framework_async_macros::async_test]
async fn every_emitted_subject_path_parses_and_resolves() {
    let doc = En1990Snapshot::default();
    let report = evaluate(&doc);
    let root = dsl::ToValue::to_value(&doc);
    let mut seen = 0usize;
    for check in &report.checks {
        for path in std::iter::once(check.subject.path.as_str()).chain(check.remedies.iter().map(|r| r.target.path.as_str())) {
            if path.is_empty() {
                continue;
            }
            crate::app_surface::parse_path(path).unwrap_or_else(|e| panic!("parse '{path}': {e}"));
            crate::app_surface::get_value_at_path(&root, path).unwrap_or_else(|e| panic!("resolve '{path}': {e}"));
            seen += 1;
            assert!(!path.contains("members[") || path.contains("members[id="), "entity list path must use [id=…]: {path}");
        }
    }
    assert!(seen > 0, "expected at least one subject/remedy path");
}

#[semio_framework_async_macros::async_test]
async fn editing_gk_changes_str_utilization() {
    let mut doc = En1990Snapshot::default();
    let before = evaluate(&doc);
    let u0 = before.checks.iter().find(|c| c.id.contains("str.") && c.id.contains("6.10.")).map(|c| c.utilization).unwrap();
    doc.permanents.iter_mut().find(|p| p.kind == "g_sup").unwrap().gk *= 2.0;
    let after = evaluate(&doc);
    let u1 = after.checks.iter().find(|c| c.id.contains("str.") && c.id.contains("6.10.")).map(|c| c.utilization).unwrap();
    assert!(u1 > u0 + 0.05, "doubling gk must raise STR utilization ({u0} → {u1})");
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_beta_writing_required_makes_check_pass() {
    let mut doc = crate::standards::v1::subsets::any::examples::high_consequence_office::reference_snapshot();
    let report = evaluate(&doc);
    let fail = report.checks.iter().find(|c| c.id.contains("beta") && c.status == CheckStatus::Fail && c.remedies.iter().any(|r| r.applicable)).expect("β fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable).unwrap();
    apply_path(&mut doc, &remedy.target.path, remedy.required.value);
    let after = evaluate(&doc);
    let same = after.checks.iter().find(|c| c.id == fail.id).expect("check");
    assert!(same.status != CheckStatus::Fail || same.utilization <= 1.0 + 1e-9, "β remedy failed: {:?} util={}", same.status, same.utilization);
}

#[semio_framework_async_macros::async_test]
async fn remedy_law_deflection_writing_required_makes_check_pass() {
    let mut doc = crate::standards::v1::subsets::any::examples::high_consequence_office::reference_snapshot();
    let report = evaluate(&doc);
    let fail = report
        .checks
        .iter()
        .find(|c| c.id.contains("deflection") && c.status == CheckStatus::Fail && c.remedies.iter().any(|r| r.applicable))
        .expect("deflection fail");
    let remedy = fail.remedies.iter().find(|r| r.applicable).unwrap();
    apply_path(&mut doc, &remedy.target.path, remedy.required.value);
    let after = evaluate(&doc);
    let same = after.checks.iter().find(|c| c.id == fail.id).expect("check");
    assert!(same.status != CheckStatus::Fail || same.utilization <= 1.0 + 1e-9, "deflection remedy failed: {:?} util={}", same.status, same.utilization);
}

#[semio_framework_async_macros::async_test]
async fn sls_clause_ids_are_614b_615b_616b() {
    let report = evaluate(&En1990Snapshot::default());
    assert!(report.checks.iter().any(|c| c.clause.section == "6.14b"));
    assert!(report.checks.iter().any(|c| c.clause.section == "6.15b"));
    assert!(report.checks.iter().any(|c| c.clause.section == "6.16b"));
    assert!(!report.checks.iter().any(|c| c.clause.section == "6.17"));
}

#[semio_framework_async_macros::async_test]
async fn geo_and_equ_stab_checks_emitted() {
    let report = evaluate(&En1990Snapshot::default());
    assert!(report.checks.iter().any(|c| c.id.contains(".geo.")));
    assert!(report.checks.iter().any(|c| c.id.contains("equ.stab")));
}

#[semio_framework_async_macros::async_test]
async fn python_oracle_matches_evaluate_json_within_half_percent() {
    let oracle = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🐍️.py");
    assert!(oracle.exists(), "missing oracle {}", oracle.display());
    for (name, doc) in [
        ("default", En1990Snapshot::default()),
        ("high", crate::standards::v1::subsets::any::examples::high_consequence_office::reference_snapshot()),
    ] {
        let report = evaluate(&doc);
        let ed = report
            .checks
            .iter()
            .find(|c| c.id.contains("str.") && c.id.contains(".6.10.") && !c.id.contains("6.10a") && !c.id.contains("6.10b") && c.id.ends_with(".0"))
            .map(|c| c.computed.value)
            .expect("governing 6.10 leading0");
        let actions = action_set_for_member(&doc, &doc.members[0].id);
        let payload = serde_json::json!({
            "g": actions.g_k,
            "g_inf": actions.g_k_inf,
            "q": actions.q_k,
            "cc": doc.consequence_class,
            "annex": if doc.annex == crate::document::AnnexChoice::De { "de" } else { "en" },
            "rust_ed": ed,
            "name": name,
        });
        let tmp = std::env::temp_dir().join(format!("en1990-oracle-{name}.json"));
        std::fs::write(&tmp, payload.to_string()).unwrap();
        let out = std::process::Command::new("python3")
            .arg("-c")
            .arg(format!(
                r#"
import json
from importlib.machinery import SourceFileLoader
o = SourceFileLoader('oracle', r'{oracle}').load_module()
d = json.load(open(r'{tmp}'))
k = o.k_fi(d['cc'])
psi0 = o.psi0_de if d['annex']=='de' else o.psi0_en
g, g_inf, q = d['g'], d['g_inf'], d['q']
ed_a = k*(1.35*g + 1.0*g_inf)
ed_b = k*(0.85*1.35*g + 1.0*g_inf)
for i,(cat,qi) in enumerate(q):
    f = k*(1.5 if i==0 else 1.5*psi0(cat))
    ed_a += f*qi
    ed_b += f*qi
ed = max(ed_a, ed_b)
rust = d['rust_ed']
rel = abs(ed-rust)/max(abs(rust),1.0)
assert rel <= 0.005, (d['name'], ed, rust, rel)
print('ok', d['name'], ed, rust)
"#,
                oracle = oracle.display(),
                tmp = tmp.display(),
            ))
            .output()
            .expect("python");
        assert!(
            out.status.success(),
            "oracle {name}: {}\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }
}

fn apply_path(doc: &mut En1990Snapshot, path: &str, value: f64) {
    let mut root = dsl::ToValue::to_value(&*doc);
    crate::app_surface::set_value_at_path(&mut root, path, dsl::DslValue::float(value)).unwrap_or_else(|e| panic!("set {path}: {e}"));
    *doc = dsl::FromValue::from_value(root).expect("from_value");
}

#[semio_framework_async_macros::async_test]
async fn table_2_1_bridge_category_remedy_flips_to_pass() {
    let mut doc = En1990Snapshot::default();
    doc.structure_kind = "footbridge".into();
    doc.design_working_life_category = 4;
    doc.design_working_life_years = 50.0;
    let report = evaluate(&doc);
    let fail = report
        .checks
        .iter()
        .find(|c| c.id == "en1990.table-2.1.category" && c.status == CheckStatus::Fail)
        .expect("category fail for bridge");
    let remedy = fail.remedies.iter().find(|r| r.applicable).expect("remedy");
    assert_eq!(remedy.target.path, "designWorkingLifeCategory");
    let choice = remedy.options.first().expect("one_of option");
    assert_eq!(choice, "5");
    doc.design_working_life_category = choice.parse().unwrap();
    doc.design_working_life_years = 100.0;
    let after = evaluate(&doc);
    let same = after.checks.iter().find(|c| c.id == fail.id).unwrap();
    assert_ne!(same.status, CheckStatus::Fail, "category remedy should pass");
}

#[semio_framework_async_macros::async_test]
async fn bridge_a2_partial_factors_de_vs_en_diverge_on_rail_gamma_q() {
    use crate::standards::v1::subsets::any::schema::{combination_str_geo_610a_for, gamma_q_bridge_traffic, ActionSet, NaDe, NaEn};
    use crate::document::AnnexChoice;
    assert!((gamma_q_bridge_traffic(AnnexChoice::De, "rail_traffic") - 1.40).abs() < 1e-12);
    assert!((gamma_q_bridge_traffic(AnnexChoice::En, "rail_traffic") - 1.45).abs() < 1e-12);
    let actions = ActionSet {
        g_k: 100.0,
        g_k_inf: 0.0,
        p_k: 0.0,
        q_k: vec![("rail_traffic".into(), 50.0)],
        a_d: 0.0,
        a_ed: 0.0,
    };
    let de = combination_str_geo_610a_for(&NaDe, &actions, 0, 2, "rail_bridge");
    let en = combination_str_geo_610a_for(&NaEn, &actions, 0, 2, "rail_bridge");
    assert!((de - (1.35 * 100.0 + 1.40 * 50.0)).abs() < 1e-9, "de={de}");
    assert!((en - (1.35 * 100.0 + 1.45 * 50.0)).abs() < 1e-9, "en={en}");
    assert!((en - de).abs() > 1.0);
}

#[semio_framework_async_macros::async_test]
async fn evaluate_reads_project_id_in_subject_labels() {
    let doc = En1990Snapshot::default();
    let report = evaluate(&doc);
    assert!(
        report.checks.iter().any(|c| c.subject.label.en.contains(&doc.project_id) || c.subject.entity_id == doc.project_id),
        "projectId must appear as report/entity label"
    );
}




#[semio_framework_async_macros::async_test]
async fn road_bridge_compliant_subject_complies() {
    let doc = crate::standards::v1::subsets::any::examples::road_bridge_compliant::reference_snapshot();
    let report = evaluate(&doc);
    assert!(report.complies(), "bridge compliant must pass, fails={:?}", report.checks.iter().filter(|c| c.status == CheckStatus::Fail).map(|c| &c.id).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn road_bridge_failing_fails_with_multiple_checks() {
    let doc = crate::standards::v1::subsets::any::examples::road_bridge_failing::reference_snapshot();
    let report = evaluate(&doc);
    assert!(report.summary.fail >= 2, "fail_count={}", report.summary.fail);
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn accidental_seismic_compliant_activates_6_11_and_6_12b() {
    let doc = crate::standards::v1::subsets::any::examples::accidental_seismic_compliant::reference_snapshot();
    let report = evaluate(&doc);
    let c611 = report.checks.iter().find(|c| c.clause.section == "6.11").expect("6.11");
    let c612 = report.checks.iter().find(|c| c.clause.section == "6.12b").expect("6.12b");
    assert_ne!(c611.status, CheckStatus::NotApplicable);
    assert_ne!(c612.status, CheckStatus::NotApplicable);
    assert!(report.complies(), "accidental/seismic compliant should pass");
}

#[semio_framework_async_macros::async_test]
async fn accidental_seismic_failing_fails_6_11_or_6_12b() {
    let doc = crate::standards::v1::subsets::any::examples::accidental_seismic_failing::reference_snapshot();
    let report = evaluate(&doc);
    assert!(report.summary.fail >= 2, "fail_count={}", report.summary.fail);
    assert!(report.checks.iter().any(|c| c.clause.section == "6.11" && c.status == CheckStatus::Fail));
    assert!(report.checks.iter().any(|c| c.clause.section == "6.12b" && c.status == CheckStatus::Fail));
}

#[semio_framework_async_macros::async_test]
async fn fatigue_compliant_subject_complies() {
    let doc = crate::standards::v1::subsets::any::examples::fatigue_compliant::reference_snapshot();
    let report = evaluate(&doc);
    let fat = report.checks.iter().find(|c| c.id.contains("fat") || c.clause.section.to_lowercase().contains("fat")).expect("FAT");
    assert_ne!(fat.status, CheckStatus::NotApplicable);
    assert!(report.complies());
}

#[semio_framework_async_macros::async_test]
async fn fatigue_failing_fails_fat_check() {
    let doc = crate::standards::v1::subsets::any::examples::fatigue_failing::reference_snapshot();
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| (c.id.contains("fat") || c.clause.section.to_lowercase().contains("fat")) && c.status == CheckStatus::Fail));
    assert!(!report.complies());
}

#[semio_framework_async_macros::async_test]
async fn every_editable_leaf_changes_a_check_when_perturbed_in_applicable_scope() {
    // CORRECTION 13:43 — descriptive report labels exempted explicitly.
    const LABEL_EXCEPTIONS: &[&str] = &["projectId", "labelEn", "labelDe"];

    fn leaf_name(path: &str) -> &str {
        path.rsplit(['.', ']']).next().unwrap_or(path).trim_start_matches('.')
    }

    fn is_exempt(path: &str) -> bool {
        let leaf = leaf_name(path);
        LABEL_EXCEPTIONS.contains(&leaf) || leaf == "name" || leaf == "title"
    }

    fn table_name(path: &str) -> Option<&str> {
        path.split('[').next()
    }

    fn sibling_id(path: &str, tree: &serde_json::Value, current: &str) -> Option<String> {
        let table = table_name(path)?;
        let arr = get_json(tree, table)?.as_array()?;
        for item in arr {
            if let Some(id) = item.get("id").and_then(|v| v.as_str()) {
                if id != current {
                    return Some(id.to_string());
                }
            }
        }
        None
    }

    fn perturb_leaf(path: &str, cur: &serde_json::Value, tree: &serde_json::Value) -> Option<serde_json::Value> {
        let leaf = leaf_name(path);
        if leaf == "memberId" || leaf == "actionId" {
            let s = cur.as_str()?;
            return Some(serde_json::json!(format!("{s}-x")));
        }
        if leaf == "id" {
            let s = cur.as_str()?;
            if let Some(sibling) = sibling_id(path, tree, s) {
                return Some(serde_json::json!(sibling));
            }
            return Some(serde_json::json!(format!("{s}-x")));
        }
        perturb(cur)
    }


    fn walk(prefix: &str, value: &serde_json::Value, out: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    let p = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
                    walk(&p, v, out);
                }
            }
            serde_json::Value::Array(items) => {
                for (i, v) in items.iter().enumerate() {
                    let id = v.get("id").and_then(|x| x.as_str());
                    let p = match id {
                        Some(id) => format!("{prefix}[id={id}]"),
                        None => format!("{prefix}[{i}]"),
                    };
                    walk(&p, v, out);
                }
            }
            _ => {
                if !prefix.is_empty() {
                    out.push(prefix.to_string());
                }
            }
        }
    }

    fn get_json<'a>(root: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
        let mut cur = root;
        let mut rest = path;
        while !rest.is_empty() {
            if let Some(r) = rest.strip_prefix('.') {
                rest = r;
            }
            if let Some(r) = rest.strip_prefix('[') {
                let close = r.find(']')?;
                let inner = &r[..close];
                let arr = cur.as_array()?;
                cur = if let Some(id) = inner.strip_prefix("id=") {
                    arr.iter().find(|v| v.get("id").and_then(|x| x.as_str()) == Some(id))?
                } else {
                    arr.get(inner.parse::<usize>().ok()?)?
                };
                rest = &r[close + 1..];
                continue;
            }
            let end = rest.find(['.', '[']).unwrap_or(rest.len());
            let key = &rest[..end];
            cur = cur.get(key)?;
            rest = &rest[end..];
        }
        Some(cur)
    }

    fn set_json(root: &mut serde_json::Value, path: &str, value: serde_json::Value) -> bool {
        let mut cur = root;
        let mut rest = path;
        while !rest.is_empty() {
            if let Some(r) = rest.strip_prefix('.') {
                rest = r;
            }
            if let Some(r) = rest.strip_prefix('[') {
                let close = match r.find(']') {
                    Some(c) => c,
                    None => return false,
                };
                let inner = &r[..close];
                let arr = match cur.as_array_mut() {
                    Some(a) => a,
                    None => return false,
                };
                let idx = if let Some(id) = inner.strip_prefix("id=") {
                    match arr.iter().position(|v| v.get("id").and_then(|x| x.as_str()) == Some(id)) {
                        Some(i) => i,
                        None => return false,
                    }
                } else {
                    match inner.parse::<usize>() {
                        Ok(i) => i,
                        Err(_) => return false,
                    }
                };
                rest = &r[close + 1..];
                if rest.is_empty() {
                    arr[idx] = value;
                    return true;
                }
                cur = &mut arr[idx];
                continue;
            }
            let end = rest.find(['.', '[']).unwrap_or(rest.len());
            let key = &rest[..end];
            rest = &rest[end..];
            if rest.is_empty() {
                if let Some(obj) = cur.as_object_mut() {
                    obj.insert(key.to_string(), value);
                    return true;
                }
                return false;
            }
            cur = match cur.get_mut(key) {
                Some(v) => v,
                None => return false,
            };
        }
        false
    }

    fn perturb(cur: &serde_json::Value) -> Option<serde_json::Value> {
        match cur {
            serde_json::Value::Number(n) => {
                let v = n.as_f64().unwrap_or(0.0);
                let nv = if v.abs() < 1e-12 {
                    1.0
                } else if (0.0..1000.0).contains(&v) && (v - 0.0).abs() < 1e-9 {
                    1200.0
                } else {
                    v * 1.25 + 0.5
                };
                Some(serde_json::json!(nv))
            }
            serde_json::Value::String(s) => {
                let ns = match s.as_str() {
                    "De" => "En",
                    "En" => "De",
                    "building" => "road_bridge",
                    "road_bridge" | "footbridge" | "rail_bridge" => "building",
                    "DSL1" => "DSL2",
                    "DSL2" => "DSL3",
                    "DSL3" => "DSL1",
                    "IL1" => "IL2",
                    "IL2" => "IL3",
                    "IL3" => "IL1",
                    "office" => "snow",
                    "wind" => "snow",
                    "snow" => "snow_high",
                    "road_traffic" => "rail_traffic",
                    "g_sup" => "g_inf",
                    "g_inf" => "prestress",
                    other => return Some(serde_json::json!(format!("{other}-x"))),
                };
                Some(serde_json::json!(ns))
            }
            serde_json::Value::Bool(b) => Some(serde_json::json!(!b)),
            _ => None,
        }
    }

    fn sig(report: &crate::document::CheckReport) -> Vec<(String, CheckStatus, i64, i64, i64)> {
        report
            .checks
            .iter()
            .map(|c| {
                (
                    c.id.clone(),
                    c.status,
                    (c.computed.value * 1e3).round() as i64,
                    (c.limit.value * 1e3).round() as i64,
                    (c.utilization * 1e6).round() as i64,
                )
            })
            .collect()
    }

    let building = En1990Snapshot::default();
    let mut building_snow = crate::standards::v1::subsets::any::examples::high_consequence_office::reference_snapshot();
    building_snow.annex = crate::document::AnnexChoice::De;
    building_snow.altitude_m = 0.0;
    let bridge = crate::standards::v1::subsets::any::examples::road_bridge_compliant::reference_snapshot();

    let mut unchanged = Vec::new();
    let mut checked = 0usize;

    // Building leaves (non-bridgeSls, non-altitude — altitude needs snow).
    {
        let base = &building;
        let base_sig = sig(&evaluate(base));
        let tree0 = serde_json::to_value(base).expect("json");
        let mut leaves = Vec::new();
        walk("", &tree0, &mut leaves);
        for path in leaves {
            if is_exempt(&path) {
                continue;
            }
            if path.starts_with("bridgeSls") || path == "altitudeM" {
                continue;
            }
            let cur = match get_json(&tree0, &path) {
                Some(v) => v.clone(),
                None => continue,
            };
            let next = if path == "referencePeriodYears" {
                // Cross the 25 a threshold used by target β selection.
                let v = cur.as_f64().unwrap_or(50.0);
                serde_json::json!(if v >= 25.0 { 10.0 } else { 50.0 })
            } else {
                match perturb_leaf(&path, &cur, &tree0) {
                    Some(v) => v,
                    None => continue,
                }
            };
            let mut tree = tree0.clone();
            if !set_json(&mut tree, &path, next) {
                continue;
            }
            let Ok(perturbed) = serde_json::from_value::<En1990Snapshot>(tree) else { continue };
            checked += 1;
            // Annex DE↔EN is a no-op for office/wind ψ; use seismic example where ψ₂ diverges.
            if path == "annex" {
                let mut annex_base = crate::standards::v1::subsets::any::examples::accidental_seismic_compliant::reference_snapshot();
                // Category "other" has divergent ψ₂ DE vs EN (see seismic_combination_de_vs_en_diverge_on_other_psi_2).
                annex_base.variables[0].category = "other".into();
                annex_base.annex = crate::document::AnnexChoice::De;
                let annex_sig = sig(&evaluate(&annex_base));
                annex_base.annex = crate::document::AnnexChoice::En;
                if sig(&evaluate(&annex_base)) == annex_sig {
                    unchanged.push(format!("building:{path}"));
                }
            } else if sig(&evaluate(&perturbed)) == base_sig {
                unchanged.push(format!("building:{path}"));
            }
        }
    }

    // Site altitude — DE snow subject (≤1000 m → >1000 m flips ψ).
    {
        let base = &building_snow;
        let base_sig = sig(&evaluate(base));
        let mut tree = serde_json::to_value(base).expect("json");
        assert!(set_json(&mut tree, "altitudeM", serde_json::json!(1200.0)));
        let perturbed: En1990Snapshot = serde_json::from_value(tree).expect("from");
        checked += 1;
        if sig(&evaluate(&perturbed)) == base_sig {
            unchanged.push("building_snow:altitudeM".into());
        }
    }

    // Bridge SLS leaves on committed bridge example.
    {
        let base = &bridge;
        let base_sig = sig(&evaluate(base));
        let tree0 = serde_json::to_value(base).expect("json");
        let mut leaves = Vec::new();
        walk("", &tree0, &mut leaves);
        for path in leaves {
            if is_exempt(&path) {
                continue;
            }
            if !path.starts_with("bridgeSls") {
                continue;
            }
            let cur = match get_json(&tree0, &path) {
                Some(v) => v.clone(),
                None => continue,
            };
            let Some(next) = perturb(&cur) else { continue };
            let mut tree = tree0.clone();
            if !set_json(&mut tree, &path, next) {
                continue;
            }
            let Ok(perturbed) = serde_json::from_value::<En1990Snapshot>(tree) else { continue };
            checked += 1;
            if sig(&evaluate(&perturbed)) == base_sig {
                unchanged.push(format!("bridge:{path}"));
            }
        }
    }

    
    // Accidental + seismic leaves on committed example (6.11 / 6.12b active).
    {
        let base = crate::standards::v1::subsets::any::examples::accidental_seismic_compliant::reference_snapshot();
        let base_report = evaluate(&base);
        assert!(base_report.checks.iter().any(|c| c.clause.section == "6.11" && c.status != CheckStatus::NotApplicable));
        assert!(base_report.checks.iter().any(|c| c.clause.section == "6.12b" && c.status != CheckStatus::NotApplicable));
        let base_sig = sig(&base_report);
        let tree0 = serde_json::to_value(&base).expect("json");
        let mut leaves = Vec::new();
        walk("", &tree0, &mut leaves);
        for path in leaves {
            if is_exempt(&path) {
                continue;
            }
            if !(path.starts_with("accidentals") || path.starts_with("seismics")) {
                continue;
            }
            let cur = match get_json(&tree0, &path) {
                Some(v) => v.clone(),
                None => continue,
            };
            let next = if path.ends_with("importanceClass") {
                let s = cur.as_str().unwrap_or("II");
                serde_json::json!(match s { "I" => "II", "II" => "III", "III" => "IV", _ => "I" })
            } else {
                match perturb_leaf(&path, &cur, &tree0) {
                    Some(v) => v,
                    None => continue,
                }
            };
            let mut tree = tree0.clone();
            if !set_json(&mut tree, &path, next) {
                continue;
            }
            let Ok(perturbed) = serde_json::from_value::<En1990Snapshot>(tree) else { continue };
            checked += 1;
            let after = evaluate(&perturbed);
            if sig(&after) == base_sig {
                unchanged.push(format!("accidental_seismic:{path}"));
            }
        }
        // Explicit flip from N/A on empty tables: removing actions should restore N/A.
        let mut cleared = base.clone();
        cleared.accidentals.clear();
        cleared.seismics.clear();
        cleared.effects.retain(|e| e.action_id != "A-impact" && e.action_id != "E-1");
        let cleared_report = evaluate(&cleared);
        assert!(cleared_report.checks.iter().any(|c| c.clause.section == "6.11" && c.status == CheckStatus::NotApplicable));
        assert!(cleared_report.checks.iter().any(|c| c.clause.section == "6.12b" && c.status == CheckStatus::NotApplicable));
    }

    assert!(checked >= 20, "expected broad leaf coverage, checked={checked}");
    assert!(unchanged.is_empty(), "leaves with no check influence: {unchanged:?} (checked={checked})");
}
