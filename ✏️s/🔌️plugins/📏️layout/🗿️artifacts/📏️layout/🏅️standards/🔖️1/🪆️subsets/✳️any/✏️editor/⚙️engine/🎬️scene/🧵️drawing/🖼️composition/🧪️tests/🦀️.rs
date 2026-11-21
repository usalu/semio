//! 🧪️ Genuine Draw loans fund UTF-8 copies with exact quotes and preserve paid consumer errors.
use crate::*;
use semio_framework_value::{OriginalRetirementFailure, RetainedCloneStep, RetainedWorkPoll, retirement::controlled::ControlledRetirement};
use semio_framework_value_clone_source_testing::{FixtureSource, drive::drive_reply};
use semio_s_artifact_draw_drawing_model::{snapshot::DrawingSnapshot,validation::DRAWING_ADMITTER,vector_source::DRAWING_VECTOR_SOURCE};
fn grant()->VectorSceneGrant{VectorSceneGrant{maximum_source_items:4096,maximum_depth:128,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24}}
fn retained(g:VectorSceneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:g.maximum_source_items,maximum_copy_bytes:g.maximum_copy_bytes,maximum_capacity_bytes:g.maximum_capacity_bytes,maximum_release_bytes:g.maximum_release_bytes,maximum_depth:g.maximum_depth}}
fn advance(state: &mut DrawingCompositionState, snapshot: &semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot, read: &ChildAdmissionRead<'_>, grant: VectorSceneGrant) -> Result<DrawingCompositionTurn, ValueError> {
    drive_reply(retained(grant), 1024, |answer| answer.as_ref().map_or_else(
        ValueError::retained_progress, |(_, receipt)| *receipt,
    ), |effect, reply, recover| {
        assert!(!recover);
        let turn = state.advance(snapshot, read, effect);
        let p = state.last_receipt();
        let receipt = RetainedCloneProgress {
            copied_items: p.source_items,
            copied_bytes: p.copied_bytes,
            retained_capacity_bytes: p.retained_capacity_bytes,
            released_bytes: p.released_bytes,
        };
        Ok(reply.complete(Ok((turn, receipt))))
    }).result.unwrap().0
}
fn close(state: DrawingCompositionState) {
    let mut owner = ControlledRetirement::new(state).unwrap_or_else(|_| panic!("original drawing composition retirement"));
    let body = 4096;
    for _ in 0..100_000 {
        if owner.terminal_is_empty() {
            return;
        }
        let demand = owner.selected_demand(body).unwrap();
        let grant = RetainedCloneGrant {
            maximum_items: grant().maximum_source_items,
            maximum_copy_bytes: demand.copy_bytes,
            maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes,
            maximum_depth: demand.depth,
        };
        drive_reply(grant, 1024, |answer| answer.as_ref().map_or_else(
            OriginalRetirementFailure::retained_progress, |step: &RetainedCloneStep| step.progress(),
        ), |effect, reply, _| {
            Ok(match owner.step(body, effect) {
                RetainedWorkPoll::Pending { .. } => reply.pending(),
                RetainedWorkPoll::Complete(turn) => reply.complete(turn.result),
            })
        }).result.unwrap();
    }
    panic!("Drawing composition closure did not finish")
}
fn close_source(owner: &mut FixtureSource<DrawingSnapshot>) {
    for _ in 0..100_000 {
        if owner.terminal_is_empty() {
            return;
        }
        owner.close_once().unwrap();
    }
    panic!("Original Drawing source did not close")
}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
#[test]
fn exact_original_text_loan_plus_ascii_and_multibyte_copy_quotes_progress(){let law:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧵️graph.json")).unwrap();for text in law["textLoanCases"].as_array().unwrap(){let mut content=fixture()["cases"][0]["content"].clone();let mut label=content["layers"][0]["children"][1].clone();label["content"]=text.clone();content["layers"]=serde_json::json!([label]);let document:DrawingSnapshot=serde_json::from_value(content).unwrap();let mut owner=FixtureSource::from_owner(document);let read=ChildAdmissionRead{source:semio_framework_io_ports::erase_child_admission_source(owner.borrow(),0x4348494c),admitter:&DRAWING_ADMITTER,vector:&DRAWING_VECTOR_SOURCE};let snapshot=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();let mut state=DrawingCompositionState::new();state.begin(DrawingSourceAddress::Document,DrawingPlacement::Content,LayoutBounds{x:0.0,y:0.0,width:200.0,height:100.0,rotation:0.0}).unwrap();for _ in 0..10000{if state.test_text_copy_phase()==2{break}advance(&mut state,&snapshot,&read,grant()).unwrap();}assert_eq!(state.test_text_copy_phase(),2);let mut funded=0;while state.test_text_copy_phase()==2{assert!(funded<100);let d=state.next_demand(&snapshot,&read,grant().maximum_copy_bytes).unwrap();let items=state.test_text_copy_items();let exact=VectorSceneGrant{maximum_source_items:items,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert!(matches!(advance(&mut state,&snapshot,&read,VectorSceneGrant{maximum_source_items:items-1,..exact}).unwrap(),DrawingCompositionTurn::Progress));assert_eq!(state.last_receipt(),Default::default());if d.copy_bytes>0{assert!(matches!(advance(&mut state,&snapshot,&read,VectorSceneGrant{maximum_copy_bytes:d.copy_bytes-1,..exact}).unwrap(),DrawingCompositionTurn::Progress));assert_eq!(state.last_receipt(),Default::default());}let before=state.test_text_copy_position();advance(&mut state,&snapshot,&read,exact).unwrap();assert_ne!(state.test_text_copy_position(),before);let p=state.last_receipt();assert!(p.source_items<=items&&p.copied_bytes<=d.copy_bytes);funded+=1;}assert_eq!(state.test_text_copy_buffer(),text.as_str().unwrap());close(state);close_source(&mut owner);}}
struct PaidVector;
static PAID_VECTOR:PaidVector=PaidVector;
impl semio_framework_io_ports::EmbeddedChildVectorSourceV1 for PaidVector{fn read_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:16,depth:1,..Default::default()}}fn read<'a>(&self,_:semio_framework_io_ports::ChildAdmissionSource<'a>,_:RetainedCloneGrant)->Result<semio_framework_io_ports::VectorGraphAdvance<'a>,ValueError>{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Paid fixture failure").with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:16,..Default::default()}))}}
#[test]
fn paid_original_graph_failure_survives_planner_and_composition_string_boundary(){let document:DrawingSnapshot=serde_json::from_value(fixture()["cases"][0]["content"].clone()).unwrap();let mut owner=FixtureSource::from_owner(document);let read=ChildAdmissionRead{source:semio_framework_io_ports::erase_child_admission_source(owner.borrow(),0x4348494c),admitter:&DRAWING_ADMITTER,vector:&PAID_VECTOR};let snapshot=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();let mut state=DrawingCompositionState::new();state.begin(DrawingSourceAddress::Document,DrawingPlacement::Content,LayoutBounds{x:0.0,y:0.0,width:100.0,height:100.0,rotation:0.0}).unwrap();let mut failed=false;for _ in 0..10000{if let Err(error)=advance(&mut state,&snapshot,&read,grant()){assert_eq!(error.to_string(),"Paid fixture failure");let p=state.last_receipt();assert!(p.source_items>1&&p.copied_bytes>16);assert_eq!(p,state.test_active_receipt());failed=true;break}}assert!(failed);close(state);close_source(&mut owner);}
fn vector_nodes(nodes:&mut serde_json::Value){let values=nodes.as_array_mut().unwrap();values.retain(|n|n["kind"]!="image");for node in values{if node["kind"]=="group"{vector_nodes(&mut node["children"])}}}
#[test]
fn authored_paths_fonts_and_page_canvas_use_the_actual_draw_pair(){let corpus=fixture();for row in corpus["cases"].as_array().unwrap(){for page in [None,Some(0),Some(1)]{let mut content=row["content"].clone();vector_nodes(&mut content["layers"]);if page.is_some()&&content.get("artboard").is_none(){continue}let document:DrawingSnapshot=serde_json::from_value(content).unwrap();let mut owner=FixtureSource::from_owner(document);let read=ChildAdmissionRead{source:semio_framework_io_ports::erase_child_admission_source(owner.borrow(),0x4348494c),admitter:&DRAWING_ADMITTER,vector:&DRAWING_VECTOR_SOURCE};let snapshot=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();let target=&row["target"];let target=LayoutBounds{x:target["x"].as_f64().unwrap(),y:target["y"].as_f64().unwrap(),width:target["width"].as_f64().unwrap(),height:target["height"].as_f64().unwrap(),rotation:target["rotation"].as_f64().unwrap()};let oracle=ttf_parser::Face::parse(crate::scene::text_fonts::LAYOUT_SANS,0).unwrap();let mut state=DrawingCompositionState::new();state.begin(page.map_or(DrawingSourceAddress::Document,DrawingSourceAddress::Page),if page.is_some(){DrawingPlacement::Canvas}else{DrawingPlacement::Content},target).unwrap();let(mut paths,mut glyphs,mut done)=(0,0,false);for _ in 0..1000000{match advance(&mut state,&snapshot,&read,grant()).unwrap(){DrawingCompositionTurn::Path{index}=>{state.path(index).unwrap();paths+=1},DrawingCompositionTurn::Glyph{label,glyph,..}=>{let scalar="Plan".chars().nth(glyphs%4).unwrap();assert_eq!(state.glyph(label,glyph).unwrap().glyph,oracle.glyph_index(scalar).unwrap().0);state.glyph_cluster(label,glyph).unwrap();glyphs+=1},DrawingCompositionTurn::Done=>{done=true;break},_=>{}}}assert!(done);let kinds=row["expected"]["primitiveKinds"].as_array().unwrap();assert_eq!(paths,kinds.iter().filter(|v|v.as_str()==Some("beginPath")).count());assert_eq!(glyphs,kinds.iter().filter(|v|v.as_str()==Some("text")).count()*4);close(state);close_source(&mut owner);}}}
