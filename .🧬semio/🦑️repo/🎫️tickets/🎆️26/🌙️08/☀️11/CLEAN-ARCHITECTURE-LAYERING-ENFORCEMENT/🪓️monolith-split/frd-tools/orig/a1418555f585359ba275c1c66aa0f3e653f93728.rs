use serde_json::Value;
#[allow(unused_imports)]
use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
#[allow(unused_imports)]
use crate::editor::flow::commands::{add_widget, connect_media_ports, context_menu_at, delete_selection, disconnect, duplicate_widget, evaluate, flow_eval_resolve, flow_eval_tick, focus_selection, move_media_node, node_graph_edit, node_graph_viewport, open_spotlight, patch_flow_widgets, remove_widget, rename_flow_widget, reorganize, replace_image, run_extension_action, set_active_example, set_catalogue_sections, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_lod_mode, set_contributions, set_preview_off, set_proximity_distance, spotlight_commit, toggle_extension};
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
use std::sync::Arc;
#[allow(unused_imports)]
use semio_framework_2d::compute::EngineHandles;
use crate::editor::flow::{FlowContributionsWork, flow_contributions_registry_receipt, FlowInstanceOperationOwner};
use crate::editor::flow::*;
use semio_framework_job::{CancelToken,Generation,OperationId,StepBudget,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};

fn progress(value:&serde_json::Value)->RetainedCloneProgress{
    RetainedCloneProgress{copied_items:value["copiedItems"].as_u64().unwrap() as usize,copied_bytes:value["copiedBytes"].as_u64().unwrap() as usize,retained_capacity_bytes:value["retainedCapacityBytes"].as_u64().unwrap() as usize,released_bytes:value["releasedBytes"].as_u64().unwrap() as usize}
}

fn grant(value:&serde_json::Value)->RetainedCloneGrant{
    RetainedCloneGrant{maximum_items:value["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes:value["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:value["maximumCapacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes:value["maximumReleaseBytes"].as_u64().unwrap() as usize,maximum_depth:value["maximumDepth"].as_u64().unwrap() as usize}
}

/// ⚖️ Every original success or refusal reaches the same external recipient exactly once.
#[test]
fn flow_contributions_records_original_registry_receipts_before_yield_or_refusal(){
    let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../../🧪️tests/🧫️fixtures/🎟️contributions/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let mut actual=progress(&row["initial"]);let mut sequence=0;
        let mut cx=StepContext::new(OperationId(107_901),Generation(9),StepBudget::new(1,u64::MAX,grant(&row["grant"])),CancelToken::root_now(),semio_framework_job::logical_now_us,&mut sequence,&mut actual);
        let effects=progress(&row["progress"]);
        let result=match row["status"].as_str().unwrap(){"progress"=>Ok(RetainedCloneStep::Progress(effects)),"complete"=>Ok(RetainedCloneStep::Complete(effects)),"refused"=>Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original contribution registry refusal").with_retained_progress(effects)),_=>unreachable!()};
        let received=flow_contributions_registry_receipt(&mut cx,result);
        assert_eq!(received.is_err(),row["failed"].as_bool().unwrap(),"{}",row["id"]);
        match received{Ok(ready)=>assert_eq!(ready,row["ready"].as_bool().unwrap()),Err(fault)=>assert_eq!(fault.retained_progress,effects)}
        assert_eq!(cx.retained_progress(),progress(&row["recorded"]),"{}",row["id"]);
        assert_eq!(cx.retained_grant(),grant(&row["remaining"]),"{}",row["id"]);
        drop(cx);assert_eq!(actual,progress(&row["recorded"]));
    }
}

/// 🪢️ A command retires its own original alias while the live shared instance remains usable.
#[test]
fn flow_contributions_retires_only_its_original_alias_before_terminal_work(){
    let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../../🧪️tests/🧫️fixtures/🎟️contributions/🔣️.json")).unwrap();
    for policy in fixture["aliasPolicies"].as_array().unwrap(){
        let shared=semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(Box::new(FlowInstanceOperationOwner::new()));
        let source=shared.with_mut::<FlowInstanceOperationOwner,_>(|owner|Ok(owner as *mut _ as usize)).unwrap();
        let mut work=FlowContributionsWork::new(shared.clone());work.begin_close();
        let incoming=grant(policy);let denied=RetainedCloneGrant{maximum_items:0,..incoming};
        assert!(matches!(work.close_step(denied),semio_framework_job::InteractiveJobCloseStep::Blocked));
        assert!(work.instance_owner.is_some()&&work.instance_retirement.is_none());
        for turn in 0..128{
            if work.terminal_is_empty(){break}
            assert!(turn<127,"original contribution alias must reach terminal");
            match work.close_step(incoming){semio_framework_job::InteractiveJobCloseStep::Pending{progress}|semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>{assert!(progress.fits(incoming));assert_eq!(progress.retained_capacity_bytes,0);assert_eq!(progress.released_bytes,0);},other=>panic!("original nonfinal alias close refused: {other:?}")}
        }
        assert!(work.terminal_is_empty());assert_eq!(work.terminal_frame_release_bytes(),Some(std::mem::size_of::<FlowContributionsWork>()));
        shared.with_mut::<FlowInstanceOperationOwner,_>(|owner|{assert_eq!(owner as *mut _ as usize,source);assert!(!owner.closing&&owner.eval_session.is_some());Ok(())}).unwrap();
        drop(work);
        let mut final_owner=semio_framework_plugin::ArtifactInstanceOperationOwnerAliasRetirement::new(shared);
        for turn in 0..1_000_000{if final_owner.terminal_is_empty(){break}assert!(turn<999_999,"actual original final instance owner stalled");final_owner.step(incoming).unwrap();}
        assert!(final_owner.terminal_is_empty());drop(final_owner);
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
