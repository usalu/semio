use protocol::{ColdMutation as _,ColdMutationDiff as _,ColdDiffAlgebra as _};
/// ✒️ Actual Drawing issuer reads preserve authored PDF geometry, paint, Unicode and explicit unresolved sample refusal.
fn original_pdf_fixture(name:&str)->serde_json::Value{let laws:serde_json::Value=serde_json::from_str(include_str!("../../../📖️drawing-read/🧪️pdf.json")).unwrap();laws["cases"].as_array().unwrap().iter().find(|row|row["name"]==name).unwrap().clone()}
fn original_pdf_snapshot(placed:bool)->semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot{
 let mut snapshot=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();let transport:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🧬️schema/🧬️mutations/🖊️set-page-background-drawing/🧪️tests/🔣️transport.json")).unwrap();snapshot.background_drawing=Some(semio_framework_pack_json::from_json_str(&transport["cases"][0]["input"]["SetPageBackgroundDrawing"]["drawing"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
 if placed{snapshot.links[0].artifact_kind="s.draw.drawing".into();snapshot.links[0].artifact_ref="plan-1".into();}else{snapshot.pages[0].width=100.0;snapshot.pages[0].height=50.0;}snapshot
}
fn original_pdf_content(row:&serde_json::Value)->semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot{semio_framework_pack_json::from_json_str(&row["content"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn with_original_pdf_source<T>(actual:semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot,body:impl FnOnce(&semio_framework_io_ports::ChildAdmissionRead<'_>)->T)->T{
 use semio_s_artifact_draw_drawing_editor::child_editor::DRAWING_CHILD_VECTOR_READ;
 let payload=Arc::new(actual);let registry=store::os_store::snapshot_read::SnapshotReadRegistryHandle::new();let lease=registry.try_issue(Arc::clone(&payload)).unwrap_or_else(|_|panic!("actual original Drawing registry"));let read=store::os_store::snapshot_read::ErasedSnapshotRead::new(Arc::clone(&payload),lease).try_into_typed::<semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot>().unwrap_or_else(|_|panic!("actual typed Drawing read"));let(mut source,_)=read.admit_retained_clone_source(LAYOUT_EXPORT_FIXTURE_TURN).unwrap_or_else(|_|panic!("original Drawing source"));
 let result=body(&semio_framework_io_ports::ChildAdmissionRead{source:semio_framework_io_ports::erase_child_admission_source(source.borrow(),0x44524157),admitter:DRAWING_CHILD_VECTOR_READ.admission,vector:DRAWING_CHILD_VECTOR_READ.vector});
 while !source.terminal_is_empty(){let demand=source.next_close_demand(LAYOUT_EXPORT_FIXTURE_TURN.maximum_copy_bytes).unwrap();source.close_step(RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}).unwrap();}assert!(registry.terminal_is_empty());drop(payload);let mut close=Some(store::os_store::snapshot_read::SnapshotReadRegistryAliasRetirement::new(registry));while close.is_some(){store::os_store::snapshot_read::snapshot_registry_alias_close_step(&mut close,LAYOUT_EXPORT_FIXTURE_TURN).unwrap();}result
}
fn original_drawing_pdf(snapshot:&semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot,actual:semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot)->Result<Vec<u8>,String>{
 with_original_pdf_source(actual,|read|{let op=operation();let request=LayoutExportRequest{kind:LayoutExportKind::Pdf,page_id:Some(snapshot.pages[0].id.clone()),snapshot:Arc::new(snapshot.clone()),preflight_json:None,parent_document_id:"actual-layout".into(),canonical_base_revision_hex:"29".repeat(32)};let witness=Arc::clone(&request.snapshot);let mut job=fixture_export_job(op,request);let mut caller=layout_export_fixture_caller(op,op.generation);let mut result=Err("original Drawing export did not complete".into());
  for _ in 0..LAYOUT_EXPORT_FIXTURE_TURN_BUDGET{let turn=caller.turn(LAYOUT_EXPORT_FIXTURE_TURN,|cx|Ok(job.advance_original_work(cx,Some(read)))).unwrap();match turn{Ok(LayoutExportWorkTurn::Ready)=>{result=LayoutExportCommit::from_chunks(LayoutExportKind::Pdf,&snapshot.name,job.output_chunks.as_ref().unwrap()).and_then(|all|decode_base64(&all.data));break},Ok(_)=>{},Err(error)=>{result=Err(format!("{error:?}"));break}}}close_job(job,&mut caller);caller.retire(LAYOUT_EXPORT_FIXTURE_STOCK);drop(witness);result})
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

fn original_drawing_canvas(snapshot:&semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot,actual:semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot)->(Vec<String>,usize){
 use semio_framework_value::retirement::{RetireOwned,RetirementStep};use semio_s_artifact_layout_layout_engine::scene::drawing_planning::{DrawingSourceAddress,DrawingPlacement,composition::{DrawingCompositionState,DrawingCompositionTurn,canvas}};
 with_original_pdf_source(actual,|read|{let mut state=DrawingCompositionState::new();state.begin(DrawingSourceAddress::Document,DrawingPlacement::Content,semio_s_artifact_layout_layout_model::LayoutBounds{x:0.0,y:0.0,width:snapshot.pages[0].width,height:snapshot.pages[0].height,rotation:0.0}).unwrap();let grant=semio_framework_io_ports::VectorSceneGrant{maximum_source_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128};let mut paths=Vec::new();let mut glyphs=0;
  for _ in 0..1000000{if !state.has_active(){break}match state.advance(snapshot,read,grant).unwrap(){DrawingCompositionTurn::Path{index}=>paths.push(canvas::path_end(state.path(index).unwrap().info.paint).unwrap().as_str().to_owned()),DrawingCompositionTurn::Glyph{..}=>glyphs+=1,_=>{}}}assert!(!state.has_active());let mut stack=vec![state.retirement()];for _ in 0..1000000{let Some(cursor)=stack.last_mut()else{break};match cursor.close_step(LAYOUT_EXPORT_FIXTURE_TURN){RetirementStep::Complete=>{assert!(cursor.terminal_is_empty());stack.pop();},RetirementStep::Child(child)|RetirementStep::ChildProgress(child,_)=>stack.push(child),RetirementStep::Failure(error)=>panic!("{error}"),RetirementStep::BudgetExhausted=>panic!("original composition close grant"),_=>{}}}assert!(stack.is_empty());(paths,glyphs)})
}
#[test]fn pdf_export_prints_the_background_drawing(){let snapshot=original_pdf_snapshot(false);let bytes=original_drawing_pdf(&snapshot,original_pdf_content(&original_pdf_fixture("background"))).unwrap();assert_pdf_structure(&bytes);assert!(pdf_has_operation(&bytes,"m",&[0.0,50.0])&&pdf_has_operation(&bytes,"l",&[100.0,0.0])&&pdf_has_operation(&bytes,"S",&[]));}
#[test]fn pdf_export_prints_a_styled_plan_stroke(){let snapshot=original_pdf_snapshot(false);let row=original_pdf_fixture("stroke");let(canvas,_)=original_drawing_canvas(&snapshot,original_pdf_content(&row));assert!(canvas.iter().any(|row|row.contains("[1,0,0,0.5]")&&row.contains("\"width\":2")));let bytes=original_drawing_pdf(&snapshot,original_pdf_content(&row)).unwrap();assert_pdf_structure(&bytes);assert!(pdf_has_operation(&bytes,"RG",&[1.0,0.0,0.0])&&pdf_has_operation(&bytes,"w",&[2.0]));}
#[test]fn pdf_export_prints_a_filled_plan_path(){let snapshot=original_pdf_snapshot(false);let row=original_pdf_fixture("fill");let(canvas,_)=original_drawing_canvas(&snapshot,original_pdf_content(&row));assert!(canvas.iter().any(|row|row.contains("[0,0,1,1]")));let bytes=original_drawing_pdf(&snapshot,original_pdf_content(&row)).unwrap();assert!(pdf_has_operation(&bytes,"rg",&[0.0,0.0,1.0])&&pdf_operations(&bytes).iter().any(|op|op.operator=="f"||op.operator=="B"));}
#[test]fn pdf_export_prints_a_placed_drawing_inside_its_frame(){let snapshot=original_pdf_snapshot(true);let bytes=original_drawing_pdf(&snapshot,original_pdf_content(&original_pdf_fixture("placed"))).unwrap();assert_pdf_structure(&bytes);assert!(pdf_has_operation(&bytes,"m",&[146.0,65.0])&&pdf_has_operation(&bytes,"l",&[186.0,25.0]));}
#[test]fn pdf_export_prints_drawing_text(){let snapshot=original_pdf_snapshot(true);let row=original_pdf_fixture("text");let(_,glyphs)=original_drawing_canvas(&snapshot,original_pdf_content(&row));assert!(glyphs>=4);let bytes=original_drawing_pdf(&snapshot,original_pdf_content(&row)).unwrap();assert_pdf_structure(&bytes);assert!(pdf_text_compact(&bytes).contains("Plan"));}
#[test]fn pdf_export_refuses_original_drawing_samples_without_a_resolution_owner(){for rotated in[false,true]{let name=if rotated{"rotated-sample"}else{"sample"};let row=original_pdf_fixture(name);assert_eq!(row["content"]["assets"]["red"]["samples"][0],serde_json::json!([255,0,0,255]));let mut snapshot=original_pdf_snapshot(true);if rotated{let frame=snapshot.pages[0].frames.iter_mut().find(|frame|frame.id()=="frame-image-1").unwrap();let semio_s_artifact_layout_layout_model::Frame::Image{bounds,..}=frame else{panic!("image")};bounds.rotation=std::f64::consts::FRAC_PI_2;}let error=original_drawing_pdf(&snapshot,original_pdf_content(&row)).unwrap_err();assert!(error.contains("resolution")||error.contains("resolve"),"{error}");}}
#[test]fn pdf_export_prints_edited_drawing_text(){let snapshot=original_pdf_snapshot(false);let actual=original_pdf_content(&original_pdf_fixture("edited-text"));let mutation=semio_s_artifact_draw_drawing_mutations::drawing_op_for_layer_field(&actual,"text","textContent",&semio_framework_value::DslValue::String("Title".into())).expect("actual Draw text mutation");let edited=protocol::apply_diff_cold(mutation.diff(&actual).diff(),&actual).expect("actual shared Drawing text edit");let(_,glyphs)=original_drawing_canvas(&snapshot,edited.clone());assert!(glyphs>=5);let bytes=original_drawing_pdf(&snapshot,edited).unwrap();assert_pdf_structure(&bytes);assert!(pdf_extracted_text(&bytes).contains("Title"));}
#[test]fn pdf_export_refuses_group_opacity_without_compositing_owner(){let snapshot=original_pdf_snapshot(false);let error=original_drawing_pdf(&snapshot,original_pdf_content(&original_pdf_fixture("compositing"))).unwrap_err();assert!(error.contains("compositing"),"{error}");}
#[test]fn pdf_export_drawing_without_original_read_is_explicitly_refused(){let snapshot=original_pdf_snapshot(false);assert!(headless_batch_export(LayoutExportKind::Pdf,&snapshot,Some("page-1"),None).is_err());}

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
    let operations = pdf_operations(&bytes);
    let position = |x: f64, y: f64| operations.iter().position(|op| op.operator == "m" && op.operands.len() == 2 && op.operands[0].as_float().is_ok_and(|value| (f64::from(value)-x).abs()<0.0001) && op.operands[1].as_float().is_ok_and(|value| (f64::from(value)-y).abs()<0.0001)).unwrap();
    let image = position(136.0, 65.0); let rect = position(10.0, 490.0);
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
