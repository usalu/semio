fn draft(plan:&LayoutImportSourcePlan)->&LayoutSnapshot{match plan{LayoutImportSourcePlan::Document{document,..}=>document,LayoutImportSourcePlan::External{..}=>panic!("mapped document source plan required")}}
fn pending(plan:&LayoutImportSourcePlan)->&[PendingDrawingSource]{match plan{LayoutImportSourcePlan::Document{drawing_sources,..}=>drawing_sources,LayoutImportSourcePlan::External{..}=>panic!("mapped drawing source plan required")}}
//! 🧪️ Language-neutral interchange laws with independent file readers and writers.
use super::*;
#[path = "🔮️oracle/🦀️.rs"] mod oracle;
fn fixtures() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️fixtures/🔣️.json")).unwrap() }
fn seed(case: &serde_json::Value) -> LayoutSnapshot {
    let source = &case["page"];
    let mut doc = document("Interchange Fixture");
    let mut page = page("page-1".into(), source["width"].as_f64().unwrap(), source["height"].as_f64().unwrap());
    text_frame(&mut doc, &mut page, source["text"].as_str().unwrap_or("").into());
    doc.pages.push(page); finish_document(&mut doc); doc
}
#[test]
fn pdf_page_text_matches_the_independent_pdf_reader() {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::{reference_page_count};
    let cases=fixtures(); let case=&cases["cases"][0]; let doc=seed(case);
    let output=export_document("pdf",&doc,&mut |_|true).unwrap();
    assert_eq!(reference_page_count(&output.value).unwrap(),case["expected"]["pages"].as_u64().unwrap() as usize);
    assert_eq!(oracle::pdf_text(&output.value).unwrap().trim(),case["expected"]["text"].as_str().unwrap());
    let imported=import_document("pdf",&output.value,"Read PDF",&mut |_|true).unwrap();
    assert_eq!(draft(&imported.value).stories[0].content,case["expected"]["text"].as_str().unwrap());
    assert_eq!(draft(&imported.value).pages[0].width,doc.pages[0].width);
    assert_eq!(draft(&imported.value).pages[0].height,doc.pages[0].height);
    assert!(!imported.diagnostics.is_empty());
}
#[test]
fn step_sheet_survives_the_independent_part21_reader_and_writer() {
    semio_s_artifact_layout_drawing_composition::bind().unwrap();
    use semio_s_artifact_stdio_step_test_oracle::standards::v_ap214::subsets::base::{oracle_round_trip,project_step_ap214_any};
    let cases=fixtures();let case=&cases["cases"][1];let doc=seed(case);
    let output=export_document("step",&doc,&mut |_|true).unwrap();
    let rewritten=oracle_round_trip(&output.value).unwrap();
    assert_eq!(project_step_ap214_any(&output.value).unwrap(),project_step_ap214_any(&rewritten).unwrap());
    let imported=import_document("step",&rewritten,"Read STEP",&mut |_|true).unwrap();
    assert!((draft(&imported.value).pages[0].width-case["expected"]["width"].as_f64().unwrap()).abs()<1e-9);
    assert!((draft(&imported.value).pages[0].height-case["expected"]["height"].as_f64().unwrap()).abs()<1e-9);
    assert_eq!(imported.fidelity,"projection");
}
#[test]
fn idml_unicode_story_survives_independent_zip_and_xml_writers() {
    use semio_s_artifact_stdio_zip_test_oracle::standards::v2_0::subsets::base::oracle_round_trip as zip_round_trip;
    use semio_s_artifact_stdio_xml_test_oracle::standards::v1_0::subsets::base::{oracle_round_trip as xml_round_trip,project_xml_1_0};
    use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::{decode_zip,encode_zip};
    let cases=fixtures();let case=&cases["cases"][2];let doc=seed(case);
    let output=export_document("idml",&doc,&mut |_|true).unwrap();
    let rewritten=zip_round_trip(&output.value).unwrap();
    let mut archive=decode_zip(&rewritten).unwrap();
    archive.entries.iter_mut().find(|entry|entry.name=="mimetype").unwrap().metadata.compression_method=0;
    for entry in &mut archive.entries {
        if entry.name.ends_with(".xml") {
            let rewritten=xml_round_trip(&entry.data).unwrap();
            assert_eq!(project_xml_1_0(&entry.data).unwrap(),project_xml_1_0(&rewritten).unwrap());
            entry.data=rewritten;
        }
    }
    let imported=import_document("idml",&encode_zip(&archive).unwrap(),"Read IDML",&mut |_|true).unwrap();
    assert_eq!(draft(&imported.value).pages.len(),1);
    assert_eq!(draft(&imported.value).stories[0].content,case["expected"]["text"].as_str().unwrap());
    assert_eq!(draft(&imported.value).pages[0].frames.len(),1);
    assert_eq!(draft(&imported.value).pages[0].width,doc.pages[0].width);
}
#[test]
fn idml_refuses_archive_member_traversal() {
    use semio_s_artifact_stdio_zip::{ZipSnapshot,schema::snapshot::ZipEntry,standards::v2_0::subsets::base::io::encode_zip};
    let cases=fixtures();let case=&cases["cases"][3];let mut archive=ZipSnapshot::default();
    archive.entries.push(ZipEntry{name:case["member"].as_str().unwrap().into(),data:Vec::new(),..ZipEntry::default()});
    let error=import_document("idml",&encode_zip(&archive).unwrap(),"Unsafe IDML",&mut |_|true).unwrap_err();
    assert_eq!(error,case["expectedError"].as_str().unwrap());
}
#[test]
fn cancellation_precedes_decoding_and_encoding() {
    let cases=fixtures();let expected=cases["cases"][4]["expectedError"].as_str().unwrap();
    assert_eq!(import_document("pdf",b"invalid","Cancelled",&mut |_|false).unwrap_err(),expected);
    let doc=seed(&cases["cases"][0]);
    assert_eq!(export_document("pdf",&doc,&mut |_|false).unwrap_err(),expected);
}

#[test]
fn w3c_accepted_rectangle_fixture_survives_canonical_svg_and_independent_xml() {
    use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::{parse_svg_xml,write_svg_xml};
    use semio_s_artifact_stdio_xml_test_oracle::standards::v1_0::subsets::base::{oracle_round_trip,project_xml_1_0};
    let fixture=include_str!("../🧪️fixtures/w3c-rect.svg");
    let document=parse_svg_xml(fixture).unwrap();
    let written=write_svg_xml(&document).unwrap();
    let independent=oracle_round_trip(written.as_bytes()).unwrap();
    assert_eq!(project_xml_1_0(written.as_bytes()).unwrap(),project_xml_1_0(&independent).unwrap());
    assert_eq!(parse_svg_xml(std::str::from_utf8(&independent).unwrap()).unwrap(),document);
}

#[test]
fn printable_scene_preserves_content_and_center_rotation() {
    use semio_framework_io_ports::{VectorNode,VectorPoint3};
    use semio_s_artifact_layout_layout_model::ImageLink;
    let cases=fixtures();let mut doc=seed(&cases["cases"][0]);let expected=&cases["cases"][6];
    let b=&expected["frame"];let bounds=LayoutBounds{x:b["x"].as_f64().unwrap(),y:b["y"].as_f64().unwrap(),width:b["width"].as_f64().unwrap(),height:b["height"].as_f64().unwrap(),rotation:b["rotation"].as_f64().unwrap()};
    let layer=doc.pages[0].layer_ids[0].clone();
    doc.pages[0].frames.push(Frame::Rect{id:"shape".into(),layer_id:layer.clone(),bounds,fill:Some([0.25,0.5,0.75,0.5]),stroke:None,locked:None,visible:None});
    doc.links.push(ImageLink{id:"image".into(),path:"image.png".into(),hash:String::new(),width:1,height:1,dpi:72,color_profile:None,state:None,proxy_data_url:Some("data:image/png;base64,AQID".into()),artifact_kind:"s.stdio.png".into(),artifact_ref:String::new()});
    doc.pages[0].frames.push(Frame::Image{id:"image-frame".into(),layer_id:layer,bounds:LayoutBounds{x:0.0,y:0.0,width:10.0,height:10.0,rotation:0.0},link_id:"image".into(),locked:None,visible:None});
    let scene=crate::layout_snapshot_to_scene(&doc).unwrap();
    assert_eq!(scene.texts(),vec![expected["expected"]["text"].as_str().unwrap()]);
    let VectorNode::Group{children,..}=&scene.layers[0].root else{panic!("page group")};
    let VectorNode::Group{transform,children:rect,..}=&children[1]else{panic!("frame group")};
    assert_eq!(transform.translation,VectorPoint3{x:30.0,y:30.0,z:0.0});
    assert!((transform.rotation.z-expected["expected"]["quaternion"]["z"].as_f64().unwrap()).abs()<1e-12);
    assert!((transform.rotation.w-expected["expected"]["quaternion"]["w"].as_f64().unwrap()).abs()<1e-12);
    assert!(matches!(rect[0],VectorNode::Path{..}));
    let VectorNode::Group{children:images,..}=&children[2]else{panic!("image group")};
    assert!(matches!(&images[0],VectorNode::Image{bytes,..} if bytes==&vec![1,2,3]));
    assert_eq!(scene.styles[1].fill.unwrap().a,0.5);
}
#[test]
fn idml_center_rotation_round_trips_without_changing_native_units() {
    let cases=fixtures();let mut doc=seed(&cases["cases"][2]);
    let frame=&mut doc.pages[0].frames[0];
    match frame{Frame::Text{bounds,..}=>{bounds.x=10.0;bounds.y=20.0;bounds.width=40.0;bounds.height=20.0;bounds.rotation=std::f64::consts::FRAC_PI_2;},_=>unreachable!()};
    let output=export_document("idml",&doc,&mut |_|true).unwrap();
    let imported=import_document("idml",&output.value,"Rotation",&mut |_|true).unwrap();
    let actual=draft(&imported.value).pages[0].frames[0].bounds();let expected=doc.pages[0].frames[0].bounds();
    for (a,b) in [(actual.x,expected.x),(actual.y,expected.y),(actual.width,expected.width),(actual.height,expected.height),(actual.rotation,expected.rotation)]{assert!((a-b).abs()<1e-10);}
}

#[test]
fn pdf_paint_transform_and_alpha_images_match_the_independent_reader() {
    use semio_s_artifact_stdio_png::{PngSnapshot,standards::v1_2::subsets::any::io::{encode_png,project_png}};
    use semio_s_artifact_layout_layout_model::ImageLink;
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_image_sample_streams;
    let cases=fixtures();let case=&cases["cases"][7];let mut doc=seed(case);let layer=doc.pages[0].layer_ids[0].clone();
    let r=&case["rect"];doc.pages[0].frames.push(Frame::Rect{id:"rect".into(),layer_id:layer.clone(),bounds:LayoutBounds{x:r["x"].as_f64().unwrap(),y:r["y"].as_f64().unwrap(),width:r["width"].as_f64().unwrap(),height:r["height"].as_f64().unwrap(),rotation:r["rotation"].as_f64().unwrap()},fill:Some([0.25,0.5,0.75,1.0]),stroke:None,locked:None,visible:None});
    let mut png=PngSnapshot::default();png.image.width=2;png.image.height=1;png.image.samples=case["image"]["pixels"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u16).collect();
    let source=encode_png(&png).unwrap();doc.links.push(ImageLink{id:"link".into(),path:"fixture.png".into(),hash:String::new(),width:2,height:1,dpi:72,color_profile:None,state:Some("ok".into()),proxy_data_url:Some(format!("data:image/png;base64,{}",store::pack_rt::pack_value_to_base64(&source))),artifact_kind:"s.stdio.png".into(),artifact_ref:String::new()});
    doc.pages[0].frames.push(Frame::Image{id:"image".into(),layer_id:layer,bounds:LayoutBounds{x:70.0,y:40.0,width:20.0,height:10.0,rotation:0.0},link_id:"link".into(),locked:None,visible:None});
    let output=export_document("pdf",&doc,&mut |_|true).unwrap();
    let independent=reference_image_sample_streams(&output.value).unwrap();
    assert!(independent.contains(&(8,vec![255,0,0,0,255,0])));assert!(independent.contains(&(8,vec![255,128])));
    let imported=import_document("pdf",&output.value,"Paint",&mut |_|true).unwrap();assert_eq!(draft(&imported.value).pages[0].frames.len(),2);
    let rect=draft(&imported.value).pages[0].frames.iter().find(|frame|matches!(frame,Frame::Rect{..})).unwrap();
    let b=rect.bounds();assert!((b.rotation-std::f64::consts::FRAC_PI_2).abs()<1e-10);assert!((b.x-10.0).abs()<1e-10);assert!((b.y-20.0).abs()<1e-10);
    let proxy=draft(&imported.value).links[0].proxy_data_url.as_ref().unwrap().strip_prefix("data:image/png;base64,").unwrap();let pixels=project_png(&store::pack_rt::pack_value_from_base64(proxy).unwrap()).unwrap().pixels;
    assert_eq!(pixels,case["expected"]["imagePixels"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>());
}

#[test]
fn streamed_idml_has_bounded_chunks_and_independently_readable_unicode() {
    use semio_s_artifact_stdio_zip_test_oracle::standards::v2_0::subsets::base::project_zip_mutation;
    let cases=fixtures();let case=&cases["cases"][8];let mut doc=seed(&cases["cases"][2]);doc.stories[0].content=case["story"].as_str().unwrap().into();
    let mut session=InterchangeExportSession::new("idml").unwrap();let mut bytes=Vec::new();let mut units=0;
    loop{units+=1;assert!(units<10000);match session.advance(&doc).unwrap(){InterchangeExportAdvance::Chunk(chunk)=>{assert!(chunk.len()<=case["maximumChunkBytes"].as_u64().unwrap()as usize);bytes.extend(chunk);},InterchangeExportAdvance::Progress{..}=>{},InterchangeExportAdvance::Done=>break,}}
    assert!(units>10);project_zip_mutation(&bytes).unwrap();
    let imported=import_document("idml",&bytes,"Stream",&mut |_|true).unwrap();assert_eq!(draft(&imported.value).stories[0].content,doc.stories[0].content);
    while !session.terminal_is_empty(){let release=session.next_release_bytes();assert!(session.close_one(release));}
}
#[test]
fn cancelled_interchange_session_retires_partial_archive_without_publication() {
    let cases=fixtures();let mut doc=seed(&cases["cases"][2]);doc.stories[0].content=cases["cases"][8]["story"].as_str().unwrap().into();
    let mut session=InterchangeExportSession::new("idml").unwrap();for _ in 0..8{assert!(!matches!(session.advance(&doc).unwrap(),InterchangeExportAdvance::Done));}
    while !session.terminal_is_empty(){let release=session.next_release_bytes();assert!(session.close_one(release));}
}

#[test]
fn official_verapdf_witnesses_match_the_pdfjs_page_and_unicode_oracle() {
    let oracle:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🏅️conformance/📖️pdf/pdfjs-oracle.json")).unwrap();
    let inputs:[(&str,&[u8]);3]=[
        ("implementation-limit.pdf",include_bytes!("../../../🧪️tests/🧫️fixtures/🏅️conformance/📖️pdf/implementation-limit.pdf")),
        ("file-trailer.pdf",include_bytes!("../../../🧪️tests/🧫️fixtures/🏅️conformance/📖️pdf/file-trailer.pdf")),
        ("text-state-negative.pdf",include_bytes!("../../../🧪️tests/🧫️fixtures/🏅️conformance/📖️pdf/text-state-negative.pdf")),
    ];
    for (name,bytes) in inputs {
        let expected=oracle["fixtures"].as_array().unwrap().iter().find(|fixture|fixture["file"]==name).unwrap();
        let imported=import_document("pdf",bytes,name,&mut |_|true).unwrap();
        assert_eq!(draft(&imported.value).pages.len(),expected["pageCount"].as_u64().unwrap()as usize);
        for (actual,expected) in draft(&imported.value).pages.iter().zip(expected["pages"].as_array().unwrap()) {
            let view=expected["view"].as_array().unwrap();
            assert_eq!(actual.width,view[2].as_f64().unwrap()-view[0].as_f64().unwrap());
            assert_eq!(actual.height,view[3].as_f64().unwrap()-view[1].as_f64().unwrap());
        }
        let expected=expected["pages"][0]["text"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect::<String>();
        let actual=draft(&imported.value).stories.iter().map(|story|story.content.as_str()).collect::<String>();
        assert_eq!(actual,expected);
        assert!(imported.diagnostics.iter().any(|diagnostic|diagnostic.code=="layout.pdf.editable-projection"));
    }
}

#[test]
fn official_nist_ap203_curved_geometry_becomes_a_scaled_orthographic_trace() {
    use semio_s_artifact_stdio_contract::part21::parse_part21;
    use semio_s_artifact_stdio_step_test_oracle::standards::v_ap214::subsets::base::oracle_round_trip;
    let bytes=include_bytes!("../../../🧪️tests/🧫️fixtures/🏅️conformance/📐️step/nist-ftc11-ap203.stp");
    let rewritten=oracle_round_trip(bytes).unwrap();
    let graph=parse_part21(std::str::from_utf8(&rewritten).unwrap()).unwrap();
    assert!(graph.header.file_schema.iter().filter_map(|value|value.as_list()).flatten().any(|value|value.as_str()==Some("CONFIG_CONTROL_DESIGN")));
    semio_s_artifact_layout_drawing_composition::bind().unwrap();
    let imported=import_document("step",&rewritten,"NIST AP203",&mut |_|true).unwrap();
    let page=&draft(&imported.value).pages[0];
    assert!((page.width-63.0*72.0/25.4).abs()<1e-8);
    assert!((page.height-63.0*72.0/25.4).abs()<1e-8);
    let trace=&pending(&imported.value)[0].scene;
    let semio_framework_io_ports::VectorNode::Group{children,..}=&trace.layers[0].root else{panic!("projection group")};
    assert_eq!(children.len(),6);
    assert!(children.iter().all(|node|matches!(node,semio_framework_io_ports::VectorNode::Path{segments,..}if segments.len()==65)));
    assert!(imported.diagnostics.iter().any(|diagnostic|diagnostic.code=="layout.step.source-schema"&&diagnostic.message.contains("CONFIG_CONTROL_DESIGN")));
}

#[test]
fn idml_rgba_rectangle_paint_round_trips_through_both_export_routes() {
    use semio_s_artifact_stdio_zip_test_oracle::standards::v2_0::subsets::base::project_zip_mutation;
    let cases=fixtures();let case=cases["cases"].as_array().unwrap().iter().find(|case|case["id"]=="idml-rgba-rectangle").unwrap();let mut doc=seed(&cases["cases"][2]);let layer_id=doc.pages[0].layer_ids[0].clone();
    let fill=Some(std::array::from_fn(|index|case["fill"][index].as_f64().unwrap()as f32));let stroke=Some(std::array::from_fn(|index|case["stroke"][index].as_f64().unwrap()as f32));
    doc.pages[0].frames.push(Frame::Rect{id:"rgb".into(),layer_id,bounds:LayoutBounds{x:10.0,y:10.0,width:20.0,height:30.0,rotation:0.0},fill,stroke,visible:None,locked:None});
    let one_shot=export_document("idml",&doc,&mut |_|true).unwrap().value;let mut session=InterchangeExportSession::new("idml").unwrap();let mut streamed=Vec::new();loop{match session.advance(&doc).unwrap(){InterchangeExportAdvance::Chunk(bytes)=>streamed.extend(bytes),InterchangeExportAdvance::Progress{..}=>{},InterchangeExportAdvance::Done=>break,}}
    for bytes in [one_shot,streamed]{project_zip_mutation(&bytes).unwrap();let imported=import_document("idml",&bytes,"Colour",&mut |_|true).unwrap();let actual=draft(&imported.value).pages[0].frames.iter().find(|frame|frame.id()=="rgb").unwrap();assert!(matches!(actual,Frame::Rect{fill:actual_fill,stroke:actual_stroke,..}if actual_fill==&fill&&actual_stroke==&stroke));}

}

#[test]
fn idml_typography_and_utf8_style_ranges_survive_independent_markup_rewriting(){
    use semio_s_artifact_stdio_zip::{standards::v2_0::subsets::base::io::{decode_zip,encode_zip}};
    use semio_s_artifact_stdio_xml_test_oracle::standards::v1_0::subsets::base::oracle_round_trip;
    let cases=fixtures();let case=cases["cases"].as_array().unwrap().iter().find(|case|case["id"]=="idml-style-runs").unwrap();let mut doc=seed(&cases["cases"][2]);
    doc.paragraph_styles=semio_framework_pack_json::from_json_str(&case["paragraphStyles"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    doc.character_styles=semio_framework_pack_json::from_json_str(&case["characterStyles"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    doc.pages[0].margins=semio_framework_pack_json::from_json_str(&case["page"]["margins"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();doc.pages[0].columns=semio_framework_pack_json::from_json_str(&case["page"]["columns"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();if let Frame::Text{columns,inset,wrap_mode,..}=&mut doc.pages[0].frames[0]{*columns=3;*inset=LayoutRect{x:2.0,y:3.0,width:4.0,height:5.0};*wrap_mode="none".into();}
    doc.stories[0].content=case["story"].as_str().unwrap().into();doc.stories[0].style_runs=semio_framework_pack_json::from_json_str(&case["styleRuns"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let one_shot=export_document("idml",&doc,&mut |_|true).unwrap().value;let mut session=InterchangeExportSession::new("idml").unwrap();let mut streamed=Vec::new();loop{match session.advance(&doc).unwrap(){InterchangeExportAdvance::Chunk(bytes)=>streamed.extend(bytes),InterchangeExportAdvance::Progress{..}=>{},InterchangeExportAdvance::Done=>break,}}
    for bytes in [one_shot,streamed]{let mut zip=decode_zip(&bytes).unwrap();for entry in &mut zip.entries{if entry.name.ends_with(".xml"){entry.data=oracle_round_trip(&entry.data).unwrap();}}let imported=import_document("idml",&encode_zip(&zip).unwrap(),"Styles",&mut |_|true).unwrap();assert_eq!(draft(&imported.value).stories[0],doc.stories[0]);assert_eq!(draft(&imported.value).paragraph_styles,doc.paragraph_styles);assert_eq!(draft(&imported.value).character_styles,doc.character_styles);assert_eq!(draft(&imported.value).pages[0].margins,doc.pages[0].margins);assert_eq!(draft(&imported.value).pages[0].columns,doc.pages[0].columns);let Frame::Text{columns,inset,..}=&draft(&imported.value).pages[0].frames[0]else{panic!("text")};assert_eq!(*columns,3);assert_eq!(*inset,LayoutRect{x:2.0,y:3.0,width:4.0,height:5.0});}
}

#[test]
fn idml_embedded_png_and_editable_parent_pages_round_trip(){
    use semio_s_artifact_stdio_png::{PngSnapshot,standards::v1_2::subsets::any::io::encode_png};
    use semio_s_artifact_layout_layout_model::{ImageLink,ParentPage};
    let cases=fixtures();let case=cases["cases"].as_array().unwrap().iter().find(|case|case["id"]=="idml-image-and-parent").unwrap();let mut doc=seed(&cases["cases"][2]);let source=&case["image"];let mut png=PngSnapshot::default();png.image.width=2;png.image.height=1;png.image.samples=source["pixels"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u16).collect();let bytes=encode_png(&png).unwrap();let payload=store::pack_rt::pack_value_to_base64(&bytes);
    doc.links.push(ImageLink{id:"placed-image".into(),path:"image.png".into(),hash:String::new(),width:2,height:1,dpi:72,color_profile:None,state:Some("ok".into()),proxy_data_url:Some(format!("data:image/png;base64,{payload}")),artifact_kind:"s.stdio.png".into(),artifact_ref:String::new()});let layer=doc.pages[0].layer_ids[0].clone();doc.pages[0].frames.push(Frame::Image{id:"image-frame".into(),layer_id:layer,bounds:LayoutBounds{x:20.0,y:30.0,width:40.0,height:20.0,rotation:0.1},link_id:"placed-image".into(),locked:None,visible:None});
    let master_layer=Layer{id:"master-layer".into(),name:"Master".into(),visible:true,locked:false,object_ids:vec!["master-rect".into()]};doc.parent_pages.push(ParentPage{id:"master-body".into(),name:"Header".into(),width:200.0,height:300.0,layer_ids:vec![master_layer.id.clone()],layers:vec![master_layer],frames:vec![Frame::Rect{id:"master-rect".into(),layer_id:"master-layer".into(),bounds:LayoutBounds{x:5.0,y:5.0,width:50.0,height:10.0,rotation:0.0},fill:Some([0.2,0.3,0.4,0.5]),stroke:None,locked:None,visible:None}]});doc.pages[0].parent_page_id=Some("master-body".into());
    let one=export_document("idml",&doc,&mut |_|true).unwrap().value;let mut stream=InterchangeExportSession::new("idml").unwrap();let mut two=Vec::new();loop{match stream.advance(&doc).unwrap(){InterchangeExportAdvance::Chunk(bytes)=>two.extend(bytes),InterchangeExportAdvance::Progress{..}=>{},InterchangeExportAdvance::Done=>break,}}
    for output in [one,two]{let imported=import_document("idml",&output,"Assets",&mut |_|true).unwrap();assert_eq!(draft(&imported.value).parent_pages.len(),1);assert_eq!(draft(&imported.value).parent_pages[0].id,"master-body");assert_eq!(draft(&imported.value).parent_pages[0].frames.len(),1);assert_eq!(draft(&imported.value).pages[0].parent_page_id.as_deref(),Some("master-body"));let image=draft(&imported.value).links.iter().find(|link|link.id=="placed-image").unwrap();assert_eq!(image.width,2);assert_eq!(image.height,1);assert_eq!(image.proxy_data_url.as_deref(),Some(format!("data:image/png;base64,{payload}").as_str()));}
}

#[test]
fn idml_facing_pages_preserve_spread_membership_and_local_coordinates(){let cases=fixtures();let case=cases["cases"].as_array().unwrap().iter().find(|case|case["id"]=="idml-facing-spread").unwrap();let mut doc=document("Facing");for(index,source)in case["pages"].as_array().unwrap().iter().enumerate(){let id=source["id"].as_str().unwrap().to_string();let mut page=page(id,source["width"].as_f64().unwrap(),source["height"].as_f64().unwrap());page.spread_id="facing-spread".into();if index==0{page.guides.push(LayoutRect{x:5.0,y:8.0,width:20.0,height:30.0});}let frame=Frame::Rect{id:format!("frame-{index}"),layer_id:page.layer_ids[0].clone(),bounds:LayoutBounds{x:if index==0{10.0}else{30.0},y:20.0,width:40.0,height:30.0,rotation:0.1},fill:Some([0.1,0.2,0.3,0.4]),stroke:None,locked:None,visible:None};page.frames.push(frame);doc.pages.push(page);}doc.spreads=vec![Spread{id:"facing-spread".into(),name:"Facing Pages".into(),page_ids:vec!["left".into(),"right".into()]}];let one=export_document("idml",&doc,&mut |_|true).unwrap().value;let mut stream=InterchangeExportSession::new("idml").unwrap();let mut two=Vec::new();loop{match stream.advance(&doc).unwrap(){InterchangeExportAdvance::Chunk(bytes)=>two.extend(bytes),InterchangeExportAdvance::Progress{..}=>{},InterchangeExportAdvance::Done=>break,}}for bytes in[one,two]{let imported=import_document("idml",&bytes,"Facing",&mut |_|true).unwrap();assert_eq!(draft(&imported.value).spreads,doc.spreads);assert_eq!(draft(&imported.value).pages.len(),2);for(actual,expected)in draft(&imported.value).pages.iter().zip(&doc.pages){assert_eq!(actual.id,expected.id);assert_eq!(actual.spread_id,expected.spread_id);assert_eq!(actual.guides,expected.guides);let actual=actual.frames[0].bounds();let expected=expected.frames[0].bounds();assert!((actual.x-expected.x).abs()<1e-8);assert!((actual.rotation-expected.rotation).abs()<1e-8);}}}

#[test]
fn idml_layer_stack_order_agrees_between_direct_and_resumable_exports(){
    use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::{decode_zip,encode_zip};
    use semio_s_artifact_stdio_xml_test_oracle::standards::v1_0::subsets::base::oracle_round_trip;
    let cases=fixtures();let case=cases["cases"].as_array().unwrap().iter().find(|case|case["id"]=="idml-layer-order").unwrap();let mut doc=document("Layer stack");let mut page=page("ordered-page".into(),200.0,300.0);page.layers=case["layers"].as_array().unwrap().iter().map(|value|Layer{id:value["id"].as_str().unwrap().into(),name:value["name"].as_str().unwrap().into(),visible:true,locked:false,object_ids:Vec::new()}).collect();page.layer_ids=page.layers.iter().map(|layer|layer.id.clone()).collect();doc.spreads=vec![Spread{id:page.spread_id.clone(),name:"Ordered".into(),page_ids:vec![page.id.clone()]}];doc.pages.push(page);
    let direct=export_document("idml",&doc,&mut |_|true).unwrap().value;let mut session=InterchangeExportSession::new("idml").unwrap();let mut streamed=Vec::new();loop{match session.advance(&doc).unwrap(){InterchangeExportAdvance::Chunk(bytes)=>streamed.extend(bytes),InterchangeExportAdvance::Progress{..}=>{},InterchangeExportAdvance::Done=>break,}}
    for bytes in[direct,streamed]{let mut archive=decode_zip(&bytes).unwrap();for entry in &mut archive.entries{if entry.name.ends_with(".xml"){entry.data=oracle_round_trip(&entry.data).unwrap();}}let actual=import_document("idml",&encode_zip(&archive).unwrap(),"Order",&mut |_|true).unwrap();assert_eq!(draft(&actual.value).pages[0].layers.iter().map(|layer|layer.name.as_str()).collect::<Vec<_>>(),vec!["Back","Front"]);}
}

#[test]
fn idml_page_overrides_keep_the_master_and_preserve_sparse_changes_in_both_export_routes(){
    use semio_s_artifact_layout_layout_model::{ParentPage,PageOverride};
    use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::{decode_zip,encode_zip};
    use semio_s_artifact_stdio_xml_test_oracle::standards::v1_0::subsets::base::oracle_round_trip;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../📦️idml/📎️overrides/🧪️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap().iter().filter(|case|case.get("override").is_some()&&case.get("error").is_none()){let mut doc=seed(&fixtures()["cases"][2]);let object=case["override"]["objectId"].as_str().unwrap();let layer=Layer{id:"master-content".into(),name:"Parent content".into(),visible:true,locked:false,object_ids:vec![object.into()]};let bounds=LayoutBounds{x:5.0,y:8.0,width:50.0,height:30.0,rotation:0.0};
        let frame=match case["kind"].as_str().unwrap(){"text"=>Frame::Text{id:object.into(),layer_id:layer.id.clone(),bounds,story_id:doc.stories[0].id.clone(),thread_next:None,columns:1,inset:LayoutRect{x:0.0,y:0.0,width:0.0,height:0.0},wrap_mode:"none".into(),visible:None,locked:None},"image"=>{use semio_s_artifact_stdio_png::{PngSnapshot,standards::v1_2::subsets::any::io::encode_png};let mut png=PngSnapshot::default();png.image.width=1;png.image.height=1;png.image.samples=vec![255,0,0,128];let bytes=encode_png(&png).unwrap();doc.links.push(semio_s_artifact_layout_layout_model::ImageLink{id:"override-image".into(),path:"placed.png".into(),hash:String::new(),width:1,height:1,dpi:72,color_profile:None,state:Some("ok".into()),proxy_data_url:Some(format!("data:image/png;base64,{}",store::pack_rt::pack_value_to_base64(&bytes))),artifact_kind:"s.stdio.png".into(),artifact_ref:String::new()});Frame::Image{id:object.into(),layer_id:layer.id.clone(),bounds,link_id:"override-image".into(),visible:None,locked:None}},_=>Frame::Rect{id:object.into(),layer_id:layer.id.clone(),bounds,fill:Some([0.2,0.3,0.4,0.5]),stroke:None,visible:None,locked:None}};
        doc.parent_pages.push(ParentPage{id:"master".into(),name:"Parent".into(),width:200.0,height:300.0,layer_ids:vec![layer.id.clone()],layers:vec![layer],frames:vec![frame]});doc.pages[0].parent_page_id=Some("master".into());let change:PageOverride=semio_framework_pack_json::from_json_str(&case["override"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();doc.pages[0].overrides=vec![change.clone()];let mut second=page("unaffected".into(),200.0,300.0);second.parent_page_id=Some("master".into());second.spread_id=doc.spreads[0].id.clone();doc.spreads[0].page_ids.push(second.id.clone());doc.pages.push(second);
        let direct=export_document("idml",&doc,&mut |_|true).unwrap().value;let mut session=InterchangeExportSession::new("idml").unwrap();let mut streamed=Vec::new();loop{match session.advance(&doc).unwrap(){InterchangeExportAdvance::Chunk(bytes)=>{assert!(bytes.len()<=4096);streamed.extend(bytes)},InterchangeExportAdvance::Progress{..}=>{},InterchangeExportAdvance::Done=>break,}}
        for bytes in[direct,streamed]{let mut archive=decode_zip(&bytes).unwrap();let mut ids=std::collections::HashSet::new();fn visit(node:&semio_s_artifact_stdio_xml::schema::snapshot::XmlNode,ids:&mut std::collections::HashSet<String>){if let semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element{attrs,children,..}=node{if let Some(id)=attrs.iter().find(|attr|attr.name=="Self"){assert!(ids.insert(id.value.clone()),"duplicate IDML Self {}",id.value);}for child in children{visit(child,ids);}}}for entry in &archive.entries{if entry.name.ends_with(".xml"){let xml=semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text(std::str::from_utf8(&entry.data).unwrap()).unwrap();visit(xml.root.as_ref().unwrap(),&mut ids);}}let spread=archive.entries.iter().find(|entry|entry.name=="Spreads/Spread_0.xml").unwrap();let text=std::str::from_utf8(&spread.data).unwrap();assert!(text.contains(&format!("OverrideList=\"{object}\"")));for entry in &mut archive.entries{if entry.name.ends_with(".xml"){entry.data=oracle_round_trip(&entry.data).unwrap();}}let imported=import_document("idml",&encode_zip(&archive).unwrap(),"Override",&mut |_|true).unwrap();assert_eq!(draft(&imported.value).parent_pages.len(),1);assert_eq!(draft(&imported.value).parent_pages[0].frames[0].bounds(),doc.parent_pages[0].frames[0].bounds());let actual=&draft(&imported.value).pages[0].overrides[0];assert_eq!(actual.object_id,change.object_id);assert_eq!(actual.visible,change.visible);assert_eq!(actual.locked,change.locked);match(&actual.bounds,&change.bounds){(Some(a),Some(b))=>{assert!((a.x-b.x).abs()<1e-8);assert!((a.y-b.y).abs()<1e-8);assert!((a.rotation-b.rotation).abs()<1e-8);},(None,None)=>{},_=>panic!("sparse bounds presence")};assert!(draft(&imported.value).pages[1].overrides.is_empty());assert_eq!(draft(&imported.value).pages[0].frames.len(),doc.pages[0].frames.len());if case["kind"]=="image"{assert_eq!(draft(&imported.value).links[0].proxy_data_url,doc.links[0].proxy_data_url);}}
        while !session.terminal_is_empty(){assert!(session.close_one(session.next_release_bytes()));}
    }
}

#[test]
fn idml_external_override_lists_and_unrepresentable_copy_edits_are_handled_explicitly(){
    use semio_s_artifact_layout_layout_model::{ParentPage,PageOverride};
    use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::{decode_zip,encode_zip};
    use semio_s_artifact_stdio_xml_test_oracle::standards::v1_0::subsets::base::oracle_round_trip;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../📦️idml/📎️overrides/🧪️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap().iter().filter(|case|case.get("inputXml").is_some()||case.get("rewrite").is_some()){let mut doc=seed(&fixtures()["cases"][2]);doc.parent_pages.push(ParentPage{id:"master".into(),name:"Parent".into(),width:200.0,height:300.0,layer_ids:vec!["master-layer".into()],layers:vec![Layer{id:"master-layer".into(),name:"Master".into(),visible:true,locked:false,object_ids:vec!["master-rect".into()]}],frames:vec![Frame::Rect{id:"master-rect".into(),layer_id:"master-layer".into(),bounds:LayoutBounds{x:5.0,y:8.0,width:50.0,height:30.0,rotation:0.0},fill:Some([0.2,0.3,0.4,0.5]),stroke:None,locked:None,visible:None}]});doc.pages[0].parent_page_id=Some("master".into());if case.get("rewrite").is_some(){doc.pages[0].overrides.push(PageOverride{object_id:"master-rect".into(),bounds:None,visible:Some(false),locked:None});}
        let output=export_document("idml",&doc,&mut |_|true).unwrap();let mut archive=decode_zip(&output.value).unwrap();let spread=archive.entries.iter_mut().find(|entry|entry.name=="Spreads/Spread_0.xml").unwrap();let mut text=std::str::from_utf8(&spread.data).unwrap().to_string();if let Some(input)=case["inputXml"].as_str(){if let Some(list)=input.split("OverrideList=\"").nth(1).and_then(|value|value.split('"').next()){text=text.replacen("OverrideList=\"\"",&format!("OverrideList=\"{list}\""),1);}if let Some(matrix)=input.split("MasterPageTransform=\"").nth(1).and_then(|value|value.split('"').next()){text=text.replacen("MasterPageTransform=\"1 0 0 1 0 0\"",&format!("MasterPageTransform=\"{matrix}\""),1);}if case["spreadShowMasterItems"]==false{text=text.replacen("<Spread ","<Spread ShowMasterItems=\"false\" ",1);}}else{text=text.replacen("FillColor=\"Color/master-rect-fill\"","FillColor=\"Color/Black\"",1);}spread.data=oracle_round_trip(text.as_bytes()).unwrap();let imported=import_document("idml",&encode_zip(&archive).unwrap(),"External override",&mut |_|true);if let Some(error)=case["error"].as_str(){assert_eq!(imported.unwrap_err(),error);}else{let imported=imported.unwrap();let expected:Vec<PageOverride>=semio_framework_pack_json::from_json_str(&case["expected"]["overrides"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(draft(&imported.value).pages[0].overrides,expected);assert_eq!(draft(&imported.value).pages[0].frames.len(),1);}
    }
}

#[test]
fn non_ascii_pdf_text_keeps_utf8_native_style_ranges_through_idml(){
    use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::{decode_zip,encode_zip};
    use semio_s_artifact_stdio_xml_test_oracle::standards::v1_0::subsets::base::oracle_round_trip;
    let all=fixtures();let case=all["cases"].as_array().unwrap().iter().find(|case|case["id"]=="pdf-idml-utf8-ranges").unwrap();let source=seed(case);let pdf=export_document("pdf",&source,&mut |_|true).unwrap();assert_eq!(oracle::pdf_text(&pdf.value).unwrap().trim(),case["expected"]["text"].as_str().unwrap());let native=import_document("pdf",&pdf.value,"Unicode",&mut |_|true).unwrap();let story=&draft(&native.value).stories[0];assert_eq!(story.content,case["expected"]["text"].as_str().unwrap());assert_eq!(story.style_runs[0].end,case["expected"]["utf8Bytes"].as_u64().unwrap());let output=export_document("idml",draft(&native.value),&mut |_|true).unwrap();let mut zip=decode_zip(&output.value).unwrap();for entry in &mut zip.entries{if entry.name.ends_with(".xml"){entry.data=oracle_round_trip(&entry.data).unwrap();}}let restored=import_document("idml",&encode_zip(&zip).unwrap(),"Unicode",&mut |_|true).unwrap();assert_eq!(draft(&restored.value).stories[0].content,story.content);assert_eq!(draft(&restored.value).stories[0].style_runs[0].end,story.style_runs[0].end);
}

#[test]
fn step_export_contains_rotated_frame_geometry_parent_overrides_unicode_and_asset_placement(){
    use semio_s_artifact_layout_layout_model::{ImageLink,ParentPage,PageOverride};
    use semio_s_artifact_stdio_contract::part21::{parse_part21,Part21Value};
    use semio_s_artifact_stdio_step_test_oracle::standards::v_ap214::subsets::base::{oracle_round_trip,project_step_ap214_any};
    let all=fixtures();let case=all["cases"].as_array().unwrap().iter().find(|case|case["id"]=="step-layout-content").unwrap();let mut doc=seed(case);let b=|value:&serde_json::Value|LayoutBounds{x:value["x"].as_f64().unwrap(),y:value["y"].as_f64().unwrap(),width:value["width"].as_f64().unwrap(),height:value["height"].as_f64().unwrap(),rotation:value["rotation"].as_f64().unwrap()};let layer=doc.pages[0].layer_ids[0].clone();doc.pages[0].frames.push(Frame::Rect{id:"frame-rect".into(),layer_id:layer.clone(),bounds:b(&case["rectFrame"]["bounds"]),fill:Some([0.25,0.5,0.75,1.0]),stroke:Some([0.8,0.1,0.2,1.0]),visible:None,locked:None});doc.links.push(ImageLink{id:"placed-image".into(),path:"photo.png".into(),hash:String::new(),width:1,height:1,dpi:72,color_profile:None,state:None,proxy_data_url:None,artifact_kind:"s.stdio.png".into(),artifact_ref:String::new()});doc.pages[0].frames.push(Frame::Image{id:"frame-image".into(),layer_id:layer.clone(),bounds:b(&case["image"]["bounds"]),link_id:"placed-image".into(),visible:None,locked:None});doc.pages[0].layers.push(Layer{id:"hidden".into(),name:"Hidden".into(),visible:false,locked:false,object_ids:vec!["hidden-frame".into()]});doc.pages[0].frames.push(Frame::Rect{id:"hidden-frame".into(),layer_id:"hidden".into(),bounds:b(&case["rectFrame"]["bounds"]),fill:None,stroke:Some([0.0,0.0,0.0,1.0]),visible:None,locked:None});doc.parent_pages.push(ParentPage{id:"parent".into(),name:"Parent".into(),width:200.0,height:300.0,layer_ids:vec!["parent-layer".into()],layers:vec![Layer{id:"parent-layer".into(),name:"Parent".into(),visible:true,locked:false,object_ids:vec!["parent-frame".into()]}],frames:vec![Frame::Rect{id:"parent-frame".into(),layer_id:"parent-layer".into(),bounds:LayoutBounds{x:1.0,y:1.0,width:10.0,height:10.0,rotation:0.0},fill:Some([0.1,0.2,0.3,1.0]),stroke:None,visible:None,locked:None}]});doc.pages[0].parent_page_id=Some("parent".into());doc.pages[0].overrides=vec![PageOverride{object_id:"parent-frame".into(),bounds:Some(b(&case["parent"]["overrideBounds"])),visible:None,locked:None}];
    let output=export_document("step",&doc,&mut |_|true).unwrap();let mut session=InterchangeExportSession::new("step").unwrap().with_selected_page(0);let mut streamed=Vec::new();let mut turns=0;loop{turns+=1;assert!(turns<20000);match session.advance(&doc).unwrap(){InterchangeExportAdvance::Chunk(chunk)=>{assert!(chunk.len()<=4096);streamed.extend(chunk);},InterchangeExportAdvance::Progress{..}=>{},InterchangeExportAdvance::Done=>break,}}assert!(turns>30);assert_eq!(streamed,output.value);while !session.terminal_is_empty(){assert!(session.close_one(session.next_release_bytes()));}let rewritten=oracle_round_trip(&output.value).unwrap();assert_eq!(project_step_ap214_any(&output.value).unwrap(),project_step_ap214_any(&rewritten).unwrap());let graph=parse_part21(std::str::from_utf8(&rewritten).unwrap()).unwrap();assert!(graph.by_type("POLYLINE").count()>=3);assert!(graph.by_type("TEXT_LITERAL_WITH_EXTENT").count()>=3);let named=|kind:&str,name:&str|graph.instances.iter().find(|instance|instance.entity(kind).and_then(|args|args.first()).and_then(Part21Value::as_str)==Some(name));assert!(named("POLYLINE","hidden-frame").is_none());let args=named("POLYLINE","frame-rect").unwrap().entity("POLYLINE").unwrap();for(value,expected)in args[1].as_list().unwrap().iter().zip(case["expected"]["rectPointsMm"].as_array().unwrap()){let point=graph.instance(value.as_ref_id().unwrap()).unwrap().entity("CARTESIAN_POINT").unwrap()[1].as_list().unwrap();for(actual,expected)in point.iter().zip(expected.as_array().unwrap()){assert!((actual.as_real().unwrap()-expected.as_f64().unwrap()).abs()<1e-9);}}let inherited=named("POLYLINE","parent-frame").unwrap().entity("POLYLINE").unwrap()[1].as_list().unwrap();for(value,expected)in inherited.iter().zip(case["expected"]["parentPointsMm"].as_array().unwrap()){let point=graph.instance(value.as_ref_id().unwrap()).unwrap().entity("CARTESIAN_POINT").unwrap()[1].as_list().unwrap();for(actual,expected)in point.iter().zip(expected.as_array().unwrap()){assert!((actual.as_real().unwrap()-expected.as_f64().unwrap()).abs()<1e-9);}}assert_eq!(doc.parent_pages[0].frames[0].bounds().x,1.0);let texts=graph.by_type("TEXT_LITERAL_WITH_EXTENT").map(|instance|instance.entity("TEXT_LITERAL_WITH_EXTENT").unwrap()[1].as_str().unwrap()).collect::<String>();assert!(texts.contains("Grüße\nSTEP"));assert!(texts.contains("photo.png"));let imported=import_document("step",&rewritten,"Roundtrip",&mut |_|true).unwrap();let restored=draft(&imported.value);assert!((restored.pages[0].width-200.0).abs()<1e-9);assert!((restored.pages[0].height-300.0).abs()<1e-9);assert!(restored.background_drawing.is_none());assert_eq!(restored.pages[0].frames.iter().filter(|frame|matches!(frame,Frame::Rect{..})).count(),3);let rectangle=restored.pages[0].frames.iter().find(|frame|frame.id()=="frame-rect").unwrap();assert!((rectangle.bounds().rotation-case["rectFrame"]["bounds"]["rotation"].as_f64().unwrap()).abs()<1e-9);if let Frame::Rect{fill,stroke,..}=rectangle{assert_eq!(*fill,Some([0.25,0.5,0.75,1.0]));assert_eq!(*stroke,Some([0.8,0.1,0.2,1.0]));}else{panic!("editable rectangle")};let restored_text=restored.stories.iter().map(|story|story.content.as_str()).collect::<String>();assert_eq!(restored_text,case["expected"]["import"]["textFragmentsJoin"].as_str().unwrap());for story in &restored.stories{assert_eq!(story.style_runs[0].end,story.content.len()as u64);}assert!(restored.pages[0].frames.iter().filter(|frame|matches!(frame,Frame::Text{..})).count()>=3);
}

#[test]
fn pdf_authored_marked_text_restores_positioned_styles_through_canonical_pdf(){
 use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::{io,schema::snapshot::{PdfOp,PdfObject,PdfPropertyList,PdfDictEntry,PdfTextString,PdfExtGState}};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../📕️pdf/🏷️text/🧪️fixtures/🔣️.json")).unwrap();let case=&fixture["styled"];let all=fixtures();let input=all["cases"].as_array().unwrap().iter().find(|case|case["id"]=="pdf-idml-utf8-ranges").unwrap();let source=export_document("pdf",&seed(input),&mut |_|true).unwrap();let mut pdf=io::import::decode_pdf(&source.value).unwrap();let mut family=vec![0xfe,0xff];for unit in case["fontFamily"].as_str().unwrap().encode_utf16(){family.extend_from_slice(&unit.to_be_bytes());}let font=pdf.fonts.first().unwrap().id.clone();pdf.ext_g_states.push(PdfExtGState{id:"semantic-alpha".into(),fill_alpha:Some(case["fill"][3].as_f64().unwrap()),..Default::default()});pdf.pages[0].content=vec![PdfOp::BeginMarkedContentWithProperties{tag:fixture["tag"].as_str().unwrap().into(),properties:PdfPropertyList::Inline{entries:vec![PdfDictEntry::new("SemioFontFamily",PdfObject::Str(family)),PdfDictEntry::new("SemioFontWeight",PdfObject::Int(case["fontWeight"].as_i64().unwrap())),PdfDictEntry::new("SemioItalic",PdfObject::Bool(true))]}},PdfOp::SetExtGState{name:"semantic-alpha".into()},PdfOp::SetFillRgb{r:0.2,g:0.3,b:0.4},PdfOp::BeginText,PdfOp::SetFont{name:font,size:case["fontSize"].as_f64().unwrap()},PdfOp::SetTextRenderingMode{mode:3},PdfOp::SetTextMatrix{matrix:[1.0,0.0,0.0,1.0,20.0,70.0]},PdfOp::ShowText{text:PdfTextString::text(case["text"].as_str().unwrap())},PdfOp::EndText,PdfOp::EndMarkedContent];let bytes=io::export::encode_pdf(&pdf).unwrap();assert_eq!(oracle::pdf_text(&bytes).unwrap().trim(),case["text"].as_str().unwrap());let imported=import_document("pdf",&bytes,"Semantic text",&mut |_|true).unwrap();assert_eq!(draft(&imported.value).stories[0].content,case["text"].as_str().unwrap());let frame=draft(&imported.value).pages[0].frames.iter().find(|frame|matches!(frame,Frame::Text{..})).unwrap();assert!((frame.bounds().x-case["expected"]["x"].as_f64().unwrap()).abs()<1e-8);assert!((frame.bounds().y-case["expected"]["y"].as_f64().unwrap()).abs()<1e-8);assert_eq!(draft(&imported.value).paragraph_styles[0].font_family,case["fontFamily"].as_str().unwrap());assert_eq!(draft(&imported.value).paragraph_styles[0].font_weight,700);assert_eq!(draft(&imported.value).paragraph_styles[0].font_size,18.0);assert_eq!(draft(&imported.value).character_styles[0].italic,Some(true));assert_eq!(draft(&imported.value).character_styles[0].color,Some([0.2,0.3,0.4,0.5]));
}

#[test]
fn step_background_uses_borrowed_scene_events_and_real_geometry(){
 semio_s_artifact_layout_drawing_composition::bind().unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../📐️step/🖍️background/🧪️fixtures/🔣️.json")).unwrap();let scene:semio_framework_io_ports::VectorScene=semio_framework_pack_json::from_json_str(&fixture["scene"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let content=crate::drawing_codec().unwrap().lift(&scene).unwrap();let mut doc=document("Background");doc.pages.push(page("page-1".into(),200.0,300.0));finish_document(&mut doc);doc.background_drawing=Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("portable-background",&content));let direct=export_document("step",&doc,&mut |_|true).unwrap();let mut session=InterchangeExportSession::new("step").unwrap();let mut bytes=Vec::new();let mut turns=0;loop{turns+=1;assert!(turns<20000);match session.advance(&doc).unwrap(){InterchangeExportAdvance::Chunk(chunk)=>{assert!(chunk.len()<=4096);bytes.extend(chunk)},InterchangeExportAdvance::Progress{..}=>{},InterchangeExportAdvance::Done=>break,}}assert_eq!(bytes,direct.value);while !session.terminal_is_empty(){assert!(session.close_one(session.next_release_bytes()));}let rewritten=semio_s_artifact_stdio_step_test_oracle::standards::v_ap214::subsets::base::oracle_round_trip(&bytes).unwrap();let graph=semio_s_artifact_stdio_contract::part21::parse_part21(std::str::from_utf8(&rewritten).unwrap()).unwrap();assert!(graph.by_type("POLYLINE").count()>=2);let projected=semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::projection::project_edges(&graph).unwrap();let expected=fixture["expected"]["rectanglePointsMm"].as_array().unwrap();{let actual=&projected.curves[0].points;for(point,expected)in actual.iter().zip(expected){for(axis,value)in point.iter().enumerate(){assert!((value*25.4/72.0-expected[axis].as_f64().unwrap()).abs()<1e-9);}}}let drawing=semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::draughting::read_draughting(&graph).unwrap();assert!(drawing.texts.iter().any(|text|text.literal.contains(fixture["expected"]["text"].as_str().unwrap())));assert!(drawing.texts.iter().any(|text|text.literal.contains("image/png")));assert!(drawing.paints.iter().any(|paint|paint.fill==Some([0.25,0.5,0.75])));
}

#[test]
fn step_large_borrowed_background_resumes_and_cancels_without_whole_scene_lowering(){semio_s_artifact_layout_drawing_composition::bind().unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../📐️step/🖍️background/🧪️fixtures/🔣️.json")).unwrap();let life=&fixture["lifecycle"];let mut scene:semio_framework_io_ports::VectorScene=semio_framework_pack_json::from_json_str(&fixture["scene"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let style="a".repeat(life["longStyleBytes"].as_u64().unwrap()as usize);scene.styles[0].name=style.clone();scene.styles[0].fill=None;let mut segments=vec![semio_framework_io_ports::VectorSegment::MoveTo{to:semio_framework_io_ports::VectorPoint2{x:0.0,y:10.0}}];for index in 1..=life["strokeSegments"].as_u64().unwrap(){segments.push(semio_framework_io_ports::VectorSegment::LineTo{to:semio_framework_io_ports::VectorPoint2{x:index as f64*0.01,y:10.0}});}scene.layers[0].root=semio_framework_io_ports::VectorNode::Group{transform:semio_framework_io_ports::VectorTransform::identity(),children:vec![semio_framework_io_ports::VectorNode::Path{segments,style:Some(style.clone())},semio_framework_io_ports::VectorNode::Text{value:"Grüße".repeat(life["textRepeat"].as_u64().unwrap()as usize),at:semio_framework_io_ports::VectorPoint2{x:5.0,y:20.0},style:Some(style)}]};let content=crate::drawing_codec().unwrap().lift(&scene).unwrap();let mut doc=document("Long background");doc.pages.push(page("page-1".into(),200.0,300.0));finish_document(&mut doc);doc.background_drawing=Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("long-background",&content));for cut in life["cancellationTurns"].as_array().unwrap(){let mut session=InterchangeExportSession::new("step").unwrap();for _ in 0..cut.as_u64().unwrap(){assert!(!matches!(session.advance(&doc).unwrap(),InterchangeExportAdvance::Done));}let mut turns=0;while !session.terminal_is_empty(){turns+=1;assert!(turns<20000);let demand=session.next_release_bytes();assert!(session.close_one(demand));}assert!(session.terminal_is_empty());}let mut session=InterchangeExportSession::new("step").unwrap();let mut output=Vec::new();let mut turns=0;loop{turns+=1;assert!(turns<100000);match session.advance(&doc).unwrap(){InterchangeExportAdvance::Chunk(bytes)=>{assert!(bytes.len()<=4096);output.extend(bytes)},InterchangeExportAdvance::Progress{..}=>{},InterchangeExportAdvance::Done=>break}}assert!(turns>4097);while !session.terminal_is_empty(){assert!(session.close_one(session.next_release_bytes()));}let graph=semio_s_artifact_stdio_contract::part21::parse_part21(std::str::from_utf8(&output).unwrap()).unwrap();assert!(graph.by_type("POLYLINE").count()>=64);assert!(graph.by_type("TEXT_LITERAL_WITH_EXTENT").count()>16);}

#[test]
fn unsupported_filled_background_geometry_refuses_before_publication_and_remains_closeable(){semio_s_artifact_layout_drawing_composition::bind().unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../📐️step/🖍️background/🧪️fixtures/🔣️.json")).unwrap();for case in fixture["refusals"].as_array().unwrap(){let mut scene:semio_framework_io_ports::VectorScene=semio_framework_pack_json::from_json_str(&fixture["scene"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let mut segments=vec![semio_framework_io_ports::VectorSegment::MoveTo{to:semio_framework_io_ports::VectorPoint2{x:10.0,y:0.0}}];if case["kind"]=="compound"{segments.push(semio_framework_io_ports::VectorSegment::LineTo{to:semio_framework_io_ports::VectorPoint2{x:10.0,y:10.0}});segments.push(semio_framework_io_ports::VectorSegment::MoveTo{to:semio_framework_io_ports::VectorPoint2{x:20.0,y:20.0}});}else{let count=case["count"].as_u64().unwrap();for index in 1..=count{let angle=std::f64::consts::TAU*index as f64/count as f64;segments.push(semio_framework_io_ports::VectorSegment::LineTo{to:semio_framework_io_ports::VectorPoint2{x:angle.cos()*10.0,y:angle.sin()*10.0}});}}scene.layers[0].root=semio_framework_io_ports::VectorNode::Path{segments,style:Some("accent".into())};let content=crate::drawing_codec().unwrap().lift(&scene).unwrap();let mut doc=document("Refused background");doc.pages.push(page("page-1".into(),200.0,300.0));finish_document(&mut doc);doc.background_drawing=Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("refused-background",&content));let mut session=InterchangeExportSession::new("step").unwrap();let mut turns=0;let error=loop{turns+=1;assert!(turns<10000);match session.advance(&doc){Err(error)=>break error,Ok(InterchangeExportAdvance::Done)=>panic!("unsupported fill published"),_=>{}}};assert_eq!(error,case["error"].as_str().unwrap());while !session.terminal_is_empty(){assert!(session.close_one(session.next_release_bytes()));}}}

#[test]
fn pdf_marked_cluster_owns_all_outlines_only_after_editable_promotion(){
 use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::{io,schema::snapshot::{PdfOp,PdfTextString}};
 semio_s_artifact_layout_drawing_composition::bind().unwrap();let all=fixtures();let input=all["cases"].as_array().unwrap().iter().find(|case|case["id"]=="pdf-idml-utf8-ranges").unwrap();let source=export_document("pdf",&seed(input),&mut |_|true).unwrap();let mut pdf=io::import::decode_pdf(&source.value).unwrap();let font=pdf.fonts[0].id.clone();
 for unsupported in [false,true]{pdf.pages[0].content=vec![PdfOp::BeginMarkedContent{tag:PDF_LAYOUT_TEXT_TAG.into()},PdfOp::BeginText,PdfOp::SetFont{name:font.clone(),size:18.0},PdfOp::SetTextRenderingMode{mode:3},PdfOp::SetHorizontalScale{scale:60.0},PdfOp::SetTextMatrix{matrix:if unsupported{[1.0,0.0,0.5,1.0,20.0,70.0]}else{[1.0,0.0,0.0,1.0,20.0,70.0]}},PdfOp::ShowText{text:PdfTextString::text("Grüße")},PdfOp::EndText,PdfOp::Rectangle{x:20.0,y:70.0,width:5.0,height:8.0},PdfOp::Fill,PdfOp::MoveTo{x:30.0,y:70.0},PdfOp::CurveTo{x1:35.0,y1:75.0,x2:40.0,y2:65.0,x3:45.0,y3:70.0},PdfOp::ClosePath,PdfOp::Fill,PdfOp::EndMarkedContent];let bytes=io::export::encode_pdf(&pdf).unwrap();assert_eq!(oracle::pdf_text(&bytes).unwrap().trim(),"Grüße");let imported=import_document("pdf",&bytes,"Cluster",&mut |_|true).unwrap();if unsupported{assert!(draft(&imported.value).stories.is_empty());assert!(draft(&imported.value).pages[0].background_drawing.is_none());assert_eq!(pending(&imported.value).len(),1);}else{assert_eq!(draft(&imported.value).stories.len(),1);assert_eq!(draft(&imported.value).stories[0].content,"Grüße");assert!(draft(&imported.value).pages[0].background_drawing.is_none());assert_eq!(draft(&imported.value).pages[0].frames.len(),1);}}
 pdf.pages[0].content=vec![PdfOp::MoveTo{x:1.0,y:2.0},PdfOp::CurveToInitial{x2:3.0,y2:4.0,x3:5.0,y3:6.0},PdfOp::CurveToFinal{x1:7.0,y1:8.0,x3:9.0,y3:10.0},PdfOp::ClosePath,PdfOp::Fill];let bytes=io::export::encode_pdf(&pdf).unwrap();let imported=import_document("pdf",&bytes,"Cubic",&mut |_|true).unwrap();let background=&pending(&imported.value)[0].scene;let semio_framework_io_ports::VectorNode::Group{children,..}=&background.layers[0].root else{panic!("drawing group")};let semio_framework_io_ports::VectorNode::Path{segments,..}=&children[0] else{panic!("drawing path")};assert_eq!(segments.len(),4);assert!(matches!(segments[1],semio_framework_io_ports::VectorSegment::CubicTo{..}));
}

#[test]
fn pdf_paths_keep_separate_unpublished_page_sources_without_placeholder_references(){use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::{io,schema::snapshot::{PdfSnapshot,PdfPage,PdfOp}};let mut pdf=PdfSnapshot::default();for index in 0..2{let mut page=PdfPage::new(200.0,300.0);page.content=vec![PdfOp::MoveTo{x:10.0+index as f64*20.0,y:20.0},PdfOp::LineTo{x:20.0+index as f64*20.0,y:20.0},PdfOp::LineTo{x:20.0+index as f64*20.0,y:30.0},PdfOp::ClosePath,PdfOp::Fill];pdf.pages.push(page);}let bytes=io::export::encode_pdf(&pdf).unwrap();let imported=import_document("pdf",&bytes,"Pages",&mut |_|true).unwrap();assert_eq!(draft(&imported.value).pages.len(),2);assert!(draft(&imported.value).background_drawing.is_none());assert_eq!(pending(&imported.value).len(),2);for(index,page)in draft(&imported.value).pages.iter().enumerate(){assert!(page.background_drawing.is_none());let source=&pending(&imported.value)[index];assert_eq!(source.source,semio_s_artifact_layout_layout_model::LayoutDrawingBirthSource::Page{page_id:page.id.clone()});let scene=&source.scene;assert_eq!(scene.canvas.width,200.0);assert_eq!(scene.canvas.height,300.0);let semio_framework_io_ports::VectorNode::Group{children,..}=&scene.layers[0].root else{panic!("page drawing group")};let semio_framework_io_ports::VectorNode::Path{segments,..}=&children[0]else{panic!("page drawing path")};let semio_framework_io_ports::VectorSegment::MoveTo{to}=segments[0]else{panic!("path origin")};assert_eq!(to.x,10.0+index as f64*20.0);assert_eq!(to.y,280.0);}}
