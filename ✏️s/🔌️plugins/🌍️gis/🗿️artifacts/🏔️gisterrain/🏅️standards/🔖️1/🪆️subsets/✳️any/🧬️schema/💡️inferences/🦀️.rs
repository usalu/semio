//! 💡️ GIS terrain inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📦bounds/`).

use crate::GisTerrainSnapshot;
use ::semio_framework_schema::ArtifactSchema;

use super::bounds::{imported_lon_lat_positions, lon_lat_bounds};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Inference
/// 💡️ Everything inferable from a gisterrain snapshot. Today: the geographic bounding box and
/// position count of the `map:in` overlay decoded from `imported_map` (see
/// `📦bounds/🦀️.rs`). A simple whole-snapshot scalar — no `InferredField` caching, the
/// overlay is small and re-decoding is O(positions).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.gis.gisterrain.inference")]
pub struct GisTerrainInference {
    #[derived]
    pub position_count: usize,
    #[derived]
    pub bounds: Option<GisTerrainBounds>,
}

impl protocol::Inference<GisTerrainSnapshot> for GisTerrainInference {
    fn infer(snapshot: &GisTerrainSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        let positions = imported_lon_lat_positions(snapshot);
        Self { position_count: positions.len(), bounds: lon_lat_bounds(&positions) }
    
        })
    }
}

impl protocol::InferenceSpec<GisTerrainSnapshot> for GisTerrainInference {
    fn inference_schema_id() -> &'static str {
        "s.gis.gisterrain.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.gis.gisterrain.inference.positionCount", reads: &["importedMap"] }, protocol::InferenceFieldSpec { id: "s.gis.gisterrain.inference.bounds", reads: &["importedMap"] }]
    }
}
//#endregion 🔖️Inference


//#region 🔖️Descriptor
/// 💡️ Registers `s.gis.gisterrain.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `gisterrain_artifact_schema_descriptor`'s registration.
pub fn gisterrain_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::register_schema_leaf_documents(&[
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.gis.gisterrain.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.gis.gisterrain.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.gis.gisterrain.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.gis.gisterrain.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🛰️.proto" },
    ]);
    #[cfg(target_arch = "wasm32")]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, "");
    #[cfg(not(target_arch = "wasm32"))]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, env!("CARGO_MANIFEST_DIR"));
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.gis.gisterrain.inference",
        inference: ::semio_framework_schema_registry::FacetLeaves {
            rust: "", typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: ""
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::bounds::GisTerrainBounds;
//#endregion 🔁️Re-exports
