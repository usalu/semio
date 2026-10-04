//! 📏️ Borrowed native terrain leaves enforce semantic bytes independently of paid backing.
use semio_framework_dsl_record::{FieldValue,RecordValue};
use semio_framework_value::{DslValue,NativeDecodeControl,ValueError,ValueRefusalKind};
struct Bytes{used:usize,maximum:usize}
impl Bytes{
 fn add(&mut self,bytes:usize)->Result<(),ValueError>{self.used=self.used.checked_add(bytes).filter(|total|*total<=self.maximum).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"terrain native literal values exceed semantic byte limit"))?;Ok(())}
 fn intrinsic(&mut self,value:&DslValue,depth:usize,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
  native.checkpoint()?;if depth>=64{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"terrain native literal census exceeds intrinsic depth limit"));}
  match value{
   DslValue::Null=>Ok(()),DslValue::Bool(_)=>self.add(1),DslValue::Number(_)=>self.add(8),DslValue::String(text)=>self.add(text.len()),DslValue::Bytes(bytes)=>self.add(bytes.len()),
   DslValue::Array(values)=>native.scoped_stage(|native|{native.begin_stage(values.len())?;for value in values{self.intrinsic(value,depth+1,native)?;native.step()?;}Ok(())}),
   DslValue::Object(members)=>native.scoped_stage(|native|{native.begin_stage(members.len())?;for(name,value)in members{self.add(name.len())?;self.intrinsic(value,depth+1,native)?;native.step()?;}Ok(())}),
  }
 }
 fn field(&mut self,value:&FieldValue,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
  native.scoped_depth(32,|native|match value{
   FieldValue::Absent=>Ok(()),FieldValue::Float(_)=>self.add(8),FieldValue::Text(text)=>self.add(text.len()),FieldValue::Value(value)=>self.intrinsic(value,0,native),
   FieldValue::List(values)=>native.scoped_stage(|native|{native.begin_stage(values.len())?;for value in values{self.field(value,native)?;native.step()?;}Ok(())}),
   FieldValue::Record(record)=>self.record(record,native),
   _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"terrain native field is outside its authored literal domain")),
  })
 }
 fn record(&mut self,record:&RecordValue,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{native.scoped_stage(|native|{native.begin_stage(record.fields.len())?;for value in record.fields.values(){self.field(value,native)?;native.step()?;}Ok(())})}
}
pub(super) fn native(record:&RecordValue,maximum:usize,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{Bytes{used:0,maximum}.record(record,control)}
