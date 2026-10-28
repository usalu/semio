//! 📕️ Pdf, png and package encoding of the layout export job: the pdf objects, page and item operators, the png rows and idat chunks and the zip package entries. See [`super`].

use super::*;
#[path="📦️zip/🦀️.rs"]pub(super) mod zip;
#[path = "🖼️raster/🦀️.rs"]
pub(super) mod raster;
#[path = "🔤️unicode/🦀️.rs"]
mod unicode;
#[path = "📝️text/🦀️.rs"]
mod semantic_text;

#[path="🧭️projection/🦀️.rs"]
mod projection;
use semio_s_artifact_layout_layout_engine::scene::paragraph::{LayoutTextCluster,LayoutGlyphStyle};
struct PdfTextProjection<'a>{cluster:LayoutTextCluster<'a>,style:LayoutGlyphStyle<'a>,record:&'a semio_framework_2d_text_font_run::paragraph::ParagraphGlyph,affine:[f64;6],color:[f32;4],italic:bool}
fn pdf_text_projection<'a>(item:&PdfItem,text:Option<&'a ExportTextState>,drawing:Option<&'a DrawingCompositionState>,snapshot:&'a LayoutSnapshot)->Result<Option<PdfTextProjection<'a>>,String>{Ok(match item{PdfItem::Outline{source,glyph,color,italic,rotation,center}=>{let text=text.ok_or("layout-export-paragraph-source-missing")?;let(s,c)=f64::from(*rotation).sin_cos();let(cx,cy)=(f64::from(center[0]),f64::from(center[1]));Some(PdfTextProjection{cluster:text.glyph_cluster(snapshot,*source,*glyph)?,style:text.glyph_style(snapshot,*source,*glyph)?,record:text.glyph(*source,*glyph)?,affine:[c,s,-s,c,cx-c*cx+s*cy,cy-s*cx-c*cy],color:*color,italic:*italic})},PdfItem::DrawingOutline{label,glyph,paint,affine}=>{let drawing=drawing.ok_or("layout-export-drawing-source-missing")?;Some(PdfTextProjection{cluster:drawing.glyph_cluster(*label,*glyph)?,style:drawing.glyph_style(snapshot,*label,*glyph)?,record:drawing.glyph(*label,*glyph)?,affine:*affine,color:paint.fill.or(paint.stroke).unwrap_or([0.0;4]),italic:false})},_=>None})}
fn pdf_same_text_cluster(item:&PdfItem,next:Option<&PdfItem>,text:Option<&ExportTextState>,drawing:Option<&DrawingCompositionState>)->Result<bool,String>{Ok(match(item,next){(PdfItem::Outline{source:a,glyph:x,..},Some(PdfItem::Outline{source:b,glyph:y,..}))if a==b=>{let text=text.ok_or("layout-export-paragraph-source-missing")?;text.glyph(*a,*x)?.source_at==text.glyph(*b,*y)?.source_at},(PdfItem::DrawingOutline{label:a,glyph:x,..},Some(PdfItem::DrawingOutline{label:b,glyph:y,..}))if a==b=>{let drawing=drawing.ok_or("layout-export-drawing-source-missing")?;drawing.glyph(*a,*x)?.source_at==drawing.glyph(*b,*y)?.source_at},_=>false})}

//#region 🧩️Job
impl LayoutExportJob {
    /// 📕️ Advances one original-source Drawing command, addressed frame or retained paragraph turn.
    pub(super) fn plan_pdf_one(&mut self,work:&mut semio_framework_2d_text_font::work::FontWorkLedger,drawing_read:Option<&semio_framework_io_ports::ChildAdmissionRead<'_>>) -> Result<(), String> {
        if !work.copy_bytes(std::mem::size_of::<Option<DrawingSourceAddress>>()){return Ok(());}
        let request=self.request.as_ref().unwrap();
        let selected=self.target_page.filter(|_|request.page_id.is_some()||matches!(request.kind,LayoutExportKind::Svg|LayoutExportKind::Png));
        let page_index=match selected{Some(index)if self.pdf_page_cursor==0=>Some(index),Some(_)=>None,None=>(self.pdf_page_cursor<request.snapshot.pages.len()).then_some(self.pdf_page_cursor)};
        let Some(page_index)=page_index else{self.stage=ExportStage::BeginEncode;return Ok(());};
        let snapshot=Arc::clone(&request.snapshot);let page=snapshot.pages.get(page_index).ok_or("layout-export-page-missing")?;
        if self.drawing.as_ref().is_some_and(DrawingCompositionState::has_active){
            let read=drawing_read.ok_or("layout-export-original-drawing-read-required")?;let grant=work.child_grant();let drawing=self.drawing.as_mut().unwrap();let demand=drawing.next_demand(&snapshot,read,grant.maximum_copy_bytes).map_err(|error|error.to_string())?;let result=drawing.advance(&snapshot,read,semio_framework_io_ports::VectorSceneGrant{maximum_source_items:grant.maximum_items,maximum_depth:grant.maximum_depth,maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes});let used=drawing.last_receipt();
            let item=result.map(|turn|match turn{DrawingCompositionTurn::Progress|DrawingCompositionTurn::Done=>None,DrawingCompositionTurn::Path{index}=>Some(PdfItem::DrawingPath{index}),DrawingCompositionTurn::Glyph{label,glyph,paint,affine}=>Some(PdfItem::DrawingOutline{label,glyph,paint,affine}),DrawingCompositionTurn::Image{x,y,width,height,affine,proxy}=>{let[a,b,c,d,e,f]=affine;Some(PdfItem::Image{x:0.0,y:0.0,width:1.0,height:1.0,rotation:0.0,affine:Some([a*width,b*width,c*height,d*height,a*x+c*y+e,b*x+d*y+f]),link:None,placeholder:false,mark:String::new(),proxy:Some(proxy),raster:None})}});
            if !drawing.has_active(){self.drawing_source=None;}let debit=work.consume_child(semio_framework_value::RetainedCloneProgress{copied_items:used.source_items,copied_bytes:used.copied_bytes,retained_capacity_bytes:used.retained_capacity_bytes,released_bytes:used.released_bytes});match item{Ok(Some(item))=>self.push_plan_item(item,work)?,Ok(None)=>{},Err(error)=>{debit.map_err(str::to_owned)?;return Err(error);}}debit.map_err(str::to_owned)?;if used.source_items==0{work.require_child(demand).map_err(str::to_owned)?;}return Ok(());
        }
        if !self.pdf_page_started{self.pdf_page_started=true;self.pdf_plan_items=0;self.pdf_background_cursor=0;return Ok(());}
        if self.pdf_background_cursor<2{let page_child=self.pdf_background_cursor==1;self.pdf_background_cursor+=1;let present=if page_child{page.background_drawing.is_some()}else{snapshot.background_drawing.is_some()};if present{let source=if page_child{DrawingSourceAddress::Page(page_index)}else{DrawingSourceAddress::Document};self.drawing.get_or_insert_with(DrawingCompositionState::new).begin(source,if page_child{DrawingPlacement::Canvas}else{DrawingPlacement::Content},LayoutBounds{x:0.0,y:0.0,width:page.width,height:page.height,rotation:0.0})?;self.drawing_source=Some(source);}return Ok(());}
        match self.next_frame_plan(page_index)?{
            FramePlanTurn::Pending=>{},
            FramePlanTurn::Frame(address)=>{
                if !address.visible(&snapshot)?{return Ok(());}let frame=address.frame(&snapshot)?;let bounds=address.bounds(&snapshot)?;
                match frame{
                    Frame::Text{..}=>self.text.get_or_insert_with(ExportTextState::new).begin_addressed_frame(&snapshot,address.page_index,address.parent_index.is_some(),address.frame_index)?,
                    Frame::Rect{fill,stroke,..}=>self.push_plan_item(PdfItem::Rect{x:bounds.x,y:bounds.y,width:bounds.width,height:bounds.height,rotation:bounds.rotation,fill:*fill,stroke:*stroke},work)?,
                    Frame::Image{..}=>{
                        let link_index=address.link_index.ok_or("layout-export-image-link-missing")?;let link=snapshot.links.get(link_index).ok_or("layout-export-image-link-address")?;let proxy=link.proxy_data_url.as_deref().filter(|value|!value.is_empty());let placed=proxy.is_none()&&snapshot.background_drawing.is_some()&&semio_s_artifact_layout_layout_engine::scene::drawing_kind(&link.artifact_kind);
                        let mark=String::new();
                        self.push_plan_item(PdfItem::Image{x:bounds.x,y:bounds.y,width:bounds.width,height:bounds.height,rotation:bounds.rotation,affine:None,link:Some(link_index),placeholder:link.state.as_deref()==Some("missing")||proxy.is_none(),mark,proxy:None,raster:None},work)?;
                        if placed{self.drawing.get_or_insert_with(DrawingCompositionState::new).begin(semio_s_artifact_layout_layout_engine::scene::drawing_planning::DrawingSourceAddress::Document,semio_s_artifact_layout_layout_engine::scene::drawing_planning::DrawingPlacement::Content,bounds)?;self.drawing_source=Some(DrawingSourceAddress::Document);}else if proxy.is_none()&&!link.artifact_kind.is_empty(){self.text.get_or_insert_with(ExportTextState::new).begin_link_label(&snapshot,link_index,&bounds)?;}
                    }
                }
            },
            FramePlanTurn::Complete=>{self.pdf_pages.push_in(PdfPagePlan{width:finite_dimension(page.width)?,height:finite_dimension(page.height)?,items:self.pdf_plan_items,first_item:self.pdf_items.len()-self.pdf_plan_items},work).map_err(|error|error.to_string())?;self.pdf_page_cursor+=1;self.pdf_page_started=false;self.pdf_background_cursor=0;}
        }Ok(())
    }

    pub(super) fn push_plan_item(&mut self,item:PdfItem,work:&mut semio_framework_2d_text_font::work::FontWorkLedger)->Result<(),String>{self.pdf_items.push_in(item,work).map_err(|error|error.to_string())?;self.pdf_plan_items+=1;Ok(())}

    pub(super) fn begin_pdf(&mut self,work:&mut OutputWork) -> Result<(), String> {
        self.output.append(b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n",work)?;
        self.pdf_section = PdfSection::Catalog;
        self.pdf_cursor = 0;
        self.pdf_item_cursor = 0;
        self.pdf_text_marked = false;
        Ok(())
    }

    pub(super) fn pdf_offset_demand(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if self.pdf_offset_needed(){self.pdf_offsets.next_append_demand()}else{Ok(Default::default())}}
    fn pdf_offset_needed(&self)->bool{matches!(self.pdf_section,PdfSection::Catalog|PdfSection::Pages|PdfSection::FontType0|PdfSection::FontCid|PdfSection::FontDescriptor|PdfSection::FontFileBegin|PdfSection::PageObject|PdfSection::ContentBegin|PdfSection::ContentLength|PdfSection::ImageBegin|PdfSection::ImageAlphaBegin|PdfSection::UnicodeBegin|PdfSection::UnicodeLength|PdfSection::EncodingCmap)}
    fn prepare_pdf_offset_in(&mut self,work:&mut OutputWork)->Result<bool,String>{if !self.pdf_offset_needed(){return Ok(true);}let mut ledger=semio_framework_2d_text_font::work::FontWorkLedger::new(work.remaining());let result=self.pdf_offsets.reserve_one_in(&mut ledger);work.consume(ledger.receipt()).map_err(str::to_owned)?;let ready=result.map_err(|error|error.to_string())?;if !ready{work.mark_preparatory();}Ok(ready)}
    fn pdf_object_offset(&mut self,id:usize,work:&mut OutputWork)->Result<(),String>{
        if self.pdf_offsets.len()+1!=id{return Err("layout-export-pdf-object-order".into());}let value=u32::try_from(self.output.len).map_err(|_|"layout-export-output-limit")?;
        let mut ledger=semio_framework_2d_text_font::work::FontWorkLedger::new(work.remaining());let result=(||{if !self.pdf_offsets.reserve_one_in(&mut ledger)?{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout-export-offset-reservation"));}self.pdf_offsets.push_in(value,&mut ledger)})();work.consume(ledger.receipt()).map_err(str::to_owned)?;result.map_err(|error|error.to_string())
    }

    fn pdf_object(&mut self, id: usize, body: &[u8],work:&mut OutputWork) -> Result<(), String> {
        self.pdf_object_offset(id,work)?;
        self.output.append_format(format_args!("{id} 0 obj\n"),work)?;
        self.output.append(body,work)?;
        self.output.append(b"\nendobj\n",work)
    }

    fn pdf_object_format(&mut self,id:usize,body:std::fmt::Arguments<'_>,work:&mut OutputWork)->Result<(),String>{self.pdf_object_offset(id,work)?;self.output.append_format(format_args!("{id} 0 obj\n"),work)?;self.output.append_format(body,work)?;self.output.append(b"\nendobj\n",work)}

    fn pdf_page_object_id(page: usize) -> usize {
        7 + 3 * page
    }



    pub(super) fn encode_pdf_one(&mut self,work:&mut OutputWork) -> Result<(), String> {
        if !self.prepare_pdf_offset_in(work)?{return Ok(());}
        const FONT: &[u8] = semio_s_artifact_layout_layout_engine::scene::text_fonts::LAYOUT_SANS;
        let page_count = self.pdf_pages.len();
        match self.pdf_section {
            PdfSection::Header => return Err("layout-export-pdf-header-state".into()),
            PdfSection::Catalog => {
                self.pdf_object(1, b"<< /Type /Catalog /Pages 2 0 R >>",work)?;
                self.pdf_section = PdfSection::Pages;
            }
            PdfSection::Pages => {
                self.pdf_object_offset(2,work)?;
                self.output.append(b"2 0 obj\n<< /Type /Pages /Kids [",work)?;
                self.pdf_cursor = 0;
                self.pdf_section = PdfSection::PageKids;
            }
            PdfSection::PageKids => {
                if self.pdf_cursor < page_count {
                    self.output.append_format(format_args!("{} 0 R ", Self::pdf_page_object_id(self.pdf_cursor)),work)?;
                    self.pdf_cursor += 1;
                } else {
                    self.pdf_section = PdfSection::PagesEnd;
                }
            }
            PdfSection::PagesEnd => {
                self.output.append_format(format_args!("] /Count {page_count} >>\nendobj\n"),work)?;
                self.pdf_section = PdfSection::FontType0;
            }
            PdfSection::FontType0 => {
                let unicode = 7 + 3 * page_count + 2 * self.fidelity.as_ref().ok_or("layout-export-paint-plan-missing")?.image_count();
                self.pdf_object_format(3,format_args!("<< /Type /Font /Subtype /Type0 /BaseFont /LayoutSans /Encoding {} 0 R /ToUnicode {unicode} 0 R /DescendantFonts [4 0 R] >>", unicode + 2),work)?;
                self.pdf_section = PdfSection::FontCid;
            }
            PdfSection::FontCid => {
                self.pdf_object(4, b"<< /Type /Font /Subtype /CIDFontType2 /BaseFont /LayoutSans /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> /FontDescriptor 5 0 R /DW 1000 /CIDToGIDMap /Identity >>",work)?;
                self.pdf_section = PdfSection::FontDescriptor;
            }
            PdfSection::FontDescriptor => {
                self.pdf_object(5, b"<< /Type /FontDescriptor /FontName /LayoutSans /Flags 32 /FontBBox [-200 -300 1200 1000] /ItalicAngle 0 /Ascent 900 /Descent -250 /CapHeight 700 /StemV 80 /FontFile2 6 0 R >>",work)?;
                self.pdf_section = PdfSection::FontFileBegin;
            }
            PdfSection::FontFileBegin => {
                self.pdf_object_offset(6,work)?;
                self.output.append_format(format_args!("6 0 obj\n<< /Length {} /Length1 {} >>\nstream\n", FONT.len(), FONT.len()),work)?;
                self.pdf_font_cursor = 0;
                self.pdf_section = PdfSection::FontFileBytes;
            }
            PdfSection::FontFileBytes => {
                let end = (self.pdf_font_cursor + OUTPUT_CHUNK_BYTES).min(FONT.len());
                if self.pdf_font_cursor < end {
                    self.output.append(&FONT[self.pdf_font_cursor..end],work)?;
                    self.pdf_font_cursor = end;
                } else {
                    self.pdf_section = PdfSection::FontFileEnd;
                }
            }
            PdfSection::FontFileEnd => {
                self.output.append(b"\nendstream\nendobj\n",work)?;
                self.pdf_cursor = 0;
                self.pdf_item_cursor = 0;
                self.pdf_section = if page_count == 0 { PdfSection::ImageBegin } else { PdfSection::PageObject };
            }
            PdfSection::PageObject => {
                let page = self.pdf_pages.get(self.pdf_cursor).copied().ok_or("layout-export-page-missing")?;
                let id = Self::pdf_page_object_id(self.pdf_cursor);
                self.pdf_object_offset(id,work)?;
                self.output.append_format(format_args!("{id} 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /Font << /F1 3 0 R >> /XObject <<", page.width, page.height),work)?;
                self.pdf_primitive_cursor = page.first_item;
                self.pdf_section = PdfSection::PageResources;
            }
            PdfSection::PageResources => {
                let page = self.pdf_pages.get(self.pdf_cursor).copied().ok_or("layout-export-page-missing")?;
                if self.pdf_primitive_cursor < page.first_item + page.items {
                    if let Some(PdfItem::Image { raster: Some(image), .. }) = self.pdf_items.get(self.pdf_primitive_cursor) {
                        let resource = image.resource.ok_or("layout-export-image-resource-missing")?;
                        let id = 7 + 3 * page_count + 2 * resource;
                        self.output.append_format(format_args!(" /Im{resource} {id} 0 R"),work)?;
                    }
                    self.pdf_primitive_cursor += 1;
                } else { self.output.append(b" >> /ExtGState <<",work)?; self.pdf_primitive_cursor = page.first_item; self.pdf_section = PdfSection::PageGraphicsStates; }
            }
            PdfSection::PageGraphicsStates => {
                let page = self.pdf_pages.get(self.pdf_cursor).copied().ok_or("layout-export-page-missing")?;
                if self.pdf_primitive_cursor < page.first_item + page.items {
                    if let Some(PaintShape::Path(path)) = self.fidelity.as_ref().ok_or("layout-export-paint-plan-missing")?.shape(self.pdf_primitive_cursor) {
                        let fill = path.fill.map_or(1.0, |color| color[3].clamp(0.0, 1.0)); let stroke = path.stroke.map_or(1.0, |color| color[3].clamp(0.0, 1.0));
                        self.output.append_format(format_args!(" /GS{} << /Type /ExtGState /ca {fill:.6} /CA {stroke:.6} >>", self.pdf_primitive_cursor),work)?;
                    }
                    self.pdf_primitive_cursor += 1;
                } else { let id = Self::pdf_page_object_id(self.pdf_cursor); self.output.append_format(format_args!(" >> >> /Contents {} 0 R >>\nendobj\n", id + 1),work)?; self.pdf_primitive_cursor = 0; self.pdf_section = PdfSection::ContentBegin; }
            }
            PdfSection::ContentBegin => {
                let id = Self::pdf_page_object_id(self.pdf_cursor) + 1;
                self.pdf_object_offset(id,work)?;
                self.output.append_format(format_args!("{id} 0 obj\n<< /Length {} 0 R >>\nstream\n", id + 1),work)?;
                self.pdf_content_start = self.output.len;
                self.pdf_section = PdfSection::ContentItem;
            }
            PdfSection::ContentItem => {
                let page = self.pdf_pages.get(self.pdf_cursor).copied().ok_or("layout-export-page-missing")?;
                if self.pdf_item_cursor >= page.items { self.pdf_section = PdfSection::ContentEnd; return Ok(()); }
                let source = page.first_item + self.pdf_item_cursor;
                let shape = self.fidelity.as_ref().ok_or("layout-export-paint-plan-missing")?.shape(source).ok_or("layout-export-paint-source-missing")?;
                match shape {
                    PaintShape::Image { source } => {
                        let item=self.pdf_items.get(*source).ok_or("layout-export-image-missing")?;
                        let PdfItem::Image{x,y,width,height,raster:Some(image),..}=item else{return Err("layout-export-image-unavailable".into());};
                        let resource=image.resource.ok_or("layout-export-image-resource-missing")?;
                        let affine=faithful_primitives::image_affine(item).ok_or("layout-export-image-transform-missing")?;
                        let m=projection::image_matrix(affine,[f64::from(*x),f64::from(*y),f64::from(*width),f64::from(*height)],f64::from(page.height));
                        self.output.append_format(format_args!("q {} {} {} {} {} {} cm /Im{resource} Do Q\n",m[0],m[1],m[2],m[3],m[4],m[5]),work)?;
                        self.pdf_item_cursor+=1;
                    }
                    PaintShape::Path(path) => {
                        if self.pdf_text_phase < 6 {
                            let item=self.pdf_items.get(source).ok_or("layout-export-paint-source-missing")?;
                            if let Some(text)=pdf_text_projection(item,self.text.as_ref(),self.drawing.as_ref(),&self.request.as_ref().unwrap().snapshot)?{
                                if !text.cluster.first_glyph||text.cluster.text.is_empty(){self.pdf_text_phase=6;}
                                else if self.pdf_text_phase==0{if let Some(scalar)=text.cluster.text[self.pdf_text_byte..].chars().next(){self.pdf_text_byte+=scalar.len_utf8();self.pdf_text_count+=1;return Ok(());}self.pdf_text_phase=1;self.pdf_text_byte=0;return Ok(());}
                                else if self.pdf_text_phase==1{self.output.append(semantic_text::PDF_LAYOUT_TEXT_PREFIX.as_bytes(),work)?;self.pdf_text_marked=true;self.pdf_text_phase=2;return Ok(());}
                                else if self.pdf_text_phase==2{if let Some(scalar)=text.style.family[self.pdf_text_byte..].chars().next(){self.output.append_format(format_args!("{}",semantic_text::font_family_scalar(scalar)),work)?;self.pdf_text_byte+=scalar.len_utf8();return Ok(());}self.pdf_text_phase=3;self.pdf_text_byte=0;return Ok(());}
                                else if self.pdf_text_phase==3{self.output.append_format(format_args!("{}",semantic_text::text_properties(text.style.weight,text.style.italic)),work)?;let m=projection::text_matrix(text.affine,text.record.origin,f64::from(page.height),text.italic);let scale=text.record.advance*100.0/(text.record.size*self.pdf_text_count.max(1)as f64);self.output.append_format(format_args!("q /GS{source} gs {} {} {} rg BT /F1 {} Tf 3 Tr {scale} Tz {} {} {} {} {} {} Tm <",text.color[0].clamp(0.0,1.0),text.color[1].clamp(0.0,1.0),text.color[2].clamp(0.0,1.0),text.record.size,m[0],m[1],m[2],m[3],m[4],m[5]),work)?;self.pdf_text_phase=4;return Ok(());}
                                else if self.pdf_text_phase==4{if let Some(scalar)=text.cluster.text[self.pdf_text_byte..].chars().next(){self.output.append_format(format_args!("{:08X}",scalar as u32),work)?;self.pdf_text_byte+=scalar.len_utf8();return Ok(());}self.pdf_text_phase=5;return Ok(());}
                                else{self.output.append(b"> Tj ET Q\n",work)?;self.pdf_text_phase=6;return Ok(());}
                            }else{self.pdf_text_phase=6;}
                        }
                        if self.pdf_primitive_cursor == 0 {
                            let m=projection::path_matrix(path.affine,f64::from(page.height));
                            self.output.append_format(format_args!("q {} {} {} {} {} {} cm /GS{source} gs 1 J 1 j\n",m[0],m[1],m[2],m[3],m[4],m[5]),work)?;
                            if let Some(color) = path.fill { self.output.append_format(format_args!("{} {} {} rg\n", color[0].clamp(0.0,1.0), color[1].clamp(0.0,1.0), color[2].clamp(0.0,1.0)),work)?; }
                            if let Some(color) = path.stroke { self.output.append_format(format_args!("{} {} {} RG {} w\n", color[0].clamp(0.0,1.0), color[1].clamp(0.0,1.0), color[2].clamp(0.0,1.0), path.stroke_width),work)?; }
                            self.pdf_primitive_cursor = 1; return Ok(());
                        }
                        if let Some(segment) = path.segments.get(self.pdf_primitive_cursor - 1) {
                            use semio_framework_2d_engine::PathSegment;
                            match segment {
                                PathSegment::Move{to}=>{self.output.append_format(format_args!("{} {} m\n",to[0],to[1]),work)?;self.pdf_path_current=*to;self.pdf_path_first=*to;},
                                PathSegment::Line{to}=>{self.output.append_format(format_args!("{} {} l\n",to[0],to[1]),work)?;self.pdf_path_current=*to;},
                                PathSegment::Quad{ctrl,to}=>{let from=self.pdf_path_current;let a=[from[0]+(ctrl[0]-from[0])*2.0/3.0,from[1]+(ctrl[1]-from[1])*2.0/3.0];let b=[to[0]+(ctrl[0]-to[0])*2.0/3.0,to[1]+(ctrl[1]-to[1])*2.0/3.0];self.output.append_format(format_args!("{} {} {} {} {} {} c\n",a[0],a[1],b[0],b[1],to[0],to[1]),work)?;self.pdf_path_current=*to;},
                                PathSegment::Cubic{ctrl1,ctrl2,to}=>{self.output.append_format(format_args!("{} {} {} {} {} {} c\n",ctrl1[0],ctrl1[1],ctrl2[0],ctrl2[1],to[0],to[1]),work)?;self.pdf_path_current=*to;},
                                PathSegment::Close=>{self.output.append(b"h\n",work)?;self.pdf_path_current=self.pdf_path_first;},
                                _=>return Err("layout-export-path-arc-unresolved".into()),
                            }
                            self.pdf_primitive_cursor += 1; return Ok(());
                        }
                        let fill = path.fill.is_some_and(|color| color[3] > 0.0); let stroke = path.stroke.is_some_and(|color| color[3] > 0.0);
                        self.output.append(match (fill,stroke) { (true,true) => b"B Q\n", (true,false) => b"f Q\n", (false,true) => b"S Q\n", (false,false) => b"n Q\n" },work)?;
                        if self.pdf_text_marked{let next=(source+1<page.first_item+page.items).then(||self.pdf_items.get(source+1)).flatten();if !pdf_same_text_cluster(self.pdf_items.get(source).unwrap(),next,self.text.as_ref(),self.drawing.as_ref())?{self.output.append(semantic_text::PDF_LAYOUT_TEXT_SUFFIX.as_bytes(),work)?;self.pdf_text_marked=false;}}
                        self.pdf_primitive_cursor = 0; self.pdf_text_phase = 0; self.pdf_text_byte = 0; self.pdf_text_count = 0; self.pdf_item_cursor += 1;
                    }
                }
            }
            PdfSection::ContentEnd => {
                self.pdf_xref_offset = self.output.len - self.pdf_content_start;
                self.output.append(b"\nendstream\nendobj\n",work)?;
                self.pdf_section = PdfSection::ContentLength;
            }
            PdfSection::ContentLength => {
                let id = Self::pdf_page_object_id(self.pdf_cursor) + 2;
                let length = self.pdf_xref_offset;
                self.pdf_object_format(id,format_args!("{length}"),work)?;
                self.pdf_cursor += 1;
                self.pdf_item_cursor = 0;
                self.pdf_section = if self.pdf_cursor < page_count { PdfSection::PageObject } else { PdfSection::ImageBegin };
            }
            PdfSection::ImageBegin => {
                let Some(item) = self.pdf_items.get(self.pdf_item_cursor) else { self.pdf_section = PdfSection::UnicodeBegin; return Ok(()); };
                let PdfItem::Image { raster: Some(image), .. } = item else { self.pdf_item_cursor += 1; return Ok(()); };
                let resource = image.resource.ok_or("layout-export-image-resource-missing")?; let id = 7 + 3 * page_count + 2 * resource;
                let len = raster::raster_length(image.width, image.height, &image.rgba, raster::RasterPlane::Rgb)?;
                let(width,height)=(image.width,image.height);self.pdf_object_offset(id,work)?;self.output.append_format(format_args!("{id} 0 obj\n<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB /BitsPerComponent 8 /SMask {} 0 R /Length {len} >>\nstream\n", width, height, id+1),work)?; self.pdf_primitive_cursor = 0; self.pdf_section = PdfSection::ImageRgb;
            }
            PdfSection::ImageRgb | PdfSection::ImageAlpha => {
                let PdfItem::Image { raster: Some(image), .. } = self.pdf_items.get(self.pdf_item_cursor).ok_or("layout-export-image-missing")? else { return Err("layout-export-image-unavailable".into()); };
                let plane = if self.pdf_section == PdfSection::ImageRgb { raster::RasterPlane::Rgb } else { raster::RasterPlane::Alpha };
                work.copy(4096,1)?;let mut bytes=[0u8;4096];let count=raster::raster_chunk(image.width,image.height,&image.rgba,plane,self.pdf_primitive_cursor,&mut bytes,work)?;
                if count==0 { self.pdf_section = if self.pdf_section == PdfSection::ImageRgb { PdfSection::ImageRgbEnd } else { PdfSection::ImageAlphaEnd }; }
                else { self.output.append(&bytes[..count],work)?; self.pdf_primitive_cursor += count; }
            }
            PdfSection::ImageRgbEnd => { self.output.append(b"\nendstream\nendobj\n",work)?; self.pdf_section = PdfSection::ImageAlphaBegin; }
            PdfSection::ImageAlphaBegin => {
                let PdfItem::Image { raster: Some(image), .. } = self.pdf_items.get(self.pdf_item_cursor).ok_or("layout-export-image-missing")? else { return Err("layout-export-image-unavailable".into()); };
                let resource = image.resource.ok_or("layout-export-image-resource-missing")?; let id = 8 + 3 * page_count + 2 * resource;
                let len = raster::raster_length(image.width, image.height, &image.rgba, raster::RasterPlane::Alpha)?;
                let(width,height)=(image.width,image.height);self.pdf_object_offset(id,work)?;self.output.append_format(format_args!("{id} 0 obj\n<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceGray /BitsPerComponent 8 /Length {len} >>\nstream\n", width,height),work)?; self.pdf_primitive_cursor = 0; self.pdf_section = PdfSection::ImageAlpha;
            }
            PdfSection::ImageAlphaEnd => { self.output.append(b"\nendstream\nendobj\n",work)?; self.pdf_item_cursor += 1; self.pdf_primitive_cursor = 0; self.pdf_section = PdfSection::ImageBegin; }
            PdfSection::UnicodeBegin => {
                let id = 7 + 3 * page_count + 2 * self.fidelity.as_ref().ok_or("layout-export-paint-plan-missing")?.image_count();
                self.pdf_object_offset(id,work)?; self.output.append_format(format_args!("{id} 0 obj\n<< /Length {} 0 R >>\nstream\n", id + 1),work)?;
                self.pdf_content_start = self.output.len; self.output.append(unicode::PDF_UNICODE_PREFIX.as_bytes(),work)?; self.pdf_primitive_cursor = 0; self.pdf_section = PdfSection::UnicodeRanges;
            }
            PdfSection::UnicodeRanges => {
                if let Some(range) = unicode::unicode_range(self.pdf_primitive_cursor) { self.output.append_format(format_args!("{range}"),work)?; self.pdf_primitive_cursor += 1; }
                else { self.pdf_section = PdfSection::UnicodeEnd; }
            }
            PdfSection::UnicodeEnd => {
                self.output.append(unicode::PDF_UNICODE_SUFFIX.as_bytes(),work)?; self.pdf_xref_offset = self.output.len - self.pdf_content_start;
                self.output.append(b"\nendstream\nendobj\n",work)?; self.pdf_section = PdfSection::UnicodeLength;
            }
            PdfSection::UnicodeLength => {
                let id = 8 + 3 * page_count + 2 * self.fidelity.as_ref().ok_or("layout-export-paint-plan-missing")?.image_count();
                let length=self.pdf_xref_offset;self.pdf_object_format(id,format_args!("{length}"),work)?; self.pdf_section = PdfSection::EncodingCmap;
            }
            PdfSection::EncodingCmap => {
                let id = 9 + 3 * page_count + 2 * self.fidelity.as_ref().ok_or("layout-export-paint-plan-missing")?.image_count();
                self.pdf_object_format(id,format_args!("<< /Length {} >>\nstream\n{}\nendstream", unicode::PDF_ENCODING_CMAP.len(), unicode::PDF_ENCODING_CMAP),work)?; self.pdf_section = PdfSection::XrefBegin;
            }
            PdfSection::XrefBegin => {
                self.pdf_xref_offset = self.output.len;
                self.output.append_format(format_args!("xref\n0 {}\n0000000000 65535 f \n", self.pdf_offsets.len() + 1),work)?;
                self.pdf_cursor = 0;
                self.pdf_section = PdfSection::XrefEntries;
            }
            PdfSection::XrefEntries => {
                if let Some(offset) = self.pdf_offsets.get(self.pdf_cursor).copied() {
                    self.output.append_format(format_args!("{offset:010} 00000 n \n"),work)?;
                    self.pdf_cursor += 1;
                } else {
                    self.pdf_section = PdfSection::Trailer;
                }
            }
            PdfSection::Trailer => {
                self.output.append_format(format_args!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n", self.pdf_offsets.len() + 1, self.pdf_xref_offset),work)?;
                self.pdf_section = PdfSection::Complete;
            }
            PdfSection::Complete => self.stage = ExportStage::Base64,
        }
        Ok(())
    }

    pub(super) fn begin_png(&mut self,work:&mut OutputWork) -> Result<(), String> {
        let page = self.page()?;
        let width = finite_dimension(page.width)?;
        let height = finite_dimension(page.height)?;
        self.output.append(b"\x89PNG\r\n\x1a\n",work)?;
        let mut ihdr=[0u8;13];ihdr[..4].copy_from_slice(&width.to_be_bytes());ihdr[4..8].copy_from_slice(&height.to_be_bytes());ihdr[8..].copy_from_slice(&[8,6,0,0,0]);
        self.png_chunk(b"IHDR", &ihdr,work)?;
        let row_bytes = width as usize * 4 + 1;
        let idat_len = 2usize.checked_add(height as usize * (row_bytes + 5)).and_then(|value| value.checked_add(4)).ok_or("layout-export-output-limit")?;
        self.output.append(&u32::try_from(idat_len).map_err(|_| "layout-export-output-limit")?.to_be_bytes(),work)?;
        self.output.append(b"IDAT",work)?;
        self.png_idat_crc = crc32_update(0, b"IDAT");
        self.append_idat(&[0x78, 0x01],work)
    }

    fn png_chunk(&mut self, kind: &[u8; 4], data: &[u8],work:&mut OutputWork) -> Result<(), String> {
        self.output.append(&u32::try_from(data.len()).map_err(|_| "layout-export-output-limit")?.to_be_bytes(),work)?;
        self.output.append(kind,work)?;
        self.output.append(data,work)?;
        self.output.append(&crc32_update(crc32_update(0, kind), data).to_be_bytes(),work)
    }

    fn append_idat(&mut self, bytes: &[u8],work:&mut OutputWork) -> Result<(), String> {
        self.png_idat_crc = crc32_update(self.png_idat_crc, bytes);
        self.output.append(bytes,work)
    }

    pub(super) fn encode_png_one(&mut self,work:&mut OutputWork) -> Result<(), String> {
        let (width, height) = {
            let page = self.page()?;
            (finite_dimension(page.width)?, finite_dimension(page.height)?)
        };
        if self.encode_cursor < height as usize {
            let y = self.encode_cursor as u32;
            match self.png_row_section {
                PngRowSection::Initialize => {
                    let row_len = width as usize * 4 + 1;
                    if !work.byte_buffer(&mut self.png_row,row_len)?{return Ok(());}
                    if self.png_row.is_empty(){work.copy(1+size_of::<usize>()*3,1)?;self.png_row.push(0);}
                    let end=self.png_row.len().saturating_add(OUTPUT_CHUNK_BYTES).min(row_len);
                    work.copy(end-self.png_row.len()+size_of::<usize>()*3,1)?;self.png_row.resize(end,255);
                    if end == row_len {
                        self.png_shape_cursor = 0;
                        self.png_fill_x = 0;
                        self.png_row_section = PngRowSection::Fill;
                    }
                    return Ok(());
                }
                PngRowSection::Fill => {
                    let fidelity = self.fidelity.as_mut().ok_or("layout-export-paint-plan-missing")?;
                    let Some(shape) = fidelity.shape(self.png_shape_cursor) else { self.png_row_section = PngRowSection::Header; return Ok(()); };
                    let (bounds, image) = match shape {
                        PaintShape::Path(path) => (path.world_bounds().unwrap_or([0.0;4]),None),
                        PaintShape::Image { source } => (faithful_primitives::image_bounds(&self.pdf_items[*source]).ok_or("layout-export-image-missing")?, Some(*source)),
                    };
                    let left = bounds[0].floor().max(0.0).min(f64::from(width)) as u32;
                    let right = bounds[2].ceil().max(0.0).min(f64::from(width)) as u32;
                    let x = self.png_fill_x.max(left);
                    if x >= right || f64::from(y + 1) < bounds[1] || f64::from(y) > bounds[3] {
                        self.png_shape_cursor += 1; self.png_fill_x = 0; fidelity.reset_sample(); return Ok(());
                    }
                    let colors = match image { Some(source) => Some((faithful_primitives::image_sample(&self.pdf_items[source], x, y), [0.0; 4])), None => fidelity.sample_path_one(self.png_shape_cursor, x, y) };
                    if let Some((fill, stroke)) = colors {
                        let index = x as usize * 4 + 1;
                        work.copy(8+size_of::<usize>()*3,1)?;
                        faithful_primitives::source_over(&mut self.png_row[index..index + 4], fill);
                        faithful_primitives::source_over(&mut self.png_row[index..index + 4], stroke);
                        self.png_fill_x = x + 1;
                    }
                    return Ok(());
                }
                PngRowSection::Header => {
                    let len = u16::try_from(self.png_row.len()).map_err(|_| "layout-export-png-row-limit")?;
                    let mut header = [0u8; 5];
                    header[0] = u8::from(self.encode_cursor + 1 == height as usize);
                    header[1..3].copy_from_slice(&len.to_le_bytes());
                    header[3..5].copy_from_slice(&(!len).to_le_bytes());
                    self.append_idat(&header,work)?;
                    self.png_row_byte_cursor = 0;
                    self.png_row_section = PngRowSection::Data;
                    return Ok(());
                }
                PngRowSection::Data => {
                    let end = self.png_row_byte_cursor.saturating_add(OUTPUT_CHUNK_BYTES).min(self.png_row.len());
                    if self.png_row_byte_cursor < end {
                        let bytes = &self.png_row[self.png_row_byte_cursor..end];
                        for byte in bytes {
                            self.png_adler_s1 = (self.png_adler_s1 + u32::from(*byte)) % 65_521;
                            self.png_adler_s2 = (self.png_adler_s2 + self.png_adler_s1) % 65_521;
                        }
                        self.png_idat_crc = crc32_update(self.png_idat_crc, bytes);
                        self.output.append(bytes,work)?;
                        self.png_row_byte_cursor = end;
                        return Ok(());
                    }
                    self.png_row.clear();
                    self.png_row_section = PngRowSection::Initialize;
                    self.png_row_byte_cursor = 0;
                    self.png_shape_cursor = 0;
                    self.encode_cursor += 1;
                    return Ok(());
                }
            }
        }
        let adler = (self.png_adler_s2 << 16) | self.png_adler_s1;
        self.append_idat(&adler.to_be_bytes(),work)?;
        self.output.append(&self.png_idat_crc.to_be_bytes(),work)?;
        self.png_chunk(b"IEND", &[],work)?;
        self.stage = ExportStage::Base64;
        Ok(())
    }

    pub(super) fn begin_zip_entry(&mut self,member:PackageMember,work:&mut OutputWork)->Result<(),String>{if zip::begin(&mut self.zip,&mut self.output,member,work)?{Ok(())}else{Err("layout-output-package-not-funded".into())}}
    fn append_zip_data(&mut self,bytes:&[u8],work:&mut OutputWork)->Result<(),String>{if zip::data(&mut self.zip,&mut self.output,bytes,work)?{Ok(())}else{Err("layout-output-package-not-funded".into())}}
    fn end_zip_entry(&mut self,work:&mut OutputWork)->Result<(),String>{if zip::finish(&mut self.zip,&mut self.output,work)?{Ok(())}else{Err("layout-output-package-not-funded".into())}}
    pub(super) fn encode_package_one(&mut self,work:&mut OutputWork)->Result<(),String>{match self.package_section{
        PackageSection::Begin=>{self.package_json=Some(TypedJsonCursor::document());self.package_section=PackageSection::Scalars},
        PackageSection::Scalars=>{let(bytes,done)=self.package_json.as_mut().ok_or("layout-export-json-cursor")?.advance(&self.request.as_ref().unwrap().snapshot)?;if !bytes.is_empty(){self.append_zip_data(&bytes,work)?}if done{self.package_json=None;self.end_zip_entry(work)?;self.begin_zip_entry(PackageMember::Preflight,work)?;self.package_byte_cursor=0;if self.request.as_ref().unwrap().preflight_json.is_none(){self.package_json=Some(TypedJsonCursor::preflight())}self.package_section=PackageSection::Preflight}},
        PackageSection::Preflight=>{if let Some(preflight)=&self.request.as_ref().unwrap().preflight_json{let end=self.package_byte_cursor.saturating_add(OUTPUT_CHUNK_BYTES).min(preflight.len());if self.package_byte_cursor<end{if !zip::data(&mut self.zip,&mut self.output,&preflight.as_bytes()[self.package_byte_cursor..end],work)?{return Err("layout-output-package-not-funded".into())}self.package_byte_cursor=end}else{self.finish_preflight_entry(work)?}return Ok(())}let(bytes,done)=self.package_json.as_mut().ok_or("layout-export-json-cursor")?.advance(&self.request.as_ref().unwrap().snapshot)?;if !bytes.is_empty(){self.append_zip_data(&bytes,work)?}if done{self.package_json=None;self.finish_preflight_entry(work)?}},
        PackageSection::Manifest=>{let(bytes,done)=self.package_json.as_mut().ok_or("layout-export-json-cursor")?.advance(&self.request.as_ref().unwrap().snapshot)?;if !bytes.is_empty(){self.append_zip_data(&bytes,work)?}if done{self.package_json=None;self.end_zip_entry(work)?;self.package_index=0;self.zip_central_offset=u32::try_from(self.output.len).map_err(|_|"layout-export-output-limit")?;self.package_section=PackageSection::CentralDirectory}},
        PackageSection::CentralDirectory=>{if let Some(entry)=self.zip.entry(self.package_index){if !zip::central(entry,&mut self.output,work)?{return Err("layout-output-package-not-funded".into())}self.package_index+=1}else{if !zip::end(&self.zip,&mut self.output,self.zip_central_offset,work)?{return Err("layout-output-package-not-funded".into())}self.package_section=PackageSection::Complete}},
        PackageSection::Complete=>self.stage=ExportStage::Base64,
    }Ok(())}
    fn finish_preflight_entry(&mut self,work:&mut OutputWork)->Result<(),String>{self.end_zip_entry(work)?;self.begin_zip_entry(PackageMember::Manifest,work)?;self.package_json=Some(TypedJsonCursor::manifest());self.package_index=0;self.package_section=PackageSection::Manifest;Ok(())}

}
//#endregion 🧩️Job
