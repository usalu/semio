//! 🌐️ Original splice rows overlay only addressed native graph records on one immutable scene base.
use semio_framework_artifact_flow_flow::{FlowHostSnapshot,FlowSnapshotSource,FlowSourceIdentity,Widget,SynapseSpec};
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"gumball graph changed its actual splice row or indexed source")}
fn slice_identity<T>(source:&[T])->(usize,usize){(source.as_ptr()as usize,source.len())}
#[derive(Clone,Copy,PartialEq,Eq)]
pub struct GumballGraphIdentity{base:FlowSourceIdentity,rows:(usize,usize),created:(usize,usize),updated:(usize,usize),rewired:(usize,usize),links:(usize,usize)}
#[derive(Clone,Copy)]
pub struct GumballGraphRead<'a>{base:&'a FlowHostSnapshot,rows:&'a[Generation3dMutation],created:&'a[usize],updated:&'a[Option<usize>],rewired:&'a[Option<usize>],links:&'a[usize]}
impl<'a> GumballGraphRead<'a>{
 /// 🧷️ Borrows exact producer indices while their native row owners remain immutable.
 pub fn new(base:&'a FlowHostSnapshot,rows:&'a[Generation3dMutation],created:&'a[usize],updated:&'a[Option<usize>],rewired:&'a[Option<usize>],links:&'a[usize])->Result<Self,ValueError>{
  if base.widgets.len().checked_add(created.len())!=Some(updated.len())||rewired.len()!=base.synapses.len()||base.synapses.len().checked_add(links.len()).is_none(){return Err(invalid())}Ok(Self{base,rows,created,updated,rewired,links})
 }
 pub fn identity(self)->GumballGraphIdentity{GumballGraphIdentity{base:self.base.source_identity(),rows:slice_identity(self.rows),created:slice_identity(self.created),updated:slice_identity(self.updated),rewired:slice_identity(self.rewired),links:slice_identity(self.links)}}
 pub fn widget_count(self)->usize{self.updated.len()}
 pub fn synapse_count(self)->usize{self.base.synapses.len()+self.links.len()}
 /// 🌿️ Returns an actual base, creation or preview-update record without cloning its native fields.
 pub fn widget(self,index:usize)->Result<Option<&'a Widget>,ValueError>{
  let Some(update)=self.updated.get(index)else{return Ok(None)};
  if let Some(row)=update{return match self.rows.get(*row){Some(Generation3dMutation::UpdateWidget(change))=>Ok(Some(&change.widget)),_=>Err(invalid())}}
  if index<self.base.widgets.len(){return Ok(self.base.widgets.get(index))}
  let row=self.created.get(index-self.base.widgets.len()).ok_or_else(invalid)?;match self.rows.get(*row){Some(Generation3dMutation::CreateWidget(create))=>Ok(Some(&create.widget)),_=>Err(invalid())}
 }
 /// 🔗️ Returns the exact rewired or appended native edge through constant-size index metadata.
 pub fn synapse(self,index:usize)->Result<Option<&'a SynapseSpec>,ValueError>{
  if index<self.base.synapses.len(){return match self.rewired.get(index).ok_or_else(invalid)?{None=>Ok(self.base.synapses.get(index)),Some(row)=>match self.rows.get(*row){Some(Generation3dMutation::UpdateSynapse(change))=>Ok(Some(&change.synapse)),_=>Err(invalid())}}}
  let Some(row)=self.links.get(index-self.base.synapses.len())else{return Ok(None)};match self.rows.get(*row){Some(Generation3dMutation::ConnectSynapse(connect))=>Ok(Some(&connect.synapse)),_=>Err(invalid())}
 }
}
#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
