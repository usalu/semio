//! 🗂️ Production root collections share genuine sparse staging and original row movement.
use crate::*;
use semio_s_artifact_layout_layout_model::{snapshot::LayoutSnapshot,ParagraphStyle,CharacterStyle,ParentPage,Spread,Page,TextStory,ImageLink};
use semio_framework_os_kernel::ApplyCapability;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use super::super::{collection::RetainedCollection,keyed_plan::PlanAdvance,setters::OwnedPatch};
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:128,depth:1,..Default::default()}}
fn paid()->RetainedCloneStep{RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:128,..Default::default()})}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Layout root collection address is invalid")}
macro_rules! roots{($($index:literal=>$field:ident:$row:ty,$delta:ty,$patch:ty;$tag:literal),+$(,)?)=>{
 pub struct RootCollections{$(pub $field:RetainedCollection<$row,$delta>,)+}
 impl Default for RootCollections{fn default()->Self{Self{$($field:RetainedCollection::new(),)+}}}
 impl RootCollections{
  pub fn supplied(d:&LayoutDiff,index:usize)->bool{match index{$($index=>d.$field.is_some(),)+_=>false}}
  pub fn preflight_demand(&self,d:RetainedCloneRef<'_,LayoutDiff>,post:&LayoutSnapshot,index:usize,body:usize)->Result<RetirementDemand,ValueError>{if !Self::supplied(d.get(),index){return Ok(fixed())}match index{$($index=>self.$field.preflight_demand(d.project($tag,|d|d.$field.as_ref().unwrap()),&post.$field,body),)+_=>Err(invalid())}}
  pub fn preflight(&mut self,d:RetainedCloneRef<'_,LayoutDiff>,post:&LayoutSnapshot,index:usize,body:usize,g:RetainedCloneGrant)->Result<PlanAdvance,ValueError>{if !Self::supplied(d.get(),index){if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,fixed()){return Ok(PlanAdvance{progress:Default::default(),done:false,error:None})}return Ok(PlanAdvance{progress:paid().progress(),done:true,error:None})}match index{$($index=>self.$field.preflight(d.project($tag,|d|d.$field.as_ref().unwrap()),&post.$field,body,g),)+_=>Err(invalid())}}
  pub fn commit_demand(&self,d:&LayoutDiff,index:usize)->Result<RetirementDemand,ValueError>{if !Self::supplied(d,index){return Ok(fixed())}match index{$($index=>self.$field.membership_commit_demand(),)+_=>Err(invalid())}}
  pub fn commit(&mut self,d:&LayoutDiff,post:&mut LayoutSnapshot,index:usize,g:RetainedCloneGrant,capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>{if !Self::supplied(d,index){return Ok(if semio_framework_value::retained_clone::retained_clone_grant_admits(g,fixed()){paid()}else{RetainedCloneStep::Progress(Default::default())})}match index{$($index=>self.$field.commit_membership(&mut post.$field,g,capability),)+_=>Err(invalid())}}
  pub fn modifications(&self,index:usize)->usize{match index{$($index=>self.$field.modification_targets().map_or(0,|rows|rows.len()),)+_=>0}}
  pub fn fields(index:usize)->Result<usize,ValueError>{match index{$($index=>Ok(<$patch as OwnedPatch<$row>>::FIELD_COUNT),)+_=>Err(invalid())}}
  pub fn patch_demand(&self,index:usize,modification:usize,field:usize)->Result<RetirementDemand,ValueError>{match index{$($index=>self.$field.staged().ok_or_else(invalid)?.modified.get(modification).ok_or_else(invalid)?.patch.field_demand(field),)+_=>Err(invalid())}}
  pub fn patch(&mut self,post:&mut LayoutSnapshot,index:usize,modification:usize,field:usize,g:RetainedCloneGrant,capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>{match index{$($index=>{let target=*self.$field.modification_targets().ok_or_else(invalid)?.get(modification).ok_or_else(invalid)?;let patch=&mut self.$field.staged_mut(capability)?.modified.get_mut(modification).ok_or_else(invalid)?.patch;patch.exchange_field(post.$field.get_mut(target).ok_or_else(invalid)?,field,g,capability)},)+_=>Err(invalid())}}
  pub fn begin_close(&mut self){$(self.$field.begin_close();)+}
  pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{$ (if !self.$field.terminal_is_empty(){return self.$field.close_demand(body)})+Ok(Default::default())}
  pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{$ (if !self.$field.terminal_is_empty(){return self.$field.close_step(g).map(|step|RetainedCloneStep::Progress(step.progress()))})+Ok(RetainedCloneStep::Complete(Default::default()))}
  pub fn terminal_is_empty(&self)->bool{true$(&&self.$field.terminal_is_empty())+}
 }
};}
roots!(0=>paragraph_styles:ParagraphStyle,LayoutParagraphStylesDelta,ParagraphStylePatch;3,1=>character_styles:CharacterStyle,LayoutCharacterStylesDelta,CharacterStylePatch;4,2=>parent_pages:ParentPage,LayoutParentPagesDelta,ParentPagePatch;7,3=>spreads:Spread,LayoutSpreadsDelta,SpreadPatch;8,4=>pages:Page,LayoutPagesDelta,PagePatch;9,5=>stories:TextStory,LayoutStoriesDelta,TextStoryPatch;5,6=>links:ImageLink,LayoutLinksDelta,ImageLinkPatch;6);
