//! 🪆️ Genuine composed fields assemble one retained execution scene before downstream borrowing.
use super::*;
use super::node::{NodeWork,NodeOutput};
use semio_framework_artifact_flow_flow::{FlowHostSnapshot as HostSnapshot,FlowSourceIdentity,Widget,WidgetLayout,SynapseSpec,CameraJson,FlowSnapshotSource};
use semio_framework_job::{admit_original_construction,OriginalConstructionDemand,close_original_retained_step};
use semio_framework_value::RetainedWorkOutcome;
use semio_framework_job::OriginalSourceBinding;
type Child=semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
#[derive(Clone,Copy,PartialEq,Eq)]
struct SourceBinding{parent_reference:(u64,usize,usize,usize),child_reference:(u64,usize,usize,usize),document:usize,schema:usize,schema_length:usize,child:usize,nodes:usize,node_count:usize,edges:usize,edge_count:usize}
/// 🏞️ A funded immutable scene keeps the same source address through jobs and cache publication.
pub struct FlowComposedScene{original:ManuallyDrop<Option<std::sync::Arc<HostSnapshot>>>}
impl FlowComposedScene{
 pub fn source(&self)->&HostSnapshot{self.original.as_ref().expect("live original composed scene")}
 pub fn birth_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<HostSnapshot>()+2*size_of::<Self>(),capacity_bytes:semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<HostSnapshot>(),..fixed()}}
 fn admit(original:&mut Option<HostSnapshot>,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{let d=Self::birth_demand();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(None),work:0}}let Some(original)=original.take()else{return RetainedWorkOutcome{result:Err(invalid()),work:0}};let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()};RetainedWorkOutcome{result:Ok(Some((Self{original:ManuallyDrop::new(Some(std::sync::Arc::new(original)))},p))),work:1}}
 pub fn alias_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>(),..fixed()}}
 pub fn retain(&self,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Self,RetainedCloneProgress)>>{let d=self.alias_demand();if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(None),work:0}}let Some(original)=self.original.as_ref()else{return RetainedWorkOutcome{result:Err(invalid()),work:0}};RetainedWorkOutcome{result:Ok(Some((Self{original:ManuallyDrop::new(Some(std::sync::Arc::clone(original)))},RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()}))),work:1}}
}
impl semio_framework_value::retirement::RetireOwned for FlowComposedScene{
 fn retirement(mut self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(semio_framework_value::retirement::arc_custody::SharedControlledRetirement::lease(self.original.take().expect("original composed scene retirement")))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::arc_custody::shared_retirement_birth_bytes::<HostSnapshot>())}
 fn controlled_retirement_supported()->bool{true}
}
impl Drop for FlowComposedScene{fn drop(&mut self){assert!(self.original.is_none(),"composed scene must retire its actual Arc");unsafe{ManuallyDrop::drop(&mut self.original)}}}
struct Owners{schema:Option<String>,widgets:Option<Vec<Widget>>,synapses:Option<Vec<SynapseSpec>>,layout:Option<flow::OrderedMap<WidgetLayout>>,node:Option<NodeOutput>,snapshot:Option<HostSnapshot>,scene:Option<FlowComposedScene>}
/// 🪆️ The original command owns every admitted field and the actual stable composed execution scene.
pub struct FlowComposedSourceWork{owners:ManuallyDrop<Owners>,node:Option<NodeWork>,edge:Option<flow::FlowSynapseFieldsWork>,text:Option<flow::FlowTextCopyWork>,entry:Option<flow::FlowLayoutEntryWork>,retirement:RetainedCloneClose,binding:Option<SourceBinding>,local_binding:Option<FlowSourceIdentity>,origin:Option<OriginalSourceBinding>,parent_reference:Option<(u64,usize,usize,usize)>,index:usize,phase:u8,ready:bool,local:bool,closing:bool}
/// 🔎️ A read borrows the actual job-held execution source or the immutable document's genuine local source.
pub enum FlowComposedRead<'a>{Projected(&'a HostSnapshot),Local(crate::schema::snapshot::FlowDocumentRead<'a>)}
impl FlowSnapshotSource for FlowComposedRead<'_>{
 fn schema(&self)->&str{match self{Self::Projected(s)=>s.schema(),Self::Local(s)=>s.schema()}}
 fn camera(&self)->&CameraJson{match self{Self::Projected(s)=>s.camera(),Self::Local(s)=>s.camera()}}
 fn widgets(&self)->&[Widget]{match self{Self::Projected(s)=>s.widgets(),Self::Local(s)=>s.widgets()}}
 fn synapses(&self)->&[SynapseSpec]{match self{Self::Projected(s)=>s.synapses(),Self::Local(s)=>s.synapses()}}
 fn layout(&self)->&flow::OrderedMap<WidgetLayout>{match self{Self::Projected(s)=>s.layout(),Self::Local(s)=>s.layout()}}
}
impl FlowComposedSourceWork{
 pub fn new()->Self{Self{owners:ManuallyDrop::new(Owners{schema:None,widgets:None,synapses:None,layout:Some(flow::OrderedMap::new()),node:None,snapshot:None,scene:None}),node:None,edge:None,text:None,entry:None,retirement:Default::default(),binding:None,local_binding:None,origin:None,parent_reference:None,index:0,phase:0,ready:false,local:false,closing:false}}
 fn paid(cx:&mut StepContext<'_>,copy_bytes:usize)->Result<bool,OriginalTurnFailure>{let d=RetirementDemand{copy_bytes,..fixed()};if cx.should_yield()||cx.is_cancelled()||!retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_bytes,..unit()})?;cx.consume_fuel(1);Ok(true)}
 fn source_binding(parent:semio_framework_value::retained_clone::RetainedCloneRef<'_,FlowSnapshot>,original_child:semio_framework_value::retained_clone::RetainedCloneRef<'_,Child>)->SourceBinding{let document=parent.get();let child=original_child.get();SourceBinding{parent_reference:parent.reference_identity(),child_reference:original_child.reference_identity(),document:document as*const _ as usize,schema:document.schema.as_ptr()as usize,schema_length:document.schema.len(),child:child as*const _ as usize,nodes:child.nodes.as_ptr()as usize,node_count:child.nodes.len(),edges:child.edges.as_ptr()as usize,edge_count:child.edges.len()}}
 fn validate(&self,parent:semio_framework_value::retained_clone::RetainedCloneRef<'_,FlowSnapshot>,child:semio_framework_value::retained_clone::RetainedCloneRef<'_,Child>,cx:&StepContext<'_>)->Result<(),ValueError>{if self.origin.is_some_and(|o|!o.matches(cx))||self.binding.is_some_and(|b|b!=Self::source_binding(parent,child)){return Err(invalid())}Ok(())}
 fn reserve(&mut self,count:usize,widgets:bool,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  let bytes=count.checked_mul(if widgets{size_of::<Widget>()}else{size_of::<SynapseSpec>()}).ok_or_else(invalid)?;let d=OriginalConstructionDemand{items:1,copy_bytes:2*size_of::<usize>(),capacity_bytes:bytes,release_bytes:0,depth:1};let mut accepted=None;
  admit_original_construction(cx,&mut accepted,d,|_|{let result=(||{let actual=if widgets{let mut rows=Vec::new();rows.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original composed widget roster refused"))?;let actual=rows.capacity()*size_of::<Widget>();self.owners.widgets=Some(rows);actual}else{let mut rows=Vec::new();rows.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original composed synapse roster refused"))?;let actual=rows.capacity()*size_of::<SynapseSpec>();self.owners.synapses=Some(rows);actual};self.index=0;self.phase=if widgets{1}else{6};Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:actual,..Default::default()}))})();RetainedWorkOutcome{result,work:1}})?;Ok(())
 }
 /// ⏱️ Only the original context's remaining grant funds each bounded child event.
 pub fn step(&mut self,parent:semio_framework_value::retained_clone::RetainedCloneRef<'_,FlowSnapshot>,child:Option<semio_framework_value::retained_clone::RetainedCloneRef<'_,Child>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  for _ in 0..512{let before=(cx.retained_progress(),cx.fuel_remaining());if self.once(parent,child,cx)?{return Ok(true)}if cx.should_yield()||cx.is_cancelled()||before==(cx.retained_progress(),cx.fuel_remaining()){break}}Ok(false)
 }
 fn once(&mut self,parent:semio_framework_value::retained_clone::RetainedCloneRef<'_,FlowSnapshot>,original_child:Option<semio_framework_value::retained_clone::RetainedCloneRef<'_,Child>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing{return Err(invalid().into())}if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
  if self.parent_reference.is_some_and(|reference|reference!=parent.reference_identity()){return Err(invalid().into())}
  let document=parent.get();let Some(original_child)=original_child else{
   let source=document.read_source()?;let identity=source.source_identity();let origin=OriginalSourceBinding::borrow(cx);if self.origin.is_some_and(|o|!o.matches(cx))||self.local_binding.is_some_and(|s|s!=identity){return Err(invalid().into())}
   if self.ready{return Ok(true)}if Self::paid(cx,size_of::<Option<FlowSourceIdentity>>()+size_of::<Option<OriginalSourceBinding>>()+size_of::<Option<(u64,usize,usize,usize)>>())?{self.origin=Some(origin);self.parent_reference=Some(parent.reference_identity());self.local_binding=Some(identity);self.local=true;self.ready=true}return Ok(self.ready)
  };
  if self.local{return Err(invalid().into())}self.validate(parent,original_child,cx)?;let child=original_child.get();
  if self.binding.is_none(){if Self::paid(cx,size_of::<Option<SourceBinding>>()+size_of::<Option<OriginalSourceBinding>>()+size_of::<Option<(u64,usize,usize,usize)>>())?{self.binding=Some(Self::source_binding(parent,original_child));self.origin=Some(OriginalSourceBinding::borrow(cx));self.parent_reference=Some(parent.reference_identity())}return Ok(false)}
  if self.ready{return Ok(true)}
  match self.phase{
   0=>self.reserve(child.nodes.len(),true,cx)?,
   1=>{if self.index==child.nodes.len(){if Self::paid(cx,size_of::<u8>())?{self.phase=5}return Ok(false)}if Self::paid(cx,size_of::<NodeWork>())?{self.node=Some(NodeWork::new());self.phase=2}},
   2=>if let Some(output)=self.node.as_mut().unwrap().step(&child.nodes[self.index],cx)?{self.owners.node=Some(output);self.node.as_mut().unwrap().begin_close();self.phase=3},
   3=>{let node=self.node.as_mut().unwrap();if !node.terminal_is_empty(){let d=node.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|node.close_step(g))?;return Ok(false)}if Self::paid(cx,size_of::<usize>())?{self.node=None;self.phase=4}},
   4=>{if self.entry.is_none(){if Self::paid(cx,size_of::<flow::FlowLayoutEntryWork>()+size_of::<Widget>())?{let NodeOutput{widget,layout_id}=self.owners.node.take().ok_or_else(invalid)?;self.owners.widgets.as_mut().ok_or_else(invalid)?.push(widget);let position=&child.nodes[self.index].position;self.entry=Some(flow::FlowLayoutEntryWork::new(self.owners.layout.take().ok_or_else(invalid)?,layout_id,WidgetLayout{x:position.x,y:position.y}));}return Ok(false)}
    let entry=self.entry.as_mut().unwrap();let d=entry.next_demands(cx.retained_grant().maximum_copy_bytes)?;if !retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}let turn=entry.advance(cx.retained_grant());cx.consume_fuel(turn.work);match turn.result{Ok(output)=>{if let Some(output)=output{self.owners.layout=Some(output);entry.begin_close();self.phase=8}cx.consume_retained(entry.step_progress())?;},Err(error)=>return Err(OriginalTurnFailure::debit_original(error,cx))}},
   8=>{let entry=self.entry.as_mut().unwrap();if !entry.terminal_is_empty(){let d=entry.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|entry.close_step(g))?;return Ok(false)}if Self::paid(cx,size_of::<usize>()*2)?{self.entry=None;self.index+=1;self.phase=1}},
   5=>self.reserve(child.edges.len(),false,cx)?,
   6=>{if self.index==child.edges.len(){if Self::paid(cx,size_of::<flow::FlowTextCopyWork>())?{self.text=Some(flow::FlowTextCopyWork::new());self.phase=10}return Ok(false)}if Self::paid(cx,size_of::<flow::FlowSynapseFieldsWork>())?{self.edge=Some(flow::FlowSynapseFieldsWork::new());self.phase=7}},
   7=>{let wire=&child.edges[self.index];let fields=[wire.id.as_str(),wire.from.node.as_str(),wire.to.node.as_str(),wire.from.port.as_str(),wire.to.port.as_str()];if let Some(wire)=self.edge.as_mut().unwrap().step(fields,cx)?{self.owners.synapses.as_mut().ok_or_else(invalid)?.push(wire);self.edge.as_mut().unwrap().begin_close();self.phase=9}},
   9=>{let edge=self.edge.as_mut().unwrap();if !edge.terminal_is_empty(){let d=edge.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|edge.close_step(g))?;return Ok(false)}if Self::paid(cx,2*size_of::<usize>())?{self.edge=None;self.index+=1;self.phase=6}},
   10=>{let text=self.text.as_mut().unwrap();if text.ready(){text.take_into(&mut self.owners.schema,cx)?;if self.owners.schema.is_some(){text.begin_close();self.phase=11}}else{text.advance(&document.schema,cx)?;}},
   11=>{let text=self.text.as_mut().unwrap();if !text.terminal_is_empty(){let d=text.close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|text.close_step(g))?;return Ok(false)}if Self::paid(cx,size_of::<usize>())?{self.text=None;self.phase=12}},
   12=>if Self::paid(cx,size_of::<HostSnapshot>())?{self.owners.snapshot=Some(HostSnapshot{schema:self.owners.schema.take().ok_or_else(invalid)?,camera:CameraJson{x:0.0,y:0.0,zoom:1.0},widgets:self.owners.widgets.take().ok_or_else(invalid)?,synapses:self.owners.synapses.take().ok_or_else(invalid)?,layout:self.owners.layout.take().ok_or_else(invalid)?});self.phase=13},
   13=>{let d=FlowComposedScene::birth_demand();let demand=OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:0,depth:d.depth};let grant=cx.retained_grant();let mut accepted=None;admit_original_construction(cx,&mut accepted,demand,|_|{let turn=FlowComposedScene::admit(&mut self.owners.snapshot,grant);RetainedWorkOutcome{result:turn.result.and_then(|output|{let Some((scene,p))=output else{return Err(invalid())};self.owners.scene=Some(scene);self.phase=14;self.ready=true;Ok(((),p))}),work:turn.work}})?;return Ok(self.ready)},
   _=>return Err(invalid().into()),
  }Ok(false)
 }
 pub fn read_source<'a>(&'a self,document:&'a FlowSnapshot)->Result<FlowComposedRead<'a>,ValueError>{
  if !self.ready||self.closing{return Err(invalid())}if self.local{let read=document.read_source()?;if self.local_binding!=Some(read.source_identity()){return Err(invalid())}return Ok(FlowComposedRead::Local(read))}
  if self.binding.is_none_or(|b|b.document!=document as*const _ as usize||b.schema!=document.schema.as_ptr()as usize||b.schema_length!=document.schema.len()){return Err(invalid())}Ok(FlowComposedRead::Projected(self.owners.scene.as_ref().ok_or_else(invalid)?.source()))
 }
 pub fn original_binding(&self)->Option<OriginalSourceBinding>{self.origin}
 pub fn scene_alias_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready||self.closing{return Err(invalid())}self.owners.scene.as_ref().map(FlowComposedScene::alias_demand).ok_or_else(invalid)}
 pub fn retain_scene(&self,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<(FlowComposedScene,RetainedCloneProgress)>>{match self.owners.scene.as_ref(){Some(scene)if self.ready&&!self.closing=>scene.retain(g),_=>RetainedWorkOutcome{result:Err(invalid()),work:0}}}
 pub fn begin_close(&mut self){self.closing=true;if let Some(v)=self.node.as_mut(){v.begin_close()}if let Some(v)=self.edge.as_mut(){v.begin_close()}if let Some(v)=self.entry.as_mut(){v.begin_close()}if let Some(v)=self.text.as_mut(){v.begin_close()}}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  macro_rules! child{($field:ident,$demands:ident)=>{if let Some(child)=self.$field.as_ref(){return if child.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<usize>(),..fixed()})}else{child.$demands(copy)}}}}
  child!(node,close_demands);child!(edge,close_demands);child!(entry,next_close_demands);child!(text,close_demands);if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  macro_rules! owner{($field:ident,$ty:ty)=>{if self.owners.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,copy)}}}
  owner!(scene,FlowComposedScene);owner!(snapshot,HostSnapshot);owner!(node,NodeOutput);owner!(widgets,Vec<Widget>);owner!(synapses,Vec<SynapseSpec>);owner!(layout,flow::OrderedMap<WidgetLayout>);owner!(schema,String);
  Ok(if self.binding.is_some()||self.local_binding.is_some()||self.origin.is_some()||self.parent_reference.is_some(){RetirementDemand{copy_bytes:size_of::<Option<SourceBinding>>()+size_of::<Option<FlowSourceIdentity>>()+size_of::<Option<OriginalSourceBinding>>()+size_of::<Option<(u64,usize,usize,usize)>>(),..fixed()}}else{Default::default()})
 }
 fn close_raw(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
  if !self.closing{return RetainedWorkOutcome{result:Err(invalid()),work:0}}
  macro_rules! child{($field:ident)=>{if let Some(child)=self.$field.as_mut(){if !child.terminal_is_empty(){return child.close_step(g)}let d=RetirementDemand{copy_bytes:size_of::<usize>(),..fixed()};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.$field=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})),work:1}}}}
  child!(node);child!(edge);child!(entry);child!(text);if !self.retirement.is_empty(){return self.retirement.step_granted(g)}
  macro_rules! owner{($field:ident)=>{if self.owners.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.owners.$field,g);return RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}}}
  owner!(scene);owner!(snapshot);owner!(node);owner!(widgets);owner!(synapses);owner!(layout);owner!(schema);
  if self.binding.is_some()||self.local_binding.is_some()||self.origin.is_some()||self.parent_reference.is_some(){let d=RetirementDemand{copy_bytes:size_of::<Option<SourceBinding>>()+size_of::<Option<FlowSourceIdentity>>()+size_of::<Option<OriginalSourceBinding>>()+size_of::<Option<(u64,usize,usize,usize)>>(),..fixed()};if !retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.binding=None;self.local_binding=None;self.origin=None;self.parent_reference=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_bytes:d.copy_bytes,..unit()})),work:1}}RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,OriginalTurnFailure>{
  if self.origin.is_some_and(|binding|!binding.matches(cx)){return Err(invalid().into())}
  semio_framework_job::close_original_granted_step(cx,|g|{let turn=self.close_raw(g);RetainedWorkOutcome{result:turn.result.map(|step|if self.terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{semio_framework_job::InteractiveJobCloseStep::Pending{progress:step.progress()}}),work:turn.work}})
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.node.is_none()&&self.edge.is_none()&&self.entry.is_none()&&self.text.is_none()&&self.retirement.is_empty()&&self.owners.schema.is_none()&&self.owners.widgets.is_none()&&self.owners.synapses.is_none()&&self.owners.layout.is_none()&&self.owners.node.is_none()&&self.owners.snapshot.is_none()&&self.owners.scene.is_none()&&self.binding.is_none()&&self.local_binding.is_none()&&self.origin.is_none()&&self.parent_reference.is_none()}
}
impl Drop for FlowComposedSourceWork{fn drop(&mut self){assert!(self.terminal_is_empty(),"composed source must close every original field and stable scene");unsafe{ManuallyDrop::drop(&mut self.owners)}}}

#[cfg(test)]
#[path="🧪️scene.rs"]
mod scene_laws;
