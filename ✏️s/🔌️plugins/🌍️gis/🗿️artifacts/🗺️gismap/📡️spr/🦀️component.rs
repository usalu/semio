//! ⚖️ GIS map artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::gismap::op::GisMapOperation;
use crate::artifacts::gismap::{GisMapDocument, MapFeature, MapFeaturePatch};
use protocol::{CollectionOperation, OpBinary};

//#region 🔖️OpTextMirror
/// ✂️ Local DSL-only mirror of `GisMapOperation` — `protocol::CollectionOperation<K,V,P>` is declared
/// in the `protocol` crate (foreign type), so it cannot itself gain a `dsl::DslField`/`dsl::DslVariants`
/// binding here (orphan rule: neither the trait nor the type is local to this crate). This twin
/// flattens each `Positions|Routes|Regions { collection }` wrapper into its own four keyworded
/// variants — mirroring `process::Process3dOperationDsl`'s identical fix for the same foreign-
/// `CollectionOperation` problem — and converts at the `protocol::OpText` boundary only; `GisMapOperation`
/// itself, and every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum GisMapOperationDsl {
    AddPosition {
        index: usize,
        #[dsl(block)]
        item: MapFeature,
    },
    RemovePosition {
        id: String,
    },
    MovePosition {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    PatchPosition {
        id: String,
        #[dsl(block)]
        patch: MapFeaturePatch,
    },
    AddRoute {
        index: usize,
        #[dsl(block)]
        item: MapFeature,
    },
    RemoveRoute {
        id: String,
    },
    MoveRoute {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    PatchRoute {
        id: String,
        #[dsl(block)]
        patch: MapFeaturePatch,
    },
    AddRegion {
        index: usize,
        #[dsl(block)]
        item: MapFeature,
    },
    RemoveRegion {
        id: String,
    },
    MoveRegion {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    PatchRegion {
        id: String,
        #[dsl(block)]
        patch: MapFeaturePatch,
    },
    SetDocument {
        #[dsl(block)]
        document: GisMapDocument,
    },
}

fn gis_map_operation_to_dsl(operation: &GisMapOperation) -> GisMapOperationDsl {
    match operation {
        GisMapOperation::Positions(CollectionOperation::Add { index: at, item }) => GisMapOperationDsl::AddPosition { index: *at, item: item.clone() },
        GisMapOperation::Positions(CollectionOperation::Remove { id }) => GisMapOperationDsl::RemovePosition { id: id.clone() },
        GisMapOperation::Positions(CollectionOperation::Move { id, to_index: to }) => GisMapOperationDsl::MovePosition { id: id.clone(), to_index: *to },
        GisMapOperation::Positions(CollectionOperation::Patch { id, patch }) => GisMapOperationDsl::PatchPosition { id: id.clone(), patch: patch.clone() },
        GisMapOperation::Routes(CollectionOperation::Add { index: at, item }) => GisMapOperationDsl::AddRoute { index: *at, item: item.clone() },
        GisMapOperation::Routes(CollectionOperation::Remove { id }) => GisMapOperationDsl::RemoveRoute { id: id.clone() },
        GisMapOperation::Routes(CollectionOperation::Move { id, to_index: to }) => GisMapOperationDsl::MoveRoute { id: id.clone(), to_index: *to },
        GisMapOperation::Routes(CollectionOperation::Patch { id, patch }) => GisMapOperationDsl::PatchRoute { id: id.clone(), patch: patch.clone() },
        GisMapOperation::Regions(CollectionOperation::Add { index: at, item }) => GisMapOperationDsl::AddRegion { index: *at, item: item.clone() },
        GisMapOperation::Regions(CollectionOperation::Remove { id }) => GisMapOperationDsl::RemoveRegion { id: id.clone() },
        GisMapOperation::Regions(CollectionOperation::Move { id, to_index: to }) => GisMapOperationDsl::MoveRegion { id: id.clone(), to_index: *to },
        GisMapOperation::Regions(CollectionOperation::Patch { id, patch }) => GisMapOperationDsl::PatchRegion { id: id.clone(), patch: patch.clone() },
        GisMapOperation::SetDocument { document } => GisMapOperationDsl::SetDocument { document: document.clone() },
    }
}

fn gis_map_operation_from_dsl(operation: GisMapOperationDsl) -> GisMapOperation {
    match operation {
        GisMapOperationDsl::AddPosition { index, item } => GisMapOperation::Positions(CollectionOperation::Add { index: index, item }),
        GisMapOperationDsl::RemovePosition { id } => GisMapOperation::Positions(CollectionOperation::Remove { id }),
        GisMapOperationDsl::MovePosition { id, to_index } => GisMapOperation::Positions(CollectionOperation::Move { id, to_index: to_index }),
        GisMapOperationDsl::PatchPosition { id, patch } => GisMapOperation::Positions(CollectionOperation::Patch { id, patch }),
        GisMapOperationDsl::AddRoute { index, item } => GisMapOperation::Routes(CollectionOperation::Add { index: index, item }),
        GisMapOperationDsl::RemoveRoute { id } => GisMapOperation::Routes(CollectionOperation::Remove { id }),
        GisMapOperationDsl::MoveRoute { id, to_index } => GisMapOperation::Routes(CollectionOperation::Move { id, to_index: to_index }),
        GisMapOperationDsl::PatchRoute { id, patch } => GisMapOperation::Routes(CollectionOperation::Patch { id, patch }),
        GisMapOperationDsl::AddRegion { index, item } => GisMapOperation::Regions(CollectionOperation::Add { index: index, item }),
        GisMapOperationDsl::RemoveRegion { id } => GisMapOperation::Regions(CollectionOperation::Remove { id }),
        GisMapOperationDsl::MoveRegion { id, to_index } => GisMapOperation::Regions(CollectionOperation::Move { id, to_index: to_index }),
        GisMapOperationDsl::PatchRegion { id, patch } => GisMapOperation::Regions(CollectionOperation::Patch { id, patch }),
        GisMapOperationDsl::SetDocument { document } => GisMapOperation::SetDocument { document },
    }
}

impl protocol::OpText for GisMapOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(gis_map_operation_from_dsl(<GisMapOperationDsl as protocol::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <GisMapOperationDsl as protocol::OpText>::print_op(&gis_map_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `GisMapOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl OpBinary for GisMapOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        gis_map_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(gis_map_operation_from_dsl(GisMapOperationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpTextMirror

//#region 🔖️Codec
/// 📦️ Encodes a `GisMapOperation` to its binary command form.
pub fn encode_op(operation: &GisMapOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `GisMapOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<GisMapOperation, protocol::ProtocolError> {
    GisMapOperation::decode_op(bytes)
}
//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gismap::engine::{default_document, empty_gis_map_projection};
    use crate::artifacts::gismap::GIS_MAP_SCHEMA;
    use serde_json::json;

    fn dsl_of(value: &serde_json::Value) -> dsl::DslValue {
        dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null)
    }

    fn sample_patch_feature() -> MapFeature {
        MapFeature { id: "p1".into(), data: dsl_of(&json!({ "id": "p1", "lon": 1.0, "lat": 2.0 })) }
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = GisMapOperation::Positions(CollectionOperation::Add { index: 0, item: sample_patch_feature() });
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn gis_map_positions_op_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Positions(CollectionOperation::Add { index: 0, item: sample_patch_feature() }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Positions(CollectionOperation::Remove { id: "p1".into() }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Positions(CollectionOperation::Move { id: "p1".into(), to_index: 3 }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Positions(CollectionOperation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: Some(dsl_of(&json!({ "label": "Home" }))) } }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Positions(CollectionOperation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: None } }));
    }

    #[test]
    fn gis_map_routes_op_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Routes(CollectionOperation::Add { index: 0, item: sample_patch_feature() }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Routes(CollectionOperation::Remove { id: "p1".into() }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Routes(CollectionOperation::Move { id: "p1".into(), to_index: 1 }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Routes(CollectionOperation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: Some(dsl_of(&json!({ "kind": "reuse" }))) } }));
    }

    #[test]
    fn gis_map_regions_op_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Regions(CollectionOperation::Add { index: 0, item: sample_patch_feature() }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Regions(CollectionOperation::Remove { id: "p1".into() }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Regions(CollectionOperation::Move { id: "p1".into(), to_index: 2 }));
        store::test_support::assert_op_line_round_trip(&GisMapOperation::Regions(CollectionOperation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: Some(dsl_of(&json!({ "kind": "boundary" }))) } }));
    }

    #[test]
    fn gis_map_set_document_op_line_round_trips() {
        store::test_support::assert_op_line_round_trip(&GisMapOperation::SetDocument { document: default_document() });
    }

    /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the taxonomy split could have
    /// silently rewritten (unit-ish `Patch` with a `None` option field, and the collection ordinals).
    /// Hex copied verbatim from the pre-migration baseline dump (ticket
    /// `26/08/05/GIS-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`, `🧪️wire-baseline-2d-before.txt`).
    #[test]
    fn operation_rows_keep_their_pre_migration_bytes() {
        let hex = |operation: &GisMapOperation| OpBinary::encode_op(operation).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
        assert_eq!(hex(&GisMapOperation::Positions(CollectionOperation::Remove { id: "p1".into() })), "01010102703101000600");
        assert_eq!(hex(&GisMapOperation::Positions(CollectionOperation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: None } })), "01030102703102000600010e0d00");
        assert_eq!(hex(&GisMapOperation::Routes(CollectionOperation::Move { id: "p1".into(), to_index: 1 })), "01060102703102000600010401");
        assert_eq!(hex(&GisMapOperation::Regions(CollectionOperation::Move { id: "p1".into(), to_index: 2 })), "010a0102703102000600010402");
    }

    #[test]
    fn gis_map_document_text_round_trips_through_store() {
        let initial = empty_gis_map_projection();
        let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis2d-demo", initial, None);
        let mut store = store::DocumentStore::new(envelope);
        store.dispatch(store::DocumentCommand::Apply { operations: vec![GisMapOperation::Positions(CollectionOperation::Add { index: 0, item: sample_patch_feature() })], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
