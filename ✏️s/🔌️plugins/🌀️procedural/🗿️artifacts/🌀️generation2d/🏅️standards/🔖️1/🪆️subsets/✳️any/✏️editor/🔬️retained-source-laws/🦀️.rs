//! 🔬️ Generation2d laws of the shared retained source: rows, scan, overlay, host scratch, projection, generation fold and scene preparation.
#![cfg(test)]
use semio_s_artifact_procedural_retained_source::{GenerationPlayState,FormGeneration,retained_flow_overlay,retained_generation_fold,retained_host_scratch,retained_native_provisional_fold,retained_projected_clone,retained_provisional_scene,retained_scene_base,retained_scratch_driver,retained_scratch_rows,retained_scratch_scan,retained_scratch_testing};
use semio_s_artifact_procedural_2d_retained_provisional::retained_provisional_dispatch::ProvisionalFolds as ProvisionalSceneTestFolds;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_s_artifact_procedural_2d_editor::Generation2dPlayApp;
use semio_plugin::EditorApp;

type ProvisionalSceneTestApp=EditorApp<Generation2dPlayApp>;
type HostScratchFixtureMutation=Generation2dMutation;
fn host_scratch_test_domain()->retained_host_scratch::HostScratchDomain{retained_host_scratch::HostScratchDomain::Generation2d}
fn host_scratch_test_original(source:&semio_framework_artifact_flow_flow::FlowHostSnapshot,wire:&serde_json::Value)->semio_framework_artifact_flow_flow::FlowHostSnapshot{let mut snapshot=Generation2dSnapshot{host_snapshot:source.clone(),generation:Default::default()};let mutation:Generation2dMutation=semio_framework_pack_json::from_json_str(&wire.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let _=semio_s_artifact_procedural_2d_mutations::central_apply::apply_generation2d_mutation(&mut snapshot,&mutation);mutation.retire_cold();let result=std::mem::take(&mut snapshot.host_snapshot);snapshot.retire_cold();result}
fn generation_fold_test_original(source:&GenerationPlayState,wire:&serde_json::Value)->(GenerationPlayState,bool){let mut snapshot=Generation2dSnapshot{host_snapshot:Default::default(),generation:source.clone().into()};let mutation:Generation2dMutation=semio_framework_pack_json::from_json_str(&wire.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let accepted=semio_s_artifact_procedural_2d_mutations::central_apply::apply_generation2d_mutation(&mut snapshot,&mutation).is_ok();mutation.retire_cold();let result=snapshot.generation.as_state().clone();snapshot.retire_cold();(result,accepted)}

#[path="../../../../../../../../🫀️foundation/🧬️generation/🌊️flow-overlay/🧪️laws/🦀️.rs"]
mod retained_flow_overlay_laws;
#[path="../../../../../../../../🫀️foundation/🌊️flow-source/📋️rows/🧪️tests/🦀️.rs"]
mod retained_scratch_rows_laws;
#[path="../../../../../../../../🫀️foundation/🌊️flow-source/🔎️rows/🧪️tests/🦀️.rs"]
mod retained_scratch_scan_laws;
#[path="../../../../../../../../🫀️foundation/🌊️flow-source/✏️host-scratch/🧪️laws/🦀️.rs"]
mod retained_host_scratch_laws;
#[path="../../../../../../../../🫀️foundation/🌊️flow-source/📋️projection/🧪️laws/🦀️.rs"]
mod retained_projected_clone_laws;
#[path="../../../../../../../../🫀️foundation/🧬️generation/🧵️fold/🧪️tests.rs"]
mod retained_generation_fold_laws;
#[path="../../../../../../../../🫀️foundation/🌊️flow-source/🎬️scene/🧪️.rs"]
mod retained_provisional_scene_laws;
