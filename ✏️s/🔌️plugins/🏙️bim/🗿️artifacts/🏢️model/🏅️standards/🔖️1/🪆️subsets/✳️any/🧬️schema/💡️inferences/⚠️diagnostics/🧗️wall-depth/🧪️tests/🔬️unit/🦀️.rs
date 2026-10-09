use super::super::compute_diagnostics;
use crate::standards::v1::subsets::any::schema::inferences::diagnostics::{Diagnostic, DiagnosticCode, Severity};
use crate::standards::v1::subsets::any::schema::inferences::wall_layout::attach::testing::{gable_roof, model, rect, wall};
use serde_json::json;

fn codes(found: &[Diagnostic]) -> Vec<DiagnosticCode> {
    found.iter().map(|finding| finding.code).collect()
}

fn of(found: &[Diagnostic], code: DiagnosticCode) -> Option<&Diagnostic> {
    found.iter().find(|finding| finding.code == code)
}

#[semio_framework_async_macros::async_test]
async fn an_attach_that_reaches_only_part_of_the_axis_is_a_warning_with_the_covered_share() {
    let slab = json!({ "sl": { "storey": "st", "slab_type": "slt", "boundary": rect(0.0, 0.0, 4.0, 4.0), "holes": [], "offset": 0.0, "phase": "New", "name": "" } });
    let snapshot = model(json!({ "w": wall([0.0, 1.0, 8.0, 1.0], json!({ "Slab": { "slab": "sl", "offset": 0.5 } }), json!({})) }), json!({ "slabs": slab }));
    let found = compute_diagnostics(&snapshot);
    let finding = of(&found, DiagnosticCode::WallAttachUnreached).unwrap_or_else(|| panic!("{:?}", codes(&found)));
    assert_eq!((finding.severity, finding.elements.clone(), finding.values["covered"]), (Severity::Warning, vec!["w".to_string(), "sl".to_string()], 50.0));
    assert!(finding.text("en").is_some_and(|text| text.contains("50 %")) && finding.text("de").is_some_and(|text| text.contains("50 %")), "{:?} {:?}", finding.text("en"), finding.text("de"));
    assert!(of(&found, DiagnosticCode::RefAttachTarget).is_none() && of(&found, DiagnosticCode::WallAttachCycle).is_none());
}

#[semio_framework_async_macros::async_test]
async fn a_surface_below_the_base_lifts_the_top_and_says_so() {
    let snapshot = model(json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "Roof": { "roof": "r", "offset": -9.0 } }), json!({})) }), json!({ "roofs": gable_roof() }));
    assert!(of(&compute_diagnostics(&snapshot), DiagnosticCode::WallAttachCollapsed).is_some());
}

#[semio_framework_async_macros::async_test]
async fn a_fully_covered_attach_is_silent() {
    let snapshot = model(json!({ "w": wall([8.0, 0.0, 8.0, 6.0], json!({ "Roof": { "roof": "r", "offset": 0.0 } }), json!({})) }), json!({ "roofs": gable_roof() }));
    let found = compute_diagnostics(&snapshot);
    assert!(codes(&found).iter().all(|code| !matches!(code, DiagnosticCode::WallAttachUnreached | DiagnosticCode::WallAttachCollapsed | DiagnosticCode::WallAttachCycle | DiagnosticCode::RefAttachTarget)), "{:?}", codes(&found));
}

#[semio_framework_async_macros::async_test]
async fn dangling_attach_targets_and_sweep_hosts_are_reference_errors_in_both_languages() {
    let sweep = json!({ "ws": { "host": "w-gone", "side": "Left", "profile": { "Rectangle": { "width": 0.02, "depth": 0.1 } }, "height": 0.0, "inset": 0.0, "material": "paint", "name": "" } });
    let snapshot = model(json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "Roof": { "roof": "r-gone", "offset": 0.0 } }), json!({ "base_slab": "sl-gone" })) }), json!({ "wall_sweeps": sweep }));
    let found = compute_diagnostics(&snapshot);
    let targets: Vec<&Diagnostic> = found.iter().filter(|finding| finding.code == DiagnosticCode::RefAttachTarget).collect();
    assert_eq!(targets.len(), 2, "{:?}", codes(&found));
    assert_eq!(targets.iter().map(|finding| finding.missing[0].as_str()).collect::<Vec<_>>(), vec!["r-gone", "sl-gone"]);
    let host = of(&found, DiagnosticCode::RefWallSweepHost).expect("the sweep names a missing wall");
    assert_eq!((host.severity, host.missing.clone()), (Severity::Error, vec!["w-gone".to_string()]));
    assert!(host.text("en").is_some_and(|text| text.contains("w-gone")) && host.text("de").is_some_and(|text| text.contains("w-gone")));
}

#[semio_framework_async_macros::async_test]
async fn a_sweep_that_reaches_above_the_lowest_point_of_its_wall_is_reported() {
    let sweep = |height: f64| json!({ "ws": { "host": "w", "side": "Left", "profile": { "Rectangle": { "width": 0.02, "depth": 0.1 } }, "height": height, "inset": 0.0, "material": "paint", "name": "" } });
    let walls = || json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "StoreyTop": { "offset": 0.0 } }), json!({})) });
    let low = compute_diagnostics(&model(walls(), json!({ "wall_sweeps": sweep(0.0) })));
    assert!(of(&low, DiagnosticCode::WallSweepAboveWall).is_none(), "{:?}", codes(&low));
    let high = compute_diagnostics(&model(walls(), json!({ "wall_sweeps": sweep(2.95) })));
    let finding = of(&high, DiagnosticCode::WallSweepAboveWall).expect("the sweep ends above the wall top");
    assert!((finding.values["height"] - 3.05).abs() < 1e-9 && (finding.values["wall_height"] - 3.0).abs() < 1e-9, "{:?}", finding.values);
}

#[semio_framework_async_macros::async_test]
async fn a_reveal_that_leaves_no_room_for_the_frame_is_reported() {
    let walls = json!({ "w": wall([0.0, 0.0, 8.0, 0.0], json!({ "StoreyTop": { "offset": 0.0 } }), json!({})) });
    let opening = |reveal: f64| json!({ "o": { "host": "w", "kind": { "Window": { "window_type": "wi" } }, "offset": 4.0, "flip_hand": false, "flip_facing": false, "reveal_depth": reveal, "name": "" } });
    assert!(of(&compute_diagnostics(&model(walls.clone(), json!({ "openings": opening(0.2) }))), DiagnosticCode::OpeningRevealDepth).is_none(), "0.2 + 0.08 fits a wall of 0.3");
    let found = compute_diagnostics(&model(walls, json!({ "openings": opening(0.25) })));
    let finding = of(&found, DiagnosticCode::OpeningRevealDepth).expect("0.25 + 0.08 > 0.3");
    assert_eq!(finding.elements, vec!["o".to_string()]);
    assert!((finding.values["thickness"] - 0.3).abs() < 1e-9 && (finding.values["frame_depth"] - 0.08).abs() < 1e-9 && (finding.values["reveal"] - 0.25).abs() < 1e-9);
}

#[test]
fn every_wall_depth_code_has_both_languages_and_a_stable_slug() {
    let mine = [
        (DiagnosticCode::RefWallSweepHost, "reference.wall-sweep-host", Severity::Error),
        (DiagnosticCode::RefAttachTarget, "reference.attach-target", Severity::Error),
        (DiagnosticCode::WallAttachCycle, "wall.attach-cycle", Severity::Error),
        (DiagnosticCode::WallAttachUnreached, "wall.attach-unreached", Severity::Warning),
        (DiagnosticCode::WallAttachCollapsed, "wall.attach-collapsed", Severity::Warning),
        (DiagnosticCode::WallSweepAboveWall, "wall-sweep.above-wall", Severity::Warning),
        (DiagnosticCode::WallSweepNoRun, "wall-sweep.no-run", Severity::Info),
        (DiagnosticCode::OpeningRevealDepth, "opening.reveal-depth", Severity::Warning),
    ];
    for (code, slug, severity) in mine {
        assert_eq!((code.slug(), code.severity()), (slug, severity));
        assert!(DiagnosticCode::ALL.contains(&code));
        let finding = Diagnostic::new(code, &["a", "b"]).lacking("c").with("covered", 40.0).with("height", 1.0).with("wall_height", 2.0).with("reveal", 0.1).with("frame_depth", 0.1).with("thickness", 0.3);
        for locale in ["en", "de"] {
            let text = finding.text(locale).expect("a text");
            assert!(!text.contains('{'), "{slug} {locale}: {text}");
        }
        assert_ne!(finding.text("en"), finding.text("de"));
    }
}
