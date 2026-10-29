//! ♻️ Retirement of the GIS map owned values: the snapshot and mutation retirement factories. See [`super`].

use super::*;

pub const GIS_MAP_OWNED_FIELD_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct GisMapSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<GisMapSnapshot> for GisMapSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &GisMapSnapshot) -> usize { semio_framework_value::retirement::owned_retirement_birth_bytes::<GisMapSnapshot>() }

    fn retire_owned(&self, value: GisMapSnapshot, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, GisMapSnapshot)> {
        semio_framework_value::retirement::admit_owned_retirement(value, grant)
    }
}

impl store::SnapshotRetirementFactory<GisMapSnapshot> for GisMapSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<GisMapSnapshot>) -> usize { semio_framework_value::shared_retirement_birth_bytes::<GisMapSnapshot>() }

    fn retire(&self, snapshot: std::sync::Arc<GisMapSnapshot>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<GisMapSnapshot>)> {
        semio_framework_value::admit_shared_retirement(snapshot, grant, true)
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct GisMapMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<GisMapMutation> for GisMapMutationRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &GisMapMutation) -> usize { semio_framework_value::retirement::owned_retirement_birth_bytes::<GisMapMutation>() }

    fn retire_owned(&self, value: GisMapMutation, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, GisMapMutation)> {
        semio_framework_value::retirement::admit_owned_retirement(value, grant)
    }
}
