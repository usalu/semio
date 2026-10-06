//! 📐️ Neutral flat scene topology produces owned positions under caller scheduling authority.
use super::{Payload,BoardSceneError,BoardSceneRetirement,NormalPortError};
use crate::board::graph;
use geometry::Vec2;
use semio_framework_value::{FromValue,NativeDecodeControl,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use std::mem::ManuallyDrop;
pub use graph::drawing::force::ForceLayoutOptions;

/// ⚙️ Physics parameters and pins belong to the neutral owner.
pub struct BoardForceLayoutOptions{pub parameters:ForceLayoutOptions,pub center:Option<[f64;2]>,pub locked_node_ids:Vec<String>,pub redraw_handles:bool}
impl Default for BoardForceLayoutOptions{fn default()->Self{Self{parameters:ForceLayoutOptions::default(),center:None,locked_node_ids:Vec::new(),redraw_handles:false}}}
impl RetireOwned for BoardForceLayoutOptions{fn retirement(self)->Box<dyn RetirementCursor>{self.locked_node_ids.retirement()}}

/// 🧭️ The four neutral tree growth directions require no owned text allocation.
#[derive(Clone,Copy,Debug)]
pub enum TreeDirection{Downwards,Upwards,Right,Left}
/// 🌳️ Tree spacing, pins and world anchor belong to the neutral layout operation.
pub struct BoardTreeLayoutOptions{pub layer_spacing:f64,pub sibling_gap:f64,pub direction:TreeDirection,pub center:Option<[f64;2]>,pub locked_node_ids:Vec<String>,pub redraw_handles:bool}
impl Default for BoardTreeLayoutOptions{fn default()->Self{Self{layer_spacing:120.0,sibling_gap:28.0,direction:TreeDirection::Downwards,center:None,locked_node_ids:Vec::new(),redraw_handles:false}}}
impl RetireOwned for BoardTreeLayoutOptions{fn retirement(self)->Box<dyn RetirementCursor>{self.locked_node_ids.retirement()}}
/// 🎛️ Algorithm selection is an owned enum, independent of fixture schema or Product metadata.
pub enum BoardLayoutOptions{Force(BoardForceLayoutOptions),Hierarchical(BoardTreeLayoutOptions),SnapHandles}

/// 📍️ A node position has no Product descriptor or external backend field.
#[derive(Debug,semio_framework_value_derive::ToValue)]
pub struct NodePosition{pub id:String,pub x:f64,pub y:f64}
impl RetireOwned for NodePosition{fn retirement(self)->Box<dyn RetirementCursor>{self.id.retirement()}}
/// 🧷️ A handle position refers to the declared flat handle identifier.
#[derive(Debug,semio_framework_value_derive::ToValue)]
pub struct HandlePosition{pub id:String,pub angle:f64}
impl RetireOwned for HandlePosition{fn retirement(self)->Box<dyn RetirementCursor>{self.id.retirement()}}

struct CopyBacking<T:Copy+Send+'static>(Vec<T>);
impl<T:Copy+Send+'static> Default for CopyBacking<T>{fn default()->Self{Self(Vec::new())}}
struct CopyRetirement<T:Copy+Send+'static>{backing:ManuallyDrop<Vec<T>>,remaining:usize,terminal:bool}
impl<T:Copy+Send+'static> RetirementCursor for CopyRetirement<T>{
 fn close_step(&mut self,maximum_bytes:usize)->RetirementStep{if self.remaining!=0{if maximum_bytes==0{return RetirementStep::BudgetExhausted;}let bytes=self.remaining.min(maximum_bytes);self.remaining-=bytes;return RetirementStep::Bytes(bytes);}if !self.terminal{unsafe{ManuallyDrop::drop(&mut self.backing)}self.terminal=true;}RetirementStep::Complete}
 fn terminal_is_empty(&self)->bool{self.terminal}
 fn next_close_byte_demand(&self)->Option<usize>{Some(self.remaining.min(65536))}
}
impl<T:Copy+Send+'static> Drop for CopyRetirement<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal,"layout copy backing must retire");}}
impl<T:Copy+Send+'static> RetireOwned for CopyBacking<T>{fn retirement(self)->Box<dyn RetirementCursor>{let remaining=self.0.capacity()*std::mem::size_of::<T>();Box::new(CopyRetirement{backing:ManuallyDrop::new(self.0),remaining,terminal:false})}}

struct Workspace {
 payload:Payload,options:BoardForceLayoutOptions,indices:Vec<usize>,lookup:Vec<usize>,merge:Vec<usize>,handle_nodes:Vec<Option<usize>>,handle_lookup:Vec<usize>,handle_merge:Vec<usize>,
 positions:CopyBacking<Vec2>,velocities:CopyBacking<Vec2>,forces:CopyBacking<Vec2>,pins:CopyBacking<Option<Vec2>>,radii:Vec<f64>,edges:Vec<(usize,usize)>,edge_merge:Vec<(usize,usize)>,directed_edges:Vec<(usize,usize)>,hierarchy:Option<tree::HierarchyState>,snap_only:bool,
 nodes:Vec<NodePosition>,handles:Vec<HandlePosition>,angles:Vec<f64>,state:graph::drawing::force::ForceLayoutState,phase:u8,index:usize,complete:bool,
}
impl RetireOwned for Workspace{fn retirement(self)->Box<dyn RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.payload,self.options,self.indices,self.lookup,self.merge,self.handle_nodes,self.handle_lookup,self.handle_merge,self.positions,self.velocities,self.forces,self.pins,self.radii,self.edges,self.edge_merge,self.directed_edges,self.hierarchy,self.nodes,self.handles,self.angles)}}

fn refusal(message:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn compare_text(a:&str,b:&str,control:&mut NativeDecodeControl<'_>)->Result<std::cmp::Ordering,ValueError>{let length=a.len().min(b.len());let mut offset=0;while offset<length{control.step()?;let end=(offset+64).min(length);let order=a.as_bytes()[offset..end].cmp(&b.as_bytes()[offset..end]);if order!=std::cmp::Ordering::Equal{return Ok(order);}offset=end;}Ok(a.len().cmp(&b.len()))}

impl Workspace {
 fn new(payload:Payload,options:BoardLayoutOptions)->Self{let(options,hierarchy,snap_only)=match options{BoardLayoutOptions::Force(options)=>(options,None,false),BoardLayoutOptions::Hierarchical(mut tree)=>{let options=BoardForceLayoutOptions{parameters:ForceLayoutOptions::default(),center:tree.center,locked_node_ids:std::mem::take(&mut tree.locked_node_ids),redraw_handles:tree.redraw_handles};(options,Some(tree::HierarchyState::new(tree)),false)},BoardLayoutOptions::SnapHandles=>(BoardForceLayoutOptions{redraw_handles:true,..Default::default()},None,true)};Self{payload,options,indices:Vec::new(),lookup:Vec::new(),merge:Vec::new(),handle_nodes:Vec::new(),handle_lookup:Vec::new(),handle_merge:Vec::new(),positions:CopyBacking::default(),velocities:CopyBacking::default(),forces:CopyBacking::default(),pins:CopyBacking::default(),radii:Vec::new(),edges:Vec::new(),edge_merge:Vec::new(),directed_edges:Vec::new(),hierarchy,snap_only,nodes:Vec::new(),handles:Vec::new(),angles:Vec::new(),state:graph::drawing::force::ForceLayoutState::default(),phase:0,index:0,complete:false}}
 fn node_index(&self,id:&str,control:&mut NativeDecodeControl<'_>)->Result<Option<usize>,ValueError>{let mut low=0;let mut high=self.lookup.len();while low<high{control.step()?;let middle=low+(high-low)/2;let index=self.lookup[middle];match compare_text(&self.payload.nodes[self.indices[index]].id,id,control)?{std::cmp::Ordering::Less=>low=middle+1,std::cmp::Ordering::Greater=>high=middle,std::cmp::Ordering::Equal=>return Ok(Some(index)),}}Ok(None)}
 fn handle_index(&self,id:&str,control:&mut NativeDecodeControl<'_>)->Result<Option<usize>,ValueError>{let mut low=0;let mut high=self.handle_lookup.len();while low<high{control.step()?;let middle=low+(high-low)/2;let index=self.handle_lookup[middle];match compare_text(&self.payload.handles[index].id,id,control)?{std::cmp::Ordering::Less=>low=middle+1,std::cmp::Ordering::Greater=>high=middle,std::cmp::Ordering::Equal=>return Ok(Some(index)),}}Ok(None)}
 fn endpoint(&self,id:&str,control:&mut NativeDecodeControl<'_>)->Result<Option<usize>,ValueError>{if self.payload.mode=="ported"{if let Some(index)=self.handle_index(id,control)?{return Ok(self.handle_nodes[index]);}}self.node_index(id,control)}
 fn snapped_angle(&self,index:usize,toward:usize)->Option<f64>{let node=&self.payload.nodes[self.indices[index]];let from=self.positions.0[index];let to=self.positions.0[toward];if(from-to).hypot()<=1e-9{return None;}let center=geometry::Point::new(from.x,from.y);let target=geometry::Point::new(to.x,to.y);if node.shape.as_deref()==Some("rectangle"){Some(crate::board::rectangle_handle_angle_toward(center,node.width?,node.height?,target))}else{node.radius?;Some(crate::board::circle_handle_angle_toward(center,target))}}
 fn prepare(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
  control.begin_stage(0)?;
  if self.payload.mode!="normal"&&self.payload.mode!="ported"{return Err(refusal("unknown neutral layout mode"));}
  if self.snap_only&&self.payload.mode!="ported"{return Err(refusal("handle snapping requires declared ported topology"));}
  let p=&self.options.parameters;if ![p.ideal_edge_length,p.repulsion_strength,p.spring_strength,p.gravity,p.center_x,p.center_y,p.time_step,p.velocity_damping,p.max_speed,p.barnes_hut_theta].into_iter().all(f64::is_finite)||p.max_speed<0.0||p.time_step<0.0{return Err(refusal("invalid force parameters"));}
  self.indices=control.allocate_vec(self.payload.nodes.len())?;
  for(index,node)in self.payload.nodes.iter().enumerate(){control.step()?;if node.visible!=Some(false){if ![node.x,node.y].into_iter().all(f64::is_finite){return Err(refusal("non-finite layout node center"));}self.indices.push(index);}}
  let n=self.indices.len();self.lookup=control.allocate_vec(n)?;self.merge=control.allocate_vec(n)?;
  for index in 0..n{control.step()?;self.lookup.push(index);self.merge.push(0);}
  let mut width=1;while width<n{let mut start=0;while start<n{let middle=(start+width).min(n);let end=(middle+width).min(n);let(mut left,mut right)=(start,middle);for output in start..end{control.step()?;let take_left=right==end||(left<middle&&compare_text(&self.payload.nodes[self.indices[self.lookup[left]]].id,&self.payload.nodes[self.indices[self.lookup[right]]].id,control)?!=std::cmp::Ordering::Greater);self.merge[output]=if take_left{let value=self.lookup[left];left+=1;value}else{let value=self.lookup[right];right+=1;value};}start=end;}std::mem::swap(&mut self.lookup,&mut self.merge);width=width.saturating_mul(2);}
  for index in 1..n{control.step()?;if compare_text(&self.payload.nodes[self.indices[self.lookup[index-1]]].id,&self.payload.nodes[self.indices[self.lookup[index]]].id,control)?==std::cmp::Ordering::Equal{return Err(refusal("duplicate neutral layout node id"));}}
  self.positions.0=control.allocate_vec(n)?;self.velocities.0=control.allocate_vec(n)?;self.forces.0=control.allocate_vec(n)?;self.pins.0=control.allocate_vec(n)?;self.radii=control.allocate_vec(n)?;self.nodes=control.allocate_vec(n)?;self.handles=control.allocate_vec(self.payload.handles.len())?;
  let mut anchor=Vec2::ZERO;
  for &index in &self.indices{control.step()?;let node=&self.payload.nodes[index];let position=Vec2::new(node.x,node.y);anchor+=position;self.positions.0.push(position);self.velocities.0.push(Vec2::ZERO);self.forces.0.push(Vec2::ZERO);self.pins.0.push(if node.locked==Some(true){Some(position)}else{None});let radius=if node.shape.as_deref()==Some("rectangle"){let width=node.width.unwrap_or(40.0);let height=node.height.unwrap_or(40.0);((width*width+height*height).sqrt()*0.5).max(8.0)}else{node.radius.filter(|radius|radius.is_finite()&&*radius>0.0).unwrap_or(32.0)};self.radii.push(radius);}
  if n!=0{anchor/=n as f64;}
  if let Some([x,y])=self.options.center{if ![x,y].into_iter().all(f64::is_finite){return Err(refusal("non-finite force center"));}self.options.parameters.center_x=x;self.options.parameters.center_y=y;}else{self.options.parameters.center_x=anchor.x;self.options.parameters.center_y=anchor.y;}
  for id in &self.options.locked_node_ids{control.step()?;if let Some(index)=self.node_index(id,control)?{self.pins.0[index]=Some(self.positions.0[index]);}}
  if self.hierarchy.is_none()&&!self.snap_only{let mut seed=self.options.parameters.random_seed;for index in 0..n{control.step()?;graph::drawing::force::seed_position(index,&mut self.positions.0[index],self.pins.0[index],anchor,&mut seed);}}
  self.angles=control.allocate_vec(self.payload.handles.len())?;self.handle_nodes=control.allocate_vec(self.payload.handles.len())?;self.handle_lookup=control.allocate_vec(self.payload.handles.len())?;self.handle_merge=control.allocate_vec(self.payload.handles.len())?;for(index,handle)in self.payload.handles.iter().enumerate(){control.step()?;if !handle.angle.is_finite(){return Err(refusal("non-finite neutral handle angle"));}self.angles.push(handle.angle);self.handle_nodes.push(if handle.visible==Some(false){None}else{self.node_index(&handle.node_id,control)?});if handle.visible!=Some(false){self.handle_lookup.push(index);self.handle_merge.push(0);}}
  let mut width=1;while width<self.handle_lookup.len(){let mut start=0;while start<self.handle_lookup.len(){let middle=(start+width).min(self.handle_lookup.len());let end=(middle+width).min(self.handle_lookup.len());let(mut left,mut right)=(start,middle);for output in start..end{control.step()?;let take_left=right==end||(left<middle&&compare_text(&self.payload.handles[self.handle_lookup[left]].id,&self.payload.handles[self.handle_lookup[right]].id,control)?!=std::cmp::Ordering::Greater);self.handle_merge[output]=if take_left{let value=self.handle_lookup[left];left+=1;value}else{let value=self.handle_lookup[right];right+=1;value};}start=end;}std::mem::swap(&mut self.handle_lookup,&mut self.handle_merge);width=width.saturating_mul(2);}
  for index in 1..self.handle_lookup.len(){control.step()?;if compare_text(&self.payload.handles[self.handle_lookup[index-1]].id,&self.payload.handles[self.handle_lookup[index]].id,control)?==std::cmp::Ordering::Equal{return Err(refusal("duplicate neutral layout handle id"));}}
  self.edges=control.allocate_vec(self.payload.edges.len())?;self.edge_merge=control.allocate_vec(self.payload.edges.len())?;self.directed_edges=control.allocate_vec(self.payload.edges.len())?;
  for edge in &self.payload.edges{control.step()?;if edge.visible==Some(false){continue;}if let(Some(a),Some(b))=(self.endpoint(&edge.source,control)?,self.endpoint(&edge.target,control)?){if a!=b{self.edges.push((a.min(b),a.max(b)));self.directed_edges.push((a,b));}}}
  for _ in 0..self.edges.len(){control.step()?;self.edge_merge.push((0,0));}
  let mut width=1;while width<self.edges.len(){let mut start=0;while start<self.edges.len(){let middle=(start+width).min(self.edges.len());let end=(middle+width).min(self.edges.len());let(mut left,mut right)=(start,middle);for output in start..end{control.step()?;let take_left=right==end||(left<middle&&self.edges[left]<=self.edges[right]);self.edge_merge[output]=if take_left{let value=self.edges[left];left+=1;value}else{let value=self.edges[right];right+=1;value};}start=end;}std::mem::swap(&mut self.edges,&mut self.edge_merge);width=width.saturating_mul(2);}
  let mut unique=0;for index in 0..self.edges.len(){control.step()?;if unique==0||self.edges[index]!=self.edges[unique-1]{self.edges[unique]=self.edges[index];unique+=1;}}self.edges.truncate(unique);if let Some(mut tree)=self.hierarchy.take(){let result=tree.prepare(self,control);self.hierarchy=Some(tree);result?;}control.checkpoint()
 }
 fn advance(&mut self,maximum_units:usize,control:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{
  if maximum_units==0{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"layout advance requires positive work credits"));}
  control.begin_stage(0)?;
  if self.phase==0{let complete=if let Some(mut tree)=self.hierarchy.take(){let result=tree.advance(maximum_units,self,control);self.hierarchy=Some(tree);result?}else if self.snap_only{true}else{self.state.advance(maximum_units,&mut self.positions.0,&self.radii,&self.edges,&self.pins.0,&mut self.velocities.0,&mut self.forces.0,&self.options.parameters,control)?};if !complete{return Ok(false);}self.phase=1;return Ok(false);}
  for _ in 0..maximum_units{control.step()?;match self.phase{1=>{if self.index==self.indices.len(){self.phase=2;self.index=0;continue;}let position=self.positions.0[self.index];let id=control.copy_text(&self.payload.nodes[self.indices[self.index]].id)?;self.nodes.push(NodePosition{id,x:position.x,y:position.y});self.index+=1;},2=>{if !self.options.redraw_handles||self.payload.mode!="ported"||self.index==self.payload.edges.len(){self.phase=3;self.index=0;continue;}let edge=&self.payload.edges[self.index];if edge.visible!=Some(false){if let(Some(a),Some(b))=(self.handle_index(&edge.source,control)?,self.handle_index(&edge.target,control)?){if let(Some(na),Some(nb))=(self.handle_nodes[a],self.handle_nodes[b]){if let Some(angle)=self.snapped_angle(na,nb){self.angles[a]=angle;}if let Some(angle)=self.snapped_angle(nb,na){self.angles[b]=angle;}}}}self.index+=1;},3=>{if self.index==self.payload.handles.len(){self.complete=true;return Ok(true);}let handle=&self.payload.handles[self.index];if handle.visible!=Some(false){self.handles.push(HandlePosition{id:control.copy_text(&handle.id)?,angle:self.angles[self.index]});}self.index+=1;},_=>return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"invalid layout output phase")),}}
  control.checkpoint()?;Ok(false)
 }
}

/// ⏯️ A scene and all partial position buffers survive cancellation until explicitly retired.
pub struct BoardLayoutOperation{owner:ManuallyDrop<Workspace>,consumed:bool}
impl BoardLayoutOperation {
 fn build(payload:Payload,options:BoardLayoutOptions,control:&mut NativeDecodeControl<'_>)->Result<Self,BoardSceneError>{let mut owner=Workspace::new(payload,options);if let Err(cause)=owner.prepare(control){return Err(BoardSceneError::retain(NormalPortError::Scene(cause),owner));}Ok(Self{owner:ManuallyDrop::new(owner),consumed:false})}
 pub fn from_json(json:&str,options:BoardLayoutOptions,control:&mut NativeDecodeControl<'_>)->Result<Self,BoardSceneError>{Self::build(semio_framework_pack_json::from_json_str_controlled(json,semio_framework_pack_json::JsonMemberPolicy::Reject,control)?,options,control)}
 pub fn from_value(value:&semio_framework_value::DslValue,options:BoardLayoutOptions,control:&mut NativeDecodeControl<'_>)->Result<Self,BoardSceneError>{Self::build(Payload::from_value_controlled(value,control)?,options,control)}
 pub fn advance(&mut self,maximum_units:usize,control:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{self.owner.advance(maximum_units,control)}
 fn into_owner(mut self)->Workspace{self.consumed=true;unsafe{ManuallyDrop::take(&mut self.owner)}}
 pub fn finish(self)->Result<BoardLayoutPositions,BoardSceneError>{let owner=self.into_owner();if !owner.complete{return Err(BoardSceneError::retain(NormalPortError::Scene(refusal("layout has not completed")),owner));}Ok(BoardLayoutPositions{owner:ManuallyDrop::new(owner),consumed:false})}
 pub fn retirement(self)->BoardSceneRetirement{BoardSceneRetirement::new(self.into_owner())}
}
impl Drop for BoardLayoutOperation{fn drop(&mut self){assert!(std::thread::panicking()||self.consumed,"layout operation must finish or retire");}}

/// 🫴️ Specific producers apply these neutral positions to their own records by identifier.
pub struct BoardLayoutPositions{owner:ManuallyDrop<Workspace>,consumed:bool}
impl BoardLayoutPositions {
 pub fn nodes(&self)->&[NodePosition]{&self.owner.nodes}
 pub fn handles(&self)->&[HandlePosition]{&self.owner.handles}
 pub fn retirement(mut self)->BoardSceneRetirement{self.consumed=true;BoardSceneRetirement::new(unsafe{ManuallyDrop::take(&mut self.owner)})}
}
impl Drop for BoardLayoutPositions{fn drop(&mut self){assert!(std::thread::panicking()||self.consumed,"layout positions must retire");}}

#[path="🌳️tree/🦀️.rs"]
mod tree;

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
