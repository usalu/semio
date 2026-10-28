extern crate semio_framework_os_kernel as store;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/abort-tests.rs"]mod original_stream_abort;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as dsl;
use semio_framework_plugin_window_config::*;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_value::{ToValue,FromValue,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_framework_job::*;
use std::{sync::Arc,mem::size_of};
use semio_framework_tool_run::*;
use store::OpBinary;
#[global_allocator]static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🗄️store-owners/🌱️initialization/🎟️birth/🧪️tests/🧸️fixture/🦀️.rs"]mod fixture;
pub(crate) use fixture::{Owner,State,Mutation};
impl semio_framework_schema_composition::ArtifactCompositionFields for State {
 fn visit_child_refs<'a,V:semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self,_:&mut V)->Result<(),V::Error>{Ok(())}
}
use semio_framework_plugin_tool_run::original_operations::OriginalToolRunOperations;
pub(crate) const TURN:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096};
pub(crate) fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(8701),Generation(51),RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<29,maximum_release_bytes:1<<29,maximum_depth:4096},default_now_us,u64::MAX,1<<30)}
#[track_caller]
pub(crate) fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,f:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,f));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes),"original opwire physical receipts: {p:?}");result.unwrap()}
pub(crate) fn native_store(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner)->Box<store::ConfigStore<State,Mutation>>{let wide=RetainedCloneGrant{maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,..TURN};let actor=protocol::ActorId("tool-root-reader".into());let mut birth=turn(caller,decoder,wide,|cx|WindowConfigPartitionBirth::<Owner>::admit("operations",&actor,cx)).unwrap();let mut output=None;for index in 0..262144{output=turn(caller,decoder,wide,|cx|birth.step(cx));if output.is_some(){break}assert!(index<262143,"actual native Store birth did not finish");}let(address,store)=output.unwrap();assert_eq!(address,"operations");drop(address);while !birth.terminal_is_empty(){turn(caller,decoder,wide,|cx|cx.with_original_retirement(|cx|birth.close_step(cx).map(|_|())));}turn(caller,decoder,wide,|cx|{let d=WindowConfigPartitionBirth::<Owner>::boxed_release_demands();close_original_retained_step(cx,d,|_|{drop(birth);Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;Ok(())});store}
#[test]fn native_wire_root_operations_replay_retract_finalize_and_close(){let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut store=native_store(&mut caller,&mut decoder);let mut owner=OriginalToolRunOperations::<State,Mutation>::default();let mut overlay=store.snapshot_owner();assert_eq!(overlay.zoom,1.25);assert!(owner.terminal_is_empty());for _ in 0..2{assert!(turn(&mut caller,&mut decoder,TURN,|cx|owner.refold(&store,&mut overlay,cx).map_err(|error|panic!("original refold {error:?}; next {:?}",owner.diagnostic_cursor(&store)))));}let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let identity=ToolRunIdentity::new(ToolRunId{app_instance_id:71,run:18},[71;32]);let mut sequence=0;for (zooms,retract,expected)in [(vec![law["append"][0].as_f64().unwrap()],None,law["expected"][0].as_f64().unwrap()),(vec![law["append"][1].as_f64().unwrap()],None,law["expected"][1].as_f64().unwrap()),(vec![],Some(law["retractTo"].as_u64().unwrap()as u32),law["expected"][2].as_f64().unwrap()),(vec![law["replacement"][0].as_f64().unwrap()],None,law["expected"][3].as_f64().unwrap())]{sequence+=1;let ops:Vec<_>=zooms.iter().map(|zoom|{let mut wire=b"op".to_vec();wire.extend_from_slice(&serde_json::to_vec(&serde_json::json!({"kind":"zoom","zoom":zoom})).unwrap());wire}).collect();let pointers:Vec<_>=ops.iter().map(|bytes|bytes.as_ptr()).collect();let mut tick=ToolRunTick{identity,sequence,progress:None,steps:Vec::new(),trace:Vec::new(),append_ops:ops,append_entities:Vec::new(),retract_to:retract,payload:None,payload_update:None};let mut ready=false;for index in 0..262144{ready=turn(&mut caller,&mut decoder,TURN,|cx|owner.ingest(&mut tick,&store,&mut overlay,cx).map_err(|error|panic!("original ingest {error:?}; next {:?}",owner.diagnostic_cursor(&store))));if ready{break}assert!(index<262143,"native original operation did not settle: {:?} receipt={:?}",owner.diagnostic_cursor(&store),caller.receipt());}assert!(ready);assert_eq!(overlay.zoom,expected);assert_eq!(store.snapshot().unwrap().zoom,1.25);assert!(tick.append_ops.iter().all(Vec::is_empty));for(index,pointer)in pointers.iter().enumerate(){assert_eq!(owner.original_wire(owner.len()-pointers.len()+index).unwrap().as_ptr(),*pointer);}close_typed(tick,&mut caller,&mut decoder);}owner.rebase();for index in 0..262144{if turn(&mut caller,&mut decoder,TURN,|cx|owner.refold(&store,&mut overlay,cx).map_err(|error|panic!("original refold {error:?}; next {:?}",owner.diagnostic_cursor(&store)))){break}assert!(index<262143,"same genuine Store frontier rebase must finish");}assert_eq!(overlay.zoom,8.0);let mut output=Vec::new();for index in 0..262144{if turn(&mut caller,&mut decoder,TURN,|cx|owner.prepare_finalization(&store,&mut output,cx).map_err(|error|panic!("original finalize {error:?}; next {:?}",owner.diagnostic_cursor(&store)))){break}assert!(index<262143,"native typed finalization did not finish");}assert_eq!(output.iter().map(|op|match op{Mutation::Zoom{zoom}=>*zoom}).collect::<Vec<_>>(),vec![2.5,8.0]);close_typed(output,&mut caller,&mut decoder);for _ in 0..262144{if turn(&mut caller,&mut decoder,TURN,|cx|cx.with_original_retirement(|cx|owner.close_step(&store,cx).map_err(|error|panic!("original close {error:?}; next {:?}",owner.diagnostic_cursor(&store))))){break}}assert!(owner.terminal_is_empty());drop(overlay);for index in 0..1048576{if store.close_owned_store_terminal_is_empty(){break}turn(&mut caller,&mut decoder,TURN,|cx|cx.with_original_retirement(|cx|{let d=store.close_owned_demands(4096)?;assert!(d.copy_bytes<=TURN.maximum_copy_bytes,"[DEBUG] genuine native Store close header exceeds original policy: {d:?}");close_original_retained_step(cx,d,|g|store.close_owned_store_step(g))?;Ok(())}));assert!(index<1048575,"[DEBUG] native Store close stalled: {:?} {} {}",store.close_owned_demands(4096),store.close_owned_phase_witness(),store.close_displaced_witness());}assert!(store.close_owned_store_terminal_is_empty());release_native_store_box(store,&mut caller,&mut decoder);while !decoder.terminal_is_empty(){caller.turn(TURN,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(4096)?;close_original_retained_step(cx,d,|g|decoder.close_step(g))?;Ok(())})).unwrap();}caller.retire(TURN);}

pub(crate) fn close_typed<T:semio_framework_value::retirement::RetireOwned>(input:T,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){let mut input=Some(input);let mut owner=None;turn(caller,decoder,TURN,|cx|{let d=RetirementDemand{copy_bytes:2*size_of::<T>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<T>>(),depth:1,..Default::default()};close_original_retained_step(cx,d,|_|{owner=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(input.take().unwrap()).map_err(|(error,_)|error)?);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;Ok(())});let owner=owner.as_mut().unwrap();for index in 0..262144{if owner.terminal_is_empty(){break}turn(caller,decoder,TURN,|cx|cx.with_original_retirement(|cx|{let d=owner.next_demand(4096)?;close_original_retained_step(cx,d,|g|owner.step(g))?;Ok(())}));assert!(index<262143);}}

#[test]fn cancelled_native_wire_owners_close_under_original_token_at_every_partial_cut(){let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for cut in law["cancelCuts"].as_array().unwrap(){let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut store=native_store(&mut caller,&mut decoder);let mut owner=OriginalToolRunOperations::<State,Mutation>::default();let mut overlay=store.snapshot_owner();let identity=ToolRunIdentity::new(ToolRunId{app_instance_id:71,run:19},[72;32]);let mut tick=ToolRunTick{identity,sequence:1,progress:None,steps:Vec::new(),trace:Vec::new(),append_ops:vec![b"op{\"kind\":\"zoom\",\"zoom\":3.25}".to_vec()],append_entities:Vec::new(),retract_to:None,payload:None,payload_update:None};for _ in 0..cut.as_u64().unwrap(){turn(&mut caller,&mut decoder,TURN,|cx|owner.ingest(&mut tick,&store,&mut overlay,cx).map_err(|error|panic!("original ingest {error:?}; next {:?}",owner.diagnostic_cursor(&store))));}caller.cancel().cancel_now();for index in 0..262144{if turn(&mut caller,&mut decoder,TURN,|cx|cx.with_original_retirement(|cx|owner.close_step(&store,cx).map_err(|error|panic!("original close {error:?}; next {:?}",owner.diagnostic_cursor(&store))))){break}assert!(index<262143,"actual token cancellation retained native owner at cut {cut}: {:?}",owner.diagnostic_cursor(&store));}assert!(owner.terminal_is_empty());close_typed(tick,&mut caller,&mut decoder);let mut alias=Some(overlay);let mut retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>=None;for index in 0..262144{if alias.is_none()&&retirement.is_none(){break}turn(&mut caller,&mut decoder,TURN,|cx|cx.with_original_retirement(|cx|{if retirement.is_some(){let d=retirement.as_ref().unwrap().next_copy_byte_demand()?;let _=d;let g=cx.retained_grant();let step=cx.with_native_retirement(|control|semio_framework_value::close_factory_ticket_controlled(&mut retirement,g,control))?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);}else{let d=RetirementDemand{capacity_bytes:semio_framework_value::factory::FactorySharedRetirement::<State>::constructor_capacity_bytes(),depth:2,..Default::default()};close_original_retained_step(cx,d,|g|{let p=store.retire_snapshot_alias(&mut alias,g)?.map_or(Default::default(),|(owner,p)|{retirement=Some(owner);p});Ok(RetainedCloneStep::Progress(p))})?;}Ok(())}));assert!(index<262143);}for index in 0..1048576{if store.close_owned_store_terminal_is_empty(){break}turn(&mut caller,&mut decoder,TURN,|cx|cx.with_original_retirement(|cx|{let d=store.close_owned_demands(4096)?;assert!(d.copy_bytes<=TURN.maximum_copy_bytes,"[DEBUG] genuine native Store close header exceeds original policy: {d:?}");close_original_retained_step(cx,d,|g|store.close_owned_store_step(g))?;Ok(())}));assert!(index<1048575,"[DEBUG] native Store close stalled: {:?} {} {}",store.close_owned_demands(4096),store.close_owned_phase_witness(),store.close_displaced_witness());}assert!(store.close_owned_store_terminal_is_empty());release_native_store_box(store,&mut caller,&mut decoder);while !decoder.terminal_is_empty(){caller.turn(TURN,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(4096)?;close_original_retained_step(cx,d,|g|decoder.close_step(g))?;Ok(())})).unwrap();}caller.retire(TURN);}}

#[path="publication.rs"]
mod publication;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🧪️tests/🦀️.rs"]
mod original_report;

pub(crate) fn release_native_store_box(store:Box<store::ConfigStore<State,Mutation>>,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){assert!(store.close_owned_store_terminal_is_empty());let mut original=Some(store);turn(caller,decoder,TURN,|cx|cx.with_original_retirement(|cx|{let d=RetirementDemand{copy_bytes:size_of::<Option<Box<store::ConfigStore<State,Mutation>>>>(),release_bytes:size_of::<store::ConfigStore<State,Mutation>>(),depth:1,..Default::default()};close_original_retained_step(cx,d,|_|{original.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;Ok(())}));assert!(original.is_none());}

/// 🧪️ A document fixture declares its history shape before original genesis custody begins.
pub(crate) struct DocumentGenesis;
impl semio_framework_plugin_ownership::original_store_genesis::OriginalStoreGenesisOwner for DocumentGenesis{
 type State=State;type Mutation=Mutation;type Catalog=WindowConfigInitializationOwners<Owner>;type Initial=semio_framework_plugin_ownership::original_store_genesis::FixedStoreInitial<Self>;
 const SCHEMA:&'static str=<Owner as WindowConfigOwner>::SCHEMA;const MAXIMUM_INITIAL_PACK_BYTES:usize=<Owner as WindowConfigOwner>::MAXIMUM_INITIAL_PACK_BYTES;
 fn pack_envelope_id()->Option<&'static str>{Some(<State as pack::snapshot::ArtifactDsl>::envelope_id())}
 fn artifact_dialect()->Option<semio_framework_artifact_reference::Dialect>{Some(semio_framework_artifact_reference::Dialect{artifact_kind:"native-document",standard:semio_framework_artifact_reference::StandardId("1"),subset:semio_framework_artifact_reference::SubsetId::ANY})}
 fn identity_parts<'a>(label:&'a str)->[&'a str;4]{["native-document:","replay",":",label]}
 fn initial_state_pack_source(state:&State)->Result<(&dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),ValueError>{<Owner as WindowConfigOwner>::initial_state_pack_source(state)}
}
impl semio_framework_plugin_ownership::original_store_genesis::OriginalFixedStoreInitial for DocumentGenesis{type State=State;fn initial_state_demands()->Result<RetirementDemand,ValueError>{<Owner as WindowConfigOwner>::initial_state_demands()}fn initial_state(grant:RetainedCloneGrant)->Result<(State,RetainedCloneProgress),ValueError>{<Owner as WindowConfigOwner>::initial_state(grant)}}
pub(crate) fn native_document_store(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner)->Box<store::ConfigStore<State,Mutation>>{
 type Birth=semio_framework_plugin_ownership::original_store_genesis::OriginalStoreGenesis<DocumentGenesis>;let wide=RetainedCloneGrant{maximum_copy_bytes:32768,maximum_capacity_bytes:16*1024*1024,..TURN};let actor=protocol::ActorId("tool-root-reader".into());let mut birth=turn(caller,decoder,wide,|cx|Birth::admit("operations",&actor,cx)).unwrap();let mut output=None;for index in 0..262144{output=turn(caller,decoder,wide,|cx|birth.step(cx));if output.is_some(){break}assert!(index<262143,"original document genesis stalled; close quote {:?}",birth.retirement_demands(4096));}let(address,store)=output.unwrap();drop(address);while !birth.terminal_is_empty(){turn(caller,decoder,wide,|cx|cx.with_original_retirement(|cx|birth.close_step(cx).map(|_|())));}turn(caller,decoder,wide,|cx|{let d=Birth::boxed_release_demands();close_original_retained_step(cx,d,|_|{drop(birth);Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;Ok(())});assert_eq!(store.parts_ref().log.envelope.history_shape,protocol::HistoryShape::Document);store
}

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/🧪️tests/🦀️.rs"]mod original_replay_publication;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🎮️command/🧪️tests/🦀️.rs"]mod original_native_history_command;

#[test]fn original_manifest_metadata_native_custody(){semio_framework_plugin_runtime_ops::ops_ingress::original_manifest_receiving::original_manifest_metadata_law()}

#[test]fn original_surface_refresh_native_custody(){semio_framework_plugin_runtime_ops::ops_ingress::original_manifest_receiving::original_manifest_surface_refresh_law()}

#[test]fn original_surface_view_read_native_custody(){semio_framework_plugin_runtime_ops::ops_ingress::original_manifest_receiving::original_manifest_surface_read_law()}

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/🧪️tests/🤝️handoff/🦀️.rs"]
mod original_member_handoff;

#[test]fn original_response_metadata_native_custody(){semio_framework_plugin_runtime_ops::ops_typed_operations::original_response::tests::original_response_metadata_source_captures_once_and_restores_cache_on_cancellation()}

#[test]fn original_surface_binding_view_read_native_custody(){semio_framework_plugin_runtime_ops::ops_ingress::original_manifest_receiving::original_manifest_binding_read_law()}

#[path="follow.rs"]mod original_tool_overlay_follow;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🌿️branch/commit-tests.rs"]mod original_commit_metadata;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/checkpoint-tests.rs"]mod original_checkpoint;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/checkpoint-command-tests.rs"]mod original_checkpoint_command;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/checkout-tests.rs"]mod original_checkpoint_checkout;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/🧪️tests/🎮️native-vcs/🦀️.rs"]mod original_member_vcs;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/checkout-command-tests.rs"]mod original_checkout_command;

#[path="ledger_source.rs"]mod production_ledger_source;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/🪆️child-vcs/🧪️tests/🦀️.rs"]mod original_child_vcs_unit;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/🪆️child-vcs/📌️checkpoint/🧪️tests/🦀️.rs"]
mod original_child_checkpoint_preparation;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/💾️binary/📸️export/🧪️tests.rs"]mod original_snapshot_pack_export;

#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/📌️checkpoint/🧪️tests/🦀️.rs"]
mod original_checkpoint_route;
