//! ⚖️ Mathematical app — binary command protocol surface + laws (constitutional: protocol).

use mathematical::{MathCamera, MathGeometry, MathGraphDsl};
use mathematical_op::MathOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `MathOperation` to its binary command form.
pub fn encode_op(operation: &MathOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `MathOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<MathOperation, protocol::ProtocolError> {
    MathOperation::decode_op(bytes)
}

//#region 🔖️MathCommand
/// 🎯️ B1: `MathematicalPlayApp::Command` — the SOLE dispatch surface for mathematical's own behavior
/// (mirrors `shooting_protocol::ShootingCommand`). One variant per `create_mathematical_app`'s declared
/// action; field shapes mirror each action's former JSON `args` object exactly. `NodeGraphEdit` keeps
/// its former batched-array shape (`operations_json`, a JSON array of tagged sub-edits) verbatim rather
/// than splitting into one typed variant per sub-edit kind: `nodeGraphActions.edit` (`"nodeGraphEdit"`,
/// `🧰️framework/⚡️implementations/🟦️typescript/📦️index.ts`) is the shared renderer-wide action id the
/// generic node-graph canvas dispatches its edit gestures under (see the React node-graph surface,
/// `dispatch(nodeGraphActions.edit, { operations: [...] })`) — renaming or splitting it here would
/// silently strand every node-graph interaction the frontend still targets under that id.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
pub enum MathCommand {
    #[dsl(key = "set-document")]
    SetDocument {
        #[dsl(block)]
        graph: MathGraphDsl,
        #[dsl(block)]
        geometry: MathGeometry,
    },
    #[dsl(key = "set-algorithm")]
    SetAlgorithm { algorithm: String, seed: Option<String> },
    #[dsl(key = "set-directed")]
    SetDirected { directed: bool },
    #[dsl(key = "node-graph-edit")]
    NodeGraphEdit { operations_json: String },
    #[dsl(key = "node-graph-viewport")]
    NodeGraphViewport {
        #[dsl(block)]
        camera: MathCamera,
    },
    #[dsl(key = "set-points")]
    SetPoints {
        #[dsl(block)]
        geometry: MathGeometry,
    },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}
//#endregion 🔖️MathCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use mathematical::{MathGraph, MathProjection};

    #[test]
    fn math_document_text_round_trips_through_store() {
        let initial = MathProjection::default();
        let envelope = store::create_document_envelope("semio.mathematical/v1", "math-demo", initial, None);
        let mut store = store::DocumentStore::new(envelope);
        let mut graph = MathGraph::default();
        graph.algorithm = "components".into();
        store.dispatch(store::DocumentCommand::Apply { operations: vec![MathOperation::SetGraph { graph }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `MathOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this file's
    /// existing pack round-trip law (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};

        let initial = MathProjection::default();
        let envelope = store::create_document_envelope("semio.mathematical/v1", "math-demo", initial, None);
        let mut store = store::DocumentStore::new(envelope);
        let mut graph = MathGraph::default();
        graph.algorithm = "components".into();
        store.dispatch(store::DocumentCommand::Apply { operations: vec![MathOperation::SetGraph { graph }], description: None }).expect("apply");
        let edit: &Edit<MathOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<MathProjection, MathOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests

    //#region MathCommand
    #[test]
    fn command_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&MathCommand::SetDocument { graph: mathematical::math_graph_to_dsl(&MathGraph::default()), geometry: MathGeometry::default() });
        store::test_support::assert_op_line_round_trip(&MathCommand::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) });
        store::test_support::assert_op_line_round_trip(&MathCommand::SetAlgorithm { algorithm: "topo".into(), seed: None });
        store::test_support::assert_op_line_round_trip(&MathCommand::SetDirected { directed: true });
        store::test_support::assert_op_line_round_trip(&MathCommand::NodeGraphEdit { operations_json: r#"[{"operation":"addNode","x":12.0,"y":34.0}]"#.into() });
        store::test_support::assert_op_line_round_trip(&MathCommand::NodeGraphViewport { camera: MathCamera { x: 5.0, y: 6.0, zoom: 2.0 } });
        store::test_support::assert_op_line_round_trip(&MathCommand::SetPoints { geometry: MathGeometry::default() });
        store::test_support::assert_op_line_round_trip(&MathCommand::SetLocale { value: "de-DE".into() });

        let bytes = MathCommand::SetDirected { directed: false }.encode_op().expect("encode");
        assert_eq!(MathCommand::decode_op(&bytes).expect("decode"), MathCommand::SetDirected { directed: false });
    }
    //#endregion MathCommand
}
//#endregion 🧪️Tests

//#region 🧪️DEBUG_WireBaseline
#[cfg(test)]
mod wire_baseline_dump {
    use super::*;
    use mathematical::{MathCamera, MathGeometry, MathGraph};

    fn dump(label: &str, command: &MathCommand) {
        let text = protocol::OpText::print_op(command);
        let bytes = protocol::OpBinary::encode_op(command).expect("encode");
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        println!("[DEBUG] {label} | text={text:?} | len={} | hex={hex}", bytes.len());
    }

    #[test]
    fn dump_every_command_variant() {
        dump("SetDocument", &MathCommand::SetDocument { graph: mathematical::math_graph_to_dsl(&MathGraph::default()), geometry: MathGeometry::default() });
        dump("SetAlgorithm(topo,None)", &MathCommand::SetAlgorithm { algorithm: "topo".into(), seed: None });
        dump("SetAlgorithm(bfs,Some(a))", &MathCommand::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) });
        dump("SetDirected(true)", &MathCommand::SetDirected { directed: true });
        dump("SetDirected(false)", &MathCommand::SetDirected { directed: false });
        dump("NodeGraphEdit", &MathCommand::NodeGraphEdit { operations_json: r#"[{"operation":"addNode","x":12.0,"y":34.0}]"#.into() });
        dump("NodeGraphViewport", &MathCommand::NodeGraphViewport { camera: MathCamera { x: 5.0, y: 6.0, zoom: 2.0 } });
        dump("SetPoints", &MathCommand::SetPoints { geometry: MathGeometry::default() });
        dump("SetLocale", &MathCommand::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️DEBUG_WireBaseline
