//! 🧩️ Original child nodes receive fields independently and retain their actual widget output.
use super::*;
use super::field::{FieldKind,FieldValue,FieldWork};
use super::unknown::UnknownTextWork;
use semio_framework_artifact_flow_flow::{Widget,FlowGui,OrderedSet,neural::{Dictionary,Tree}};
use semio_framework_value::RetainedWorkOutcome;
type Node=semio_s_artifact_stdio_semio_flow::schema::snapshot::FlowNode;
struct Raw<'a>(&'a str);
impl JsonReadSource for Raw<'_>{fn byte_len(&self)->usize{self.0.len()}fn byte_at(&self,index:usize)->Option<u8>{self.0.as_bytes().get(index).copied()}}
enum Input<'a>{Text(&'a str),Raw(&'a str),Number(&'a str),Missing,Unknown}
#[derive(semio_framework_value::RetireOwned)]
pub struct NodeOutput{pub widget:Widget,pub layout_id:String}
#[derive(semio_framework_value::RetireOwned)]
struct Fields{id:Option<String>,text:Option<String>,aux:Option<String>,layout_id:Option<String>,plain:Option<String>,pending_number:Option<Option<f64>>,dictionary:Option<Dictionary>,inputs:Option<Vec<String>>,outputs:Option<Vec<String>>,expanded:Option<OrderedSet>,tree:Option<Tree>,gui:Option<FlowGui>,pending:Option<FieldValue>,output:Option<NodeOutput>}
pub struct NodeWork{fields:ManuallyDrop<Option<Fields>>,field:Option<FieldWork>,plain:Option<flow::FlowTextCopyWork>,number:Option<semio_framework_job::OriginalFloatFieldWork>,unknown:Option<UnknownTextWork>,retirement:RetainedCloneClose,columns:[Option<usize>;18],binding:Option<(usize,usize,usize)>,scan:usize,index:usize,numbers:[f64;4],preview:bool,phase:u8,closing:bool}
impl NodeWork{
 pub fn new()->Self{Self{fields:ManuallyDrop::new(Some(Fields{id:None,text:None,aux:None,layout_id:None,plain:None,pending_number:None,dictionary:None,inputs:None,outputs:None,expanded:None,tree:None,gui:None,pending:None,output:None})),field:None,plain:None,number:None,unknown:None,retirement:Default::default(),columns:[None;18],binding:None,scan:0,index:0,numbers:[0.0,0.0,10.0,0.1],preview:true,phase:0,closing:false}}
 fn paid(cx:&mut StepContext<'_>,copy_bytes:usize)->Result<bool,OriginalTurnFailure>{let d=RetirementDemand{copy_bytes,..fixed()};if cx.should_yield()||cx.is_cancelled()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_bytes:copy_bytes,..unit()})?;cx.consume_fuel(1);Ok(true)}
 fn column(key:&str)->Option<usize>{Some(match key{"neuronKind"=>0,"params"=>1,"inputPorts"=>2,"outputPorts"=>3,"preview"=>4,"value"=>5,"min"=>6,"max"=>7,"step"=>8,"text"=>9,"src"=>10,"name"=>11,"schema"=>12,"action"=>13,"format"=>14,"expanded"=>15,"tree"=>16,"flow"=>17,_=>return None})}
 fn param<'a>(&self,node:&'a Node,key:&str)->Option<&'a str>{Self::column(key).and_then(|i|self.columns[i]).and_then(|i|node.params.get(i)).map(|p|p.value.as_str())}
 fn last(node:&Node)->usize{match node.kind.as_str(){"neuron"|"inputSlider"=>6,"variable"|"outputPreview"=>3,"cluster"=>4,_=>2}}
 fn input<'a>(&self,node:&'a Node)->Result<(FieldKind,Input<'a>),ValueError>{
  if self.index==0||self.index==Self::last(node){return Ok((FieldKind::Text,Input::Text(&node.id)))}
  let text=|value:Option<&'a str>|(FieldKind::Text,value.map_or(Input::Missing,Input::Text));
  let raw=|kind,key|{let source=self.param(node,key);(kind,source.filter(|s|!s.is_empty()).map_or(Input::Missing,Input::Raw))};
  Ok(match(node.kind.as_str(),self.index){
   ("neuron",1)=>text(self.param(node,"neuronKind")),("neuron",2)=>raw(FieldKind::Dictionary,"params"),("neuron",3)=>raw(FieldKind::Ports,"inputPorts"),("neuron",4)=>raw(FieldKind::Ports,"outputPorts"),("neuron",5)=>{let value=self.param(node,"preview").filter(|v|matches!(*v,"true"|"false"));(FieldKind::Boolean,value.map_or(Input::Missing,Input::Raw))},
   ("inputSlider",1)=>(FieldKind::Text,Input::Text(&node.label)),("inputSlider",2..=5)=>{let key=match self.index{2=>"value",3=>"min",4=>"max",_=>"step"};let value=self.param(node,key);(FieldKind::Number,value.map_or(Input::Missing,Input::Number))},
   ("inputNote",1)=>text(self.param(node,"text")),("inputImage",1)=>text(self.param(node,"src")),("variable",1)=>text(self.param(node,"name")),("variable",2)=>text(self.param(node,"schema")),
   ("outputAction",1)=>text(self.param(node,"action")),("outputExport",1)=>text(self.param(node,"format")),("outputPreview",1)=>raw(FieldKind::Dictionary,"preview"),("outputPreview",2)=>raw(FieldKind::Expanded,"expanded"),
   ("cluster",1)=>text(self.param(node,"name")),("cluster",2)=>raw(FieldKind::Tree,"tree"),("cluster",3)=>raw(FieldKind::Gui,"flow"),
   (_,1)=>(FieldKind::Text,Input::Unknown),_=>return Err(invalid()),
  })
 }
 fn fallback(&self,kind:FieldKind)->FieldValue{match kind{FieldKind::Text=>FieldValue::Text(String::new()),FieldKind::Dictionary=>FieldValue::Dictionary(Dictionary::new()),FieldKind::Ports=>FieldValue::Ports(Vec::new()),FieldKind::Boolean=>FieldValue::Boolean(true),FieldKind::Number=>FieldValue::Number(match self.index{4=>10.0,5=>0.1,_=>0.0}),FieldKind::Expanded=>FieldValue::Expanded(OrderedSet::new()),FieldKind::Tree=>FieldValue::Tree(Tree::default()),FieldKind::Gui=>FieldValue::Gui(FlowGui::default())}}
 fn store(&mut self,node:&Node)->Result<(),ValueError>{
  let fields=self.fields.as_mut().ok_or_else(invalid)?;let value=fields.pending.take().ok_or_else(invalid)?;
  match value{
   FieldValue::Text(value)=>{if self.index==0{fields.id=Some(value)}else if self.index==Self::last(node){fields.layout_id=Some(value)}else if node.kind=="variable"&&self.index==2{fields.aux=Some(value)}else{fields.text=Some(value)}},
   FieldValue::Dictionary(value)=>fields.dictionary=Some(value),FieldValue::Ports(value)=>if self.index==3{fields.inputs=Some(value)}else{fields.outputs=Some(value)},
   FieldValue::Boolean(value)=>self.preview=value,FieldValue::Number(value)=>self.numbers[self.index-2]=value,FieldValue::Expanded(value)=>fields.expanded=Some(value),FieldValue::Tree(value)=>fields.tree=Some(value),FieldValue::Gui(value)=>fields.gui=Some(value),
  }Ok(())
 }
 pub fn step(&mut self,node:&Node,cx:&mut StepContext<'_>)->Result<Option<NodeOutput>,OriginalTurnFailure>{
  let binding=(node as*const _ as usize,node.params.as_ptr()as usize,node.params.len());if self.closing||self.binding.is_some_and(|b|b!=binding){return Err(invalid().into())}match self.phase{
   0=>{if self.binding.is_none(){if Self::paid(cx,size_of::<Option<(usize,usize,usize)>>())?{self.binding=Some(binding)}return Ok(None)}if let Some(param)=node.params.get(self.scan){if Self::paid(cx,size_of::<[Option<usize>;18]>()+size_of::<usize>()+param.key.len().min(13))?{if let Some(slot)=Self::column(&param.key){self.columns[slot]=Some(self.scan)}self.scan+=1}}else if Self::paid(cx,size_of::<u8>())?{self.phase=1}},
   1=>{if self.index>Self::last(node){if Self::paid(cx,1)?{self.phase=7}return Ok(None)}let(kind,input)=self.input(node)?;match input{
    Input::Missing=>if Self::paid(cx,size_of::<FieldValue>())?{let value=self.fallback(kind);self.fields.as_mut().unwrap().pending=Some(value);self.phase=6},
    Input::Unknown=>if Self::paid(cx,size_of::<UnknownTextWork>())?{self.unknown=Some(UnknownTextWork::new());self.phase=9},
    Input::Text(_)=>if Self::paid(cx,size_of::<flow::FlowTextCopyWork>())?{self.plain=Some(flow::FlowTextCopyWork::new());self.phase=2},
    Input::Raw(_)=>if Self::paid(cx,size_of::<FieldWork>())?{self.field=Some(FieldWork::new(kind));self.phase=4},
    Input::Number(_)=>if Self::paid(cx,size_of::<semio_framework_job::OriginalFloatFieldWork>())?{self.number=Some(semio_framework_job::OriginalFloatFieldWork::new());self.phase=14},
   }},
   2=>{let(_,Input::Text(text))=self.input(node)?else{return Err(invalid().into())};let plain=self.plain.as_mut().unwrap();if plain.ready(){plain.take_into(&mut self.fields.as_mut().unwrap().plain,cx)?;if self.fields.as_ref().unwrap().plain.is_some(){plain.begin_close();self.phase=12}}else{plain.advance(text,cx)?;}},
   12=>if Self::paid(cx,size_of::<FieldValue>())?{let fields=self.fields.as_mut().unwrap();fields.pending=Some(FieldValue::Text(fields.plain.take().ok_or_else(invalid)?));self.phase=6},
   14=>{let(_,Input::Number(word))=self.input(node)?else{return Err(invalid().into())};if let Some(value)=self.number.as_mut().unwrap().step(word,cx)?{self.fields.as_mut().unwrap().pending_number=Some(value);self.number.as_mut().unwrap().begin_close();self.phase=15}},
   15=>if Self::paid(cx,size_of::<FieldValue>())?{let fallback=match self.index{4=>10.0,5=>0.1,_=>0.0};let fields=self.fields.as_mut().unwrap();fields.pending=Some(FieldValue::Number(fields.pending_number.take().ok_or_else(invalid)?.unwrap_or(fallback)));self.phase=6},
   4=>{let(_,input)=self.input(node)?;let result=match input{Input::Raw(raw)=>self.field.as_mut().unwrap().step(&Raw(raw),cx)?,_=>return Err(invalid().into())};if let Some(result)=result{match result{Ok(value)=>{self.fields.as_mut().unwrap().pending=Some(value);self.phase=6},Err(())=>self.phase=5}self.field.as_mut().unwrap().begin_close()}},
   5=>{let(kind,_)=self.input(node)?;if Self::paid(cx,size_of::<FieldValue>())?{let value=self.fallback(kind);self.fields.as_mut().unwrap().pending=Some(value);self.phase=6}},
   6=>{if let Some(number)=self.number.as_mut(){if !number.terminal_is_empty(){let d=number.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_with_effect(cx,d,|_,effect|number.close_step(effect))?;return Ok(None)}}if let Some(plain)=self.plain.as_mut(){if !plain.terminal_is_empty(){let d=plain.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_with_effect(cx,d,|g,effect|poll_outcome(plain.close_step(g.maximum_copy_bytes,effect)))?;return Ok(None)}}if let Some(field)=self.field.as_mut(){if !field.terminal_is_empty(){let d=field.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_with_effect(cx,d,|g,effect|field.close_step(g,effect))?;return Ok(None)}}if let Some(unknown)=self.unknown.as_mut(){if !unknown.terminal_is_empty(){let d=unknown.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_with_effect(cx,d,|g,effect|unknown.close_step(g,effect))?;return Ok(None)}}if Self::paid(cx,4*size_of::<usize>()+size_of::<FieldValue>())?{self.field=None;self.plain=None;self.number=None;self.unknown=None;self.store(node)?;self.index+=1;self.phase=1}},
   7=>{if Self::paid(cx,size_of::<Widget>()+size_of::<NodeOutput>())?{let f=self.fields.as_mut().unwrap();let id=f.id.take().ok_or_else(invalid)?;let widget=match node.kind.as_str(){
    "neuron"=>Widget::Neuron{id,neuron_kind:f.text.take().ok_or_else(invalid)?,params:f.dictionary.take().ok_or_else(invalid)?,input_ports:f.inputs.take().ok_or_else(invalid)?,output_ports:f.outputs.take().ok_or_else(invalid)?,preview:self.preview},
    "inputSlider"=>Widget::InputSlider{id,label:f.text.take().ok_or_else(invalid)?,value:self.numbers[0],min:self.numbers[1],max:self.numbers[2],step:self.numbers[3]},
    "inputNote"=>Widget::InputNote{id,text:f.text.take().ok_or_else(invalid)?},"inputImage"=>Widget::InputImage{id,src:f.text.take().ok_or_else(invalid)?},
    "variable"=>Widget::Variable{id,name:f.text.take().ok_or_else(invalid)?,schema:f.aux.take().ok_or_else(invalid)?},
    "outputPreview"=>Widget::OutputPreview{id,preview:f.dictionary.take().ok_or_else(invalid)?,expanded:f.expanded.take().ok_or_else(invalid)?},
    "outputAction"=>Widget::OutputAction{id,action:f.text.take().ok_or_else(invalid)?},"outputExport"=>Widget::OutputExport{id,format:f.text.take().ok_or_else(invalid)?},
    "cluster"=>Widget::Cluster{id,name:f.text.take().ok_or_else(invalid)?,tree:f.tree.take().ok_or_else(invalid)?,flow:f.gui.take().ok_or_else(invalid)?},
    _=>Widget::InputNote{id,text:f.text.take().ok_or_else(invalid)?},
   };f.output=Some(NodeOutput{widget,layout_id:f.layout_id.take().ok_or_else(invalid)?});self.phase=8}},
   8=>if Self::paid(cx,size_of::<Option<NodeOutput>>())?{self.phase=10;return Ok(self.fields.as_mut().unwrap().output.take())},
   9=>if let Some(text)=self.unknown.as_mut().unwrap().step(&node.kind,cx)?{self.fields.as_mut().unwrap().pending=Some(FieldValue::Text(text));self.unknown.as_mut().unwrap().begin_close();self.phase=6},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub fn restore_output(&mut self,output:NodeOutput){assert_eq!(self.phase,10);self.fields.as_mut().unwrap().output=Some(output);self.phase=8}
 pub fn begin_close(&mut self){self.closing=true;if let Some(number)=self.number.as_mut(){number.begin_close()}if let Some(field)=self.field.as_mut(){field.begin_close()}if let Some(plain)=self.plain.as_mut(){plain.begin_close()}if let Some(unknown)=self.unknown.as_mut(){unknown.begin_close()}}
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(number)=self.number.as_ref(){return if number.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<usize>(),..fixed()})}else{number.close_demands(copy)}}if let Some(plain)=self.plain.as_ref(){return if plain.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<usize>(),..fixed()})}else{plain.close_demands(copy)}}if let Some(field)=self.field.as_ref(){return if field.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<usize>(),..fixed()})}else{field.close_demands(copy)}}if let Some(unknown)=self.unknown.as_ref(){return if unknown.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<usize>(),..fixed()})}else{unknown.close_demands(copy)}}if !self.retirement.is_empty(){return self.retirement.demands(copy)}self.retirement.next_owner_demands::<Fields>(self.fields.is_some(),&None,copy)}
 pub fn close_step(&mut self,g:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  macro_rules! child{($name:ident,$child:ident=>$close:expr)=>{if let Some($child)=self.$name.as_mut(){if !$child.terminal_is_empty(){return $close}let d=RetirementDemand{copy_bytes:size_of::<usize>(),..fixed()};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.$name=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})),work:1}}}}
  child!(number,c=>c.close_step(effect));child!(plain,c=>poll_outcome(c.close_step(g.maximum_copy_bytes,effect)));child!(field,c=>c.close_step(g,effect));child!(unknown,c=>c.close_step(g,effect));if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))}let turn=self.retirement.begin_granted(&mut self.fields,effect);RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Complete(Default::default()))),work:turn.work}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.number.is_none()&&self.plain.is_none()&&self.field.is_none()&&self.unknown.is_none()&&self.fields.is_none()&&self.retirement.is_empty()}
}
impl Drop for NodeWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original child node fields require terminal source and output custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}

#[cfg(test)]
#[path="🧪️laws/🦀️.rs"]
mod laws;
