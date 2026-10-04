//! 📏️ Held constant-storage census of explicit public owner fields; no encoded output is constructed.
use semio_framework_os_kernel as store;
use store::sqlite_snapshot::*;
pub(super) struct Census<'c,'p>{control:&'c mut SqliteSnapshotControl<'p>,semantic:usize,body:usize,units:usize,entities:usize,expected:usize}
fn add(a:usize,b:usize)->Result<usize,ValueError>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"public native forecast count overflow"))}
fn mul(a:usize,b:usize)->Result<usize,ValueError>{a.checked_mul(b).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"public native forecast size overflow"))}
impl<'c,'p>Census<'c,'p>{
 pub(super)fn new(control:&'c mut SqliteSnapshotControl<'p>,sql:&str,rows:usize)->Result<Self,ValueError>{
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;
  if sql.len()>control.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"public native forecast exceeds authored schema allowance"))}
  control.check_rows(rows)?;Ok(Self{control,semantic:0,body:0,units:0,entities:0,expected:rows})
 }
 fn unit(&mut self)->Result<(),ValueError>{self.units=add(self.units,1)?;if self.units%256==0{self.control.checkpoint(SqliteSnapshotPhase::EncodeNative,self.units,0)?;}Ok(())}
 fn semantic(&mut self,bytes:usize)->Result<(),ValueError>{self.semantic=add(self.semantic,bytes)?;self.control.check_value_bytes(self.semantic)}
 fn syntax(&mut self,bytes:usize)->Result<(),ValueError>{self.body=add(self.body,bytes)?;self.unit()}
 pub(super)fn labels(&mut self,labels:&[&str])->Result<(),ValueError>{for label in labels{self.syntax(add(mul(label.len(),6)?,32)?)?;}Ok(())}
 pub(super)fn entity(&mut self,scalar_cells:usize)->Result<(),ValueError>{
  self.entities=add(self.entities,1)?;self.control.check_rows(self.entities)?;self.semantic(mul(scalar_cells,8)?)?;self.syntax(add(mul(scalar_cells,32)?,32)?)
 }
 pub(super)fn text(&mut self,text:&str)->Result<(),ValueError>{
  self.semantic(text.len())?;self.control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,text.len())?;
  let mut completed=0;let mut escaped=0;
  for character in text.chars(){
   let length=character.len_utf8();
   let bound=if character<' ' {6}else if character=='\\'||character=='"'{2}else if character.is_ascii(){1}else{mul(length,6)?};
   escaped=add(escaped,bound)?;let previous=completed;completed=add(completed,length)?;
   if previous/65536!=completed/65536||completed==text.len(){self.control.checkpoint(SqliteSnapshotPhase::EncodeNative,completed,text.len())?;}
  }
  self.syntax(add(escaped,32)?)
 }
 pub(super)fn optional(&mut self,text:Option<&str>)->Result<(),ValueError>{if let Some(text)=text{self.text(text)?;}else{self.syntax(4)?;}Ok(())}
 pub(super)fn finish(self,encoding:SnapshotEncoding,envelope:Option<&str>)->Result<(),ValueError>{
  if self.entities!=self.expected{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"public native census differs from its actual SQL entity count"))}
  let bytes=match encoding{
   SnapshotEncoding::Text=>self.body,
   SnapshotEncoding::Binary=>{
    let frames=add(self.body.div_ceil(1024*1024).max(1),2)?;
    let raw=add(self.body,173)?;
    let stored=add(add(mul(raw,9)?,7)?/8,mul(frames,3)?)?;
    add(add(store::os_pack::format::HEADER_SIZE,store::os_pack::format::FOOTER_SIZE)?,add(stored,add(mul(frames,26)?,16)?)?)?
   }
  };
  let prefix=if let Some(id)=envelope{
   let component=match encoding{SnapshotEncoding::Binary=>store::os_store::semio_format::Component::Pack,SnapshotEncoding::Text=>store::os_store::semio_format::Component::Dsl};
   store::os_store::semio_format::declared_envelope_prefix_len(id,component,1)?
  }else{0};
  if add(bytes,prefix)?>self.control.limits().max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"public native forecast exceeds physical file allowance"))}
  self.control.checkpoint(SqliteSnapshotPhase::EncodeNative,self.units,self.units)
 }
}
