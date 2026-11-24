//! 🚦️ Gate parity of the EN 1999 compliance tests: every failing remedy clears its status. See [`super`].

use super::*;

/// 🩹 Writes the `required` value (or the first option) of one remedy into the JSON tree of a document; `false` when the path does not take the value.
fn write_remedy(tree: &mut serde_json::Value, remedy: &semio_s_plugin_norm_compliance::Remedy) -> bool {
    let path = remedy.target.path.as_str();
    if path.is_empty() {
        return false;
    }
    if matches!(remedy.bound, semio_s_plugin_norm_compliance::RemedyBound::OneOf) {
        let option = remedy.options.first().cloned().unwrap_or_default();
        return set_string_at_path(tree, path, &option).is_ok()
            || set_u64_at_path(tree, path, option.parse().unwrap_or(0)).is_ok()
            || set_number_at_path(tree, path, option.parse().unwrap_or(0.0)).is_ok();
    }
    let rounded = remedy.required.value.round();
    if rounded >= 0.0 && (remedy.required.value - rounded).abs() < 1e-9 {
        set_u64_at_path(tree, path, rounded as u64).is_ok() || set_number_at_path(tree, path, remedy.required.value).is_ok()
    } else {
        set_number_at_path(tree, path, remedy.required.value).is_ok() || set_u64_at_path(tree, path, remedy.required.value.round().max(0.0) as u64).is_ok()
    }
}

/// 🩹 Applies every applicable remedy of `check` to `doc` at once and evaluates the result; `None` when no remedy took.
fn apply_applicable(doc: &En1999Snapshot, check: &semio_s_plugin_norm_compliance::CheckResult) -> Option<En1999Snapshot> {
    let mut tree = serde_json::to_value(doc).expect("json");
    let applied = check.remedies.iter().filter(|r| r.applicable).fold(false, |any, remedy| write_remedy(&mut tree, remedy) | any);
    applied.then(|| serde_json::from_value(tree).expect("decode"))
}

/// 🩹 The remedy law is iterative: a remedy that lowers the check alone may leave it failing at a finite utilisation (a hot member whose resistance vanished only regains a finite resistance once the temperature drops), whose new remedies then clear it. Each round applies every applicable remedy of the current report and must lower the utilisation strictly.
fn cleared_by_rounds(doc: &En1999Snapshot, id: &str, rounds: usize, partials: &mut Vec<String>) -> bool {
    let mut current = doc.clone();
    for round in 0..rounds {
        let check = evaluate_structure(&current).checks.into_iter().find(|c| c.id == id);
        let Some(check) = check.filter(|c| matches!(c.status, CheckStatus::Fail)) else { return true };
        let Some(next) = apply_applicable(&current, &check) else {
            partials.push(format!("round {round}: no remedy took"));
            return false;
        };
        let after = evaluate_structure(&next).checks.into_iter().find(|c| c.id == id);
        match after {
            None => return true,
            Some(u) if !matches!(u.status, CheckStatus::Fail) => return true,
            Some(u) if u.utilization < check.utilization => {
                partials.push(format!("round {round}: utilisation {:.4}→{:.4}", check.utilization, u.utilization));
                current = next;
            }
            Some(u) => {
                partials.push(format!("round {round}: left failing without progress (u={:.4})", u.utilization));
                return false;
            }
        }
    }
    false
}

#[test]
fn gate_parity_every_fail_remedy_clears_status() {
    let doc = En1999Snapshot::noncompliant_multi_fail();
    let report = evaluate_structure(&doc);
    let listed = [
        "en1999.6.2.5.m.beam-fail",
        "en1999.6.2.5.mz.beam-fail",
        "en1999.6.2.6.v.beam-fail",
        "en1999.6.7.4.vb.beam-fail",
        "en1999.6.2.9.nm.beam-fail",
        "en1999.6.3.2.ltb.beam-fail",
        "en1999.6.3.3.int.beam-fail",
        "en1999.7.2.defl.beam-fail",
        "en1999.8.6.weld.weld-thin",
        "en1999.8.sls-freq.weld-thin",
        "en1999.8.5.bolt.bolt-short",
        "en1999.8.sls-freq.bolt-short",
        "en1999.1-2.fire.fire-hot",
        "en1999.1-3.fat.fat-hot",
        "en1999.1-3.fat.range.fat-hot",
        "en1999.1-4.bend.sheet-fail",
        "en1999.1-4.axial.sheet-fail",
        "en1999.1-4.nm.sheet-fail",
        "en1999.1-5.ring.shell-fail",
        "en1999.1-5.shear.shell-fail",
        "en1999.1-5.int.shell-fail",
    ];

    for id in listed {
        let check = report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing listed check {id}"));
        assert!(matches!(check.status, CheckStatus::Fail), "{id} should start Fail (got {:?})", check.status);
        assert!(!check.remedies.is_empty(), "{id}: Fail without remedies");
        let applicables: Vec<_> = check.remedies.iter().filter(|r| r.applicable).collect();
        assert!(!applicables.is_empty(), "{id}: no applicable remedies");
        let mut partials = Vec::new();
        let mut cleared = false;
        for (remedy_index, remedy) in applicables.iter().enumerate() {
            let mut tree = serde_json::to_value(&doc).expect("json");
            if !write_remedy(&mut tree, remedy) {
                partials.push(format!("remedy[{remedy_index}] apply failed at {}", remedy.target.path));
                continue;
            }
            let fixed: En1999Snapshot = serde_json::from_value(tree).expect("decode");
            match evaluate_structure(&fixed).checks.iter().find(|c| c.id == id) {
                Some(u) if matches!(u.status, CheckStatus::Fail) => partials.push(format!("remedy[{remedy_index}] left failing (u={:.4}→{:.4})", check.utilization, u.utilization)),
                _ => {
                    cleared = true;
                    break;
                }
            }
        }
        assert!(cleared || cleared_by_rounds(&doc, id, 4, &mut partials), "{id}: no applicable remedy cleared the Fail ({})", partials.join("; "));
    }
}
