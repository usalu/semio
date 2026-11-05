//! 🚶️ One static metadata edge or original value member advances per caller turn; emitted paths borrow compiled keys.
use super::original::{NormInspectorPageOriginal,NormInspectorSchemaPath};
use semio_framework::{ArgSchema,ActionArgDef};
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(crate) enum Part{Draft,Key(usize),Index{bytes:[u8;20],len:u8}}
impl Part{
 pub fn index(mut value:usize)->Self{let mut bytes=[0;20];let mut at=20;loop{at-=1;bytes[at]=b'0'+(value%10)as u8;value/=10;if value==0{break}}let len=20-at;bytes.copy_within(at..20,0);Self::Index{bytes,len:len as u8}}
 pub fn text<'a>(&'a self,original:&'a NormInspectorPageOriginal)->Option<&'a str>{match self{Self::Draft=>Some(original.draft_key.as_str()),Self::Key(index)=>original.seeds.borrow().get().get(*index)?.key.as_deref(),Self::Index{bytes,len}=>std::str::from_utf8(&bytes[..usize::from(*len)]).ok()}}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(crate) enum Seed{Static(usize),Initial,EmptyText}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(crate) enum Kind{Text,Boolean,Choice{count:usize,replace_parent:bool},Create{seed:Seed},Clear,Remove{row:usize},Append{seed:Seed,absent:bool}}
#[derive(Clone,Copy)]
pub(crate) struct Candidate{pub node:usize,pub kind:Kind,pub required:bool,pub path_len:usize,pub value_len:usize,pub present:bool,pub variant_group:Option<usize>}
#[derive(Clone,Copy)]
enum Phase{Enter,Selector,Fields,FindField,AfterField,Rows,AfterRow,RemoveRows,Clear,Append,Exit}
#[derive(Clone,Copy)]
struct Frame{node:usize,phase:Phase,next:usize,scan:usize,selected:Option<usize>,path_len:usize,value_len:usize,present:bool,required:bool,replace_parent:bool,synthetic:bool}
impl Frame{const fn empty()->Self{Self{node:0,phase:Phase::Enter,next:0,scan:0,selected:None,path_len:0,value_len:0,present:false,required:false,replace_parent:false,synthetic:false}}}
pub(crate) enum WalkStep{Progress,Candidate(Candidate),Complete}
/// 🧭️ Fixed path and compact frames never allocate while traversing arbitrarily wide descriptor/value collections.
pub(crate) struct Walk{frames:[Frame;64],depth:usize,pub path:[Part;64],pub values:[usize;64],root_scan:usize,seed_scan:usize,started:bool,done:bool,variant_index:usize}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"norm.inspector.walk")}
fn resource()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"staging.resource-limit")}
pub(crate) fn value<'a>(original:&'a NormInspectorPageOriginal,path:&[usize])->Option<&'a DslValue>{
 let mut value=&original.request.args;
 for index in path{value=match value{DslValue::Object(fields)=>&fields.get(*index)?.1,DslValue::Array(values)=>values.get(*index)?,_=>return None}}
 Some(value)
}
pub(crate) fn schema(original:&NormInspectorPageOriginal,node:usize)->Result<(&ActionArgDef,&ArgSchema),ValueError>{
 let rows=original.seeds.borrow();let row=rows.get().get(node).ok_or_else(invalid)?;
 original.schema(NormInspectorSchemaPath::new(&row.schema_path)?)
}
impl Walk{
 pub fn step_copy_bytes()->usize{std::mem::size_of::<Frame>()*2+std::mem::size_of::<Part>()+std::mem::size_of::<usize>()*8}
 pub fn new()->Self{Self{frames:[Frame::empty();64],depth:0,path:[Part::Draft;64],values:[0;64],root_scan:0,seed_scan:0,started:false,done:false,variant_index:0}}
 fn emit(&self,frame:Frame,kind:Kind)->WalkStep{WalkStep::Candidate(Candidate{node:frame.node,kind,required:frame.required,path_len:frame.path_len,value_len:frame.value_len,present:frame.present,variant_group:None})}
 fn push(&mut self,frame:Frame)->Result<(),ValueError>{if self.depth==64{return Err(resource())}self.frames[self.depth]=frame;self.depth+=1;Ok(())}
 pub fn step(&mut self,original:&NormInspectorPageOriginal)->Result<WalkStep,ValueError>{
  if self.done{return Ok(WalkStep::Complete)}
  if !self.started{
   let rows=original.seeds.borrow();
   let Some(row)=rows.get().get(self.seed_scan)else{return Err(invalid())};
   if row.schema_path!=original.schema_path.as_slice(){self.seed_scan+=1;return Ok(WalkStep::Progress)}
   let Some(fields)=original.request.args.as_object()else{return Err(invalid())};
   if let Some((key,_))=fields.get(self.root_scan){if key!=original.draft_key.as_str(){self.root_scan+=1;return Ok(WalkStep::Progress)}}
   let present=self.root_scan<fields.len();self.values[0]=self.root_scan;
   self.push(Frame{node:self.seed_scan,path_len:1,value_len:1,present,required:true,..Frame::empty()})?;self.started=true;return Ok(WalkStep::Progress)
  }
  if self.depth==0{self.done=true;return Ok(WalkStep::Complete)}
  let slot=self.depth-1;let frame=self.frames[slot];let current=frame.present.then(||value(original,&self.values[..frame.value_len])).flatten();
  let(_,shape)=schema(original,frame.node)?;
  let rows=original.seeds.borrow();let row=rows.get().get(frame.node).ok_or_else(invalid)?;
  match frame.phase{
   Phase::Enter=>{
    if frame.synthetic{self.frames[slot].phase=Phase::Exit;return Ok(self.emit(frame,Kind::Text))}
    match shape{
     ArgSchema::Object{..}=>{
      if current.and_then(DslValue::as_object).is_none(){
       self.frames[slot].phase=Phase::Exit;
       let seed=if self.depth==1&&original.draft_key==super::original::NormInspectorDraftKey::Replacement{if original.semantics.initial.is_none(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"norm.inspector.initial-source"))}Seed::Initial}else{Seed::Static(frame.node)};
       return Ok(self.emit(frame,Kind::Create{seed}))
      }
      self.frames[slot].phase=if row.variant_selector.is_some(){Phase::Selector}else{Phase::Fields};
     },
     ArgSchema::Array{..}|ArgSchema::Reference{many:true,..}=>self.frames[slot].phase=Phase::Rows,
     ArgSchema::Vector{..}=>{
      if current.and_then(DslValue::as_array).is_none(){self.frames[slot].phase=Phase::Exit;return Ok(self.emit(frame,Kind::Create{seed:Seed::Static(frame.node)}))}
      self.frames[slot].phase=Phase::Rows;
     },
     ArgSchema::Boolean=>{self.frames[slot].phase=Phase::Exit;return Ok(self.emit(frame,Kind::Boolean))},
     ArgSchema::String{options,..}if !options.is_empty()=>{self.frames[slot].phase=Phase::Exit;
      let mut output=self.emit(frame,Kind::Choice{count:options.len(),replace_parent:frame.replace_parent});
      if frame.replace_parent{let WalkStep::Candidate(candidate)=&mut output else{return Err(invalid())};candidate.variant_group=Some(self.variant_index);self.variant_index=self.variant_index.checked_add(1).ok_or_else(resource)?;}
      return Ok(output)},
     _=>{self.frames[slot].phase=Phase::Exit;return Ok(self.emit(frame,Kind::Text))},
    }
   },
   Phase::Selector=>{
    let index=row.variant_selector.ok_or_else(invalid)?;let child=*row.children.get(index).ok_or_else(invalid)?;
    let key=rows.get().get(child).and_then(|row|row.key.as_deref()).ok_or_else(invalid)?;
    let fields=current.and_then(DslValue::as_object).ok_or_else(invalid)?;
    if let Some((name,_))=fields.get(frame.scan){if name==key{self.frames[slot].selected=Some(frame.scan);self.frames[slot].phase=Phase::Fields}else{self.frames[slot].scan+=1}}else{self.frames[slot].phase=Phase::Fields}
   },
   Phase::Fields=>{
    let ArgSchema::Object{fields}=shape else{return Err(invalid())};
    let Some(input)=fields.get(frame.next)else{self.frames[slot].phase=Phase::Clear;return Ok(WalkStep::Progress)};
    let child=*row.children.get(frame.next).ok_or_else(invalid)?;
    let selected=frame.selected.and_then(|index|current.and_then(DslValue::as_object).and_then(|fields|fields.get(index))).and_then(|(_,value)|value.as_str());
    self.frames[slot].next+=1;
    if row.variant_selector.is_some()&&input.group.is_some()&&input.group.as_deref()!=selected{return Ok(WalkStep::Progress)}
    if frame.path_len==64||frame.value_len==64{return Err(resource())}
    self.path[frame.path_len]=Part::Key(child);self.frames[slot].scan=0;self.frames[slot].phase=Phase::FindField;
   },
   Phase::FindField=>{
    let child=*row.children.get(frame.next-1).ok_or_else(invalid)?;let key=rows.get().get(child).and_then(|row|row.key.as_deref()).ok_or_else(invalid)?;
    let fields=current.and_then(DslValue::as_object).ok_or_else(invalid)?;
    if let Some((name,_))=fields.get(frame.scan){if name!=key{self.frames[slot].scan+=1;return Ok(WalkStep::Progress)}}
    let present=frame.scan<fields.len();self.values[frame.value_len]=frame.scan;self.frames[slot].phase=Phase::AfterField;
    let ArgSchema::Object{fields}=shape else{return Err(invalid())};
    self.push(Frame{node:child,path_len:frame.path_len+1,value_len:frame.value_len+1,present,required:fields[frame.next-1].required,replace_parent:row.variant_selector==Some(frame.next-1),..Frame::empty()})?;
   },
   Phase::AfterField=>self.frames[slot].phase=Phase::Fields,
   Phase::Rows=>{
    let count=match shape{ArgSchema::Vector{dims,..}=>*dims as usize,_=>current.and_then(DslValue::as_array).map_or(0,|values|values.len())};
    if frame.next==count{self.frames[slot].next=0;self.frames[slot].phase=if matches!(shape,ArgSchema::Vector{..}){Phase::Append}else{Phase::RemoveRows};return Ok(WalkStep::Progress)}
    if frame.path_len==64||frame.value_len==64{return Err(resource())}
    self.path[frame.path_len]=Part::index(frame.next);self.values[frame.value_len]=frame.next;
    let present=current.and_then(DslValue::as_array).is_some_and(|values|frame.next<values.len());
    let(child,synthetic)=match shape{ArgSchema::Array{..}=>(*row.children.first().ok_or_else(invalid)?,false),_=>(frame.node,true)};
    self.frames[slot].phase=Phase::AfterRow;
    self.push(Frame{node:child,path_len:frame.path_len+1,value_len:frame.value_len+1,present,required:true,synthetic,..Frame::empty()})?;
   },
   Phase::AfterRow=>{
    self.frames[slot].next+=1;self.frames[slot].phase=Phase::Rows;
   },
   Phase::RemoveRows=>{
    let count=current.and_then(DslValue::as_array).map_or(0,|values|values.len());
    if frame.next==count{self.frames[slot].phase=Phase::Append;return Ok(WalkStep::Progress)}
    self.path[frame.path_len]=Part::index(frame.next);self.values[frame.value_len]=frame.next;
    self.frames[slot].next+=1;let mut selected=frame;selected.path_len+=1;selected.value_len+=1;return Ok(self.emit(selected,Kind::Remove{row:frame.next}))
   },
   Phase::Clear=>{self.frames[slot].phase=Phase::Exit;if !frame.required{return Ok(self.emit(frame,Kind::Clear))}},
   Phase::Append=>{
    self.frames[slot].phase=Phase::Exit;
    let seed=match shape{ArgSchema::Array{..}=>Seed::Static(*row.children.first().ok_or_else(invalid)?),ArgSchema::Reference{many:true,..}=>Seed::EmptyText,_=>return Ok(WalkStep::Progress)};
    return Ok(self.emit(frame,Kind::Append{seed,absent:current.is_none()}))
   },
   Phase::Exit=>{self.depth-=1;},
  }Ok(WalkStep::Progress)
 }
}
