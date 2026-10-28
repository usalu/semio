//! 📦️ FEM 3D artifact — binary document surface + laws (constitutional: pack).

use crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes a `Fem3dSnapshot` to its binary pack form.
pub fn encode(document: &Fem3dSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Fem3dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Fem3dSnapshot, PackError> {
    <Fem3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-protocol-conformance/🦀️.rs"]
mod semio_protocol_conformance;

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemAnalysisSettings;
use crate::standards::v1::subsets::any::schema::structure::FemCombination;
use crate::standards::v1::subsets::any::schema::structure::FemElement;
use crate::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_fem_material::volumetric::FemMaterialValue;
use semio_s_fem_node::volumetric::FemNodeValue;
use crate::standards::v1::subsets::any::schema::structure::FemSection;
use crate::standards::v1::subsets::any::schema::structure::FemSolid;
use crate::standards::v1::subsets::any::schema::structure::FemSupport;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

impl pack::snapshot::ArtifactPack for Fem3dSnapshot {
 
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
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Fem3dSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec()->Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec>{Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())}
}
}
#[cfg(test)] pub use snapshot_codec::*;

/// 🫴️ Declares this owner’s actual controlled native record factories.
impl store::ArtifactNativeSnapshot for Fem3dSnapshot {
 fn decode_native_snapshot(payload:store::NativeSnapshotInput<'_>,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{store::decode_native_snapshot_record(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{Self::validate_interaction_record(record,native)?;Self::__dsl_from_record_controlled(record,native)},control)}
 fn encode_native_snapshot(&self,encoding:store::NativeSnapshotEncoding,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<store::io_schema::IoPayload,semio_framework_value::ValueError>{store::encode_native_snapshot_record(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
}

impl store::ArtifactPackReceiving for Fem3dSnapshot{
 /// 🫴️ The original pack parser, field graph, authoring rows and persistent output share the caller's receiving and retirement frames.
 fn receive_pack(bytes:&[u8],owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,semio_framework_value::ValueError>{
  use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};
  use semio_framework_dsl_record::{BorrowedDslRecord,RecordValue};
  use pack::record::RetainedRecordReceiving;
  owner.receive_nested::<RecordValue,Self>(|slot,original|{
   let body=store::semio_format::unwrap_binary_controlled(bytes,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1,original.native())?;
   *slot=Some(original.receive_nested::<RetainedRecordReceiving,RecordValue>(|cursor,original|{
    let grant=original.remaining_grant();
    let Some((receiving,progress))=RetainedRecordReceiving::admit_document(body,(0,body.len()),<Fem3dSnapshotValue as BorrowedDslRecord>::RECORD,protocol::PackLimits::default(),grant)? else{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original FEM pack grant cannot admit its receiving parser"));};
    *cursor=Some(receiving);original.record_progress(progress)?;
    original.drive_cursor(cursor,|cursor,native,grant|{
     if cursor.is_ready(){let mut out=None;match cursor.take_into(&mut out,grant){Ok(p)=>(Ok(out),p),Err(e)=>{let p=e.retained_progress();(Err(e),p)}}}else{match cursor.step(body,native,grant){Ok(p)=>(Ok(None),p),Err(e)=>{let p=e.retained_progress();(Err(e),p)}}}
    })
   })?);
   Self::receive_exchange_record(slot.as_ref().unwrap(),original)
  })
 }
}
