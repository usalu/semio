use super::*;

/// ⚖️ LAW: an armed re-dispatch always carries BOTH halves of the window address, because the
/// shell redispatches an `Effect::DispatchAction` under whichever window is current and the
/// retained route matches the payload against the ViewModel roster, never against that window.
#[test]
fn a_rearm_carries_the_window_id_and_kind_it_was_addressed_to() {
    let Effect::DispatchAction { action, args, .. } = rearm("preview-1", "procedural-view-preview", 105) else {
        panic!("a re-arm must be a dispatch effect");
    };
    assert_eq!(action, "flowEvalTick");
    let args = args.expect("an addressed re-arm carries args");
    assert_eq!(args.get("windowId").and_then(dsl::DslValue::as_str), Some("preview-1"));
    assert_eq!(args.get("windowKindId").and_then(dsl::DslValue::as_str), Some("procedural-view-preview"));
}

/// ⚖️ LAW: only a roster entry whose kind the surface declared as a preview may be armed, and a
/// surface that declares no preview kind at all arms nothing — the served-app stall was exactly an
/// address the route could only ever refuse.
#[test]
fn only_declared_preview_kinds_are_armed_from_a_roster() {
    use semio_framework_plugin::ViewWindowInstance;
    let view = ViewModel {
        window_instances: vec![
            ViewWindowInstance { id: "main".into(), window_kind_id: "procedural-main".into() },
            ViewWindowInstance { id: "view-preview".into(), window_kind_id: "procedural-view-preview".into() },
        ],
        ..Default::default()
    };
    let viewer_kinds: &[&'static str] = &["procedural-view-preview"];
    assert_eq!(attached_preview_windows(Some(&view), viewer_kinds), vec![("view-preview", "procedural-view-preview")]);
    assert!(attached_preview_windows(Some(&view), &[]).is_empty(), "a surface with no preview kind arms nothing");
    assert!(attached_preview_windows(None, viewer_kinds).is_empty(), "no roster at all arms nothing");
    assert_eq!(preview_kind("procedural-main", viewer_kinds), None, "a flow window holds no evaluation publication");
}

/// ⚖️ LAW: the deflection ladder is ONE table — an editor mesh and a viewer mesh of the same handle
/// at the same LOD are tessellated to the same tolerance, so the two surfaces cannot drift.
#[test]
fn the_lod_deflection_ladder_is_shared_and_monotone() {
    assert!(preview_tolerance("coarse") > preview_tolerance("medium"));
    assert!(preview_tolerance("medium") > preview_tolerance("fine"));
    assert_eq!(preview_tolerance(""), preview_tolerance("medium"), "an unset LOD is medium");
}
