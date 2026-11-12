//! 👥️ Original presence identifiers and camera retain their complete paid ownership.
use super::{FlowPresence,FlowPresenceMutation};
use std::sync::Arc;

semio_framework_value::artifact_retire_struct!(FlowPresence{preview_off_node_ids,camera});

type SharedFactory=semio_framework_value::SharedValueRetirementFactory<FlowPresence>;
type Storage=semio_framework_value::factory::SharedOriginalStorage<FlowPresence>;
type ErasedReply<'reply>=semio_framework_value::FactoryProducerReply<'reply,dyn store::ErasedSnapshotRetirement,semio_framework_value::OriginalRetirementFailure>;
type ErasedReplyComplete<'reply>=semio_framework_value::FactoryProducerReplyComplete<'reply,semio_framework_value::OriginalRetirementFailure>;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct FlowPresenceRetirementFactory;
impl store::SnapshotRetirementFactory<FlowPresence> for FlowPresenceRetirementFactory{
 fn retirement_birth_bytes(&self,source:&Storage)->usize{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().retirement_birth_bytes(source)}
 fn retire_demands(&self,source:&Storage)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().retire_demands(source)}
 fn retire_work_demand(&self,source:&Storage)->Result<u64,semio_framework_value::ValueError>{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().retire_work_demand(source)}
 fn retire<'reply>(&self,source:&mut Option<Storage>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:ErasedReply<'reply>)->ErasedReplyComplete<'reply>{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().retire(source,effect,reply)}
 fn recover_retire<'reply>(&self,source:&mut Option<Storage>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:ErasedReply<'reply>)->ErasedReplyComplete<'reply>{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().recover_retire(source,effect,reply)}
}
pub fn store_disposer()->Box<dyn semio_plugin::ArtifactOwnedDisposer<store::PresenceStore<FlowPresence,FlowPresenceMutation>>>{Box::new(semio_plugin::PresenceStoreOwnedDisposer::new(Arc::new(FlowPresence::default()),|value|value.preview_off_node_ids.is_empty()).expect("default Flow presence has no dynamic identifier owner"))}


