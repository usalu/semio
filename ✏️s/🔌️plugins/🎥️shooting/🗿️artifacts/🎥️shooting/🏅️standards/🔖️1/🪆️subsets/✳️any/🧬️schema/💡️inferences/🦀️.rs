//! 💡️ Shooting inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `🦀️.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧭topology/`).

use crate::ShootingSnapshot;
use schema::ArtifactSchema;

use super::topology::compute_shooting_topology;
//#region 🔖️Inference
/// 💡️ Everything inferable from a shooting snapshot. One field per named inference under
/// `💡️inferences/` (currently: `topology`, backed by the `🧭topology/` slug dir) — the ONLY
/// cross-entity reference this snapshot carries is `ShootingShot.camera_id` into `savedCameras`,
/// so "topology" here is that reference graph recast as a trivial two-level DAG.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.shooting.shooting.inference")]
pub struct ShootingInference {
    #[derived]
    pub topology: ShootingTopology,
}

impl protocol::Inference<ShootingSnapshot> for ShootingInference {
    fn infer(snapshot: &ShootingSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok({
        Self { topology: compute_shooting_topology(snapshot) }
    
        })
    }
}

/// 🌱 Defined in terms of `infer` (not derived) so this stays correct regardless of what
/// `ShootingSnapshot::default()` happens to contain.
impl Default for ShootingInference {
    fn default() -> Self {
        let snapshot = &ShootingSnapshot::default();

        Self { topology: compute_shooting_topology(snapshot) }
    }
}

impl protocol::InferenceSpec<ShootingSnapshot> for ShootingInference {
    fn inference_schema_id() -> &'static str {
        "s.shooting.shooting.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.shooting.shooting.inference.topology", reads: &["shots", "saved_cameras"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ Registers `s.shooting.shooting.inference`'s facet leaves into the OS-wide inference catalog
/// — call once at plugin init, alongside `shooting_artifact_schema_descriptor`'s registration.
pub fn shooting_artifact_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::register_schema_leaf_documents(&[
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.shooting.shooting.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.shooting.shooting.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.shooting.shooting.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.shooting.shooting.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🛰️.proto" },
    ]);
    #[cfg(target_arch = "wasm32")]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, "");
    #[cfg(not(target_arch = "wasm32"))]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, env!("CARGO_MANIFEST_DIR"));
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.shooting.shooting.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" },
    }
}
//#endregion 🔖️Descriptor


//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::topology::ShootingTopology;
//#endregion 🔁️Re-exports
