//! ⚖️ Puzzle 3d app — binary command protocol surface + laws (constitutional: protocol).

use puzzle_3d_op::Puzzle3dOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `Puzzle3dOperation` to its binary command form.
pub fn encode_op(operation: &Puzzle3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Puzzle3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Puzzle3dOperation, protocol::ProtocolError> {
    Puzzle3dOperation::decode_op(bytes)
}

//#region 🔖️Puzzle3dEngineCommand
/// 🎯️ Re-exports the puzzle 3d app-engine command envelope (`HEADLESS-ENGINE-LAW-AND-OFFENDER-FIXES`).
/// `#[derive(dsl::DslOps)]` is applied where the type is declared, in `puzzle_3d_engine` — not here —
/// because the derive's generated code needs `SceneConfig`/`BrushPlacePayload` (types `puzzle_3d_engine`
/// owns) by value, and this crate depends on `puzzle_3d_engine` (not the other way around); re-exporting
/// it here plus wrapping `encode_op`/`decode_op` mirrors exactly how `Puzzle3dOperation` (declared in
/// `puzzle_3d_op`) is surfaced above.
pub use puzzle_3d_engine::{Puzzle3dEngineCommand, Puzzle3dEngineOutcome};

/// 📦️ Encodes a `Puzzle3dEngineCommand` to its binary command form.
pub fn encode_engine_command(command: &Puzzle3dEngineCommand) -> Result<Vec<u8>, protocol::ProtocolError> {
    command.encode_op()
}

/// 📖️ Decodes a `Puzzle3dEngineCommand` from its binary command form.
pub fn decode_engine_command(bytes: &[u8]) -> Result<Puzzle3dEngineCommand, protocol::ProtocolError> {
    Puzzle3dEngineCommand::decode_op(bytes)
}
//#endregion 🔖️Puzzle3dEngineCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle3d_document_vcs_replays_granular_operations() {
        use puzzle_3d::{Puzzle3dObject, PUZZLE_3D_SCHEMA};
        use puzzle_3d_op::Puzzle3dStore;
        use store::{create_document_envelope, DocumentCommand};

        let mut store = Puzzle3dStore::new(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", puzzle_3d_engine::empty_puzzle3d_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![Puzzle3dOperation::SetObject { index: 0, object: Puzzle3dObject { id: "o1".into(), label: None, object_kind: None, origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false } }],
                description: None,
            })
            .expect("apply");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.objects.len(), 1);
        assert_eq!(projection.objects[0].id, "o1");
    }

    /// 🔗️ Minimal scene JSON matching `puzzle_3d_engine::SceneConfig`'s real wire shape (camelCase,
    /// per its `#[serde(rename = ...)]` attrs) — deserialized rather than struct-literal-built since
    /// `SceneConfig`'s fields are private to `puzzle_3d_engine` (this crate only needs the type
    /// nameable, not its fields, to carry it inside `Puzzle3dEngineCommand::SetScene`).
    fn sample_scene_config() -> puzzle_3d_engine::SceneConfig {
        let json = r#"{
            "fixture": {
                "objects": [{"id": "host", "objectKind": "Host", "meshUrl": "/test/host.glb", "origin": [0,0,0], "orientation": [0,0,0,1], "vortices": [{"id": "v0", "vortexKind": "port-a", "position": [0,0,0], "direction": [0,0,-1]}]}],
                "attractions": [],
                "targetVolumes": []
            },
            "kindCatalogs": {"objects": [{"id": "Host", "meshUrl": "/test/host.glb", "vortices": []}], "vortices": [{"id": "port-a"}], "cables": []},
            "kindCompatibility": [],
            "overlapBudget": 0.02,
            "seed": 1
        }"#;
        serde_json::from_str(json).expect("sample scene config parses")
    }

    #[test]
    fn engine_command_set_scene_binary_round_trips_and_agrees_with_text() {
        let command = Puzzle3dEngineCommand::SetScene { scene: sample_scene_config() };
        store::test_support::assert_op_text_binary_equivalence(&command);
        let bytes = encode_engine_command(&command).expect("encode");
        assert_eq!(decode_engine_command(&bytes).expect("decode"), command);
    }

    #[test]
    fn engine_command_brush_preview_binary_round_trips_and_agrees_with_text() {
        let command = Puzzle3dEngineCommand::BrushPreview { vortex_full_id: "host:v0".to_string(), candidate_index: 2 };
        store::test_support::assert_op_text_binary_equivalence(&command);
        let bytes = encode_engine_command(&command).expect("encode");
        assert_eq!(decode_engine_command(&bytes).expect("decode"), command);
    }

    #[test]
    fn engine_command_update_kind_weights_binary_round_trips_and_agrees_with_text() {
        let mut object_weights = std::collections::BTreeMap::new();
        object_weights.insert("Host".to_string(), 0.5);
        let command = Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights: std::collections::BTreeMap::new() };
        store::test_support::assert_op_text_binary_equivalence(&command);
        let bytes = encode_engine_command(&command).expect("encode");
        assert_eq!(decode_engine_command(&bytes).expect("decode"), command);
    }

    /// 🎯️ Behavioral parity: `dispatch` must reach the exact same `Puzzle3dEngine` logic the old
    /// JSON-string wasm-bindgen methods delegated to — `SetScene` seeds a fill session,
    /// `ApplyFillCount`/`ComposeFillDisplay` read/apply its prefix, matching what
    /// `puzzle_3d_engine`'s own `precompute_session_native_wrapper_exercises_public_methods` test
    /// already asserts for the pre-dispatch API.
    #[test]
    fn dispatch_set_scene_then_apply_and_compose_fill_count_round_trip() {
        use puzzle_3d_engine::{Puzzle3dEngineOutcome, Puzzle3dPrecomputeSession};

        let mut session = Puzzle3dPrecomputeSession::new();
        session.dispatch(Puzzle3dEngineCommand::SetScene { scene: sample_scene_config() }).expect("set scene");
        assert!(!session.fill_is_done(), "a freshly seeded fill session has not stalled or hit max_count yet");

        session.precompute_step(50);

        let outcome = session.dispatch(Puzzle3dEngineCommand::ComposeFillDisplay { count: 0 }).expect("compose fill display");
        let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
        assert!(fixture.objects.iter().any(|object| object.id == "host"), "the base scene's host object must survive compose_fill_display(0)");

        let outcome = session.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count: 0 }).expect("apply fill count");
        let Puzzle3dEngineOutcome::Fixture(fixture) = outcome else { panic!("expected a Fixture outcome") };
        assert!(fixture.objects.iter().any(|object| object.id == "host"));
    }

    #[test]
    fn dispatch_brush_preview_without_scene_returns_none() {
        use puzzle_3d_engine::{Puzzle3dEngineOutcome, Puzzle3dPrecomputeSession};

        let mut session = Puzzle3dPrecomputeSession::new();
        let outcome = session.dispatch(Puzzle3dEngineCommand::BrushPreview { vortex_full_id: "host:v0".to_string(), candidate_index: 0 }).expect("brush preview never errors");
        assert_eq!(outcome, Puzzle3dEngineOutcome::BrushPreview(None), "no scene means no cached brush candidates yet");
    }
}
//#endregion 🧪️Tests
