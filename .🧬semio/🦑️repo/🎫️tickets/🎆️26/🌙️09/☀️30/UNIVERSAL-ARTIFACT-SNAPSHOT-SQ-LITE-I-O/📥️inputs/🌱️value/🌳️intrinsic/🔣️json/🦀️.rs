//! 🔣️ Lossless intrinsic JSON spells exact magnitudes, float words and ordered member occurrences.
use crate::{DslValue as Value,Number,FromValue,ValueError,ValueRefusalKind};
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn fields(value:&Value,names:&[&str])->Result<(),ValueError>{let Value::Object(entries)=value else{return Err(invalid("intrinsic JSON requires an object"))};if entries.len()!=names.len()||names.iter().any(|name|entries.iter().filter(|(key,_)|key==name).count()!=1){return Err(invalid("intrinsic JSON field set differs"))}Ok(())}
fn text(value:&Value)->Result<&str,ValueError>{value.as_str().ok_or_else(||invalid("intrinsic JSON requires text"))}
fn list(value:&Value)->Result<&[Value],ValueError>{value.as_array().ok_or_else(||invalid("intrinsic JSON requires an ordered collection"))}
struct Values(Vec<Value>);
impl Drop for Values{fn drop(&mut self){while let Some(value)=self.0.pop(){value.retire_decoded();}}}
enum Decode<'a>{Visit(&'a Value),Array(usize),Object(Vec<String>)}
fn take(values:&mut Values,count:usize)->Vec<Value>{let start=values.0.len()-count;values.0.drain(start..).collect()}

/// 📥️ Decode canonical tagged JSON through an iterative ownership stack.
pub fn decode(source:&Value)->Result<Value,ValueError>{
 let mut pending=vec![Decode::Visit(source)];let mut values=Values(Vec::new());
 while let Some(frame)=pending.pop(){match frame{
  Decode::Array(count)=>{let items=take(&mut values,count);values.0.push(Value::Array(items));},
  Decode::Object(names)=>{let items=take(&mut values,names.len());values.0.push(Value::Object(names.into_iter().zip(items).collect()));},
  Decode::Visit(value)=>{let kind=text(&value["kind"])?;fields(value,match kind{"null"=>&["kind"],"array"=>&["kind","items"],"object"=>&["kind","members"],_=>&["kind","value"]})?;match kind{
   "null"=>values.0.push(Value::Null),
   "boolean"=>{let Value::Bool(value)=value["value"]else{return Err(invalid("intrinsic JSON boolean differs"))};values.0.push(Value::Bool(value));},
   "unsigned"=>{let source=text(&value["value"])?;if source.is_empty()||source.len()>20||!source.bytes().all(|byte|byte.is_ascii_digit())||(source.len()>1&&source.starts_with('0')){return Err(invalid("intrinsic JSON unsigned decimal differs"))}values.0.push(Value::Number(Number::UInt(source.parse().map_err(|_|invalid("intrinsic JSON unsigned range"))?)));},
   "signed"=>{let source=text(&value["value"])?;let magnitude=source.strip_prefix('-').unwrap_or(source);if magnitude.is_empty()||source.len()>20||!magnitude.bytes().all(|byte|byte.is_ascii_digit())||(magnitude.len()>1&&magnitude.starts_with('0'))||source=="-0"{return Err(invalid("intrinsic JSON signed decimal differs"))}values.0.push(Value::Number(Number::Int(source.parse().map_err(|_|invalid("intrinsic JSON signed range"))?)));},
   "float"=>{let source=text(&value["value"])?;if source.len()!=16||!source.bytes().all(|byte|byte.is_ascii_digit()||(b'a'..=b'f').contains(&byte)){return Err(invalid("intrinsic JSON binary64 word differs"))}let bits=u64::from_str_radix(source,16).map_err(|_|invalid("intrinsic JSON binary64 width"))?;values.0.push(Value::Number(Number::Float(f64::from_bits(bits))));},
   "text"=>values.0.push(Value::String(text(&value["value"])?.into())),
   "bytes"=>{let bytes=list(&value["value"])?.iter().map(|value|value.as_u64().and_then(|value|u8::try_from(value).ok()).ok_or_else(||invalid("intrinsic JSON octet range"))).collect::<Result<Vec<_>,_>>()?;values.0.push(Value::Bytes(bytes));},
   "array"=>{let items=list(&value["items"])?;pending.push(Decode::Array(items.len()));for value in items.iter().rev(){pending.push(Decode::Visit(value));}},
   "object"=>{let items=list(&value["members"])?;let mut names=Vec::with_capacity(items.len());for value in items{fields(value,&["name","value"])?;names.push(text(&value["name"])?.into());}pending.push(Decode::Object(names));for value in items.iter().rev(){pending.push(Decode::Visit(&value["value"]));}},
   _=>return Err(invalid("intrinsic JSON kind differs")),
  }},
 }}
 values.0.pop().ok_or_else(||invalid("intrinsic JSON root absent"))
}
enum Encode<'a>{Visit(&'a Value),Array(usize),Object(Vec<String>)}
fn tagged(kind:&str,field:Option<(&str,Value)>)->Value{let mut entries=vec![("kind".into(),Value::String(kind.into()))];if let Some((name,value))=field{entries.push((name.into(),value));}Value::Object(entries)}
/// 📤️ Encode every intrinsic kind without converting float words through JSON numbers.
pub fn encode(source:&Value)->Value{
 let mut pending=vec![Encode::Visit(source)];let mut values=Values(Vec::new());
 while let Some(frame)=pending.pop(){match frame{
  Encode::Array(count)=>{let items=take(&mut values,count);values.0.push(tagged("array",Some(("items",Value::Array(items)))));},
  Encode::Object(names)=>{let items=take(&mut values,names.len());let members=names.into_iter().zip(items).map(|(name,value)|Value::Object(vec![("name".into(),Value::String(name)),("value".into(),value)])).collect();values.0.push(tagged("object",Some(("members",Value::Array(members)))));},
  Encode::Visit(value)=>{let output=match value{
   Value::Null=>tagged("null",None),Value::Bool(value)=>tagged("boolean",Some(("value",Value::Bool(*value)))),
   Value::Number(Number::UInt(value))=>tagged("unsigned",Some(("value",Value::String(value.to_string())))),
   Value::Number(Number::Int(value))=>tagged("signed",Some(("value",Value::String(value.to_string())))),
   Value::Number(Number::Float(value))=>tagged("float",Some(("value",Value::String(format!("{:016x}",value.to_bits()))))),
   Value::String(value)=>tagged("text",Some(("value",Value::String(value.clone())))),
   Value::Bytes(value)=>tagged("bytes",Some(("value",Value::Array(value.iter().map(|byte|Value::Number(Number::UInt(u64::from(*byte)))).collect())))),
   Value::Array(items)=>{pending.push(Encode::Array(items.len()));for value in items.iter().rev(){pending.push(Encode::Visit(value));}continue;},
   Value::Object(items)=>{pending.push(Encode::Object(items.iter().map(|(name,_)|name.clone()).collect()));for(_,value)in items.iter().rev(){pending.push(Encode::Visit(value));}continue;},
  };values.0.push(output);},
 }}
 values.0.pop().expect("one intrinsic source produces one tagged root")
}
