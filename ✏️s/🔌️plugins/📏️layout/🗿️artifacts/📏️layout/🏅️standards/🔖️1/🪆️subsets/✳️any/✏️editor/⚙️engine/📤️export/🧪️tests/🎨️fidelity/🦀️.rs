//! 🎨️ Language-neutral contours and pixels agree with independent native vector oracles.
use super::*;
use faithful_primitives::{FidelityState, PaintShape, source_over};
use semio_framework_2d_engine::PathSegment;

fn plan_fixture(rows:impl IntoIterator<Item=PdfItem>)->PlanItems{let mut owner=PlanItems::new();for row in rows{append_plan_fixture(&mut owner,row);}owner}
fn corpus() -> serde_json::Value { serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🎨️export-fidelity/🔣️.json")).unwrap() }
fn color(value: &serde_json::Value) -> Option<[f32; 4]> { value.as_array().map(|rows| std::array::from_fn(|i| rows[i].as_f64().unwrap() as f32)) }
fn prepare(items:&PlanItems,text:Option<&ExportTextState>)->FidelityState{let mut state=FidelityState::new();for _ in 0..1_000_000{if state.prepared{return state;}state.prepare_one(items,text,None,&mut semio_framework_2d_text_font::work::FontWorkLedger::new(LAYOUT_EXPORT_FIXTURE_TURN)).unwrap();}panic!("bounded primitive plan did not terminate");}
fn pixel(state: &mut FidelityState, shape: usize, x: u32, y: u32) -> [u8; 4] { let mut output = [255; 4]; for _ in 0..1_000_000 { if let Some((fill, stroke)) = state.sample_path_one(shape, x, y) { source_over(&mut output, fill); source_over(&mut output, stroke); return output; } } panic!("bounded raster sample did not terminate"); }
#[test]
fn portable_fill_stroke_rotation_and_alpha_match_tiny_skia() {
    for row in corpus()["cases"].as_array().unwrap() {
        let paths = row["paths"].as_array().unwrap();
        let items: Vec<_> = paths.iter().map(|path| PdfItem::Stroke { points: path["points"].as_array().unwrap().iter().map(|p| (p[0].as_f64().unwrap() as f32, p[1].as_f64().unwrap() as f32)).collect(), closed: path["closed"].as_bool().unwrap(), fill: color(&path["fill"]), color: color(&path["stroke"]).unwrap_or([0.0; 4]), width: path["strokeWidth"].as_f64().unwrap() as f32 }).collect();
        let mut items=plan_fixture(items);let mut state=prepare(&items,None);
        let mut reference = tiny_skia::Pixmap::new(row["width"].as_u64().unwrap() as u32, row["height"].as_u64().unwrap() as u32).unwrap(); reference.fill(tiny_skia::Color::WHITE);
        for path in paths {
            let points = path["points"].as_array().unwrap(); let mut builder = tiny_skia::PathBuilder::new(); builder.move_to(points[0][0].as_f64().unwrap() as f32, points[0][1].as_f64().unwrap() as f32); for point in &points[1..] { builder.line_to(point[0].as_f64().unwrap() as f32, point[1].as_f64().unwrap() as f32); } if path["closed"].as_bool().unwrap() { builder.close(); } let path_data = builder.finish().unwrap();
            if let Some(fill) = color(&path["fill"]) { let mut paint = tiny_skia::Paint::default(); paint.set_color_rgba8((fill[0] * 255.0).round() as u8, (fill[1] * 255.0).round() as u8, (fill[2] * 255.0).round() as u8, (fill[3] * 255.0).round() as u8); reference.fill_path(&path_data, &paint, tiny_skia::FillRule::Winding, tiny_skia::Transform::identity(), None); }
            if let Some(stroke) = color(&path["stroke"]) { let mut paint = tiny_skia::Paint::default(); paint.set_color_rgba8((stroke[0] * 255.0).round() as u8, (stroke[1] * 255.0).round() as u8, (stroke[2] * 255.0).round() as u8, (stroke[3] * 255.0).round() as u8); reference.stroke_path(&path_data, &paint, &tiny_skia::Stroke { width: path["strokeWidth"].as_f64().unwrap() as f32, ..Default::default() }, tiny_skia::Transform::identity(), None); }
        }
        for probe in row["probes"].as_array().unwrap() { let x = probe["x"].as_u64().unwrap() as u32; let y = probe["y"].as_u64().unwrap() as u32; let actual = pixel(&mut state, 0, x, y); let at = (y * reference.width() + x) as usize * 4; for channel in 0..4 { let expected = probe["rgba"][channel].as_u64().unwrap() as u8; let tolerance = probe["tolerance"].as_u64().unwrap() as u8; assert!(actual[channel].abs_diff(expected) <= tolerance, "{} channel={channel}", row["id"]); assert!(actual[channel].abs_diff(reference.data()[at + channel]) <= tolerance); } }
        close_original_fixture(state);close_plan_fixture(&mut items);
    }
}
struct OracleOutline { segments: Vec<PathSegment>, origin: [f64; 2], scale: f64, rotation: f64, center: [f64; 2] }
impl OracleOutline { fn map(&self, x: f32, y: f32) -> [f64; 2] { let px = self.origin[0] + f64::from(x) * self.scale - self.center[0]; let py = self.origin[1] - f64::from(y) * self.scale - self.center[1]; let (sin, cos) = self.rotation.sin_cos(); [self.center[0] + px * cos - py * sin, self.center[1] + px * sin + py * cos] } }
impl ttf_parser::OutlineBuilder for OracleOutline { fn move_to(&mut self, x: f32, y: f32) { self.segments.push(PathSegment::Move { to: self.map(x, y) }); } fn line_to(&mut self, x: f32, y: f32) { self.segments.push(PathSegment::Line { to: self.map(x, y) }); } fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) { self.segments.push(PathSegment::Quad { ctrl: self.map(x1, y1), to: self.map(x, y) }); } fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) { self.segments.push(PathSegment::Cubic { ctrl1: self.map(x1, y1), ctrl2: self.map(x2, y2), to: self.map(x, y) }); } fn close(&mut self) { self.segments.push(PathSegment::Close); } }
#[test]
fn actual_shaped_glyph_outlines_match_original_font_ttf_parser() {
    for row in corpus()["glyphs"].as_array().unwrap(){
        let size=row["size"].as_f64().unwrap();let x=row["origin"][0].as_f64().unwrap()as f32;let y=row["origin"][1].as_f64().unwrap()as f32;let rotation=row["rotation"].as_f64().unwrap_or(0.0)as f32;let center=row["center"].as_array().map_or([0.0;2],|values|[values[0].as_f64().unwrap()as f32,values[1].as_f64().unwrap()as f32]);
        let snapshot=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();let mut text=ExportTextState::new();text.begin_label_fixture(row["character"].as_str().unwrap().into(),x,y-size as f32,size as f32,rotation,center);let mut items=PlanItems::new();
        for _ in 0..1_000_000{if !text.has_active(){break;}let mut work=semio_framework_2d_text_font::work::FontWorkLedger::new(LAYOUT_EXPORT_FIXTURE_TURN);if !items.reserve_one_in(&mut work).unwrap(){continue;}if let Some(item)=text.advance_one_in(&snapshot,&mut work).unwrap(){items.push_in(item,&mut work).unwrap();}}
        assert!(!text.has_active());let PdfItem::Outline{source,glyph,..}=items.get(0).unwrap()else{panic!("retained paragraph did not emit outline");};let record=text.glyph(*source,*glyph).unwrap();let state=prepare(&items,Some(&text));let bytes=semio_s_artifact_layout_layout_engine::scene::text_fonts::export_font_source(record.face).unwrap();let face=ttf_parser::Face::parse(bytes,0).unwrap();
        let mut oracle=OracleOutline{segments:Vec::new(),origin:record.origin,scale:size/f64::from(face.units_per_em()),rotation:f64::from(rotation),center:center.map(f64::from)};face.outline_glyph(ttf_parser::GlyphId(record.glyph),&mut oracle).unwrap();
        let Some(PaintShape::Path(path)) = state.shape(0) else { panic!("glyph became a placeholder"); }; assert!(path.segments.len() > 3); assert!(path.edges.len() > 3);
        let actual = rendered_segments(path.segments.iter()); let expected = rendered_segments(oracle.segments.iter()); assert!(actual.data().iter().zip(expected.data()).all(|(a, b)| a.abs_diff(*b) <= 1), "transformed glyph contours must match the original-font oracle");
        close_original_fixture(state);close_plan_fixture(&mut items);close_original_fixture(text);
    }
}

#[test]
fn portable_rgba_proxy_rotation_and_source_over_preserve_pixels() {
    for row in corpus()["images"].as_array().unwrap() {
        let f = &row["frame"]; let image = &row["image"];
        let rgba: Vec<u8> = image["rgba"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
        let item = PdfItem::Image {link:None,affine:None, x: f["x"].as_f64().unwrap(), y: f["y"].as_f64().unwrap(), width: f["width"].as_f64().unwrap(), height: f["height"].as_f64().unwrap(), rotation: f["rotation"].as_f64().unwrap(), placeholder: false, mark: String::new(), proxy: None, raster: Some(PdfRaster { width: image["width"].as_u64().unwrap() as u32, height: image["height"].as_u64().unwrap() as u32, resource: None, rgba }) };
        for probe in row["probes"].as_array().unwrap() { let mut pixel = [255; 4]; source_over(&mut pixel, faithful_primitives::image_sample(&item, probe["x"].as_u64().unwrap() as u32, probe["y"].as_u64().unwrap() as u32)); for channel in 0..4 { assert!(pixel[channel].abs_diff(probe["rgba"][channel].as_u64().unwrap() as u8) <= probe["tolerance"].as_u64().unwrap() as u8); } }
    }
}

fn artifact_request(kind: LayoutExportKind) -> LayoutExportRequest { let row = corpus(); let snapshot = semio_framework_pack_json::from_json_str(&row["artifact"]["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(); LayoutExportRequest { kind, page_id: Some("page-1".into()), snapshot: Arc::new(snapshot), preflight_json: None, parent_document_id: "fidelity".into(), canonical_base_revision_hex: "19".repeat(32) } }
fn operation() -> Operation { Operation::new(semio_framework_job::OperationId(92), semio_framework_job::RevisionId(19), semio_framework_job::Generation(3), 17) }
#[test]
fn complete_svg_and_png_job_preserve_real_text_and_transparent_rotated_images() {
    let svg = run_layout_export_headless_batch(operation(), artifact_request(LayoutExportKind::Svg)).unwrap(); assert!(svg.data.contains('Q'), "text must export as genuine font curves"); assert!(svg.data.contains("<image")); assert!(svg.data.contains("data:image/png;base64,"));
    let png = run_layout_export_headless_batch(operation(), artifact_request(LayoutExportKind::Png)).unwrap(); let bytes = decode_base64(&png.data).unwrap(); let image = semio_framework_pixels::decode_png(&bytes).unwrap(); assert_eq!((image.width, image.height), (64, 48));
    for probe in corpus()["artifact"]["probes"].as_array().unwrap() { let x = probe["x"].as_u64().unwrap() as usize; let y = probe["y"].as_u64().unwrap() as usize; let at = (y * image.width as usize + x) * 4; for channel in 0..4 { assert!(image.pixels[at + channel].abs_diff(probe["rgba"][channel].as_u64().unwrap() as u8) <= probe["tolerance"].as_u64().unwrap() as u8); } }
}
#[test]
fn glyph_preparation_checkpoint_replays_and_cancels_with_funded_retirement() {
    let operation = operation(); let request = artifact_request(LayoutExportKind::Svg); let mut caller = layout_export_fixture_caller(operation, operation.generation); let mut job = LayoutExportJob::from_admitted(operation, (operation.operation, operation.generation), request.clone(), ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES));
    let capture_turn=semio_framework_value::RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:32768,..LAYOUT_EXPORT_FIXTURE_TURN};
    let mut checkpoint = None;
    for _ in 0..100_000 { let mut observed = None; caller.turn(capture_turn, |cx| cx.with_original_work_loan(1, u64::MAX, |same| { observed = job.step(same)?.as_ref().map(LayoutExportObserved::of_borrow); Ok(()) })).unwrap(); if let Some(LayoutExportObserved::Checkpoint { state, .. }) = observed { if job.fidelity.as_ref().is_some_and(|state| !state.prepared) { checkpoint = Some(state); break; } } }
    let checkpoint=checkpoint.expect("font admission and contours must yield work-progress checkpoints");let decoded=decode_checkpoint(&operation,&request,&checkpoint).unwrap();assert!(decoded.work_units>decoded.committed_units); caller.cancel().cancel_now(); let mut cancelled = false;
    for _ in 0..64 { if caller.turn(LAYOUT_EXPORT_FIXTURE_TURN, |cx| Ok(job.step(cx)?.as_ref().map(LayoutExportObserved::of_borrow))).unwrap() == Some(LayoutExportObserved::Cancelled) { cancelled = true; break; } } assert!(cancelled);
    job.begin_close(); for _ in 0..100_000 { if job.terminal_is_empty() { break; } caller.turn(LAYOUT_EXPORT_FIXTURE_TURN, |cx| job.close_step(cx)).unwrap(); } assert!(job.terminal_is_empty()); caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);
    let restored = LayoutExportJob::restore(operation, request.clone(), &checkpoint).unwrap(); let chunks = restored.output_chunks.as_ref().unwrap().clone(); let (outcome, resumed) = drive_job_batch(restored, operation, operation.generation, false, "layout.export.fidelity.restore", || LayoutExportCommit::from_chunks(LayoutExportKind::Svg, "fidelity", &chunks).unwrap().data); assert!(matches!(outcome, LayoutExportObserved::Complete { .. })); assert_eq!(resumed, run_layout_export_headless_batch(operation, request).unwrap().data);
}

fn rendered_segments<'a>(segments: impl Iterator<Item = &'a PathSegment>) -> tiny_skia::Pixmap {
    let mut builder = tiny_skia::PathBuilder::new(); for segment in segments { match segment { PathSegment::Move { to } => builder.move_to(to[0] as f32, to[1] as f32), PathSegment::Line { to } => builder.line_to(to[0] as f32, to[1] as f32), PathSegment::Quad { ctrl, to } => builder.quad_to(ctrl[0] as f32, ctrl[1] as f32, to[0] as f32, to[1] as f32), PathSegment::Cubic { ctrl1, ctrl2, to } => builder.cubic_to(ctrl1[0] as f32, ctrl1[1] as f32, ctrl2[0] as f32, ctrl2[1] as f32, to[0] as f32, to[1] as f32), PathSegment::Close => builder.close(), _ => panic!("unresolved path") } }
    let mut image = tiny_skia::Pixmap::new(64, 48).unwrap(); image.fill(tiny_skia::Color::WHITE); if let Some(path) = builder.finish() { image.fill_path(&path, &tiny_skia::Paint::default(), tiny_skia::FillRule::Winding, tiny_skia::Transform::identity(), None); } image
}

#[test]
fn export_plan_carries_text_frame_rotation_without_blueprint_chrome() {
    let fixture = corpus(); let snapshot: LayoutSnapshot = semio_framework_pack_json::from_json_str(&fixture["rotatedArtifact"]["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut request = artifact_request(LayoutExportKind::Svg); request.snapshot = Arc::new(snapshot); let snapshot_owner = Arc::clone(&request.snapshot);
    let operation = operation(); let mut job = LayoutExportJob::from_admitted(operation, (operation.operation, operation.generation), request, ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES));
    for _ in 0..100_000 { if job.stage == ExportStage::BeginEncode { break; } match job.stage { ExportStage::Validate => job.validate_one().unwrap(), ExportStage::Plan => job.plan_one().unwrap(), _ => panic!("unexpected stage") } }
    let expected_rotation = fixture["rotatedArtifact"]["expectedRotation"].as_f64().unwrap() as f32; let expected_center = [28.0, 20.0]; let mut glyphs = 0;
    for item in job.pdf_items.iter() { match item { PdfItem::Outline { rotation, center, .. } => { glyphs += 1; assert!((rotation - expected_rotation).abs() < 1e-6); assert_eq!(*center, expected_center); }, PdfItem::Rect { .. } => panic!("text frame export must not need blueprint chrome"), _ => {} } } assert!(glyphs > 0);
    let cancelled = drive_job_batch(job, operation, operation.generation, true, "layout.export.rotation.cancel", || ()).0; assert_eq!(cancelled, LayoutExportObserved::Cancelled); drop(snapshot_owner);
}
#[test]
fn fidelity_original_close_dispatch_handles_present_and_absent_state() {
    for present in [false, true] { let operation = operation(); let request = artifact_request(LayoutExportKind::Svg); let snapshot_owner = Arc::clone(&request.snapshot); let mut caller = layout_export_fixture_caller(operation, operation.generation); let mut job = LayoutExportJob::from_admitted(operation, (operation.operation, operation.generation), request, ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES)); if present { job.fidelity = Some(FidelityState::new()); } job.begin_close();
        for _ in 0..100_000 { if job.terminal_is_empty() { break; } caller.turn(LAYOUT_EXPORT_FIXTURE_TURN, |cx| job.close_step(cx)).unwrap(); } assert!(job.terminal_is_empty(), "present={present}"); assert!(job.fidelity.is_none()); assert!(job.fidelity_retirement.is_none()); caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK); drop(snapshot_owner);
    }
    for kind in [LayoutExportKind::Svg, LayoutExportKind::Png, LayoutExportKind::Pdf, LayoutExportKind::Package] { run_layout_export_headless_batch(operation(), artifact_request(kind)).unwrap(); }
}
