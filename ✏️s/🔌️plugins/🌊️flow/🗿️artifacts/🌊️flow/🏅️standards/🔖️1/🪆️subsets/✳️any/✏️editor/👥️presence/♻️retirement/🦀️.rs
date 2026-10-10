//! 👥️ Original presence identifiers and camera retain their complete paid ownership.
use super::{FlowPresence,FlowPresenceMutation};
use std::sync::Arc;
use semio_framework_value::{ValueError,RetainedCloneGrant,RetainedCloneProgress};

semio_framework_value::artifact_retire_struct!(FlowPresence{preview_off_node_ids,camera});

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct FlowPresenceRetirementFactory;
impl store::SnapshotRetirementFactory<FlowPresence> for FlowPresenceRetirementFactory{
 fn retirement_birth_bytes(&self,_source:&Arc<FlowPresence>)->usize{semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<FlowPresence>()}
 fn retire(&self,source:Arc<FlowPresence>,grant:RetainedCloneGrant)->Result<(Box<dyn store::ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<FlowPresence>)>{semio_framework_value::retirement::shared::admit_shared_retirement(source,grant,true)}
}
pub fn store_disposer()->Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<FlowPresence,FlowPresenceMutation>>>{Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(Arc::new(FlowPresence::default()),|value|value.preview_off_node_ids.is_empty()).expect("default Flow presence has no dynamic identifier owner"))}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
