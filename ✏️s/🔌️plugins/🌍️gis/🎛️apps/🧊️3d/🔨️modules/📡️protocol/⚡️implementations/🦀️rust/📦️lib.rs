//! ⚖️ GIS 3D app — binary command protocol surface + laws (constitutional: protocol).

use gis3d_op::Gis3dTerrainOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `Gis3dTerrainOperation` to its binary command form.
pub fn encode_op(operation: &Gis3dTerrainOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Gis3dTerrainOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Gis3dTerrainOperation, protocol::ProtocolError> {
    Gis3dTerrainOperation::decode_op(bytes)
}

//#region 🔖️Gis3dCommand
/// 🎯️ B1: `Gis3dPlayApp::Command` — the SOLE dispatch surface for gis3d's own behavior, covering
/// every action `create_gis3d_app` declares. Mirrors `gis2d_protocol::Gis2dCommand`'s/
/// `shooting_protocol::ShootingCommand`'s identical `#[derive(dsl::DslOps)]` conventions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Gis3dCommand {
    // 🔧️ Document-mutating — dispatched as a VCS operation with a true inverse.
    #[dsl(key = "exaggeration")]
    SetExaggeration { exaggeration: f64 },

    // 👁️ Config-only (was ephemeral `Gis3dPlayRuntime` state) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "camera")]
    SetCamera { camera_json: String },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "world-select")]
    WorldSelect { ids: Vec<String> },
    /// 🗣️ B1: locale is `cfg.locale`, set via this typed config command — not palette-declared
    /// (host/test infra dispatches it directly, mirrors `gis2d_protocol::Gis2dCommand::SetLocale`).
    #[dsl(key = "locale")]
    SetLocale { value: String },
}
//#endregion 🔖️Gis3dCommand

//#region 🧪️WireBaseline
/// 🧪️ [DEBUG] Temporary pre-migration wire dump (crate-consolidation ticket
/// `26/08/05/GIS-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`, TEMPLATE §0.4) — deleted with
/// this crate.
#[cfg(test)]
mod wire_baseline {
    use super::*;
    use protocol::OpText;

    fn dump(label: &str, command: &Gis3dCommand) {
        let bytes = command.encode_op().expect("encode");
        println!("{label} | {} | {} | {}", command.print_op(), bytes.len(), bytes.iter().map(|b| format!("{b:02x}")).collect::<String>());
    }

    fn dump_op(label: &str, operation: &Gis3dTerrainOperation) {
        let bytes = encode_op(operation).expect("encode");
        println!("{label} | {} | {} | {}", operation.print_op(), bytes.len(), bytes.iter().map(|b| format!("{b:02x}")).collect::<String>());
    }

    #[test]
    fn gis3d_wire_baseline() {
        dump("SetExaggeration", &Gis3dCommand::SetExaggeration { exaggeration: 2.5 });
        dump("SetCamera", &Gis3dCommand::SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() });
        dump("SetSelection", &Gis3dCommand::SetSelection { ids: vec!["p1".into()] });
        dump("SetSelection.empty", &Gis3dCommand::SetSelection { ids: Vec::new() });
        dump("WorldSelect", &Gis3dCommand::WorldSelect { ids: vec!["p1".into()] });
        dump("SetLocale", &Gis3dCommand::SetLocale { value: "de-DE".into() });

        dump_op("Op.SetExaggeration", &Gis3dTerrainOperation::SetExaggeration { exaggeration: 3.0 });
        dump_op("Op.SetImportedFeatures", &Gis3dTerrainOperation::SetImportedFeatures { features_json: r#"{"positions":[]}"#.into() });
        dump_op("Op.SetDocument", &Gis3dTerrainOperation::SetDocument { document: gis3d::Gis3dTerrainDocument { exaggeration: 2.0, imported_features_json: "null".into() } });
    }
}
//#endregion 🧪️WireBaseline

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use gis3d::Gis3dTerrainDocument;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = Gis3dTerrainOperation::SetExaggeration { exaggeration: 2.0 };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn gis3d_terrain_document_text_round_trips_through_store() {
        let initial = Gis3dTerrainDocument { exaggeration: 1.0, imported_features_json: String::new() };
        let envelope = store::create_document_envelope(gis3d::GIS_3D_TERRAIN_SCHEMA, "gis3d-demo", initial, None);
        let mut store = store::DocumentStore::new(envelope);
        store.dispatch(store::DocumentCommand::Apply { operations: vec![Gis3dTerrainOperation::SetExaggeration { exaggeration: 2.0 }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    #[test]
    fn gis3d_command_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&Gis3dCommand::SetExaggeration { exaggeration: 2.5 });
        store::test_support::assert_op_line_round_trip(&Gis3dCommand::SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() });
        store::test_support::assert_op_line_round_trip(&Gis3dCommand::SetSelection { ids: vec!["p1".into()] });
        store::test_support::assert_op_line_round_trip(&Gis3dCommand::WorldSelect { ids: vec!["p1".into()] });
        store::test_support::assert_op_line_round_trip(&Gis3dCommand::SetLocale { value: "de-DE".into() });
    }

    #[test]
    fn gis3d_command_binary_round_trips() {
        let command = Gis3dCommand::SetExaggeration { exaggeration: 2.5 };
        let bytes = command.encode_op().expect("encode");
        assert_eq!(Gis3dCommand::decode_op(&bytes).expect("decode"), command);
    }
}
//#endregion 🧪️Tests
