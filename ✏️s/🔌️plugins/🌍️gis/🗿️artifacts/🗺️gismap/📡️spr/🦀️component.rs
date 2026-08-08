//! ⚖️ GIS map artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::{GisMapDocument, MapFeature, MapFeaturePatch};
use protocol::{CollectionMutation, OpBinary};

//#region 🔖️OpTextMirror
/// ✂️ Local DSL-only mirror of `GisMapMutation` — `protocol::CollectionMutation<K,V,P>` is declared
/// in the `protocol` crate (foreign type), so it cannot itself gain a `dsl::DslField`/`dsl::DslVariants`
/// binding here (orphan rule: neither the trait nor the type is local to this crate). This twin
/// flattens each `Positions|Routes|Regions { collection }` wrapper into its own four keyworded
/// variants — mirroring `process::Process3dOperationDsl`'s identical fix for the same foreign-
/// `CollectionMutation` problem — and converts at the `protocol::OpText` boundary only; `GisMapMutation`
/// itself, and every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
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
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for GisMapOperationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for GisMapOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




fn gis_map_operation_to_dsl(operation: &GisMapMutation) -> GisMapOperationDsl {
    match operation {
        GisMapMutation::Positions(CollectionMutation::Add { index: at, item }) => GisMapOperationDsl::AddPosition { index: *at, item: item.clone() },
        GisMapMutation::Positions(CollectionMutation::Remove { id }) => GisMapOperationDsl::RemovePosition { id: id.clone() },
        GisMapMutation::Positions(CollectionMutation::Move { id, to_index: to }) => GisMapOperationDsl::MovePosition { id: id.clone(), to_index: *to },
        GisMapMutation::Positions(CollectionMutation::Patch { id, patch }) => GisMapOperationDsl::PatchPosition { id: id.clone(), patch: patch.clone() },
        GisMapMutation::Routes(CollectionMutation::Add { index: at, item }) => GisMapOperationDsl::AddRoute { index: *at, item: item.clone() },
        GisMapMutation::Routes(CollectionMutation::Remove { id }) => GisMapOperationDsl::RemoveRoute { id: id.clone() },
        GisMapMutation::Routes(CollectionMutation::Move { id, to_index: to }) => GisMapOperationDsl::MoveRoute { id: id.clone(), to_index: *to },
        GisMapMutation::Routes(CollectionMutation::Patch { id, patch }) => GisMapOperationDsl::PatchRoute { id: id.clone(), patch: patch.clone() },
        GisMapMutation::Regions(CollectionMutation::Add { index: at, item }) => GisMapOperationDsl::AddRegion { index: *at, item: item.clone() },
        GisMapMutation::Regions(CollectionMutation::Remove { id }) => GisMapOperationDsl::RemoveRegion { id: id.clone() },
        GisMapMutation::Regions(CollectionMutation::Move { id, to_index: to }) => GisMapOperationDsl::MoveRegion { id: id.clone(), to_index: *to },
        GisMapMutation::Regions(CollectionMutation::Patch { id, patch }) => GisMapOperationDsl::PatchRegion { id: id.clone(), patch: patch.clone() },
        GisMapMutation::SetDocument { document } => GisMapOperationDsl::SetDocument { document: document.clone() },
    }
}

fn gis_map_operation_from_dsl(operation: GisMapOperationDsl) -> GisMapMutation {
    match operation {
        GisMapOperationDsl::AddPosition { index, item } => GisMapMutation::Positions(CollectionMutation::Add { index: index, item }),
        GisMapOperationDsl::RemovePosition { id } => GisMapMutation::Positions(CollectionMutation::Remove { id }),
        GisMapOperationDsl::MovePosition { id, to_index } => GisMapMutation::Positions(CollectionMutation::Move { id, to_index: to_index }),
        GisMapOperationDsl::PatchPosition { id, patch } => GisMapMutation::Positions(CollectionMutation::Patch { id, patch }),
        GisMapOperationDsl::AddRoute { index, item } => GisMapMutation::Routes(CollectionMutation::Add { index: index, item }),
        GisMapOperationDsl::RemoveRoute { id } => GisMapMutation::Routes(CollectionMutation::Remove { id }),
        GisMapOperationDsl::MoveRoute { id, to_index } => GisMapMutation::Routes(CollectionMutation::Move { id, to_index: to_index }),
        GisMapOperationDsl::PatchRoute { id, patch } => GisMapMutation::Routes(CollectionMutation::Patch { id, patch }),
        GisMapOperationDsl::AddRegion { index, item } => GisMapMutation::Regions(CollectionMutation::Add { index: index, item }),
        GisMapOperationDsl::RemoveRegion { id } => GisMapMutation::Regions(CollectionMutation::Remove { id }),
        GisMapOperationDsl::MoveRegion { id, to_index } => GisMapMutation::Regions(CollectionMutation::Move { id, to_index: to_index }),
        GisMapOperationDsl::PatchRegion { id, patch } => GisMapMutation::Regions(CollectionMutation::Patch { id, patch }),
        GisMapOperationDsl::SetDocument { document } => GisMapMutation::SetDocument { document },
    }
}

impl protocol::OpText for GisMapMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(gis_map_operation_from_dsl(<GisMapOperationDsl as protocol::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <GisMapOperationDsl as protocol::OpText>::print_op(&gis_map_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `GisMapOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl OpBinary for GisMapMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        gis_map_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(gis_map_operation_from_dsl(GisMapOperationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpTextMirror

//#region 🔖️Codec
/// 📦️ Encodes a `GisMapMutation` to its binary command form.
pub fn encode_op(operation: &GisMapMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `GisMapMutation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<GisMapMutation, protocol::ProtocolError> {
    GisMapMutation::decode_op(bytes)
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
        let operation = GisMapMutation::Positions(CollectionMutation::Add { index: 0, item: sample_patch_feature() });
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn gis_map_positions_op_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Positions(CollectionMutation::Add { index: 0, item: sample_patch_feature() }));
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Positions(CollectionMutation::Remove { id: "p1".into() }));
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Positions(CollectionMutation::Move { id: "p1".into(), to_index: 3 }));
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Positions(CollectionMutation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: Some(dsl_of(&json!({ "label": "Home" }))) } }));
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Positions(CollectionMutation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: None } }));
    }

    #[test]
    fn gis_map_routes_op_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Routes(CollectionMutation::Add { index: 0, item: sample_patch_feature() }));
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Routes(CollectionMutation::Remove { id: "p1".into() }));
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Routes(CollectionMutation::Move { id: "p1".into(), to_index: 1 }));
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Routes(CollectionMutation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: Some(dsl_of(&json!({ "kind": "reuse" }))) } }));
    }

    #[test]
    fn gis_map_regions_op_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Regions(CollectionMutation::Add { index: 0, item: sample_patch_feature() }));
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Regions(CollectionMutation::Remove { id: "p1".into() }));
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Regions(CollectionMutation::Move { id: "p1".into(), to_index: 2 }));
        store::test_support::assert_op_line_round_trip(&GisMapMutation::Regions(CollectionMutation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: Some(dsl_of(&json!({ "kind": "boundary" }))) } }));
    }

    #[test]
    fn gis_map_set_document_op_line_round_trips() {
        store::test_support::assert_op_line_round_trip(&GisMapMutation::SetDocument { document: default_document() });
    }

    /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the taxonomy split could have
    /// silently rewritten (unit-ish `Patch` with a `None` option field, and the collection ordinals).
    /// Hex copied verbatim from the pre-migration baseline dump (ticket
    /// `26/08/05/GIS-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`, `🧪️wire-baseline-2d-before.txt`).
    #[test]
    fn operation_rows_keep_their_pre_migration_bytes() {
        let hex = |operation: &GisMapMutation| OpBinary::encode_op(operation).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
        assert_eq!(hex(&GisMapMutation::Positions(CollectionMutation::Remove { id: "p1".into() })), "01010102703101000600");
        assert_eq!(hex(&GisMapMutation::Positions(CollectionMutation::Patch { id: "p1".into(), patch: MapFeaturePatch { data: None } })), "01030102703102000600010e0d00");
        assert_eq!(hex(&GisMapMutation::Routes(CollectionMutation::Move { id: "p1".into(), to_index: 1 })), "01060102703102000600010401");
        assert_eq!(hex(&GisMapMutation::Regions(CollectionMutation::Move { id: "p1".into(), to_index: 2 })), "010a0102703102000600010402");
    }

    #[test]
    fn gis_map_document_text_round_trips_through_store() {
        let initial = empty_gis_map_projection();
        let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis2d-demo", initial, None);
        let mut store = store::DocumentStore::new(envelope);
        store.dispatch(store::DocumentCommand::Apply { mutations: vec![GisMapMutation::Positions(CollectionMutation::Add { index: 0, item: sample_patch_feature() })], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
