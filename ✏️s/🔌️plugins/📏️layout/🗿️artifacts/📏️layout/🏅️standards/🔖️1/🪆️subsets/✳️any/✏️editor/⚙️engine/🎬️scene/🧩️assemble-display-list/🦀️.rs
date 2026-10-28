//! 🧩️ Assemble display list of the layout scene: the assembly of one page display list from frames, stories, drawings, guides and layers. See [`super`].

use super::*;

//#region ⚙️Scene
pub(super) fn assemble_display_list(engine: Option<&mut LayoutEngine>, doc: &LayoutSnapshot, page: &Page, active_page_id: &str, selected_ids: &[String], hovered_id: Option<&str>, chrome_blueprint: bool, proxy_band: bool, camera_x: f64, camera_y: f64, zoom: f64, deferred:bool) -> DisplayList {
    assemble_display_list_items(engine, doc, page, active_page_id, selected_ids, hovered_id, chrome_blueprint, proxy_band, camera_x, camera_y, zoom, resolve_page(doc, page), true, false, deferred)
}

/// 🧩️ Builds one bounded frame or background batch using the same scene primitives as the full page.
pub(super) fn assemble_display_list_items(mut engine: Option<&mut LayoutEngine>, doc: &LayoutSnapshot, page: &Page, active_page_id: &str, selected_ids: &[String], hovered_id: Option<&str>, chrome_blueprint: bool, proxy_band: bool, camera_x: f64, camera_y: f64, zoom: f64, resolved: Vec<semio_s_artifact_layout_layout_model::ResolvedFrame>, include_background: bool, export: bool, deferred:bool) -> DisplayList {
    let source_page_index=deferred.then(||doc.pages.iter().position(|candidate|candidate.id==page.id)).flatten();
    let mut rects = Vec::new();
    let mut text_runs = Vec::new();
    let mut images = Vec::new();
    let mut frame_strokes = Vec::new();
    let background = if deferred{None}else{background_scene(doc)};
    let raw_drawing = raw_drawing_strokes(background.as_ref());
    let raw_labels = raw_drawing_labels(background.as_ref());
    let raw_rasters = raw_drawing_rasters(background.as_ref());
    let mut frame_labels: Vec<(u32, DrawingLabel)> = Vec::new();
    let mut guides = if chrome_blueprint && page.id == active_page_id { page_margin_guides(page) } else { Vec::new() };

    if chrome_blueprint && page.id == active_page_id {
        for guide in &page.guides {
            guides.push(DisplayGuide { rect: guide.clone(), kind: "guide".into() });
        }
        let col_count = page.columns.count.max(1) as f64;
        let col_width = (page.width - page.margins.left - page.margins.right - page.columns.gutter * (col_count - 1.0)) / col_count;
        for i in 0..page.columns.count {
            let x = page.margins.left + (i as f64) * (col_width + page.columns.gutter);
            guides.push(DisplayGuide { rect: LayoutRect { x, y: page.margins.top, width: col_width, height: page.height - page.margins.top - page.margins.bottom }, kind: "column".into() });
        }
        if engine.is_some() && !proxy_band && doc.grid.snap_to_baseline && doc.grid.baseline_grid > 0.0 {
            let mut y = doc.grid.baseline_offset;
            while y < page.height {
                guides.push(DisplayGuide { rect: LayoutRect { x: 0.0, y, width: page.width, height: 0.0 }, kind: "baseline".into() });
                y += doc.grid.baseline_grid;
            }
        }
    }

    let mut next_stack = 1u32;
    for item in resolved {
        if !item.frame.visible() || !frame_layer_visible(doc, page, &item.frame) {
            continue;
        }
        let selected = selected_ids.iter().any(|id| id == item.frame.id());
        let hovered = hovered_id.is_some_and(|id| id == item.frame.id());
        if !export && engine.is_none() && !selected && !hovered && outside_interactive_view(item.frame.bounds(), camera_x, camera_y, zoom) {
            continue;
        }
        let stack = next_stack;
        next_stack += 1;
        let rect_from = rects.len();
        let image_from = images.len();
        let text_from = text_runs.len();
        let stroke_from = frame_strokes.len();
        match &item.frame {
            Frame::Rect { id, bounds, fill, stroke, .. } => {
                rects.push(bounds_to_display_rect(id, bounds, item.inherited, selected, hovered, *fill, stroke.or(if chrome_blueprint && item.inherited { Some([0.4, 0.5, 0.7, 0.8]) } else { None })));
            }
            Frame::Text { id, bounds, story_id, inset, .. } => {
                if chrome_blueprint {
                    rects.push(bounds_to_display_rect(id, bounds, item.inherited, selected, hovered, None, Some([0.2, 0.55, 0.9, 0.9])));
                }
                if !proxy_band {
                    if let Some((story_index,story)) = doc.stories.iter().enumerate().find(|(_,s)| s.id == *story_id) {
                        if deferred{let paragraph=default_paragraph(doc);text_runs.push(DisplayTextRun{deferred:Some(display_list_and_hit_testing::DisplayParagraph{page_index:source_page_index,parent:item.parent,frame_index:item.frame_index,story:story_index,width:(bounds.width.min(inset.x+inset.width)-inset.x.max(0.0)).max(0.0),height:(bounds.height.min(inset.y+inset.height)-inset.y.max(0.0)).max(0.0)}),rotation:0.0,center:[0.0;2],object_id:id.clone(),glyphs:Vec::new(),content:String::new(),origin_x:(bounds.x+inset.x.max(0.0))as f32,origin_y:(bounds.y+inset.y.max(0.0))as f32,font_size:paragraph.font_size as f32,stack:0,color:[0.0,0.0,0.0,1.0]});}else{
                        let paragraph = default_paragraph(doc);
                        let base_x = (bounds.x + inset.x.max(0.0)) as f32;
                        let base_y = (bounds.y + inset.y.max(0.0)) as f32;
                        let font_size = paragraph.font_size as f32;
                        let glyphs = if let Some(engine) = engine.as_deref_mut() {
                            let frame_width = (bounds.width.min(inset.x+inset.width)-inset.x.max(0.0)).max(1.0) as f32;
                            let frame_height = (bounds.height.min(inset.y+inset.height)-inset.y.max(0.0)).max(1.0) as f32;
                            if story.style_runs.is_empty() {
                                let (shaped, _overset) = layout_story_in_frame(engine, story, &paragraph, frame_width, frame_height);
                                shaped.glyphs.iter().map(|glyph| DisplayGlyph { font: glyph.font, glyph_id: glyph.glyph_id as u32, font_size, x: base_x + glyph.x, y: base_y + glyph.y, color: DisplayColor([0.0, 0.0, 0.0, 1.0]), italic: false }).collect()
                            } else {
                                story_run_glyphs(engine, doc, story, &paragraph, frame_width, frame_height, base_x, base_y)
                            }
                        } else {
                            Vec::new()
                        };
                        let (origin_x, origin_y) = glyphs.first().map(|glyph| (glyph.x, glyph.y - font_size)).unwrap_or((base_x, base_y));
                        if glyphs.is_empty() && !story.style_runs.is_empty() {
                            let mut pen = base_x;
                            for (index, span) in style_spans(story, &paragraph, doc).into_iter().enumerate() {
                                if span.start >= span.end || !story.content.is_char_boundary(span.start) || !story.content.is_char_boundary(span.end) {
                                    continue;
                                }
                                let slice = story.content[span.start..span.end].to_string();
                                if slice.is_empty() {
                                    continue;
                                }
                                let chars = slice.chars().count() as f32;
                                text_runs.push(DisplayTextRun { deferred: None, rotation: 0.0, center: [0.0; 2], object_id: format!("{id}.span.{index}"), glyphs: Vec::new(), content: slice, origin_x: pen, origin_y: base_y, font_size: span.font_size, stack: 0, color: span.color.0 });
                                pen += span.font_size * chars * 0.6 + span.tracking * chars;
                            }
                        } else {
                            text_runs.push(DisplayTextRun { deferred: None, rotation: 0.0, center: [0.0; 2], object_id: id.clone(), glyphs, content: story.content.clone(), origin_x, origin_y, font_size, stack: 0, color: [0.0, 0.0, 0.0, 1.0] });
                        }
                        }
                    }
                }
            }
            Frame::Image { id, bounds, link_id, .. } => {
                let link = doc.links.iter().find(|l| l.id == *link_id);
                let proxy_data_url = link.and_then(|link| link.proxy_data_url.clone()).filter(|url| !url.is_empty());
                let placeholder = link.is_none_or(|l| l.state.as_deref() == Some("missing") || l.proxy_data_url.is_none());
                if chrome_blueprint {
                    rects.push(bounds_to_display_rect(id, bounds, item.inherited, selected, hovered, None, Some([0.85, 0.45, 0.2, 0.9])));
                }
                let deferred_drawing=deferred&&!proxy_band&&proxy_data_url.is_none()&&doc.background_drawing.is_some()&&link.is_some_and(|link|drawing_kind(&link.artifact_kind));
                let placed = if !deferred && !proxy_band && proxy_data_url.is_none() {
                    link.and_then(|link| drawing_kind(&link.artifact_kind).then(|| place_drawing_in_frame(&raw_drawing, bounds))).filter(|strokes| !strokes.is_empty())
                } else {
                    None
                };
                let preview = if proxy_band || proxy_data_url.is_some() || placed.is_some() || deferred_drawing { String::new() } else { link.map(|link| preview_mark(&link.artifact_kind)).unwrap_or_default() };
                images.push(DisplayImage { drawing_bounds:deferred_drawing.then_some([bounds.x,bounds.y,bounds.width,bounds.height,bounds.rotation]), object_id: id.clone(), x: bounds.x as f32, y: bounds.y as f32, width: bounds.width as f32, height: bounds.height as f32, rotation: bounds.rotation as f32, placeholder, proxy_data_url: proxy_data_url.clone(), preview, stack: 0 });
                if let Some(placed) = placed {
                    frame_strokes.extend(placed);
                    frame_labels.extend(place_drawing_labels(&raw_labels, &raw_drawing, bounds).into_iter().map(|label| (stack, label)));
                    images.extend(place_drawing_rasters(&raw_rasters, &raw_drawing, bounds));
                } else if !deferred_drawing && !proxy_band && proxy_data_url.is_none() {
                    if let Some(kind) = link.and_then(|link| (!link.artifact_kind.is_empty()).then(|| link.artifact_kind.clone())) {
                        let glyphs = placed_kind_glyphs(engine.as_deref_mut(), doc, bounds, &kind);
                        text_runs.push(DisplayTextRun { deferred: None, rotation: 0.0, center: [0.0; 2], object_id: id.clone(), glyphs, content: kind, origin_x: bounds.x as f32 + 4.0, origin_y: bounds.y as f32 + 4.0, font_size: 11.0, stack: 0, color: [0.0, 0.0, 0.0, 1.0] });
                    }
                }
            }
        }
        for rect in &mut rects[rect_from..] {
            rect.stack = stack;
        }
        for image in &mut images[image_from..] {
            image.stack = stack;
        }
        for run in &mut text_runs[text_from..] {
            run.stack = stack;
            let bounds = item.frame.bounds();
            run.rotation = bounds.rotation as f32;
            run.center = [(bounds.x + bounds.width * 0.5) as f32, (bounds.y + bounds.height * 0.5) as f32];
        }
        for stroke in &mut frame_strokes[stroke_from..] {
            stroke.stack = stack;
        }
    }

    let strokes = if include_background { drawing_strokes(&raw_drawing, page.width, page.height) } else { Vec::new() };
    let label_fit = include_background.then(|| fit_from_points(stroke_points(&raw_drawing), page.width, page.height).or_else(|| fit_from_points(raw_labels.iter().map(|label| (label.x, label.y)), page.width, page.height))).flatten();
    if let Some(engine) = engine.as_deref_mut() {
        if let Some((scale, ox, oy)) = label_fit {
            for label in &raw_labels {
                text_runs.push(shape_drawing_label(engine, doc, &label.text, label.x * scale + ox, label.y * scale + oy));
            }
        }
        for (stack, label) in &frame_labels {
            text_runs.push(shape_drawing_label(engine, doc, &label.text, label.x, label.y));
            text_runs.last_mut().unwrap().stack = *stack;
        }
    } else if let Some((scale, ox, oy)) = label_fit {
        for label in &raw_labels {
            text_runs.push(unshaped_drawing_label(&label.text, label.x * scale + ox, label.y * scale + oy));
        }
        for (stack, label) in &frame_labels {
            text_runs.push(unshaped_drawing_label(&label.text, label.x, label.y));
            text_runs.last_mut().unwrap().stack = *stack;
        }
    }
    if let Some((scale, ox, oy)) = include_background.then(|| fit_from_points(stroke_points(&raw_drawing), page.width, page.height).or_else(|| fit_from_points(raw_rasters.iter().flat_map(|raster| [(raster.x, raster.y), (raster.x + raster.width, raster.y + raster.height)]), page.width, page.height))).flatten() {
        for raster in &raw_rasters {
            push_drawing_raster(&mut images, raster, raster.x * scale + ox, raster.y * scale + oy, raster.width * scale, raster.height * scale, 0.0);
        }
    }
    DisplayList { page_id: page.id.clone(), page_width: page.width as f32, page_height: page.height as f32, strokes, frame_strokes, rects, text_runs, images, guides }
}
//#endregion ⚙️Scene
