//! 🧩️ Parent JSON borrows only genuine Drawing member references; child heads stay in their member stores.
use super::{TypedJsonCursor,JsonStringWriteCursor,JSON_OUTPUT_BYTES_PER_UNIT,MAX_LAYOUT_EXPORT_STRING_BYTES,MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES};
use semio_s_artifact_layout_layout_model::{DrawingSourceAddress,drawing_member_at,snapshot::LayoutSnapshot};
#[derive(Clone,Debug)]
enum Task{Static(&'static[u8]),Handle(u8,JsonStringWriteCursor)}
/// 🧵️ A captured source address and bounded escaping state own no Drawing payload.
#[derive(Clone,Debug)]
pub struct ChildJsonCursor{source:DrawingSourceAddress,tasks:Vec<Task>,bytes:usize}
impl ChildJsonCursor{
 pub fn new(source:DrawingSourceAddress)->Self{Self{source,tasks:vec![Task::Static(b"}}}}"),Task::Handle(4,Default::default()),Task::Static(b",\"subset\":"),Task::Handle(3,Default::default()),Task::Static(b",\"standard\":"),Task::Handle(2,Default::default()),Task::Static(b",\"dialect\":{\"artifactKind\":"),Task::Handle(1,Default::default()),Task::Static(b",\"target\":{\"artifactId\":"),Task::Handle(0,Default::default()),Task::Static(b"{\"handle\":{\"childId\":")],bytes:0}}
 pub fn owned_bytes(&self)->usize{self.tasks.capacity().saturating_mul(std::mem::size_of::<Task>())}
 pub fn advance(&mut self,snapshot:&LayoutSnapshot)->Result<(Vec<u8>,bool),String>{
  let Some(task)=self.tasks.pop()else{return Ok((Vec::new(),true))};
  let output=match task{
   Task::Static(bytes)=>bytes.to_vec(),
   Task::Handle(field,mut cursor)=>{
    let member=drawing_member_at(snapshot,self.source).map_err(|error|error.to_string())?.ok_or("layout-export-child-source")?;
    let handle=&member.handle;
    let text=match field{0=>handle.child_id.as_str(),1=>handle.target.artifact_id.as_str(),2=>handle.target.dialect.artifact_kind.as_str(),3=>handle.target.dialect.standard.as_str(),4=>handle.target.dialect.subset.as_str(),_=>return Err("layout-export-child-handle".into())};
    if text.len()>MAX_LAYOUT_EXPORT_STRING_BYTES{return Err("layout-export-json-string-limit".into())}
    let(bytes,done)=TypedJsonCursor::escaped_string_step(text,&mut cursor);
    if !done{self.tasks.push(Task::Handle(field,cursor));}
    bytes
   }
  };
  if output.len()>JSON_OUTPUT_BYTES_PER_UNIT{return Err("layout-export-json-output-credit".into())}
  self.bytes=self.bytes.checked_add(output.len()).ok_or("layout-export-json-byte-limit")?;
  if self.bytes>MAX_LAYOUT_EXPORT_PACKAGE_FRAGMENT_BYTES{return Err("layout-export-json-byte-limit".into())}
  Ok((output,self.tasks.is_empty()))
 }
}
