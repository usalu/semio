//! ♻️ Retirement of the GIS map owned values: the snapshot and mutation retirement factories delegate to the framework's exact owned and shared value factories. See [`super`].

use super::*;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory as _, SnapshotRetirementFactory as _};

pub const GIS_MAP_OWNED_FIELD_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

type OwnedFactory<T> = semio_framework_value::OwnedValueRetirementFactory<T>;
type SharedFactory<T> = semio_framework_value::SharedValueRetirementFactory<T>;
type ErasedReply<'reply> = semio_framework_value::FactoryProducerReply<'reply, dyn store::ErasedSnapshotRetirement, semio_framework_value::OriginalRetirementFailure>;
type ErasedReplyComplete<'reply> = semio_framework_value::FactoryProducerReplyComplete<'reply, semio_framework_value::OriginalRetirementFailure>;

macro_rules! gis_map_owned_factory {
    ($value:ty) => {
        fn retirement_birth_bytes(&self, value: &$value) -> usize { OwnedFactory::<$value>::default().retirement_birth_bytes(value) }
        fn retire_owned_work_demand(&self, value: &$value) -> Result<u64, semio_framework_value::ValueError> { OwnedFactory::<$value>::default().retire_owned_work_demand(value) }
        fn retire_owned_demands(&self, value: &$value) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { OwnedFactory::<$value>::default().retire_owned_demands(value) }
        fn retire_owned<'reply>(&self, value: &mut Option<$value>, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>, reply: ErasedReply<'reply>) -> ErasedReplyComplete<'reply> { OwnedFactory::<$value>::default().retire_owned(value, effect, reply) }
        fn recover_retire_owned<'reply>(&self, value: &mut Option<$value>, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>, reply: ErasedReply<'reply>) -> ErasedReplyComplete<'reply> { OwnedFactory::<$value>::default().recover_retire_owned(value, effect, reply) }
    };
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct GisMapSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<GisMapSnapshot> for GisMapSnapshotRetirementFactory {
    gis_map_owned_factory!(GisMapSnapshot);
}

impl store::SnapshotRetirementFactory<GisMapSnapshot> for GisMapSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, snapshot: &std::sync::Arc<GisMapSnapshot>) -> usize { SharedFactory::<GisMapSnapshot>::default().retirement_birth_bytes(snapshot) }
    fn retire_demands(&self, snapshot: &std::sync::Arc<GisMapSnapshot>) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { SharedFactory::<GisMapSnapshot>::default().retire_demands(snapshot) }
    fn retire_work_demand(&self, snapshot: &std::sync::Arc<GisMapSnapshot>) -> Result<u64, semio_framework_value::ValueError> { SharedFactory::<GisMapSnapshot>::default().retire_work_demand(snapshot) }
    fn retire<'reply>(&self, snapshot: &mut Option<std::sync::Arc<GisMapSnapshot>>, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>, reply: ErasedReply<'reply>) -> ErasedReplyComplete<'reply> { SharedFactory::<GisMapSnapshot>::default().retire(snapshot, effect, reply) }
    fn recover_retire<'reply>(&self, snapshot: &mut Option<std::sync::Arc<GisMapSnapshot>>, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>, reply: ErasedReply<'reply>) -> ErasedReplyComplete<'reply> { SharedFactory::<GisMapSnapshot>::default().recover_retire(snapshot, effect, reply) }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct GisMapMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<GisMapMutation> for GisMapMutationRetirementFactory {
    gis_map_owned_factory!(GisMapMutation);
}
