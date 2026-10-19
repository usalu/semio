use semio_framework_plugin::app::ChildEmitPreparation;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::Emit;
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
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
use crate::editor::flow::editor_vocabulary::{flow_graph_edge_target_id, flow_graph_node_target_id, flow_graph_selection_domains, FLOW_GRAPH_EDGE_TARGET_PREFIX, FLOW_GRAPH_NODE_TARGET_PREFIX, FLOW_GRAPH_OPERATION_RAW_BYTES, FLOW_INTERACTION_GRAPH, FLOW_PLAY_APP_ID, FLOW_PLAY_WINDOW_MAIN, FLOW_STORE_MAX_MUTATION_ITEMS, FLOW_STORE_MAX_SCENE_ITEMS, FLOW_STORE_MAX_TEXT_BYTES};
#[allow(unused_imports)]
use crate::editor::flow::editor_vocabulary::{FLOW_PLAY_WINDOW_GENERATIONS, FLOW_PLAY_WINDOW_GENERATE_FORM, FLOW_PLAY_WINDOW_GENERATE_PREVIEW};
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
use semio_framework_plugin::app::InteractionView;
#[allow(unused_imports)]
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
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
use semio_s_artifact_stdio_semio_flow::schema::mutations::{insert_edge, insert_node};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowEdge, PortRef};
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_flow::schema::snapshot::FlowNode;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::editor::flow::{FlowChildGroupWork};
use crate::editor::flow::*;

fn quoted_close_grant(work:&FlowChildGroupWork,body:usize)->semio_framework_value::RetainedCloneGrant{
    let copy=work.next_close_copy_byte_demand().unwrap().max(body);
    semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:work.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:work.next_close_release_byte_demand().unwrap(),maximum_depth:work.next_close_depth_demand().unwrap().max(1)}
}

#[test]
fn rejected_child_preparation_remains_owned_until_bounded_flow_work_close(){
    use semio_framework_plugin::retained_command::{ArtifactCommandWork,ArtifactCommandWorkStep};
    use semio_framework_plugin::app::ChildEmitPreparation;
    use semio_framework_job::InteractiveJobCloseStep;
    use {semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation, semio_s_artifact_stdio_semio_flow::schema::mutations::drag_nodes::DragNodes};
    let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../✏️node-graph-edit/🧪️tests/🧫️fixtures/child-preparation/🔣️.json")).unwrap();
    let child_id=fixture["childId"].as_str().unwrap();
    let mut work=FlowChildGroupWork{tool_id:"nodeGraphEdit",instance_owner:None,output:None,completed:false,closing:false};
    for (slot,accepted) in [(fixture["foreignSlot"].as_str().unwrap(),false),(fixture["slot"].as_str().unwrap(),true)]{
        let operation=SemioFlowMutation::DragNodes(DragNodes{targets:vec![fixture["target"].as_str().unwrap().into()],dx:fixture["dx"].as_f64().unwrap(),dy:fixture["dy"].as_f64().unwrap()});
        let emit=semio_framework_plugin::Emit{child_preparations:std::collections::VecDeque::from([ChildEmitPreparation::of::<SemioFlowSnapshot,_>(slot,child_id,vec![operation])]),..Default::default()};
        let source=emit.child_preparations.front().unwrap();
        assert_eq!(source.matches_source::<SemioFlowMutation>(fixture["slot"].as_str().unwrap(),child_id,1),accepted);
        assert!(!source.matches_source::<u8>(slot,child_id,1));
        assert!(!source.matches_source::<SemioFlowMutation>(slot,"foreign-child",1));
        assert!(!source.matches_source::<SemioFlowMutation>(slot,child_id,0));
        assert_eq!(source.retained_operation_count(),fixture["operationCount"].as_u64().unwrap()as usize);
        assert!(source.accepted_prefix().unwrap().ops.is_empty());
        let result=work.accept_output(emit,child_id);
        if accepted{
            let ArtifactCommandWorkStep::Complete(emit)=result.unwrap()else{panic!("exact original typed source must transfer complete");};
            assert!(work.output.is_none());work.output=Some(emit);
        }else{
            let fault=match result{Err(fault)=>fault,_=>panic!("foreign literal slot must refuse")};
            assert_eq!(fault.message,fixture["refusalCode"].as_str().unwrap());
            assert_eq!(work.output.as_ref().unwrap().child_preparations.front().unwrap().retained_operation_count(),1);
        }
        work.begin_close();
        let zero=semio_framework_value::RetainedCloneGrant{maximum_items:0,..quoted_close_grant(&work,4096)};
        assert_eq!(semio_framework_job::close_with_self_funded_context(zero, |cx| work.close_step(cx)).expect("original fixture close turn"),InteractiveJobCloseStep::Pending{progress:Default::default()});
        assert_eq!(work.output.as_ref().unwrap().child_preparations.front().unwrap().retained_operation_count(),1);
        let body=fixture["closeGrant"].as_u64().unwrap()as usize;
        let mut terminal=false;
        for _ in 0..fixture["closeSteps"].as_u64().unwrap(){
            let grant=quoted_close_grant(&work,body);
            match semio_framework_job::close_with_self_funded_context(grant, |cx| work.close_step(cx)).expect("original fixture close turn"){
                InteractiveJobCloseStep::Pending{progress}=>assert!(progress.fits(grant)),
                InteractiveJobCloseStep::Complete{progress}=>{assert!(progress.fits(grant));assert!(work.terminal_is_empty());terminal=true;break;},
                other=>panic!("[DEBUG] actual child owner did not close: {other:?}"),
            }
        }
        assert!(terminal);work.closing=false;work.completed=false;
    }
}
