//! 🧱️ Material entities seal authored text fields through the shared original-bound receiving owner.
extern crate protocol;
use semio_framework_value::Number;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
pub(crate)fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"material requires an admitted identity and finite positive elastic properties")}
fn validate(id:&str,e:f64,nu:f64,rho:f64,g:Option<f64>)->Result<(),ValueError>{
 protocol::wire::validate_scoped_identity_segments(&["material",id]).map_err(|_|invalid())?;
 if e.is_finite()&&e>0.0&&nu.is_finite()&&nu> -1.0&&nu<0.5&&rho.is_finite()&&rho>0.0&&g.is_none_or(|g|g.is_finite()&&g>0.0){Ok(())}else{Err(invalid())}
}
pub(crate)fn validate_parts<const N:usize>(id:&str,fields:[semio_framework_value::native_decoding::owned::NativeRecordField;N],numbers:[Number;N])->Result<(),ValueError>{let property=|name|fields.iter().position(|field|field.name==name).map(|index|numbers[index].as_f64());validate(id,property("e").ok_or_else(invalid)?,property("nu").ok_or_else(invalid)?,property("rho").ok_or_else(invalid)?,property("g"))}
macro_rules! native_material_row{
 ($row:ident,$value:ty,$count:literal,{$($field:ident:$ordinal:literal),+})=>{
  #[cfg(test)]pub(crate)const NATIVE_MATERIAL_DIMENSION:usize=$count-1;
  #[cfg(test)]pub(crate)const NATIVE_MATERIAL_COUNT:usize=$count;
  /// 🔒️ Native material text fields preserve sealed original authority; construction requires explicit receiving grants.
  #[derive(Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::RetireOwned,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
  #[canonical_json(owner=semio_framework_pack_json)]
  #[value(rename_all="camelCase")]
  pub struct $row{pub id:semio_framework_value::retirement::shared::sealed::SealedShared<String>,pub name:semio_framework_value::retirement::shared::sealed::SealedShared<String>,$(pub $field:f64),+}
  impl semio_framework_value::native_decoding::owned::NativeFlatRecord<2,$count> for $row{
   const TEXT_FIELDS:[semio_framework_value::native_decoding::owned::NativeRecordField;2]=[semio_framework_value::native_decoding::owned::NativeRecordField{name:"id",ordinal:0},semio_framework_value::native_decoding::owned::NativeRecordField{name:"name",ordinal:1}];
   const NUMBER_FIELDS:[semio_framework_value::native_decoding::owned::NativeRecordField;$count]=[$(semio_framework_value::native_decoding::owned::NativeRecordField{name:stringify!($field),ordinal:$ordinal}),+];
   fn validation_demand(_:[&str;2],_:[semio_framework_value::Number;$count])->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:2*std::mem::size_of::<&str>()+2*$count*std::mem::size_of::<f64>(),depth:1,..Default::default()}}
   fn validate_parts(texts:[&str;2],numbers:[semio_framework_value::Number;$count])->Result<(),semio_framework_value::ValueError>{$crate::validate_parts(texts[0],Self::NUMBER_FIELDS,numbers)}
   fn from_parts([id,name]:[semio_framework_value::retirement::shared::sealed::SealedShared<String>;2],numbers:[semio_framework_value::Number;$count])->Self{Self{id,name,$($field:numbers[$ordinal-2].as_f64()),+}}
  }
  semio_framework_value::native_flat_record_receive!($row,2,$count);
  impl $crate::patch::NativeMaterialRow<$count> for $row{
   fn texts(&self)->[&semio_framework_value::retirement::shared::sealed::SealedShared<String>;2]{[&self.id,&self.name]}
   fn numbers(&self)->[semio_framework_value::Number;$count]{[$(semio_framework_value::Number::Float(self.$field)),+]}
  }
  impl semio_framework_value::native_decoding::owned::NativeRecordSource for $value{
   fn field_count(&self)->Result<usize,semio_framework_value::ValueError>{Ok(2+$count)}
   fn field_key(&self,index:usize)->Result<semio_framework_value::native_decoding::owned::NativeRecordKey<'_>,semio_framework_value::ValueError>{if index<2+$count{Ok(semio_framework_value::native_decoding::owned::NativeRecordKey::Ordinal(index as u16))}else{Err($crate::invalid())}}
   fn text_field(&self,index:usize)->Result<&String,semio_framework_value::ValueError>{match index{0=>Ok(&self.id),1=>Ok(&self.name),_=>Err($crate::invalid())}}
   fn number_field(&self,index:usize)->Result<semio_framework_value::Number,semio_framework_value::ValueError>{match index{$($ordinal=>Ok(semio_framework_value::Number::Float(self.$field)),)+_=>Err($crate::invalid())}}
  }
 };
}
pub(crate)use native_material_row;
#[path="🩹️patch/🦀️.rs"]pub mod patch;
#[path="🧬️schema/◻️2d/🦀️.rs"]pub mod planar;
#[path="🧬️schema/🧊️3d/🦀️.rs"]pub mod volumetric;
#[cfg(test)]#[global_allocator]static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
