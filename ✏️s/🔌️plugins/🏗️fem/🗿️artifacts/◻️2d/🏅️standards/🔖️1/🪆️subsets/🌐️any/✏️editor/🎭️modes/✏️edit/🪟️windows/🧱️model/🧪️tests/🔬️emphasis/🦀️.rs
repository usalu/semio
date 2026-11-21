//! 🎨️ The model window paints the interaction emphasis over the bare structure layers and nothing else.

use super::*;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::{fem2d_entity_target_id, fem2d_load_target_id, Fem2dInteractionSnapshot};
use pack::snapshot::ArtifactDsl;

#[test]
fn structure_layers_paint_stable_selected_and_hovered_emphasis() {
    let doc = Fem2dSnapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("demo document parses");
    let load = fem2d_load_target_id("live", "l6").unwrap();
    let snapshot = Fem2dInteractionSnapshot { selected_ids: vec![fem2d_entity_target_id("node", "n1").unwrap(), fem2d_entity_target_id("element", "e3").unwrap(), fem2d_entity_target_id("region", "r1").unwrap(), fem2d_entity_target_id("support", "s1").unwrap(), load.clone()], hovered_ids: vec![fem2d_entity_target_id("node", "n2").unwrap()] };
    let bare = semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(fem2d_structure_layers(&doc, "#38bdf8", "#94a3b8", "#f97316")));
    let painted = semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(fem2d_structure_layers_with(&doc, "#38bdf8", "#94a3b8", "#f97316", &snapshot)));
    let empty = semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(fem2d_structure_layers_with(&doc, "#38bdf8", "#94a3b8", "#f97316", &Fem2dInteractionSnapshot::default())));
    <Fem2dSnapshot as semio_framework_value::FromValue>::retire_decoded(doc);
    for id in ["sel-node-n1", "sel-el-e3", "sel-region-r1", "sel-support-s1", &format!("sel-load-{load}"), "hov-node-n2"] {
        assert!(painted.contains(id), "missing emphasis layer {id}");
        assert!(!bare.contains(id), "the bare structure layers must stay free of interaction state ({id})");
    }
    assert!(painted.contains(SELECTION_COLOR_2D), "the selected emphasis carries its own colour");
    assert!(painted.contains(HOVER_COLOR_2D), "the hovered emphasis carries the lighter colour");
    assert_eq!(empty, bare, "an empty interaction snapshot paints exactly the bare layers");
}
