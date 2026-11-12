//! 📸️ Exact Flow snapshot and local scene sources retain their defining paid close frontiers.
use crate::{FlowContentChild,FlowSnapshot,FlowWorkingScene};
use semio_framework_value::retirement::{RetireOwned,arc_custody::ArcLease};

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
#[doc(hidden)]
pub struct FlowSnapshotSources{schema:String,content:FlowContentChild,scene:Option<ArcLease<FlowWorkingScene>>}

type SceneParts=(Vec<semio_framework_artifact_flow_flow::Widget>,Vec<semio_framework_artifact_flow_flow::SynapseSpec>,flow::OrderedMap<semio_framework_artifact_flow_flow::WidgetLayout>);

impl RetireOwned for FlowWorkingScene{
 type RetirementCursor=<SceneParts as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{self.into_parts().retirement_seed()}
 fn controlled_retirement_supported()->bool{true}
}

impl RetireOwned for FlowSnapshot{
 type RetirementCursor=<FlowSnapshotSources as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{let Self{schema,mut content}=self;let scene=content.take_local_owner::<FlowWorkingScene>().ok().flatten().map(ArcLease::new);FlowSnapshotSources{schema,content,scene}.retirement_seed()}
 fn controlled_retirement_supported()->bool{true}
}

type OwnedFactory=semio_framework_value::OwnedValueRetirementFactory<FlowSnapshot>;
type SharedFactory=semio_framework_value::SharedValueRetirementFactory<FlowSnapshot>;
type ErasedReply<'reply>=semio_framework_value::FactoryProducerReply<'reply,dyn store::ErasedSnapshotRetirement,semio_framework_value::OriginalRetirementFailure>;
type ErasedReplyComplete<'reply>=semio_framework_value::FactoryProducerReplyComplete<'reply,semio_framework_value::OriginalRetirementFailure>;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct SnapshotRetirementFactory;
impl store::SnapshotRetirementFactory<FlowSnapshot> for SnapshotRetirementFactory{
 fn retirement_birth_bytes(&self,source:&semio_framework_value::factory::SharedOriginalStorage<FlowSnapshot>)->usize{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().retirement_birth_bytes(source)}
 fn retire_demands(&self,source:&semio_framework_value::factory::SharedOriginalStorage<FlowSnapshot>)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().retire_demands(source)}
 fn retire_work_demand(&self,source:&semio_framework_value::factory::SharedOriginalStorage<FlowSnapshot>)->Result<u64,semio_framework_value::ValueError>{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().retire_work_demand(source)}
 fn retire<'reply>(&self,source:&mut Option<semio_framework_value::factory::SharedOriginalStorage<FlowSnapshot>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:ErasedReply<'reply>)->ErasedReplyComplete<'reply>{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().retire(source,effect,reply)}
 fn recover_retire<'reply>(&self,source:&mut Option<semio_framework_value::factory::SharedOriginalStorage<FlowSnapshot>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:ErasedReply<'reply>)->ErasedReplyComplete<'reply>{use semio_framework_value::SnapshotRetirementFactory as _;SharedFactory::default().recover_retire(source,effect,reply)}
}
impl store::ArtifactOwnedValueRetirementFactory<FlowSnapshot> for SnapshotRetirementFactory{
 fn retirement_birth_bytes(&self,source:&FlowSnapshot)->usize{use semio_framework_value::ArtifactOwnedValueRetirementFactory as _;OwnedFactory::default().retirement_birth_bytes(source)}
 fn retire_owned_work_demand(&self,source:&FlowSnapshot)->Result<u64,semio_framework_value::ValueError>{use semio_framework_value::ArtifactOwnedValueRetirementFactory as _;OwnedFactory::default().retire_owned_work_demand(source)}
 fn retire_owned_demands(&self,source:&FlowSnapshot)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{use semio_framework_value::ArtifactOwnedValueRetirementFactory as _;OwnedFactory::default().retire_owned_demands(source)}
 fn retire_owned<'reply>(&self,source:&mut Option<FlowSnapshot>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:ErasedReply<'reply>)->ErasedReplyComplete<'reply>{use semio_framework_value::ArtifactOwnedValueRetirementFactory as _;OwnedFactory::default().retire_owned(source,effect,reply)}
 fn recover_retire_owned<'reply>(&self,source:&mut Option<FlowSnapshot>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:ErasedReply<'reply>)->ErasedReplyComplete<'reply>{use semio_framework_value::ArtifactOwnedValueRetirementFactory as _;OwnedFactory::default().recover_retire_owned(source,effect,reply)}
}
