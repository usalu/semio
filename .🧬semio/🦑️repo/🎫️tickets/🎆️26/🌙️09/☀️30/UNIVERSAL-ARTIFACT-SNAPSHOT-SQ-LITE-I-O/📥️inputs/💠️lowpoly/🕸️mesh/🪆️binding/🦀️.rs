//! 🎨️ Canonical mesh channel enums own their explicit native spec and ordinal field factories.
use super::{MeshAttributeDomain,MeshAttributeSemantic,MeshAttributeInterpolation};
use semio_framework_dsl_record::{DslField,FieldValue,Shape,NativeSchemaControl};
use pack::value::{ValueError,ValueRefusalKind,NativeEncodeControl,NativeDecodeControl};
macro_rules! channel {
 ($ty:ty,$count:expr,$($case:ident=>$ordinal:literal,$name:literal);+)=>{
  impl DslField for $ty{
   fn shape()->Shape{Shape::Enum(vec![$(($name.into(),$ordinal)),+])}
   fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_stage(|control|{control.begin_stage($count)?;let mut variants=control.allocate_vec($count)?;$(variants.push((control.copy_text($name)?,$ordinal));control.step()?;)+Ok(Shape::Enum(variants))})}
   fn to_value(&self)->FieldValue{FieldValue::Enum(match self{$(Self::$case=>$ordinal),+})}
   fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.checkpoint()?;Ok(<Self as DslField>::to_value(self))}
   fn from_value(value:&FieldValue)->Result<Self,String>{match value{$(FieldValue::Enum($ordinal)=>Ok(Self::$case)),+,_=>Err("invalid mesh channel ordinal".into())}}
   fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;match value{$(FieldValue::Enum($ordinal)=>Ok(Self::$case)),+,_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid mesh channel ordinal"))}}
  }
 };
}
channel!(MeshAttributeDomain,4,Vertex=>0,"vertex";Corner=>1,"corner";Face=>2,"face";Edge=>3,"edge");
channel!(MeshAttributeSemantic,5,Normal=>0,"normal";Uv=>1,"uv";Color=>2,"color";Material=>3,"material";Custom=>4,"custom");
channel!(MeshAttributeInterpolation,3,Linear=>0,"linear";Nearest=>1,"nearest";Constant=>2,"constant");
