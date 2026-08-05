//! ⚖️ GIS 2D app — binary command protocol surface + laws (constitutional: protocol).

use gis2d_op::GisMapOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `GisMapOperation` to its binary command form.
pub fn encode_op(operation: &GisMapOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `GisMapOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<GisMapOperation, protocol::ProtocolError> {
    GisMapOperation::decode_op(bytes)
}

//#region 🔖️Gis2dCommand
/// 🎯️ B1: `Gis2dPlayApp::Command` — the SOLE dispatch surface for gis2d's own behavior, covering
/// every action `create_gis2d_app` declares. Field shapes mirror each action's real `args` object
/// exactly. `#[derive(dsl::DslOps)]` gives this a binary (`OpBinary`) AND text (`OpText`) codec,
/// matching `GisMapOperationDsl`'s (`gis2d_op`) and `shooting_protocol::ShootingCommand`'s identical
/// derive/attribute conventions, even though this enum is never dispatched through
/// `store::DocumentCommand` (it is not a `protocol::Operation` — no `diff`/`backwards` — purely a
/// command-channel wire codec).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Gis2dCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "patch-positions")]
    PatchPositions { positions_json: String },
    #[dsl(key = "patch-routes")]
    PatchRoutes { route_ids: Vec<String>, field: String, value: String },
    #[dsl(key = "patch-route")]
    PatchRoute { route_id: String, field: String, value: String },

    // 👁️ Config-only (was ephemeral `Gis2dPlayRuntime` state) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "toggle-layer-visibility")]
    ToggleLayerVisibility { layer_id: String },
    #[dsl(key = "fit-world")]
    FitWorld,
    #[dsl(key = "camera")]
    SetCamera { camera_json: String },
    #[dsl(key = "render-mode")]
    SetRenderMode { value: String },
    #[dsl(key = "vector-style")]
    SetVectorStyle { value: String },
    #[dsl(key = "lod-mode")]
    SetLodMode { value: String },
    #[dsl(key = "feature-selection")]
    SetFeatureSelection { positions: Vec<String>, routes: Vec<String>, mode: String },
    #[dsl(key = "hover")]
    SetHover { hover_json: String },
    #[dsl(key = "selection-method")]
    SetSelectionMethod { value: String },
    #[dsl(key = "selection-mode")]
    SetSelectionMode { value: String },
    #[dsl(key = "clear-selection")]
    ClearSelection,
    #[dsl(key = "select-all")]
    SelectAll,
    #[dsl(key = "deselect")]
    Deselect { feature_id: String, feature_kind: String },
    #[dsl(key = "focus-feature")]
    FocusFeature { feature_id: String, feature_kind: String },
    #[dsl(key = "layer-stroke-scale")]
    SetLayerStrokeScale { layer_id: String, value: f64 },
    /// 🗣️ B1: locale is `cfg.locale`, set via this typed config command — no more `ViewState`-pushed
    /// locale (mirrors `shooting_protocol::ShootingCommand::SetLocale`). Not palette-declared (host/test
    /// infra dispatches it directly, same as the shooting pilot).
    #[dsl(key = "locale")]
    SetLocale { value: String },

    // 🌐️ Shell effect — opens the picked feature's source URL through the host.
    #[dsl(key = "open-source")]
    OpenSource { feature_id: String },
}
//#endregion 🔖️Gis2dCommand

//#region 🧪️WireBaseline
/// 🧪️ [DEBUG] Temporary pre-migration wire dump (crate-consolidation ticket
/// `26/08/05/GIS-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`, TEMPLATE §0.4) — deleted with
/// this crate.
#[cfg(test)]
mod wire_baseline {
    use super::*;
    use gis2d_op::GisMapOperation;
    use protocol::{CollectionOperation, OpText};

    fn dump(label: &str, command: &Gis2dCommand) {
        let bytes = command.encode_op().expect("encode");
        println!("{label} | {} | {} | {}", command.print_op(), bytes.len(), bytes.iter().map(|b| format!("{b:02x}")).collect::<String>());
    }

    fn dump_op(label: &str, operation: &GisMapOperation) {
        let bytes = protocol::OpBinary::encode_op(operation).expect("encode");
        println!("{label} | {} | {} | {}", operation.print_op(), bytes.len(), bytes.iter().map(|b| format!("{b:02x}")).collect::<String>());
    }

    fn feature() -> gis2d::MapFeature {
        gis2d::MapFeature { id: "p1".into(), data: dsl::to_dsl_value(&serde_json::json!({ "id": "p1", "lon": 1.0, "lat": 2.0 })).unwrap_or(dsl::DslValue::Null) }
    }

    #[test]
    fn gis2d_wire_baseline() {
        dump("SetActiveExample", &Gis2dCommand::SetActiveExample { example_id: "reuse-map".into() });
        dump("PatchPositions", &Gis2dCommand::PatchPositions { positions_json: r#"[{"id":"p1","lon":1.0,"lat":2.0}]"#.into() });
        dump("PatchRoutes", &Gis2dCommand::PatchRoutes { route_ids: vec!["r1".into(), "r2".into()], field: "label".into(), value: "Home".into() });
        dump("PatchRoutes.empty", &Gis2dCommand::PatchRoutes { route_ids: Vec::new(), field: "label".into(), value: String::new() });
        dump("PatchRoute", &Gis2dCommand::PatchRoute { route_id: "r1".into(), field: "label".into(), value: "Home".into() });
        dump("SetSelection", &Gis2dCommand::SetSelection { ids: vec!["roads".into()] });
        dump("SetSelection.empty", &Gis2dCommand::SetSelection { ids: Vec::new() });
        dump("ToggleLayerVisibility", &Gis2dCommand::ToggleLayerVisibility { layer_id: "water".into() });
        dump("FitWorld", &Gis2dCommand::FitWorld);
        dump("SetCamera", &Gis2dCommand::SetCamera { camera_json: r#"{"x":0,"y":0,"zoom":1}"#.into() });
        dump("SetRenderMode", &Gis2dCommand::SetRenderMode { value: "vector".into() });
        dump("SetVectorStyle", &Gis2dCommand::SetVectorStyle { value: "colored".into() });
        dump("SetLodMode", &Gis2dCommand::SetLodMode { value: "automatic".into() });
        dump("SetFeatureSelection", &Gis2dCommand::SetFeatureSelection { positions: vec!["p1".into()], routes: vec!["r1".into()], mode: "default".into() });
        dump("SetFeatureSelection.empty", &Gis2dCommand::SetFeatureSelection { positions: Vec::new(), routes: Vec::new(), mode: "additive".into() });
        dump("SetHover", &Gis2dCommand::SetHover { hover_json: "null".into() });
        dump("SetSelectionMethod", &Gis2dCommand::SetSelectionMethod { value: "lasso".into() });
        dump("SetSelectionMode", &Gis2dCommand::SetSelectionMode { value: "additive".into() });
        dump("ClearSelection", &Gis2dCommand::ClearSelection);
        dump("SelectAll", &Gis2dCommand::SelectAll);
        dump("Deselect", &Gis2dCommand::Deselect { feature_id: "p1".into(), feature_kind: "position".into() });
        dump("FocusFeature", &Gis2dCommand::FocusFeature { feature_id: "p1".into(), feature_kind: "position".into() });
        dump("SetLayerStrokeScale", &Gis2dCommand::SetLayerStrokeScale { layer_id: "roads".into(), value: 1.5 });
        dump("SetLocale", &Gis2dCommand::SetLocale { value: "de-DE".into() });
        dump("OpenSource", &Gis2dCommand::OpenSource { feature_id: "p1".into() });

        dump_op("Op.Positions.Add", &GisMapOperation::Positions(CollectionOperation::Add { id: "p1".into(), item: feature(), at: 0 }));
        dump_op("Op.Positions.Remove", &GisMapOperation::Positions(CollectionOperation::Remove { id: "p1".into() }));
        dump_op("Op.Positions.Move", &GisMapOperation::Positions(CollectionOperation::Move { id: "p1".into(), to: 3 }));
        dump_op("Op.Positions.Patch.Some", &GisMapOperation::Positions(CollectionOperation::Patch { id: "p1".into(), patch: gis2d::MapFeaturePatch { data: Some(dsl::to_dsl_value(&serde_json::json!({ "label": "Home" })).unwrap_or(dsl::DslValue::Null)) } }));
        dump_op("Op.Positions.Patch.None", &GisMapOperation::Positions(CollectionOperation::Patch { id: "p1".into(), patch: gis2d::MapFeaturePatch { data: None } }));
        dump_op("Op.Routes.Add", &GisMapOperation::Routes(CollectionOperation::Add { id: "p1".into(), item: feature(), at: 0 }));
        dump_op("Op.Routes.Remove", &GisMapOperation::Routes(CollectionOperation::Remove { id: "p1".into() }));
        dump_op("Op.Routes.Move", &GisMapOperation::Routes(CollectionOperation::Move { id: "p1".into(), to: 1 }));
        dump_op("Op.Routes.Patch", &GisMapOperation::Routes(CollectionOperation::Patch { id: "p1".into(), patch: gis2d::MapFeaturePatch { data: Some(dsl::to_dsl_value(&serde_json::json!({ "kind": "reuse" })).unwrap_or(dsl::DslValue::Null)) } }));
        dump_op("Op.Regions.Add", &GisMapOperation::Regions(CollectionOperation::Add { id: "p1".into(), item: feature(), at: 0 }));
        dump_op("Op.Regions.Remove", &GisMapOperation::Regions(CollectionOperation::Remove { id: "p1".into() }));
        dump_op("Op.Regions.Move", &GisMapOperation::Regions(CollectionOperation::Move { id: "p1".into(), to: 2 }));
        dump_op("Op.Regions.Patch", &GisMapOperation::Regions(CollectionOperation::Patch { id: "p1".into(), patch: gis2d::MapFeaturePatch { data: Some(dsl::to_dsl_value(&serde_json::json!({ "kind": "boundary" })).unwrap_or(dsl::DslValue::Null)) } }));
        dump_op("Op.SetDocument", &GisMapOperation::SetDocument { document: gis2d::GisMapDocument { positions: vec![feature()], routes: Vec::new(), regions: Vec::new() } });
    }
}
//#endregion 🧪️WireBaseline

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use gis2d::{MapFeature, GIS_MAP_SCHEMA};
    use protocol::CollectionOperation;
    use serde_json::json;

    fn sample_patch_feature() -> MapFeature {
        MapFeature { id: "p1".into(), data: dsl::to_dsl_value(&json!({ "id": "p1", "lon": 1.0, "lat": 2.0 })).unwrap_or(dsl::DslValue::Null) }
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = GisMapOperation::Positions(CollectionOperation::Add { id: "p1".into(), item: sample_patch_feature(), at: 0 });
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn gis2d_command_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SetActiveExample { example_id: "reuse-map".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::PatchPositions { positions_json: r#"[{"id":"p1","lon":1.0,"lat":2.0}]"#.into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::PatchRoutes { route_ids: vec!["r1".into()], field: "label".into(), value: "Home".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::PatchRoute { route_id: "r1".into(), field: "label".into(), value: "Home".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SetSelection { ids: vec!["roads".into()] });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::ToggleLayerVisibility { layer_id: "water".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::FitWorld);
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SetCamera { camera_json: r#"{"x":0,"y":0,"zoom":1}"#.into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SetRenderMode { value: "vector".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SetVectorStyle { value: "colored".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SetLodMode { value: "automatic".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SetFeatureSelection { positions: vec!["p1".into()], routes: Vec::new(), mode: "default".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SetHover { hover_json: "null".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SetSelectionMethod { value: "lasso".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SetSelectionMode { value: "additive".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::ClearSelection);
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SelectAll);
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::Deselect { feature_id: "p1".into(), feature_kind: "position".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::FocusFeature { feature_id: "p1".into(), feature_kind: "position".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SetLayerStrokeScale { layer_id: "roads".into(), value: 1.5 });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_line_round_trip(&Gis2dCommand::OpenSource { feature_id: "p1".into() });
    }

    #[test]
    fn gis2d_command_binary_round_trips() {
        let command = Gis2dCommand::SetRenderMode { value: "vector".into() };
        let bytes = command.encode_op().expect("encode");
        assert_eq!(Gis2dCommand::decode_op(&bytes).expect("decode"), command);
    }

    #[test]
    fn gis_map_document_text_round_trips_through_store() {
        let initial = gis2d_engine::empty_gis_map_projection();
        let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis2d-demo", initial, None);
        let mut store = store::DocumentStore::new(envelope);
        store.dispatch(store::DocumentCommand::Apply { operations: vec![GisMapOperation::Positions(CollectionOperation::Add { id: "p1".into(), item: sample_patch_feature(), at: 0 })], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
