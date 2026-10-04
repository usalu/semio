//! 📏️ Literal Generation relational census before typed native field ownership.
use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_dsl_record::NativeSchemaControl;
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::RecordValue;
use semio_framework_value::DslValue;
use semio_framework_artifact_flow_flow::{FlowUi,neural::Tree};
struct Rows<'a,C>{control:&'a mut C,maximum:usize,count:usize}
impl<C:NativeSchemaControl> Rows<'_,C>{
 fn add(&mut self,count:usize)->Result<(),ValueError>{self.count=self.count.checked_add(count).filter(|count|*count<=self.maximum).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit, "Generation semantic row limit exceeded"))?;self.control.step()}
 fn push<T>(&mut self,pending:&mut Vec<T>,value:T)->Result<(),ValueError>{if pending.len()==pending.capacity(){let growth=pending.capacity().max(1);let bytes=growth.checked_mul(std::mem::size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit, "Generation census frontier size overflow"))?;self.control.charge(bytes)?;pending.try_reserve_exact(growth).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Generation census frontier allocation failed"))?;}pending.push(value);Ok(())}
}
enum Owned<'a>{Dictionary(&'a Dictionary),Tree(&'a Tree),Gui(&'a FlowUi),Answer(&'a DslValue)}
pub(super) fn owned<C:NativeSchemaControl>(snapshot:&ControlledSnapshot,maximum:usize,c:&mut C)->Result<usize,ValueError>{c.scoped_stage(|c|{c.begin_stage(0)?;let mut r=Rows{control:c,maximum,count:0};let mut pending=Vec::new();r.add(2)?;r.add(snapshot.host_snapshot.synapses.len())?;r.add(snapshot.host_snapshot.layout.len())?;
 for widget in &snapshot.host_snapshot.widgets{r.add(2)?;match widget{
  Widget::Neuron{params,input_ports,output_ports,..}=>{r.add(input_ports.len())?;r.add(output_ports.len())?;r.push(&mut pending,Owned::Dictionary(params))?;},
  Widget::OutputPreview{preview,expanded,..}=>{r.add(expanded.len())?;r.push(&mut pending,Owned::Dictionary(preview))?;},
  Widget::Cluster{tree,flow,..}=>{r.push(&mut pending,Owned::Tree(tree))?;r.push(&mut pending,Owned::Gui(flow))?;},_=>{}
 }}
 for generation in &snapshot.generation.generations{r.add(1)?;for value in generation.values.values(){r.add(1)?;r.push(&mut pending,Owned::Answer(value))?;}}
 while let Some(value)=pending.pop(){match value{
  Owned::Dictionary(value)=>{r.add(1)?;for(_,value)in value.iter(){r.add(2)?;if let NeuralValue::Dictionary(value)=value{r.push(&mut pending,Owned::Dictionary(value))?;}}},
  Owned::Tree(value)=>{r.add(1)?;r.add(value.synapses.len())?;for neuron in &value.neurons{r.add(1)?;r.push(&mut pending,Owned::Dictionary(&neuron.params))?;if let Some(child)=&neuron.tree{r.push(&mut pending,Owned::Tree(child))?;}}},
  Owned::Gui(value)=>{r.add(1)?;for _ in value.nodes.iter(){r.add(2)?;}for preview in &value.previews{r.add(1)?;r.add(preview.expanded.len())?;r.push(&mut pending,Owned::Dictionary(&preview.preview))?;}},
  Owned::Answer(value)=>{r.add(1)?;match value{semio_framework_value::DslValue::Array(values)=>for value in values{r.add(1)?;r.push(&mut pending,Owned::Answer(value))?;},semio_framework_value::DslValue::Object(values)=>for(_,value)in values{r.add(1)?;r.push(&mut pending,Owned::Answer(value))?;},_=>{}}}
 }}Ok(r.count)
})}
fn field(record:&RecordValue,id:u16)->Result<&FieldValue,ValueError>{record.fields.get(&id).ok_or(ValueError::new(ValueRefusalKind::InvalidValue, "Generation census field missing"))}
fn record(value:&FieldValue)->Result<&RecordValue,ValueError>{match value{semio_framework_dsl_record::FieldValue::Record(value)=>Ok(value),semio_framework_dsl_record::FieldValue::Block(value)=>record(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue, "Generation census record differs"))}}
fn list(value:&FieldValue)->Result<&[FieldValue],ValueError>{match value{semio_framework_dsl_record::FieldValue::List(value)=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue, "Generation census list differs"))}}
fn map(value:&FieldValue)->Result<&[(String,FieldValue)],ValueError>{match value{semio_framework_dsl_record::FieldValue::Map(value)=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue, "Generation census map differs"))}}
fn dynamic(value:&FieldValue)->Result<&DslValue,ValueError>{match value{semio_framework_dsl_record::FieldValue::Value(value)=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue, "Generation census intrinsic value differs"))}}
fn optional(value:Option<&FieldValue>)->Option<&FieldValue>{value.filter(|v|!matches!(v,FieldValue::Absent))}
fn member<'a>(value:&'a DslValue,key:&str)->Result<&'a DslValue,ValueError>{let semio_framework_value::DslValue::Object(values)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Generation census object differs"))};let mut found=values.iter().filter(|(k,_)|k==key);let value=&found.next().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue, "Generation census member missing"))?.1;if found.next().is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Generation census member repeated"))}Ok(value)}
fn array(value:&DslValue)->Result<&[DslValue],ValueError>{match value{semio_framework_value::DslValue::Array(values)=>Ok(values),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue, "Generation census array differs"))}}
fn object(value:&DslValue)->Result<&[(String,DslValue)],ValueError>{match value{semio_framework_value::DslValue::Object(values)=>Ok(values),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue, "Generation census object differs"))}}
enum Borrowed<'a>{Dictionary(&'a[FieldValue]),DynamicDictionary(&'a DslValue),Tree(&'a DslValue),Gui(&'a DslValue),Answer(&'a DslValue)}
pub(super) fn borrowed<C:NativeSchemaControl>(source:&RecordValue,maximum:usize,c:&mut C)->Result<usize,ValueError>{c.scoped_stage(|c|{c.begin_stage(0)?;let mut r=Rows{control:c,maximum,count:0};let mut pending=Vec::new();r.add(2)?;r.add(list(field(source,3)?)?.len())?;r.add(map(field(source,4)?)?.len())?;
 let widgets=match field(source,2)?{semio_framework_dsl_record::FieldValue::Block(value)=>match value.as_ref(){semio_framework_dsl_record::FieldValue::Statements(values)=>values,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Generation census statements differ"))},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Generation census widget block differs"))};
 for(kind,widget)in widgets{r.add(2)?;match kind.as_str(){
  "neuron"=>{r.add(list(field(widget,3)?)?.len())?;r.add(list(field(widget,4)?)?.len())?;r.push(&mut pending,Borrowed::Dictionary(list(field(widget,5)?)?))?;},
  "output-preview"=>{r.add(list(field(widget,2)?)?.len())?;r.push(&mut pending,Borrowed::Dictionary(list(field(widget,1)?)?))?;},
  "cluster"=>{r.push(&mut pending,Borrowed::Tree(dynamic(field(widget,2)?)?))?;r.push(&mut pending,Borrowed::Gui(dynamic(field(widget,3)?)?))?;},
  "input-slider"|"input-note"|"input-image"|"variable"|"output-action"|"output-export"=>{},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue, "Generation census widget kind differs"))
 }}
 for generation in list(field(source,7)?)?{r.add(1)?;for(_,value)in map(field(record(generation)?,2)?)?{r.add(1)?;r.push(&mut pending,Borrowed::Answer(dynamic(value)?))?;}}
 while let Some(value)=pending.pop(){match value{
  Borrowed::Dictionary(entries)=>{r.add(1)?;for entry in entries{r.add(2)?;let value=record(field(record(entry)?,1)?)?;if let Some(value)=optional(value.fields.get(&5)){r.push(&mut pending,Borrowed::Dictionary(list(value)?))?;}}},
  Borrowed::DynamicDictionary(value)=>{r.add(1)?;for(_,value)in object(value)?{r.add(2)?;if matches!(value,DslValue::Object(_)){r.push(&mut pending,Borrowed::DynamicDictionary(value))?;}}},
  Borrowed::Tree(value)=>{r.add(1)?;r.add(array(member(value,"synapses")?)?.len())?;for neuron in array(member(value,"neurons")?)?{r.add(1)?;r.push(&mut pending,Borrowed::DynamicDictionary(member(neuron,"params")?))?;let child=member(neuron,"tree")?;if !matches!(child,DslValue::Null){r.push(&mut pending,Borrowed::Tree(child))?;}}},
  Borrowed::Gui(value)=>{r.add(1)?;for _ in object(member(value,"nodes")?)?{r.add(2)?;}for preview in array(member(value,"previews")?)?{r.add(1)?;r.add(array(member(preview,"expanded")?)?.len())?;r.push(&mut pending,Borrowed::DynamicDictionary(member(preview,"preview")?))?;}},
  Borrowed::Answer(value)=>{r.add(1)?;match value{semio_framework_value::DslValue::Array(values)=>for value in values{r.add(1)?;r.push(&mut pending,Borrowed::Answer(value))?;},semio_framework_value::DslValue::Object(values)=>for(_,value)in values{r.add(1)?;r.push(&mut pending,Borrowed::Answer(value))?;},_=>{}}}
 }}Ok(r.count)
})}
