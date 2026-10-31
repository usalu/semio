//! 📍️ Actual dimensional node owners seal identity and retain original primitive coordinates.
extern crate protocol;
use semio_framework_value::Number;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
pub fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"node requires a scoped identity and finite coordinates")}
pub fn validate<const N:usize>(id:&str,coordinates:[Number;N])->Result<(),ValueError>{protocol::wire::validate_scoped_identity_segments(&["node",id]).map_err(|_|invalid())?;if coordinates.into_iter().all(|coordinate|coordinate.as_f64().is_finite()){Ok(())}else{Err(invalid())}}
/// 🔢️ A borrowed binary64 coordinate presents itself to native receiving as one original number.
#[repr(transparent)]pub struct Scalar(f64);
impl semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource for Scalar{
 fn as_source(&self)->&dyn semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource{self}
 fn tree_kind(&self)->Option<semio_framework_value_native_owned::native_decoding::owned::tree::NativeTreeKind>{Some(semio_framework_value_native_owned::native_decoding::owned::tree::NativeTreeKind::Number)}
 fn number(&self)->Result<Number,ValueError>{Ok(Number::Float(self.0))}
}
/// 🪟️ Reinterprets one stored coordinate in place; `Scalar` is a transparent newtype over `f64`.
pub fn scalar(value:&f64)->&Scalar{unsafe{&*(value as *const f64).cast::<Scalar>()}}
macro_rules! native_node{
 ($count:literal,{$($field:ident:$ordinal:literal),+})=>{
  /// 📍️ Native nodes share sealed identity while coordinates remain fixed copied scalars.
  #[derive(Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::RetireOwned,semio_framework_value::NativeReceive,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
  #[canonical_json(owner=semio_framework_pack_json)]
  #[value(rename_all="camelCase")]
  pub struct FemNode{pub id:semio_framework_value::shared_issuance::SealedShared<String>,$(pub $field:f64),+}
  /// ❄️ Canonical authoring and reference algebra keep their explicitly owned node values.
  #[derive(Clone,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned,semio_framework_dsl_record_derive::DslRecord,semio_framework_value::CanonicalJsonTree)]
  #[canonical_json(owner=semio_framework_pack_json)]
  #[value(rename_all="camelCase")]
  pub struct FemNodeValue{pub id:String,$(#[dsl(unit="m")]pub $field:f64),+}
  impl semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource for FemNodeValue{
   fn as_source(&self)->&dyn semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource{self}
   fn tree_kind(&self)->Option<semio_framework_value_native_owned::native_decoding::owned::tree::NativeTreeKind>{Some(semio_framework_value_native_owned::native_decoding::owned::tree::NativeTreeKind::Object)}
   fn record_len(&self)->Result<usize,semio_framework_value::ValueError>{Ok(1+$count)}
   fn record_key(&self,index:usize)->Result<semio_framework_value_native_owned::native_decoding::owned::NativeRecordKey<'_>,semio_framework_value::ValueError>{if index<=$count{Ok(semio_framework_value_native_owned::native_decoding::owned::NativeRecordKey::Ordinal(index as u16))}else{Err($crate::invalid())}}
   fn record_child(&self,index:usize)->Result<&dyn semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource,semio_framework_value::ValueError>{match index{0=>Ok(&self.id),$($ordinal=>Ok($crate::scalar(&self.$field)),)+_=>Err($crate::invalid())}}
  }
  impl FemNode{
   /// 🛡️ A received row is usable only after its scoped identity and finite coordinates were validated by a separate funded turn.
   pub fn validate(&self)->Result<(),semio_framework_value::ValueError>{$crate::validate(self.id.as_str(),[$(semio_framework_value::Number::Float(self.$field)),+])}
  }
  #[cfg(test)]const DIMENSION:usize=$count;
 };
}
#[path="🧬️schema/◻️2d/🦀️.rs"]pub mod planar;
#[path="🧬️schema/🧊️3d/🦀️.rs"]pub mod volumetric;
#[cfg(test)]#[global_allocator]static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
