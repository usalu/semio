//! 🧪️ The actual live render owner publishes source-isolated Page Canvas paths, glyphs and original images through original turns.
use super::*;
use semio_framework_io_ports::{ChildAdmissionRead,erase_child_admission_source,testkit::bind};
use semio_framework_value_clone_source_testing::FixtureSource;
use semio_s_artifact_draw_drawing_model::{snapshot::DrawingSnapshot,validation::DRAWING_ADMITTER,vector::DRAWING_VECTOR_SOURCE};
#[test]fn live_page_drawing_uses_shared_original_composition_and_retires_cancelled_frontiers(){
 bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);bind(&semio_s_artifact_layout_layout_model::LAYOUT_PAGE_DRAWING);
 let corpus:Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../⚙️engine/🎬️scene/🧵️drawing/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let mut doc=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();for page in &mut doc.pages{page.frames.clear();page.layers.clear();page.parent_page_id=None;}doc.stories.clear();let content=semio_framework_value::DslValue::from(&corpus["cases"][0]["content"]);doc.background_drawing=Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("live-document",&content));doc.pages[1].background_drawing=Some(semio_s_artifact_layout_layout_model::page_drawing_child_handle(&doc.pages[1].id,&content).unwrap());let original=doc.clone();
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
