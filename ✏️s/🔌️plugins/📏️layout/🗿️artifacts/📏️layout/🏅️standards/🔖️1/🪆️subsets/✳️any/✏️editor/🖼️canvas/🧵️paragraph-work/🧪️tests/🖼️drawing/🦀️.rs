//! 🧪️ The actual live render owner publishes source-isolated Page Canvas paths, glyphs and original images through original turns.
use super::*;
use semio_framework_io_ports::{ChildAdmissionRead,erase_child_admission_source};
use semio_framework_value::{DslValue,FromValue};
use semio_s_artifact_layout_layout_model::LayoutDrawingMember;
use semio_framework_value_clone_source_testing::FixtureSource;
use semio_s_artifact_draw_drawing_model::{snapshot::DrawingSnapshot,validation::DRAWING_ADMITTER,vector_source::DRAWING_VECTOR_SOURCE};
fn member(child_id:&str,artifact_id:&str)->LayoutDrawingMember{
 LayoutDrawingMember::from_value(DslValue::from(json!({"handle":{"childId":child_id,"target":{"artifactId":artifact_id,"dialect":{"artifactKind":"s.draw.drawing","standard":"1","subset":"*"}}}}))).unwrap()
}
#[test]fn live_page_drawing_uses_shared_original_composition_and_retires_cancelled_frontiers(){
 let corpus:Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../⚙️engine/🎬️scene/🧵️drawing/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let mut doc=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
 for page in &mut doc.pages{page.frames.clear();page.layers.clear();page.parent_page_id=None;}
 doc.stories.clear();
 doc.background_drawing=Some(member("live-document","live-document-drawing"));
 doc.pages[1].background_drawing=Some(member("live-page","live-page-drawing"));
 let original=doc.clone();
 let content=corpus["cases"][0]["content"].clone();
 let document=FixtureSource::from_owner(serde_json::from_value::<DrawingSnapshot>(content.clone()).unwrap());
 let page=FixtureSource::from_owner(serde_json::from_value::<DrawingSnapshot>(content).unwrap());
 let reads=[
    ChildAdmissionRead{source:erase_child_admission_source(document.borrow(),0x4348494c),admitter:&DRAWING_ADMITTER,vector:&DRAWING_VECTOR_SOURCE},
    ChildAdmissionRead{source:erase_child_admission_source(page.borrow(),0x4348494c),admitter:&DRAWING_ADMITTER,vector:&DRAWING_VECTOR_SOURCE},
 ];let mut config=LayoutWindowConfig::default();config.active_page_id=doc.pages[1].id.clone();
 let mut work=LayoutRenderWork::new();work.prepare(&doc,&config,&LayoutWindowTransient::default(),&LayoutInteractionSnapshot::default(),false).unwrap();assert!(work.owners.list.as_ref().unwrap().strokes.is_empty());let mut payer=caller();let mut published=false;
 for _ in 0..2_000_000{if payer.turn(TURN,|cx|work.advance_from_source(&doc,None,cx,Some(&reads))).unwrap(){published=true;break}}
 assert!(published);payer.retire(STOCK);let layers:Value=serde_json::from_str(&work.owners.output).unwrap();let drawings:Vec<_>=layers.as_array().unwrap().iter().filter(|layer|layer["id"].as_str().is_some_and(|id|id.starts_with("layout.drawing."))).collect();assert_eq!(drawings.iter().filter(|layer|layer["kind"]=="image").count(),2);assert_eq!(drawings.iter().filter(|layer|layer["kind"]=="path").count(),10);assert!(drawings.iter().all(|layer|layer["transform"].as_array().is_some_and(|matrix|matrix.len()==6)));assert!(drawings.iter().filter(|layer|layer["kind"]=="image").all(|layer|layer["image"]["src"].as_str().unwrap().starts_with("data:image/png;base64,")));assert_eq!(doc,original);retire(work);
 for cut in[0,24,4096,16000]{let mut work=LayoutRenderWork::new();work.prepare(&doc,&config,&LayoutWindowTransient::default(),&LayoutInteractionSnapshot::default(),false).unwrap();let mut caller=caller();for _ in 0..cut{if caller.turn(TURN,|cx|work.advance_from_source(&doc,None,cx,Some(&reads))).unwrap(){break;}}let state=(work.phase,work.bytes,work.owners.chunks.len(),work.owners.output.len());caller.cancel().cancel_now();assert!(!caller.turn(TURN,|cx|work.advance_from_source(&doc,None,cx,Some(&reads))).unwrap());assert_eq!(state,(work.phase,work.bytes,work.owners.chunks.len(),work.owners.output.len()));assert_eq!(caller.receipt(),Default::default());retire(work);caller.retire(STOCK);}
}

#[test]
fn live_drawing_fits_an_original_png_and_path_to_the_page_and_frame() {
    let corpus: Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../⚙️engine/🎬️scene/🧵️drawing/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut content = corpus["cases"].as_array().unwrap().iter()
        .find(|row| row["name"] == "removed: rotated drawing frame turns the plan and keeps its png")
        .unwrap()["content"].clone();
    content["layers"][0]["segments"] = json!([
        {"kind":"move","to":[0,0]}, {"kind":"line","to":[1,0]},
        {"kind":"line","to":[1,1]}, {"kind":"line","to":[0,1]}, {"kind":"close"}
    ]);
    let image = &mut content["layers"][1];
    image["transform"]["x"] = json!(0);
    image["transform"]["y"] = json!(0);
    image["width"] = json!(1);
    image["height"] = json!(1);
    let original = FixtureSource::from_owner(serde_json::from_value::<DrawingSnapshot>(content).unwrap());
    let read = || ChildAdmissionRead {
        source: erase_child_admission_source(original.borrow(), 0x4348494c),
        admitter: &DRAWING_ADMITTER,
        vector: &DRAWING_VECTOR_SOURCE,
    };
    let reads = [read(), read()];
    let mut doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    doc.background_drawing = Some(member("dwg", "rot"));
    doc.links[0].artifact_kind = "s.draw.drawing".into();
    doc.links[0].artifact_ref = "rot".into();

    let mut work = LayoutRenderWork::new();
    work.prepare(&doc, &LayoutWindowConfig::default(), &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), false).unwrap();
    let mut payer = caller();
    let published = (0..2_000_000).any(|_| payer.turn(TURN, |cx| work.advance_from_source(&doc, None, cx, Some(&reads))).unwrap());
    payer.retire(STOCK);
    let layers = serde_json::from_str::<Value>(&work.owners.output);
    retire(work);
    assert!(published, "Canvas Drawing publication must terminate");
    let layers = layers.expect("retained Canvas layers");

    let mut drawing_layers = layers.as_array().unwrap().iter().filter(|layer| {
        layer["id"].as_str().is_some_and(|id| id.starts_with("layout.drawing."))
    });
    for expected in [[400.0, 0.0, 0.0, 400.0, 0.0, 50.0], [40.0, 0.0, 0.0, 40.0, 146.0, 435.0]] {
        let path = drawing_layers.next().expect("fitted Drawing path");
        assert_eq!(path["kind"], "path");
        assert_eq!(path["transform"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>(), expected);
        assert_eq!(path["segments"][0], json!({"kind":"move","to":[0,0]}));
        assert_eq!(path["segments"][4], json!({"kind":"close"}));

        let raster = drawing_layers.next().expect("fitted Drawing image");
        assert_eq!(raster["kind"], "image");
        assert_eq!(raster["transform"], path["transform"]);
        let url = raster["image"]["src"].as_str().unwrap();
        let payload = url.strip_prefix("data:image/png;base64,").expect("PNG data URL");
        let bytes = crate::canvas::decode_base64(payload).expect("Drawing PNG base64");
        let png = semio_framework_pixels::decode_png(&bytes).expect("Drawing PNG");
        assert_eq!((png.width, png.height), (1, 1));
        assert_eq!(png.pixels, [255, 0, 0, 255]);
    }
    assert!(drawing_layers.next().is_none());
}
