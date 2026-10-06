//! 🏗️ typed Process3d backing, field and retained owner construction controls.
use super::Process3dSnapshot;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SqliteValue,ValueError,artifact::{Cell,Reconstruction}};
type Result<T>=std::result::Result<T,ValueError>;
/// 📦️ Pays complete typed owner collection backing before constructing its first item.
pub(super) fn collection<T>(count:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<T>>{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,count)?;store::sqlite_snapshot::transfer::reserve(count,control)}
/// 🧵 Copies literal text through the existing semantic and cumulative backing authority.
pub(super) fn text(value:&str,control:&mut SqliteSnapshotControl<'_>)->Result<String>{Reconstruction::new(control)?.text(value)}
/// 🧬️ Copies intrinsic octets through the existing semantic and cumulative backing authority.
pub(super) fn blob(value:&[u8],control:&mut SqliteSnapshotControl<'_>)->Result<Vec<u8>>{Reconstruction::new(control)?.blob(value)}
/// 🫳️ Uses borrowed declared cells whose actual owned row frontier is charged by Projection.
pub(super) fn cell(value:&SqliteValue)->Cell<'_>{match value{SqliteValue::Null=>Cell::Null,SqliteValue::Integer(value)=>Cell::Integer(*value),SqliteValue::Real(value)=>Cell::Real(*value),SqliteValue::Text(value)=>Cell::Text(value),SqliteValue::Blob(value)=>Cell::Blob(value)}}
/// 🛡️ Retains completed typed partial collections until publication or their declared retirement.
pub(super) fn guarded_collection<T:semio_framework_dsl_record::DslField>(count:usize,control:&mut SqliteSnapshotControl<'_>)->Result<semio_framework_dsl_record::__rt::DecodedFieldOwner<Vec<T>>>{Ok(semio_framework_dsl_record::__rt::DecodedFieldOwner::new(collection(count,control)?,<Vec<T> as semio_framework_dsl_record::DslField>::retire_decoded))}
/// 🏷️ Guards declared statement variants whose owner implements DslVariants rather than DslField.
pub(super) fn guarded_variants<T:semio_framework_dsl_record::DslVariants>(count:usize,control:&mut SqliteSnapshotControl<'_>)->Result<semio_framework_dsl_record::__rt::DecodedFieldOwner<Vec<T>>>{Ok(semio_framework_dsl_record::__rt::DecodedFieldOwner::new(collection(count,control)?,|values:Vec<T>|{for value in values{T::retire_decoded_variant(value);}}))}
/// ♻️ Retires the finite declared domain fields through their canonical typed constructor authority.
pub(super) fn retire(snapshot:Process3dSnapshot){<Process3dSnapshot as semio_framework_dsl_record::DslField>::retire_decoded(snapshot);}
/// 📥️ Protects the actual completed Process3d root across final validation and publication.
pub(super) fn guarded(snapshot:Process3dSnapshot)->semio_framework_dsl_record::__rt::DecodedFieldOwner<Process3dSnapshot>{semio_framework_dsl_record::__rt::DecodedFieldOwner::new(snapshot,retire)}

/// 🛡️ Keeps an actual typed field under its declared retirement until construction succeeds.
pub(super) fn field<T:semio_framework_dsl_record::DslField>(value:T)->semio_framework_dsl_record::__rt::DecodedFieldOwner<T>{semio_framework_dsl_record::__rt::DecodedFieldOwner::new(value,T::retire_decoded)}

/// 🌿️ Retains an optional declared typed field with the same present-field retirement authority.
pub(super) fn optional<T:semio_framework_dsl_record::DslField>(value:Option<T>)->semio_framework_dsl_record::__rt::DecodedFieldOwner<Option<T>>{semio_framework_dsl_record::__rt::DecodedFieldOwner::new(value,|value|{if let Some(value)=value{T::retire_decoded(value);}})}
