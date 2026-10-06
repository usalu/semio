import{readFileSync,writeFileSync}from"node:fs";import{join}from"node:path";import assert from"node:assert/strict";
const root="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/📸️snapshot/🪶️sqlite";
const cells=String.raw`//! 📏️ Complete Run SQL cells are independent of native parser and typed backing ownership.
use super::{RunArtifact,RunTrigger,list};
use semio_framework_value::{ValueError,ValueRefusalKind,NativeDecodeControl};
use semio_framework_dsl_record::{RecordValue,FieldValue};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase};
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn add(total:&mut usize,bytes:usize,maximum:usize)->Result<(),ValueError>{*total=total.checked_add(bytes).filter(|n|*n<=maximum).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Run semantic SQL cell bytes exceed caller limit"))?;Ok(())}
fn ieee(number:f64)->usize{if number.is_nan(){11}else if number.is_infinite(){32}else{22}}
struct Census<'a,'b>{total:usize,completed:usize,rows:usize,control:&'a mut SqliteSnapshotControl<'b>}
impl Census<'_,'_>{
 fn bytes(&mut self,bytes:usize)->Result<(),ValueError>{add(&mut self.total,bytes,self.control.limits().max_value_bytes)}
 fn text(&mut self,text:&str)->Result<(),ValueError>{self.bytes(text.len())}
 fn step(&mut self)->Result<(),ValueError>{self.completed=self.completed.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Run semantic cell census work overflow"))?;if self.completed%256==0||self.completed==self.rows{self.control.checkpoint(SqliteSnapshotPhase::EncodeNative,self.completed,self.rows)?;}Ok(())}
}
pub(super) fn owner(value:&RunArtifact,rows:usize,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,rows)?;let mut c=Census{total:0,completed:0,rows,control};c.bytes(24)?;
 for text in[&value.schema,&value.workflow_ref,&value.workflow_checkpoint_id,&value.input_collection_ref,&value.input_snapshot_id,&value.output_collection_ref,&value.started_at]{c.text(text)?;}if let Some(text)=&value.finished_at{c.text(text)?;}c.step()?;
 c.bytes(16)?;match&value.trigger{RunTrigger::Manual{actor}=>{c.text("manual")?;c.text(actor)?;},RunTrigger::Automation{automation_ref,event_fingerprint}=>{c.text("automation")?;c.text(automation_ref)?;c.text(event_fingerprint)?;}}c.step()?;
 for value in&value.parameter_values{c.bytes(24)?;c.text(&value.parameter_id)?;c.text(&value.value)?;c.step()?;}
 for node in&value.node_records{c.bytes(32)?;for text in[&node.node_id,&node.document_fingerprint,&node.config_fingerprint]{c.text(text)?;}c.bytes(ieee(node.duration_ms))?;
  for(direction,values)in[("input",&node.input_fingerprints),("output",&node.output_fingerprints)]{for value in values{c.bytes(24)?;c.text(direction)?;c.text(&value.port_id)?;c.text(&value.fingerprint)?;c.step()?;}}
  for value in&node.outputs{c.bytes(24)?;for text in[&value.port_id,&value.artifact_id,&value.path]{c.text(text)?;}c.step()?;}c.step()?;
 }
 for value in&value.logs{c.bytes(24)?;for text in[&value.node_id,&value.level,&value.message,&value.at]{c.text(text)?;}c.step()?;}if c.completed!=rows{return Err(invalid("Run complete SQL cell traversal differs from admitted rows"))}c.control.check_value_bytes(c.total)
}
fn text(value:Option<&FieldValue>)->Result<&str,ValueError>{match value{Some(FieldValue::Text(text))=>Ok(text),_=>Err(invalid("Run SQL semantic text requires literal Text"))}}
fn item(value:&FieldValue)->Result<&RecordValue,ValueError>{match value{FieldValue::Record(record)=>Ok(record),FieldValue::Block(value)=>item(value),_=>Err(invalid("Run SQL semantic owner requires a literal Record"))}}
pub(super) fn record(record:&RecordValue,maximum:usize,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(0)?;let mut total=0;add(&mut total,24,maximum)?;
  for field in[0,1,2,3,4,6,11]{add(&mut total,text(record.get(field))?.len(),maximum)?;native.step()?;}if !matches!(record.get(12),None|Some(FieldValue::Absent)){add(&mut total,text(record.get(12))?.len(),maximum)?;}native.step()?;
  add(&mut total,16,maximum)?;let trigger=item(record.get(8).ok_or_else(||invalid("Run trigger owner is required"))?)?;add(&mut total,text(trigger.get(0))?.len(),maximum)?;for field in 1..4{if !matches!(trigger.get(field),None|Some(FieldValue::Absent)){add(&mut total,text(trigger.get(field))?.len(),maximum)?;}native.step()?;}
  for value in list(record.get(5))?{let row=item(value)?;add(&mut total,24,maximum)?;for field in 0..2{add(&mut total,text(row.get(field))?.len(),maximum)?;}native.step()?;}
  for value in list(record.get(9))?{let row=item(value)?;add(&mut total,32,maximum)?;for field in[0,2,3]{add(&mut total,text(row.get(field))?.len(),maximum)?;}let number=match row.get(7){Some(FieldValue::Float(number))=>*number,_=>return Err(invalid("Run SQL duration requires native Float"))};add(&mut total,ieee(number),maximum)?;
   for(field,direction)in[(4,"input"),(5,"output")]{for value in list(row.get(field))?{let value=item(value)?;add(&mut total,24,maximum)?;add(&mut total,direction.len(),maximum)?;for field in 0..2{add(&mut total,text(value.get(field))?.len(),maximum)?;}native.step()?;}}
   for value in list(row.get(6))?{let value=item(value)?;add(&mut total,24,maximum)?;for field in 0..3{add(&mut total,text(value.get(field))?.len(),maximum)?;}native.step()?;}native.step()?;
  }
  for value in list(record.get(10))?{let row=item(value)?;add(&mut total,24,maximum)?;for field in 0..4{add(&mut total,text(row.get(field))?.len(),maximum)?;}native.step()?;}native.checkpoint()
 })
}
`;
const path=join(root,"📏️value/🦀️.rs"),before=readFileSync(path,"utf8"),native=join(root,"🦀️.rs"),nativeBefore=readFileSync(native,"utf8");let nativeAfter=nativeBefore.replace("control.check_rows(2)?;let maximum=","control.check_rows(2)?;control.check_value_bytes(40)?;let maximum=").replace("workload(self,control,SqliteSnapshotPhase::EncodeNative).map(|_|())","let rows=workload(self,control,SqliteSnapshotPhase::EncodeNative)?;value_bytes::owner(self,rows,control)");assert.notEqual(nativeBefore,nativeAfter);
writeFileSync(join(import.meta.dir,"held-provider-pairs.json"),JSON.stringify([{path,before,after:cells},{path:native,before:nativeBefore,after:nativeAfter}],null,2)+"\n");console.log("[DEBUG] Run typed and borrowed complete seven-table SQL cells HELD paths=2 production_mutations=0 paid_reconstruction_unchanged=true");

