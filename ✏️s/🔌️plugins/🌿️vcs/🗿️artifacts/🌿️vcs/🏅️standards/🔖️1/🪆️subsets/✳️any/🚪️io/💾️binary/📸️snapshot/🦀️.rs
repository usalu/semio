//! 📦️ VCS artifact — native binary pack codec (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §1 CORRECTION: relocated here from
//! `🚪️io/💾️binary/📸️snapshot` — the real `pack::snapshot::ArtifactPack for VcsSnapshot` impl moved with
//! it; `🧬️schema` keeps only the `VcsSnapshot` type). `pack::snapshot::ArtifactDsl`'s twin impl sits in the
//! sibling `📝️text` facet.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::VcsSnapshot;
use store::PackError;

//#region 🔖️ArtifactPackCodec
impl pack::snapshot::ArtifactPack for VcsSnapshot {
    
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for VcsSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec()->Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec>{Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())}
}
//#endregion 🔖️ArtifactPackCodec

/// 📦️ Encodes a `VcsSnapshot` to its binary pack form.
pub fn encode(projection: &VcsSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(projection)
}

/// 📖️ Decodes a `VcsSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<VcsSnapshot, PackError> {
    <VcsSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
//#endregion 🧪️Tests

/// 🫴️ Declares this owner’s actual controlled native record factories.
impl pack::snapshot::ArtifactNativeSnapshot for VcsSnapshot {
 fn decode_native_snapshot(payload:pack::snapshot::NativeSnapshotInput<'_>,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{store::decode_native_snapshot_record(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|Self::__dsl_from_record_controlled(record,native),control)}
 fn encode_native_snapshot(&self,encoding:pack::snapshot::NativeSnapshotEncoding,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<store::io_schema::IoPayload,semio_framework_value::ValueError>{store::encode_native_snapshot_record(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
}

//#region 🧩️ArtifactItems
pub mod artifact_items {
pub const VCS_DOCUMENT_SCHEMA: &str = "vcs.vcs";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.vcs.vcs")?)
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.vcs.schema.artifact")?, ArtifactCapabilityKind::schema()).descriptor(b"s.vcs.vcs")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.vcs.vcs")?)?)?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.vcs.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.vcs.vcs.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.vcs.vcs.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.vcs.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.vcs.vcs@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.vcs.vcs@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.vcs.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.vcs.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"vcs.vcs:vcs")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "vcs.vcs")?)?
                .claim(ArtifactIdentityClaim::codec_extension("vcs.vcs", "vcs")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.vcs.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"VCS")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "VCS")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.vcs.vcs.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"VCS")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "VCS")?)?)
}

//#endregion 🔖️Declaration
}
//#endregion 🧩️ArtifactItems4
