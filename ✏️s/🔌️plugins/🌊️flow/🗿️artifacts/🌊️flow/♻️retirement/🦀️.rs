//! 🗃️ Original Flow snapshot and uninhabited parent mutation ownership use full paid factory grants.
use crate::{FlowMutation,FlowSnapshot};
use std::sync::Arc;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,retirement::{RetireOwned,RetirementCursor}};
#[path="📸️snapshot/🦀️.rs"]
mod snapshot;
pub use snapshot::SnapshotRetirementFactory;

pub fn store_owners_source_demands()->Result<semio_framework_value::RetirementDemand,ValueError>{
 Ok(semio_framework_value::RetirementDemand{capacity_bytes:store::DocumentStoreOwners::<FlowSnapshot,FlowMutation>::source_birth_bytes::<SnapshotRetirementFactory,SnapshotRetirementFactory,MutationRetirementFactory,store::ArtifactStoreCursorDisposer<FlowSnapshot,FlowMutation>>()?,depth:1,..Default::default()})
}

pub fn store_owners(grant:RetainedCloneGrant)->Result<(store::DocumentStoreOwners<FlowSnapshot,FlowMutation>,RetainedCloneProgress),store::DocumentStoreOwnersAdmissionError<FlowSnapshot,FlowMutation>>{
 store::DocumentStoreOwners::admit_source_constructor(grant,||(SnapshotRetirementFactory,SnapshotRetirementFactory,MutationRetirementFactory,store::ArtifactStoreCursorDisposer::<FlowSnapshot,FlowMutation>::new()))
}

impl RetireOwned for FlowMutation{
 fn retirement(self)->Box<dyn RetirementCursor>{match self{}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(0)}
 fn controlled_retirement_supported()->bool{true}
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct MutationRetirementFactory;
impl store::SnapshotRetirementFactory<FlowMutation> for MutationRetirementFactory{
 fn retirement_birth_bytes(&self,_source:&Arc<FlowMutation>)->usize{semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<FlowMutation>()}
 fn retire(&self,source:Arc<FlowMutation>,grant:RetainedCloneGrant)->Result<(Box<dyn store::ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<FlowMutation>)>{semio_framework_value::retirement::shared::admit_shared_retirement(source,grant,true)}
}
impl store::ArtifactOwnedValueRetirementFactory<FlowMutation> for MutationRetirementFactory{
 fn retirement_birth_bytes(&self,_source:&FlowMutation)->usize{semio_framework_value::retirement::controlled::controlled_retirement_birth_bytes::<FlowMutation>()}
 fn retire_owned(&self,source:FlowMutation,grant:RetainedCloneGrant)->Result<(Box<dyn store::ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,FlowMutation)>{semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(source,grant).map(|(owner,p)|(owner as Box<dyn store::ErasedSnapshotRetirement>,p))}
}
