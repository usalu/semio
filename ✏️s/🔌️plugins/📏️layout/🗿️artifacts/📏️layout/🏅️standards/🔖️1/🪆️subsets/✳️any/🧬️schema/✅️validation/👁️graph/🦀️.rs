//! 👁️ Indexed final graph fields borrow actual live-post and authored source ownership without constructing candidate rows.
use crate::{snapshot::LayoutSnapshot,LayoutBounds,LayoutRect,TextStyleRun,DrawingSourceAddress,LayoutDrawingMember,Frame};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum ContentOwner{Parent(usize),Page(usize)}
#[derive(Clone,Copy)]pub struct GridRead{pub baseline_grid:f64,pub baseline_offset:f64}
#[derive(Clone,Copy)]pub struct ParagraphRead<'a>{pub id:&'a str,pub name:&'a str,pub font_family:&'a str,pub font_size:f64,pub font_weight:u32,pub leading:f64,pub tracking:f64,pub alignment:&'a str}
#[derive(Clone,Copy)]pub struct CharacterRead<'a>{pub id:&'a str,pub font_size:Option<f64>,pub font_weight:Option<u32>,pub tracking:Option<f64>,pub color:Option<[f32;4]>}
#[derive(Clone,Copy)]pub struct StoryRead<'a>{pub id:&'a str,pub content:&'a str}
#[derive(Clone,Copy)]pub struct ParentRead<'a>{pub id:&'a str,pub name:&'a str,pub width:f64,pub height:f64}
#[derive(Clone,Copy)]pub struct SpreadRead<'a>{pub id:&'a str,pub name:&'a str}
#[derive(Clone,Copy)]pub struct PageRead<'a>{pub id:&'a str,pub spread_id:&'a str,pub parent_page_id:Option<&'a str>,pub width:f64,pub height:f64,pub margins:[f64;4],pub columns:u32,pub gutter:f64}
#[derive(Clone,Copy)]pub struct OverrideRead<'a>{pub object_id:&'a str,pub bounds:Option<&'a LayoutBounds>}
#[derive(Clone,Copy)]pub enum FrameContentRead<'a>{Rect{fill:Option<[f32;4]>,stroke:Option<[f32;4]>},Text{story_id:&'a str,thread_next:Option<&'a str>,columns:u32,inset:&'a LayoutRect},Image{link_id:&'a str}}
#[derive(Clone,Copy)]pub struct FrameRead<'a>{pub id:&'a str,pub layer_id:&'a str,pub bounds:&'a LayoutBounds,pub content:FrameContentRead<'a>}
impl<'a> From<&'a Frame> for FrameRead<'a>{fn from(frame:&'a Frame)->Self{let content=match frame{Frame::Rect{fill,stroke,..}=>FrameContentRead::Rect{fill:*fill,stroke:*stroke},Frame::Text{story_id,thread_next,columns,inset,..}=>FrameContentRead::Text{story_id,thread_next:thread_next.as_deref(),columns:*columns,inset},Frame::Image{link_id,..}=>FrameContentRead::Image{link_id}};Self{id:frame.id(),layer_id:frame.layer_id(),bounds:frame.bounds(),content}}}
/// 🛂️ Reborrow-only reads confer no original authority; the cursor separately retains both genuine original sources and fixed reader storage.
pub trait LayoutGraphRead:Sync{
 fn schema(&self)->&str;fn grid(&self)->GridRead;
 fn paragraph_count(&self)->usize;fn paragraph(&self,row:usize)->ParagraphRead<'_>;
 fn character_count(&self)->usize;fn character(&self,row:usize)->CharacterRead<'_>;
 fn story_count(&self)->usize;fn story(&self,row:usize)->StoryRead<'_>;fn run_count(&self,story:usize)->usize;fn run(&self,story:usize,row:usize)->&TextStyleRun;
 fn link_count(&self)->usize;fn link_id(&self,row:usize)->&str;
 fn parent_count(&self)->usize;fn parent(&self,row:usize)->ParentRead<'_>;
 fn spread_count(&self)->usize;fn spread(&self,row:usize)->SpreadRead<'_>;fn spread_page_count(&self,spread:usize)->usize;fn spread_page(&self,spread:usize,row:usize)->&str;
 fn page_count(&self)->usize;fn page(&self,row:usize)->PageRead<'_>;fn guide_count(&self,page:usize)->usize;fn guide(&self,page:usize,row:usize)->&LayoutRect;
 fn layer_count(&self,owner:ContentOwner)->usize;fn layer_id(&self,owner:ContentOwner,row:usize)->&str;fn cached_layer_count(&self,owner:ContentOwner)->usize;fn cached_layer(&self,owner:ContentOwner,row:usize)->&str;
 fn member_count(&self,owner:ContentOwner,layer:usize)->usize;fn member(&self,owner:ContentOwner,layer:usize,row:usize)->&str;
 fn frame_count(&self,owner:ContentOwner)->usize;fn frame(&self,owner:ContentOwner,row:usize)->FrameRead<'_>;
 fn override_count(&self,page:usize)->usize;fn page_override(&self,page:usize,row:usize)->OverrideRead<'_>;
 fn drawing(&self,source:DrawingSourceAddress)->Option<&LayoutDrawingMember>;
}
impl LayoutGraphRead for LayoutSnapshot{
 fn schema(&self)->&str{&self.schema}fn grid(&self)->GridRead{GridRead{baseline_grid:self.grid.baseline_grid,baseline_offset:self.grid.baseline_offset}}
 fn paragraph_count(&self)->usize{self.paragraph_styles.len()}fn paragraph(&self,row:usize)->ParagraphRead<'_>{let v=&self.paragraph_styles[row];ParagraphRead{id:&v.id,name:&v.name,font_family:&v.font_family,font_size:v.font_size,font_weight:v.font_weight,leading:v.leading,tracking:v.tracking,alignment:&v.alignment}}
 fn character_count(&self)->usize{self.character_styles.len()}fn character(&self,row:usize)->CharacterRead<'_>{let v=&self.character_styles[row];CharacterRead{id:&v.id,font_size:v.font_size,font_weight:v.font_weight,tracking:v.tracking,color:v.color}}
 fn story_count(&self)->usize{self.stories.len()}fn story(&self,row:usize)->StoryRead<'_>{let v=&self.stories[row];StoryRead{id:&v.id,content:&v.content}}fn run_count(&self,story:usize)->usize{self.stories[story].style_runs.len()}fn run(&self,story:usize,row:usize)->&TextStyleRun{&self.stories[story].style_runs[row]}
 fn link_count(&self)->usize{self.links.len()}fn link_id(&self,row:usize)->&str{&self.links[row].id}
 fn parent_count(&self)->usize{self.parent_pages.len()}fn parent(&self,row:usize)->ParentRead<'_>{let v=&self.parent_pages[row];ParentRead{id:&v.id,name:&v.name,width:v.width,height:v.height}}
 fn spread_count(&self)->usize{self.spreads.len()}fn spread(&self,row:usize)->SpreadRead<'_>{let v=&self.spreads[row];SpreadRead{id:&v.id,name:&v.name}}fn spread_page_count(&self,spread:usize)->usize{self.spreads[spread].page_ids.len()}fn spread_page(&self,spread:usize,row:usize)->&str{&self.spreads[spread].page_ids[row]}
 fn page_count(&self)->usize{self.pages.len()}fn page(&self,row:usize)->PageRead<'_>{let v=&self.pages[row];PageRead{id:&v.id,spread_id:&v.spread_id,parent_page_id:v.parent_page_id.as_deref(),width:v.width,height:v.height,margins:[v.margins.top,v.margins.right,v.margins.bottom,v.margins.left],columns:v.columns.count,gutter:v.columns.gutter}}fn guide_count(&self,page:usize)->usize{self.pages[page].guides.len()}fn guide(&self,page:usize,row:usize)->&LayoutRect{&self.pages[page].guides[row]}
 fn layer_count(&self,owner:ContentOwner)->usize{match owner{ContentOwner::Parent(at)=>self.parent_pages[at].layers.len(),ContentOwner::Page(at)=>self.pages[at].layers.len()}}fn layer_id(&self,owner:ContentOwner,row:usize)->&str{match owner{ContentOwner::Parent(at)=>&self.parent_pages[at].layers[row].id,ContentOwner::Page(at)=>&self.pages[at].layers[row].id}}
 fn cached_layer_count(&self,owner:ContentOwner)->usize{match owner{ContentOwner::Parent(at)=>self.parent_pages[at].layer_ids.len(),ContentOwner::Page(at)=>self.pages[at].layer_ids.len()}}fn cached_layer(&self,owner:ContentOwner,row:usize)->&str{match owner{ContentOwner::Parent(at)=>&self.parent_pages[at].layer_ids[row],ContentOwner::Page(at)=>&self.pages[at].layer_ids[row]}}
 fn member_count(&self,owner:ContentOwner,layer:usize)->usize{match owner{ContentOwner::Parent(at)=>self.parent_pages[at].layers[layer].object_ids.len(),ContentOwner::Page(at)=>self.pages[at].layers[layer].object_ids.len()}}fn member(&self,owner:ContentOwner,layer:usize,row:usize)->&str{match owner{ContentOwner::Parent(at)=>&self.parent_pages[at].layers[layer].object_ids[row],ContentOwner::Page(at)=>&self.pages[at].layers[layer].object_ids[row]}}
 fn frame_count(&self,owner:ContentOwner)->usize{match owner{ContentOwner::Parent(at)=>self.parent_pages[at].frames.len(),ContentOwner::Page(at)=>self.pages[at].frames.len()}}fn frame(&self,owner:ContentOwner,row:usize)->FrameRead<'_>{match owner{ContentOwner::Parent(at)=>(&self.parent_pages[at].frames[row]).into(),ContentOwner::Page(at)=>(&self.pages[at].frames[row]).into()}}
 fn override_count(&self,page:usize)->usize{self.pages[page].overrides.len()}fn page_override(&self,page:usize,row:usize)->OverrideRead<'_>{let v=&self.pages[page].overrides[row];OverrideRead{object_id:&v.object_id,bounds:v.bounds.as_ref()}}
 fn drawing(&self,source:DrawingSourceAddress)->Option<&LayoutDrawingMember>{match source{DrawingSourceAddress::Document=>self.background_drawing.as_ref(),DrawingSourceAddress::Page(at)=>self.pages[at].background_drawing.as_ref()}}
}

#[path="🔗️bindings/🦀️.rs"]pub mod bindings;
pub use bindings::FinalGraphSourceBindings;

#[cfg(test)]#[path="🧪️tests.rs"]mod tests;
