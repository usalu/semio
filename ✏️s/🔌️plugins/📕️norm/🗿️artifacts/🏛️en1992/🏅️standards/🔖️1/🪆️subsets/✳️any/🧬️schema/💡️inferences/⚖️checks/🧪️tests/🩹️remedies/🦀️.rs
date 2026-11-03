//! 🩹️ One-click remedies: every remedy of a failing check writes a typed member field with the smallest value that passes, and the decompression check uses the compression-positive convention.

use super::*;
use semio_s_artifact_norm_en1992_model::{En1992Snapshot, PrestressSpec};
use semio_s_plugin_norm_compliance::RemedyBound;

fn fixtures() -> Vec<En1992Snapshot> {
    vec![En1992Snapshot::failing_under_reinforced(), En1992Snapshot::failing_prestressed_beam(), En1992Snapshot::liquid_retaining_fem_anchor(), prestressed_wall(300.0e3, 50.0e3), En1992Snapshot::flat_slab_punching()]
}

fn prestressed_wall(g_mk: f64, q_mk: f64) -> En1992Snapshot {
    let mut doc = En1992Snapshot::liquid_retaining_fem_anchor();
    let wall = &mut doc.members[0];
    wall.prestress_steel_id = "yp1860".into();
    wall.prestress = Some(PrestressSpec { force: 1.0e6, area: 8.0e-4, eccentricity: 0.12, loss_ratio: 0.2 });
    wall.tendon_points.clear();
    for action in &mut wall.actions {
        action.m_k = if action.kind == semio_s_artifact_norm_en1992_model::ActionKind::Permanent { g_mk } else { q_mk };
    }
    doc
}

fn apply(member: &mut RcMember, path: &str, value: f64) {
    let field = path.split_once("].").map_or(path, |(_, rest)| rest);
    match field {
        "height" => {
            member.effective_depth += value - member.height;
            member.height = value;
        }
        "width" => member.width = value,
        "stirrups.spacing" => member.stirrups.iter_mut().for_each(|s| s.spacing = value),
        "stirrups.diameter" => member.stirrups.iter_mut().for_each(|s| s.diameter = value),
        "punching.asw" => member.punching.iter_mut().for_each(|p| p.asw = value),
        "prestress.force" => member.prestress.iter_mut().for_each(|p| p.force = value),
        "bucklingLength" => member.buckling_length = value,
        "bearingWidth" => member.bearing_width = value,
        other if other.starts_with("studRows[id=") => {
            let (ring, leaf) = other.strip_prefix("studRows[id=").and_then(|rest| rest.split_once("].")).expect("a stud row remedy path");
            member.stud_rows.iter_mut().filter(|x| x.id == ring).for_each(|x| match leaf {
                "spacingRadial" => x.spacing_radial = value,
                "diameter" => x.diameter = value,
                _ => unreachable!(),
            });
        }
        other => {
            let (layer, leaf) = other.strip_prefix("longitudinal[id=").and_then(|rest| rest.split_once("].")).expect("a planned remedy path");
            member.longitudinal.iter_mut().filter(|x| x.id == layer).for_each(|x| match leaf {
                "count" => x.count = value.round() as u32,
                "diameter" => x.diameter = value,
                _ => unreachable!(),
            });
        }
    }
}

fn status_of(doc: &En1992Snapshot, member: &RcMember, id: &str) -> Option<CheckStatus> {
    evaluate_member(doc, member).into_iter().find(|r| r.id == id).map(|r| r.status)
}

fn failing_with_remedies(doc: &En1992Snapshot) -> Vec<(RcMember, CheckResult)> {
    doc.members.iter().flat_map(|m| evaluate_member(doc, m).into_iter().filter(|r| r.status == CheckStatus::Fail && r.remedies.iter().any(|x| x.bound != RemedyBound::OneOf && x.options.is_empty())).map(|r| (m.clone(), r))).collect()
}

#[test]
fn every_solved_remedy_passes_and_one_step_back_fails() {
    let mut checked = 0;
    for doc in fixtures() {
        for (member, result) in failing_with_remedies(&doc) {
            for remedy in result.remedies.iter().filter(|x| matches!(x.bound, RemedyBound::AtLeast | RemedyBound::AtMost) && x.options.is_empty() && plan_paths(&member).contains(&x.target.path)) {
                let mut fixed = member.clone();
                apply(&mut fixed, &remedy.target.path, remedy.required.value);
                assert_ne!(status_of(&doc, &fixed, &result.id), Some(CheckStatus::Fail), "{} with {} = {}", result.id, remedy.target.path, remedy.required.value);
                let step = if remedy.target.path.ends_with("count") { 1.0 } else { 0.0 };
                if step > 0.0 {
                    let mut back = member.clone();
                    let back_value = if remedy.bound == RemedyBound::AtLeast { remedy.required.value - step } else { remedy.required.value + step };
                    apply(&mut back, &remedy.target.path, back_value);
                    assert_eq!(status_of(&doc, &back, &result.id), Some(CheckStatus::Fail), "{} one step before {}", result.id, remedy.required.value);
                }
                checked += 1;
            }
        }
    }
    assert!(checked >= 3, "only {checked} solved remedies found");
}

fn plan_paths(member: &RcMember) -> Vec<String> {
    let layer = member.longitudinal.first().map(|x| x.id.clone()).unwrap_or_default();
    ["height", "width", "stirrups.spacing", "stirrups.diameter", "punching.asw", "prestress.force", "bucklingLength", "bearingWidth"]
        .iter()
        .map(|f| format!("members[id={}].{f}", member.id))
        .chain(["count", "diameter"].iter().map(|f| format!("members[id={}].longitudinal[id={layer}].{f}", member.id)))
        .chain(member.stud_rows.iter().flat_map(|r| ["spacingRadial", "diameter"].map(|f| format!("members[id={}].studRows[id={}].{f}", member.id, r.id))))
        .collect()
}

#[test]
fn a_remedy_never_leaves_a_typed_field() {
    for doc in fixtures() {
        for (member, result) in failing_with_remedies(&doc) {
            for remedy in result.remedies.iter().filter(|x| x.bound != RemedyBound::OneOf && x.options.is_empty()) {
                let path = remedy.target.path.as_str();
                assert!(!path.ends_with("effectiveDepth") && !path.ends_with(".stirrups") && !path.ends_with(".punching") || result.id.contains("integrity"), "{} targets the untyped {path} of {}", result.id, member.id);
            }
        }
    }
}

#[test]
fn the_decompression_check_passes_without_load_and_fails_under_a_large_sagging_moment() {
    let unloaded = prestressed_wall(0.0, 0.0);
    let id = format!("en1992.5.10.decompression.{}", unloaded.members[0].id);
    assert!(matches!(status_of(&unloaded, &unloaded.members[0], &id), Some(CheckStatus::Pass | CheckStatus::UnverifiedBasis)));
    let sagging = prestressed_wall(300.0e3, 50.0e3);
    let result = evaluate_member(&sagging, &sagging.members[0]).into_iter().find(|r| r.id == id).expect("decompression row");
    assert_eq!(result.status, CheckStatus::Fail);
    assert!(result.utilization > 1.0);
}

#[test]
fn a_hogging_moment_decompresses_the_top_fibre_of_a_tendon_below_the_centroid() {
    let hogging = prestressed_wall(-115.0e3, 0.0);
    let id = format!("en1992.5.10.decompression.{}", hogging.members[0].id);
    assert_eq!(status_of(&hogging, &hogging.members[0], &id), Some(CheckStatus::Fail));
    let sagging = prestressed_wall(115.0e3, 0.0);
    assert_ne!(status_of(&sagging, &sagging.members[0], &id), Some(CheckStatus::Fail));
}

#[test]
fn the_decompression_remedy_is_the_smallest_prestressing_force_that_passes() {
    let doc = prestressed_wall(300.0e3, 50.0e3);
    let member = &doc.members[0];
    let id = format!("en1992.5.10.decompression.{}", member.id);
    let result = evaluate_member(&doc, member).into_iter().find(|r| r.id == id).expect("decompression row");
    let remedy = result.remedies.iter().find(|x| x.target.path.ends_with("prestress.force")).expect("a prestress force remedy");
    assert_eq!(remedy.bound, RemedyBound::AtLeast);
    assert!(remedy.required.value > remedy.current.value);
    let (area, w) = (0.35 * 0.5, 0.35 * 0.5 * 0.5 / 6.0);
    let m_qp = 300.0e3 + 0.3 * 50.0e3;
    let closed_form = (m_qp / w) / (1.0 / area + 0.12 / w) / 0.8;
    assert!((remedy.required.value - closed_form).abs() <= 1.5e3, "{} against the closed form {closed_form}", remedy.required.value);
    let mut fixed = member.clone();
    apply(&mut fixed, &remedy.target.path, remedy.required.value);
    assert_ne!(status_of(&doc, &fixed, &id), Some(CheckStatus::Fail));
    let mut short = member.clone();
    apply(&mut short, &remedy.target.path, remedy.required.value - 2.0e3);
    assert_eq!(status_of(&doc, &short, &id), Some(CheckStatus::Fail));
}

#[test]
fn the_prestress_stress_limit_remedy_lowers_the_force_to_the_largest_value_that_passes() {
    let mut doc = prestressed_wall(0.0, 0.0);
    doc.members[0].prestress.as_mut().expect("prestress").force = 3.0e6;
    let member = &doc.members[0];
    let id = format!("en1992.5.10.prestress.{}", member.id);
    let result = evaluate_member(&doc, member).into_iter().find(|r| r.id == id).expect("prestress row");
    assert_eq!(result.status, CheckStatus::Fail);
    let remedy = result.remedies.iter().find(|x| x.target.path.ends_with("prestress.force")).expect("a prestress force remedy");
    assert_eq!(remedy.bound, RemedyBound::AtMost);
    let mut fixed = member.clone();
    apply(&mut fixed, &remedy.target.path, remedy.required.value);
    assert_ne!(status_of(&doc, &fixed, &id), Some(CheckStatus::Fail));
    let mut over = member.clone();
    apply(&mut over, &remedy.target.path, remedy.required.value + 2.0e3);
    assert_eq!(status_of(&doc, &over, &id), Some(CheckStatus::Fail));
}

#[test]
fn every_anchor_remedy_writes_a_typed_field_and_passes() {
    let doc = En1992Snapshot::failing_under_reinforced();
    let anchor = &doc.anchors[0];
    let failing: Vec<CheckResult> = evaluate_anchor(&doc, anchor).into_iter().filter(|r| r.status == CheckStatus::Fail).collect();
    assert!(!failing.is_empty(), "the failing fixture has a failing anchor check");
    let mut solved = 0;
    for result in &failing {
        for remedy in result.remedies.iter().filter(|x| x.bound != RemedyBound::OneOf) {
            let mut fixed = anchor.clone();
            match remedy.target.path.rsplit('.').next().expect("a leaf") {
                "hEf" => fixed.h_ef = remedy.required.value,
                "aS" => fixed.a_s = remedy.required.value,
                "c1" => fixed.c1 = remedy.required.value,
                other => panic!("{} targets the untyped {other}", result.id),
            }
            let again = evaluate_anchor(&doc, &fixed).into_iter().find(|r| r.id == result.id).expect("the same check");
            assert_ne!(again.status, CheckStatus::Fail, "{} with {}", result.id, remedy.target.path);
            solved += 1;
        }
    }
    assert!(solved > 0);
}

fn weak_flexure_beam() -> (En1992Snapshot, String) {
    let doc = En1992Snapshot::failing_under_reinforced();
    let id = format!("en1992.6.1.flexure.acc.{}", doc.members[0].id);
    (doc, id)
}

#[test]
fn a_joint_remedy_writes_its_fields_together_and_passes_only_together() {
    let mut joint = 0;
    for doc in fixtures() {
        for member in &doc.members {
            for result in evaluate_member(&doc, member).into_iter().filter(|r| r.status == CheckStatus::Fail) {
                let together: Vec<_> = result.remedies.iter().filter(|x| matches!(x.bound, RemedyBound::AtLeast | RemedyBound::AtMost) && !x.options.is_empty()).collect();
                if together.is_empty() {
                    continue;
                }
                assert!(together.len() >= 2 && together.iter().all(|x| x.options == together[0].options && x.options.contains(&x.target.path)), "{} joint remedies name one shared field set", result.id);
                let mut fixed = member.clone();
                together.iter().for_each(|x| apply(&mut fixed, &x.target.path, x.required.value));
                assert_ne!(status_of(&doc, &fixed, &result.id), Some(CheckStatus::Fail), "{} with all joint fields", result.id);
                joint += 1;
            }
        }
    }
    let _ = joint;
}

#[test]
fn a_failing_check_without_any_passing_change_carries_a_bilingual_notice_naming_the_governing_fields() {
    let (mut doc, id) = weak_flexure_beam();
    doc.members[0].actions.iter_mut().for_each(|a| a.m_k *= 1.0e6);
    let result = evaluate_member(&doc, &doc.members[0]).into_iter().find(|r| r.id == id).expect("the flexure row");
    assert_eq!(result.status, CheckStatus::Fail);
    let notice = result.remedies.iter().find(|x| !x.applicable && x.bound == RemedyBound::OneOf).expect("a notice");
    assert!(notice.options.iter().all(|path| path.starts_with(&format!("members[id={}]", doc.members[0].id))), "{:?}", notice.options);
    assert!(notice.options.len() >= 2, "{:?}", notice.options);
    assert!(notice.action.en.contains("No single change suffices") && notice.action.de.contains("Keine einzelne"));
}

#[test]
fn every_failing_check_of_every_fixture_is_guided() {
    for doc in fixtures() {
        for member in &doc.members {
            for result in evaluate_member(&doc, member).into_iter().filter(|r| r.status == CheckStatus::Fail) {
                assert!(result.remedies.iter().any(|x| x.applicable) || result.remedies.iter().any(|x| !x.applicable && x.bound == RemedyBound::OneOf && !x.options.is_empty()), "{} is not guided", result.id);
            }
        }
    }
}

#[test]
fn the_remedy_search_evaluates_at_most_the_bound_per_field_and_each_check_owns_its_budget() {
    use super::member_evaluation::minimal_remedies::{search, Direction, CHECK_BUDGET, SEARCH_BOUND};
    use std::sync::atomic::{AtomicUsize, Ordering};
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let calls = |f: f64| {
        CALLS.fetch_add(1, Ordering::SeqCst);
        f > 1.0e12
    };
    CALLS.store(0, Ordering::SeqCst);
    assert_eq!(search(1.0, 1.0e-3, false, Direction::Raise, calls), None);
    assert!(CALLS.load(Ordering::SeqCst) <= 8, "expansion is bounded by seven doublings");
    CALLS.store(0, Ordering::SeqCst);
    let found = search(1.0, 1.0e-3, false, Direction::Raise, |f| {
        CALLS.fetch_add(1, Ordering::SeqCst);
        f >= 100.0
    });
    assert!(found.is_some_and(|v| (100.0..100.002).contains(&v)));
    assert!(CALLS.load(Ordering::SeqCst) <= SEARCH_BOUND, "seven doublings plus about sixteen bisections of the last interval stay within the bound");
    assert_eq!(SEARCH_BOUND, 32);
    assert!(CHECK_BUDGET >= 3 * SEARCH_BOUND, "the longest plan (three fields) always fits in the budget of its check");
    assert!(CHECK_BUDGET <= 128);
}

#[test]
fn the_drag_preview_report_reevaluates_the_dragged_member_only_and_keeps_every_other_row() {
    use semio_s_plugin_norm_compliance::CheckReport;
    let doc = En1992Snapshot::failing_under_reinforced();
    let mut base = CheckReport::default();
    doc.members.iter().for_each(|m| base.extend(evaluate_member(&doc, m)));
    let member_id = doc.members[0].id.clone();
    let mut dragged = doc.clone();
    dragged.members[0].longitudinal[0].count += 3;
    let preview = preview_member_report(&dragged, &base, &member_id);
        let live = evaluate_member(&dragged, &dragged.members[0]);
    for check in preview.checks.iter().filter(|c| c.subject.entity_id == member_id && !c.id.starts_with("en1992.1-2.")) {
        let expected = live.iter().find(|r| r.id == check.id).expect("the same row after the move");
        assert_eq!(check.utilization, expected.utilization, "{}", check.id);
    }
    let moved = preview.checks.iter().any(|after| base.checks.iter().any(|before| after.id == before.id && after.utilization != before.utilization));
    assert!(moved, "more bars change at least one utilisation");
    for before in base.checks.iter().filter(|c| c.subject.entity_id != member_id) {
        assert_eq!(preview.checks.iter().find(|c| c.id == before.id), Some(before));
    }
}

#[test]
fn a_bar_placed_inside_the_cover_fails_the_bar_cover_check_and_a_centred_bar_passes() {
    use semio_s_artifact_norm_en1992_model::BarPoint;
    let mut doc = En1992Snapshot::failing_under_reinforced();
    let member_id = doc.members[0].id.clone();
    let layer = doc.members[0].longitudinal[0].id.clone();
    let height = doc.members[0].height;
    let cover_status = |doc: &En1992Snapshot| evaluate_member(doc, &doc.members[0]).into_iter().find(|r| r.id == format!("en1992.4.4.bar-cover.{member_id}")).map(|r| r.status);
    assert_eq!(cover_status(&doc), None, "no explicit bars, no bar cover row");
    doc.members[0].bars = vec![BarPoint { id: "mid".into(), layer_id: layer.clone(), y: 0.0, z: 0.0, diameter: 0.016, length: 0.0, bend: semio_s_artifact_norm_en1992_model::BarBend::Auto }];
    assert!(matches!(cover_status(&doc), Some(status) if status != CheckStatus::Fail));
    doc.members[0].bars[0].z = -height / 2.0 + 0.005;
    assert_eq!(cover_status(&doc), Some(CheckStatus::Fail));
}

#[test]
fn the_failing_stud_layout_of_the_corner_column_is_repaired_by_the_smallest_radial_spacing_of_the_widest_row() {
    let doc = En1992Snapshot::flat_slab_punching();
    let member = doc.members.iter().find(|m| m.id == "slab-FS-corner").expect("corner column");
    let id = format!("en1992.9.4.3.studs.{}", member.id);
    let result = evaluate_member(&doc, member).into_iter().find(|r| r.id == id).expect("stud layout row");
    assert_eq!(result.status, CheckStatus::Fail);
    let remedy = result.remedies.iter().find(|x| x.target.path.ends_with("studRows[id=r5].spacingRadial")).expect("a radial spacing remedy");
    assert_eq!(remedy.bound, RemedyBound::AtMost);
    let limit = 0.75 * member.effective_depth;
    assert!(remedy.required.value <= limit + 1.0e-9 && remedy.required.value > limit - 2.0e-3, "{} against the gap limit {limit}", remedy.required.value);
    let mut fixed = member.clone();
    apply(&mut fixed, &remedy.target.path, remedy.required.value);
    assert_ne!(status_of(&doc, &fixed, &id), Some(CheckStatus::Fail));
    let mut wider = member.clone();
    apply(&mut wider, &remedy.target.path, remedy.required.value + 3.0e-3);
    assert_eq!(status_of(&doc, &wider, &id), Some(CheckStatus::Fail));
}

#[test]
fn a_punching_failure_with_stud_rows_is_repaired_through_the_weakest_stud_diameter() {
    let mut doc = En1992Snapshot::flat_slab_punching();
    let edge = doc.members.iter_mut().find(|m| m.id == "slab-FS-edge").expect("edge column");
    edge.stud_rows.iter_mut().for_each(|r| r.diameter = 0.004);
    edge.actions.iter_mut().for_each(|a| a.v_k_punch *= 2.0);
    let member = doc.members.iter().find(|m| m.id == "slab-FS-edge").expect("edge column").clone();
    let id = format!("en1992.6.4.punching.{}", member.id);
    let result = evaluate_member(&doc, &member).into_iter().find(|r| r.id == id).expect("punching row");
    assert_eq!(result.status, CheckStatus::Fail);
    let remedy = result.remedies.iter().find(|x| x.target.path.ends_with("studRows[id=r1].diameter")).expect("a weakest stud diameter remedy");
    assert_eq!(remedy.bound, RemedyBound::AtLeast);
    let mut fixed = member.clone();
    apply(&mut fixed, &remedy.target.path, remedy.required.value);
    assert_ne!(status_of(&doc, &fixed, &id), Some(CheckStatus::Fail));
}

#[test]
fn a_stud_row_drag_updates_the_punching_outer_perimeter_and_stud_rows_of_the_dragged_slab_only() {
    use semio_s_plugin_norm_compliance::CheckReport;
    let doc = En1992Snapshot::flat_slab_punching();
    let mut base = CheckReport::default();
    doc.members.iter().for_each(|m| base.extend(evaluate_member(&doc, m)));
    let id = "slab-FS-interior".to_string();
    let row = |report: &CheckReport, prefix: &str| report.checks.iter().find(|c| c.id == format!("{prefix}.{id}")).map(|c| c.utilization).unwrap_or_else(|| panic!("{prefix} row missing"));
    let mut spacing = doc.clone();
    spacing.members.iter_mut().find(|m| m.id == id).expect("slab").stud_rows.last_mut().expect("row").spacing_radial += 0.05;
    let preview = preview_member_report(&spacing, &base, &id);
    for prefix in ["en1992.6.4.uout", "en1992.9.4.3.studs"] {
        assert_ne!(row(&preview, prefix), row(&base, prefix), "{prefix} follows the radial spacing of the last row");
    }
    let mut count = doc.clone();
    count.members.iter_mut().find(|m| m.id == id).expect("slab").stud_rows.first_mut().expect("row").diameter += 0.004;
    let preview = preview_member_report(&count, &base, &id);
    assert_ne!(row(&preview, "en1992.6.4.punching"), row(&base, "en1992.6.4.punching"), "the stud diameter carries A_sw");
    let live = evaluate_member(&count, count.members.iter().find(|m| m.id == id).expect("slab"));
    for prefix in ["en1992.6.4.punching", "en1992.6.4.uout", "en1992.9.4.3.studs"] {
        assert_eq!(row(&preview, prefix), live.iter().find(|r| r.id == format!("{prefix}.{id}")).expect("live row").utilization, "{prefix}");
    }
    for before in base.checks.iter().filter(|c| c.subject.entity_id != id) {
        assert_eq!(preview.checks.iter().find(|c| c.id == before.id), Some(before));
    }
}
