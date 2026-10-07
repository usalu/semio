//! 📏️ Borrowed flat Forms records bound escaped literals, field syntax and native framing.
use super::FormsSnapshot;
use semio_framework_value::{ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,artifact::NativeEncodingBound};
const FIELD_BYTES:usize=18+24+16;
const RECORD_BYTES:usize=16;
/// 🫳️ Pays only admission scratch while forecasting every actual flat native record field.
pub(crate) fn preflight(snapshot:&FormsSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let counts=crate::standards::v1::subsets::any::io::sqlite::snapshot::admission::forecast(snapshot,control,SqliteSnapshotPhase::EncodeNative)?;
 let component=match encoding{SnapshotEncoding::Binary=>store::semio_format::Component::Pack,SnapshotEncoding::Text=>store::semio_format::Component::Dsl};
 let prefix=store::semio_format::declared_envelope_prefix_len(<FormsSnapshot as store::ArtifactDsl>::envelope_id(),component,1)?;
 let mut bound=NativeEncodingBound::new(control)?;
 bound.add(prefix.checked_add(24*FIELD_BYTES+7*RECORD_BYTES).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Forms native fixed syntax bound overflow"))?)?;
 bound.repeated(counts.bytes,6)?;
 for(count,fields)in[(counts.steps,4),(counts.questions,26),(counts.options,2),(counts.fields,3),(counts.responses,4),(counts.answers,4),(counts.values,9),(counts.conditions,7),(counts.object_members,2)]{bound.repeated(count,fields*FIELD_BYTES+RECORD_BYTES)?;}
 for count in[counts.array_elements,counts.condition_items]{bound.repeated(count,21)?;}
 bound.finish()
}
