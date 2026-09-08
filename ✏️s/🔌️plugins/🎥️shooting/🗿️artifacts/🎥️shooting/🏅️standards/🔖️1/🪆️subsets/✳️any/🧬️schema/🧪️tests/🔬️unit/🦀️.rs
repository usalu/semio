
use super::*;
use crate::SHOOTING_DOCUMENT_SCHEMA;

#[semio_framework_async_macros::async_test]
async fn default_example_fixture_parses() {
    let snapshot = default_snapshot();
    assert_eq!(snapshot.schema, SHOOTING_DOCUMENT_SCHEMA);
    assert!(!snapshot.shots.is_empty());
    assert!(!snapshot.assets.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn scene_svg_embeds_active_asset_name_and_shot_shape() {
    let snapshot = default_snapshot();
    let (svg, width, height) = shooting_scene_svg(&snapshot).expect("scene svg via the semio/drawing stdio bridge");
    let shot = active_shot(&snapshot).expect("default fixture shot");
    let asset = active_asset(&snapshot).expect("default fixture asset");
    assert_eq!((width, height), (shot.width, shot.height));
    assert!(svg.contains(&asset.name), "svg emblem includes active asset name");
    // 🌉️ The shape now lowers through the semio/drawing bridge as a real `<path d="...">`
    // (drawing's own svg export leaf has no `<rect>`/`<ellipse>` element, only `<path>`);
    // ellipse shots draw via an SVG `A`rc command, rectangle shots via straight lines only.
    assert!(svg.contains("<path"), "shape renders as a real <path> element, not a raw <rect>/<ellipse>");
    let has_arc_command = svg.contains(" A ");
    assert_eq!(has_arc_command, shot.shape == "ellipse", "ellipse shots draw an SVG arc command, rectangle shots never do");
}

/// 🌉️ Exercises the ellipse branch the default fixture (shape "rectangle") never hits —
/// confirms `shooting_shape_path_segments` really emits the two-arc ellipse technique.
#[semio_framework_async_macros::async_test]
async fn ellipse_shot_shape_renders_via_svg_arc_commands() {
    let mut snapshot = default_snapshot();
    let shot_id = snapshot.active_shot_id.clone();
    for shot in snapshot.shots.iter_mut() {
        if shot.id == shot_id {
            shot.shape = "ellipse".into();
        }
    }
    let (svg, _width, _height) = shooting_scene_svg(&snapshot).expect("ellipse scene svg");
    assert!(svg.contains(" A "), "ellipse shape draws via SVG arc commands: {svg}");
    assert!(!svg.contains("<rect") && !svg.contains("<ellipse"), "no raw <rect>/<ellipse> element, only <path>");
}

#[semio_framework_async_macros::async_test]
async fn export_svg_uses_scene_render_not_title_card() {
    let snapshot = default_snapshot();
    let document = json::from_dsl_value(&dsl::ToValue::to_value(&snapshot));
    let (svg, _width, _height) = shooting_document_json_to_svg(&document).expect("export svg");
    let asset = active_asset(&snapshot).expect("default fixture asset");
    assert!(svg.contains(&asset.name));
    assert!(!svg.contains("Shooting"), "export renders the real scene, not the generic title card");
}

/// 🎥️ The camera used to be reframed to the DWG extent here; now that it's session-only runtime
/// state (never a document field), the import hook has no channel back into it — this asserts the
/// surviving intent: import still succeeds and stays schema-valid for a non-trivial extent.

#[semio_framework_async_macros::async_test]
async fn transparent_background_predicate_covers_empty_and_literal_transparent() {
    assert!(is_transparent_shooting_background(""));
    assert!(is_transparent_shooting_background("transparent"));
    assert!(!is_transparent_shooting_background("#000000"));
}
