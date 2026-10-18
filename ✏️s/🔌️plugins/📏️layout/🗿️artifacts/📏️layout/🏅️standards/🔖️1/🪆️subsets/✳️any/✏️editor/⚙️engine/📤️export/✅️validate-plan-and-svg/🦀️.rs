//! ✅️ Validate, plan and svg of the layout export job: the bounded validation of one page against the export caps, the page plan, the svg encoding and the plan primitives it shares. See [`super`].

use super::*;

//#region 🧩️Job
impl LayoutExportJob {
    pub(super) fn validate_one(&mut self) -> Result<(), String> {
        let snapshot = &self.request.as_ref().unwrap().snapshot;
        match self.validation_group {
            0 => {
                if snapshot.pages.len() > MAX_LAYOUT_EXPORT_PAGES
                    || snapshot.stories.len() > MAX_LAYOUT_EXPORT_STORIES
                    || snapshot.links.len() > MAX_LAYOUT_EXPORT_LINKS
                    || snapshot.parent_pages.len() > MAX_LAYOUT_EXPORT_PARENT_PAGES
                    || snapshot.spreads.len() > MAX_LAYOUT_EXPORT_SPREADS
                    || snapshot.paragraph_styles.len() > MAX_LAYOUT_EXPORT_STYLES
                    || snapshot.character_styles.len() > MAX_LAYOUT_EXPORT_STYLES
                    || !bounded_string(&snapshot.schema)
                    || !bounded_string(&snapshot.name)
                {
                    return Err("layout-export-document-envelope".into());
                }
                self.validation_total_items = snapshot.pages.len() + snapshot.stories.len() + snapshot.links.len() + snapshot.parent_pages.len() + snapshot.spreads.len() + snapshot.paragraph_styles.len() + snapshot.character_styles.len();
                self.validation_group = 1;
            }
            1 => match snapshot.pages.get(self.validation_index) {
                Some(page) => {
                    let width = finite_dimension(page.width)?;
                    let height = finite_dimension(page.height)?;
                    if u64::from(width) * u64::from(height) > MAX_LAYOUT_EXPORT_PIXELS
                        || page.frames.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || page.overrides.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || page.guides.len() > MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE
                        || page.layers.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                        || page.layer_ids.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                        || !bounded_id(&page.id)
                        || !bounded_string(&page.name)
                        || !bounded_id(&page.spread_id)
                        || page.parent_page_id.as_ref().is_some_and(|id| !bounded_id(id))
                        || page.frames.iter().any(|frame| !valid_frame(frame))
                        || page.overrides.iter().any(|value| !bounded_id(&value.object_id) || value.bounds.as_ref().is_some_and(|bounds| !valid_bounds(bounds)))
                        || page.guides.iter().any(|guide| !valid_rect(guide))
                        || !page.margins.top.is_finite()
                        || !page.margins.right.is_finite()
                        || !page.margins.bottom.is_finite()
                        || !page.margins.left.is_finite()
                        || page.columns.count as usize > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || !page.columns.gutter.is_finite()
                        || page.layers.iter().any(|layer| layer.object_ids.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE || !bounded_id(&layer.id) || !bounded_string(&layer.name) || layer.object_ids.iter().any(|id| !bounded_id(id)))
                        || page.layer_ids.iter().any(|id| !bounded_id(id))
                    {
                        return Err("layout-export-page-envelope".into());
                    }
                    let nested = page.frames.len() + page.overrides.len() + page.guides.len() + page.layers.len() + page.layer_ids.len() + page.layers.iter().map(|layer| layer.object_ids.len()).sum::<usize>();
                    self.validation_total_frames = self.validation_total_frames.checked_add(page.frames.len()).ok_or("layout-export-item-limit")?;
                    add_validated_items(&mut self.validation_total_items, nested)?;
                    if self.request.as_ref().unwrap().page_id.as_deref().is_some_and(|id| id == page.id) || self.request.as_ref().unwrap().page_id.is_none() && self.target_page.is_none() {
                        self.target_page = Some(self.validation_index);
                    }
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 2;
                    self.validation_index = 0;
                }
            },
            2 => match snapshot.stories.get(self.validation_index) {
                Some(story) => {
                    if !bounded_id(&story.id)
                        || !bounded_string(&story.content)
                        || story.style_runs.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || story.style_runs.iter().any(|run| run.paragraph_style_id.as_ref().is_some_and(|id| !bounded_id(id)) || run.character_style_id.as_ref().is_some_and(|id| !bounded_id(id)))
                    {
                        return Err("layout-export-story-envelope".into());
                    }
                    add_validated_items(&mut self.validation_total_items, story.style_runs.len())?;
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 3;
                    self.validation_index = 0;
                }
            },
            3 => match snapshot.links.get(self.validation_index) {
                Some(link) => {
                    if !bounded_id(&link.id)
                        || !bounded_string(&link.path)
                        || link.hash.len() > 256
                        || link.width > MAX_LAYOUT_EXPORT_DIMENSION
                        || link.height > MAX_LAYOUT_EXPORT_DIMENSION
                        || u64::from(link.width) * u64::from(link.height) > MAX_LAYOUT_EXPORT_PIXELS
                        || link.dpi > 9_600
                        || link.proxy_data_url.as_ref().is_some_and(|value| !bounded_string(value))
                        || link.color_profile.as_ref().is_some_and(|value| !bounded_string(value))
                        || link.state.as_ref().is_some_and(|value| !bounded_string(value))
                    {
                        return Err("layout-export-link-envelope".into());
                    }
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 4;
                    self.validation_index = 0;
                }
            },
            4 => match snapshot.parent_pages.get(self.validation_index) {
                Some(parent) => {
                    finite_dimension(parent.width)?;
                    finite_dimension(parent.height)?;
                    if parent.frames.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                        || parent.layers.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                        || parent.layer_ids.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                        || !bounded_id(&parent.id)
                        || !bounded_string(&parent.name)
                        || parent.frames.iter().any(|frame| !valid_frame(frame))
                        || parent.layers.iter().any(|layer| layer.object_ids.len() > MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE || !bounded_id(&layer.id) || !bounded_string(&layer.name) || layer.object_ids.iter().any(|id| !bounded_id(id)))
                        || parent.layer_ids.iter().any(|id| !bounded_id(id))
                    {
                        return Err("layout-export-parent-page-envelope".into());
                    }
                    let nested = parent.frames.len() + parent.layers.len() + parent.layer_ids.len() + parent.layers.iter().map(|layer| layer.object_ids.len()).sum::<usize>();
                    self.validation_total_frames = self.validation_total_frames.checked_add(parent.frames.len()).ok_or("layout-export-item-limit")?;
                    add_validated_items(&mut self.validation_total_items, nested)?;
                    self.validation_index += 1;
                }
                None => {
                    if self.validation_total_frames > MAX_LAYOUT_EXPORT_TOTAL_FRAMES {
                        return Err("layout-export-frame-envelope".into());
                    }
                    self.validation_group = 5;
                    self.validation_index = 0;
                }
            },
            5 => match snapshot.paragraph_styles.get(self.validation_index) {
                Some(style) => {
                    if !bounded_id(&style.id) || !bounded_string(&style.name) || !bounded_string(&style.font_family) || !bounded_string(&style.alignment) || !style.font_size.is_finite() || !style.leading.is_finite() || !style.tracking.is_finite() {
                        return Err("layout-export-paragraph-style-envelope".into());
                    }
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 6;
                    self.validation_index = 0;
                }
            },
            6 => match snapshot.character_styles.get(self.validation_index) {
                Some(style) => {
                    if !bounded_id(&style.id)
                        || style.name.as_ref().is_some_and(|value| !bounded_string(value))
                        || style.font_family.as_ref().is_some_and(|value| !bounded_string(value))
                        || style.font_size.is_some_and(|value| !value.is_finite())
                        || style.tracking.is_some_and(|value| !value.is_finite())
                        || style.color.is_some_and(|color| color.iter().any(|channel| !channel.is_finite()))
                    {
                        return Err("layout-export-character-style-envelope".into());
                    }
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 7;
                    self.validation_index = 0;
                }
            },
            7 => match snapshot.spreads.get(self.validation_index) {
                Some(spread) => {
                    if spread.page_ids.len() > MAX_LAYOUT_EXPORT_SPREAD_PAGE_IDS || !bounded_id(&spread.id) || !bounded_string(&spread.name) || spread.page_ids.iter().any(|id| !bounded_id(id)) {
                        return Err("layout-export-spread-envelope".into());
                    }
                    add_validated_items(&mut self.validation_total_items, spread.page_ids.len())?;
                    self.validation_index += 1;
                }
                None => {
                    self.validation_group = 8;
                    self.validation_index = 0;
                }
            },
            8 => {
                if snapshot.print_target.as_ref().is_some_and(|value| !bounded_string(value)) {
                    return Err("layout-export-print-target-envelope".into());
                }
                if !snapshot.grid.baseline_grid.is_finite() || !snapshot.grid.baseline_offset.is_finite() {
                    return Err("layout-export-grid-envelope".into());
                }
                self.validation_group = 9;
            }
            9 => {
                if snapshot.data_fields.is_none() {
                    self.validation_group = 10;
                    return Ok(());
                }
                if self.typed_validation.is_none() {
                    self.typed_validation = Some(TypedJsonCursor::validating(TypedJsonNode::Dictionary(DictionaryJsonCursor::new())));
                }
                if self.typed_validation.as_mut().ok_or("layout-export-json-validator")?.advance(snapshot)?.1 {
                    self.typed_validation = None;
                    self.validation_group = 10;
                }
            }
            10 => {
                let Some(value) = &self.request.as_ref().unwrap().preflight_json else {
                    self.validation_group = 11;
                    return Ok(());
                };
                if self.json_validation.is_none() {
                    self.json_validation = Some(JsonValidationCursor::new(value, true)?);
                }
                if self.json_validation.as_mut().ok_or("layout-export-json-validator")?.advance(value)? {
                    self.json_validation = None;
                    self.validation_group = 11;
                }
            }
            11 => {
                if snapshot.background_drawing.is_none() {
                    self.validation_group = 12;
                    return Ok(());
                }
                if self.typed_validation.is_none() {
                    self.typed_validation = Some(TypedJsonCursor::validating(TypedJsonNode::BackgroundDrawing));
                }
                if self.typed_validation.as_mut().ok_or("layout-export-json-validator")?.advance(snapshot)?.1 {
                    self.typed_validation = None;
                    self.validation_group = 12;
                }
            }
            12 => {
                if snapshot.referenced_model.is_none() {
                    self.validation_group = 13;
                    return Ok(());
                }
                if self.typed_validation.is_none() {
                    self.typed_validation = Some(TypedJsonCursor::validating(TypedJsonNode::ReferencedModel));
                }
                if self.typed_validation.as_mut().ok_or("layout-export-json-validator")?.advance(snapshot)?.1 {
                    self.typed_validation = None;
                    self.validation_group = 13;
                }
            }
            13 => {
                if !matches!(self.request.as_ref().unwrap().kind, LayoutExportKind::Package) && self.target_page.is_none() {
                    return Err("layout-export-page-missing".into());
                }
                self.parent_frame_count =
                    self.target_page.and_then(|index| snapshot.pages.get(index)).and_then(|page| page.parent_page_id.as_ref().and_then(|id| snapshot.parent_pages.iter().find(|parent| &parent.id == id))).map_or(0, |parent| parent.frames.len());
                if matches!(self.request.as_ref().unwrap().kind, LayoutExportKind::Package) {
                    self.begin_zip_entry("document.json")?;
                    self.stage = ExportStage::Encode;
                } else {
                    self.stage = ExportStage::Plan;
                }
            }
            _ => unreachable!("closed validation state"),
        }
        Ok(())
    }

    pub(super) fn plan_one(&mut self) -> Result<(), String> {
        if matches!(self.request.as_ref().unwrap().kind, LayoutExportKind::Pdf) {
            return self.plan_pdf_one();
        }
        let page = self.page()?;
        let parent = page.parent_page_id.as_ref().and_then(|id| self.request.as_ref().unwrap().snapshot.parent_pages.iter().find(|candidate| &candidate.id == id));
        let frame = if self.plan_cursor < self.parent_frame_count { parent.and_then(|value| value.frames.get(self.plan_cursor)) } else { page.frames.get(self.plan_cursor - self.parent_frame_count) };
        let Some(frame) = frame else {
            self.begin_encoder()?;
            self.stage = ExportStage::Encode;
            return Ok(());
        };
        let inherited = self.plan_cursor < self.parent_frame_count;
        let override_value = if inherited { override_for(page, frame) } else { None };
        if let Some(rect) = export_rect(frame, override_value, finite_dimension(page.width)?, finite_dimension(page.height)?) {
            self.rects.push(rect);
        }
        self.plan_cursor += 1;
        Ok(())
    }

    pub(super) fn begin_encoder(&mut self) -> Result<(), String> {
        match self.request.as_ref().unwrap().kind {
            LayoutExportKind::Svg => {
                let page = self.page()?;
                let width = finite_dimension(page.width)?;
                let height = finite_dimension(page.height)?;
                self.output.append(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\"><rect width=\"100%\" height=\"100%\" fill=\"white\"/>").as_bytes())?;
            }
            LayoutExportKind::Pdf => self.begin_pdf()?,
            LayoutExportKind::Png => self.begin_png()?,
            LayoutExportKind::Package => unreachable!("package begins after validation"),
        }
        Ok(())
    }

    pub(super) fn encode_one(&mut self) -> Result<(), String> {
        match self.request.as_ref().unwrap().kind {
            LayoutExportKind::Svg => self.encode_svg_one(),
            LayoutExportKind::Pdf => self.encode_pdf_one(),
            LayoutExportKind::Png => self.encode_png_one(),
            LayoutExportKind::Package => self.encode_package_one(),
        }
    }

    fn encode_svg_one(&mut self) -> Result<(), String> {
        if let Some(rect) = self.rects.get(self.encode_cursor) {
            self.output.append(
                format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{:.4}\"/>", rect.x, rect.y, rect.width, rect.height, rect.rgba[0], rect.rgba[1], rect.rgba[2], f32::from(rect.rgba[3]) / 255.0)
                    .as_bytes(),
            )?;
            self.encode_cursor += 1;
        } else {
            self.output.append(b"</svg>")?;
            self.stage = ExportStage::PackageCommit;
        }
        Ok(())
    }

}
//#endregion 🧩️Job

//#region 🔧️PlanPrimitives
fn frame_visible(frame: &Frame) -> bool {
    match frame {
        Frame::Rect { visible, .. } | Frame::Text { visible, .. } | Frame::Image { visible, .. } => visible.unwrap_or(true),
    }
}

pub(super) fn frame_id(frame: &Frame) -> &str {
    match frame {
        Frame::Rect { id, .. } | Frame::Text { id, .. } | Frame::Image { id, .. } => id,
    }
}

fn override_for<'a>(page: &'a Page, frame: &Frame) -> Option<&'a PageOverride> {
    page.overrides.iter().find(|candidate| candidate.object_id == frame_id(frame))
}

fn export_rect(frame: &Frame, override_value: Option<&PageOverride>, page_width: u32, page_height: u32) -> Option<ExportRect> {
    if !override_value.and_then(|value| value.visible).unwrap_or_else(|| frame_visible(frame)) {
        return None;
    }
    let bounds = override_value.and_then(|value| value.bounds.as_ref()).unwrap_or_else(|| frame_bounds(frame));
    let x = bounds.x.max(0.0).min(f64::from(page_width)) as u32;
    let y = bounds.y.max(0.0).min(f64::from(page_height)) as u32;
    let x1 = (bounds.x + bounds.width).max(0.0).min(f64::from(page_width)) as u32;
    let y1 = (bounds.y + bounds.height).max(0.0).min(f64::from(page_height)) as u32;
    if x1 <= x || y1 <= y {
        return None;
    }
    let rgba = match frame {
        Frame::Rect { fill, .. } => fill.map_or([0, 0, 0, 0], |color| color.map(|channel| (channel.clamp(0.0, 1.0) * 255.0).round() as u8)),
        Frame::Text { .. } => [24, 24, 24, 255],
        Frame::Image { .. } => [220, 220, 220, 255],
    };
    Some(ExportRect { x, y, width: x1 - x, height: y1 - y, rgba })
}

pub(super) fn finite_dimension(value: f64) -> Result<u32, String> {
    if !value.is_finite() || value < 1.0 || value > f64::from(MAX_LAYOUT_EXPORT_DIMENSION) {
        return Err("layout-export-dimension-limit".into());
    }
    Ok(value.ceil() as u32)
}

pub fn bounded_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= 256
}

fn bounded_string(value: &str) -> bool {
    value.len() <= MAX_LAYOUT_EXPORT_STRING_BYTES
}

fn valid_frame(frame: &Frame) -> bool {
    match frame {
        Frame::Rect { id, layer_id, bounds, fill, stroke, .. } => bounded_id(id) && bounded_id(layer_id) && valid_bounds(bounds) && fill.is_none_or(valid_color) && stroke.is_none_or(valid_color),
        Frame::Text { id, layer_id, bounds, story_id, thread_next, columns, inset, wrap_mode, .. } => {
            bounded_id(id)
                && bounded_id(layer_id)
                && valid_bounds(bounds)
                && bounded_id(story_id)
                && thread_next.as_ref().is_none_or(|id| bounded_id(id))
                && (*columns as usize) <= MAX_LAYOUT_EXPORT_FRAMES_PER_PAGE
                && valid_rect(inset)
                && bounded_string(wrap_mode)
        }
        Frame::Image { id, layer_id, bounds, link_id, .. } => bounded_id(id) && bounded_id(layer_id) && valid_bounds(bounds) && bounded_id(link_id),
    }
}

fn valid_bounds(bounds: &LayoutBounds) -> bool {
    bounds.x.is_finite() && bounds.y.is_finite() && bounds.width.is_finite() && bounds.height.is_finite() && bounds.rotation.is_finite()
}

fn valid_rect(rect: &semio_s_artifact_layout_layout_model::LayoutRect) -> bool {
    rect.x.is_finite() && rect.y.is_finite() && rect.width.is_finite() && rect.height.is_finite()
}

fn valid_color(color: [f32; 4]) -> bool {
    color.iter().all(|channel| channel.is_finite())
}

fn add_validated_items(total: &mut usize, count: usize) -> Result<(), String> {
    *total = total.checked_add(count).ok_or("layout-export-item-limit")?;
    if *total > MAX_LAYOUT_EXPORT_DECODED_ITEMS {
        return Err("layout-export-item-limit".into());
    }
    Ok(())
}
//#endregion 🔧️PlanPrimitives
