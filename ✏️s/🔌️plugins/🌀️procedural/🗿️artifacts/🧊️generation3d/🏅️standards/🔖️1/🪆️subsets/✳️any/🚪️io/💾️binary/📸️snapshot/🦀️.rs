//! 📦️ Generation3d artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[cfg(test)]
use store::PackError;

/// 📦️ Encodes a `Generation3dSnapshot` to its binary pack form.
pub fn encode(document: &Generation3dSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 🔬️ Batch decode exists only for constitutional equivalence tests; mounted UI code
/// has no production symbol that can reach the whole-document decoder.
#[cfg(test)]
pub fn decode(bytes: &[u8]) -> Result<Generation3dSnapshot, PackError> {
    <Generation3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

use store::mounted_pack_rt as mounted;
/// 📡️ The producer counterpart of [`Generation3dMountedPackSession`]: this artifact's own `P3D3`
/// discriminator followed by the canonical `.spk` stream — the SAME bytes [`encode`] produces,
/// with the outer `\x89SEM` semio container unwrapped, since the mounted cursors decode a pack
/// file and not a semio envelope.
///
/// The mounted ingress route refuses any stream that does not lead with `P3D3` (`admit_byte`'s
/// prefix gate), and a whole-document `ArtifactPack` encode carries the repo-wide container magic
/// and no artifact discriminator — exactly like every committed `🎒️.pack.semio` asset — so it is
/// not admissible there and never was. generation2d builds the same shape inside its own
/// `ArtifactPack` impl (`P2D2` + `pack_rt::encode_document`), which is the cross-artifact
/// inconsistency this keeps out of the pack contract (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn encode_mounted(document: &Generation3dSnapshot) -> Vec<u8> {
    let container = encode(document);
    let (_envelope, canonical) = store::semio_format::unwrap_binary(&container).expect("this artifact's own pack encode is a well-formed semio binary container");
    let mut bytes = Vec::with_capacity(GENERATION3D_MOUNTED_PREFIX.len() + canonical.len());
    bytes.extend_from_slice(&GENERATION3D_MOUNTED_PREFIX);
    bytes.extend_from_slice(&canonical);
    bytes
}

#[path = "🧱️mounted-snapshot-owner-types-and-stepping/🦀️.rs"]
mod mounted_snapshot_owner_types_and_stepping;
pub use mounted_snapshot_owner_types_and_stepping::*;

#[path = "🪢️mounted-snapshot-owner-assignment/🦀️.rs"]
mod mounted_snapshot_owner_assignment;

#[path = "🧳️mounted-pack-session/🦀️.rs"]
mod mounted_pack_session;
pub use mounted_pack_session::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️retained-mounted-laws/🦀️.rs"]
mod retained_mounted_laws;

mod native_codec {
#[cfg(test)] use super::*;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
#[cfg(test)] use semio_framework_artifact_flow_flow::neural::{Atom, Dictionary, Value as NeuralValue};
#[cfg(test)] use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, SynapseSpec, Widget, WidgetLayout};
#[cfg(test)] use semio_framework_artifact_playbook_playbook::{FormGeneration, GenerationPlayState};
#[cfg(test)] use std::collections::BTreeMap;
#[cfg(test)] use semio_framework_dsl_record::native_encoding::{FieldProjectionSource, FieldProjectionView as ProjectionView, projection_path_error};
#[cfg(test)] use semio_framework_artifact_flow_flow::{FlowUi, FlowNodeGui, NodeChrome, FlowPreviewGui, FlowChannelRef};
#[cfg(test)] use semio_framework_artifact_flow_flow::neural::{Tree, Neuron, Synapse};
#[cfg(test)] use semio_framework_value_ordered::ordered::{OrderedMap, OrderedSet};
#[cfg(test)] use semio_framework_value::{DslValue, Number, ValueError};
use semio_s_artifact_procedural_3d_io_text::snapshot::Generation3dSnapshotDsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::generation3d_document_to_dsl;
use semio_s_artifact_procedural_3d_io_text::snapshot::generation3d_document_from_dsl;

impl pack::snapshot::ArtifactPack for Generation3dSnapshotDsl {
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
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Generation3dSnapshotDsl {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}

impl pack::snapshot::ArtifactPack for Generation3dSnapshot {

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <Generation3dSnapshotDsl as pack::snapshot::ArtifactPack>::encode_pack_with(&generation3d_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <Generation3dSnapshotDsl as pack::snapshot::ArtifactPack>::decode_pack_with(bytes, options)?;
        generation3d_document_from_dsl(parsed).map_err(store::os_store::serialization::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Generation3dSnapshotDsl as pack::snapshot::ArtifactPack>::record_spec()
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Generation3dSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()) }
}
}
