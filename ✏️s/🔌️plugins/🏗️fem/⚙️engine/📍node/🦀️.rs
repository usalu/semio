//! 📍️ Actual dimensional node owners seal identity and retain original primitive coordinates.
extern crate protocol;
use semio_framework_value::Number;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"node requires a scoped identity and finite coordinates")}
fn validate<const N:usize>(id:&str,coordinates:[Number;N])->Result<(),ValueError>{protocol::wire::validate_scoped_identity_segments(&["node",id]).map_err(|_|invalid())?;if coordinates.into_iter().all(|coordinate|coordinate.as_f64().is_finite()){Ok(())}else{Err(invalid())}}
macro_rules! native_node{
 ($count:literal,{$($field:ident:$ordinal:literal),+})=>{
  /// 📍️ Native nodes share sealed identity while coordinates remain fixed copied scalars.
  #[derive(Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
  #[canonical_json(owner=semio_framework_pack_json)]
  #[value(rename_all="camelCase")]
  pub struct FemNode{pub id:semio_framework_value::retirement::shared::sealed::SealedShared<String>,$(pub $field:f64),+}
  /// ❄️ Canonical authoring and reference algebra keep their explicitly owned node values.
  #[derive(Clone,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned,semio_framework_dsl_record_derive::DslRecord,semio_framework_value::CanonicalJsonTree)]
  #[canonical_json(owner=semio_framework_pack_json)]
  #[value(rename_all="camelCase")]
  pub struct FemNodeValue{pub id:String,$(#[dsl(unit="m")]pub $field:f64),+}
  impl semio_framework_value::native_decoding::owned::NativeFlatRecord<1,$count>for FemNode{
   const TEXT_FIELDS:[semio_framework_value::native_decoding::owned::NativeRecordField;1]=[semio_framework_value::native_decoding::owned::NativeRecordField{name:"id",ordinal:0}];
   const NUMBER_FIELDS:[semio_framework_value::native_decoding::owned::NativeRecordField;$count]=[$(semio_framework_value::native_decoding::owned::NativeRecordField{name:stringify!($field),ordinal:$ordinal}),+];
   fn validation_demand(_:[&str;1],_:[semio_framework_value::Number;$count])->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<&str>()+2*$count*std::mem::size_of::<f64>(),depth:1,..Default::default()}}
   fn validate_parts([id]:[&str;1],coordinates:[semio_framework_value::Number;$count])->Result<(),semio_framework_value::ValueError>{$crate::validate(id,coordinates)}
   fn from_parts([id]:[semio_framework_value::retirement::shared::sealed::SealedShared<String>;1],coordinates:[semio_framework_value::Number;$count])->Self{Self{id,$($field:coordinates[$ordinal-1].as_f64()),+}}
  }
  semio_framework_value::native_flat_record_receive!(FemNode,1,$count);
  impl semio_framework_value::native_decoding::owned::NativeRecordSource for FemNodeValue{
   fn field_count(&self)->Result<usize,semio_framework_value::ValueError>{Ok(1+$count)}
   fn field_key(&self,index:usize)->Result<semio_framework_value::native_decoding::owned::NativeRecordKey<'_>,semio_framework_value::ValueError>{if index<= $count{Ok(semio_framework_value::native_decoding::owned::NativeRecordKey::Ordinal(index as u16))}else{Err($crate::invalid())}}
   fn text_field(&self,index:usize)->Result<&String,semio_framework_value::ValueError>{if index==0{Ok(&self.id)}else{Err($crate::invalid())}}
   fn number_field(&self,index:usize)->Result<semio_framework_value::Number,semio_framework_value::ValueError>{match index{$($ordinal=>Ok(semio_framework_value::Number::Float(self.$field)),)+_=>Err($crate::invalid())}}
  }
  #[cfg(test)]const DIMENSION:usize=$count;
  #[cfg(test)]#[path="../../🧪️tests/🦀️.rs"]mod tests;
 };
}
#[path="🧬️schema/◻️2d/🦀️.rs"]pub mod planar;
#[path="🧬️schema/🧊️3d/🦀️.rs"]pub mod volumetric;
#[cfg(test)]#[global_allocator]static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
