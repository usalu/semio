//! ⚙️ Layout engine and views of the layout scene: the layout engine state, its story layout, the paragraph defaults, the interactive and per page display list builders and the interactive view and layer visibility checks. See [`super`].

use super::*;

//#region ⚙️Scene
pub struct LayoutEngine {
    pub(super) text: TextSystem,
    pub(super) font: FontDependencyId,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine {
    /// 🏗️ An engine over the built-in text fonts a host installed with [`install_text_fonts`]. A native process that
    /// installed nothing resolves the catalog once through [`install_native_text_fonts`]; a `wasm32` guest that
    /// installed nothing shapes with the layout font alone, because it has no file system to read them from.
    pub fn new() -> Self {
        Self::from_text_system(Self::built_in_text_system())
    }

    /// 🏗️ An engine over exactly `fonts`, independent of the process-wide slot.
    pub fn with_text_fonts(fonts: &TextFontSet<'_>) -> Result<Self, TextFontError> {
        TextSystem::new(fonts).map(Self::from_text_system)
    }

    fn built_in_text_system() -> TextSystem {
        #[cfg(not(target_arch = "wasm32"))]
        {
            static NATIVE_DOOR: std::sync::Once = std::sync::Once::new();
            NATIVE_DOOR.call_once(|| {
                if let Err(error) = install_native_text_fonts(None) {
                    if !matches!(error, TextFontsInstallError::AlreadyInstalled) {
                        eprintln!("[TRACE] layout engine built-in text fonts unavailable: {error}; stories shape with the layout font alone");
                    }
                }
            });
        }
        match TEXT_FONTS.get() {
            Some(fonts) => TextSystem::new(fonts).expect("installed text fonts were registered when they were installed"),
            None => TextSystem::without_builtin_fonts(),
        }
    }

    /// 🏗️ Registers the embedded `LAYOUT_SANS` font under [`TextSystem`]'s
    /// request/provide-bytes seam synchronously — never `Pending`, since the bytes are already in
    /// the binary (`include_bytes!`), unlike a real host-fetched [`FontSource`].
    pub(super) fn from_text_system(mut text: TextSystem) -> Self {
        let font = text.request_font("layout-sans");
        text.provide_font_bytes(font, LAYOUT_SANS);
        Self { text, font }
    }

    pub fn layout_story(&mut self, story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (ShapedText, bool) {
        let style = TextRunStyle {
            base: TextStyle { family: FontFamilyChoice::Custom(self.font), size_px: paragraph.font_size as f32 },
            weight: paragraph.font_weight as f32,
            line_height_relative: (paragraph.leading / paragraph.font_size.max(1.0)) as f32,
            letter_spacing_px: paragraph.tracking as f32,
            alignment: alignment_from_str(&paragraph.alignment),
        };
        let shaped = self.text.shape_paragraph(&story.content, &style, frame_width);
        let overset = shaped.height > frame_height;
        (shaped, overset)
    }
}

fn alignment_from_str(value: &str) -> TextAlignment {
    match value {
        "center" | "middle" => TextAlignment::Middle,
        "right" => TextAlignment::Right,
        "justify" | "justified" => TextAlignment::Justified,
        _ => TextAlignment::Left,
    }
}

pub(super) fn default_paragraph(doc: &LayoutSnapshot) -> ParagraphStyle {
    doc.paragraph_styles.first().cloned().unwrap_or(ParagraphStyle { id: "paragraph.body".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: "left".into() })
}

pub fn layout_story_in_frame(engine: &mut LayoutEngine, story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (ShapedText, bool) {
    engine.layout_story(story, paragraph, frame_width, frame_height)
}

/// 👁️ Live canvas fidelity. Below this zoom the page is a proxy: frame chrome and image
/// proxies, no story strings. At or above it, stories are emitted as text runs without shaping.
pub const INTERACTIVE_TEXT_ZOOM: f64 = 0.35;

/// 🖼️ Host-canvas display list. Never shapes paragraphs and never loads a font.
pub fn build_interactive_display_list(doc: &LayoutSnapshot, page: &Page, active_page_id: &str, selected_ids: &[String], hovered_id: Option<&str>, chrome_blueprint: bool, camera_x: f64, camera_y: f64, zoom: f64) -> DisplayList {
    assemble_display_list(None, doc, page, active_page_id, selected_ids, hovered_id, chrome_blueprint, zoom < INTERACTIVE_TEXT_ZOOM, camera_x, camera_y, zoom, false)
}

pub fn build_display_list_for_page(engine: &mut LayoutEngine, doc: &LayoutSnapshot, page: &Page, active_page_id: &str, selected_ids: &[String], hovered_id: Option<&str>, chrome_blueprint: bool) -> DisplayList {
    assemble_display_list(Some(engine), doc, page, active_page_id, selected_ids, hovered_id, chrome_blueprint, false, 0.0, 0.0, 1.0, false)
}

const INTERACTIVE_VIEW_SPAN: f64 = 2000.0;

pub(super) fn outside_interactive_view(bounds: &LayoutBounds, camera_x: f64, camera_y: f64, zoom: f64) -> bool {
    let span = INTERACTIVE_VIEW_SPAN / zoom.max(0.05);
    let x1 = bounds.x + bounds.width;
    let y1 = bounds.y + bounds.height;
    x1 < camera_x - span || bounds.x > camera_x + span || y1 < camera_y - span || bounds.y > camera_y + span
}

pub(super) fn frame_layer_visible(doc: &LayoutSnapshot, page: &Page, frame: &Frame) -> bool {
    let layer_id = frame.layer_id();
    if let Some(layer) = page.layers.iter().find(|layer| layer.id == layer_id) {
        return layer.visible;
    }
    if let Some(parent_id) = &page.parent_page_id {
        if let Some(parent) = doc.parent_pages.iter().find(|parent| parent.id == *parent_id) {
            if let Some(layer) = parent.layers.iter().find(|layer| layer.id == layer_id) {
                return layer.visible;
            }
        }
    }
    true
}
//#endregion ⚙️Scene

/// 🧱️ Shapes exactly one resolved frame for resumable export planning.
pub fn build_display_list_for_resolved_frame(engine: &mut LayoutEngine, doc: &LayoutSnapshot, page: &Page, frame: semio_s_artifact_layout_layout_model::ResolvedFrame) -> DisplayList {
    super::display_list_assembly::assemble_display_list_items(Some(engine), doc, page, "", &[], None, false, false, 0.0, 0.0, 1.0, vec![frame], false, true, false)
}

/// 🌄️ Shapes the background once before the frame-by-frame export plan.
pub fn build_display_list_for_page_background(engine: &mut LayoutEngine, doc: &LayoutSnapshot, page: &Page) -> DisplayList {
    super::display_list_assembly::assemble_display_list_items(Some(engine), doc, page, "", &[], None, false, false, 0.0, 0.0, 1.0, Vec::new(), true, true, false)
}

impl LayoutEngine {
    /// 🔤️ Captured font authority of a shaped display glyph.
    pub fn glyph_font_source(&self, font: ui_render::text::FontHandle) -> Option<(&[u8], u32)> { self.text.glyph_font_source(font) }
}

/// 🌄️ Export projection retains real paths and text sources without a synchronous font engine.
pub fn build_unshaped_export_background(doc: &LayoutSnapshot, page: &Page) -> DisplayList { super::display_list_assembly::assemble_display_list_items(None, doc, page, "", &[], None, false, false, 0.0, 0.0, 1.0, Vec::new(), true, true, false) }

/// 🖼️ A resolved non-text frame projects without shaping or viewport culling.
pub fn build_unshaped_export_frame(doc: &LayoutSnapshot, page: &Page, frame: semio_s_artifact_layout_layout_model::ResolvedFrame) -> DisplayList { super::display_list_assembly::assemble_display_list_items(None, doc, page, "", &[], None, false, false, 0.0, 0.0, 1.0, vec![frame], false, true, false) }

/// 🧳️ Both interactive surfaces retain story identities and geometry without copying story strings.
pub fn build_deferred_display_list(doc:&LayoutSnapshot,page:&Page,active_page_id:&str,selected_ids:&[String],hovered_id:Option<&str>,chrome_blueprint:bool,camera_x:f64,camera_y:f64,zoom:f64)->DisplayList{assemble_display_list(None,doc,page,active_page_id,selected_ids,hovered_id,chrome_blueprint,zoom<INTERACTIVE_TEXT_ZOOM,camera_x,camera_y,zoom,true)}

#[cfg(feature="testkit")]
impl LayoutEngine {
    pub fn test_text_system(&mut self) -> &mut TextSystem { &mut self.text }
    pub fn test_font_dependency(&self) -> FontDependencyId { self.font }
    pub fn test_without_builtin_fonts() -> Self { Self::from_text_system(TextSystem::without_builtin_fonts()) }
}
