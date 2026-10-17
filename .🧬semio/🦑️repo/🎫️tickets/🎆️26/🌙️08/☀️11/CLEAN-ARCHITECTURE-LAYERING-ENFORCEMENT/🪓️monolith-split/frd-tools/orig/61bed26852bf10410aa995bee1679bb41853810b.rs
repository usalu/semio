use crate::{FlowSnapshot, FlowWorkingScene};
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_plugin::Effect;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use serde_json::Value;
use std::sync::Arc;
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
use crate::{FlowMutation, FLOW_DOCUMENT_SCHEMA};
#[allow(unused_imports)]
use flow::{flow_host_with_session, FlowEvalSession, FlowHost, FLOW_LOD_MODE_AUTOMATIC};
#[allow(unused_imports)]
use semio_framework_artifact_flow_flow::CameraJson;
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
use semio_framework_2d::compute::EngineHandles;
use crate::editor::flow::*;
use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};

#[global_allocator]
static ORIGINAL_COMMAND_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

fn observe<T>(work:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,heap.requested_bytes,heap.released_bytes)}

#[test]
fn original_flow_commands_close_full_sources_with_system_receipts(){
 let corpus:Value=serde_json::from_str(runtime_fixture_text("../../🧫️fixtures/🔣️.json")).unwrap();
 for copy in [1,3,64]{for cancel in [0,1,9,37]{
  let(commands,born,free)=observe(unit_tests::every_command);assert_eq!(commands.len(),corpus["commands"].as_array().unwrap().len());let original=born-free;let(mut births,mut releases)=(0,0);
  let(mut owner,born,free)=observe(||ControlledRetirement::new(commands).unwrap_or_else(|_|unreachable!("every defining command must retain full close authority")));assert_eq!((born,free),(0,0));let mut idle=0;let mut cancelled=false;
  for turn in 0..100000{
   if owner.terminal_is_empty(){break}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
   let(step,born,free)=observe(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((born,free),(0,0));if turn==cancel{cancelled=true;}
   let(step,born,free)=observe(||owner.step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=born;releases+=free;if receipt==Default::default(){idle+=1}else{idle=0}assert!(idle<64,"original full command close stalled at turn {turn} copy {copy}");
  }
  assert!(cancelled);assert!(owner.terminal_is_empty());assert_eq!(original+births,releases);assert_eq!(observe(||drop(owner)),((),0,0));
 }}
}

#[test]
fn original_flow_snapshot_preserves_local_scene_and_full_system_receipts(){
 for copy in [1,3,64]{for cancel in [0,1,9,37]{
  let(source,born,free)=observe(||{let mut snapshot=FlowSnapshot::default();crate::cache_flow_content(&mut snapshot.content,vec![Widget::InputNote{id:"n雪".into(),text:"original🌊️".into()}],Vec::new(),flow::OrderedMap::default());snapshot});let original=born-free;
  let scene=source.content.local_owner::<FlowWorkingScene>().unwrap();let pointer=Arc::as_ptr(&scene);assert_eq!(scene.widgets.len(),1);assert_eq!(observe(||drop(scene)),((),0,0));
  let(mut owner,born,free)=observe(||ControlledRetirement::new(source).unwrap_or_else(|_|unreachable!()));assert_eq!((born,free),(0,0));let(mut births,mut releases,mut idle)=(0,0,0);let mut cancelled=false;
  for turn in 0..100000{if owner.terminal_is_empty(){break}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
   let(step,born,free)=observe(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((born,free),(0,0));if let Some(source)=owner.original(){let scene=source.content.local_owner::<FlowWorkingScene>().unwrap();assert_eq!(Arc::as_ptr(&scene),pointer);assert_eq!(observe(||drop(scene)),((),0,0));}if turn==cancel{cancelled=true;}
   let(step,born,free)=observe(||owner.step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=born;releases+=free;if receipt==Default::default(){idle+=1}else{idle=0}assert!(idle<64,"original editor snapshot close stalled");}
  assert!(cancelled);assert!(owner.terminal_is_empty());assert_eq!(original+births,releases);assert_eq!(observe(||drop(owner)),((),0,0));
 }}
}

#[test]
fn original_flow_completion_rejection_admits_whole_source_and_exact_system_receipts(){
 type Source=semio_framework_plugin::ArtifactToolCompletionRejection<semio_framework_plugin::EditorApp<FlowPlayApp>>;
 let corpus:Value=serde_json::from_str(runtime_fixture_text("../../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📬️completion/♻️retirement/🚫️refusal/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for copy in corpus["copies"].as_array().unwrap().iter().map(|copy|copy.as_u64().unwrap()as usize){for row in corpus["receivingCases"].as_array().unwrap(){
  let(source,born,free)=observe(||{
   let fault=||{let mut fault=Fault::new(semio_framework_diagnostic::FaultOrigin::App,"original-completion-rejection",row["message"].as_str().unwrap());fault.scope.module=Some(row["scope"].as_str().unwrap().into());fault.params=Some(Box::new(semio_framework_diagnostic::FaultParams(vec![("original".into(),row["parameter"].as_str().unwrap().into())])));fault};
   let emit=if row["outcome"]=="fault"{Err(fault())}else{Ok(Emit{effects:vec![Effect::DispatchAction{req:semio_framework_plugin::RequestId(107),action:"flowEvalTick".into(),args:None,delay_ms:0}],extension_invocations:vec![semio_framework_plugin::ExtensionInvocation::new(row["extension"].as_str().unwrap(),"evaluate",row["input"].as_str().unwrap(),"flowEvalResolve")],..Default::default()})};
   Source{emit,ephemeral:semio_framework_plugin::EphemeralEmit::default(),fault:fault()}
  });let original=born-free;
  let pointer=source.fault.message.as_ptr();let capacity=semio_framework_value::retirement::owned_retirement_birth_bytes::<Source>();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1};
  let((error,source),born,free)=observe(||semio_framework_value::retirement::admit_owned_retirement(source,RetainedCloneGrant{maximum_items:0,..grant}).err().unwrap());assert_eq!((born,free),(0,0));assert_eq!(source.fault.message.as_ptr(),pointer);assert_eq!(error.retained_progress(),Default::default());assert_eq!(observe(||drop(error)),((),0,0));
  let((owner,receipt),born,free)=observe(||semio_framework_value::retirement::admit_owned_retirement(source,grant).unwrap_or_else(|_|panic!("whole original rejection admission")));assert!(receipt.fits(grant));assert_eq!((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes));let(mut births,mut releases)=(born,free);let mut owner=Some(owner);let mut idle=0;
  for turn in 0..100000{if owner.is_none(){break}let demand=semio_framework_value::factory_ticket_demands(owner.as_ref().unwrap(),copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,born,free)=observe(||semio_framework_value::close_factory_ticket(&mut owner,RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((born,free),(0,0));let(step,born,free)=observe(||semio_framework_value::close_factory_ticket(&mut owner,grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((born,free),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=born;releases+=free;if receipt==Default::default(){idle+=1}else{idle=0}assert!(idle<64,"original whole rejection stalled turn={turn} copy={copy} demand={demand:?}");}
  assert!(owner.is_none());assert_eq!(original+births,releases);assert_eq!(observe(||drop(owner)),((),0,0));
 }}
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
