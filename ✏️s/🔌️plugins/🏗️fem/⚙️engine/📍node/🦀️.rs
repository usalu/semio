//! 📍️ Actual dimensional node owners seal identity and retain original primitive coordinates.
extern crate protocol;
use semio_framework_value::Number;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
pub fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"node requires a scoped identity and finite coordinates")}
pub fn validate<const N:usize>(id:&str,coordinates:[Number;N])->Result<(),ValueError>{protocol::wire::validate_scoped_identity_segments(&["node",id]).map_err(|_|invalid())?;if coordinates.into_iter().all(|coordinate|coordinate.as_f64().is_finite()){Ok(())}else{Err(invalid())}}
/// 🔎️ One original scalar query of a node source.
pub type Query=semio_framework_value_native_owned::native_decoding::owned::tree::NativeScalarQuery;
/// 📬️ The borrowed reply of one scalar query.
pub type Reply<'reply>=semio_framework_value_native_owned::native_decoding::OriginalEffectReply<'reply,semio_framework_value_native_owned::native_decoding::owned::tree::NativeSourceScalar,ValueError>;
/// 🏁️ The recorded completion of one scalar query.
pub type Done<'reply>=Result<semio_framework_value_native_owned::native_decoding::OriginalEffectReplyComplete<'reply,semio_framework_value_native_owned::native_decoding::owned::tree::NativeSourceScalar,ValueError>,ValueError>;
/// 🧮️ Answers a coordinate query: a number leaf has only a kind, a null test and its number.
pub fn number_answer(query:Query,value:f64)->Result<semio_framework_value_native_owned::native_decoding::owned::tree::NativeSourceScalar,ValueError>{use semio_framework_value_native_owned::native_decoding::owned::tree::{NativeScalarQuery,NativeSourceScalar,NativeTreeKind};match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Number))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::Number=>Ok(NativeSourceScalar::Number(Number::Float(value))),_=>Err(invalid())}}
/// 🧮️ Answers a node query: an object of identity plus `count` coordinates.
pub fn node_answer(query:Query,count:usize)->Result<semio_framework_value_native_owned::native_decoding::owned::tree::NativeSourceScalar,ValueError>{use semio_framework_value_native_owned::native_decoding::owned::tree::{NativeScalarQuery,NativeSourceScalar,NativeTreeKind};match query{NativeScalarQuery::Kind=>Ok(NativeSourceScalar::Kind(Some(NativeTreeKind::Object))),NativeScalarQuery::Null=>Ok(NativeSourceScalar::Null(false)),NativeScalarQuery::RecordLength=>Ok(NativeSourceScalar::Length(1+count)),_=>Err(invalid())}}
/// 📬️ Records one answer inside the original effect after its single unit of work.
pub fn record_answer<'reply>(answer:Result<semio_framework_value_native_owned::native_decoding::owned::tree::NativeSourceScalar,ValueError>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Done<'reply>{if effect.maximum_work_units()==0{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar query lacks its original work grant"))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"scalar query exceeded its original work authority"))))}Ok(reply.complete(answer))}
/// 🔁️ A recovered query stays pending; the answer was never recorded twice.
pub fn recover_answer<'reply>(reply:Reply<'reply>)->Done<'reply>{Ok(reply.pending())}
/// 🔢️ A borrowed binary64 coordinate presents itself to native receiving as one original number.
#[repr(transparent)]pub struct Scalar(f64);
impl semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource for Scalar{
 fn query_scalar<'reply>(&self,query:Query,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Done<'reply>{record_answer(number_answer(query,self.0),effect,reply)}
 fn recover_scalar<'reply>(&self,_query:Query,_effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Done<'reply>{recover_answer(reply)}
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
   fn query_schema_key<'reply,'source>(&'source self,query:semio_framework_value_native_owned::native_decoding::owned::NativeSchemaKeySource,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:semio_framework_value_native_owned::native_decoding::owned::NativeSourceKeyReply<'reply,'source>)->Result<semio_framework_value_native_owned::native_decoding::owned::NativeSourceKeyReplyComplete<'reply>,semio_framework_value::ValueError>{semio_framework_value_native_owned::native_decoding::owned::answer_schema_key(self,query,effect,reply)}
   fn query_scalar<'reply>(&self,query:$crate::Query,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:$crate::Reply<'reply>)->$crate::Done<'reply>{$crate::record_answer($crate::node_answer(query,$count),effect,reply)}
   fn recover_scalar<'reply>(&self,_query:$crate::Query,_effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,reply:$crate::Reply<'reply>)->$crate::Done<'reply>{$crate::recover_answer(reply)}
   fn as_source(&self)->&dyn semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource{self}
   fn tree_kind(&self)->Option<semio_framework_value_native_owned::native_decoding::owned::tree::NativeTreeKind>{Some(semio_framework_value_native_owned::native_decoding::owned::tree::NativeTreeKind::Object)}
   fn record_len(&self)->Result<usize,semio_framework_value::ValueError>{Ok(1+$count)}
   fn record_key(&self,index:usize)->Result<semio_framework_value_native_owned::native_decoding::owned::NativeRecordKey<'_>,semio_framework_value::ValueError>{if index<=$count{Ok(semio_framework_value_native_owned::native_decoding::owned::NativeRecordKey::Ordinal(index as u16))}else{Err($crate::invalid())}}
   fn record_child(&self,index:usize)->Result<&dyn semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource,semio_framework_value::ValueError>{match index{0=>Ok(&self.id),$($ordinal=>Ok($crate::scalar(&self.$field)),)+_=>Err($crate::invalid())}}
  }
  impl semio_framework_value::shared_issuance::ColdRetire for FemNode{
   fn cold_exclusive(&self)->bool{self.id.original_is_exclusive()}
   fn cold_release(self){drop(semio_framework_value::shared_issuance::cold_take_exclusive(self.id))}
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
#[cfg(test)]use semio_framework_value_testing_allocator as _;
