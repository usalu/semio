import{readFileSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import assert from"node:assert/strict";
const repo="/Users/ueli/Documents/semio",root=join(repo,"🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/📸️snapshot/🪶️sqlite");
const cells=String.raw`//! 📏️ Workflow SQL semantic cells retain every identity, relationship and IEEE companion.
use super::{WorkflowSnapshot,WorkflowParameter,ValueError,native_list,native_record};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase};
use semio_framework_dsl_record::{FieldValue,RecordValue};
fn invalid(message:&str)->ValueError{super::super::workflow_invalid(message)}
struct Extent{bytes:usize,maximum:usize}
impl Extent{
 fn add(&mut self,n:usize)->Result<(),ValueError>{self.bytes=self.bytes.checked_add(n).filter(|n|*n<=self.maximum).ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit,"Workflow semantic SQL cells exceed caller limit"))?;Ok(())}
 fn text(&mut self,s:&str)->Result<(),ValueError>{self.add(s.len())}
 fn ieee(&mut self,n:f64)->Result<(),ValueError>{self.add(if n.is_nan(){11}else if n.is_infinite(){32}else{22})}
}
pub(super) fn typed(value:&WorkflowSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{
 let mut e=Extent{bytes:0,maximum:control.limits().max_value_bytes};e.add(24)?;e.text(&value.schema)?;e.text(&value.graph.schema)?;control.checkpoint(phase,0,0)?;
 for(index,node)in value.graph.nodes.iter().enumerate(){e.add(24)?;for text in[&node.id,&node.plugin_id,&node.app_id,&node.label,&node.yields,&node.artifact_ref,&node.config_ref]{e.text(text)?;}for number in[node.x,node.y,node.width,node.height]{e.ieee(number)?;}
  for(bucket,ports)in[("input",&node.inputs),("output",&node.outputs)]{for(index,port)in ports.iter().enumerate(){e.add(64)?;for text in[bucket,&port.id,&port.spec.id,&port.spec.label]{e.text(text)?;}if let Some(kind)=&port.spec.kind_id{e.text(kind)?;}if(index+1)%256==0{control.checkpoint(phase,index+1,ports.len())?;}}}
  if(index+1)%256==0{control.checkpoint(phase,index+1,value.graph.nodes.len())?;}
 }
 for(index,edge)in value.graph.edges.iter().enumerate(){e.add(24)?;for text in[&edge.id,&edge.source_node_id,&edge.source_port_id,&edge.target_node_id,&edge.target_port_id]{e.text(text)?;}let c=&edge.contract;e.add(32)?;e.text(&c.kind_id)?;let(kind,text)=match&c.wire{super::super::MediaWireFormat::Binary{format_kind}=>("binary",format_kind),super::super::MediaWireFormat::Document{schema}=>("document",schema),super::super::MediaWireFormat::Intrinsic{schema}=>("intrinsic",schema)};e.text(kind)?;e.text(text)?;if c.conversion.is_some(){e.add(16)?;}if(index+1)%256==0{control.checkpoint(phase,index+1,value.graph.edges.len())?;}}
 for(index,parameter)in value.parameters.iter().enumerate(){let(id,name,kind)=match parameter{WorkflowParameter::Numeric{id,name,..}=>(id,name,"numeric"),WorkflowParameter::Categorical{id,name,..}=>(id,name,"categorical"),WorkflowParameter::Toggle{id,name,..}=>(id,name,"toggle"),WorkflowParameter::Text{id,name,..}=>(id,name,"text")};e.add(24)?;for text in[id.as_str(),name,kind]{e.text(text)?;}
  match parameter{WorkflowParameter::Numeric{value,min,max,step,..}=>{e.add(16)?;e.ieee(*value)?;for value in[min,max,step]{if let Some(value)=value{e.ieee(*value)?;}}},WorkflowParameter::Categorical{value,options,..}=>{e.add(16)?;e.text(value)?;for(index,option)in options.iter().enumerate(){e.add(24)?;e.text(option)?;if(index+1)%256==0{control.checkpoint(phase,index+1,options.len())?;}}},WorkflowParameter::Toggle{..}=>e.add(24)?,WorkflowParameter::Text{value,..}=>{e.add(16)?;e.text(value)?;}}
  if(index+1)%256==0{control.checkpoint(phase,index+1,value.parameters.len())?;}
 }
 for(index,row)in value.parameter_bindings.iter().enumerate(){e.add(24)?;for text in[&row.parameter_id,&row.node_id,&row.field_path]{e.text(text)?;}if(index+1)%256==0{control.checkpoint(phase,index+1,value.parameter_bindings.len())?;}}
 for(index,row)in value.inputs.iter().enumerate(){e.add(40)?;for text in[&row.id,&row.kind_id,&row.selector]{e.text(text)?;}if(index+1)%256==0{control.checkpoint(phase,index+1,value.inputs.len())?;}}
 for(index,row)in value.input_bindings.iter().enumerate(){e.add(24)?;for text in[&row.input_id,&row.node_id,&row.port_id]{e.text(text)?;}if(index+1)%256==0{control.checkpoint(phase,index+1,value.input_bindings.len())?;}}
 for(index,row)in value.output_bindings.iter().enumerate(){e.add(24)?;for text in[&row.node_id,&row.port_id,&row.path_template]{e.text(text)?;}if(index+1)%256==0{control.checkpoint(phase,index+1,value.output_bindings.len())?;}}
 control.check_value_bytes(e.bytes)?;control.checkpoint(phase,0,0)
}
fn text(value:Option<&FieldValue>)->Result<&str,ValueError>{match value{Some(FieldValue::Text(s))=>Ok(s),_=>Err(invalid("Workflow semantic cells require literal Text"))}}
fn scalar(value:Option<&FieldValue>)->Result<f64,ValueError>{match value{Some(FieldValue::Float(n))=>Ok(*n),_=>Err(invalid("Workflow semantic IEEE cells require literal Float"))}}
fn optional(value:Option<&FieldValue>)->bool{!matches!(value,None|Some(FieldValue::Absent))}
fn record(value:Option<&FieldValue>)->Result<&RecordValue,ValueError>{native_record(value.ok_or_else(||invalid("Workflow semantic entity is missing"))?)}
pub(super) fn borrowed(value:&RecordValue,native:&mut semio_framework_value::NativeDecodeControl<'_>,maximum:usize)->Result<(),ValueError>{
 let mut e=Extent{bytes:0,maximum};e.add(24)?;e.text(text(value.get(0))?)?;let graph=record(value.get(1))?;e.text(text(graph.get(0))?)?;
 let nodes=native_list(graph.get(1))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(nodes.len())?;for node in nodes{let row=native_record(node)?;e.add(24)?;for field in 0..7{e.text(text(row.get(field))?)?;}for field in 7..11{e.ieee(scalar(row.get(field))?)?;}
  for(field,bucket)in[(11,"input"),(12,"output")]{let ports=native_list(row.get(field))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(ports.len())?;for port in ports{let p=native_record(port)?;e.add(64)?;e.text(bucket)?;for field in 0..3{e.text(text(p.get(field))?)?;}if optional(p.get(6)){e.text(text(p.get(6))?)?;}native.step()?;}Ok(())})?;}native.step()?;}Ok(())})?;
 let edges=native_list(graph.get(2))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(edges.len())?;for edge in edges{let row=native_record(edge)?;e.add(24)?;for field in 0..5{e.text(text(row.get(field))?)?;}let c=record(row.get(5))?;e.add(32)?;e.text(text(c.get(0))?)?;e.text(text(c.get(3))?)?;for field in[4,5]{if optional(c.get(field)){e.text(text(c.get(field))?)?;}}for field in[6,7]{if optional(c.get(field)){e.add(8)?;}}native.step()?;}Ok(())})?;
 let parameters=match value.get(2){Some(FieldValue::Statements(values))=>values.as_slice(),None|Some(FieldValue::Absent)=>&[],_=>return Err(invalid("Workflow semantic parameters require literal statements"))};
 native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(parameters.len())?;for(kind,p)in parameters{e.add(24)?;e.text(kind)?;for field in 0..2{e.text(text(p.get(field))?)?;}match kind.as_str(){"numeric"=>{e.add(16)?;e.ieee(scalar(p.get(2))?)?;for field in 3..6{if optional(p.get(field)){e.ieee(scalar(p.get(field))?)?;}}},"categorical"=>{e.add(16)?;e.text(text(p.get(2))?)?;let options=native_list(p.get(3))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(options.len())?;for option in options{e.add(24)?;e.text(text(Some(option))?)?;native.step()?;}Ok(())})?;},"toggle"=>e.add(24)?,"text"=>{e.add(16)?;e.text(text(p.get(2))?)?;},_=>return Err(invalid("Workflow semantic parameter branch is undeclared"))}native.step()?;}Ok(())})?;
 for field in 3..7{let rows=native_list(value.get(field))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(rows.len())?;for row in rows{let row=native_record(row)?;e.add(if field==4{40}else{24})?;for id in 0..3{e.text(text(row.get(id))?)?;}native.step()?;}Ok(())})?;}
 native.checkpoint()
}
`;
const backing=String.raw`//! 💰️ Paid Workflow indexes retain original rows under one cumulative native allocation ledger.
use super::{SqliteRow,ValueError,entity};
use semio_framework_value::NativeDecodeControl;
use std::cmp::Ordering;
fn invalid(message:&str)->ValueError{super::super::workflow_invalid(message)}
fn sort(rows:&mut[&SqliteRow],native:&mut NativeDecodeControl<'_>,mut compare:impl FnMut(&SqliteRow,&SqliteRow)->Result<Ordering,ValueError>)->Result<(),ValueError>{
 fn sift(rows:&mut[&SqliteRow],mut root:usize,end:usize,native:&mut NativeDecodeControl<'_>,compare:&mut impl FnMut(&SqliteRow,&SqliteRow)->Result<Ordering,ValueError>)->Result<(),ValueError>{loop{let Some(mut child)=root.checked_mul(2).and_then(|n|n.checked_add(1)).filter(|n|*n<end)else{return Ok(())};if child+1<end{native.step()?;if compare(rows[child],rows[child+1])?==Ordering::Less{child+=1;}}native.step()?;if compare(rows[root],rows[child])?!=Ordering::Less{return Ok(())}rows.swap(root,child);root=child;}}
 native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(0)?;for root in(0..rows.len()/2).rev(){let end=rows.len();sift(rows,root,end,native,&mut compare)?;}for end in(1..rows.len()).rev(){rows.swap(0,end);sift(rows,0,end,native,&mut compare)?;}native.checkpoint()})
}
fn unique(rows:&mut[&SqliteRow],columns:usize,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(rows.len())?;for row in rows.iter(){entity(row,columns)?;row.integer(1)?;native.step()?;}Ok(())})?;
 sort(rows,native,|a,b|Ok(a.rowid.cmp(&b.rowid)))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(0)?;for pair in rows.windows(2){if pair[0].rowid==pair[1].rowid{return Err(invalid("Workflow entity identity is duplicated"))}native.step()?;}Ok(())})
}
fn refs<'a>(rows:&'a[SqliteRow],columns:usize,native:&mut NativeDecodeControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{let mut result=native.allocate_vec(rows.len())?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(rows.len())?;for row in rows{result.push(row);native.step()?;}Ok(())})?;unique(&mut result,columns,native)?;Ok(result)}
pub(super) fn ordered<'a>(rows:&[&'a SqliteRow],columns:usize,slot:usize,native:&mut NativeDecodeControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
 let mut result=native.allocate_vec(rows.len())?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(rows.len())?;for row in rows{result.push(*row);native.step()?;}Ok(())})?;unique(&mut result,columns,native)?;sort(&mut result,native,|a,b|Ok(a.integer(slot)?.cmp(&b.integer(slot)?)))?;
 native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(result.len())?;for(index,row)in result.iter().enumerate(){if row.integer(slot)?!=i64::try_from(index).map_err(|_|invalid("Workflow ordinal exceeds INTEGER width"))?{return Err(invalid("Workflow collection ordinals must be dense and unique"))}native.step()?;}Ok(())})?;Ok(result)
}
pub(super) fn parent_rows<'a>(rows:&'a[SqliteRow],parent:i64,columns:usize,native:&mut NativeDecodeControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
 let rows=refs(rows,columns,native)?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(rows.len())?;for row in&rows{if row.integer(1)?!=parent{return Err(invalid("Workflow child has an unknown parent"))}native.step()?;}Ok(())})?;ordered(&rows,columns,2,native)
}
pub(super) struct Groups<'a>{rows:Vec<&'a SqliteRow>,used:Vec<u8>,remaining:usize}
impl<'a>Groups<'a>{
 fn new(rows:&'a[SqliteRow],columns:usize,single:bool,native:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let mut rows=refs(rows,columns,native)?;sort(&mut rows,native,|a,b|Ok(a.integer(1)?.cmp(&b.integer(1)?)))?;if single{native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(0)?;for pair in rows.windows(2){if pair[0].integer(1)?==pair[1].integer(1)?{return Err(invalid("Workflow branch requires one unique entity per parent"))}native.step()?;}Ok(())})?;}let mut used=native.allocate_vec(rows.len())?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(rows.len())?;for _ in&rows{used.push(0);native.step()?;}Ok(())})?;let remaining=rows.len();Ok(Self{rows,used,remaining})}
 fn range(&self,parent:i64)->std::ops::Range<usize>{let start=self.rows.partition_point(|row|row.integer(1).expect("validated Workflow parent INTEGER")<parent);let end=self.rows.partition_point(|row|row.integer(1).expect("validated Workflow parent INTEGER")<=parent);start..end}
 pub(super) fn remove(&mut self,parent:&i64)->Option<&[&'a SqliteRow]>{let range=self.range(*parent);if range.is_empty()||self.used[range.start]!=0{return None}self.used[range.start]=1;self.remaining-=range.len();Some(&self.rows[range])}
 pub(super) fn is_empty(&self)->bool{self.remaining==0}
}
pub(super) struct Single<'a>(Groups<'a>);
impl<'a>Single<'a>{pub(super) fn remove(&mut self,parent:&i64)->Option<&'a SqliteRow>{self.0.remove(parent).map(|rows|rows[0])}pub(super) fn is_empty(&self)->bool{self.0.is_empty()}}
pub(super) fn single_rows<'a>(rows:&'a[SqliteRow],columns:usize,native:&mut NativeDecodeControl<'_>)->Result<Single<'a>,ValueError>{Groups::new(rows,columns,true,native).map(Single)}
pub(super) fn grouped<'a>(rows:&'a[SqliteRow],columns:usize,native:&mut NativeDecodeControl<'_>)->Result<Groups<'a>,ValueError>{Groups::new(rows,columns,false,native)}
`;
const path=join(root,"🦀️.rs"),before=readFileSync(path,"utf8");let after=before.replace("use std::collections::BTreeMap;\n","");
after=after.replace("validate_sqlite_database_schema,artifact:","validate_sqlite_database_schema_controlled,artifact:");
const start=after.indexOf("fn ordered<'a>"),end=after.indexOf("fn boolean(",start);assert(start>=0&&end>start);after=after.slice(0,start)+'#[path="📏️cells/🦀️.rs"]mod cells;\n#[path="💰️backing/🦀️.rs"]mod backing;\nuse backing::{ordered,parent_rows,single_rows,grouped};\n'+after.slice(end);
after=after.replace("control.check_rows(2)?;let maximum=control.limits().max_rows;","control.check_rows(2)?;control.check_value_bytes(24)?;let maximum=control.limits().max_rows;let value_maximum=control.limits().max_value_bytes;");
after=after.replace("native_rows(record,maximum,native)?;Self::__dsl_from_record_controlled","native_rows(record,maximum,native)?;cells::borrowed(record,native,value_maximum)?;Self::__dsl_from_record_controlled");
after=after.replace("workload(self,control,SqliteSnapshotPhase::EncodeNative)?;store::encode","workload(self,control,SqliteSnapshotPhase::EncodeNative)?;cells::typed(self,control,SqliteSnapshotPhase::EncodeNative)?;store::encode");
after=after.replace("workload(self,control,SqliteSnapshotPhase::EncodeNative).map(|_|())","workload(self,control,SqliteSnapshotPhase::EncodeNative)?;cells::typed(self,control,SqliteSnapshotPhase::EncodeNative)");
after=after.replace("validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;","validate_sqlite_database_schema_controlled(database,Self::SQLITE_SCHEMA,SqliteSnapshotPhase::ReconstructSnapshot,control)?;");
const workspace=after.indexOf("let count=database.tables.iter().try_fold"),construct=after.indexOf("  let graph_id=",workspace);assert(workspace>0&&construct>workspace);
after=after.slice(0,workspace)+`let count=database.tables.iter().try_fold(0usize,|n,table|add(n,table.rows.len()))?;
  control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,progress|{let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|progress(event.completed,event.total);let mut native=semio_framework_value::NativeDecodeControl::new(remaining,&mut callback);let result=(||->Result<Self,ValueError>{native.begin_stage(count)?;
`+after.slice(construct);
const close="parameters,parameter_bindings,inputs,input_bindings,output_bindings})\n";assert.equal(after.split(close).length,2);after=after.replace(close,close.replace("\n","")+ "\n  })();(result,native.owned_bytes())})?\n");
const source=join(root,"🟦️.ts"),sourceBefore=readFileSync(source,"utf8");let sourceAfter=sourceBefore.replace(",artifactSqliteValueBudget,type ArtifactSqliteOptions",",type ArtifactSqliteOptions").replace("let owned=total*512,completed=2;artifactSqliteValueBudget(owned,options);","let completed=2;").replace("const charge=(bytes:number):void=>{owned+=bytes;artifactSqliteValueBudget(owned,options)};","").replace("charge(rows.length*8);","");
assert(!after.includes("BTreeMap"));assert(!sourceAfter.includes("charge("));
const pairs=[{path:join(root,"📏️cells/🦀️.rs"),before:null,after:cells},{path:join(root,"💰️backing/🦀️.rs"),before:null,after:backing},{path,before,after},{path:source,before:sourceBefore,after:sourceAfter}];writeFileSync(join(import.meta.dir,"held-provider-pairs.json"),JSON.stringify(pairs,null,2)+"\n");console.log("[DEBUG] Workflow coherent semantic and native paid indexes HELD paths=4 production_mutations=0");
