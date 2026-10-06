//! 🌳️ Flat neutral topology advances an iterative Buchheim forest without recursive allocation.
use super::{Workspace,BoardTreeLayoutOptions,TreeDirection,refusal};
use semio_framework_value::{NativeDecodeControl,ValueError};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
const NONE:usize=usize::MAX;

#[derive(Clone,Copy)]
struct Node{parent:usize,first:usize,last:usize,next:usize,previous:usize,thread:usize,ancestor:usize,number:usize,x:f64,y:f64,modifier:f64,change:f64,shift:f64}
impl Default for Node{fn default()->Self{Self{parent:NONE,first:NONE,last:NONE,next:NONE,previous:NONE,thread:NONE,ancestor:NONE,number:0,x:-1.0,y:0.0,modifier:0.0,change:0.0,shift:0.0}}}
#[derive(Clone,Copy)]
struct Frame{node:usize,child:usize,ancestor:usize,phase:u8,shift:f64,change:f64,modifier:f64,depth:usize}
impl Frame{fn new(node:usize)->Self{Self{node,child:NONE,ancestor:NONE,phase:0,shift:0.0,change:0.0,modifier:0.0,depth:0}}}
#[derive(Clone,Copy)]
struct Contour{v:usize,ancestor:usize,vil:usize,vir:usize,vol:usize,vor:usize,sil:f64,sir:f64,sol:f64,sor:f64}
semio_framework_value::artifact_retire_leaf!(Node,Frame);

pub(super) struct HierarchyState{options:BoardTreeLayoutOptions,nodes:Vec<Node>,frames:Vec<Frame>,offsets:Vec<usize>,adjacency:Vec<usize>,queue:Vec<usize>,depth:Vec<usize>,roots:Vec<bool>,parents:Vec<usize>,rank:Vec<usize>,incoming:Vec<usize>,cursor:Vec<usize>,contour:Option<Contour>,phase:u8,index:usize,edge:usize,queue_read:usize,min_x:f64,min_y:f64,max_x:f64,max_y:f64,along_scale:f64,complete:bool}
impl RetireOwned for HierarchyState{fn retirement(self)->Box<dyn RetirementCursor>{semio_framework_value::artifact_retirement_sequence!(self.options,self.nodes,self.frames,self.offsets,self.adjacency,self.queue,self.depth,self.roots,self.parents,self.rank,self.incoming,self.cursor)}}
impl HierarchyState{
 pub(super) fn new(options:BoardTreeLayoutOptions)->Self{Self{options,nodes:Vec::new(),frames:Vec::new(),offsets:Vec::new(),adjacency:Vec::new(),queue:Vec::new(),depth:Vec::new(),roots:Vec::new(),parents:Vec::new(),rank:Vec::new(),incoming:Vec::new(),cursor:Vec::new(),contour:None,phase:0,index:0,edge:0,queue_read:0,min_x:f64::INFINITY,min_y:f64::INFINITY,max_x:f64::NEG_INFINITY,max_y:f64::NEG_INFINITY,along_scale:0.0,complete:false}}
 pub(super) fn prepare(&mut self,owner:&Workspace,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
  let n=owner.indices.len();if ![self.options.layer_spacing,self.options.sibling_gap].into_iter().all(f64::is_finite){return Err(refusal("invalid tree spacing"));}
  self.nodes=control.allocate_vec(n+1)?;self.frames=control.allocate_vec(n+1)?;self.offsets=control.allocate_vec(n+1)?;self.adjacency=control.allocate_vec(owner.directed_edges.len())?;self.queue=control.allocate_vec(n)?;self.depth=control.allocate_vec(n)?;self.roots=control.allocate_vec(n)?;self.parents=control.allocate_vec(n)?;self.rank=control.allocate_vec(n)?;
  for _ in 0..n{control.step()?;self.nodes.push(Node::default());self.offsets.push(0);self.depth.push(NONE);self.roots.push(false);self.parents.push(n);self.rank.push(0);}self.nodes.push(Node::default());self.offsets.push(0);
  for(rank,&index)in owner.lookup.iter().enumerate(){control.step()?;self.rank[index]=rank;}
  self.incoming=control.allocate_vec::<usize>(n)?;for _ in 0..n{control.step()?;self.incoming.push(0);}for &(source,target)in &owner.directed_edges{control.step()?;self.offsets[source+1]+=1;self.incoming[target]+=1;}
  for index in 1..=n{control.step()?;self.offsets[index]+=self.offsets[index-1];}
  self.cursor=control.allocate_vec::<usize>(n)?;for &offset in &self.offsets[..n]{control.step()?;self.cursor.push(offset);}for _ in 0..owner.directed_edges.len(){control.step()?;self.adjacency.push(0);}for &(source,target)in &owner.directed_edges{control.step()?;self.adjacency[self.cursor[source]]=target;self.cursor[source]+=1;}
  let mut explicit=false;for(index,&raw)in owner.indices.iter().enumerate(){control.step()?;self.roots[index]=owner.payload.nodes[raw].root==Some(true);explicit|=self.roots[index];}if !explicit{for(index,&count)in self.incoming.iter().enumerate(){control.step()?;self.roots[index]=count==0;}}
  let mut has_root=false;for root in &self.roots{control.step()?;has_root|=*root;}if !has_root{for root in &mut self.roots{control.step()?;*root=true;}}
  for &index in &owner.lookup{control.step()?;if self.roots[index]{self.depth[index]=0;self.queue.push(index);}}
  let mut extent=0.0;for &raw in &owner.indices{control.step()?;extent+=half_extent(&owner.payload.nodes[raw]);}self.along_scale=(self.options.sibling_gap+2.0*extent/n.max(1)as f64).max(8.0);
  control.checkpoint()
 }
 fn left(&self,index:usize)->usize{let node=self.nodes[index];if node.thread!=NONE{node.thread}else{node.first}}
 fn right(&self,index:usize)->usize{let node=self.nodes[index];if node.thread!=NONE{node.thread}else{node.last}}
 fn finish_contour(&mut self){let mut contour=self.contour.take().unwrap();let right=self.right(contour.vil);if right!=NONE&&self.right(contour.vor)==NONE{self.nodes[contour.vor].thread=right;self.nodes[contour.vor].modifier+=contour.sil-contour.sor;}else if self.left(contour.vir)!=NONE&&self.left(contour.vol)==NONE{self.nodes[contour.vol].thread=self.left(contour.vir);self.nodes[contour.vol].modifier+=contour.sir-contour.sol;contour.ancestor=contour.v;}let frame=self.frames.last_mut().unwrap();frame.ancestor=contour.ancestor;frame.child=self.nodes[contour.v].next;frame.phase=1;}
 fn first_unit(&mut self){
  if let Some(mut contour)=self.contour{let vil=self.right(contour.vil);let vir=self.left(contour.vir);if vil==NONE||vir==NONE{self.finish_contour();return;}contour.vil=vil;contour.vir=vir;let vol=self.left(contour.vol);let vor=self.right(contour.vor);if vol==NONE||vor==NONE{self.contour=Some(contour);self.finish_contour();return;}contour.vol=vol;contour.vor=vor;self.nodes[vor].ancestor=contour.v;let shift=(self.nodes[vil].x+contour.sil)-(self.nodes[vir].x+contour.sir)+1.0;if shift>0.0{let candidate=self.nodes[vil].ancestor;let ancestor=if candidate!=NONE&&self.nodes[candidate].parent==self.nodes[contour.v].parent{candidate}else{contour.ancestor};let subtrees=self.nodes[contour.v].number.saturating_sub(self.nodes[ancestor].number)as f64;if subtrees>0.0{self.nodes[contour.v].change-=shift/subtrees;self.nodes[contour.v].shift+=shift;self.nodes[ancestor].change+=shift/subtrees;self.nodes[contour.v].x+=shift;self.nodes[contour.v].modifier+=shift;}contour.sir+=shift;contour.sor+=shift;}contour.sil+=self.nodes[vil].modifier;contour.sir+=self.nodes[vir].modifier;contour.sol+=self.nodes[vol].modifier;contour.sor+=self.nodes[vor].modifier;self.contour=Some(contour);return;}
  let frame=*self.frames.last().unwrap();let v=frame.node;match frame.phase{
   0=>{if self.nodes[v].first==NONE{self.nodes[v].x=if self.nodes[v].previous!=NONE{self.nodes[self.nodes[v].previous].x+1.0}else{0.0};self.frames.pop();}else{let frame=self.frames.last_mut().unwrap();frame.child=self.nodes[v].first;frame.ancestor=frame.child;frame.phase=1;}},
   1=>{if frame.child==NONE{let frame=self.frames.last_mut().unwrap();frame.child=self.nodes[v].last;frame.phase=3;}else{self.frames.last_mut().unwrap().phase=2;self.frames.push(Frame::new(frame.child));}},
   2=>{let child=frame.child;let previous=self.nodes[child].previous;let first=self.nodes[v].first;if previous==NONE||first==child{self.frames.last_mut().unwrap().child=self.nodes[child].next;self.frames.last_mut().unwrap().phase=1;}else{self.contour=Some(Contour{v:child,ancestor:frame.ancestor,vil:previous,vir:child,vol:first,vor:child,sil:self.nodes[previous].modifier,sir:self.nodes[child].modifier,sol:self.nodes[first].modifier,sor:self.nodes[child].modifier});}},
   3=>{if frame.child!=NONE{let child=frame.child;self.nodes[child].x+=frame.shift;self.nodes[child].modifier+=frame.shift;let frame=self.frames.last_mut().unwrap();frame.change+=self.nodes[child].change;frame.shift+=self.nodes[child].shift+frame.change;frame.child=self.nodes[child].previous;}else{let midpoint=(self.nodes[self.nodes[v].first].x+self.nodes[self.nodes[v].last].x)*0.5;if self.nodes[v].previous!=NONE{self.nodes[v].x=self.nodes[self.nodes[v].previous].x+1.0;self.nodes[v].modifier=self.nodes[v].x-midpoint;}else{self.nodes[v].x=midpoint;}self.frames.pop();}},
   _=>unreachable!(),
  }
 }
 fn second_unit(&mut self){let frame=*self.frames.last().unwrap();let node=frame.node;if frame.phase==0{self.nodes[node].x+=frame.modifier;self.nodes[node].y=frame.depth as f64;self.min_x=self.min_x.min(self.nodes[node].x);let frame=self.frames.last_mut().unwrap();frame.child=self.nodes[node].first;frame.phase=1;}else if frame.child!=NONE{self.frames.last_mut().unwrap().child=self.nodes[frame.child].next;let mut child=Frame::new(frame.child);child.modifier=frame.modifier+self.nodes[node].modifier;child.depth=frame.depth+1;self.frames.push(child);}else{self.frames.pop();}}
 pub(super) fn advance(&mut self,maximum_units:usize,owner:&mut Workspace,control:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{
  let n=owner.indices.len();for _ in 0..maximum_units{if self.complete{return Ok(true);}control.step()?;match self.phase{
   0=>{if self.queue_read==self.queue.len(){if self.index==n{self.phase=1;self.index=0;continue;}let index=owner.lookup[self.index];self.index+=1;if self.depth[index]==NONE{self.roots[index]=true;self.depth[index]=0;self.queue.push(index);}continue;}let source=self.queue[self.queue_read];let start=self.offsets[source];let end=self.offsets[source+1];if start+self.edge==end{self.queue_read+=1;self.edge=0;continue;}let target=self.adjacency[start+self.edge];self.edge+=1;if self.roots[target]{continue;}let depth=self.depth[source]+1;if self.depth[target]==NONE{self.depth[target]=depth;self.parents[target]=source;self.queue.push(target);}else if self.depth[target]==depth&&self.rank[source]<self.rank[self.parents[target]]{self.parents[target]=source;}},
   1=>{if self.index==n{self.frames.push(Frame::new(n));self.phase=2;continue;}let index=owner.lookup[self.index];let parent=self.parents[index];let previous=self.nodes[parent].last;self.nodes[index].parent=parent;self.nodes[index].previous=previous;self.nodes[index].ancestor=index;self.nodes[index].number=if previous==NONE{1}else{self.nodes[previous].number+1};if previous==NONE{self.nodes[parent].first=index;}else{self.nodes[previous].next=index;}self.nodes[parent].last=index;self.index+=1;},
   2=>{if self.frames.is_empty(){self.frames.push(Frame::new(n));self.phase=3;continue;}self.first_unit();},
   3=>{if self.frames.is_empty(){self.phase=4;self.index=0;continue;}self.second_unit();},
   4=>{if self.index==n{self.min_x=f64::INFINITY;self.phase=5;self.index=0;continue;}if self.min_x.is_finite()&&self.min_x<0.0{self.nodes[self.index].x-=self.min_x;}self.index+=1;},
   5=>{if self.index==n{self.phase=6;self.index=0;continue;}let node=self.nodes[self.index];let along=node.x*self.along_scale;let orthogonal=node.y*self.options.layer_spacing;let(x,y)=match self.options.direction{TreeDirection::Downwards=>(along,orthogonal),TreeDirection::Upwards=>(along,-orthogonal),TreeDirection::Right=>(orthogonal,along),TreeDirection::Left=>(-orthogonal,along)};owner.positions.0[self.index]=geometry::Vec2::new(x,y);let extent=half_extent(&owner.payload.nodes[owner.indices[self.index]]);self.min_x=self.min_x.min(x-extent);self.max_x=self.max_x.max(x+extent);self.min_y=self.min_y.min(y-extent);self.max_y=self.max_y.max(y+extent);self.index+=1;},
   6=>{if self.index==n{self.complete=true;return Ok(true);}let center=self.options.center.unwrap_or([0.0,0.0]);let dx=center[0]-(self.min_x+self.max_x)*0.5;let dy=center[1]-(self.min_y+self.max_y)*0.5;if let Some(pin)=owner.pins.0[self.index]{owner.positions.0[self.index]=pin;}else{owner.positions.0[self.index].x+=dx;owner.positions.0[self.index].y+=dy;}self.index+=1;},
   _=>return Err(refusal("invalid tree phase")),
  }}control.checkpoint()?;Ok(self.complete)
 }
}
fn half_extent(node:&crate::board::NodeRecord)->f64{if node.shape.as_deref()==Some("rectangle"){(node.width.unwrap_or(40.0).max(node.height.unwrap_or(40.0))*0.5).max(8.0)}else{node.radius.filter(|radius|radius.is_finite()&&*radius>0.0).unwrap_or(ui_styling::radii::NODE_DEFAULT)}}
