//! 🗃️ Original Flow snapshot and uninhabited parent mutation ownership use full paid factory grants.
use crate::{FlowMutation,FlowSnapshot};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,retirement::RetireOwned};
pub use semio_s_artifact_flow_flow_schema::standards::v1::subsets::any::schema::retirement::snapshot;
pub use snapshot::SnapshotRetirementFactory;

pub fn store_owners_source_demands()->Result<semio_framework_value::RetirementDemand,ValueError>{
 Ok(semio_framework_value::RetirementDemand{capacity_bytes:store::os_store::document_store_owners::DocumentStoreOwners::<FlowSnapshot,FlowMutation>::source_birth_bytes::<SnapshotRetirementFactory,SnapshotRetirementFactory,MutationRetirementFactory,store::ArtifactStoreCursorDisposer<FlowSnapshot,FlowMutation>>()?,depth:1,..Default::default()})
}

pub fn store_owners(grant:RetainedCloneGrant)->Result<(store::os_store::document_store_owners::DocumentStoreOwners<FlowSnapshot,FlowMutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<FlowSnapshot,FlowMutation>>{
 store::os_store::document_store_owners::DocumentStoreOwners::admit_source_constructor(grant,||(SnapshotRetirementFactory,SnapshotRetirementFactory,MutationRetirementFactory,store::ArtifactStoreCursorDisposer::<FlowSnapshot,FlowMutation>::new()))
}

impl RetireOwned for FlowMutation{
 type RetirementCursor=semio_framework_value::retirement::Leaf<()>;
 fn retirement_seed(self)->Self::RetirementCursor{match self{}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(0)}
 fn controlled_retirement_supported()->bool{true}
}

type SharedFactory=semio_framework_value::SharedValueRetirementFactory<FlowMutation>;
type OwnedFactory=semio_framework_value::OwnedValueRetirementFactory<FlowMutation>;
type Storage=semio_framework_value::factory::SharedOriginalStorage<FlowMutation>;
type ErasedReply<'reply>=semio_framework_value::FactoryProducerReply<'reply,dyn store::ErasedSnapshotRetirement,semio_framework_value::OriginalRetirementFailure>;
type ErasedReplyComplete<'reply>=semio_framework_value::FactoryProducerReplyComplete<'reply,semio_framework_value::OriginalRetirementFailure>;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct MutationRetirementFactory;
impl store::SnapshotRetirementFactory<FlowMutation> for MutationRetirementFactory{
 fn retirement_birth_bytes(&self,source:&Storage)->usize{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().retirement_birth_bytes(source)}
 fn retire_demands(&self,source:&Storage)->Result<semio_framework_value::RetirementDemand,ValueError>{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().retire_demands(source)}
 fn retire_work_demand(&self,source:&Storage)->Result<u64,ValueError>{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().retire_work_demand(source)}
 fn retire<'reply>(&self,source:&mut Option<Storage>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:ErasedReply<'reply>)->ErasedReplyComplete<'reply>{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().retire(source,effect,reply)}
 fn recover_retire<'reply>(&self,source:&mut Option<Storage>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:ErasedReply<'reply>)->ErasedReplyComplete<'reply>{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().recover_retire(source,effect,reply)}
}
impl store::ArtifactOwnedValueRetirementFactory<FlowMutation> for MutationRetirementFactory{
 fn retirement_birth_bytes(&self,source:&FlowMutation)->usize{use semio_framework_value::ArtifactOwnedValueRetirementFactory as _;OwnedFactory::default().retirement_birth_bytes(source)}
 fn retire_owned_work_demand(&self,source:&FlowMutation)->Result<u64,ValueError>{use semio_framework_value::ArtifactOwnedValueRetirementFactory as _;OwnedFactory::default().retire_owned_work_demand(source)}
 fn retire_owned_demands(&self,source:&FlowMutation)->Result<semio_framework_value::RetirementDemand,ValueError>{use semio_framework_value::ArtifactOwnedValueRetirementFactory as _;OwnedFactory::default().retire_owned_demands(source)}
 fn retire_owned<'reply>(&self,source:&mut Option<FlowMutation>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:ErasedReply<'reply>)->ErasedReplyComplete<'reply>{use semio_framework_value::ArtifactOwnedValueRetirementFactory as _;OwnedFactory::default().retire_owned(source,effect,reply)}
 fn recover_retire_owned<'reply>(&self,source:&mut Option<FlowMutation>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:ErasedReply<'reply>)->ErasedReplyComplete<'reply>{use semio_framework_value::ArtifactOwnedValueRetirementFactory as _;OwnedFactory::default().recover_retire_owned(source,effect,reply)}
}
