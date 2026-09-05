//! 🧪️ Exact app_commands OpBinary witness, field ordinals, partitions, cancellation, and hostile input.

use super::*;
use store::os_pack::{ScalarRecordWireStep, ScalarRecordWireWitness};

//#region 🔣️Fixture
fn text(field: &serde_json::Value) -> String { field["unit"].as_str().unwrap().repeat(field["repeat"].as_u64().unwrap() as usize) + field["suffix"].as_str().unwrap() }
fn command(row: &serde_json::Value) -> FlowCommand {
    let fields = &row["fields"];
    match row["id"].as_str().unwrap() {
        "evaluate" => FlowCommand::Evaluate(evaluate::Evaluate {}),
        "contextMenuAt" => FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: text(&fields[0]) }),
        "openSpotlight" => FlowCommand::OpenSpotlight(open_spotlight::OpenSpotlight {}),
        "replaceImage" => FlowCommand::ReplaceImage(replace_image::ReplaceImage { id: text(&fields[0]) }),
        "flowEvalTick" => FlowCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}),
        "flowEvalResolve" => FlowCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { node_hash: fields[0]["value"].as_str().unwrap().parse().unwrap(), output_json: text(&fields[1]) }),
        _ => unreachable!(),
    }
}
fn advance(cursor: &mut ScalarRecordWireWitness<FlowCommand>, wire: &[u8], grant: usize, cancel: Option<usize>) -> Result<bool, &'static str> {
    let mut offset = 0; let mut steps = 0;
    loop { let mut charged = 0;
        for _ in 0..grant {
            if cancel == Some(steps) { return Ok(false); } steps += 1;
            match cursor.advance(wire.get(offset).copied())? {
                ScalarRecordWireStep::Progress { compared_bytes } => { assert!(compared_bytes <= 1); charged += compared_bytes; }
                ScalarRecordWireStep::Consumed { compared_bytes } => { assert_eq!(compared_bytes, 1); charged += compared_bytes; offset += 1; }
                ScalarRecordWireStep::Complete => { assert_eq!(offset, wire.len()); return Ok(true); }
            }
            assert!(charged <= grant); assert!(steps < 500_000);
        }
    }
}
fn close(mut cursor: ScalarRecordWireWitness<FlowCommand>, root: &Arc<FlowCommand>) {
    cursor.begin_close(); let original = cursor.take_root().unwrap(); assert!(Arc::ptr_eq(&original, root)); assert!(cursor.terminal_is_empty());
}
//#endregion 🔣️Fixture

//#region 🧪️Laws
#[test]
fn host_wire_witness_matches_real_opbinary_for_all_six_routes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/📡️host-wire/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let root = Arc::new(command(row)); let bytes = protocol::OpBinary::encode_op(root.as_ref()).unwrap();
        assert_eq!(bytes.len(), row["wireBytes"].as_u64().unwrap() as usize); assert_eq!(bytes[0], 1); assert_eq!(bytes[1] as u64, row["ordinal"].as_u64().unwrap());
        for grant in fixture["grants"].as_array().unwrap() { let mut cursor = ScalarRecordWireWitness::new(root.clone(), flow_scalar_command_view); assert!(advance(&mut cursor, &bytes, grant.as_u64().unwrap() as usize, None).unwrap()); close(cursor, &root); }
        for stop in fixture["cancelAfterSteps"].as_array().unwrap() { let mut cursor = ScalarRecordWireWitness::new(root.clone(), flow_scalar_command_view); let _ = advance(&mut cursor, &bytes, 1, Some(stop.as_u64().unwrap() as usize)).unwrap(); close(cursor, &root); }
        let mut wrong_ordinal = bytes.clone(); wrong_ordinal[1] ^= 1; let mut trailing = bytes.clone(); trailing.push(0);
        for invalid in [wrong_ordinal, trailing, bytes[..bytes.len()-1].to_vec(), br#"["evaluate",{}]"#.to_vec()] { let mut cursor = ScalarRecordWireWitness::new(root.clone(), flow_scalar_command_view); assert!(advance(&mut cursor, &invalid, 1, None).is_err()); assert!(cursor.advance(Some(1)).is_err()); close(cursor, &root); }
    }
    eprintln!("[DEBUG] Flow actual OpBinary six-route wire parity/cancel/fault laws reached terminal emptiness");
}
//#endregion 🧪️Laws
