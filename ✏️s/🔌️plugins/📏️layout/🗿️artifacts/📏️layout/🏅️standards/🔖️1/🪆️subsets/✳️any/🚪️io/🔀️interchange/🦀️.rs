//! 🔀️ Layout interchange owns semantic mapping; canonical stdio codecs own file syntax.
use semio_s_artifact_layout_layout_model::{snapshot::LayoutSnapshot, Frame, GridSettings, Layer, LayoutBounds, LayoutRect, Page, PageColumns, PageMargins, Spread, TextStory, LAYOUT_DOCUMENT_SCHEMA};
#[path = "📕️pdf/🦀️.rs"] mod pdf;
pub use pdf::PDF_LAYOUT_TEXT_TAG;
#[path = "📐️step/🦀️.rs"] mod step;
#[path = "📦️idml/🦀️.rs"] mod idml;

/// 📦️ Cold syntax analysis retains every drawing source beside an unpublished parent draft.
#[derive(Debug,PartialEq)]
pub struct PendingDrawingSource{pub source:semio_s_artifact_layout_layout_model::LayoutDrawingBirthSource,pub scene:semio_framework_io_ports::VectorScene}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum LayoutExternalDrawingFormat{Svg,Dxf,Dwg}
#[derive(Debug,PartialEq)]
pub enum LayoutImportSourcePlan{Document{document:LayoutSnapshot,drawing_sources:Vec<PendingDrawingSource>},External{format:LayoutExternalDrawingFormat,source:Vec<u8>,name:String}}


pub const MAX_INTERCHANGE_BYTES: usize = 8 << 20;
pub const MAX_INTERCHANGE_PAGES: usize = 64;
pub const MAX_INTERCHANGE_ITEMS: usize = 16_384;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InterchangeDiagnostic { pub code: String, pub message: String }
#[derive(Clone, Debug, PartialEq)]
pub struct InterchangeResult<T> { pub value: T, pub fidelity: &'static str, pub diagnostics: Vec<InterchangeDiagnostic> }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InterchangeProgress { pub completed: usize, pub total: usize }
pub type InterchangeControl<'a> = dyn FnMut(InterchangeProgress) -> bool + 'a;
fn checkpoint(control: &mut InterchangeControl<'_>, completed: usize, total: usize) -> Result<(), String> {
    if control(InterchangeProgress { completed, total }) { Ok(()) } else { Err("layout.interchange.cancelled".into()) }
}
fn diagnostic(code: &str, message: &str) -> InterchangeDiagnostic { InterchangeDiagnostic { code: code.into(), message: message.into() } }
fn check_dimensions(width: f64, height: f64) -> Result<(), String> {
    if width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0 && width <= 1e7 && height <= 1e7 { Ok(()) } else { Err("layout.interchange.page-dimensions".into()) }
}
fn document(name: &str) -> LayoutSnapshot {
    LayoutSnapshot { schema: LAYOUT_DOCUMENT_SCHEMA.into(), name: name.into(), grid: GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false }, paragraph_styles: Vec::new(), character_styles: Vec::new(), stories: Vec::new(), links: Vec::new(), parent_pages: Vec::new(), spreads: Vec::new(), pages: Vec::new(), print_target: None, data_fields: None, background_drawing: None, referenced_model: None }
}
fn page(id: String, width: f64, height: f64) -> Page {
    let layer_id = format!("layer-{id}");
    Page { background_drawing:None,name: id.clone(), id, spread_id: "spread-1".into(), parent_page_id: None, width, height, margins: PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 }, columns: PageColumns { count: 1, gutter: 0.0 }, guides: Vec::new(), layer_ids: vec![layer_id.clone()], layers: vec![Layer { id: layer_id, name: "Content".into(), visible: true, locked: false, object_ids: Vec::new() }], frames: Vec::new(), overrides: Vec::new() }
}
fn finish_document(doc: &mut LayoutSnapshot) {
    doc.spreads = vec![Spread { id: "spread-1".into(), name: "Spread 1".into(), page_ids: doc.pages.iter().map(|page| page.id.clone()).collect() }];
}
fn text_frame(doc: &mut LayoutSnapshot, page: &mut Page, content: String) {
    if content.is_empty() { return; }
    let story_id = format!("story-{}", page.id);
    let frame_id = format!("text-{}", page.id);
    doc.stories.push(TextStory { id: story_id.clone(), content, style_runs: Vec::new() });
    page.layers[0].object_ids.push(frame_id.clone());
    page.frames.push(Frame::Text { id: frame_id, layer_id: page.layer_ids[0].clone(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: page.width, height: page.height, rotation: 0.0 }, story_id, thread_next: None, columns: 1, inset: LayoutRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }, wrap_mode: "none".into(), locked: None, visible: None });
}
fn admit_document(doc: &LayoutSnapshot) -> Result<(), String> {
    if doc.pages.len() > MAX_INTERCHANGE_PAGES || doc.pages.iter().map(|page| page.frames.len()).sum::<usize>() > MAX_INTERCHANGE_ITEMS { return Err("layout.interchange.item-limit".into()); }
    let bytes=doc.stories.iter().map(|story|story.content.len()).chain(doc.links.iter().map(|link|link.proxy_data_url.as_ref().map_or(0,String::len))).try_fold(0usize,|total,length|total.checked_add(length)).ok_or("layout.interchange.byte-limit")?;
    if bytes>MAX_INTERCHANGE_BYTES{return Err("layout.interchange.byte-limit".into());}
    for page in &doc.pages { check_dimensions(page.width, page.height)?; }
    Ok(())
}
pub fn import_document(format: &str, bytes: &[u8], name: &str, control: &mut InterchangeControl<'_>) -> Result<InterchangeResult<LayoutImportSourcePlan>, String> {
    if bytes.len() > MAX_INTERCHANGE_BYTES { return Err("layout.interchange.byte-limit".into()); }
    checkpoint(control, 0, bytes.len())?;
    if matches!(format,"svg"|"dxf"|"dwg"){let value=crate::layout_import_sources_from_format(format,bytes,name,control)?;checkpoint(control,bytes.len(),bytes.len())?;return Ok(InterchangeResult{value,fidelity:"external-source-plan",diagnostics:vec![diagnostic("layout.import.preparation-required","Original external drawing source requires genuine target identity and retained parser/mapping preparation before publication.")]})}
    let mut drawing_sources=Vec::new();
    let result = match format {
        "pdf" => pdf::import(bytes, name, control,&mut drawing_sources),
        "step" | "stp" => step::import(bytes, name, control,&mut drawing_sources),
        "idml" => idml::import(bytes, name, control),
        "json" => {
            let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
            let value = crate::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(text).map_err(|error| error.to_string())?;
            Ok(InterchangeResult { value, fidelity: "native", diagnostics: Vec::new() })
        },
        _ => Err(format!("layout.interchange.unknown-format:{format}")),
    }?;
    admit_document(&result.value)?;
    checkpoint(control, bytes.len(), bytes.len())?;
    Ok(InterchangeResult{value:LayoutImportSourcePlan::Document{document:result.value,drawing_sources},fidelity:result.fidelity,diagnostics:result.diagnostics})
}
pub fn export_document(format: &str, doc: &LayoutSnapshot, control: &mut InterchangeControl<'_>) -> Result<InterchangeResult<Vec<u8>>, String> {
    admit_document(doc)?;
    checkpoint(control, 0, doc.pages.len())?;
    let result = match format {
        "pdf" => pdf::export(doc, control),
        "step" | "stp" => step::export(doc, control),
        "idml" => idml::export(doc, control),
        "json" => Ok(InterchangeResult { value: semio_framework_pack_json::to_json_string(doc).into_bytes(), fidelity: "native", diagnostics: Vec::new() }),
        "svg" | "dxf" | "dwg" | "png" => Ok(InterchangeResult { value: crate::layout_snapshot_to_format(doc, format)?, fidelity: "projection", diagnostics: vec![diagnostic("layout.export.trace", "Page content, basic text and embedded image proxies exported as drawing primitives; use the editor rendering jobs for shaped typography.")] }),
        _ => Err(format!("layout.interchange.unknown-format:{format}")),
    }?;
    if result.value.len() > MAX_INTERCHANGE_BYTES { return Err("layout.interchange.byte-limit".into()); }
    checkpoint(control, doc.pages.len(), doc.pages.len())?;
    Ok(result)
}
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"] mod tests;

#[path = "🧵️export/🦀️.rs"]
mod export_session;
pub use export_session::{InterchangeExportAdvance,InterchangeExportSession};

#[path="🪆️drawing/🦀️.rs"]
mod drawing;
pub use drawing::{lift_vector_scene,DrawingBirthPreparation};

#[cfg(test)]#[path="🧭️source-plan/🧪️tests/🦀️.rs"]mod source_plan_tests;
