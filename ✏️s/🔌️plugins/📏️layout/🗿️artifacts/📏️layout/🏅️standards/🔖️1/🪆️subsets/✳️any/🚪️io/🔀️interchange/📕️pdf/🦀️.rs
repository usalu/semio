//! 📕️ Editable PDF page/text projection and canonical PDF operator export.
use super::*;
#[path="🏷️text/🦀️.rs"]
mod marked_text;
#[path="🖍️paths/🦀️.rs"]mod paths;
use semio_framework_io_ports::{VectorScene,VectorCanvas,VectorLayer,VectorNode,VectorStyle,VectorRgba,VectorTransform};
pub use marked_text::PDF_LAYOUT_TEXT_TAG;
use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::{io, schema::snapshot::{PdfFont, PdfOp, PdfPage, PdfSnapshot, PdfTextString, PdfColorSpace, PdfImageBody, PdfObject, PdfImage, PdfExtGState, PdfDictEntry, PdfPropertyList}};
pub(super) fn import(bytes: &[u8], name: &str, control: &mut InterchangeControl<'_>,drawing_sources:&mut Vec<PendingDrawingSource>) -> Result<InterchangeResult<LayoutSnapshot>, String> {
    let pdf = io::import::decode_pdf(bytes).map_err(|error| error.to_string())?;
    if pdf.pages.len() > MAX_INTERCHANGE_PAGES { return Err("layout.interchange.page-limit".into()); }
    let mut doc = document(name);
    let mut diagnostics = vec![diagnostic("layout.pdf.editable-projection", "PDF pages, Unicode text, rectangular paint and supported raster XObjects are editable. Text matrices, font names/sizes and paint order are mapped for supported operators; embedded font programs, effects, annotations and unsupported image encodings require review. Nonzero cubic paths retain editable Drawing geometry; unsupported clipping, even-odd fills and stroke shapes refuse explicitly.")];
    for (index, source) in pdf.pages.iter().enumerate() {
        checkpoint(control, index, pdf.pages.len())?;
        let unit = source.user_unit.unwrap_or(1.0);
        let mut width = source.width() * unit;
        let mut height = source.height() * unit;
        if source.rotate.rem_euclid(180) != 0 { std::mem::swap(&mut width, &mut height); }
        check_dimensions(width, height)?;
        let mut page = page(format!("page-{}", index + 1), width, height);
        import_graphics(&pdf, index, &mut doc, &mut page, &mut diagnostics, control,drawing_sources)?;
        if source.rotate.rem_euclid(360)!=0 { let text=io::import::page_text(&pdf,index).map_err(|error|error.to_string())?;text_frame(&mut doc,&mut page,text); }
        doc.pages.push(page);
    }
    if doc.pages.is_empty() { diagnostics.push(diagnostic("layout.pdf.empty", "The PDF contains no pages.")); }
    finish_document(&mut doc);
    Ok(InterchangeResult { value: doc, fidelity: "editable-subset", diagnostics })
}
pub(super) fn export(doc: &LayoutSnapshot, control: &mut InterchangeControl<'_>) -> Result<InterchangeResult<Vec<u8>>, String> {
    let mut pdf = PdfSnapshot::default();
    let content = doc.stories.iter().map(|story| story.content.as_str()).collect::<Vec<_>>().join("\n");
    let font = io::export::embedded_true_type_font(&mut pdf, "F1", include_bytes!("../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️canvas/🖼️assets/🔤️MapLabelSans.ttf"), "MapLabelSans", Some(&content))?;
    let mut diagnostics = Vec::new();
    for (index, source) in doc.pages.iter().enumerate() {
        checkpoint(control, index, doc.pages.len())?;
        let mut page = PdfPage::new(source.width, source.height);
        for frame in &source.frames {
            if !frame.visible() || source.layers.iter().any(|layer| layer.id == frame.layer_id() && !layer.visible) { continue; }
            let b = frame.bounds();
            let angle = b.rotation;
            let (sin, cos) = angle.sin_cos();
            page.content.push(PdfOp::Save);
            let tx = b.x + b.width * 0.5 - cos * b.width * 0.5 + sin * b.height * 0.5;
            let ty = b.y + b.height * 0.5 - sin * b.width * 0.5 - cos * b.height * 0.5;
            page.content.push(PdfOp::Transform { matrix: [cos, -sin, sin, cos, tx, source.height - ty] });
            match frame {
                Frame::Rect { fill, stroke, .. } => {
                    if let Some(color) = fill { page.content.push(PdfOp::SetFillRgb { r: color[0] as f64, g: color[1] as f64, b: color[2] as f64 }); }
                    if let Some(color) = stroke { page.content.push(PdfOp::SetStrokeRgb { r: color[0] as f64, g: color[1] as f64, b: color[2] as f64 }); }
                    let state_id=format!("Alpha-{}-{}",index,pdf.ext_g_states.len());
                    pdf.ext_g_states.push(PdfExtGState{id:state_id.clone(),fill_alpha:fill.map(|color|color[3]as f64),stroke_alpha:stroke.map(|color|color[3]as f64),..PdfExtGState::default()});
                    page.content.push(PdfOp::SetExtGState{name:state_id});
                    page.content.push(PdfOp::Rectangle { x: 0.0, y: -b.height, width: b.width, height: b.height });
                    page.content.push(match (fill, stroke) { (Some(_), Some(_)) => PdfOp::FillStroke, (Some(_), None) => PdfOp::Fill, (None, Some(_)) => PdfOp::Stroke, _ => PdfOp::EndPath });

                },
                Frame::Text { story_id, inset, .. } => {
                    let story = doc.stories.iter().find(|story| &story.id == story_id).ok_or("layout.pdf.missing-story")?;
                    let style = story.style_runs.first().and_then(|run| run.paragraph_style_id.as_ref()).and_then(|id| doc.paragraph_styles.iter().find(|style| &style.id == id));
                    let size = style.map_or(12.0, |style| style.font_size);
                    let leading = style.map_or(size * 1.2, |style| style.leading);
                    page.content.extend([PdfOp::BeginText, PdfOp::SetFillGray { gray: 0.0 }, PdfOp::SetFont { name: "F1".into(), size }, PdfOp::SetLeading { leading }, PdfOp::MoveText { tx: inset.x, ty: -inset.y - size }]);
                    for (line, text) in story.content.split('\n').enumerate() {
                        if line > 0 { page.content.push(PdfOp::NextLine); }
                        page.content.push(PdfOp::ShowText { text: PdfTextString::text(text) });
                    }
                    page.content.push(PdfOp::EndText);
                },
                Frame::Image { link_id, .. } => {
                    let link = doc.links.iter().find(|link| &link.id == link_id).ok_or("layout.pdf.missing-link")?;
                    let source = link.proxy_data_url.as_deref().and_then(|value| value.strip_prefix("data:image/png;base64,")).ok_or("layout.pdf.image-requires-png-proxy")?;
                    let bytes = store::pack_rt::pack_value_from_base64(source).map_err(|error| error.to_string())?;
                    let image = semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::project_png(&bytes)?;
                    if u64::from(image.width) * u64::from(image.height) > 4_194_304 { return Err("layout.pdf.image-pixel-limit".into()); }
                    let mut rgb = Vec::with_capacity(image.pixels.len() / 4 * 3);
                    let mut alpha = Vec::with_capacity(image.pixels.len() / 4);
                    for pixel in image.pixels.chunks_exact(4) { rgb.extend_from_slice(&pixel[..3]); alpha.push(pixel[3]); }
                    let id = format!("Image-{}-{}", index, pdf.images.len());
                    let mut raster = PdfImage::rgb8(&id, image.width, image.height, rgb);
                    if alpha.iter().any(|&value| value != 255) { let mask_id = format!("Mask-{id}"); pdf.images.push(PdfImage::gray8(&mask_id, image.width, image.height, alpha)); raster.soft_mask = Some(mask_id); }
                    pdf.images.push(raster);
                    page.content.push(PdfOp::Transform { matrix: [b.width, 0.0, 0.0, b.height, 0.0, -b.height] });
                    page.content.push(PdfOp::PaintXObject { name: id });
                },
            }
            page.content.push(PdfOp::Restore);
        }
        pdf.pages.push(page);
    }
    pdf.fonts.push(font);
    diagnostics.push(diagnostic("layout.pdf.basic-font", "Interchange PDF embeds the shared MapLabelSans font and retains explicit line breaks. Use the editor PDF rendering job for shaped fonts, wrapping and parent-page objects. PNG image proxies retain their samples and alpha."));
    let value = io::export::encode_pdf(&pdf).map_err(|error| error.to_string())?;
    Ok(InterchangeResult { value, fidelity: "editable-subset", diagnostics })
}

#[derive(Clone)]
struct Graphics { matrix:[f64;6], fill:[f32;4], stroke:[f32;4],width:f64,stroke_flags:u8 }
fn multiply(a:[f64;6],b:[f64;6])->[f64;6]{[a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5]]}
fn bounds(matrix:[f64;6],x:f64,y:f64,w:f64,h:f64,page_height:f64,unit:f64)->Result<LayoutBounds,String>{
    let [a,b,c,d,e,f]=matrix;let sx=a.hypot(b);let sy=c.hypot(d);
    if sx<=0.0||sy<=0.0||(a*c+b*d).abs()>1e-8*sx*sy||a*d-b*c<=0.0{return Err("layout.pdf.skew-or-reflection".into());}
    let width=w.abs()*sx*unit;let height=h.abs()*sy*unit;
    let cx=(a*(x+w*0.5)+c*(y+h*0.5)+e)*unit;let cy=page_height-(b*(x+w*0.5)+d*(y+h*0.5)+f)*unit;
    Ok(LayoutBounds{x:cx-width*0.5,y:cy-height*0.5,width,height,rotation:(-b).atan2(a)})
}
fn image_bytes(pdf:&PdfSnapshot,image:&semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::snapshot::PdfImage)->Result<(String,Vec<u8>),String>{
    use io::foreign_artifacts::{NativePdfArtifactResources,PdfArtifactResourcePort};
    if image.image_mask||!image.decode.is_empty()||image.mask.is_some(){return Err("layout.pdf.image-mask-or-decode".into());}
    match &image.body{
        PdfImageBody::Artifact{reference}=>{
            if image.soft_mask.is_some(){return Err("layout.pdf.encoded-image-soft-mask".into());}
            if !reference.artifact_id.contains("s.stdio.jpeg:"){return Err("layout.pdf.image-codec".into());}
            let PdfObject::Stream{data,..}=NativePdfArtifactResources::from_objects(&pdf.objects).resolve(reference).map_err(|error|error.to_string())? else{return Err("layout.pdf.image-resource".into());};
            Ok(("image/jpeg".into(),data))
        },
        PdfImageBody::Samples{values}=>{
            use semio_s_artifact_stdio_png::{PngSnapshot,schema::snapshot::PngColorType,standards::v1_2::subsets::any::io::encode_png};
            if image.bits_per_component!=8{return Err("layout.pdf.image-bit-depth".into());}
            let components=match image.color_space.as_ref(){Some(PdfColorSpace::DeviceRgb)=>3,Some(PdfColorSpace::DeviceGray)=>1,_=>return Err("layout.pdf.image-colour-space".into())};
            if u64::from(image.width)*u64::from(image.height)>4_194_304{return Err("layout.pdf.image-pixel-limit".into());}
            let pixels=image.width as usize*image.height as usize;
            if values.len()!=pixels*components{return Err("layout.pdf.image-samples".into());}
            let alpha=if let Some(id)=&image.soft_mask{let mask=pdf.images.iter().find(|mask|&mask.id==id).ok_or("layout.pdf.missing-soft-mask")?;if mask.width!=image.width||mask.height!=image.height||mask.bits_per_component!=8||mask.color_space!=Some(PdfColorSpace::DeviceGray){return Err("layout.pdf.soft-mask-shape".into());}let PdfImageBody::Samples{values}=&mask.body else{return Err("layout.pdf.soft-mask-codec".into());};if values.len()!=pixels{return Err("layout.pdf.soft-mask-samples".into());}Some(values)}else{None};
            let mut samples=Vec::with_capacity(pixels*4);
            for (index,pixel) in values.chunks_exact(components).enumerate(){let channels=if components==3{[pixel[0],pixel[1],pixel[2],alpha.map_or(255,|values|values[index])]}else{[pixel[0],pixel[0],pixel[0],alpha.map_or(255,|values|values[index])]};for value in channels{if value>255{return Err("layout.pdf.image-sample".into());}samples.push(value as u16);}}
            let mut snapshot=PngSnapshot::default();snapshot.image.width=image.width;snapshot.image.height=image.height;snapshot.image.color_type=PngColorType::Rgba;snapshot.image.samples=samples;
            Ok(("image/png".into(),encode_png(&snapshot)?))
        }
    }
}
fn import_graphics(pdf:&PdfSnapshot,index:usize,doc:&mut LayoutSnapshot,page:&mut Page,diagnostics:&mut Vec<InterchangeDiagnostic>,control:&mut InterchangeControl<'_>,drawing_sources:&mut Vec<PendingDrawingSource>)->Result<(),String>{
    let source=&pdf.pages[index];if source.content.len()>MAX_INTERCHANGE_ITEMS{return Err("layout.interchange.item-limit".into());}
    if source.rotate.rem_euclid(360)!=0{diagnostics.push(diagnostic("layout.pdf.rotated-graphics","Rotated PDF page graphics require review; page text and dimensions are retained."));return Ok(());}
    let mut state=Graphics{matrix:[1.0,0.0,0.0,1.0,-source.media_box[0],-source.media_box[1]],fill:[0.0,0.0,0.0,1.0],stroke:[0.0,0.0,0.0,1.0],width:1.0,stroke_flags:15};
    let mut stack=Vec::new();let mut rectangles=Vec::new();let mut path=paths::PdfPath::default();let mut general_path=false;let mut drawing_nodes=Vec::new();let mut drawing_styles=Vec::new();let unit=source.user_unit.unwrap_or(1.0);
    let mut text_state=TextState::default();let mut semantic_text=marked_text::MarkedTextScope::default();
    let resources=io::foreign_artifacts::NativePdfArtifactResources::from_objects(&pdf.objects);
    let fonts=pdf.fonts.iter().map(|font|Ok((font.id.clone(),semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::modules::fonts::FontCodec::new(font,&resources).map_err(|error|error.to_string())?))).collect::<Result<std::collections::HashMap<_,_>,String>>()?;
    for (oi,op) in source.content.iter().enumerate(){
        if oi%64==0{checkpoint(control,oi,source.content.len())?;}
        let paint=match op{
            PdfOp::Save=>{stack.push(state.clone());None},PdfOp::Restore=>{state=stack.pop().ok_or("layout.pdf.graphics-stack")?;None},
            PdfOp::Transform{matrix}=>{state.matrix=multiply(state.matrix,*matrix);None},
            PdfOp::SetFillRgb{r,g,b}=>{state.fill=[*r as f32,*g as f32,*b as f32,state.fill[3]];None},PdfOp::SetStrokeRgb{r,g,b}=>{state.stroke=[*r as f32,*g as f32,*b as f32,state.stroke[3]];None},
            PdfOp::SetFillGray{gray}=>{state.fill=[*gray as f32,*gray as f32,*gray as f32,state.fill[3]];None},PdfOp::SetStrokeGray{gray}=>{state.stroke=[*gray as f32,*gray as f32,*gray as f32,state.stroke[3]];None},
            PdfOp::SetExtGState{name}=>{let ext=pdf.ext_g_state(name).ok_or("layout.pdf.missing-graphics-state")?;if let Some(width)=ext.line_width{state.width=width;}if let Some(cap)=ext.line_cap{state.stroke_flags=(state.stroke_flags&!1)|u8::from(cap==Default::default());}if let Some(join)=ext.line_join{state.stroke_flags=(state.stroke_flags&!2)|(u8::from(join==Default::default())*2);}if let Some(limit)=ext.miter_limit{state.stroke_flags=(state.stroke_flags&!4)|(u8::from((limit-10.0).abs()<1e-8)*4);}if let Some((dash,_))=&ext.dash{state.stroke_flags=(state.stroke_flags&!8)|(u8::from(dash.is_empty())*8);}if let Some(alpha)=ext.fill_alpha{state.fill[3]=alpha as f32;}if let Some(alpha)=ext.stroke_alpha{state.stroke[3]=alpha as f32;}if ext.soft_mask.is_some()||ext.blend_mode.as_ref().is_some_and(|modes|modes.iter().any(|mode|mode!="Normal")){diagnostics.push(diagnostic("layout.pdf.compositing","Non-normal blend modes and graphics soft masks require review."));}None},
            PdfOp::MoveTo{x,y}=>{general_path=true;path.move_to(state.matrix,*x,*y,page.height,unit)?;None},
            PdfOp::LineTo{x,y}=>{general_path=true;path.line_to(state.matrix,*x,*y,page.height,unit)?;None},
            PdfOp::CurveTo{x1,y1,x2,y2,x3,y3}=>{general_path=true;path.cubic(state.matrix,[*x1,*y1,*x2,*y2,*x3,*y3],page.height,unit)?;None},
            PdfOp::CurveToInitial{x2,y2,x3,y3}=>{general_path=true;path.initial(state.matrix,[*x2,*y2,*x3,*y3],page.height,unit)?;None},
            PdfOp::CurveToFinal{x1,y1,x3,y3}=>{general_path=true;path.final_curve(state.matrix,[*x1,*y1,*x3,*y3],page.height,unit)?;None},
            PdfOp::ClosePath=>{path.close()?;None},
            PdfOp::Rectangle{x,y,width,height}=>{rectangles.push((state.matrix,*x,*y,*width,*height));path.rectangle(state.matrix,[*x,*y,*width,*height],page.height,unit)?;None},
            PdfOp::SetLineWidth{width}=>{state.width=*width;None},
            PdfOp::SetLineCap{cap}=>{state.stroke_flags=(state.stroke_flags&!1)|u8::from(*cap==Default::default());None},
            PdfOp::SetLineJoin{join}=>{state.stroke_flags=(state.stroke_flags&!2)|(u8::from(*join==Default::default())*2);None},
            PdfOp::SetMiterLimit{limit}=>{state.stroke_flags=(state.stroke_flags&!4)|(u8::from((*limit-10.0).abs()<1e-8)*4);None},
            PdfOp::SetDash{array,..}=>{state.stroke_flags=(state.stroke_flags&!8)|(u8::from(array.is_empty())*8);None},
            PdfOp::Clip|PdfOp::ClipEvenOdd=>{let page_clip=!general_path&&rectangles.len()==1&&{let(m,x,y,w,h)=rectangles[0];bounds(m,x,y,w,h,page.height,unit).is_ok_and(|b|b.rotation.abs()<1e-8&&b.x.abs()<1e-8&&b.y.abs()<1e-8&&(b.width-page.width).abs()<1e-8&&(b.height-page.height).abs()<1e-8)};if !page_clip{return Err("layout.pdf.clip-not-representable".into());}None},
            PdfOp::Fill|PdfOp::FillEvenOdd=>Some((true,false)),PdfOp::Stroke|PdfOp::CloseStroke=>Some((false,true)),PdfOp::FillStroke|PdfOp::FillStrokeEvenOdd|PdfOp::CloseFillStroke|PdfOp::CloseFillStrokeEvenOdd=>Some((true,true)),
            PdfOp::EndPath=>{rectangles.clear();path.take();general_path=false;None},
            PdfOp::BeginMarkedContent{tag}=>{semantic_text.begin(tag)?;None},
            PdfOp::BeginMarkedContentWithProperties{tag,properties}=>{let entries=match properties{PdfPropertyList::Inline{entries}=>Some(entries.as_slice()),PdfPropertyList::Named{name}=>pdf.properties.iter().find(|property|&property.name==name).map(|property|property.entries.as_slice())};semantic_text.begin_with_style(tag,entries)?;None},
            PdfOp::EndMarkedContent=>{semantic_text.end()?;None},
            PdfOp::BeginText=>{text_state.matrix=IDENTITY;text_state.line=IDENTITY;None},
            PdfOp::SetFont{name,size}=>{text_state.font=name.clone();text_state.size=*size;None},
            PdfOp::SetLeading{leading}=>{text_state.leading=*leading;None},
            PdfOp::SetCharSpacing{spacing}=>{text_state.spacing=*spacing;None},
            PdfOp::SetWordSpacing{spacing}=>{text_state.word_spacing=*spacing;None},
            PdfOp::SetHorizontalScale{scale}=>{text_state.scale=*scale/100.0;None},
            PdfOp::SetTextRise{rise}=>{text_state.rise=*rise;None},
            PdfOp::SetTextRenderingMode{mode}=>{text_state.mode=*mode;None},
            PdfOp::SetTextMatrix{matrix}=>{text_state.matrix=*matrix;text_state.line=*matrix;None},
            PdfOp::MoveText{tx,ty}=>{text_state.move_line(*tx,*ty);None},
            PdfOp::MoveTextSetLeading{tx,ty}=>{text_state.leading=-*ty;text_state.move_line(*tx,*ty);None},
            PdfOp::NextLine=>{text_state.move_line(0.0,-text_state.leading);None},
            PdfOp::ShowText{text}=>{show_text(pdf,doc,page,&state,&mut text_state,text,&fonts,unit,&mut semantic_text,diagnostics)?;None},
            PdfOp::NextLineShowText{text}=>{text_state.move_line(0.0,-text_state.leading);show_text(pdf,doc,page,&state,&mut text_state,text,&fonts,unit,&mut semantic_text,diagnostics)?;None},
            PdfOp::NextLineShowTextSpaced{word_spacing,char_spacing,text}=>{text_state.word_spacing=*word_spacing;text_state.spacing=*char_spacing;text_state.move_line(0.0,-text_state.leading);show_text(pdf,doc,page,&state,&mut text_state,text,&fonts,unit,&mut semantic_text,diagnostics)?;None},
            PdfOp::ShowTextArray{items}=>{
                use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::snapshot::PdfTextArrayItem;
                for item in items{match item{PdfTextArrayItem::Text{text}=>show_text(pdf,doc,page,&state,&mut text_state,&PdfTextString::text(text),&fonts,unit,&mut semantic_text,diagnostics)?,PdfTextArrayItem::Codes{codes}=>show_text(pdf,doc,page,&state,&mut text_state,&PdfTextString::Codes{codes:codes.clone()},&fonts,unit,&mut semantic_text,diagnostics)?,PdfTextArrayItem::Adjust{amount}=>text_state.advance(-amount*text_state.size*text_state.scale/1000.0)}}None
            },
            PdfOp::PaintXObject{name}=>{
                if let Some(image)=pdf.images.iter().find(|image|&image.id==name){
                    match image_bytes(pdf,image).and_then(|(mime,bytes)|Ok((mime,bytes,bounds(state.matrix,0.0,0.0,1.0,1.0,page.height,unit)?))){
                        Ok((mime,bytes,bounds))=>{
                            let id=format!("{}-image-{oi}",page.id);let link_id=format!("link-{id}");
                            let payload=store::pack_rt::pack_value_to_base64(&bytes);doc.links.push(semio_s_artifact_layout_layout_model::ImageLink{id:link_id.clone(),path:format!("{id}.{}",if mime=="image/png"{"png"}else{"jpg"}),hash:String::new(),width:image.width,height:image.height,dpi:72,color_profile:None,state:Some("ok".into()),proxy_data_url:Some(format!("data:{mime};base64,{payload}")),artifact_kind:if mime=="image/png"{"s.stdio.png".into()}else{"s.stdio.jpg".into()},artifact_ref:String::new()});
                            page.layers[0].object_ids.push(id.clone());page.frames.push(Frame::Image{id,layer_id:page.layer_ids[0].clone(),bounds,link_id,locked:None,visible:None});
                        },Err(error)=>diagnostics.push(diagnostic("layout.pdf.image",&error)),
                    }
                }else{diagnostics.push(diagnostic("layout.pdf.form-xobject","Form XObject content is not reconstructed."));}None
            },
            _=>None,
        };
        if let Some((fill,stroke))=paint{
            if matches!(op,PdfOp::CloseStroke|PdfOp::CloseFillStroke|PdfOp::CloseFillStrokeEvenOdd){path.close()?;}
            if semantic_text.owns_paint(){rectangles.clear();path.take();general_path=false;continue;}
            let sx=state.matrix[0].hypot(state.matrix[1]);let sy=state.matrix[2].hypot(state.matrix[3]);let stroke_width=state.width*sx*unit;
            if stroke&&(state.stroke_flags!=15||!stroke_width.is_finite()||stroke_width<=0.0||(sx-sy).abs()>1e-8*sx.max(sy)||(state.matrix[0]*state.matrix[2]+state.matrix[1]*state.matrix[3]).abs()>1e-8*sx*sy){return Err("layout.pdf.stroke-not-representable".into());}
            if fill&&matches!(op,PdfOp::FillEvenOdd|PdfOp::FillStrokeEvenOdd|PdfOp::CloseFillStrokeEvenOdd){return Err("layout.pdf.even-odd-not-representable".into());}
            if general_path||(stroke&&(stroke_width-1.0).abs()>1e-8){
                if !path.is_empty(){let name=format!("pdf-paint-{oi}");let color=|c:[f32;4]|VectorRgba{r:c[0],g:c[1],b:c[2],a:c[3]};drawing_styles.push(VectorStyle{name:name.clone(),fill:fill.then_some(color(state.fill)),stroke:stroke.then_some(color(state.stroke)),stroke_width:stroke.then_some(stroke_width),opacity:None});drawing_nodes.push(VectorNode::Path{segments:path.take(),style:Some(name)});}
                rectangles.clear();general_path=false;continue;
            }
            path.take();general_path=false;
            for (matrix,x,y,w,h) in rectangles.drain(..){
                match bounds(matrix,x,y,w,h,page.height,unit){
                    Ok(bounds)if bounds.width>0.0&&bounds.height>0.0=>{let id=format!("{}-rect-{oi}-{}",page.id,page.frames.len());page.layers[0].object_ids.push(id.clone());page.frames.push(Frame::Rect{id,layer_id:page.layer_ids[0].clone(),bounds,fill:fill.then_some(state.fill),stroke:stroke.then_some(state.stroke),locked:None,visible:None});},
                    Ok(_)=>{},Err(error)=>diagnostics.push(diagnostic("layout.pdf.rectangle",&error)),
                }
            }
        }
        if page.frames.len()>MAX_INTERCHANGE_ITEMS{return Err("layout.interchange.item-limit".into());}
    }semantic_text.finish()?;if !drawing_nodes.is_empty(){let scene=VectorScene{schema:String::new(),canvas:VectorCanvas{width:page.width,height:page.height,background:None},styles:drawing_styles,layers:vec![VectorLayer{id:"pdf-paint".into(),name:"PDF paint".into(),visible:true,root:VectorNode::Group{transform:VectorTransform::identity(),children:drawing_nodes}}]};drawing_sources.push(PendingDrawingSource{source:semio_s_artifact_layout_layout_model::LayoutDrawingBirthSource::Page{page_id:page.id.clone()},scene});diagnostics.push(diagnostic("layout.pdf.path-order","PDF paths retain exact cubic controls and page coordinates as an editable Drawing child. The Drawing child paints below native frames; interleaved path/frame overlap requires review."));}Ok(())
}

const IDENTITY:[f64;6]=[1.0,0.0,0.0,1.0,0.0,0.0];
struct TextState{matrix:[f64;6],line:[f64;6],font:String,size:f64,leading:f64,spacing:f64,word_spacing:f64,scale:f64,rise:f64,mode:u32}
impl Default for TextState{fn default()->Self{Self{matrix:IDENTITY,line:IDENTITY,font:String::new(),size:12.0,leading:14.4,spacing:0.0,word_spacing:0.0,scale:1.0,rise:0.0,mode:0}}}
impl TextState{
    fn move_line(&mut self,x:f64,y:f64){self.line=multiply(self.line,[1.0,0.0,0.0,1.0,x,y]);self.matrix=self.line;}
    fn advance(&mut self,x:f64){self.matrix=multiply(self.matrix,[1.0,0.0,0.0,1.0,x,0.0]);}
}
fn show_text(pdf:&PdfSnapshot,doc:&mut LayoutSnapshot,page:&mut Page,graphics:&Graphics,state:&mut TextState,value:&PdfTextString,fonts:&std::collections::HashMap<String,semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::modules::fonts::FontCodec>,unit:f64,semantic_scope:&mut marked_text::MarkedTextScope<'_>,diagnostics:&mut Vec<InterchangeDiagnostic>)->Result<(),String>{
    let codec=fonts.get(&state.font).ok_or("layout.pdf.missing-font")?;
    let (content,width)=match value{
        PdfTextString::Text{text}=>(text.clone(),codec.text_width(text).ok_or("layout.pdf.unencodable-text")?),
        PdfTextString::Codes{codes}=>{let glyphs=codec.decode_codes(codes);let content=glyphs.iter().filter_map(|glyph|glyph.text.as_deref()).collect::<String>();if glyphs.iter().any(|glyph|glyph.text.is_none()){diagnostics.push(diagnostic("layout.pdf.unmapped-glyph","A source glyph has no Unicode mapping."));}(content,glyphs.iter().map(|glyph|glyph.width).sum())}
    };
    let advance=(width*state.size/1000.0+state.spacing*content.chars().count()as f64+state.word_spacing*content.chars().filter(|&c|c==' ').count()as f64)*state.scale;
    let mut editable=false;let promote_hidden=semantic_scope.active();let semantic_style=semantic_scope.entries();
    if !content.is_empty()&&(state.mode!=3||promote_hidden){
        let m=multiply(graphics.matrix,state.matrix);
        let geometry=if marked_text::editable_matrix(m,state.scale,promote_hidden){bounds(m,0.0,state.rise-state.size*0.2,advance,state.size,page.height,unit)}else{Err("layout.pdf.unsupported-font-shape".into())};
        match geometry{
            Ok(bounds)if bounds.width>0.0&&bounds.height>0.0=>{
                editable=true;let id=format!("{}-text-{}",page.id,page.frames.len());let story_id=format!("story-{id}");let style_id=format!("style-{id}");
                let property=|name:&str|semantic_style.and_then(|entries|entries.iter().find(|entry|entry.key==name)).map(|entry|&entry.value);let semantic_family=property("SemioFontFamily").and_then(|value|match value{PdfObject::Str(bytes)=>Some(io::text_string::decode_text_string(bytes)),PdfObject::Text(text)=>Some(text.clone()),_=>None});let weight=property("SemioFontWeight").and_then(PdfObject::as_i64).and_then(|weight|u32::try_from(weight).ok()).filter(|weight|*weight>0).unwrap_or(400);let italic=property("SemioItalic").and_then(PdfObject::as_bool);
                let family=semantic_family.unwrap_or_else(||pdf.fonts.iter().find(|font|font.id==state.font).map(|font|font.base_font().to_string()).unwrap_or_else(||"Helvetica".into()));
                doc.paragraph_styles.push(semio_s_artifact_layout_layout_model::ParagraphStyle{id:style_id.clone(),name:family.clone(),font_family:family,font_size:state.size*m[2].hypot(m[3])*unit,font_weight:weight,leading:state.leading*m[2].hypot(m[3])*unit,tracking:state.spacing,alignment:"left".into()});
                let character_id=format!("colour-{id}");doc.character_styles.push(semio_s_artifact_layout_layout_model::CharacterStyle{id:character_id.clone(),name:None,font_family:None,font_size:None,font_weight:None,italic,color:Some(graphics.fill),tracking:None});
                doc.stories.push(TextStory{id:story_id.clone(),content:content.clone(),style_runs:vec![semio_s_artifact_layout_layout_model::TextStyleRun{start:0,end:content.len()as u64,paragraph_style_id:Some(style_id),character_style_id:Some(character_id)}]});
                page.layers[0].object_ids.push(id.clone());page.frames.push(Frame::Text{id,layer_id:page.layer_ids[0].clone(),bounds,story_id,thread_next:None,columns:1,inset:LayoutRect{x:0.0,y:0.0,width:0.0,height:0.0},wrap_mode:"none".into(),locked:None,visible:None});
            },Ok(_)=>{},Err(error)=>diagnostics.push(diagnostic("layout.pdf.text-matrix",&error)),
        }
    }
    state.advance(advance);semantic_scope.record_promotion(editable);Ok(())
}
