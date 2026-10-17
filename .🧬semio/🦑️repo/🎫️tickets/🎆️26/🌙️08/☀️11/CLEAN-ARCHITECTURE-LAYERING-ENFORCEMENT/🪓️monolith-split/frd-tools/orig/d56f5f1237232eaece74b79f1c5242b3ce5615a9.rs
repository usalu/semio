//! 🧪️ interaction tests (🔬️unit): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::WorldEnvironmentRecord;
use semio_framework_os_infinite_world_cursors::{selection_method_wire_str, WORLD_INTERACTION_DOMAIN_ID};
use semio_framework_os_infinite_world_scene::{mesh_id_from_url, srgb_to_linear};
use semio_framework_os_infinite_world_interaction::{brush_preview_mesh_id, environment_clear_color, world_interaction_definition};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_interaction::merge_u32_ids;
use semio_framework::{HierarchyProvider, MergeMode, SelectionMethod};
use ui_wgpu::wgpu::Rgba;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.
fn read_static(path_from_manifest: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path_from_manifest);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_str())
}

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(read_static(path));
    }
    Box::leak(joined.into_boxed_str())
}

/// 🎯️ The component-pick merge speaks the ONE `MergeMode` vocabulary (`🕹️interaction/🧬️schema/🔣️.json`,
/// fixture `🎯️merge-modes.json`) — ticket 26/09/09/PROCEDURAL-3D-END-TO-END deleted this file's
/// private `add`/`remove`/`toggle` spelling, which is unrepresentable now the mode is decoded.
#[test]
fn merge_u32_ids_speaks_the_one_schema_merge_vocabulary() {
    assert_eq!(merge_u32_ids(&["1".into()], &["2".into()], MergeMode::Additive), vec![1, 2]);
    assert_eq!(merge_u32_ids(&["1".into(), "2".into()], &["2".into(), "3".into()], MergeMode::Invertive), vec![1, 3]);
    assert_eq!(merge_u32_ids(&["1".into(), "2".into()], &["2".into()], MergeMode::Subtractive), vec![1]);
    assert_eq!(merge_u32_ids(&["1".into()], &["2".into()], MergeMode::Replace), vec![2]);
    for word in ["add", "remove", "toggle"] {
        assert!(MergeMode::from_wire_label(word).is_none(), "the deleted word '{word}' must decode to nothing");
    }
}

//#region EnvironmentTests
/// 🎨️ Linear, not raw sRGB: the world pass renders into an sRGB view, so a wire colour is
/// linearized on the way in exactly as React's `new Color(hex)` linearizes it.
#[test]
fn environment_clear_color_uses_opaque_background() {
    let environment = WorldEnvironmentRecord { background: Some("#112233".into()), ..Default::default() };
    let theme_clear = Rgba::new(0.0, 0.0, 0.0, 1.0);
    let clear = environment_clear_color(&environment, theme_clear);
    assert!((clear.r - srgb_to_linear(0x11 as f32 / 255.0)).abs() < 1e-4);
    assert!((clear.g - srgb_to_linear(0x22 as f32 / 255.0)).abs() < 1e-4);
    assert!((clear.b - srgb_to_linear(0x33 as f32 / 255.0)).abs() < 1e-4);
}

#[test]
fn environment_clear_color_falls_back_when_transparent_or_absent() {
    let theme_clear = Rgba::new(0.1, 0.2, 0.3, 1.0);
    let transparent = WorldEnvironmentRecord { background: Some("transparent".into()), ..Default::default() };
    let absent = WorldEnvironmentRecord::default();
    assert_eq!(environment_clear_color(&transparent, theme_clear).r, theme_clear.r);
    assert_eq!(environment_clear_color(&absent, theme_clear).r, theme_clear.r);
}

//#region BrushPreviewTests
#[test]
fn brush_preview_mesh_id_falls_back_to_box_without_mesh_url() {
    assert_eq!(brush_preview_mesh_id(None), "box");
    assert_eq!(brush_preview_mesh_id(Some("/assets/tower.glb")), mesh_id_from_url("/assets/tower.glb"));
}
//#endregion BrushPreviewTests

//#region 🔖️WorldInteractionVerbs
#[test]
fn world_interaction_definition_declares_path_delimited_item_domain() {
    let def = world_interaction_definition();
    assert_eq!(def.id, WORLD_INTERACTION_DOMAIN_ID);
    assert_eq!(def.granularities.iter().map(|granularity| granularity.id.clone()).collect::<Vec<_>>(), vec!["surface".to_string(), "item".to_string(), "vertex".to_string(), "edge".to_string(), "face".to_string()]);
    assert!(matches!(def.hierarchy, HierarchyProvider::PathDelimited { ref delimiter } if delimiter == "/"));
    assert!(def.selection.methods.contains(&SelectionMethod::Pick));
    assert!(def.selection.methods.contains(&SelectionMethod::Rectangle));
    assert!(def.selection.merges.contains(&MergeMode::Additive));
}

/// 🕹️ The shared domain admits the same component granularities and gesture vocabulary.
#[test]
fn world_component_contract_admits_shared_domain_methods() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎯️component-contract/🔣️.json")).unwrap();
    let definition = world_interaction_definition();
    for value in fixture["granularities"].as_array().unwrap() { assert!(definition.granularities.iter().any(|granularity| granularity.id == value.as_str().unwrap()), "{value}"); }
    for value in fixture["methods"].as_array().unwrap() { assert!(definition.selection.methods.iter().any(|method| selection_method_wire_str(*method) == value.as_str().unwrap()), "{value}"); }
    for value in fixture["merges"].as_array().unwrap() { assert!(definition.selection.merges.iter().any(|merge| merge.wire_label() == value.as_str().unwrap()), "{value}"); }
}
