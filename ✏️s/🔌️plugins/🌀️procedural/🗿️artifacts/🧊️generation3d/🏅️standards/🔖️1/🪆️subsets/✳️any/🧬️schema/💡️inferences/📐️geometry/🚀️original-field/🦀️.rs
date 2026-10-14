//! 🚀️ The actual original inference retains source resolution, actual input/job construction, output and dynamic pending custody.
use super::*;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputsConstruction;
use semio_s_artifact_procedural_3d_geometry_compute::original::OriginalFixedOutputJob;
use semio_s_artifact_procedural_3d_geometry_value::Outputs;
use semio_s_artifact_procedural_3d_geometry_value::SelectionValue;
use semio_s_artifact_procedural_3d_geometry_value::SelectionKind;
use semio_s_artifact_procedural_3d_geometry_value::PlaneValue;
use semio_s_artifact_procedural_3d_geometry_value::OriginalWidgetFaultProduction;
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress,retained_clone::ordered_map::{BoundedOrdGrant,RetainedOrderedMapInsertCursor,RetainedOrderedMapInsertGrant,RetainedOrderedMapInsertProgress,RetainedOrderedMapInsertStep},retirement::controlled::ControlledRetirement,list::PagedList};
use std::mem::{ManuallyDrop,size_of};

type Insert=RetainedOrderedMapInsertCursor<String,GeometryValue>;
#[derive(Clone,Copy)]
struct Source{operation:u64,generation:u64,parent:usize,snapshot:usize,catalogue:usize,key:usize,key_bytes:usize,recipient:usize,clock:fn()->Option<u64>,clock_binding:Option<u64>,deadline:u64}
impl PartialEq for Source{fn eq(&self,other:&Self)->bool{self.operation==other.operation&&self.generation==other.generation&&self.parent==other.parent&&self.snapshot==other.snapshot&&self.catalogue==other.catalogue&&self.key==other.key&&self.key_bytes==other.key_bytes&&self.recipient==other.recipient&&std::ptr::fn_addr_eq(self.clock,other.clock)&&self.clock_binding==other.clock_binding&&self.deadline==other.deadline}}
impl Eq for Source{}
semio_framework_value::artifact_retire_leaf!(Source);
#[derive(Default,semio_framework_value::RetireOwned)]
struct Equality{byte:usize,binding:Option<(usize,usize,usize,usize)>}
impl Equality{
 fn step(&mut self,left:&str,right:&str,cx:&mut StepContext<'_>)->Result<Option<bool>,OriginalTurnFailure>{
  let binding=(left.as_ptr()as usize,left.len(),right.as_ptr()as usize,right.len());if self.binding.is_some_and(|source|source!=binding){return Err(invalid("original geometry source comparison changed").into())}if !open(cx){return Ok(None)}self.binding=Some(binding);post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;
  if left.len()!=right.len(){self.binding=None;self.byte=0;return Ok(Some(false))}if self.byte==left.len(){self.binding=None;self.byte=0;return Ok(Some(true))}if left.as_bytes()[self.byte]!=right.as_bytes()[self.byte]{self.binding=None;self.byte=0;return Ok(Some(false))}self.byte+=1;Ok(None)
 }
}
#[derive(Default,semio_framework_value::RetireOwned)]
struct State{completed:usize,phase:u8,widget:usize,category:usize,kind:usize,link:usize,probe:usize,duplicate:usize,port:usize,wire:usize,selected_wire:Option<usize>,parent:usize,output:usize,parent_identity:Option<usize>,parents:PagedList<usize,{usize::MAX}>,equality:Equality,values:Option<Outputs>,value:Option<GeometryValue>,key:String,byte:usize,reserved:bool,text_source:Option<(usize,usize)>,insertion:Option<Insert>,construction:WidgetInputsConstruction,inputs:Option<WidgetInputs>,job:Option<OriginalFixedOutputJob>,fault:Option<OriginalWidgetFaultProduction>}

/// 🧳️ One actual erased pending frame; every retained output stays outside ordinary Drop until donated or paid closed.
pub struct OriginalGeometryPending{source:Source,cyclic:bool,owner:ControlledRetirement<State>,evaluation:ManuallyDrop<Option<WidgetEvaluation>>,output:ManuallyDrop<Option<Arc<WidgetEvaluation>>>,output_close:Option<ControlledRetirement<(Option<WidgetEvaluation>,Option<Arc<WidgetEvaluation>>)>>,cancelled:bool}
impl OriginalGeometryPending{
 fn new(source:Source,cyclic:bool)->Self{let mut state=State::default();state.values=Some(Outputs::default());Self{source,cyclic,owner:ControlledRetirement::new(state).ok().unwrap(),evaluation:ManuallyDrop::new(None),output:ManuallyDrop::new(None),output_close:None,cancelled:false}}
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()&&self.evaluation.is_none()&&self.output.is_none()&&self.output_close.as_ref().is_none_or(ControlledRetirement::terminal_is_empty)}
 fn check(&self,snapshot:&GeometryInput<'_>,key:&str,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{if self.source!=source(snapshot,key,cx)?{return Err(invalid("original geometry pending requires its initiating caller snapshot catalogue and key").into())}Ok(())}
 fn step(&mut self,snapshot:&GeometryInput<'_>,key:&str,parents:&[Arc<WidgetEvaluation>],cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  self.check(snapshot,key,cx)?;if self.cancelled||!open(cx){return Ok(())}
  if self.owner.original().is_none_or(|state|state.phase==20){
   if !self.owner.terminal_is_empty(){close_owner(&mut self.owner,cx)?;return Ok(())}
   if self.output.is_none(){let bytes=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<WidgetEvaluation>();if cx.retained_grant().maximum_capacity_bytes<bytes{return Ok(())}let evaluation=self.evaluation.take().ok_or_else(||invalid("original geometry pending has no produced evaluation donor"))?;*self.output=Some(Arc::new(evaluation));post(cx,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})?}return Ok(())
  }
  let state=self.owner.original_mut().unwrap();let host=&snapshot.snapshot.host_snapshot;
  if state.phase==0{
   let widget=host.widgets.get(state.widget).ok_or_else(||invalid("original geometry widget does not exist"))?;
   if let Some(equal)=state.equality.step(widget_id(widget),key,cx)?{if equal{state.phase=if matches!(widget,Widget::Neuron{..}){24}else if self.cyclic{22}else{21};state.link=0;}else{state.widget+=1;}}return Ok(())
  }
  let widget=host.widgets.get(state.widget).ok_or_else(||invalid("original geometry source widget ordinal changed"))?;
  if state.phase==22{
   if state.fault.is_none(){post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.fault=Some(OriginalWidgetFaultProduction::default());return Ok(())}
   let producer=state.fault.as_mut().unwrap();let(fault,receipt)=producer.step(key,cx.retained_grant()).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;if receipt!=RetainedCloneProgress::default(){post(cx,receipt)?}
   if let Some(fault)=fault{let quality=match widget{Widget::Neuron{..}=>snapshot.catalogue.categories().get(state.category).and_then(|category|category.kinds.get(state.kind)).map_or(Quality::ExactAnalytic,|kind|kind.quality),_=>Quality::ExactAnalytic};*self.evaluation=Some(WidgetEvaluation::faulted(fault,quality));state.phase=20;}
   return Ok(())
  }
  if state.phase==21{if let Some(job)=snapshot.computes.start_original_widget(widget,cx)?{state.job=Some(job);state.phase=19;}return Ok(())}
  let Widget::Neuron{neuron_kind,params,..}=widget else{if state.phase==19{return step_job(state,&mut self.evaluation,cx)}return Err(invalid("original geometry pending source changed its widget variant").into())};
  if state.phase==24{
   let Some(category)=snapshot.catalogue.categories().get(state.category)else{if self.cyclic{post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.phase=22;return Ok(())}return Err(invalid("original geometry neuron has no admitted catalogue kind").into())};let Some(kind)=category.kinds.get(state.kind)else{post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.category+=1;state.kind=0;return Ok(())};
   if let Some(equal)=state.equality.step(neuron_kind,&kind.id,cx)?{if equal{state.phase=if self.cyclic{22}else{1};}else{state.kind+=1;}}return Ok(())
  }
  let kind=snapshot.catalogue.categories().get(state.category).and_then(|category|category.kinds.get(state.kind)).ok_or_else(||invalid("original geometry admitted catalogue ordinal changed"))?;
  match state.phase{
   1=>{
    if state.link==host.synapses.len(){post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;if state.parents.len()!=parents.len(){return Err(invalid("original geometry plan parent values do not match its distinct source order").into())}state.phase=7;state.wire=0;return Ok(())}
    let wire=&host.synapses[state.link];if let Some(equal)=state.equality.step(&wire.to,key,cx)?{if equal{state.probe=0;state.phase=2;}else{state.link+=1;}}
   }
   2=>{
    let wire=&host.synapses[state.link];let port=kind.inputs.get(state.probe).ok_or_else(||invalid("original geometry wire names an unknown input port"))?;
    if let Some(equal)=state.equality.step(&wire.to_port,&port.name,cx)?{if equal{state.probe=0;state.phase=3;}else{state.probe+=1;}}
   }
   3=>{
    let wire=&host.synapses[state.link];if let Some(widget)=host.widgets.get(state.probe){if let Some(equal)=state.equality.step(widget_id(widget),&wire.from,cx)?{if equal{state.duplicate=0;state.phase=4;}else{state.probe+=1;}}}else{post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.link+=1;state.phase=1;}
   }
   4=>{
    if state.duplicate<state.parents.len()&&cx.retained_grant().maximum_depth<state.parents.next_get_depth_demand(state.duplicate).map_err(ValueError::from)?{return Ok(())}
    post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;
    if state.duplicate==state.parents.len(){state.phase=5;}
    else if *state.parents.get(state.duplicate).ok_or_else(||invalid("original parent roster ordinal changed"))?==state.probe{state.link+=1;state.phase=1;}
    else{state.duplicate+=1;}
   }
   5=>{
    let grant=cx.retained_grant();if state.parents.has_reserved_slot(){post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.phase=6;}
    else{let bytes=state.parents.next_allocation_bytes().map_err(ValueError::from)?;let depth=state.parents.next_reserve_depth_demand().map_err(ValueError::from)?;let copy=state.parents.next_reserve_copy_byte_demand().map_err(ValueError::from)?;if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<bytes||grant.maximum_depth<depth{return Ok(())}let receipt=state.parents.reserve_one_funded(grant).map_err(|error|OriginalTurnFailure::debit_original(ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes!=0),retained_capacity_bytes:error.allocated_bytes,..Default::default()}),cx))?;post(cx,receipt)?;}
   }
   6=>{
    if cx.retained_grant().maximum_depth<state.parents.next_push_depth_demand().map_err(ValueError::from)?{return Ok(())}state.parents.push_reserved(state.probe).map_err(|_|invalid("original parent roster reserved slot changed"))?;post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.link+=1;state.phase=1;
   }
   7=>{
    if state.port==kind.inputs.len(){post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.phase=17;return Ok(())}
    if state.wire==host.synapses.len(){post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.phase=if state.selected_wire.is_some(){9}else{11};state.parent=0;return Ok(())}
    let wire=&host.synapses[state.wire];if let Some(equal)=state.equality.step(&wire.to,key,cx)?{if equal{state.phase=8;}else{state.wire+=1;}}
   }
   8=>{
    let wire=&host.synapses[state.wire];let port=&kind.inputs[state.port];if let Some(equal)=state.equality.step(&wire.to_port,&port.name,cx)?{if equal{if state.selected_wire.is_some(){return Err(invalid("original geometry input accepts only one connection").into())}state.selected_wire=Some(state.wire);}state.wire+=1;state.phase=7;}
   }
   9=>{
    if cx.retained_grant().maximum_depth<state.parents.next_get_depth_demand(state.parent).map_err(ValueError::from)?{return Ok(())}
    let wire=&host.synapses[state.selected_wire.unwrap()];let ordinal=*state.parents.get(state.parent).ok_or_else(||invalid("original geometry wire source is missing from its plan parent roster"))?;let parent=host.widgets.get(ordinal).ok_or_else(||invalid("original geometry source parent widget changed"))?;
    if let Some(equal)=state.equality.step(widget_id(parent),&wire.from,cx)?{if equal{let evaluation=parents.get(state.parent).ok_or_else(||invalid("original geometry source parent evaluation is absent"))?;if evaluation.fault.is_some(){return Err(invalid("original geometry input parent evaluation is faulted").into())}state.parent_identity=Some(Arc::as_ptr(evaluation)as usize);state.output=0;state.phase=10;}else{state.parent+=1;}}
   }
   10=>{
    let evaluation=parents.get(state.parent).ok_or_else(||invalid("original geometry source parent evaluation changed"))?;if state.parent_identity!=Some(Arc::as_ptr(evaluation)as usize){return Err(invalid("original geometry source parent owner identity changed").into())}let wire=&host.synapses[state.selected_wire.unwrap()];
    let(port,_)=evaluation.outputs.get_index(state.output).ok_or_else(||invalid("original geometry source parent has no named output"))?;
    if wire.from_port.is_empty(){post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;if evaluation.outputs.len()!=1{return Err(invalid("original geometry implicit source requires exactly one output").into())}state.phase=12;}
    else if let Some(equal)=state.equality.step(port,&wire.from_port,cx)?{if equal{state.phase=12;}else{state.output+=1;}}
   }
   11=>{
    let port=&kind.inputs[state.port];match literal(port,params,cx)?{Literal::Blocked=>return Ok(()),Literal::Missing=>{if port.optional{post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;next_port(state);return Ok(())}return Err(invalid("original geometry required input has no literal or default").into())},Literal::Value(value,receipt)=>{state.value=Some(value);post(cx,receipt)?;state.phase=13;}}
   }
   12=>{
    let evaluation=parents.get(state.parent).ok_or_else(||invalid("original geometry source parent evaluation disappeared"))?;if state.parent_identity!=Some(Arc::as_ptr(evaluation)as usize){return Err(invalid("original geometry source parent owner identity changed").into())}let value=evaluation.outputs.get_index(state.output).ok_or_else(||invalid("original geometry source output ordinal changed"))?.1;let port=&kind.inputs[state.port];if let Some((value,receipt))=copy_value(port,value,cx)?{state.value=Some(value);post(cx,receipt)?;state.phase=13;}
   }
   13=>{let port=&kind.inputs[state.port];if inputs::construction::text_event(&mut state.key,&port.name,&mut state.byte,&mut state.reserved,&mut state.text_source,cx)?{state.phase=14;}}
   14=>{
    let grant=cx.retained_grant();if grant.maximum_copy_bytes<Insert::constructor_copy_bytes()||grant.maximum_capacity_bytes<Insert::constructor_capacity_bytes(){return Ok(())}let map=state.values.take().ok_or_else(||invalid("original geometry input dictionary workspace disappeared"))?;let key=std::mem::take(&mut state.key);let value=state.value.take().ok_or_else(||invalid("original geometry resolved input donor disappeared"))?;
    match Insert::admit(map,key,value,grant){Ok((insertion,receipt))=>{state.insertion=Some(insertion);state.phase=15;post(cx,receipt)?},Err((error,map,key,value))=>{state.values=Some(map);state.key=key;state.value=Some(value);return Err(OriginalTurnFailure::debit_original(error,cx))}}
   }
   15=>{
    let insertion=state.insertion.as_mut().ok_or_else(||invalid("original geometry input insertion disappeared"))?;if insertion.output_ready(){post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;state.values=insertion.take();insertion.begin_close();state.phase=16;}else{let grant=cx.retained_grant();let step=insertion.advance(RetainedOrderedMapInsertGrant{retirement:grant,comparison:BoundedOrdGrant{maximum_items:1,maximum_bytes:1},maximum_moved_items:grant.maximum_items,maximum_moved_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes}).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;let receipt=match step{RetainedOrderedMapInsertStep::Progress(receipt)|RetainedOrderedMapInsertStep::Complete{progress:receipt,..}=>receipt};post(cx,insert_receipt(receipt)?)?;}
   }
   16=>{let insertion=state.insertion.as_mut().ok_or_else(||invalid("original geometry input insertion closure disappeared"))?;let receipt=insertion.close_step(cx.retained_grant()).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?.progress();post(cx,receipt)?;if insertion.terminal_is_empty(){state.insertion.take();next_port(state);}}
   17=>{if let Some(inputs)=WidgetInputs::new(key,kind,&mut state.values,&mut state.construction,cx)?{state.inputs=Some(inputs);state.phase=18;}}
   18=>{if let Some(job)=snapshot.computes.start_original(kind,&mut state.inputs,cx)?{state.job=Some(job);state.phase=19;}}
   19=>return step_job(state,&mut self.evaluation,cx),
   _=>return Err(invalid("original geometry pending phase is invalid").into()),
  }
  Ok(())
 }
}
fn step_job(state:&mut State,evaluation:&mut Option<WidgetEvaluation>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{if let Some(WidgetStep::Done(value))=state.job.as_mut().ok_or_else(||invalid("original geometry pending has no admitted job"))?.step_original(cx)?{*evaluation=Some(value);state.phase=20;}Ok(())}
fn next_port(state:&mut State){state.port+=1;state.wire=0;state.selected_wire=None;state.parent_identity=None;state.parent=0;state.output=0;state.phase=7;}
fn source(snapshot:&GeometryInput<'_>,key:&str,cx:&mut StepContext<'_>)->Result<Source,OriginalTurnFailure>{Ok(Source{operation:cx.operation().0,generation:cx.generation().0,parent:cx.original_source_parent()?.source_identity()?,snapshot:snapshot.snapshot as*const Generation3dSnapshot as usize,catalogue:Arc::as_ptr(snapshot.catalogue)as usize,key:key.as_ptr()as usize,key_bytes:key.len(),recipient:cx.original_recipient_identity(),clock:cx.clock_source(),clock_binding:cx.original_clock_binding(),deadline:cx.original_absolute_deadline_us()})}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn open(cx:&StepContext<'_>)->bool{let grant=cx.retained_grant();!cx.should_yield()&&!cx.is_cancelled()&&grant.maximum_items>0&&grant.maximum_depth>0}
fn post(cx:&mut StepContext<'_>,receipt:RetainedCloneProgress)->Result<(),OriginalTurnFailure>{let result=cx.consume_retained(receipt);cx.consume_fuel(1);result.map_err(|error|error.with_retained_progress(receipt).into())}
fn insert_receipt(receipt:RetainedOrderedMapInsertProgress)->Result<RetainedCloneProgress,ValueError>{receipt.retirement.checked_add(RetainedCloneProgress{copied_items:receipt.moved_items,copied_bytes:receipt.moved_bytes,retained_capacity_bytes:receipt.retained_capacity_bytes,..Default::default()})}
fn close_owner<T:semio_framework_value::retirement::RetireOwned>(owner:&mut ControlledRetirement<T>,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;Ok(())}

fn range(port:&Port,number:f64)->Result<(),ValueError>{if !number.is_finite()||port.min.is_some_and(|minimum|if port.exclusive_min{number<=minimum}else{number<minimum})||port.max.is_some_and(|maximum|number>maximum){return Err(invalid("original geometry input number is outside its declared range"))}Ok(())}
fn component(port:&Port)->Result<SelectionKind,ValueError>{use semio_s_artifact_procedural_3d_model::catalogue::SelectionComponent;match port.selection.as_ref().map(|selection|selection.component){Some(SelectionComponent::Vertex)=>Ok(SelectionKind::Vertex),Some(SelectionComponent::Edge)=>Ok(SelectionKind::Edge),Some(SelectionComponent::Face)=>Ok(SelectionKind::Face),_=>Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original input selection has no fixed component producer"))}}
fn selection_value(port:&Port,id:u64,cx:&StepContext<'_>)->Result<Option<(GeometryValue,RetainedCloneProgress)>,ValueError>{let grant=cx.retained_grant();if grant.maximum_copy_bytes<8||grant.maximum_capacity_bytes<8{return Ok(None)}let component=component(port)?;let mut ids=Vec::new();ids.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original selection id backing reserve failed"))?;ids.push(id);let capacity=ids.capacity()*size_of::<u64>();Ok(Some((GeometryValue::Selection(SelectionValue{component,ids}),RetainedCloneProgress{copied_items:1,copied_bytes:8,retained_capacity_bytes:capacity,..Default::default()})))}
fn copy_value(port:&Port,value:&GeometryValue,cx:&StepContext<'_>)->Result<Option<(GeometryValue,RetainedCloneProgress)>,ValueError>{
 let bytes=match value{GeometryValue::Number(_)|GeometryValue::Integer(_)=>8,GeometryValue::Boolean(_)=>1,GeometryValue::Vector(_)|GeometryValue::Point(_)=>24,GeometryValue::Plane(_)=>48,GeometryValue::Shape(_)=>size_of::<Arc<semio_framework_3d_brep_engine::ShapeValue>>(),GeometryValue::Mesh(_)=>size_of::<Arc<semio_framework_3d::mesh::HalfedgeMesh>>(),GeometryValue::Selection(selection)=>{if port.port_type!=PortType::Selection||selection.component!=component(port)?||selection.ids.len()!=1{return Err(invalid("original selected input requires one matching selection id"))}return selection_value(port,selection.ids[0],cx)},_=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original input payload has no funded fixed-output resolution producer"))};
 if port.list||cx.retained_grant().maximum_copy_bytes<bytes{return if port.list{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original input list payload has no funded resolution producer"))}else{Ok(None)}}
 let value=match(port.port_type,value){
  (PortType::Number|PortType::Length|PortType::Angle,GeometryValue::Number(value))=>{range(port,*value)?;GeometryValue::Number(*value)},
  (PortType::Number|PortType::Length|PortType::Angle,GeometryValue::Integer(value))=>{range(port,*value as f64)?;GeometryValue::Number(*value as f64)},
  (PortType::Integer,GeometryValue::Integer(value))=>{range(port,*value as f64)?;GeometryValue::Integer(*value)},
  (PortType::Boolean,GeometryValue::Boolean(value))=>GeometryValue::Boolean(*value),
  (PortType::Vector,GeometryValue::Vector(value))if value.iter().all(|v|v.is_finite())=>GeometryValue::Vector(*value),
  (PortType::Point,GeometryValue::Point(value))if value.iter().all(|v|v.is_finite())=>GeometryValue::Point(*value),
  (PortType::Plane,GeometryValue::Plane(value))if value.origin.iter().chain(value.normal.iter()).all(|v|v.is_finite())=>GeometryValue::Plane(*value),
  (PortType::Shape,GeometryValue::Shape(value))=>{if port.shape_kinds.as_ref().is_some_and(|kinds|!kinds.contains(&value::shape_kind(value.kind()))){return Err(invalid("original geometry input shape kind is not admitted"))}GeometryValue::Shape(Arc::clone(value))},
  (PortType::Mesh,GeometryValue::Mesh(value))=>GeometryValue::Mesh(Arc::clone(value)),
  _=>return Err(invalid("original geometry connected input has the wrong declared type")),
 };
 Ok(Some((value,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})))
}
enum Literal{Blocked,Missing,Value(GeometryValue,RetainedCloneProgress)}
fn literal(port:&Port,params:&semio_framework_artifact_flow_flow::neural::Dictionary,cx:&StepContext<'_>)->Result<Literal,ValueError>{
 let copy=match port.port_type{PortType::Number|PortType::Length|PortType::Angle|PortType::Integer|PortType::Selection=>8,PortType::Boolean=>1,PortType::Point|PortType::Vector=>24,PortType::Plane=>48,_=>0};let grant=cx.retained_grant();if grant.maximum_copy_bytes<copy||port.port_type==PortType::Selection&&grant.maximum_capacity_bytes<8{return Ok(Literal::Blocked)}
 if port.port_type==PortType::Selection{
  let id=if let Some(stored)=params.get(&port.name){let list=stored.as_dictionary().filter(|list|list.schema()==Some("list")).ok_or_else(||invalid("original selection literal is not a native list"))?;if list.get("1").is_some(){return Err(invalid("original selected input literal requires one id"))}let number=list.get("0").and_then(|item|item.as_dictionary()).filter(|item|item.schema()==Some("number")).and_then(|number|number.get("value")).ok_or_else(||invalid("original selection literal has no numeric id"))?;match number.as_atom().and_then(|atom|atom.as_str()){Some(text)if text.len()<=20=>text.parse().map_err(|_|invalid("original selection literal id is outside its range"))?,Some(_)=>return Err(invalid("original selection literal id exceeds its native extent")),None=>{let number=semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::neural_number(number).ok_or_else(||invalid("original selection literal id is not numeric"))?;if !number.is_finite()||number<0.0||number.fract()!=0.0||number>u64::MAX as f64{return Err(invalid("original selection literal id is outside its range"))}number as u64}}}
  else if let Some(default)=port.default.as_ref(){let ids=default.as_array().filter(|ids|ids.len()==1).ok_or_else(||invalid("original selection default requires one id"))?;let number=ids[0].as_f64().ok_or_else(||invalid("original selection default id is not numeric"))?;if !number.is_finite()||number<0.0||number.fract()!=0.0||number>u64::MAX as f64{return Err(invalid("original selection default id is outside its range"))}number as u64}
  else{return Ok(Literal::Missing)};
  return Ok(selection_value(port,id,cx)?.map_or(Literal::Blocked,|(value,receipt)|Literal::Value(value,receipt)))
 }
 if port.list||!matches!(port.port_type,PortType::Number|PortType::Length|PortType::Angle|PortType::Integer|PortType::Boolean|PortType::Vector|PortType::Point|PortType::Plane){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original input literal type has no allocation-free scalar decoder"))}
 let value=if let Some(stored)=params.get(&port.name){inputs::element_literal(port.port_type,stored).ok_or_else(||invalid("original geometry stored literal has the wrong declared type"))?}else if let Some(default)=port.default.as_ref(){inputs::element_default(port.port_type,default).ok_or_else(||invalid("original geometry input default has the wrong declared type"))?}else{return Ok(Literal::Missing)};
 Ok(copy_value(port,&value,cx)?.map_or(Literal::Blocked,|(value,receipt)|Literal::Value(value,receipt)))
}
impl InferencePending for OriginalGeometryPending{
 fn cancel(&mut self){self.cancelled=true;if let Some(state)=self.owner.original_mut(){if let Some(job)=state.job.as_mut(){job.cancel();}}}
 fn as_any_mut(&mut self)->&mut dyn Any{self}
 fn close_original(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.source.operation!=cx.operation().0||self.source.generation!=cx.generation().0||self.source.parent!=cx.original_source_parent()?.source_identity()?||self.source.recipient!=cx.original_recipient_identity()||!std::ptr::fn_addr_eq(self.source.clock,cx.clock_source())||self.source.clock_binding!=cx.original_clock_binding()||self.source.deadline!=cx.original_absolute_deadline_us(){return Err(invalid("original geometry pending closure requires its initiating caller").into())}
  if !self.owner.terminal_is_empty(){close_owner(&mut self.owner,cx)?;return Ok(false)}
  if self.output_close.is_none()&&(self.evaluation.is_some()||self.output.is_some()){if cx.retirement_should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}post(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;self.output_close=Some(ControlledRetirement::new((self.evaluation.take(),self.output.take())).ok().unwrap());return Ok(false)}
  if let Some(owner)=self.output_close.as_mut(){if !owner.terminal_is_empty(){close_owner(owner,cx)?;return Ok(false)}self.output_close.take();}
  Ok(self.terminal_is_empty())
 }
 fn original_box_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for OriginalGeometryPending{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original geometry pending reached Drop before original payload and output closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.evaluation);ManuallyDrop::drop(&mut self.output)}}}}

/// 🪜️ The actual InferredField entry creates, advances and closes one original dynamic widget owner.
pub fn compute_step(snapshot:&GeometryInput<'_>,key:&String,parents:&[Arc<WidgetEvaluation>],pending:&mut Option<Box<dyn InferencePending>>,cx:&mut StepContext<'_>)->Result<ComputeStep<Arc<WidgetEvaluation>>,InferenceWorkFailure>{
 compute_entry(snapshot,key,parents,pending,cx,false)
}

/// 🔄️ An actual cyclic plan row creates a localized fault under its original caller.
pub fn compute_cyclic_step(snapshot:&GeometryInput<'_>,key:&String,pending:&mut Option<Box<dyn InferencePending>>,cx:&mut StepContext<'_>)->Result<ComputeStep<Arc<WidgetEvaluation>>,InferenceWorkFailure>{compute_entry(snapshot,key,&[],pending,cx,true)}
fn compute_entry(snapshot:&GeometryInput<'_>,key:&String,parents:&[Arc<WidgetEvaluation>],pending:&mut Option<Box<dyn InferencePending>>,cx:&mut StepContext<'_>,cyclic:bool)->Result<ComputeStep<Arc<WidgetEvaluation>>,InferenceWorkFailure>{
 let binding=source(snapshot,key,cx)?;if pending.is_none(){if !open(cx)||cx.retained_grant().maximum_capacity_bytes<size_of::<OriginalGeometryPending>(){return Ok(ComputeStep::Working{fuel_used:0,progress:0.0})}*pending=Some(Box::new(OriginalGeometryPending::new(binding,cyclic)));post(cx,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:size_of::<OriginalGeometryPending>(),..Default::default()})?;return Ok(ComputeStep::Working{fuel_used:1,progress:0.0})}
 let owner=pending.as_mut().and_then(|owner|owner.as_any_mut().downcast_mut::<OriginalGeometryPending>()).ok_or_else(||invalid("original geometry pending slot has a different owner type"))?;owner.check(snapshot,key,cx)?;if owner.cyclic!=cyclic{return Err(invalid("original geometry pending cyclic plan classification changed").into())}let before=cx.fuel_remaining();owner.step(snapshot,key,parents,cx)?;let used=usize::try_from(before.saturating_sub(cx.fuel_remaining())).unwrap_or(usize::MAX);
 if let Some(state)=owner.owner.original_mut(){state.completed=state.completed.checked_add(used).ok_or_else(||invalid("original geometry pending work progress overflow"))?}
 let progress=owner.owner.original().map_or(0.99,|state|state.job.as_ref().map_or(0.8*(1.0-1.0/(state.completed as f32+1.0)),|job|0.9+0.09*job.progress()));
 if owner.owner.terminal_is_empty()&&owner.output.is_some(){let grant=cx.retained_grant();if open(cx)&&grant.maximum_release_bytes>=size_of::<OriginalGeometryPending>(){post(cx,RetainedCloneProgress{copied_items:1,released_bytes:size_of::<OriginalGeometryPending>(),..Default::default()})?;let output=owner.output.take().unwrap();assert!(owner.terminal_is_empty());pending.take();return Ok(ComputeStep::Done{value:output,fuel_used:used+1})}}
 Ok(ComputeStep::Working{fuel_used:used,progress})
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
