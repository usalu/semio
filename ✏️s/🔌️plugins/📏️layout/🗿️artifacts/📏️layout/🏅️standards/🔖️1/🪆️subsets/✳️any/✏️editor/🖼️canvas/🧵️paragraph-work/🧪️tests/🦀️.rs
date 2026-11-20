//! 🧪️ Real outline publication keeps relative inset widths, layer order and original snapshot identity.
use super::*;
use semio_framework_value::{RetainedCloneGrant,retirement::controlled::ControlledRetirement};
const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:usize::MAX/4,maximum_copy_bytes:usize::MAX/4,maximum_capacity_bytes:usize::MAX/4,maximum_release_bytes:usize::MAX/4,maximum_depth:128};
const TURN:RetainedCloneGrant=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:65536,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:64};
fn caller()->semio_framework_job::OriginalJobFixtureCaller{semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(472),semio_framework_job::Generation(2),STOCK,semio_framework_job::default_now_us,u64::MAX,u64::MAX)}
fn drive(work:&mut LayoutRenderWork,doc:&LayoutSnapshot){
    let mut caller=caller();
    for _ in 0..2_000_000{
        if caller.turn(TURN,|cx|work.advance(doc,None,cx)).unwrap(){caller.retire(STOCK);return}
    }
    panic!("original live render did not terminate")
}
fn retire(work:LayoutRenderWork){
    let mut owner=ControlledRetirement::new(work).unwrap_or_else(|_|panic!("controlled render"));
    let mut payer=caller();
    for _ in 0..1_000_000{
        if owner.terminal_is_empty(){payer.retire(STOCK);return}
        payer.turn(TURN,|cx|{
            let demand=owner.selected_demand(4096)?;
            semio_framework_job::close_original_effect_step(cx,demand,|effect|owner.step(4096,effect))?;
            Ok(())
        }).unwrap();
    }
    panic!("render retirement did not terminate")
}
#[test]fn portable_transformed_segments_match_independent_matrix_fixture(){let fixture:Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for row in fixture["segments"].as_array().unwrap(){let point=|value:&Value|[value[0].as_f64().unwrap(),value[1].as_f64().unwrap()];let s=&row["source"];let segment=match s["kind"].as_str().unwrap(){"move"=>PathSegment::Move{to:point(&s["to"])},"line"=>PathSegment::Line{to:point(&s["to"])},"quad"=>PathSegment::Quad{ctrl:point(&s["ctrl"]),to:point(&s["to"])},"cubic"=>PathSegment::Cubic{ctrl1:point(&s["ctrl1"]),ctrl2:point(&s["ctrl2"]),to:point(&s["to"])},_=>PathSegment::Close};let center=point(&row["center"]);let actual:Value=serde_json::from_str(&segment_json(&segment,row["baseline"].as_f64().unwrap(),row["italic"].as_bool().unwrap(),row["rotation"].as_f64().unwrap()as f32,[center[0]as f32,center[1]as f32]).unwrap()).unwrap();let expected=&row["expected"];assert_eq!(actual["kind"],expected["kind"]);for key in ["to","ctrl","ctrl1","ctrl2"]{if let Some(point)=expected.get(key){for at in 0..2{assert!((actual[key][at].as_f64().unwrap()-point[at].as_f64().unwrap()).abs()<0.000001);}}}}}
#[test]fn retained_render_publishes_real_glyph_paths_and_preserves_top_overlay(){let mut doc=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();doc.stories[0].content="AB".into();doc.stories[0].style_runs.clear();let mut work=LayoutRenderWork::new();work.prepare(&doc,&LayoutWindowConfig::default(),&LayoutWindowTransient::default(),&LayoutInteractionSnapshot::default(),true).unwrap();drive(&mut work,&doc);let layers:Value=serde_json::from_str(&work.owners.output).unwrap();let layers=layers.as_array().unwrap();assert!(layers.iter().any(|layer|layer["id"]=="frame-text-1.glyph.0"&&layer["kind"]=="path"));assert!(layers.iter().all(|layer|layer["kind"]!="text"));assert_eq!(layers.last().unwrap()["id"],"meta:utility");retire(work);}
#[test]
fn preview_prepares_flow_only_when_a_story_is_visible() {
    let doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let fixture: Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../⚙️engine/🎬️scene/📄️paragraph/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let bounds = doc.pages[0].frames.iter().find(|frame| frame.id() == "frame-text-1").unwrap().bounds();
    assert_eq!(Some(bounds.x), fixture["preview"]["storyXRange"][0].as_f64());
    assert_eq!(Some(bounds.x + bounds.width), fixture["preview"]["storyXRange"][1].as_f64());
    for view in fixture["preview"]["views"].as_array().unwrap() {
        let mut config = LayoutWindowConfig::default();
        config.camera.zoom = view["zoom"].as_f64().unwrap();
        config.camera.x = view["cameraX"].as_f64().unwrap();
        let expected = view["storyVisible"].as_bool().unwrap();
        let mut work = LayoutRenderWork::new();
        work.prepare(&doc, &config, &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true).unwrap();
        drive(&mut work, &doc);
        assert_eq!(work.owners.text.flow_projection_ready(), expected);
        let layers: Value = serde_json::from_str(&work.owners.output).unwrap();
        assert_eq!(layers.as_array().unwrap().iter().any(|layer| layer["id"].as_str().is_some_and(|id| id.contains(".glyph."))), expected);
        retire(work);
    }
}
#[test]fn changed_original_snapshot_is_refused(){let doc=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();let changed=doc.clone();let mut work=LayoutRenderWork::new();work.prepare(&doc,&LayoutWindowConfig::default(),&LayoutWindowTransient::default(),&LayoutInteractionSnapshot::default(),false).unwrap();assert!(work.step(&changed,None).is_err());retire(work);}

#[test]
fn gesture_preview_translates_real_text_and_keeps_live_snapshot_identity() {
    let fixture: Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../🎭️modes/✏️edit/🪟️windows/📐️blueprint/🪛️utilities/🔄️transform/🧪️tests/🔬️unit/🔣️.json")).unwrap();
    let motion=fixture["previews"][0]["motion"].as_array().unwrap();
    let dx=motion[1].as_f64().unwrap();
    let dy=motion[2].as_f64().unwrap();
    let mut doc=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    doc.stories[0].content="AB".into();
    doc.stories[0].style_runs.clear();
    let original=doc.clone();
    let frame="frame-text-1";
    let mut config=LayoutWindowConfig::default();
    config.active_utility=transform::UTILITY_ID.into();
    let interaction=LayoutInteractionSnapshot{ids:vec![frame.into()],..Default::default()};
    let gesture=transform::layout_transform_dispatch(
        "translateSelection",transform::LayoutTransformPhase::Stream,
        Some(transform::LayoutFrameRecord{
            page_id:doc.pages[0].id.clone(),targets:vec![frame.into()],motion:transform::LayoutFrameMotion::Drag{dx,dy},
        }),&doc,None,"preview-seed","preview-revision",None,
    ).tool.unwrap();
    let collect=|transform_tool| {
        let transient=LayoutWindowTransient{transform_tool,..Default::default()};
        let mut work=LayoutRenderWork::new();
        work.prepare(&doc,&config,&transient,&interaction,true).unwrap();
        drive(&mut work,&doc);
        let result: Value=serde_json::from_str(&work.owners.output).unwrap();
        retire(work);
        result
    };
    let before=collect(None);
    let preview=collect(gesture);
    let point=|layers:&Value, id:&str, read:fn(&Value)->&Value| {
        let row=layers.as_array().unwrap().iter().find(|row|row["id"]==id).unwrap();
        let point=read(row);
        [point[0].as_f64().unwrap(),point[1].as_f64().unwrap()]
    };
    for (id,read) in [
        ("frame-text-1.glyph.0",(|row:&Value|&row["segments"][0]["to"]) as fn(&Value)->&Value),
        ("meta:gumball",|row:&Value|&row["gumball"]["pivotLayer"]),
    ] {
        let a=point(&before,id,read);
        let z=point(&preview,id,read);
        assert!((z[0]-a[0]-dx).abs()<1e-4);
        assert!((z[1]-a[1]-dy).abs()<1e-4);
    }
    assert_eq!(doc,original);
}

#[path="🖼️drawing/🦀️.rs"]mod drawing_tests;
