from pathlib import Path
r=Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode")
p=r/"🫴️receiving/🦀️.rs";s=p.read_text()
s=s.replace("use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;","use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,OriginalReceivingPrimary};\nuse semio_framework_value::RetainedWorkOutcome;",1)
s=s.replace("complete:bool,closing:bool,closed:bool,","complete:bool,closing:bool,closed:bool,failed:bool,",1)
s=s.replace("complete:false,closing:false,closed:false}","complete:false,closing:false,closed:false,failed:false}")
s=s.replace("RetirementDemand{copy_bytes:size_of::<Self>().saturating_mul(2),depth:1,..Default::default()}","RetirementDemand{depth:1,..Default::default()}",1)
s=s.replace("RetirementDemand{capacity_bytes:size_of::<RetainedDocumentDriver>(),..Self::birth_demand()}","Self::birth_demand()",1)
s=s.replace("RecordCursor::Document(Some(Box::new(RetainedDocumentDriver::record(limits,range.1-range.0,maximum).map_err(pack)?)))","RecordCursor::Document(document_cursor::DocumentOwner::new(limits,range.1-range.0,maximum))",1)
s=s.replace("retained_capacity_bytes:size_of::<RetainedDocumentDriver>(),","",1)
def bounds(text,needle):
 start=text.index(needle);a=text.index("{",start);depth=1;i=a+1
 while depth:
  if text[i]=="{":depth+=1
  elif text[i]=="}":depth-=1
  i+=1
 return start,a,i
for name in ["admit<","admit_document<"]:
 start,a,end=bounds(s," pub fn "+name);header=s[start:a];body=s[a+1:end-1]
 header=header.replace("->Result<Option<(Self,RetainedCloneProgress)>,ValueError>","->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>")
 marker="  let maximum=";assert marker in body;body=body.replace(marker,"  work=1;let maximum=",1)
 s=s[:start]+header+"{let mut work=0;let result=(||{"+body+"})();RetainedWorkOutcome{result,work}}"+s[end:]
s=s.replace("demand.copy_bytes=size_of::<FieldValue>();return Ok(demand)","return Ok(demand)")
s=s.replace("demand.copy_bytes=size_of::<Frame>();return Ok(demand)","return Ok(demand)")
s=s.replace("Token::End(_)=>demand.copy_bytes=size_of::<Frame>(),","Token::End(_)=>{},")
s=s.replace("phase:if count==0{1}else{0},dense:true","phase:6,dense:true",1)
old="if let Some(table)=&frame.table{if table.phase==0"
new="if let Some(table)=&frame.table{if table.phase==6{demand.capacity_bytes=table.rows.div_ceil(8);return Ok(demand)}if table.phase==0"
assert old in s;s=s.replace(old,new,1)
old="let bitmap=if let Some(table)=&mut frame.table{table.bitmap.try_reserve_exact(table.rows.div_ceil(8)).map_err(|_|allocation())?;table.bitmap.capacity()}else{0};self.add_capacity(0,actual+bitmap)?;"
assert old in s;s=s.replace(old,"self.add_capacity(0,actual)?;",1)
s=s.replace("Box::new(FieldValue::Absent)","empty_field_box()?",1)
needle="fn allocation()->ValueError{"
pos=s.index(needle)
helper='''fn empty_field_box()->Result<Box<FieldValue>,ValueError>{let raw=unsafe{std::alloc::alloc(std::alloc::Layout::new::<FieldValue>())}.cast::<FieldValue>();if raw.is_null(){return Err(allocation())}unsafe{raw.write(FieldValue::Absent);Ok(Box::from_raw(raw))}}
'''
s=s[:pos]+helper+s[pos:]
start,a,end=bounds(s," pub fn step<");old=s[a+1:end-1]
begin=old.index("  let result=(||{");finish=old.index("\n  let progress=",begin);inner=old[begin:finish]
inner=inner.replace("if self.owned.frames.capacity()==0{","if self.owned.frames.capacity()==0{work=1;",1)
inner=inner.replace("if let Some(table)=&mut frame.table{if table.phase==0","if let Some(table)=&mut frame.table{if table.phase==6{work=1;let bytes=table.rows.div_ceil(8);table.bitmap.try_reserve_exact(bytes).map_err(|_|allocation())?;let actual=table.bitmap.capacity();table.phase=if table.rows==0{1}else{0};self.add_capacity(0,actual)?;return Ok(())}if table.phase==0",1)
inner=inner.replace("if table.phase==0{let FieldValue","if table.phase==0{work=1;let FieldValue",1)
inner=inner.replace("{let value=if table_present(table)","{work=1;let value=if table_present(table)",1)
inner=inner.replace("{let frame=self.owned.frames.pop().unwrap();","{work=1;let frame=self.owned.frames.pop().unwrap();",1)
inner=inner.replace("CopyPhase::Ready=>return self.attach()","CopyPhase::Ready=>{work=1;return self.attach()}",1)
inner=inner.replace("CopyPhase::Symbol{symbol,index,count}=>{let character","CopyPhase::Symbol{symbol,index,count}=>{work=1;let character",1)
inner=inner.replace("text.push(character);self.copy=","text.push(character);copied_bytes=character.len_utf8();self.copy=",1)
inner=inner.replace("let progress=bytes.reserve_byte_backing(grant)?;","work=1;let progress=bytes.reserve_byte_backing(grant)?;",1)
inner=inner.replace("if let Some(token)=self.pending.take(){return self.token(token)}","if let Some(token)=self.pending.take(){work=1;let bytes=match token{Token::StringChar(c)=>c.len_utf8(),Token::Byte(_)|Token::TableBitmap{..}=>1,_=>0};let result=self.token(token);if result.is_ok(){copied_bytes=bytes}return result}",1)
oldline="let before=self.cursor.allocated_bytes();let result=self.cursor.advance(source,self.range,&mut self.position,&mut self.sealed,demand.capacity_bytes);let after=self.cursor.allocated_bytes();self.add_capacity(before,after)?;if let Some(token)=result.map_err(pack)?{self.pending=Some(token)}Ok(())"
newline="let parser_before=self.cursor.allocated_bytes();let turn=self.cursor.advance(source,self.range,&mut self.position,&mut self.sealed,demand.capacity_bytes);work=turn.work;let after=self.cursor.allocated_bytes();self.add_capacity(parser_before,after)?;if turn.result.is_ok(){copied_bytes=demand.copy_bytes}if let Some(token)=turn.result.map_err(pack)?{self.pending=Some(token)}Ok(())"
assert oldline in inner;inner=inner.replace(oldline,newline,1)
newbody='''{
  let demand=match self.demands(source){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e.into()),work:0}};
  if self.failed{return RetainedWorkOutcome{result:Err(invalid("failed original Record receiver requires paid close").into()),work:0}}
  if self.complete||self.closing||!funded(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  if demand.capacity_bytes>self.maximum.saturating_sub(self.allocated)||native.owned_bytes().checked_add(demand.capacity_bytes).is_none_or(|n|n>native.maximum_bytes()){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Record allocation maximum exhausted").into()),work:0}}
  if let Err(error)=native.checkpoint(){return RetainedWorkOutcome{result:Err(error.into()),work:0}}
  let before=self.allocated;let mut work=0;let mut copied_bytes=0;
  if self.owned.scratch.capacity()>0{let released_bytes=self.owned.scratch.capacity();drop(std::mem::take(&mut self.owned.scratch));let turn=native.receive_retained_work(grant,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()})),work:1});if turn.result.is_err(){self.failed=true}return RetainedWorkOutcome{result:turn.result.map(|step|step.progress()),work:turn.work}}
'''+inner+'''
  let born=self.allocated-before;let progress=RetainedCloneProgress{copied_items:usize::from(result.is_ok()&&work>0||born>0||copied_bytes>0),copied_bytes,retained_capacity_bytes:born,..Default::default()};
  let result=result.map(|_|RetainedCloneStep::Progress(progress)).map_err(|error|error.with_retained_progress(progress));let turn=native.receive_retained_work(grant,RetainedWorkOutcome{result,work});if turn.result.is_err(){self.failed=true}RetainedWorkOutcome{result:turn.result.map(|step|step.progress()),work:turn.work}
 }'''
header=s[start:a].replace("->Result<RetainedCloneProgress,ValueError>","->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>")
s=s[:start]+header+newbody+s[end:]
for name in ["take_into(","take_value_into("]:
 start,a,end=bounds(s," pub fn "+name);header=s[start:a].replace("->Result<RetainedCloneProgress,ValueError>","->RetainedWorkOutcome<RetainedCloneProgress>");body=s[a+1:end-1]
 body=body.replace("copy_bytes:size_of::<RecordValue>(),","").replace("copy_bytes:size_of::<DslValue>()+size_of::<FieldValue>(),","")
 if name=="take_into(":body=body.replace("*output=self.owned.output.take();","work=1;*output=self.owned.output.take();")
 else:body=body.replace("  let record=self.owned.output.as_mut()","  work=1;let record=self.owned.output.as_mut()",1)
 s=s[:start]+header+"{let mut work=0;let result=(||{"+body+"})();RetainedWorkOutcome{result,work}}"+s[end:]
start,a,end=bounds(s," pub fn close_demands(")
s=s[:start]+''' pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.closed{return Ok(Default::default())}if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()})}if let Some(owned)=&self.retiring{if !owned.terminal_is_empty(){return owned.next_demand(body)}return Ok(RetirementDemand{depth:1,..Default::default()})}if !self.cursor.terminal_is_empty(){return self.cursor.source_close_demands().map_err(pack)}Ok(RetirementDemand{depth:1,..Default::default()})}'''+s[end:]
start,a,end=bounds(s," pub fn close_step(")
s=s[:start]+''' pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{
  if self.closed{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let demand=match self.close_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
  if !funded(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if !self.closing{let original=std::mem::take(&mut*self.owned);let result=match ControlledRetirement::new(original){Ok(owned)=>{self.retiring=Some(owned);self.pending=None;self.closing=true;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))},Err((error,original))=>{*self.owned=original;Err(error)}};return RetainedWorkOutcome{result,work:1}}
  if let Some(owned)=&mut self.retiring{if !owned.terminal_is_empty(){let result=owned.step(grant);return RetainedWorkOutcome{result,work:owned.last_turn_work()}}self.retiring=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
  if !self.cursor.terminal_is_empty(){let turn=self.cursor.close_step(grant);return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(pack),work:turn.work}}
  self.closed=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }'''+s[end:]
old='match self.0.close_step(grant){Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress,1),Err(error)=>RetirementStep::Failure(error,0)}'
new='let turn=self.0.close_step(grant);match turn.result{Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress,turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}'
assert old in s;s=s.replace(old,new,1)
s=s.replace("/// 🎟️ The allocation-free Body/Document header and returned owner transfer share an exact fixed copy quote.","/// 🎟️ A cold Body header moves original custody without copying its owned payload.")
s=s.replace("/// 🎟️ The Document header additionally pays for the one boxed driver that keeps the header itself small.","/// 🎟️ A cold Document header retains its declaration before separately funded fallible driver backing.")
p.write_text(s)
print("Pack receiving matched public full outcomes and staged actual ownership authored")
