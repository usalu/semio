//! 🧩️ Actual owned Flow widgets become native child nodes through funded field and parameter events.
use super::param_json::ParamJsonWork;
use semio_framework_artifact_flow_flow::{Widget,WidgetLayout,FlowGui,OrderedSet,neural::{Dictionary,Tree}};
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowNode,FlowParam};
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_framework_value::{RetireOwned,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}};
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalConstructionDemand,admit_original_construction,close_original_retained_step};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Flow widget publication frontier changed")}
fn fixed()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
fn unit()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
#[derive(RetireOwned)]
enum Input{Text(String),Number(f64),Bool(bool),Label,Dictionary(Dictionary),Ports(Vec<String>),Expanded(OrderedSet),Tree(Tree),Gui(FlowGui)}
enum Json{Dictionary(ParamJsonWork<Dictionary>),Ports(ParamJsonWork<Vec<String>>),Expanded(ParamJsonWork<OrderedSet>),Tree(ParamJsonWork<Tree>),Gui(ParamJsonWork<FlowGui>)}
macro_rules! json{($value:expr,$cursor:ident,$body:expr)=>{match $value{Json::Dictionary($cursor)=>$body,Json::Ports($cursor)=>$body,Json::Expanded($cursor)=>$body,Json::Tree($cursor)=>$body,Json::Gui($cursor)=>$body}}}
#[derive(RetireOwned)]
struct NumberWord{bytes:[u8;512],length:usize}
impl std::fmt::Write for NumberWord{fn write_str(&mut self,text:&str)->std::fmt::Result{let end=self.length.checked_add(text.len()).ok_or(std::fmt::Error)?;if end>self.bytes.len(){return Err(std::fmt::Error)}self.bytes[self.length..end].copy_from_slice(text.as_bytes());self.length=end;Ok(())}}
impl NumberWord{fn text(&self)->&str{std::str::from_utf8(&self.bytes[..self.length]).unwrap()}}
#[derive(RetireOwned)]
struct Fields{id:Option<String>,kind:Option<String>,label:Option<String>,params:Option<Vec<FlowParam>>,inputs:[Option<Input>;5],key:Option<String>,value:Option<String>,number:Option<NumberWord>,output:Option<FlowNode>}
/// 🧱️ The command moves its actual planned widget here before creating native child payloads.
pub(super) struct WidgetNodeWork{fields:ManuallyDrop<Option<Fields>>,text:Option<flow::FlowTextCopyWork>,json:Option<Json>,retirement:RetainedCloneClose,kind:&'static str,keys:&'static[&'static str],position:WidgetLayout,index:usize,phase:u8,text_target:u8,value_mode:u8,origin:Option<(u64,u64)>,closing:bool}
impl WidgetNodeWork{
 pub(super) fn new(widget:Widget,position:WidgetLayout)->Self{
  let mut inputs:[Option<Input>;5]=std::array::from_fn(|_|None);
  let(id,kind,label,keys)=match widget{
   Widget::Neuron{id,neuron_kind,params,input_ports,output_ports,preview}=>{inputs=[Some(Input::Text(neuron_kind)),Some(Input::Dictionary(params)),Some(Input::Ports(input_ports)),Some(Input::Ports(output_ports)),Some(Input::Bool(preview))];(id,"neuron",None,&["neuronKind","params","inputPorts","outputPorts","preview"][..])},
   Widget::InputSlider{id,label,value,min,max,step}=>{inputs=[Some(Input::Label),Some(Input::Number(value)),Some(Input::Number(min)),Some(Input::Number(max)),Some(Input::Number(step))];(id,"inputSlider",Some(label),&["label","value","min","max","step"][..])},
   Widget::InputNote{id,text}=>{inputs[0]=Some(Input::Text(text));(id,"inputNote",None,&["text"][..])},
   Widget::InputImage{id,src}=>{inputs[0]=Some(Input::Text(src));(id,"inputImage",None,&["src"][..])},
   Widget::Variable{id,name,schema}=>{inputs[0]=Some(Input::Text(name));inputs[1]=Some(Input::Text(schema));(id,"variable",None,&["name","schema"][..])},
   Widget::OutputPreview{id,preview,expanded}=>{inputs[0]=Some(Input::Dictionary(preview));inputs[1]=Some(Input::Expanded(expanded));(id,"outputPreview",None,&["preview","expanded"][..])},
   Widget::OutputAction{id,action}=>{inputs[0]=Some(Input::Text(action));(id,"outputAction",None,&["action"][..])},
   Widget::OutputExport{id,format}=>{inputs[0]=Some(Input::Text(format));(id,"outputExport",None,&["format"][..])},
   Widget::Cluster{id,name,tree,flow}=>{inputs[0]=Some(Input::Text(name));inputs[1]=Some(Input::Tree(tree));inputs[2]=Some(Input::Gui(flow));(id,"cluster",None,&["name","tree","flow"][..])},
  };
  Self{fields:ManuallyDrop::new(Some(Fields{id:Some(id),kind:None,label,params:None,inputs,key:None,value:None,number:None,output:None})),text:None,json:None,retirement:Default::default(),kind,keys,position,index:0,phase:0,text_target:0,value_mode:0,origin:None,closing:false}
 }
 fn paid(cx:&mut StepContext<'_>,copy_bytes:usize)->Result<bool,OriginalTurnFailure>{let d=RetirementDemand{copy_bytes,..fixed()};if cx.should_yield()||cx.is_cancelled()||!cx.retained_work_can_enter(d){return Ok(false)}cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_bytes:copy_bytes,..unit()})?;Ok(true)}
 fn text_source(&self)->Result<&str,ValueError>{let fields=self.fields.as_ref().ok_or_else(invalid)?;Ok(match self.text_target{0|1=>self.kind,2=>self.keys[self.index],3=>match self.value_mode{0=>fields.label.as_deref().ok_or_else(invalid)?,1=>"true",2=>"false",3=>fields.number.as_ref().ok_or_else(invalid)?.text(),_=>return Err(invalid())},_=>return Err(invalid())})}
 pub(super) fn step(&mut self,cx:&mut StepContext<'_>)->Result<Option<FlowNode>,OriginalTurnFailure>{for _ in 0..512{let before=(cx.retained_progress(),cx.fuel_remaining());if let Some(output)=self.once(cx)?{return Ok(Some(output))}if cx.should_yield()||cx.is_cancelled()||before==(cx.retained_progress(),cx.fuel_remaining()){break}}Ok(None)}
 fn once(&mut self,cx:&mut StepContext<'_>)->Result<Option<FlowNode>,OriginalTurnFailure>{
  let origin=(cx.operation().0,cx.generation().0);if self.closing||self.origin.is_some_and(|o|o!=origin){return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled(){return Ok(None)}
  if self.origin.is_none(){if Self::paid(cx,size_of::<Option<(u64,u64)>>())?{self.origin=Some(origin)}return Ok(None)}
  match self.phase{
   0=>{let count=self.keys.len();let d=OriginalConstructionDemand{items:1,copy_bytes:2*size_of::<Vec<FlowParam>>(),capacity_bytes:count*size_of::<FlowParam>(),release_bytes:0,depth:1};let mut accepted=None;admit_original_construction(cx,&mut accepted,d,|_|{let result=(||{let mut params=Vec::new();params.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original Flow parameter roster refused"))?;let capacity=params.capacity()*size_of::<FlowParam>();self.fields.as_mut().unwrap().params=Some(params);self.phase=1;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}))})();RetainedWorkOutcome{result,work:1}})?;},
   1=>{if Self::paid(cx,size_of::<flow::FlowTextCopyWork>())?{self.text=Some(flow::FlowTextCopyWork::new());self.text_target=0;self.phase=10}},
   2=>{if self.fields.as_ref().unwrap().label.is_some(){if Self::paid(cx,1)?{self.phase=3}}else if Self::paid(cx,size_of::<flow::FlowTextCopyWork>())?{self.text=Some(flow::FlowTextCopyWork::new());self.text_target=1;self.phase=10}},
   3=>{if self.index==self.keys.len(){if Self::paid(cx,1)?{self.phase=8}}else if Self::paid(cx,size_of::<flow::FlowTextCopyWork>())?{self.text=Some(flow::FlowTextCopyWork::new());self.text_target=2;self.phase=10}},
   4=>{if Self::paid(cx,size_of::<Input>()+size_of::<Json>()+size_of::<NumberWord>())?{let input=self.fields.as_mut().unwrap().inputs[self.index].take().ok_or_else(invalid)?;match input{
    Input::Text(text)=>{self.fields.as_mut().unwrap().value=Some(text);self.phase=7},
    Input::Dictionary(v)=>{self.json=Some(Json::Dictionary(ParamJsonWork::new(v)));self.phase=5},Input::Ports(v)=>{self.json=Some(Json::Ports(ParamJsonWork::new(v)));self.phase=5},Input::Expanded(v)=>{self.json=Some(Json::Expanded(ParamJsonWork::new(v)));self.phase=5},Input::Tree(v)=>{self.json=Some(Json::Tree(ParamJsonWork::new(v)));self.phase=5},Input::Gui(v)=>{self.json=Some(Json::Gui(ParamJsonWork::new(v)));self.phase=5},
    Input::Number(value)=>{use std::fmt::Write;let mut word=NumberWord{bytes:[0;512],length:0};write!(&mut word,"{value}").map_err(|_|invalid())?;self.fields.as_mut().unwrap().number=Some(word);self.value_mode=3;self.phase=6},
    Input::Label=>{self.value_mode=0;self.phase=6},Input::Bool(value)=>{self.value_mode=if value{1}else{2};self.phase=6},
   }}},
   5=>{let json=self.json.as_mut().unwrap();if let Some(value)=json!(json,c,c.step(cx))?{self.fields.as_mut().unwrap().value=Some(value);json!(json,c,c.begin_close());self.phase=13}},
   6=>{if Self::paid(cx,size_of::<flow::FlowTextCopyWork>())?{self.text=Some(flow::FlowTextCopyWork::new());self.text_target=3;self.phase=10}},
   7=>{if Self::paid(cx,size_of::<FlowParam>()+2*size_of::<usize>())?{let fields=self.fields.as_mut().unwrap();fields.params.as_mut().unwrap().push(FlowParam{key:fields.key.take().ok_or_else(invalid)?,value:fields.value.take().ok_or_else(invalid)?});fields.number=None;self.index+=1;self.phase=3}},
   8=>{if Self::paid(cx,size_of::<FlowNode>())?{let fields=self.fields.as_mut().unwrap();fields.output=Some(FlowNode{id:fields.id.take().ok_or_else(invalid)?,kind:fields.kind.take().ok_or_else(invalid)?,label:fields.label.take().ok_or_else(invalid)?,params:fields.params.take().ok_or_else(invalid)?,position:SemioPoint2{x:self.position.x,y:self.position.y}});self.phase=9}},
   9=>{if Self::paid(cx,2*size_of::<FlowNode>())?{self.phase=14;return Ok(self.fields.as_mut().unwrap().output.take())}},
   10=>{let source=self.text_source()?;let text=self.text.as_ref().unwrap();let d=text.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;if !cx.retained_work_can_enter(d){return Ok(None)}let target=self.text_target;let source=match target{0|1=>self.kind,2=>self.keys[self.index],_=>{let fields=self.fields.as_ref().unwrap();match self.value_mode{0=>fields.label.as_deref().unwrap(),1=>"true",2=>"false",_=>fields.number.as_ref().unwrap().text()}}};let text=self.text.as_mut().unwrap();if text.advance(source,cx)?{let fields=self.fields.as_mut().unwrap();let slot=match target{0=>&mut fields.kind,1=>&mut fields.label,2=>&mut fields.key,_=>&mut fields.value};if text.take_into(slot,cx)?{text.begin_close();self.phase=11}}},
   11=>{let text=self.text.as_mut().unwrap();if !text.terminal_is_empty(){let d=text.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|text.close_step(g))?;}else if Self::paid(cx,size_of::<usize>())?{self.text=None;self.phase=match self.text_target{0=>2,1=>3,2=>4,_=>7}}},
   13=>{let json=self.json.as_mut().unwrap();if !json!(json,c,c.terminal_is_empty()){let d=json!(json,c,c.close_demands(cx.retained_grant().maximum_copy_bytes))?;close_original_retained_step(cx,d,|g|json!(json,c,c.close_step(g)))?;}else if Self::paid(cx,size_of::<usize>())?{self.json=None;self.phase=7}},
   _=>return Err(invalid().into()),
  }Ok(None)
 }
 pub(super) fn restore_output(&mut self,output:FlowNode){assert_eq!(self.phase,14);self.fields.as_mut().unwrap().output=Some(output);self.phase=9}
 pub(super) fn begin_close(&mut self){self.closing=true;if let Some(text)=self.text.as_mut(){text.begin_close()}if let Some(json)=self.json.as_mut(){json!(json,c,c.begin_close())}}
 pub(super) fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(text)=self.text.as_ref(){return if text.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<usize>(),..fixed()})}else{text.close_demands(copy)}}if let Some(json)=self.json.as_ref(){return if json!(json,c,c.terminal_is_empty()){Ok(RetirementDemand{copy_bytes:size_of::<usize>(),..fixed()})}else{json!(json,c,c.close_demands(copy))}}if !self.retirement.is_empty(){return self.retirement.demands(copy)}if self.fields.is_some(){return self.retirement.next_owner_demands::<Fields>(true,&None,copy)}Ok(RetirementDemand{copy_bytes:size_of::<Option<(u64,u64)>>(),..fixed()})}
 pub(super) fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}macro_rules! child{($name:ident,$empty:expr,$close:expr)=>{if let Some(cursor)=self.$name.as_mut(){if !($empty)(cursor){return ($close)(cursor,g)}let d=RetirementDemand{copy_bytes:size_of::<usize>(),..fixed()};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.$name=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})),work:1}}}}
  child!(text,|c:&flow::FlowTextCopyWork|c.terminal_is_empty(),|c:&mut flow::FlowTextCopyWork,g|c.close_step(g));child!(json,|j:&Json|json!(j,c,c.terminal_is_empty()),|j:&mut Json,g|json!(j,c,c.close_step(g)));if !self.retirement.is_empty(){return self.retirement.step_granted(g)}if self.fields.is_some(){let turn=self.retirement.begin_granted(&mut self.fields,g);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}let d=RetirementDemand{copy_bytes:size_of::<Option<(u64,u64)>>(),..fixed()};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.origin=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})),work:1}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.fields.is_none()&&self.text.is_none()&&self.json.is_none()&&self.retirement.is_empty()&&self.origin.is_none()}
}
impl Drop for WidgetNodeWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"original Flow widget publication requires terminal input and node custody");unsafe{ManuallyDrop::drop(&mut self.fields)}}}

#[cfg(test)]
#[path="🧪️laws/🦀️.rs"]
mod laws;
