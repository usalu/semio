//! 📦️ Generation2d artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
#[cfg(test)]
use store::PackError;

/// 📦️ Encodes a `Generation2dSnapshot` to its binary pack form.
pub fn encode(document: &Generation2dSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 🔬️ Batch decode exists only for constitutional equivalence tests; mounted UI code
/// has no production symbol that can reach the whole-document decoder.
#[cfg(test)]
pub fn decode(bytes: &[u8]) -> Result<Generation2dSnapshot, PackError> {
    <Generation2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
use store::mounted_pack_rt as mounted;

//#region 🔖️MountedCanonicalPackSession

#[path = "🏷️mounted-owner-state/🦀️.rs"]
mod mounted_owner_state;
pub use mounted_owner_state::Generation2dMountedTypedSnapshotOwner;
#[cfg(test)]
use mounted_owner_state::GENERATION2D_MOUNTED_PREFIX;

#[path = "🔤️owner-string-targets/🦀️.rs"]
mod owner_string_targets;

#[path = "🧮️owner-values-and-dictionaries/🦀️.rs"]
mod owner_values_and_dictionaries;

#[path = "🗃️owner-containers-and-widgets/🦀️.rs"]
mod owner_containers_and_widgets;

#[path = "🔌️owner-pack-protocol/🦀️.rs"]
mod owner_pack_protocol;
pub use owner_pack_protocol::{generation2d_mounted_pack_session, Generation2dMountedPackSession};

//#endregion 🔖️MountedCanonicalPackSession

#[cfg(test)]
#[path = "🧪️tests/🔬️retained-mounted-laws/🦀️.rs"]
mod retained_mounted_laws;

mod native_codec {
#[cfg(test)] use super::*;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
#[cfg(test)] use semio_framework_artifact_flow_flow::neural::{Atom, Dictionary, Value as NeuralValue};
#[cfg(test)] use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, SynapseSpec, Widget, WidgetLayout};
#[cfg(test)] use semio_framework_artifact_playbook_playbook::{FormGeneration, GenerationPlayState};
#[cfg(test)] use std::collections::BTreeMap;
use semio_s_artifact_procedural_2d_io_text::snapshot::Generation2dSnapshotDsl;
use semio_s_artifact_procedural_2d_io_text::snapshot::generation2d_document_to_dsl;
use semio_s_artifact_procedural_2d_io_text::snapshot::generation2d_document_from_dsl;

impl pack::snapshot::ArtifactPack for Generation2dSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Generation2dSnapshotDsl {

}

impl pack::snapshot::ArtifactPack for Generation2dSnapshot {

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let document = generation2d_document_to_dsl(self);
        let inner = store::pack_rt::encode_document(&Generation2dSnapshotDsl::__dsl_spec(), &document.__dsl_to_record(), options)?;
        let mut bytes = Vec::with_capacity(4 + inner.len());
        bytes.extend_from_slice(b"P2D2");
        bytes.extend_from_slice(&inner);
        Ok(bytes)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if !bytes.starts_with(b"P2D2") {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "generation2d pack discriminator mismatch")));
        }
        let (record, _report) = store::pack_rt::decode_document(&bytes[4..], &Generation2dSnapshotDsl::__dsl_spec(), options)?;
        let parsed = Generation2dSnapshotDsl::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)?;
        generation2d_document_from_dsl(parsed).map_err(store::os_store::serialization::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Generation2dSnapshotDsl as pack::snapshot::ArtifactPack>::record_spec()
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Generation2dSnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()) }
}
}
