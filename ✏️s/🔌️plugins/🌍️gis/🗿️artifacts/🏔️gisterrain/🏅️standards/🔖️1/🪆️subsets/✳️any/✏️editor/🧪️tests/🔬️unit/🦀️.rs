//! 🔐️ Checks private Terrain command factory admission and exposes the shared test context.

use crate::editor::gis3d::{Gis3dCommand, Gis3dCommandJobFactory, Gis3dPlayApp, GIS3D_RETAINED_RAW_BYTES, GIS3D_RETAINED_TOOL_IDS, GIS3D_RETAINED_WORK_ITEMS, gis3d_retained_extent};
use crate::editor::gis3d::commands::view::set_camera;
use crate::standards::v1::subsets::any::io::text::snapshot::default_terrain_document;
use semio_framework::{ToolExecutionContract, ToolJobFactory};
use semio_framework_test_fixtures::fixture_text;
use semio_plugin::ArtifactEditor;
use serde_json::Value;

use crate as artifact;

#[path = "../🧰️context/🦀️.rs"]
pub mod context;

semio_plugin::history_edit_acceptance_law!("gis", Gis3dPlayApp, gis3d_app_manifest_for_tests, "../..");

use context::gis3d_app_manifest_for_tests;

#[test]
fn published_terrain_fault_notices_pass_registry_validation() {
    let notices = semio_framework::fault_notice_definitions(Gis3dPlayApp::fault_notices());
    assert_eq!(semio_framework::validate_fault_notices(&notices), []);
}

#[test]
fn history_exaggeration_fixture_matches_the_native_fold() {
    use crate::{GisTerrainSnapshot, mutations::GisTerrainMutation};
    use protocol::ColdMutation;
    use semio_framework_pack_json::{JsonMemberPolicy, from_json_str, to_json_string};

    let fixture: Value = serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🧬️history-edits/🎚️exaggeration/🦠️mutation/🔣️.json")).unwrap();
    let before: GisTerrainSnapshot = from_json_str(&fixture["before"].to_string(), JsonMemberPolicy::Reject).unwrap();
    let mutation: GisTerrainMutation = from_json_str(&fixture["mutation"].to_string(), JsonMemberPolicy::Reject).unwrap();
    let outcome = mutation.diff(&before);
    let applied = protocol::apply_diff_cold(outcome.diff(), &before).unwrap();
    let actual: Value = serde_json::from_str(&to_json_string(&applied)).unwrap();
    assert_eq!(actual, fixture["after"]);
}

#[test]
fn history_operations_receive_under_mounted_copy_quanta() {
    use crate::mutations::{ChangeExaggeration, ChangeImportedFeatures, GisTerrainMutation, ReplaceElevationSource};
    use dsl::os_dsl::original_receiving::{ReceivingStock, receive_to_completion};
    use protocol::OpBinary;
    use semio_framework_value::RetainedCloneGrant;

    let gis = default_terrain_document();
    let operations = [
        (0, GisTerrainMutation::ChangeExaggeration(ChangeExaggeration { new_exaggeration: 2.5 })),
        (1, GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_map: gis.imported_map })),
        (2, GisTerrainMutation::ReplaceElevationSource(ReplaceElevationSource { new_elevation_source: gis.elevation_source })),
        (2, GisTerrainMutation::ReplaceElevationSource(ReplaceElevationSource { new_elevation_source: None })),
    ];
    let quantum = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation;
    let stock = ReceivingStock { stock: RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, ..quantum }, quantum, fuel: 1 << 20 };
    for (tag, operation) in operations {
        let bytes = operation.encode_op().unwrap();
        assert_eq!(bytes[..2], [1, tag]);
        assert_eq!(GisTerrainMutation::decode_op(&bytes).unwrap(), operation);
        let received = receive_to_completion::<GisTerrainMutation>(&bytes, (0, bytes.len()), stock)
            .unwrap_or_else(|error| panic!("history tag {tag} with {} wire bytes: {error:?}", bytes.len()));
        assert_eq!(received, operation);
    }
}

static RETAINED_LIMITS: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../🧪️tests/🧫️fixtures/🧫️retained-command-limits/🔣️.json"));
#[test]
fn retained_command_factory_matches_the_language_neutral_maximum_oracle() {
    let fixture: Value = serde_json::from_str((*RETAINED_LIMITS)).expect("GIS terrain retained limits decode through serde_json");
    let maximum = fixture.get("maximumTextBytes").and_then(Value::as_u64).expect("maximumTextBytes") as usize;
    let additional = fixture.get("rejectedAdditionalBytes").and_then(Value::as_u64).expect("rejectedAdditionalBytes") as usize;
    let expected_items = fixture.get("expectedWorkItems").and_then(Value::as_u64).expect("expectedWorkItems") as usize;
    let tool_ids = fixture.get("toolIds").and_then(Value::as_array).expect("toolIds").iter().map(|value| value.as_str().expect("tool id")).collect::<Vec<_>>();
    assert_eq!(maximum, GIS3D_RETAINED_RAW_BYTES);
    assert_eq!(expected_items, GIS3D_RETAINED_WORK_ITEMS);
    assert_eq!(tool_ids, GIS3D_RETAINED_TOOL_IDS);
    let snapshot = default_terrain_document();
    let interaction = protocol::InteractionState::default();
    let accepted = Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: format!("{{}}{}", " ".repeat(maximum - 2)) });
    let rejected = Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json: format!("{{}}{}", " ".repeat(maximum + additional - 2)) });
    assert_eq!(gis3d_retained_extent(&accepted, &snapshot, &interaction), Some(expected_items));
    assert_eq!(gis3d_retained_extent(&rejected, &snapshot, &interaction), None);
    let factory = Gis3dCommandJobFactory::new("s.gis.gisterrain@1/*#editor");
    assert_eq!(factory.execution_contract(), ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500));
    let mut args = Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "cameraJson": "c".repeat(maximum + additional) })));
    assert!(Gis3dPlayApp::command_from_action("setCamera", &mut args, &mut None).is_err());
}
