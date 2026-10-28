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
                    if !self.validation_nested.started {
                        let width = finite_dimension(page.width)?;
                        let height = finite_dimension(page.height)?;
                        let request = self.request.as_ref().unwrap();
                        let raster_page = request.kind == LayoutExportKind::Png && request.page_id.as_deref().map_or(self.validation_index == 0, |id| id == page.id);
                        if raster_page && (width > MAX_LAYOUT_EXPORT_DIMENSION || height > MAX_LAYOUT_EXPORT_DIMENSION || u64::from(width) * u64::from(height) > MAX_LAYOUT_EXPORT_PIXELS) {
                            return Err("layout-export-raster-dimension-limit".into());
                        }
                        if page.guides.len() > MAX_LAYOUT_EXPORT_GUIDES_PER_PAGE || page.layers.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE || page.layer_ids.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE
                            || !bounded_id(&page.id) || !bounded_string(&page.name) || !bounded_id(&page.spread_id) || page.parent_page_id.as_ref().is_some_and(|id| !bounded_id(id))
                            || [page.margins.top, page.margins.right, page.margins.bottom, page.margins.left, page.columns.gutter].iter().any(|value| !value.is_finite()) || page.columns.count == 0 {
                            return Err("layout-export-page-envelope".into());
                        }
                        let nested = page.frames.len() + page.overrides.len() + page.guides.len() + page.layers.len() + page.layer_ids.len() + page.layers.iter().map(|layer| layer.object_ids.len()).sum::<usize>();
                        add_validated_items(&mut self.validation_total_items, nested)?;
                        if request.page_id.as_deref().is_some_and(|id| id == page.id) || request.page_id.is_none() && self.target_page.is_none() { self.target_page = Some(self.validation_index); }
                        self.validation_nested.started = true;
                    } else if validate_nested(&mut self.validation_nested, &page.frames, &page.overrides, &page.guides, &page.layers, &page.layer_ids).map_err(|_| "layout-export-page-envelope")? {
                        self.validation_nested = NestedValidationCursor::default();
                        self.validation_index += 1;
                    }
                }
                None => { self.validation_group = 2; self.validation_index = 0; }
            },
            2 => match snapshot.stories.get(self.validation_index) {
                Some(story) => {
                    if !self.validation_nested.started {
                        if !bounded_id(&story.id) || !bounded_string(&story.content) { return Err("layout-export-story-envelope".into()); }
                        add_validated_items(&mut self.validation_total_items, story.style_runs.len())?;
                        self.validation_nested.started = true;
                    } else if let Some(run) = story.style_runs.get(self.validation_nested.index) {
                        if run.paragraph_style_id.as_ref().is_some_and(|id| !bounded_id(id)) || run.character_style_id.as_ref().is_some_and(|id| !bounded_id(id)) { return Err("layout-export-story-envelope".into()); }
                        self.validation_nested.index += 1;
                    } else { self.validation_nested = NestedValidationCursor::default(); self.validation_index += 1; }
                }
                None => { self.validation_group = 3; self.validation_index = 0; }
            },
            3 => match snapshot.links.get(self.validation_index) {
                Some(link) => {
                    if !bounded_id(&link.id)
                        || !bounded_string(&link.path)
                        || link.hash.len() > 256
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
                    if !self.validation_nested.started {
                        finite_dimension(parent.width)?; finite_dimension(parent.height)?;
                        if parent.layers.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE || parent.layer_ids.len() > MAX_LAYOUT_EXPORT_LAYERS_PER_PAGE || !bounded_id(&parent.id) || !bounded_string(&parent.name) { return Err("layout-export-parent-page-envelope".into()); }
                        let nested = parent.frames.len() + parent.layers.len() + parent.layer_ids.len() + parent.layers.iter().map(|layer| layer.object_ids.len()).sum::<usize>();
                        add_validated_items(&mut self.validation_total_items, nested)?;
                        self.validation_nested.started = true;
                    } else if validate_nested(&mut self.validation_nested, &parent.frames, &[], &[], &parent.layers, &parent.layer_ids).map_err(|_| "layout-export-parent-page-envelope")? {
                        self.validation_nested = NestedValidationCursor::default(); self.validation_index += 1;
                    }
                }
                None => { self.validation_group = 5; self.validation_index = 0; }
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
                    self.validation_group = 12;self.validation_index=0;
                    return Ok(());
                }
                if self.typed_validation.is_none() {
                    self.typed_validation = Some(TypedJsonCursor::validating(TypedJsonNode::BackgroundDrawing));
                }
                if self.typed_validation.as_mut().ok_or("layout-export-json-validator")?.advance(snapshot)?.1 {
                    self.typed_validation = None;
                    self.validation_group = 12;self.validation_index=0;
                }
            }
            12 => {
                if let Some(page)=snapshot.pages.get(self.validation_index){
                    if page.background_drawing.is_none(){self.validation_index+=1;return Ok(())}
                    if self.typed_validation.is_none(){self.typed_validation=Some(TypedJsonCursor::validating(TypedJsonNode::Child(ChildJsonCursor::new(semio_s_artifact_layout_layout_model::DrawingSourceAddress::Page(self.validation_index)))));}
                    if self.typed_validation.as_mut().ok_or("layout-export-json-validator")?.advance(snapshot)?.1{self.typed_validation=None;self.validation_index+=1;}return Ok(())
                }
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
                if matches!(self.request.as_ref().unwrap().kind, LayoutExportKind::Package) {
                    self.stage = ExportStage::BeginEncode;
                } else if matches!(self.request.as_ref().unwrap().kind, LayoutExportKind::Step | LayoutExportKind::Idml) {
                    self.stage = ExportStage::BeginEncode;
                } else {
                    self.stage = ExportStage::Plan;
                }
            }
            _ => unreachable!("closed validation state"),
        }
        Ok(())
    }

    pub(super) fn plan_one_in(&mut self,cx:&mut StepContext<'_>,drawing_read:Option<&semio_framework_io_ports::ChildAdmissionRead<'_>>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        let drawing_read=super::drawing_read::require_original_drawing_read(self.required_drawing_source(),drawing_read)?;
        let snapshot=Arc::clone(&self.request.as_ref().unwrap().snapshot);
        if !self.text.get_or_insert_with(ExportTextState::new).prepare_flow_projection(&snapshot,cx)?{return Ok(());}
        if !self.prepare_frame_plan(&snapshot,cx)?||cx.should_yield(){return Ok(());}
        let mut work=semio_framework_2d_text_font::work::FontWorkLedger::new(cx.retained_grant());
        let entered=work.enter();let result=(||->Result<(),semio_framework_job::OriginalTurnFailure>{
            if entered.is_none(){return Ok(());}if !self.pdf_items.reserve_one_in(&mut work)?||!self.pdf_pages.reserve_one_in(&mut work)?{return Ok(());}
            if self.pdf_items.len()>=MAX_LAYOUT_EXPORT_DECODED_ITEMS{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"layout-export-item-limit").into());}
            if self.text.as_ref().is_some_and(ExportTextState::has_active){if let Some(item)=self.text.as_mut().unwrap().advance_one_in(&snapshot,&mut work)?{self.push_plan_item(item,&mut work).map_err(Fault::from)?;}return Ok(());}
            self.plan_pdf_one(&mut work,drawing_read).map_err(Fault::from)?;Ok(())
        })();if entered.is_some(){if !work.blocked(){work.note_source(entered==Some(true));}work.exit();}self.plan_work_demand=work.next_demand();cx.consume_retained(work.receipt())?;result?;Ok(())
    }

    #[cfg(test)]
    pub(super) fn plan_one(&mut self)->Result<(),String>{
        let operation=self.operation;let mut caller=layout_export_fixture_caller(operation,operation.generation);
        let result=caller.turn(LAYOUT_EXPORT_FIXTURE_TURN,|cx|Ok(self.plan_one_in(cx,None)));
        caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);result.map_err(|error|error.primary.message().to_owned())?.map_err(|error|error.primary.message().to_owned())
    }

    /// 🧭️ One scalar cursor visit preserves stable layer order without retaining or sorting a whole page.
    pub(super) fn next_frame_plan(&mut self,page_index:usize)->Result<FramePlanTurn,String>{self.frame_plan.as_mut().ok_or("layout-export-frame-plan-missing")?.next(page_index)}

    pub(super) fn begin_encoder(&mut self,work:&mut OutputWork) -> Result<(), String> {
        match self.request.as_ref().unwrap().kind {
            LayoutExportKind::Svg => {
                let page = self.page()?;
                let width = finite_dimension(page.width)?;
                let height = finite_dimension(page.height)?;
                self.output.append_format(format_args!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\"><rect width=\"100%\" height=\"100%\" fill=\"white\"/>"),work)?;
            }
            LayoutExportKind::Pdf => self.begin_pdf(work)?,
            LayoutExportKind::Png => self.begin_png(work)?,
            LayoutExportKind::Step | LayoutExportKind::Idml => {
                let kind = self.request.as_ref().unwrap().kind;
                let session = InterchangeExportSession::new(if kind == LayoutExportKind::Step { "step" } else { "idml" })?;
                self.interchange_session = Some(if kind == LayoutExportKind::Step { session.with_selected_page(self.target_page.ok_or("layout-export-page-missing")?) } else { session });
            }
            LayoutExportKind::Package => self.begin_zip_entry(PackageMember::Document,work)?,
        }
        Ok(())
    }

    pub(super) fn output_work_demand(&self,encoded:bool)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};let output=if encoded{&self.encoded}else{&self.output};if !output.has_room(OUTPUT_BODY_BYTES){return output.prepare_demand(OUTPUT_BODY_BYTES);}
        let mut demand=RetirementDemand{copy_bytes:OUTPUT_COPY_BYTES,depth:output.write_depth()?.max(1),..Default::default()};
        if !encoded&&self.stage==ExportStage::Encode&&self.request.as_ref().is_some_and(|request|request.kind==LayoutExportKind::Pdf){let child=self.pdf_offset_demand()?;demand.copy_bytes=demand.copy_bytes.checked_add(child.copy_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"layout output original copy quote overflow"))?;demand.capacity_bytes=child.capacity_bytes;demand.release_bytes=child.release_bytes;demand.depth=demand.depth.max(child.depth);}
        if !encoded&&self.stage==ExportStage::Encode&&self.request.as_ref().is_some_and(|request|request.kind==LayoutExportKind::Png)&&self.png_row_section==PngRowSection::Initialize&&self.png_row.capacity()==0{let page=self.page().map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"layout output PNG page is missing"))?;let width=finite_dimension(page.width).map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"layout output PNG width is invalid"))?;demand.capacity_bytes=width as usize*4+1;demand.copy_bytes+=size_of::<Vec<u8>>()*2;}
        Ok(demand)
    }

    pub(super) fn emit_output_one(&mut self,context:&mut StepContext<'_>,encoded:bool,emit:impl FnOnce(&mut Self,&mut OutputWork)->Result<(),String>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if context.should_yield()||context.is_cancelled(){return Ok(false);}
        let output=if encoded{&mut self.encoded}else{&mut self.output};
        if !output.has_room(OUTPUT_BODY_BYTES){let result=output.prepare_one(OUTPUT_BODY_BYTES,context.retained_grant());let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};context.consume_retained(progress)?;result?;return Ok(false);}
        let demand=self.output_work_demand(encoded)?;let grant=context.retained_grant();if grant.maximum_items<32||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(false);}
        let mut work=OutputWork::new(grant);
        let result=emit(self,&mut work);context.consume_retained(work.receipt())?;result.map_err(Fault::from)?;Ok(!work.is_preparatory())
    }

    pub(super) fn begin_encode_one(&mut self,context:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.emit_output_one(context,false,|job,work|{job.begin_encoder(work)?;job.stage=ExportStage::Encode;Ok(())})}

    pub(super) fn encode_one(&mut self,context:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if matches!(self.request.as_ref().unwrap().kind,LayoutExportKind::Svg|LayoutExportKind::Png|LayoutExportKind::Pdf){
            let fidelity=self.fidelity.get_or_insert_with(FidelityState::new);
            if !fidelity.prepared{if !fidelity.prepare_image_one(&mut self.pdf_items,&self.request.as_ref().unwrap().snapshot,context).map_err(Fault::from)?{return Ok(true);}let mut work=semio_framework_2d_text_font::work::FontWorkLedger::new(context.retained_grant());let result=fidelity.prepare_one(&self.pdf_items,self.text.as_ref(),self.drawing.as_ref(),&mut work);context.consume_retained(work.receipt())?;result.map_err(Fault::from)?;return Ok(true);}
        }
        self.emit_output_one(context,false,|job,work|match job.request.as_ref().unwrap().kind{LayoutExportKind::Svg=>job.encode_svg_one(work),LayoutExportKind::Pdf=>job.encode_pdf_one(work),LayoutExportKind::Png=>job.encode_png_one(work),LayoutExportKind::Package=>job.encode_package_one(work),LayoutExportKind::Step|LayoutExportKind::Idml=>job.encode_interchange_one(work)})
    }

    fn encode_interchange_one(&mut self,work:&mut OutputWork) -> Result<(), String> {
        let snapshot = &self.request.as_ref().unwrap().snapshot;
        match self.interchange_session.as_mut().ok_or("layout-export-session-missing")?.advance(snapshot)? {
            InterchangeExportAdvance::Chunk(bytes) => self.output.append(&bytes,work),
            InterchangeExportAdvance::Progress { .. } => Ok(()),
            InterchangeExportAdvance::Done => {
                self.stage = if self.request.as_ref().unwrap().kind.binary() { ExportStage::Base64 } else { ExportStage::PackageCommit };
                Ok(())
            }
        }
    }

    fn encode_svg_one(&mut self,work:&mut OutputWork) -> Result<(), String> {
        let fidelity = self.fidelity.as_mut().ok_or("layout-export-paint-plan-missing")?;
        match fidelity.shape(self.encode_cursor) {
            Some(PaintShape::Path(path)) => {
                if fidelity.svg_phase == 0 { self.output.append_format(format_args!("<path transform=\"{}\" d=\"",faithful_primitives::affine::svg(path.affine)),work)?; fidelity.svg_phase = 1; return Ok(()); }
                if let Some(segment) = path.segments.get(fidelity.svg_segment) { self.output.append_format(format_args!("{}",faithful_primitives::svg_segment(segment)?),work)?; fidelity.svg_segment += 1; return Ok(()); }
                self.output.append_format(format_args!("\" {} {} stroke-width=\"{}\" fill-rule=\"nonzero\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>", faithful_primitives::svg_color(path.fill, "fill")?, faithful_primitives::svg_color(path.stroke, "stroke")?, path.stroke_width),work)?;
            }
            Some(PaintShape::Image { source }) => {
                let item=self.pdf_items.get(*source).ok_or("layout-export-image-missing")?;let PdfItem::Image { x, y, width, height, .. }=item else{return Err("layout-export-image-unavailable".into())};let proxy=faithful_primitives::image_proxy(item,&self.request.as_ref().unwrap().snapshot)?;
                if fidelity.svg_phase == 0 { let matrix=faithful_primitives::image_affine(&self.pdf_items[*source]).ok_or("layout-export-image-missing")?;self.output.append_format(format_args!("<image x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" preserveAspectRatio=\"none\" transform=\"{}\" href=\"", f64::from(*x),f64::from(*y),f64::from(*width),f64::from(*height),faithful_primitives::affine::svg(matrix)),work)?; fidelity.svg_phase = 1; return Ok(()); }
                if fidelity.svg_byte < proxy.len() { let end = (fidelity.svg_byte + OUTPUT_CHUNK_BYTES).min(proxy.len()); self.output.append(&proxy.as_bytes()[fidelity.svg_byte..end],work)?; fidelity.svg_byte = end; return Ok(()); }
                self.output.append(b"\"/>",work)?;
            }
            None => { self.output.append(b"</svg>",work)?; self.stage = ExportStage::PackageCommit; return Ok(()); }
        }
        self.encode_cursor += 1; fidelity.svg_phase = 0; fidelity.svg_segment = 0; fidelity.svg_byte = 0; Ok(())
    }

}
//#endregion 🧩️Job

//#region 🔧️PlanPrimitives
pub(super) fn frame_id(frame: &Frame) -> &str {
    match frame {
        Frame::Rect { id, .. } | Frame::Text { id, .. } | Frame::Image { id, .. } => id,
    }
}

fn override_for<'a>(page: &'a Page, frame: &Frame) -> Option<&'a PageOverride> {
    page.overrides.iter().find(|candidate| candidate.object_id == frame_id(frame))
}

pub(super) fn finite_dimension(value: f64) -> Result<u32, String> {
    if !value.is_finite() || value < 1.0 || value > f64::from(u32::MAX) {
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
                && *columns > 0
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

/// 🔍️ Validates one nested row per fuel unit, independent of sheet size.
fn validate_nested(cursor: &mut NestedValidationCursor, frames: &[Frame], overrides: &[PageOverride], guides: &[semio_s_artifact_layout_layout_model::LayoutRect], layers: &[semio_s_artifact_layout_layout_model::Layer], layer_ids: &[String]) -> Result<bool, String> {
    let valid = match cursor.group {
        0 => frames.get(cursor.index).map(|frame| valid_frame(frame)),
        1 => overrides.get(cursor.index).map(|value| bounded_id(&value.object_id) && value.bounds.as_ref().is_none_or(valid_bounds)),
        2 => guides.get(cursor.index).map(valid_rect),
        3 => layers.get(cursor.index).map(|layer| bounded_id(&layer.id) && bounded_string(&layer.name)),
        4 => {
            if let Some(layer) = layers.get(cursor.layer) {
                if let Some(id) = layer.object_ids.get(cursor.index) {
                    if !bounded_id(id) { return Err("layout-export-nested-envelope".into()); }
                    cursor.index += 1;
                } else { cursor.layer += 1; cursor.index = 0; }
                return Ok(false);
            }
            None
        }
        5 => layer_ids.get(cursor.index).map(|id| bounded_id(id)),
        _ => return Ok(true),
    };
    match valid {
        Some(true) => cursor.index += 1,
        Some(false) => return Err("layout-export-nested-envelope".into()),
        None => { cursor.group += 1; cursor.index = 0; }
    }
    Ok(false)
}
