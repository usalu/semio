//! 🧵️ Generation3d retained provisional commands: typed input landing, relative transforms and the original mutation dispatch.
pub use semio_s_artifact_procedural_retained_source::{GenerationPlayState,retained_generation_fold,retained_host_scratch,retained_native_provisional_fold,retained_scratch_driver,retained_scratch_scan};
#[cfg(test)]
pub use semio_s_artifact_procedural_retained_source::retained_scratch_testing;
pub use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;

#[path="📥️provisional/🎛️input/🧬️admit/🦀️.rs"]
pub mod retained_typed_input_admission;
#[path="📥️provisional/✏️dispatch/🦀️.rs"]
pub mod retained_provisional_dispatch;
#[path="📥️provisional/🎛️input/💎️literal/🦀️.rs"]
pub mod retained_typed_literal;
#[path="📥️provisional/🎛️input/⚖️current/🦀️.rs"]
pub mod retained_typed_current;
#[path="📥️provisional/🎛️input/🦀️.rs"]
pub mod retained_typed_input;
#[path="📥️provisional/🎛️input/📚️params/🔎️read/🦀️.rs"]
pub mod retained_native_param_read;
#[path="📥️provisional/🎛️input/📚️params/✏️patch/🦀️.rs"]
pub mod retained_native_param_patch;
#[path="📥️provisional/🧭️transform/🦀️.rs"]
pub mod retained_provisional_transform;
#[cfg(test)]
#[path="📥️provisional/🎛️input/🧬️admit/🧪️laws/🦀️.rs"]
mod retained_typed_input_admission_laws;
#[cfg(test)]
#[path="📥️provisional/🎛️input/⚖️current/🧪️laws/🦀️.rs"]
mod retained_typed_current_laws;
#[cfg(test)]
#[path="📥️provisional/🎛️input/🧪️laws/🦀️.rs"]
mod retained_typed_input_laws;
#[cfg(test)]
#[path="📥️provisional/🎛️input/📚️params/🔎️read/🧪️laws/🦀️.rs"]
mod retained_native_param_read_laws;
#[cfg(test)]
#[path="📥️provisional/🧭️transform/🧪️laws/🦀️.rs"]
mod retained_provisional_transform_laws;

#[cfg(test)]
type HostScratchFixtureMutation=Generation3dMutation;

#[cfg(test)]
fn host_scratch_test_original(source:&semio_framework_artifact_flow_flow::FlowHostSnapshot,wire:&serde_json::Value)->semio_framework_artifact_flow_flow::FlowHostSnapshot{let mut snapshot=semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot{host_snapshot:source.clone(),generation:Default::default()};let mutation:Generation3dMutation=semio_framework_pack_json::from_json_str(&wire.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let _=semio_s_artifact_procedural_3d_mutations::central_apply::apply_generation3d_mutation(&mut snapshot,&mutation);mutation.retire_cold();let result=std::mem::take(&mut snapshot.host_snapshot);snapshot.retire_cold();result}
