//! 🚦️ Gate parity of the EN 1999 compliance tests: every failing remedy clears its status. See [`super`].

use super::*;

#[test]
fn gate_parity_every_fail_remedy_clears_status() {
    let doc = En1999Snapshot::noncompliant_multi_fail();
    let report = evaluate_structure(&doc);
    let listed = [
        "en1999.6.2.5.mz.beam-fail",
        "en1999.6.3.1.fb.z.beam-fail",
        "en1999.6.3.1.lambda.y.beam-fail",
        "en1999.6.3.1.lambda.t.beam-fail",
        "en1999.7.2.defl.beam-fail",
        "en1999.7.2.stress.beam-fail",
        "en1999.7.2.sls-freq.beam-fail",
        "en1999.8.6.weld.weld-thin",
        "en1999.8.sls-freq.weld-thin",
        "en1999.8.5.bolt.bolt-short",
        "en1999.8.sls-freq.bolt-short",
        "en1999.1-2.fire.fire-hot",
        "en1999.1-4.bend.sheet-fail",
        "en1999.1-4.axial.sheet-fail",
        "en1999.1-4.nm.sheet-fail",
        "en1999.1-5.ring.shell-fail",
        "en1999.1-5.shear.shell-fail",
    ];
    for id in listed {
        let check = report
            .checks
            .iter()
            .find(|c| c.id == id)
            .unwrap_or_else(|| panic!("missing listed check {id}"));
        assert!(
            matches!(check.status, CheckStatus::Fail),
            "{id} should start Fail (got {:?})",
            check.status
        );
        assert!(
            !check.remedies.is_empty(),
            "{id}: Fail without remedies"
        );
        let applicables: Vec<(usize, &semio_s_plugin_norm_compliance::Remedy)> = check
            .remedies
            .iter()
            .enumerate()
            .filter(|(_, r)| r.applicable)
            .collect();
        assert!(!applicables.is_empty(), "{id}: no applicable remedies");
        let mut cleared = false;
        let mut partials = Vec::new();
        for &(remedy_index, remedy) in &applicables {
            let mut tree = serde_json::to_value(&doc).expect("json");
            let path = remedy.target.path.as_str();
            if path.is_empty() {
                partials.push(format!("remedy[{remedy_index}] empty path"));
                continue;
            }
            let ok = if matches!(remedy.bound, semio_s_plugin_norm_compliance::RemedyBound::OneOf) {
                let opt = remedy.options.first().cloned().unwrap_or_default();
                set_string_at_path(&mut tree, path, &opt).is_ok()
                    || set_u64_at_path(&mut tree, path, opt.parse().unwrap_or(0)).is_ok()
                    || set_number_at_path(&mut tree, path, opt.parse().unwrap_or(0.0)).is_ok()
            } else {
                // Prefer u64 for discrete counts (bolts.rows / boltsPerRow) like apply_remedy_edit.
                let rounded = remedy.required.value.round();
                let as_u64 = rounded >= 0.0 && (remedy.required.value - rounded).abs() < 1e-9;
                if as_u64 {
                    set_u64_at_path(&mut tree, path, rounded as u64).is_ok()
                        || set_number_at_path(&mut tree, path, remedy.required.value).is_ok()
                } else {
                    set_number_at_path(&mut tree, path, remedy.required.value).is_ok()
                        || set_u64_at_path(&mut tree, path, remedy.required.value.round().max(0.0) as u64).is_ok()
                }
            };
            if !ok {
                partials.push(format!("remedy[{remedy_index}] apply failed at {path}"));
                continue;
            }
            let fixed: En1999Snapshot = serde_json::from_value(tree).expect("decode");
            let after = evaluate_structure(&fixed);
            let updated = after.checks.iter().find(|c| c.id == id);
            match updated {
                None => {
                    cleared = true;
                    break;
                }
                Some(u) if !matches!(u.status, CheckStatus::Fail) => {
                    cleared = true;
                    break;
                }
                Some(u) if u.utilization < check.utilization => {
                    partials.push(format!(
                        "remedy[{remedy_index}] only lowered utilization {:.4}→{:.4}",
                        check.utilization, u.utilization
                    ));
                }
                Some(u) if u.utilization > check.utilization + 1e-9 => {
                    partials.push(format!(
                        "remedy[{remedy_index}] raised utilization {:.4}→{:.4}",
                        check.utilization, u.utilization
                    ));
                }
                Some(u) => {
                    partials.push(format!(
                        "remedy[{remedy_index}] left failing (u={:.4})",
                        u.utilization
                    ));
                }
            }
        }
        if !cleared && applicables.len() > 1 {
            let mut tree = serde_json::to_value(&doc).expect("json");
            let mut sequential_ok = true;
            for (remedy_index, remedy) in &applicables {
                let path = remedy.target.path.as_str();
                let ok = if matches!(remedy.bound, semio_s_plugin_norm_compliance::RemedyBound::OneOf) {
                    let opt = remedy.options.first().cloned().unwrap_or_default();
                    set_string_at_path(&mut tree, path, &opt).is_ok()
                        || set_u64_at_path(&mut tree, path, opt.parse().unwrap_or(0)).is_ok()
                        || set_number_at_path(&mut tree, path, opt.parse().unwrap_or(0.0)).is_ok()
                } else {
                    let rounded = remedy.required.value.round();
                    let as_u64 = rounded >= 0.0 && (remedy.required.value - rounded).abs() < 1e-9;
                    if as_u64 {
                        set_u64_at_path(&mut tree, path, rounded as u64).is_ok()
                            || set_number_at_path(&mut tree, path, remedy.required.value).is_ok()
                    } else {
                        set_number_at_path(&mut tree, path, remedy.required.value).is_ok()
                            || set_u64_at_path(&mut tree, path, remedy.required.value.round().max(0.0) as u64).is_ok()
                    }
                };
                if !ok {
                    sequential_ok = false;
                    partials.push(format!("sequential remedy[{remedy_index}] apply failed"));
                    break;
                }
            }
            if sequential_ok {
                if let Ok(fixed) = serde_json::from_value::<En1999Snapshot>(tree) {
                    let after = evaluate_structure(&fixed);
                    match after.checks.iter().find(|c| c.id == id) {
                        None => cleared = true,
                        Some(u) if !matches!(u.status, CheckStatus::Fail) => cleared = true,
                        Some(u) => partials.push(format!(
                            "sequential applicables left failing (u={:.4})",
                            u.utilization
                        )),
                        _ => {}
                    }
                }
            }
        }
        assert!(
            cleared,
            "{}: no applicable remedy cleared the Fail ({})",
            id,
            partials.join("; ")
        );
    }
}
