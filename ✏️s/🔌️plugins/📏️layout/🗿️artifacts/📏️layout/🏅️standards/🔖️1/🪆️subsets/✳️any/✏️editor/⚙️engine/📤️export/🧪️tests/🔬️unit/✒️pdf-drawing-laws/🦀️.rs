/// ✒️ Pdf drawing laws: the background drawing, the styled stroke, the filled path, the placed drawing inside its frame, the drawing text, the embedded and rotated pngs, the front rectangle and the dictionary and fixture helpers.

#[test]
fn pdf_export_prints_the_background_drawing() {
    bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);
    use semio_framework_io_ports::VectorPoint2;
    use semio_framework_io_ports::VectorTransform;
    use semio_framework_io_ports::{VectorLayer, VectorNode, VectorScene, VectorSegment};
    let point = |x: f64, y: f64| VectorPoint2 { x, y };
    let content = VectorScene {
        schema: "test.scene".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![VectorLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: VectorNode::Group {
                transform: VectorTransform::identity(),
                children: vec![VectorNode::Path {
                    style: None,
                    segments: vec![
                        VectorSegment::MoveTo { to: point(10.0, 20.0) },
                        VectorSegment::LineTo { to: point(110.0, 20.0) },
                        VectorSegment::LineTo { to: point(110.0, 70.0) },
                        VectorSegment::LineTo { to: point(10.0, 70.0) },
                        VectorSegment::Close,
                    ],
                }],
            },
        }],
    };
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.pages[0].width = 100.0;
    snapshot.pages[0].height = 50.0;
    snapshot.background_drawing = Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("dwg", &scene_content(&content)));
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("0.000 50.000 m 100.000 50.000 l 100.000 0.000 l 0.000 0.000 l h S Q"), "{text}");
}

#[test]
fn pdf_export_prints_a_styled_plan_stroke() {
    bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);
    use semio_framework_io_ports::VectorPoint2;
    use semio_framework_io_ports::VectorRgba;
    use semio_framework_io_ports::VectorTransform;
    use semio_framework_io_ports::{VectorLayer, VectorNode, VectorScene, VectorSegment, VectorStyle};
    let point = |x: f64, y: f64| VectorPoint2 { x, y };
    let content = VectorScene {
        schema: "test.scene".into(),
        canvas: Default::default(),
        styles: vec![VectorStyle { name: "red".into(), fill: None, stroke: Some(VectorRgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }), stroke_width: Some(2.0), opacity: Some(0.5) }],
        layers: vec![VectorLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: VectorNode::Group {
                transform: VectorTransform::identity(),
                children: vec![VectorNode::Path {
                    style: Some("red".into()),
                    segments: vec![
                        VectorSegment::MoveTo { to: point(10.0, 20.0) },
                        VectorSegment::LineTo { to: point(110.0, 20.0) },
                        VectorSegment::LineTo { to: point(110.0, 70.0) },
                        VectorSegment::LineTo { to: point(10.0, 70.0) },
                        VectorSegment::Close,
                    ],
                }],
            },
        }],
    };
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.pages[0].width = 100.0;
    snapshot.pages[0].height = 50.0;
    snapshot.background_drawing = Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("dwg", &scene_content(&content)));
    let mut engine = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::LayoutEngine::new();
    let list = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::build_display_list_for_page(&mut engine, &snapshot, &snapshot.pages[0], "", &[], None, false);
    assert_eq!(list.strokes[0].color, [1.0, 0.0, 0.0, 0.5]);
    assert_eq!(list.strokes[0].width, 2.0);
    let json = semio_s_artifact_layout_layout_editor_commands::canvas::canvas_layers(
        &snapshot,
        &semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::config::LayoutWindowConfig::default(),
        &semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::transient::LayoutWindowTransient::default(),
        &semio_s_artifact_layout_layout_editor_commands::LayoutInteractionSnapshot::default(),
        false,
    );
    assert!(json.contains("drawing.plan.0") && json.contains("[1.0,0.0,0.0,0.5]") && json.contains("\"width\":2.0"), "{json}");
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("1.0000 0.0000 0.0000 RG 2.000 w"), "{text}");
}

#[test]
fn pdf_export_prints_a_filled_plan_path() {
    bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);
    use semio_framework_io_ports::VectorPoint2;
    use semio_framework_io_ports::VectorRgba;
    use semio_framework_io_ports::VectorTransform;
    use semio_framework_io_ports::{VectorLayer, VectorNode, VectorScene, VectorSegment, VectorStyle};
    let point = |x: f64, y: f64| VectorPoint2 { x, y };
    let content = VectorScene {
        schema: "test.scene".into(),
        canvas: Default::default(),
        styles: vec![VectorStyle { name: "blue".into(), fill: Some(VectorRgba { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }), stroke: None, stroke_width: None, opacity: None }],
        layers: vec![VectorLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: VectorNode::Group {
                transform: VectorTransform::identity(),
                children: vec![VectorNode::Path {
                    style: Some("blue".into()),
                    segments: vec![
                        VectorSegment::MoveTo { to: point(10.0, 20.0) },
                        VectorSegment::LineTo { to: point(110.0, 20.0) },
                        VectorSegment::LineTo { to: point(110.0, 70.0) },
                        VectorSegment::LineTo { to: point(10.0, 70.0) },
                        VectorSegment::Close,
                    ],
                }],
            },
        }],
    };
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.pages[0].width = 100.0;
    snapshot.pages[0].height = 50.0;
    snapshot.background_drawing = Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("dwg", &scene_content(&content)));
    let mut engine = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::LayoutEngine::new();
    let list = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::build_display_list_for_page(&mut engine, &snapshot, &snapshot.pages[0], "", &[], None, false);
    assert_eq!(list.strokes[0].fill, Some([0.0, 0.0, 1.0, 1.0]));
    let json = semio_s_artifact_layout_layout_editor_commands::canvas::canvas_layers(
        &snapshot,
        &semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::config::LayoutWindowConfig::default(),
        &semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::transient::LayoutWindowTransient::default(),
        &semio_s_artifact_layout_layout_editor_commands::LayoutInteractionSnapshot::default(),
        false,
    );
    assert!(json.contains("drawing.plan.0") && json.contains("[0.0,0.0,1.0,1.0]"), "{json}");
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("0.0000 0.0000 1.0000 rg") && text.contains("f Q"), "{text}");
}

#[test]
fn pdf_export_prints_a_placed_drawing_inside_its_frame() {
    bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);
    use semio_framework_io_ports::VectorPoint2;
    use semio_framework_io_ports::VectorTransform;
    use semio_framework_io_ports::{VectorLayer, VectorNode, VectorScene, VectorSegment};
    let point = |x: f64, y: f64| VectorPoint2 { x, y };
    let content = VectorScene {
        schema: "test.scene".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![VectorLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: VectorNode::Group {
                transform: VectorTransform::identity(),
                children: vec![VectorNode::Path {
                    style: None,
                    segments: vec![
                        VectorSegment::MoveTo { to: point(0.0, 0.0) },
                        VectorSegment::LineTo { to: point(1.0, 0.0) },
                        VectorSegment::LineTo { to: point(1.0, 1.0) },
                        VectorSegment::LineTo { to: point(0.0, 1.0) },
                        VectorSegment::Close,
                    ],
                }],
            },
        }],
    };
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.links[0].artifact_kind = "s.draw.drawing".into();
    snapshot.links[0].artifact_ref = "plan-1".into();
    snapshot.background_drawing = Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("dwg", &scene_content(&content)));
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("146.000 65.000 m 186.000 65.000 l 186.000 25.000 l 146.000 25.000 l h S Q"), "{text}");
}

#[test]
fn pdf_export_prints_drawing_text() {
    bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);
    use semio_framework_io_ports::VectorPoint2;
    use semio_framework_io_ports::VectorTransform;
    use semio_framework_io_ports::{VectorLayer, VectorNode, VectorScene, VectorSegment};
    let point = |x: f64, y: f64| VectorPoint2 { x, y };
    let content = VectorScene {
        schema: "test.scene".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![VectorLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: VectorNode::Group {
                transform: VectorTransform::identity(),
                children: vec![
                    VectorNode::Path {
                        style: None,
                        segments: vec![
                            VectorSegment::MoveTo { to: point(0.0, 0.0) },
                            VectorSegment::LineTo { to: point(1.0, 0.0) },
                            VectorSegment::LineTo { to: point(1.0, 1.0) },
                            VectorSegment::LineTo { to: point(0.0, 1.0) },
                            VectorSegment::Close,
                        ],
                    },
                    VectorNode::Text { value: "Plan".into(), at: point(0.0, 0.0), style: None },
                ],
            },
        }],
    };
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.links[0].artifact_kind = "s.draw.drawing".into();
    snapshot.links[0].artifact_ref = "plan-1".into();
    snapshot.background_drawing = Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("dwg", &scene_content(&content)));
    let mut engine = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::LayoutEngine::new();
    let list = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::build_display_list_for_page(&mut engine, &snapshot, &snapshot.pages[0], "", &[], None, false);
    let glyphs = list.text_runs.iter().map(|run| run.glyphs.len()).sum::<usize>();
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert_eq!(text.matches("> Tj ET").count(), glyphs);
    assert!(glyphs >= 12 + 8, "{glyphs}");
}

#[test]
fn pdf_export_prints_an_embedded_drawing_png() {
    bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);
    use semio_framework_io_ports::VectorPoint2;
    use semio_framework_io_ports::VectorTransform;
    use semio_framework_io_ports::{VectorLayer, VectorNode, VectorScene, VectorSegment};
    let point = |x: f64, y: f64| VectorPoint2 { x, y };
    let encoded = semio_framework_pixels::RasterImage { width: 1, height: 1, pixels: vec![255, 0, 0, 255] };
    let bytes = semio_framework_pixels::encode_png(&encoded).expect("png");
    let content = VectorScene {
        schema: "test.scene".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![VectorLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: VectorNode::Group {
                transform: VectorTransform::identity(),
                children: vec![
                    VectorNode::Path {
                        style: None,
                        segments: vec![VectorSegment::MoveTo { to: point(0.0, 0.0) }, VectorSegment::LineTo { to: point(1.0, 0.0) }, VectorSegment::LineTo { to: point(1.0, 1.0) }, VectorSegment::LineTo { to: point(0.0, 1.0) }, VectorSegment::Close],
                    },
                    VectorNode::Image { at: point(0.0, 0.0), width: 1.0, height: 1.0, mime: "image/png".into(), bytes },
                ],
            },
        }],
    };
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.links[0].artifact_kind = "s.draw.drawing".into();
    snapshot.links[0].artifact_ref = "plan-1".into();
    snapshot.background_drawing = Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("dwg", &scene_content(&content)));
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let pdf = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&pdf);
    let text = String::from_utf8_lossy(&pdf);
    assert!(text.contains("q 40.000 0 0 40.000 146.000 25.000 cm"), "{text}");
    assert!(text.contains("FF0000"), "{text}");
}

#[test]
fn pdf_export_prints_a_rotated_drawing_png() {
    bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);
    use semio_framework_io_ports::VectorPoint2;
    use semio_framework_io_ports::VectorTransform;
    use semio_framework_io_ports::{VectorLayer, VectorNode, VectorScene, VectorSegment};
    let point = |x: f64, y: f64| VectorPoint2 { x, y };
    let encoded = semio_framework_pixels::RasterImage { width: 1, height: 1, pixels: vec![255, 0, 0, 255] };
    let bytes = semio_framework_pixels::encode_png(&encoded).expect("png");
    let content = VectorScene {
        schema: "test.scene".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![VectorLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: VectorNode::Group {
                transform: VectorTransform::identity(),
                children: vec![
                    VectorNode::Path {
                        style: None,
                        segments: vec![VectorSegment::MoveTo { to: point(0.0, 0.0) }, VectorSegment::LineTo { to: point(1.0, 0.0) }, VectorSegment::LineTo { to: point(1.0, 1.0) }, VectorSegment::LineTo { to: point(0.0, 1.0) }, VectorSegment::Close],
                    },
                    VectorNode::Image { at: point(0.0, 0.0), width: 1.0, height: 1.0, mime: "image/png".into(), bytes },
                ],
            },
        }],
    };
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.links[0].artifact_kind = "s.draw.drawing".into();
    snapshot.links[0].artifact_ref = "plan-1".into();
    let frame = snapshot.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-image-1").expect("image");
    let semio_s_artifact_layout_layout_model::Frame::Image { bounds, .. } = frame else { panic!("image") };
    bounds.rotation = std::f64::consts::FRAC_PI_2;
    snapshot.background_drawing = Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("dwg", &scene_content(&content)));
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let pdf = decode_base64(&all.data).expect("base64 pdf");
    let text = String::from_utf8_lossy(&pdf);
    assert!(text.contains("q 0.000 -40.000 40.000 0.000 146.000 65.000 cm"), "{text}");
    assert!(text.contains("FF0000"), "{text}");
}

#[test]
fn pdf_export_prints_edited_drawing_text() {
    bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);
    use semio_s_artifact_layout_layout_mutations_frames::set_drawing_text::SetDrawingText;
    use semio_s_artifact_layout_layout_mutations::LayoutMutation;
    use protocol::{ColdMutation, Mutation, MutationDiff};
    use semio_framework_io_ports::VectorPoint2;
    use semio_framework_io_ports::VectorTransform;
    use semio_framework_io_ports::{VectorLayer, VectorNode, VectorScene};
    let point = |x: f64, y: f64| VectorPoint2 { x, y };
    let content = VectorScene {
        schema: "test.scene".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![VectorLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: VectorNode::Group { transform: VectorTransform::identity(), children: vec![VectorNode::Text { value: "Plan".into(), at: point(0.0, 0.0), style: None }] },
        }],
    };
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.background_drawing = Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("dwg", &scene_content(&content)));
    let mutation = LayoutMutation::SetDrawingText(SetDrawingText { index: 0, text: "Title".into() });
    let edited = protocol::apply_diff_cold(mutation.diff(&snapshot).diff(), &snapshot).expect("rename");
    let mut engine = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::LayoutEngine::new();
    let list = semio_s_artifact_layout_layout_engine::scene::layout_engine_and_views::build_display_list_for_page(&mut engine, &edited, &edited.pages[0], "", &[], None, false);
    assert!(list.text_runs.iter().any(|run| run.content == "Title"));
    let json = semio_s_artifact_layout_layout_editor_commands::canvas::canvas_layers(
        &edited,
        &semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::config::LayoutWindowConfig::default(),
        &semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::transient::LayoutWindowTransient::default(),
        &semio_s_artifact_layout_layout_editor_commands::LayoutInteractionSnapshot::default(),
        false,
    );
    assert!(json.contains("Title"), "{json}");
    let glyphs = list.text_runs.iter().map(|run| run.glyphs.len()).sum::<usize>();
    let all = headless_batch_export(LayoutExportKind::Pdf, &edited, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    assert_eq!(text.matches("> Tj ET").count(), glyphs);
}

#[test]
fn pdf_export_prints_a_front_rect_after_the_image() {
    use semio_s_artifact_layout_layout_mutations_frames::reorder_frame::ReorderFrame;
    use semio_s_artifact_layout_layout_mutations::LayoutMutation;
    use protocol::{ColdMutation, Mutation, MutationDiff};
    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    for _ in 0..8 {
        let mutation = LayoutMutation::ReorderFrame(ReorderFrame { page_id: "page-1".into(), frame_id: "frame-1".into(), forward: true });
        let outcome = mutation.diff(&snapshot);
        if outcome.diff().pages.is_none() {
            break;
        }
        snapshot = protocol::apply_diff_cold(outcome.diff(), &snapshot).expect("forward");
    }
    let all = headless_batch_export(LayoutExportKind::Pdf, &snapshot, Some("page-1"), None).expect("page pdf");
    let bytes = decode_base64(&all.data).expect("base64 pdf");
    assert_pdf_structure(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    let image = text.find("0.92 0.88 0.84 rg 136.000 25.000 60.000 40.000 re f Q").expect("image");
    let rect = text.find(" rg 10.000 450.000 40.000 40.000 re f Q").expect("rect");
    assert!(rect > image, "the front rectangle follows the image in the content stream");
}

fn dictionary_with_nulls(count:usize)->semio_framework_value_dictionary::QuestionDictionary{semio_framework_value_dictionary::QuestionDictionary{entries:vec![semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:String::new(),value:semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::Null;count])}]}}
fn dictionary_at_json_bytes(count:usize)->semio_framework_value_dictionary::QuestionDictionary{
 let mut dictionary=semio_framework_value_dictionary::QuestionDictionary{entries:(0..8).map(|index|semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:index.to_string(),value:semio_framework_value::DslValue::String(String::new())}).collect()};
 let base=semio_framework_pack_json::to_json_string(&dictionary).len();let mut remaining=count.checked_sub(base).expect("dictionary wrapper fits byte witness");
 for entry in &mut dictionary.entries{let bytes=remaining.min(MAX_LAYOUT_EXPORT_STRING_BYTES);entry.value=semio_framework_value::DslValue::String("a".repeat(bytes));remaining-=bytes;}
 assert_eq!(remaining,0);assert_eq!(semio_framework_pack_json::to_json_string(&dictionary).len(),count);dictionary
}


fn fixture_export_job(operation:Operation,request:LayoutExportRequest)->LayoutExportJob{
 LayoutExportJob::from_admitted(operation,(operation.operation,operation.generation),request,ArtifactOutputChunks::new(MAX_LAYOUT_EXPORT_OUTPUT_BYTES))
}
