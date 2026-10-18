//! 🧪️ Exact app_commands OpBinary witness, field ordinals, partitions, cancellation, and hostile input.

use crate::editor::flow::commands::{context_menu_at, evaluate, flow_eval_resolve, flow_eval_tick, open_spotlight, replace_image};
use semio_framework_test_fixtures::fixture_text;
use serde_json::Value;
use std::sync::Arc;
#[allow(unused_imports)]
use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
#[allow(unused_imports)]
use crate::editor::flow::commands::{add_widget, connect_media_ports, delete_selection, disconnect, duplicate_widget, focus_selection, move_media_node, node_graph_edit, node_graph_viewport, patch_flow_widgets, remove_widget, rename_flow_widget, reorganize, run_extension_action, set_active_example, set_catalogue_sections, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_lod_mode, set_contributions, set_preview_off, set_proximity_distance, spotlight_commit, toggle_extension};
#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
#[allow(unused_imports)]
use semio_framework_plugin::{NoConfig, NoConfigMutation};
#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::{compiled, main};
#[allow(unused_imports)]
use crate::editor::flow::modes::generate::commands::{add_generation, remove_generation, rename_generation, select_generation, update_generation_values};
#[allow(unused_imports)]
use crate::editor::flow::modes::generate::windows::{form, generations, preview};
#[allow(unused_imports)]
use crate::editor::flow::modes::{edit, generate};
#[allow(unused_imports)]
use crate::editor::flow::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
#[allow(unused_imports)]
use crate::editor::flow::presence::{FlowPresence, FlowPresenceMutation};
#[allow(unused_imports)]
use crate::editor::flow::terminology::{flow_play_labels, FlowPlayLabels};
#[allow(unused_imports)]
use crate::editor::flow::base::{flow_graph_edge_target_id, flow_graph_node_target_id, flow_graph_selection_domains, FLOW_GRAPH_EDGE_TARGET_PREFIX, FLOW_GRAPH_NODE_TARGET_PREFIX, FLOW_GRAPH_OPERATION_RAW_BYTES, FLOW_INTERACTION_GRAPH, FLOW_PLAY_APP_ID, FLOW_PLAY_WINDOW_MAIN, FLOW_STORE_MAX_MUTATION_ITEMS, FLOW_STORE_MAX_SCENE_ITEMS, FLOW_STORE_MAX_TEXT_BYTES};
#[allow(unused_imports)]
use crate::editor::flow::base::{FLOW_PLAY_WINDOW_GENERATIONS, FLOW_PLAY_WINDOW_GENERATE_FORM, FLOW_PLAY_WINDOW_GENERATE_PREVIEW};
#[allow(unused_imports)]
use crate::editor::flow::host::{cold_grant, flow_content_leaves_emit, flow_removal_leaves, focus_selection_camera, seed_host_catalogue, value_fault};
#[allow(unused_imports)]
use crate::editor::flow::host::{host_from_snapshot, sync_host_selection_domains};
#[allow(unused_imports)]
use crate::{FlowMutation, FlowSnapshot, FlowWorkingScene, FLOW_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use flow::{flow_host_with_session, FlowEvalSession, FlowHost, FLOW_LOD_MODE_AUTOMATIC};
#[allow(unused_imports)]
use semio_framework_artifact_flow_flow::{CameraJson, Widget};
#[allow(unused_imports)]
use semio_framework_artifact_flow_flow::{SynapseSpec, WidgetLayout};
#[allow(unused_imports)]
use semio_framework_os_infinite_directed_presentation::DagDrawLod;
#[allow(unused_imports)]
use semio_framework_plugin::app::{ChildEmitPreparation, InteractionView};
#[allow(unused_imports)]
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_framework_plugin::ActionArgOption;
#[allow(unused_imports)]
use semio_framework_plugin::ActionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ActionKind;
#[allow(unused_imports)]
use semio_framework_plugin::AppActionRegistry;
#[allow(unused_imports)]
use semio_framework_plugin::AppDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactEditor;
#[allow(unused_imports)]
use semio_framework_plugin::ArtifactView;
#[allow(unused_imports)]
use semio_framework_plugin::CommandDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::ConfigView;
#[allow(unused_imports)]
use semio_framework_plugin::ContextMenuItemSpec;
#[allow(unused_imports)]
use semio_framework_plugin::ContextMenuRequest;
#[allow(unused_imports)]
use semio_framework_artifact_reference::Dialect;
#[allow(unused_imports)]
use semio_framework_plugin::DomainTopology;
#[allow(unused_imports)]
use semio_framework_plugin::DraftView;
#[allow(unused_imports)]
use semio_framework_plugin::Editor;
#[allow(unused_imports)]
use semio_framework_plugin::Effect;
#[allow(unused_imports)]
use semio_framework_plugin::Emit;
#[allow(unused_imports)]
use semio_framework_plugin::Fault;
#[allow(unused_imports)]
use semio_framework_plugin::GranularityDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::HierarchyProvider;
#[allow(unused_imports)]
use semio_framework_plugin::HoverSpec;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionDefinition;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionRef;
#[allow(unused_imports)]
use semio_framework_plugin::InteractionTopology;
#[allow(unused_imports)]
use semio_framework_ui_locale::Label;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_framework_plugin::MergeMode;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraft;
#[allow(unused_imports)]
use semio_framework_plugin::NoDraftMutation;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMethod;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionMode;
#[allow(unused_imports)]
use semio_framework_plugin::SelectionSpec;
#[allow(unused_imports)]
use semio_framework_plugin::TopologyNode;
#[allow(unused_imports)]
use semio_framework_plugin::WindowMeasure;
#[allow(unused_imports)]
use serde_json::json;
#[allow(unused_imports)]
use semio_framework::kernel::UiDirtyScope;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::mutations::{insert_edge, insert_node, SemioFlowMutation};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowEdge, PortRef, SemioFlowSnapshot};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::snapshot::FlowNode;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::editor::flow::{flow_scalar_command_view};
use crate::editor::flow::*;
use store::os_pack::record::scalar_witness::{ScalarRecordWireStep, ScalarRecordWireWitness};

//#region 🔣️Fixture
fn text(field: &Value) -> String {
    field["unit"].as_str().unwrap().repeat(field["repeat"].as_u64().unwrap() as usize) + field["suffix"].as_str().unwrap()
}
fn command(row: &Value) -> FlowCommand {
    let fields = &row["fields"];
    match row["id"].as_str().unwrap() {
        "evaluate" => FlowCommand::Evaluate(evaluate::Evaluate {}),
        "contextMenuAt" => FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: text(&fields[0]) }),
        "openSpotlight" => FlowCommand::OpenSpotlight(open_spotlight::OpenSpotlight {}),
        "replaceImage" => FlowCommand::ReplaceImage(replace_image::ReplaceImage { id: text(&fields[0]) }),
        "flowEvalTick" => FlowCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: text(&fields[0]), window_kind_id: text(&fields[1]) }),
        "flowEvalResolve" => FlowCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { window_id: text(&fields[0]), node_hash: fields[1]["value"].as_str().unwrap().parse().unwrap(), output_json: fields.as_array().unwrap().get(2).map(text) }),
        _ => unreachable!(),
    }
}
fn advance(cursor: &mut ScalarRecordWireWitness<FlowCommand>, wire: &[u8], grant: usize, cancel: Option<usize>) -> Result<bool, &'static str> {
    let mut offset = 0;
    let mut steps = 0;
    loop {
        let mut charged = 0;
        for _ in 0..grant {
            if cancel == Some(steps) {
                return Ok(false);
            }
            steps += 1;
            match cursor.advance(wire.get(offset).copied())? {
                ScalarRecordWireStep::Progress { compared_bytes } => {
                    assert!(compared_bytes <= 1);
                    charged += compared_bytes;
                }
                ScalarRecordWireStep::Consumed { compared_bytes } => {
                    assert_eq!(compared_bytes, 1);
                    charged += compared_bytes;
                    offset += 1;
                }
                ScalarRecordWireStep::Complete => {
                    assert_eq!(offset, wire.len());
                    return Ok(true);
                }
            }
            assert!(charged <= grant);
            assert!(steps < 500_000);
        }
    }
}
fn close(mut cursor: ScalarRecordWireWitness<FlowCommand>, root: &Arc<FlowCommand>) {
    cursor.begin_close();
    let original = cursor.take_root().unwrap();
    assert!(Arc::ptr_eq(&original, root));
    assert!(cursor.terminal_is_empty());
}
//#endregion 🔣️Fixture

//#region 🧪️Laws
#[test]
fn host_wire_witness_matches_real_opbinary_for_all_six_routes() {
    let fixture: Value = serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/📡️host-wire/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let root = Arc::new(command(row));
        let bytes = protocol::OpBinary::encode_op(root.as_ref()).unwrap();
        assert_eq!(bytes.len(), row["wireBytes"].as_u64().unwrap() as usize);
        assert_eq!(bytes[0], 1);
        assert_eq!(bytes[1] as u64, row["ordinal"].as_u64().unwrap());
        for grant in fixture["grants"].as_array().unwrap() {
            let mut cursor = ScalarRecordWireWitness::new(root.clone(), flow_scalar_command_view);
            assert!(advance(&mut cursor, &bytes, grant.as_u64().unwrap() as usize, None).unwrap());
            close(cursor, &root);
        }
        for stop in fixture["cancelAfterSteps"].as_array().unwrap() {
            let mut cursor = ScalarRecordWireWitness::new(root.clone(), flow_scalar_command_view);
            let _ = advance(&mut cursor, &bytes, 1, Some(stop.as_u64().unwrap() as usize)).unwrap();
            close(cursor, &root);
        }
        let mut wrong_ordinal = bytes.clone();
        wrong_ordinal[1] ^= 1;
        let mut trailing = bytes.clone();
        trailing.push(0);
        for invalid in [wrong_ordinal, trailing, bytes[..bytes.len() - 1].to_vec(), br#"["evaluate",{}]"#.to_vec()] {
            let mut cursor = ScalarRecordWireWitness::new(root.clone(), flow_scalar_command_view);
            assert!(advance(&mut cursor, &invalid, 1, None).is_err());
            assert!(cursor.advance(Some(1)).is_err());
            close(cursor, &root);
        }
    }
}
//#endregion 🧪️Laws
