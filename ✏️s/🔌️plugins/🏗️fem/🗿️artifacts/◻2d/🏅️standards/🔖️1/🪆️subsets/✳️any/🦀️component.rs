//! 🪆️ Subset root for `s.fem.fem2d@1/*` (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`,
//! `terra-descriptors` packet, following the `terra-fleet-trinity-recipe` recipe —
//! `📓️terra-fleet-trinity-recipe-report.md`). Exports `subset() -> SubsetDeclaration`, assembling the
//! `🧬️schema`/`🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children — `crate::editor::fem2d`/
//! `crate::viewer::fem2d` stay mounted at the plugin's top-level `editor`/`viewer` modules (`🗒️note`/
//! `🖍️draw` recipe §5 gotcha 1), not here. `examples` is read via the plugin-root SHIM path
//! `crate::artifacts::fem2d::examples::demo` — the deep `standards::v1::subsets::any::examples` path
//! does not resolve for this plugin (this crate's own `📦️glue.rs` only mounts `examples` directly
//! under `artifacts::fem2d`, same shape trinity's jack/rewrite hit, not note's).
//!
//! ⚠️ DEVIATION from the `🗒️note`/`🖍️draw` template (documented, not an oversight — same shape
//! trinity's own subset-root files carry): those two call `io: io::io()`, delegating to a same-named
//! `io() -> IoDeclaration` function their migration added to `🚪️io/🦀️component.rs`. fem2d's own
//! `🚪️io/🦀️component.rs` is still on the OLD `ComposerEntry`/`io_registry` channel (six composer
//! rows: `fem2d`, `csv`, `md`, `json`, `stl`, `obj` — see the artifact root's `definition()`), and
//! hand-authoring typed `Deserializer<Fem2dSnapshot>`/`Serializer<Fem2dSnapshot>` impls for all six
//! foreign formats is real, non-trivial migration work outside this packet's descriptor-emission
//! scope. `io_declaration()` below is the same `IoDeclaration` shape, built here instead: `native` is
//! real (reuses `crate::artifacts::fem2d::pilot_languages()`'s already-real grammar/protocol pairs,
//! and a real `store::ArtifactCodec::of::<Fem2dSnapshot, Fem2dMutation>(...)`), but `entries: &[]` —
//! the foreign-format hops stay UNREGISTERED on the new `io_mechanism` channel (an honest gap, not an
//! oversight; `try_build()` still succeeds since an empty batch trivially passes
//! `preflight_io_entries`). Lease-request (mirrors trinity's own): once this artifact's
//! `🚪️io/🦀️component.rs` is migrated to the typed `serializer_entry`/`deserializer_entry` shape,
//! relocate `io_declaration()` there as `io()` (verbatim rename), add the six typed leaves, and swap
//! this file's `io: io_declaration()` back to `io: io::io()` to match the template exactly.

use crate::artifacts::fem2d::standards::v1::subsets::any::schema;
use crate::artifacts::fem2d::{Fem2dMutation, Fem2dSnapshot, FEM2D_DIALECT, FEM_2D_SCHEMA};
use crate::editor::fem2d as editor;
use crate::viewer::fem2d as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, IoDeclaration, LanguagePair, NativeCodecs, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;

fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::artifacts::fem2d::examples::demo::source()]).as_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: OnceLock<Vec<::semio_framework_schema::ArtifactInferenceDescriptor>> = OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::fem2d_artifact_inference_descriptor()]).as_slice()
}

/// 🚪️ See this file's own module doc for why this is not `io::io()`. `pilot_languages()` indices
/// are fixed by that function's own literal `vec![document, op, diff, pack, spr]` order — the same
/// role→slot mapping `🗒️note`'s `io()` uses for its own five-language array.
fn io_declaration() -> IoDeclaration {
    let langs = crate::artifacts::fem2d::pilot_languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::ArtifactCodec::of::<Fem2dSnapshot, Fem2dMutation>(FEM_2D_SCHEMA.to_string()),
        },
        entries: &[],
    }
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset() -> SubsetDeclaration<crate::FemApps> {
    SubsetDeclaration {
        dialect: FEM2D_DIALECT,
        schema: SchemaDeclaration { descriptor: schema::fem2d_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io_declaration(),
        viewer: viewer_surface::<viewer::Fem2dViewer, crate::FemApps>(viewer::create_fem2d_viewer()),
        editor: editor_surface::<editor::Fem2dPlayApp, crate::FemApps>(editor::create_fem2d_app()),
        examples: examples(),
    }
}
