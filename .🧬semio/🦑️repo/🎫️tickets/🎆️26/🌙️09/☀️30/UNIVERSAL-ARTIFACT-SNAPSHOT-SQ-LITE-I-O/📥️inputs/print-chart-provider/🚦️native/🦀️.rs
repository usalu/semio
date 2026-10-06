//! 🚦️ Exact Chart Record envelopes and same-caller controlled native ownership.
use crate::ChartSnapshot;
use protocol as store;
use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as R,BorrowedShape as H,RecordLayout,__rt::DecodedFieldOwner};
use semio_framework_value::{FromValue,NativeEncodeControl,ValueError,ValueRefusalKind as K,native_encoding::NativeEncodeProgress};
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase};
use semio_framework_diagnostic::{TextError,TextSpan};
static FIELD:[F;1]=[F::new(0,"chart",H::Value)];
fn spec()->R{R{keyword:None,layout:RecordLayout::Inline,fields:&FIELD}}
pub(super) fn decode(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<ChartSnapshot,ValueError>{let mut owner=DecodedFieldOwner::new(store::decode_sqlite_snapshot_record_native(payload,"print.chart",ChartSnapshot::__dsl_spec_producer(),|record,native|ChartSnapshot::__dsl_from_record_controlled(record,native),control)?,ChartSnapshot::retire_decoded);super::sqlite::forecast(owner.as_mut(),control,SqliteSnapshotPhase::DecodeNative)?;Ok(owner.take())}
pub(super) fn preflight(snapshot:&ChartSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{super::sqlite::forecast(snapshot,control,SqliteSnapshotPhase::EncodeNative)?;let component=match encoding{SnapshotEncoding::Binary=>store::semio_format::Component::Pack,SnapshotEncoding::Text=>store::semio_format::Component::Dsl};let maximum=control.limits().max_file_bytes.checked_sub(store::semio_format::declared_envelope_prefix_len("print.chart",component,1)?).ok_or_else(||ValueError::new(K::OwnershipLimit,"Chart file ceiling cannot contain its declared envelope"))?;
 control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint|{let mut progress=|event:NativeEncodeProgress|checkpoint(event.completed,event.total);let mut native=NativeEncodeControl::new(remaining,&mut progress);let result=match encoding{SnapshotEncoding::Text=>semio_framework_dsl_record::measure_print_borrowed(snapshot,&spec(),maximum,&mut native),SnapshotEncoding::Binary=>pack::record::measure_document_borrowed(snapshot,&spec(),&Default::default(),&mut native)}.and_then(|length|if length>maximum{Err(ValueError::new(K::OwnershipLimit,"Chart native output exceeds file byte ceiling"))}else{Ok(())});(result,native.owned_bytes())})?
}
pub(super) fn encode(snapshot:&ChartSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{preflight(snapshot,encoding,control)?;store::encode_sqlite_snapshot_record_native(encoding,"print.chart",ChartSnapshot::__dsl_spec_producer(),|native|snapshot.__dsl_to_record_controlled(native),control)}
impl store::ArtifactDsl for ChartSnapshot{
 const EXTENSION:&'static str="chart";
 fn envelope_id()->&'static str{"print.chart"}
 fn parse_dsl(text:&str)->Result<Self,TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|error|TextError::from_value_error(error.into_value_error(),TextSpan::at(1,1)))?;if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(TextError::new(K::InvalidValue,"Chart logical Text envelope mismatch",TextSpan::at(1,1)))}Self::__dsl_from_record(&semio_framework_dsl_record::parse_exact(body,&Self::__dsl_spec(),&Default::default())?)}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("declared Chart envelope");store::semio_format::wrap_text(&envelope,&body)}
}
impl store::ArtifactPack for ChartSnapshot{
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Self::__dsl_spec())}
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let body=store::os_pack::value::encode_document(&Self::__dsl_spec(),&self.__dsl_to_record(),options).map_err(store::PackError::from)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&body))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,body)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::from(ValueError::new(K::InvalidValue,"Chart logical Pack envelope mismatch")))}Self::__dsl_from_record(&store::os_pack::value::decode_document(&body,&Self::__dsl_spec(),options).map_err(store::PackError::from)?.0).map_err(store::PackError::from)}
}
