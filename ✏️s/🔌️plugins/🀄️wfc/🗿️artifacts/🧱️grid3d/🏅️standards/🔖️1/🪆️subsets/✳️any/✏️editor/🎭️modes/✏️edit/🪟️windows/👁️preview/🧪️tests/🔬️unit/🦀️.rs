//! 🔬️ Preview-window laws — a solved document publishes one instance per solved cell, the residency
//! makes the delta incremental instead of a second copy of the full set, and the status line stays
//! short enough to admit.

use crate::editor::grid3d::window::Grid3dWindowConfig;
use crate::host::inferences::solve;
use crate::editor::grid3d::modes::edit::windows::preview::*;
use crate::schema::inferences::Grid3dAssignment;
use crate::standards::v1::subsets::any::io::text::inferences::scene_projection::preview_instances_json;

fn row(x: u32, y: u32, z: u32, tile: &str) -> Grid3dAssignment {
    Grid3dAssignment { x, y, z, tile_id: tile.to_string() }
}

#[test]
fn the_window_kind_is_a_read_only_world3d_surface() {
    let definition = definition();
    assert_eq!(definition.id, WINDOW_KIND_ID);
    assert_eq!(definition.actions.len(),1);
    assert_eq!(definition.actions[0].id,"commit-fill");
    assert!(!definition.actions[0].in_palette);
    assert!(definition.utilities.is_empty());
}

#[test]
fn every_example_renders_a_completed_instance_local_payload() {
    use crate::editor::grid3d::{transient::Grid3dTransient,modes::edit::tools::fill::{Grid3dFillPayload,Grid3dFillCell}};
    for document in [crate::examples::blocks::snapshot(),crate::examples::pipes_3d::snapshot()] {
        let solved=solve(&document).expect("example solves");
        let payload=Grid3dFillPayload{source_revision:vec![7;32],assignments:solved.assignments.iter().map(|row|Grid3dFillCell{x:row.x,y:row.y,z:row.z,tile_id:Some(row.tile_id.clone())}).collect(),done:true,..Default::default()};
        let transient=Grid3dTransient{payload_json:String::from_utf8(payload.encode()).unwrap()};
        let node=render(&document,&Grid3dWindowConfig::default(),&transient,None,Some(&[7;32])).expect("preview renders");
        assert!(!format!("{node:?}").is_empty());
        let (assignments,_)=paint_assignments(&transient,None,Some(&[7;32]));
        let instances=preview_instances_json(&document,&assignments);
        assert_eq!(instances.matches("\"meshId\"").count(),(document.width*document.height*document.depth) as usize-document.masked.len());
        assert!(paint_assignments(&Grid3dTransient::default(),None,Some(&[7;32])).0.is_empty());
    }
}

#[test]
fn a_contradiction_paints_the_empty_scene_and_says_so() {
    let mut document = crate::examples::blocks::snapshot();
    document.rules.clear();
    render(&document, &Grid3dWindowConfig::default(), &crate::editor::grid3d::transient::Grid3dTransient::default(), None, Some(&[7;32])).expect("preview renders");
    let status = status_json(&document, false, "[]");
    assert!(status.contains("contradiction"));
}

#[test]
fn the_status_line_stays_inside_one_ui_text_admission_unit() {
    let document = crate::examples::blocks::snapshot();
    let status = status_json(&document, true, &preview_instances_json(&document, &solve(&document).expect("solves").assignments));
    assert!(status.len() <= 512, "an oversized status kills the whole surface refresh, not just this field");
    assert!(status.contains("\"state\""));
}

#[test]
fn a_partial_fill_preview_differs_from_empty_and_finished() {
    let document = crate::examples::blocks::snapshot();
    let oracle = solve(&document).expect("blocks solves");
    let empty = preview_instances_json(&document, &[]);
    let finished = preview_instances_json(&document, &oracle.assignments);
    let mut partial = oracle.assignments.clone();
    partial.truncate((oracle.assignments.len() / 2).max(1));
    assert!(partial.len() < oracle.assignments.len());
    let mid = preview_instances_json(&document, &partial);
    assert_ne!(mid, empty, "a partial board must not look empty");
    assert_ne!(mid, finished, "a partial board must not look finished");
}

/// 🪪️ Shared portable revisions fence every app-local fill result.
#[test]
fn preview_payload_requires_the_exact_render_revision() {
    let fixture: serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../🧪️tests/🧫️preview-revisions/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let source: Vec<u8>=serde_json::from_value(case["sourceRevision"].clone()).unwrap();
        let revision: Option<[u8;32]>=serde_json::from_value(case["currentRevision"].clone()).unwrap();
        assert_eq!(source_matches(&source,revision.as_ref()),case["visible"].as_bool().unwrap(),"{}",case["id"]);
    }
}

#[test]
fn an_unsolved_preview_never_creates_an_assignment() {
    let document=crate::examples::blocks::snapshot();
    let transient=crate::editor::grid3d::transient::Grid3dTransient::default();
    let (assignments,satisfiable)=paint_assignments(&transient,None,Some(&[7;32]));
    assert!(assignments.is_empty());
    assert!(satisfiable);
    let node=render(&document,&Grid3dWindowConfig::default(),&transient,None,Some(&[7;32])).unwrap();
    assert!(format!("{node:?}").contains("unsolved"));
}

#[test]
fn a_completed_payload_survives_dismissal_only_at_its_source_revision() {
    use crate::editor::grid3d::{transient::Grid3dTransient,modes::edit::tools::fill::{Grid3dFillPayload,Grid3dFillCell}};
    let payload=Grid3dFillPayload{source_revision:vec![7;32],assignments:vec![Grid3dFillCell{x:0,y:0,z:0,tile_id:Some("air".into())}],contradiction:false,done:true,trace:vec![crate::editor::grid3d::modes::edit::tools::fill::Grid3dFillTraceEvent{x:0,y:0,z:0,tile_id:"discarded-choice".into(),discarded:true}]};
    let transient=Grid3dTransient{payload_json:String::from_utf8(payload.encode()).unwrap()};
    assert_eq!(paint_assignments(&transient,None,Some(&[7;32])).0,vec![row(0,0,0,"air")]);
    assert!(paint_assignments(&transient,None,Some(&[8;32])).0.is_empty());
    assert!(paint_assignments(&transient,None,None).0.is_empty());
    let contradiction=Grid3dFillPayload{contradiction:true,assignments:Vec::new(),..payload};
    let transient=Grid3dTransient{payload_json:String::from_utf8(contradiction.encode()).unwrap()};
    assert_eq!(paint_assignments(&transient,None,Some(&[7;32])),(Vec::new(),false));
}

/// 📬️ A supported tick larger than64KiB still fits the owned completion command.
#[test]
fn a_sixteen_cell_cube_keeps_its_completed_fill_payload() {
    use crate::editor::grid3d::{Grid3dEditorCommand,modes::edit::tools::fill::{Grid3dFillPayload,Grid3dFillCell}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️commit-envelope/🔣️.json")).unwrap();
    let size:Vec<u32>=serde_json::from_value(fixture["size"].clone()).unwrap();
    let mut assignments=Vec::new();
    for z in 0..size[2]{for y in 0..size[1]{for x in 0..size[0]{assignments.push(Grid3dFillCell{x,y,z,tile_id:Some(fixture["tileId"].as_str().unwrap().into())});}}}
    let payload=Grid3dFillPayload{source_revision:serde_json::from_value(fixture["sourceRevision"].clone()).unwrap(),assignments,done:true,..Default::default()};
    assert_eq!(payload.assignments.len(),fixture["expectedAssignmentsCount"].as_u64().unwrap() as usize);
    let bytes=payload.encode();
    assert_eq!(bytes.len(),fixture["expectedPayloadBytes"].as_u64().unwrap() as usize);
    assert!(bytes.len()>65536);
    assert!(bytes.len()<semio_framework_tool_run::ToolRunReceivingLimits::REPORTS.tick.maximum_file_bytes);
    let command=Grid3dEditorCommand::CommitFill{payload_json:String::from_utf8(bytes).unwrap()};
    let encoded=protocol::OpBinary::encode_op(&command).unwrap();
    assert!(encoded.len()<fixture["maximumRawBytes"].as_u64().unwrap() as usize);
    assert_eq!(<Grid3dEditorCommand as protocol::OpBinary>::decode_op(&encoded).unwrap(),command);
    assert_eq!(crate::editor::grid3d::GRID3D_RETAINED_RAW_BYTES,fixture["maximumRawBytes"].as_u64().unwrap() as usize);
    assert_eq!(crate::editor::grid3d::GRID3D_RETAINED_OUTPUT_BYTES,fixture["maximumOutputBytes"].as_u64().unwrap() as usize);
}

/// 🪣️ Both the original run identity and decoded live payload bind a progressive preview.
#[test]
fn a_live_preview_cannot_cross_run_or_document_revisions() {
    use crate::editor::grid3d::{transient::Grid3dTransient,modes::edit::tools::fill::{Grid3dFillPayload,Grid3dFillCell}};
    use semio_framework_tool_run::{ToolRunIdentity,ToolRunId,ToolRunState};
    let identity=ToolRunIdentity::new(ToolRunId{app_instance_id:1,run:1},[7;32]);
    let mut run=semio_framework_plugin::ToolRunView::new("fill",identity,ToolRunState::Running);
    let payload=Grid3dFillPayload{source_revision:vec![7;32],assignments:vec![Grid3dFillCell{x:0,y:0,z:0,tile_id:Some("air".into())}],..Default::default()};
    run.payload=Some(std::sync::Arc::new(payload.encode()));
    let empty=Grid3dTransient::default();
    assert_eq!(paint_assignments(&empty,Some(&run),Some(&[7;32])).0,vec![row(0,0,0,"air")]);
    assert!(paint_assignments(&empty,Some(&run),Some(&[8;32])).0.is_empty());
    run.identity.base_revision=[8;32];
    assert!(paint_assignments(&empty,Some(&run),Some(&[7;32])).0.is_empty());
    run.identity=identity;
    run.payload=Some(std::sync::Arc::new(Grid3dFillPayload{source_revision:vec![8;32],..payload}.encode()));
    assert!(paint_assignments(&empty,Some(&run),Some(&[7;32])).0.is_empty());
    run.state=ToolRunState::Aborted;
    assert!(paint_assignments(&empty,Some(&run),Some(&[7;32])).0.is_empty());
}
