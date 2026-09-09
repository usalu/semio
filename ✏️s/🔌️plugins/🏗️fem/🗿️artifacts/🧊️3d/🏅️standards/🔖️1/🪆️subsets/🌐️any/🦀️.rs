//! 🪆️ Subset root for `s.fem.fem3d@1/*` (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`,
//! `terra-descriptors` packet, following the `terra-fleet-trinity-recipe` recipe —
//! `📓️terra-fleet-trinity-recipe-report.md`). Exports `subset() -> SubsetDeclaration`, assembling the
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children — `crate::editor::fem3d`/
//! `crate::viewer::fem3d` are mounted at the artifact crate's top-level `editor`/`viewer` modules.
//! The artifact root also owns the `crate::examples::demo` source used here.
//!
//! 🚪️ `io: io::io()` matches the `🗒️note`/`🧱️block` template exactly: the local
//! `io_declaration()` this file used to carry (with `entries: &[]` and a DEVIATION note explaining
//! that the six foreign formats stayed unregistered on the `io_mechanism` channel) is gone — ticket
//! 26/09/06/FEM-PLUGIN-END-TO-END, W4 hand-authored the twelve typed
//! `Serializer<Fem3dSnapshot>`/`Deserializer<Fem3dSnapshot>` entries that gap called for and relocated
//! the declaration into `🚪️io/🦀️.rs` as `io()`. See that file's own module doc for the per-format
//! fidelity table.

use crate::editor::fem3d as editor;
use crate::standards::v1::subsets::any::{io, schema};
use crate::viewer::fem3d as viewer;
use crate::FEM3D_DIALECT;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::fem3d_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<PA: crate::ArtifactApps>() -> SubsetDeclaration<PA> {
    SubsetDeclaration {
        dialect: FEM3D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::fem3d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::Fem3dViewer, PA>(viewer::create_fem3d_viewer()),
        editor: editor_surface::<editor::Fem3dPlayApp, PA>(editor::create_fem3d_app()),
        examples: examples(),
    }
}
