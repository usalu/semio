//! 🪆️ Subset root for `s.fem.fem2d@1/*` (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`,
//! `terra-descriptors` packet, following the `terra-fleet-trinity-recipe` recipe —
//! `📓️terra-fleet-trinity-recipe-report.md`). Exports `subset() -> SubsetDeclaration`, assembling the
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children — `crate::editor::fem2d`/
//! `crate::viewer::fem2d` stay mounted at the plugin's top-level `editor`/`viewer` modules (`🗒️note`/
//! `🖍️draw` recipe §5 gotcha 1), not here. `examples` is read via the plugin-root SHIM path
//! `crate::examples::demo` — the deep `standards::v1::subsets::any::examples` path
//! does not resolve for this plugin (this crate's own `🦀️.rs` only mounts `examples` directly
//! under `artifacts::fem2d`, same shape trinity's jack/rewrite hit, not note's).
//!
//! 🚪️ `io: io::io()` matches the `🗒️note`/`🧱️block` template exactly: the local
//! `io_declaration()` this file used to carry (with `entries: &[]` and a DEVIATION note explaining
//! that the six foreign formats stayed unregistered on the `io_mechanism` channel) is gone — ticket
//! 26/09/06/FEM-PLUGIN-END-TO-END, W4 hand-authored the twelve typed
//! `Serializer<Fem2dSnapshot>`/`Deserializer<Fem2dSnapshot>` entries that gap called for and relocated
//! the declaration into `🚪️io/🦀️.rs` as `io()`. See that file's own module doc for the per-format
//! fidelity table.

use crate::standards::v1::subsets::any::{io, schema};
use crate::FEM2D_DIALECT;
use crate::editor::fem2d as editor;
use crate::viewer::fem2d as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::fem2d_artifact_inference_descriptor()]).as_slice()
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset() -> SubsetDeclaration<crate::FemApps> {
    SubsetDeclaration {
        dialect: FEM2D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::fem2d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io::io(),
        viewer: viewer_surface::<viewer::Fem2dViewer, crate::FemApps>(viewer::create_fem2d_viewer()),
        editor: editor_surface::<editor::Fem2dPlayApp, crate::FemApps>(editor::create_fem2d_app()),
        examples: examples(),
    }
}
