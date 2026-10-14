//! ⚠️ Original localized widget faults reserve and copy their real UTF8 payload before evaluation donation.
use super::{WidgetFault,Localized};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,retirement::{RetireOwned,RetirementCursor}};
use std::mem::ManuallyDrop;
const CODE:&str="generation3d.geometry.cycle";
const EN:[&str;2]=["Widget “","” is part of a wiring cycle or depends on one."];
const DE:[&str;2]=["Widget “","” ist Teil eines Verbindungszyklus oder hängt von einem ab."];
#[derive(semio_framework_value::RetireOwned)]
struct State{code:String,en:String,de:String,phase:u8,segment:usize,byte:usize,subject:Option<(usize,usize)>}

/// 🧵️ Partial fault text remains protected until donated once or explicitly retired by the original caller.
pub struct OriginalWidgetFaultProduction{state:ManuallyDrop<State>,transferred:bool}
impl Default for OriginalWidgetFaultProduction{fn default()->Self{Self{state:ManuallyDrop::new(State{code:String::new(),en:String::new(),de:String::new(),phase:0,segment:0,byte:0,subject:None}),transferred:false}}}
impl OriginalWidgetFaultProduction{
 pub fn progress(&self)->f32{self.state.phase.min(7)as f32/7.0}
 pub fn terminal_is_empty(&self)->bool{self.transferred||self.state.phase==7&&self.state.code.capacity()==0&&self.state.en.capacity()==0&&self.state.de.capacity()==0}
 pub fn demand(&self,subject:&str)->Result<RetirementDemand,ValueError>{let state=&self.state;let capacity=match state.phase{0=>CODE.len(),1=>EN[0].len().checked_add(subject.len()).and_then(|bytes|bytes.checked_add(EN[1].len())).ok_or_else(overflow)?,2=>DE[0].len().checked_add(subject.len()).and_then(|bytes|bytes.checked_add(DE[1].len())).ok_or_else(overflow)?,_=>0};let copy=if(3..=5).contains(&state.phase){let source=self.segment(subject);source.get(state.byte..).and_then(|tail|tail.chars().next()).map_or(0,char::len_utf8)}else{0};Ok(RetirementDemand{capacity_bytes:capacity,copy_bytes:copy,depth:1,..Default::default()})}
 fn segment<'a>(&self,subject:&'a str)->&'a str{match self.state.phase{3=>CODE,4=>[EN[0],subject,EN[1]][self.state.segment],5=>[DE[0],subject,DE[1]][self.state.segment],_=>""}}
 pub fn step(&mut self,subject:&str,grant:RetainedCloneGrant)->Result<(Option<WidgetFault>,RetainedCloneProgress),ValueError>{
  let identity=(subject.as_ptr()as usize,subject.len());if self.state.subject.is_some_and(|original|original!=identity){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original widget fault subject changed"))}if self.state.phase==7||grant.maximum_items==0{return Ok((None,Default::default()))}let demand=self.demand(subject)?;if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_depth<demand.depth{return Ok((None,Default::default()))}
  self.state.subject=Some(identity);let mut receipt=RetainedCloneProgress{copied_items:1,..Default::default()};
  if self.state.phase<3{let phase=self.state.phase;let target=match phase{0=>&mut self.state.code,1=>&mut self.state.en,_=>&mut self.state.de};target.try_reserve_exact(demand.capacity_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original localized widget fault backing reserve failed"))?;receipt.retained_capacity_bytes=target.capacity();self.state.phase+=1;return Ok((None,receipt))}
  if self.state.phase<6{let source=self.segment(subject);if let Some(character)=source.get(self.state.byte..).and_then(|tail|tail.chars().next()){let target=match self.state.phase{3=>&mut self.state.code,4=>&mut self.state.en,_=>&mut self.state.de};target.push(character);self.state.byte+=character.len_utf8();receipt.copied_bytes=character.len_utf8();}else{self.state.byte=0;self.state.segment+=1;if self.state.phase==3||self.state.segment==3{self.state.segment=0;self.state.phase+=1;}}return Ok((None,receipt))}
  self.state.phase=7;let fault=WidgetFault{code:std::mem::take(&mut self.state.code),message:Localized{en:std::mem::take(&mut self.state.en),de:std::mem::take(&mut self.state.de)},port:None};Ok((Some(fault),receipt))
 }
}
fn overflow()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"original localized widget fault UTF8 extent overflow")}
impl RetireOwned for OriginalWidgetFaultProduction{fn retirement(mut self)->Box<dyn RetirementCursor>{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.state.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
impl Drop for OriginalWidgetFaultProduction{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original widget fault requires paid partial text closure");if self.terminal_is_empty()&&!self.transferred{unsafe{ManuallyDrop::drop(&mut self.state)}}}}
