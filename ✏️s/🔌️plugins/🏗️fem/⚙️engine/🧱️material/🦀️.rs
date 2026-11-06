//! 🧱️ Material entities seal authored text fields through the shared original-bound receiving owner.
extern crate protocol;
use semio_framework_value::Number;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
pub(crate)fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"material requires an admitted identity and finite positive elastic properties")}
pub(crate)fn validate(id:&str,e:f64,nu:f64,rho:f64,g:Option<f64>)->Result<(),ValueError>{
 protocol::wire::validate_scoped_identity_segments(&["material",id]).map_err(|_|invalid())?;
 if e.is_finite()&&e>0.0&&nu.is_finite()&&nu> -1.0&&nu<0.5&&rho.is_finite()&&rho>0.0&&g.is_none_or(|g|g.is_finite()&&g>0.0){Ok(())}else{Err(invalid())}
}
/// 🔢️ A borrowed binary64 property presents itself to native receiving as one original number.
#[repr(transparent)]pub struct Scalar(f64);
impl semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource for Scalar{
 fn as_source(&self)->&dyn semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource{self}
 fn tree_kind(&self)->Option<semio_framework_value_native_owned::native_decoding::owned::tree::NativeTreeKind>{Some(semio_framework_value_native_owned::native_decoding::owned::tree::NativeTreeKind::Number)}
 fn number(&self)->Result<Number,ValueError>{Ok(Number::Float(self.0))}
}
/// 🪟️ Reinterprets one stored property in place; `Scalar` is a transparent newtype over `f64`.
pub fn scalar(value:&f64)->&Scalar{unsafe{&*(value as *const f64).cast::<Scalar>()}}
macro_rules! native_material_row{
 ($row:ident,$value:ty,$count:literal,{$($field:ident:$ordinal:literal),+})=>{
  #[cfg(test)]pub(crate)const NATIVE_MATERIAL_DIMENSION:usize=$count-1;
  #[cfg(test)]pub(crate)const NATIVE_MATERIAL_COUNT:usize=$count;
  /// 🔒️ Native material text fields preserve sealed original authority; construction requires explicit receiving grants.
  #[derive(Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::RetireOwned,semio_framework_value::NativeReceive,semio_framework_value::SnapshotProjection,semio_framework_value::CanonicalJsonTree)]
  #[canonical_json(owner=semio_framework_pack_json)]
  #[value(rename_all="camelCase")]
  pub struct $row{pub id:semio_framework_value::shared_issuance::SealedShared<String>,pub name:semio_framework_value::shared_issuance::SealedShared<String>,$(pub $field:f64),+}
  impl $crate::patch::NativeMaterialRow<$count> for $row{
   fn texts(&self)->[&semio_framework_value::shared_issuance::SealedShared<String>;2]{[&self.id,&self.name]}
   fn numbers(&self)->[semio_framework_value::Number;$count]{[$(semio_framework_value::Number::Float(self.$field)),+]}
   fn validation_demand(_:[&str;2],_:[semio_framework_value::Number;$count])->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:2*std::mem::size_of::<&str>()+2*$count*std::mem::size_of::<f64>(),depth:1,..Default::default()}}
   fn validate_parts(texts:[&str;2],numbers:[semio_framework_value::Number;$count])->Result<(),semio_framework_value::ValueError>{let property=|name:&str|[$((stringify!($field),numbers[$ordinal-2].as_f64())),+].into_iter().find(|(field,_)|*field==name).map(|(_,value)|value);$crate::validate(texts[0],property("e").ok_or_else($crate::invalid)?,property("nu").ok_or_else($crate::invalid)?,property("rho").ok_or_else($crate::invalid)?,property("g"))}
   fn from_parts([id,name]:[semio_framework_value::shared_issuance::SealedShared<String>;2],numbers:[semio_framework_value::Number;$count])->Self{Self{id,name,$($field:numbers[$ordinal-2].as_f64()),+}}
  }
  impl semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource for $value{
   fn as_source(&self)->&dyn semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource{self}
   fn tree_kind(&self)->Option<semio_framework_value_native_owned::native_decoding::owned::tree::NativeTreeKind>{Some(semio_framework_value_native_owned::native_decoding::owned::tree::NativeTreeKind::Object)}
   fn record_len(&self)->Result<usize,semio_framework_value::ValueError>{Ok(2+$count)}
   fn record_key(&self,index:usize)->Result<semio_framework_value_native_owned::native_decoding::owned::NativeRecordKey<'_>,semio_framework_value::ValueError>{if index<2+$count{Ok(semio_framework_value_native_owned::native_decoding::owned::NativeRecordKey::Ordinal(index as u16))}else{Err($crate::invalid())}}
   fn record_child(&self,index:usize)->Result<&dyn semio_framework_value_native_owned::native_decoding::owned::tree::NativeValueSource,semio_framework_value::ValueError>{match index{0=>Ok(&self.id),1=>Ok(&self.name),$($ordinal=>Ok($crate::scalar(&self.$field)),)+_=>Err($crate::invalid())}}
  }
  impl semio_framework_value::shared_issuance::ColdRetire for $row{
   fn cold_exclusive(&self)->bool{self.id.original_is_exclusive()&&self.name.original_is_exclusive()}
   fn cold_release(self){drop(semio_framework_value::shared_issuance::cold_take_exclusive(self.id));drop(semio_framework_value::shared_issuance::cold_take_exclusive(self.name))}
  }
  impl $row{
   /// 🛡️ A received row is usable only after its scoped identity and elastic properties were validated by a separate funded turn.
   pub fn validate(&self)->Result<(),semio_framework_value::ValueError>{let property=|name:&str|[$((stringify!($field),self.$field)),+].into_iter().find(|(field,_)|*field==name).map(|(_,value)|value);$crate::validate(self.id.as_str(),property("e").ok_or_else($crate::invalid)?,property("nu").ok_or_else($crate::invalid)?,property("rho").ok_or_else($crate::invalid)?,property("g"))}
  }
 };
}
pub(crate)use native_material_row;
#[path="🩹️patch/🦀️.rs"]pub mod patch;
#[path="🧬️schema/◻️2d/🦀️.rs"]pub mod planar;
#[path="🧬️schema/🧊️3d/🦀️.rs"]pub mod volumetric;
#[cfg(test)]#[global_allocator]static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
