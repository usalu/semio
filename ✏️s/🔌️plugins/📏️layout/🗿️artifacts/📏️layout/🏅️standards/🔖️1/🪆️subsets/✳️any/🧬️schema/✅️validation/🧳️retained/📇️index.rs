//! 📇️ Fixed scratch slots retain only original key addresses and bounded UTF-8 hash state.
use super::*;
use super::super::graph::{LayoutGraphRead,ContentOwner};
use semio_framework_value::scratch_hash::{ScratchHashMap,ScratchHashSlots};
use std::sync::atomic::{AtomicU8,AtomicUsize,Ordering};
#[derive(Clone,Copy,Debug)]pub(super) struct Address{pub kind:u8,pub owner:usize,pub row:usize}
impl Address{
 pub fn scope(self)->(u8,usize){(self.kind,if self.kind==10||self.kind==13||self.kind==14{0}else{self.owner})}
 pub fn text(self,d:&dyn LayoutGraphRead)->&str{match self.kind{0=>d.paragraph(self.row).id,1=>d.character(self.row).id,2=>d.story(self.row).id,3=>d.link_id(self.row),4=>d.parent(self.row).id,5=>d.spread(self.row).id,6=>d.page(self.row).id,7=>d.layer_id(ContentOwner::Parent(self.owner),self.row),8=>d.frame(ContentOwner::Parent(self.owner),self.row).id,9=>d.layer_id(ContentOwner::Page(self.owner),self.row),10=>d.frame(ContentOwner::Page(self.owner),self.row).id,11=>d.page_override(self.owner,self.row).object_id,12=>d.spread_page(self.owner,self.row),13|14=>{let source=if self.owner==0{crate::DrawingSourceAddress::Document}else{crate::DrawingSourceAddress::Page(self.owner-1)};let c=d.drawing(source).expect("borrowed child address");if self.kind==13{&c.handle.child_id}else{&c.handle.target.artifact_id}},_=>unreachable!()}}
}
#[derive(Default)]struct Addresses{kind:u8,owner:usize,row:usize}
enum Next{Skip,Key(Address),Done}
impl Addresses{
 fn next(&mut self,d:&dyn LayoutGraphRead)->Next{if self.kind>14{return Next::Done}let owners=match self.kind{7|8=>d.parent_count(),9..=11=>d.page_count(),12=>d.spread_count(),13|14=>d.page_count()+1,_=>1};if self.owner==owners{self.kind+=1;self.owner=0;self.row=0;return Next::Skip}let count=match self.kind{0=>d.paragraph_count(),1=>d.character_count(),2=>d.story_count(),3=>d.link_count(),4=>d.parent_count(),5=>d.spread_count(),6=>d.page_count(),7=>d.layer_count(ContentOwner::Parent(self.owner)),8=>d.frame_count(ContentOwner::Parent(self.owner)),9=>d.layer_count(ContentOwner::Page(self.owner)),10=>d.frame_count(ContentOwner::Page(self.owner)),11=>d.override_count(self.owner),12=>d.spread_page_count(self.owner),13|14=>usize::from(d.drawing(if self.owner==0{crate::DrawingSourceAddress::Document}else{crate::DrawingSourceAddress::Page(self.owner-1)}).is_some()),_=>0};if self.row==count{self.owner+=1;self.row=0;return Next::Skip}let at=Address{kind:self.kind,owner:if self.kind<=6{0}else{self.owner},row:self.row};self.row+=1;Next::Key(at)}
}
pub(super) struct Entry{pub address:Address,pub incoming:AtomicUsize,pub next:AtomicUsize,pub mark:AtomicU8,pub count:AtomicUsize}
impl Entry{fn new(address:Address)->Self{Self{address,incoming:AtomicUsize::new(0),next:AtomicUsize::new(usize::MAX),mark:AtomicU8::new(0),count:AtomicUsize::new(0)}}pub fn increment_count(&self)->usize{self.count.fetch_add(1,Ordering::Relaxed)}pub fn count(&self)->usize{self.count.load(Ordering::Relaxed)}}
pub(super) type Slots<'a>=ScratchHashSlots<'a,(u8,usize,u64),Entry>;
pub(super) const HASH_OFFSET:u64=14695981039346656037;
pub(super) fn hash_byte(hash:u64,byte:u8)->u64{(hash^u64::from(byte)).wrapping_mul(1099511628211)}
pub(super) fn start(scope:(u8,usize),hash:u64,length:usize)->usize{(hash^(scope.0 as u64).wrapping_mul(0x9e3779b97f4a7c15)^(scope.1 as u64).wrapping_mul(0xd6e8feb86659fd93))as usize&(length-1)}
pub(super) struct Index{map:ScratchHashMap<(u8,usize,u64),Entry>,addresses:Addresses,phase:u8,count:usize,key:Option<Address>,offset:usize,hash:u64,probe:usize,equal:usize,error:Option<&'static str>}
impl Default for Index{fn default()->Self{Self{map:Default::default(),addresses:Default::default(),phase:0,count:0,key:None,offset:0,hash:HASH_OFFSET,probe:0,equal:0,error:None}}}
impl Index{
 pub fn done(&self)->bool{self.phase==7||self.error.is_some()}
 pub fn error(&self)->Option<&'static str>{self.error}
 pub fn slots(&self)->Slots<'_>{self.map.slots()}
 pub fn demand(&self)->Result<RetirementDemand,ValueError>{Ok(match self.phase{1=>ScratchHashMap::<(u8,usize,u64),Entry>::reservation_demands(self.count)?,2=>RetirementDemand{copy_bytes:ScratchHashMap::<(u8,usize,u64),Entry>::slot_bytes(),depth:1,..Default::default()},_=>RetirementDemand{copy_bytes:1024,depth:1,..Default::default()}})}
 pub fn advance(&mut self,d:&dyn LayoutGraphRead,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let demand=self.demand()?;if self.done()||!retained_clone_grant_admits(g,demand){return Ok(Default::default())}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};match self.phase{
 0=>match self.addresses.next(d){Next::Key(_)=>self.count=self.count.checked_add(1).ok_or_else(||LayoutValidationCursor::fault("Layout key index extent overflow"))?,Next::Skip=>{},Next::Done=>{self.phase=1;self.addresses=Default::default()}},
 1=>{self.map=ScratchHashMap::try_reserve_uninitialized(self.count)?;p.retained_capacity_bytes=self.map.backing_bytes();if p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(LayoutValidationCursor::fault("Layout key index allocator exceeded grant").with_retained_progress(p))}self.phase=2},
 2=>if !self.map.initialize_one()?{self.phase=3},
 3=>match self.addresses.next(d){Next::Done=>self.phase=7,Next::Skip=>{},Next::Key(key)=>{self.key=Some(key);self.offset=0;self.phase=4}},
 4=>{let text=self.key.unwrap().text(d);let mut used=0;let mut nonempty=false;for ch in text[self.offset..].chars().take(64){used+=ch.len_utf8();if !ch.is_whitespace(){nonempty=true;break}}self.offset+=used;if nonempty{self.offset=0;self.hash=HASH_OFFSET;self.phase=5}else if self.offset==text.len(){self.error=Some("Ids must be non-empty and unique in their collection.")}},
 5=>{let key=self.key.unwrap();let text=key.text(d);let end=text.len().min(self.offset.saturating_add(256));for byte in &text.as_bytes()[self.offset..end]{self.hash=hash_byte(self.hash,*byte)}self.offset=end;if end==text.len(){self.probe=start(key.scope(),self.hash,self.map.initialized_slots());self.equal=0;self.phase=6}},
 6=>{let key=self.key.unwrap();let(scope,owner)=key.scope();let slots=self.map.slots();if slots.is_vacant(self.probe){self.map.insert_vacant(self.probe,(scope,owner,self.hash),Entry::new(key)).unwrap_or_else(|_|unreachable!("admitted vacant Layout index slot"));self.phase=3;self.key=None;}else{let(held,address)=slots.get(self.probe).expect("initialized occupied scratch slot");let target=key.text(d);let previous=address.address.text(d);if *held==(scope,owner,self.hash)&&target.len()==previous.len(){let end=target.len().min(self.equal.saturating_add(256));if target.as_bytes()[self.equal..end]==previous.as_bytes()[self.equal..end]{self.equal=end;if end==target.len(){self.error=Some("Ids must be non-empty and unique in their collection.")}return Ok(p)}}self.probe=(self.probe+1)&(slots.len()-1);self.equal=0;}},_=>{}}Ok(p)}
 pub fn close_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:ScratchHashMap::<(u8,usize,u64),Entry>::slot_bytes(),release_bytes:if self.map.initialized_slots()==0{self.map.backing_bytes()}else{0},depth:1,..Default::default()}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{let d=self.close_demand();if !retained_clone_grant_admits(g,d){return Ok(Default::default())}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};if self.map.pop_last_slot().is_none(){p.released_bytes=self.map.release_empty_backing()?;}Ok(p)}
 pub fn terminal_is_empty(&self)->bool{self.map.initialized_slots()==0&&self.map.backing_bytes()==0}
}
