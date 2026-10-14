//! 🪆️ Subset root for `s.fem.fem3d@1/*` (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`,
//! `terra-descriptors` packet, following the `terra-fleet-trinity-recipe` recipe —
//! `📓️terra-fleet-trinity-recipe-report.md`). Exports `subset() -> SubsetDeclaration`, assembling the
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children — the editor and viewer come from
//! `semio-s-artifact-fem-3d-editor` and `semio-s-artifact-fem-3d-viewer`.
//! The bundled examples come from `semio-s-artifact-fem-3d-io`.
//!
//! 🚪️ `io: io::io()` matches the `🗒️note`/`🧱️block` template exactly: the local
//! `io_declaration()` this file used to carry (with `entries: &[]` and a DEVIATION note explaining
//! that the six foreign formats stayed unregistered on the `io_mechanism` channel) is gone — ticket
//! 26/09/06/FEM-PLUGIN-END-TO-END, W4 hand-authored the twelve typed
//! `Serializer<Fem3dSnapshot>`/`Deserializer<Fem3dSnapshot>` entries that gap called for and relocated
//! the declaration into `🚪️io/🦀️.rs` as `io()`. See that file's own module doc for the per-format
//! fidelity table.

use semio_s_artifact_fem_3d_editor::editor::fem3d as editor;
use semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema;
use semio_s_artifact_fem_3d_viewer::viewer::fem3d as viewer;
use semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::FEM3D_DIALECT;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![semio_s_artifact_fem_3d_io::examples::demo::source(), semio_s_artifact_fem_3d_io::examples::concrete_forest::source(), semio_s_artifact_fem_3d_io::examples::house::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema_registry::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![fem3d_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<PA: crate::ArtifactApps>() -> SubsetDeclaration<PA> {
    SubsetDeclaration {
        dialect: FEM3D_DIALECT,
        schema: SchemaDeclaration { descriptor: fem3d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::Fem3dViewer, PA>(viewer::create_fem3d_viewer()),
        editor: editor_surface::<editor::Fem3dPlayApp, PA>(editor::create_fem3d_app()),
        examples: examples(),
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.fem.fem3d` — twenty handcrafted schema leaves.
pub fn fem3d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.fem.fem3d",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️schema/🦀️.rs"), typescript: include_str!("🧬️schema/🟦️.ts"), graphql: include_str!("🧬️schema/🔗️.graphql"), json_schema: include_str!("🧬️schema/🔣️.json"), proto: include_str!("🧬️schema/🛰️.proto")
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️schema/📸️snapshot/🦀️.rs"),
            typescript: include_str!("🧬️schema/📸️snapshot/🟦️.ts"),
            graphql: include_str!("🧬️schema/📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("🧬️schema/📸️snapshot/🔣️.json"),
            proto: include_str!("🧬️schema/📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️schema/🔺️diff/🦀️.rs"),
            typescript: include_str!("🧬️schema/🔺️diff/🟦️.ts"),
            graphql: include_str!("🧬️schema/🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🧬️schema/🔺️diff/🔣️.json"),
            proto: include_str!("🧬️schema/🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️schema/🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️schema/🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️schema/🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️schema/🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️schema/🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️InferenceDescriptor
/// 💡️ Registers `s.fem.fem3d.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `fem3d_artifact_schema_descriptor`'s registration.
pub fn fem3d_artifact_inference_descriptor() -> ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.fem.fem3d.inference",
        inference: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️schema/💡️inferences/🦀️.rs"), typescript: include_str!("🧬️schema/💡️inferences/🟦️.ts"), graphql: include_str!("🧬️schema/💡️inferences/🔗️.graphql"), json_schema: include_str!("🧬️schema/💡️inferences/🔣️.json"), proto: include_str!("🧬️schema/💡️inferences/🛰️.proto")
        },
    }
}
//#endregion 🔖️InferenceDescriptor
