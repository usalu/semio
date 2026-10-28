//! 🪡️ One supplied typed setter exchanges original field custody under central authority.
use crate::*;
use semio_s_artifact_layout_layout_model::{ParagraphStyle,CharacterStyle,ParentPage,Spread,ImageLink,TextStory,Page,Layer,Frame,GridSettings,LayoutBounds};
use semio_framework_os_kernel::ApplyCapability;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;
/// 🎚️ Nested collections and derived caches have their own separately admitted frontiers.
pub trait OwnedPatch<R>{const FIELD_COUNT:usize;fn supplied(&self,index:usize)->bool;fn field_demand(&self,index:usize)->Result<RetirementDemand,ValueError>;fn admissible(&self,_row:&R,index:usize)->bool{index<Self::FIELD_COUNT}fn exchange_field(&mut self,row:&mut R,index:usize,g:RetainedCloneGrant,capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>;}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"owned Layout setter field is invalid")}
fn fixed<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>()*4+128,depth:1,..Default::default()}}
fn exchange<T>(source:&mut Option<T>,target:&mut T){if let Some(value)=source.as_mut(){std::mem::swap(value,target)}}
fn optional<T>(source:&mut Option<T>,target:&mut Option<T>){if source.is_some(){std::mem::swap(source,target)}}
fn progress(d:RetirementDemand)->RetainedCloneStep{RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}
macro_rules! swap{(set,$source:expr,$target:expr)=>{exchange($source,$target)};(optional,$source:expr,$target:expr)=>{optional($source,$target)};}
macro_rules! fields{($patch:ty=>$row:ty,$count:expr; $($index:literal=>$field:ident:$kind:ty=>$mode:ident $($target:ident).+),+ $(,)?)=>{
 impl OwnedPatch<$row> for $patch{
  const FIELD_COUNT:usize=$count;
  fn supplied(&self,index:usize)->bool{match index{$($index=>self.$field.is_some(),)+_=>false}}
  fn field_demand(&self,index:usize)->Result<RetirementDemand,ValueError>{match index{$($index=>Ok(fixed::<$kind>()),)+_=>Err(invalid())}}
  fn exchange_field(&mut self,row:&mut $row,index:usize,g:RetainedCloneGrant,_capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>{let d=self.field_demand(index)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}match index{$($index=>swap!($mode,&mut self.$field,&mut row.$($target).+),)+_=>return Err(invalid())}Ok(progress(d))}
 }
};}
fields!(ParagraphStylePatch=>ParagraphStyle,7;0=>name:String=>set name,1=>font_family:String=>set font_family,2=>font_size:f64=>set font_size,3=>font_weight:u32=>set font_weight,4=>leading:f64=>set leading,5=>tracking:f64=>set tracking,6=>alignment:String=>set alignment);
fields!(CharacterStylePatch=>CharacterStyle,7;0=>name:Option<String>=>set name,1=>font_family:Option<String>=>set font_family,2=>font_size:Option<f64>=>set font_size,3=>font_weight:Option<u32>=>set font_weight,4=>italic:Option<bool>=>set italic,5=>color:Option<[f32;4]>=>set color,6=>tracking:Option<f64>=>set tracking);
fields!(ParentPagePatch=>ParentPage,3;0=>name:String=>set name,1=>width:f64=>set width,2=>height:f64=>set height);
fields!(SpreadPatch=>Spread,2;0=>name:String=>set name,1=>page_ids:Vec<String>=>set page_ids);
fields!(ImageLinkPatch=>ImageLink,5;0=>path:String=>set path,1=>width:u32=>set width,2=>height:u32=>set height,3=>dpi:u32=>set dpi,4=>color_profile:Option<String>=>set color_profile);
fields!(LayerPatch=>Layer,3;0=>name:String=>set name,1=>visible:bool=>set visible,2=>locked:bool=>set locked);
fields!(TextStoryPatch=>TextStory,1;0=>content:String=>set content);
fields!(PagePatch=>Page,12;0=>name:String=>set name,1=>spread_id:String=>set spread_id,2=>width:f64=>set width,3=>height:f64=>set height,4=>margin_top:f64=>set margins.top,5=>margin_right:f64=>set margins.right,6=>margin_bottom:f64=>set margins.bottom,7=>margin_left:f64=>set margins.left,8=>columns_count:u32=>set columns.count,9=>columns_gutter:f64=>set columns.gutter,10=>parent_page_id:Option<String>=>set parent_page_id,11=>background_drawing:Option<semio_s_artifact_layout_layout_model::LayoutDrawingMember>=>set background_drawing);
fields!(GridPatch=>GridSettings,3;0=>baseline_grid:f64=>set baseline_grid,1=>baseline_offset:f64=>set baseline_offset,2=>snap_to_baseline:bool=>set snap_to_baseline);
fn bounds(row:&mut Frame)->&mut LayoutBounds{match row{Frame::Rect{bounds,..}|Frame::Text{bounds,..}|Frame::Image{bounds,..}=>bounds}}
impl OwnedPatch<Frame> for FramePatch{
 const FIELD_COUNT:usize=18;
 fn supplied(&self,index:usize)->bool{match index{0=>self.x.is_some(),1=>self.y.is_some(),2=>self.width.is_some(),3=>self.height.is_some(),4=>self.rotation.is_some(),5=>self.fill.is_some(),6=>self.stroke.is_some(),7=>self.wrap_mode.is_some(),8=>self.columns.is_some(),9=>self.locked.is_some(),10=>self.visible.is_some(),11=>self.story_id.is_some(),12=>self.thread_next.is_some(),13=>self.inset_x.is_some(),14=>self.inset_y.is_some(),15=>self.inset_width.is_some(),16=>self.inset_height.is_some(),17=>self.layer_id.is_some(),_=>false}}
 fn field_demand(&self,index:usize)->Result<RetirementDemand,ValueError>{Ok(match index{0..=4|13..=16=>fixed::<f64>(),5|6=>fixed::<Option<[f32;4]>>(),7|11|17=>fixed::<String>(),8=>fixed::<u32>(),9|10=>fixed::<Option<bool>>(),12=>fixed::<Option<String>>(),_=>return Err(invalid())})}
 fn admissible(&self,row:&Frame,index:usize)->bool{if index>=18{return false}if !self.supplied(index){return true}match index{5|6=>matches!(row,Frame::Rect{..}),7|8|11..=16=>matches!(row,Frame::Text{..}),_=>true}}
 fn exchange_field(&mut self,row:&mut Frame,index:usize,g:RetainedCloneGrant,_capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>{let d=self.field_demand(index)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.admissible(row,index){return Err(invalid())}if !self.supplied(index){return Ok(progress(d))}match index{
  0=>exchange(&mut self.x,&mut bounds(row).x),1=>exchange(&mut self.y,&mut bounds(row).y),2=>exchange(&mut self.width,&mut bounds(row).width),3=>exchange(&mut self.height,&mut bounds(row).height),4=>exchange(&mut self.rotation,&mut bounds(row).rotation),
  5=>if let Frame::Rect{fill,..}=row{exchange(&mut self.fill,fill)},6=>if let Frame::Rect{stroke,..}=row{exchange(&mut self.stroke,stroke)},
  7=>if let Frame::Text{wrap_mode,..}=row{exchange(&mut self.wrap_mode,wrap_mode)},8=>if let Frame::Text{columns,..}=row{exchange(&mut self.columns,columns)},
  9=>match row{Frame::Rect{locked,..}|Frame::Text{locked,..}|Frame::Image{locked,..}=>optional(&mut self.locked,locked)},10=>match row{Frame::Rect{visible,..}|Frame::Text{visible,..}|Frame::Image{visible,..}=>optional(&mut self.visible,visible)},
  11=>if let Frame::Text{story_id,..}=row{exchange(&mut self.story_id,story_id)},12=>if let Frame::Text{thread_next,..}=row{exchange(&mut self.thread_next,thread_next)},
  13=>if let Frame::Text{inset,..}=row{exchange(&mut self.inset_x,&mut inset.x)},14=>if let Frame::Text{inset,..}=row{exchange(&mut self.inset_y,&mut inset.y)},15=>if let Frame::Text{inset,..}=row{exchange(&mut self.inset_width,&mut inset.width)},16=>if let Frame::Text{inset,..}=row{exchange(&mut self.inset_height,&mut inset.height)},
  17=>match row{Frame::Rect{layer_id,..}|Frame::Text{layer_id,..}|Frame::Image{layer_id,..}=>exchange(&mut self.layer_id,layer_id)},_=>return Err(invalid())
 }Ok(progress(d))}
}

use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
impl OwnedPatch<LayoutSnapshot> for LayoutDiff{
 const FIELD_COUNT:usize=5;
 fn supplied(&self,index:usize)->bool{match index{0=>self.schema.is_some(),1=>self.name.is_some(),2=>self.print_target.is_some(),3=>self.background_drawing.is_some(),4=>self.referenced_model.is_some(),_=>false}}
 fn field_demand(&self,index:usize)->Result<RetirementDemand,ValueError>{Ok(match index{0|1=>fixed::<String>(),2=>fixed::<Option<String>>(),3=>fixed::<Option<semio_s_artifact_layout_layout_model::LayoutDrawingMember>>(),4=>fixed::<Option<semio_framework_os_kernel::ArtifactLink>>(),_=>return Err(invalid())})}
 fn exchange_field(&mut self,row:&mut LayoutSnapshot,index:usize,g:RetainedCloneGrant,_capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>{let d=self.field_demand(index)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}match index{0=>exchange(&mut self.schema,&mut row.schema),1=>exchange(&mut self.name,&mut row.name),2=>if let Some(change)=self.print_target.as_mut(){std::mem::swap(&mut change.target,&mut row.print_target)},3=>exchange(&mut self.background_drawing,&mut row.background_drawing),4=>exchange(&mut self.referenced_model,&mut row.referenced_model),_=>return Err(invalid())}Ok(progress(d))}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
