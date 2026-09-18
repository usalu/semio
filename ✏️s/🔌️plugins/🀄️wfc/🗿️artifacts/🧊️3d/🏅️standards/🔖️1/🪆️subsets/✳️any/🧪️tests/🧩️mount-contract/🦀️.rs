//! 🧩️ Language-agnostic `wfc3d` mount contract — the app ids, window kinds, codecs and examples the
//! shell and the plugin registry read. Its committed fixture beside it is the same statement in
//! JSON, and the TS/Python siblings replay it, so a drift shows up in three places at once.

use crate::examples::{tower_stack, two_room_corridor, wall_roof_facade_strip};
use crate::schema::snapshot::text::{parse_dsl, print_dsl};
use crate::{Wfc3dSnapshot, WFC3D_DIALECT};
use store::ArtifactPack;

const CONTRACT: &str = include_str!("../../🧫️fixtures/🧩️mount-contract/🔣️.json");

fn contract() -> serde_json::Value {
    serde_json::from_str(CONTRACT).expect("the committed mount contract decodes")
}

#[test]
fn editor_and_viewer_share_the_wfc3d_dialect() {
    assert_eq!(<crate::editor::wfc3d::Wfc3dEditor as semio_framework_plugin::ArtifactEditor>::DIALECT, WFC3D_DIALECT);
    assert_eq!(<crate::viewer::wfc3d::Wfc3dViewer as semio_framework_plugin::ArtifactViewer>::DIALECT, WFC3D_DIALECT);
}

#[test]
fn the_committed_app_ids_are_what_the_surfaces_actually_build() {
    let contract = contract();
    assert_eq!(crate::editor::wfc3d::create_wfc3d_editor().id, contract["editorAppId"].as_str().expect("editorAppId"));
    assert_eq!(crate::viewer::wfc3d::create_wfc3d_viewer().id, contract["viewerAppId"].as_str().expect("viewerAppId"));
}

#[test]
fn the_committed_window_kinds_are_what_the_surfaces_actually_declare() {
    let contract = contract();
    let editor: Vec<String> = crate::editor::wfc3d::create_wfc3d_editor().window_kinds.iter().map(|window| window.id.clone()).collect();
    let declared: Vec<String> = contract["editorWindowKinds"].as_array().expect("editorWindowKinds").iter().map(|value| value.as_str().unwrap_or_default().to_string()).collect();
    assert_eq!(editor, declared);
    let viewer: Vec<String> = crate::viewer::wfc3d::create_wfc3d_viewer().window_kinds.iter().map(|window| window.id.clone()).collect();
    let declared: Vec<String> = contract["viewerWindowKinds"].as_array().expect("viewerWindowKinds").iter().map(|value| value.as_str().unwrap_or_default().to_string()).collect();
    assert_eq!(viewer, declared);
}

#[test]
fn the_committed_mutation_roster_is_the_dispatch_roster() {
    let contract = contract();
    let declared: Vec<String> = contract["mutations"].as_array().expect("mutations").iter().map(|value| value.as_str().unwrap_or_default().to_string()).collect();
    assert_eq!(declared, crate::mutations::KINDS.iter().map(|kind| kind.to_string()).collect::<Vec<_>>());
}

#[test]
fn every_example_round_trips_through_dsl_and_pack() {
    for snapshot in [two_room_corridor::snapshot(), wall_roof_facade_strip::snapshot(), tower_stack::snapshot()] {
        let text = print_dsl(&snapshot);
        assert!(!text.trim().is_empty());
        assert_eq!(parse_dsl(&text).expect("dsl"), snapshot);
        let pack = ArtifactPack::encode_pack(&snapshot);
        assert!(pack.len() > 64);
        assert_eq!(<Wfc3dSnapshot as ArtifactPack>::decode_pack(&pack).expect("pack"), snapshot);
    }
}

#[test]
fn the_committed_example_roster_matches_the_bundled_sources() {
    let contract = contract();
    let declared: Vec<(String, u64)> = contract["examples"]
        .as_array()
        .expect("examples")
        .iter()
        .map(|entry| (entry["id"].as_str().unwrap_or_default().to_string(), entry["slots"].as_u64().unwrap_or_default()))
        .collect();
    let actual: Vec<(String, u64)> = [two_room_corridor::snapshot(), wall_roof_facade_strip::snapshot(), tower_stack::snapshot()]
        .iter()
        .zip([two_room_corridor::ID, wall_roof_facade_strip::ID, tower_stack::ID])
        .map(|(snapshot, id)| (id.to_string(), snapshot.slots.len() as u64))
        .collect();
    assert_eq!(declared, actual);
    assert_eq!(crate::examples::sources().len(), 3);
}

#[test]
fn example_labels_are_localized_en_and_de() {
    assert_eq!(two_room_corridor::label(), semio_framework_plugin::LocalizedLabel::native("Two Rooms And A Corridor", "Zwei Räume und ein Korridor"));
    assert_eq!(wall_roof_facade_strip::label(), semio_framework_plugin::LocalizedLabel::native("Wall And Roof Facade Strip", "Wand-Dach-Fassadenstreifen"));
    assert_eq!(tower_stack::label(), semio_framework_plugin::LocalizedLabel::native("Tower With A Cantilever", "Turm mit Auskragung"));
}

/// 💡️ The routed inference is metadata only: the host is told the solve answers on a cold-job route,
/// which is what the committed contract names.
#[test]
fn the_committed_inference_route_is_the_one_the_factory_registers() {
    let contract = contract();
    assert_eq!(contract["inferenceToolId"].as_str(), Some(crate::inferences::WFC3D_INFERENCE_TOOL_ID));
    assert_eq!(contract["inferenceJobKind"].as_str(), Some(crate::inferences::WFC3D_INFERENCE_JOB_KIND));
    assert_eq!(contract["inferencePayloadSchema"].as_str(), Some(crate::inferences::WFC3D_INFERENCE_PAYLOAD_SCHEMA));
}
