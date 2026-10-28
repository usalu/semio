//! 📄️ Export display primitives borrow the shared retained Layout paragraph owner.
use super::*;
use semio_framework_value::retirement::{RetireOwned,RetirementCursor};
use semio_s_artifact_layout_layout_engine::scene::paragraph::{LayoutParagraphState,LayoutParagraphTurn,LayoutTextCluster,LayoutGlyphStyle};
#[derive(semio_framework_value::RetireOwned)]
struct Owners{text:LayoutParagraphState}
pub(super) struct ExportTextState{owners:Owners}
impl RetireOwned for ExportTextState{fn retirement(self)->Box<dyn RetirementCursor>{self.owners.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.owners.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
impl ExportTextState{
 pub fn new()->Self{Self{owners:Owners{text:LayoutParagraphState::new()}}}
 #[cfg(test)]
 pub fn begin_label_fixture(&mut self,content:String,x:f32,y:f32,size:f32,rotation:f32,center:[f32;2]){self.owners.text.begin_label(content,x,y,size,[0.0,0.0,0.0,1.0],rotation,center,100.0).unwrap();}
 pub fn has_active(&self)->bool{self.owners.text.has_active()}

 pub fn prepare_flow_projection(&mut self,snapshot:&LayoutSnapshot,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if self.owners.text.flow_projection_ready(){return Ok(true);}if cx.is_cancelled()||cx.should_yield(){return Ok(false);}cx.set_stage("layout.export.text-flow");let result=self.owners.text.prepare_flow_projection(snapshot,cx.retained_grant());cx.consume_retained(self.owners.text.flow_projection_last_receipt())?;cx.consume_fuel(1);result?;Ok(self.owners.text.flow_projection_ready())}
 pub fn flow_projection_demand(&self,snapshot:&LayoutSnapshot)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.owners.text.flow_projection_demand(snapshot)}
 pub fn begin_addressed_frame(&mut self,snapshot:&LayoutSnapshot,page:usize,parent:bool,frame:usize)->Result<(),String>{self.owners.text.begin_addressed_frame(snapshot,page,parent,frame).map_err(str::to_owned)}

 pub fn begin_link_label(&mut self,snapshot:&LayoutSnapshot,link_index:usize,bounds:&LayoutBounds)->Result<(),String>{self.owners.text.begin_link_label(snapshot,link_index,bounds).map_err(str::to_owned)}
 pub fn advance_one_in(&mut self,snapshot:&LayoutSnapshot,work:&mut semio_framework_2d_text_font::work::FontWorkLedger)->Result<Option<PdfItem>,semio_framework_value::ValueError>{self.owners.text.advance_one_in(snapshot,work).map(|turn|match turn{LayoutParagraphTurn::Pending|LayoutParagraphTurn::Complete=>None,LayoutParagraphTurn::Glyph(row)=>Some(PdfItem::Outline{source:row.source,glyph:row.glyph,color:row.color,italic:row.italic,rotation:row.rotation,center:row.center})})}
 pub fn last_receipt(&self)->semio_framework_value::RetainedCloneProgress{self.owners.text.last_receipt()}
 pub fn next_demand(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.owners.text.next_demand()}
 pub fn mapped_segment(&self,source:usize,glyph:usize,at:usize,italic:bool,rotation:f32,center:[f32;2])->Result<Option<semio_framework_2d_engine::PathSegment>,String>{let record=self.glyph(source,glyph)?;self.segment(source,glyph,at)?.map(|segment|faithful_primitives::mapped(segment,|point|{let point=if italic{[point[0]-(point[1]-record.origin[1])*0.22,point[1]]}else{point};faithful_primitives::rotate(point,rotation,center)})).transpose()}
 pub fn glyph_cluster<'a>(&'a self,snapshot:&'a LayoutSnapshot,source:usize,glyph:usize)->Result<LayoutTextCluster<'a>,String>{self.owners.text.glyph_cluster(snapshot,source,glyph).map_err(str::to_owned)}
 pub fn glyph_style<'a>(&'a self,snapshot:&'a LayoutSnapshot,source:usize,glyph:usize)->Result<LayoutGlyphStyle<'a>,String>{self.owners.text.glyph_style(snapshot,source,glyph).map_err(str::to_owned)}
 pub fn glyph(&self,source:usize,glyph:usize)->Result<&semio_framework_2d_text_font_run::paragraph::ParagraphGlyph,String>{self.owners.text.glyph(source,glyph).map_err(str::to_owned)}
 pub fn segment(&self,source:usize,glyph:usize,at:usize)->Result<Option<&semio_framework_2d_engine::PathSegment>,String>{self.owners.text.segment(source,glyph,at).map_err(str::to_owned)}
}
