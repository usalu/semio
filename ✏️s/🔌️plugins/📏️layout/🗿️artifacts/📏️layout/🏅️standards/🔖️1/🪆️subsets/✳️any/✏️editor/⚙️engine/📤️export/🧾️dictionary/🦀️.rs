//! 🧾️ Borrowed, resumable dictionary word JSON publication.
use super::{JsonStringWriteCursor,TypedJsonCursor,LayoutSnapshot,MAX_LAYOUT_EXPORT_JSON_NODES,MAX_LAYOUT_EXPORT_STRING_BYTES,MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES};
use semio_framework_value::{DslValue,Number};

#[derive(Clone,Debug)]
enum Task {
 Static(&'static[u8]),
 Scalar(u64),
 Entries(usize),
 Value{entry:usize,path:Vec<usize>},
 Array{entry:usize,path:Vec<usize>,index:usize},
 Object{entry:usize,path:Vec<usize>,index:usize},
 Bytes{entry:usize,path:Vec<usize>,index:usize},
 Text{entry:usize,path:Vec<usize>,member:Option<usize>,question:bool,cursor:JsonStringWriteCursor},
}

#[derive(Clone,Debug)]
pub(super) struct DictionaryJsonCursor { tasks:Vec<Task>,bytes:usize,nodes:usize }

impl DictionaryJsonCursor {
 pub(super) fn new()->Self{Self{tasks:vec![Task::Static(b"}"),Task::Entries(0),Task::Static(b"{\"entries\":[")],bytes:0,nodes:0}}
 pub(super) fn owned_bytes(&self)->usize{self.tasks.capacity().saturating_mul(size_of::<Task>()).saturating_add(self.tasks.iter().map(|task|match task{Task::Value{path,..}|Task::Array{path,..}|Task::Object{path,..}|Task::Bytes{path,..}|Task::Text{path,..}=>path.capacity().saturating_mul(size_of::<usize>()),_=>0}).sum::<usize>())}
 fn value<'a>(snapshot:&'a LayoutSnapshot,entry:usize,path:&[usize])->Result<&'a DslValue,String>{let mut value=&snapshot.data_fields.as_ref().ok_or("layout-export-dictionary")?.entries.get(entry).ok_or("layout-export-dictionary-entry")?.value;for index in path{value=match value{DslValue::Array(items)=>items.get(*index),DslValue::Object(members)=>members.get(*index).map(|(_,value)|value),_=>None}.ok_or("layout-export-dictionary-path")?;}Ok(value)}
 fn push(&mut self,tasks:impl IntoIterator<Item=Task>){self.tasks.extend(tasks);}
 fn node(&mut self)->Result<(),String>{self.nodes=self.nodes.checked_add(1).ok_or("layout-export-json-node-limit")?;if self.nodes>MAX_LAYOUT_EXPORT_JSON_NODES{return Err("layout-export-json-node-limit".into());}Ok(())}
 pub(super) fn advance(&mut self,snapshot:&LayoutSnapshot)->Result<(Vec<u8>,bool),String>{let Some(task)=self.tasks.pop()else{return Ok((Vec::new(),true));};let mut output=Vec::new();match task{
  Task::Static(bytes)=>output.extend_from_slice(bytes),Task::Scalar(value)=>output.extend_from_slice(value.to_string().as_bytes()),
  Task::Entries(index)=>{let entries=&snapshot.data_fields.as_ref().ok_or("layout-export-dictionary")?.entries;if index==entries.len(){output.push(b']');}else{if index>0{output.push(b',');}self.push([Task::Entries(index+1),Task::Static(b"}"),Task::Value{entry:index,path:Vec::new()},Task::Static(b",\"value\":"),Task::Text{entry:index,path:Vec::new(),member:None,question:true,cursor:JsonStringWriteCursor::default()},Task::Static(b"{\"questionId\":")]);}},
  Task::Value{entry,path}=>{self.node()?;if path.len()>64{return Err("layout-export-json-depth-limit".into());}match Self::value(snapshot,entry,&path)?{
   DslValue::Null=>output.extend_from_slice(b"{\"kind\":\"null\"}"),
   DslValue::Bool(value)=>output.extend_from_slice(if *value{b"{\"kind\":\"boolean\",\"value\":true}"}else{b"{\"kind\":\"boolean\",\"value\":false}"}),
   DslValue::Number(value)=>{let(kind,word)=match value{Number::UInt(value)=>(b"{\"kind\":\"unsigned\",\"high\":".as_slice(),*value),Number::Int(value)=>(b"{\"kind\":\"signed\",\"high\":".as_slice(),*value as u64),Number::Float(value)=>(b"{\"kind\":\"float\",\"high\":".as_slice(),value.to_bits())};self.push([Task::Static(b"}"),Task::Scalar(word&0xffffffff),Task::Static(b",\"low\":"),Task::Scalar(word>>32),Task::Static(kind)]);},
   DslValue::String(_)=>self.push([Task::Static(b"}"),Task::Text{entry,path,member:None,question:false,cursor:JsonStringWriteCursor::default()},Task::Static(b"{\"kind\":\"text\",\"value\":")]),
   DslValue::Bytes(_)=>self.push([Task::Static(b"}"),Task::Bytes{entry,path,index:0},Task::Static(b"{\"kind\":\"bytes\",\"value\":[")]),
   DslValue::Array(_)=>self.push([Task::Static(b"}"),Task::Array{entry,path,index:0},Task::Static(b"{\"kind\":\"array\",\"items\":[")]),
   DslValue::Object(_)=>self.push([Task::Static(b"}"),Task::Object{entry,path,index:0},Task::Static(b"{\"kind\":\"object\",\"members\":[")]),
  }},
  Task::Array{entry,path,index}=>{let DslValue::Array(items)=Self::value(snapshot,entry,&path)?else{return Err("layout-export-dictionary-array".into());};if index==items.len(){output.push(b']');}else{if index>0{output.push(b',');}let mut child=path.clone();child.push(index);self.push([Task::Array{entry,path,index:index+1},Task::Value{entry,path:child}]);}},
  Task::Object{entry,path,index}=>{let DslValue::Object(members)=Self::value(snapshot,entry,&path)?else{return Err("layout-export-dictionary-object".into());};if index==members.len(){output.push(b']');}else{if index>0{output.push(b',');}let mut child=path.clone();child.push(index);self.push([Task::Object{entry,path:path.clone(),index:index+1},Task::Static(b"}"),Task::Value{entry,path:child},Task::Static(b",\"value\":"),Task::Text{entry,path,member:Some(index),question:false,cursor:JsonStringWriteCursor::default()},Task::Static(b"{\"name\":")]);}},
  Task::Bytes{entry,path,index}=>{let DslValue::Bytes(bytes)=Self::value(snapshot,entry,&path)?else{return Err("layout-export-dictionary-bytes".into());};if index==bytes.len(){output.push(b']');}else{self.node()?;if index>0{output.push(b',');}output.extend_from_slice(bytes[index].to_string().as_bytes());self.tasks.push(Task::Bytes{entry,path,index:index+1});}},
  Task::Text{entry,path,member,question,mut cursor}=>{let value=if question{snapshot.data_fields.as_ref().ok_or("layout-export-dictionary")?.entries.get(entry).ok_or("layout-export-dictionary-entry")?.question_id.as_str()}else{match(Self::value(snapshot,entry,&path)?,member){(DslValue::String(value),None)=>value.as_str(),(DslValue::Object(members),Some(index))=>members.get(index).map(|(name,_)|name.as_str()).ok_or("layout-export-dictionary-member")?,_=>return Err("layout-export-dictionary-text".into())}};if value.len()>MAX_LAYOUT_EXPORT_STRING_BYTES{return Err("layout-export-json-string-limit".into());}let(bytes,done)=TypedJsonCursor::escaped_string_step(value,&mut cursor);output=bytes;if !done{self.tasks.push(Task::Text{entry,path,member,question,cursor});}}
 }self.bytes=self.bytes.checked_add(output.len()).ok_or("layout-export-json-byte-limit")?;if self.bytes>MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES{return Err("layout-export-json-byte-limit".into());}Ok((output,self.tasks.is_empty()))}
}
