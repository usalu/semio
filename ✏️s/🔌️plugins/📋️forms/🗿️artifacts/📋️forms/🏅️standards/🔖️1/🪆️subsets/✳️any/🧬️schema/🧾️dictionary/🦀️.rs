//! 🧾️ Configured form values own their question identities and complete intrinsic state.
use semio_framework_value::{DslValue,FromValue,ToValue,ValueError,ValueRefusalKind,NativeDecodeControl,NativeEncodeControl};
#[path="🪪️native-json/🦀️.rs"]mod literal;
#[path="🌳️canonical/🦀️.rs"]mod canonical;
/// 🪪️ One literal question identity and its actual first-party value.
#[derive(Clone,Debug,semio_framework_dsl_record_derive::DslRecord,semio_framework_value::RetireOwned,semio_framework_value::RetainedClone)]
pub struct FormDictionaryEntry{pub question_id:String,pub value:DslValue}
/// 🧾️ An ordered dictionary distinguishes absent ownership, empty entries and member null.
#[derive(Clone,Debug,Default,PartialEq,semio_framework_dsl_record_derive::DslRecord,semio_framework_value::RetireOwned,semio_framework_value::RetainedClone)]
pub struct FormDictionary{pub entries:Vec<FormDictionaryEntry>}
impl FormDictionary{
 /// 🛂️ Each question owns exactly one entry; intrinsic object members retain their literal order.
 pub fn validate(&self)->Result<(),ValueError>{let mut callback=|_|true;self.validate_controlled(&mut NativeDecodeControl::new(usize::MAX,&mut callback))}
 /// 🔍️ Sort borrowed identities under explicit admission and bounded literal comparisons.
 pub fn validate_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(&self,control:&mut C)->Result<(),ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;let mut identities=control.allocate_vec(self.entries.len())?;for entry in &self.entries{control.step()?;identities.push(entry.question_id.as_str());}for root in(0..identities.len()/2).rev(){sift(&mut identities,root,self.entries.len(),control)?;}for end in(1..identities.len()).rev(){identities.swap(0,end);sift(&mut identities,0,end,control)?;}for pair in identities.windows(2){if compare(pair[0],pair[1],control)?.is_eq(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate FormDictionary question identity"));}}Ok(())})}
 /// 🎞️ Consume the actual ordered dictionary object transported by a workflow wire.
 pub fn from_intrinsic(value:DslValue)->Result<Self,ValueError>{let DslValue::Object(values)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"FormDictionary media must own an object"));};let owner=Self{entries:values.into_iter().map(|(question_id,value)|FormDictionaryEntry{question_id,value}).collect()};if let Err(error)=owner.validate(){<Self as FromValue>::retire_decoded(owner);return Err(error);}Ok(owner)}
 /// 🎞️ Transfer configured entries directly to the shared intrinsic media owner.
 pub fn into_intrinsic(self)->DslValue{DslValue::Object(self.entries.into_iter().map(|entry|(entry.question_id,entry.value)).collect())}
 /// 🫳️ Bind the borrowed intrinsic object directly to its owned entries on one ledger.
 pub fn from_intrinsic_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;let DslValue::Object(values)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"FormDictionary media must own an object"))};let mut owner=semio_framework_value::DecodedValue::new(Self{entries:control.allocate_vec(values.len())?},<Self as FromValue>::retire_decoded);for(question_id,value)in values{let question_id=control.copy_text(question_id)?;let value=<DslValue as FromValue>::from_value_controlled(value,control)?;owner.get_mut().entries.push(FormDictionaryEntry{question_id,value});control.step()?;}owner.get().validate_controlled(control)?;Ok(owner.take())})}
}
impl ToValue for FormDictionaryEntry{
 fn to_value(&self)->DslValue{literal::entry_encode(self,None).expect("literal dictionary entry")}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{literal::entry_encode(self,Some(control))}
}
impl FromValue for FormDictionaryEntry{
 fn from_value(value:DslValue)->Result<Self,ValueError>{let result=literal::entry_decode(&value,None);<DslValue as FromValue>::retire_decoded(value);result}
 fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{literal::entry_decode(value,Some(control))}
 fn retire_decoded(self){<DslValue as FromValue>::retire_decoded(self.value)}
}
impl ToValue for FormDictionary{
 fn to_value(&self)->DslValue{DslValue::Object(vec![("entries".into(),self.entries.to_value())])}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{control.scoped_stage(|control|{control.begin_stage(1)?;let mut values=semio_framework_value::DecodedValue::new(control.allocate_vec(1)?,|values:Vec<(String,DslValue)>|{for(_,value)in values{<DslValue as FromValue>::retire_decoded(value)}});let key=control.copy_text("entries")?;let entries=self.entries.to_value_controlled(control)?;values.get_mut().push((key,entries));control.step()?;Ok(DslValue::Object(values.take()))})}
}
impl FromValue for FormDictionary{
 fn from_value(value:DslValue)->Result<Self,ValueError>{let mut value=semio_framework_value::DecodedValue::new(value,<DslValue as FromValue>::retire_decoded);literal::field(value.get(),"entries",&["entries"])?;let DslValue::Object(fields)=value.get_mut()else{unreachable!()};let entries=std::mem::replace(&mut fields[0].1,DslValue::Null);let owner=semio_framework_value::DecodedValue::new(Self{entries:<Vec<FormDictionaryEntry> as FromValue>::from_value(entries)?},<Self as FromValue>::retire_decoded);owner.get().validate()?;Ok(owner.take())}
 fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let entries=literal::field(value,"entries",&["entries"])?;let owner=semio_framework_value::DecodedValue::new(Self{entries:<Vec<FormDictionaryEntry> as FromValue>::from_value_controlled(entries,control)?},<Self as FromValue>::retire_decoded);owner.get().validate_controlled(control)?;Ok(owner.take())}
 fn retire_decoded(self){for entry in self.entries{<FormDictionaryEntry as FromValue>::retire_decoded(entry)}}
}
fn compare<C:semio_framework_dsl_record::NativeSchemaControl>(left:&str,right:&str,control:&mut C)->Result<std::cmp::Ordering,ValueError>{for offset in(0..left.len().min(right.len())).step_by(65536){control.step()?;let end=(offset+65536).min(left.len().min(right.len()));let order=left.as_bytes()[offset..end].cmp(&right.as_bytes()[offset..end]);if !order.is_eq(){return Ok(order);}}control.step()?;Ok(left.len().cmp(&right.len()))}
fn sift<C:semio_framework_dsl_record::NativeSchemaControl>(values:&mut[&str],mut root:usize,end:usize,control:&mut C)->Result<(),ValueError>{loop{let left=root.checked_mul(2).and_then(|value|value.checked_add(1)).unwrap_or(end);if left>=end{return Ok(())}let mut child=left;if left+1<end&&compare(values[left],values[left+1],control)?.is_lt(){child=left+1;}if !compare(values[root],values[child],control)?.is_lt(){return Ok(())}values.swap(root,child);root=child;}}
impl PartialEq for FormDictionaryEntry{fn eq(&self,other:&Self)->bool{if self.question_id!=other.question_id{return false}let mut pending=vec![(&self.value,&other.value)];while let Some((left,right))=pending.pop(){match(left,right){(DslValue::Null,DslValue::Null)=>{},(DslValue::Bool(a),DslValue::Bool(b))if a==b=>{},(DslValue::Number(semio_framework_value::Number::UInt(a)),DslValue::Number(semio_framework_value::Number::UInt(b)))if a==b=>{},(DslValue::Number(semio_framework_value::Number::Int(a)),DslValue::Number(semio_framework_value::Number::Int(b)))if a==b=>{},(DslValue::Number(semio_framework_value::Number::Float(a)),DslValue::Number(semio_framework_value::Number::Float(b)))if a.to_bits()==b.to_bits()=>{},(DslValue::String(a),DslValue::String(b))if a==b=>{},(DslValue::Bytes(a),DslValue::Bytes(b))if a==b=>{},(DslValue::Array(a),DslValue::Array(b))if a.len()==b.len()=>pending.extend(a.iter().zip(b)),(DslValue::Object(a),DslValue::Object(b))if a.len()==b.len()=>{for((an,av),(bn,bv))in a.iter().zip(b){if an!=bn{return false}pending.push((av,bv));}},_=>return false}}true}}
#[cfg(test)]
impl serde::Serialize for FormDictionary{fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{serde::Serialize::serialize(&serde_json::Value::from(self.to_value()),serializer)}}
#[cfg(test)]
impl<'de>serde::Deserialize<'de>for FormDictionary{fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{let value=<serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;Self::from_value(DslValue::from(&value)).map_err(serde::de::Error::custom)}}
