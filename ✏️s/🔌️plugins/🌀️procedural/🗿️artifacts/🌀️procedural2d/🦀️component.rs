//! 📏️ Procedural2d artifact — snapshot re-exports, widget id helper, and artifact kind.


pub use crate::artifacts::procedural2d::schema::snapshot::Procedural2dSnapshot;
pub use crate::artifacts::procedural2d::schema::mutations::Procedural2dMutation;
pub use crate::artifacts::procedural2d::schema::diff::Procedural2dDiff;

use flow::Widget;
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};



pub const PROCEDURAL_2D_SCHEMA: &str = "procedural.2d";


//#region 🔖️Helpers
/// 🌡️ A flow widget's stable id, across every widget variant (mirrors flow's private accessor).
pub fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id}
}
//#endregion 🔖️Helpers

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::procedural2d::create_procedural2d_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.procedural".into(),
        name: "2D Procedural".into(),
        source_format: "procedural.2d".into(),
        component_kind: "procedural2d".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Flow },
        schema: "procedural.2d".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention already used below (note's own exemplar
/// pattern). Sole caller is `declaration()` below (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE).
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES.get_or_init(|| vec![
        dsl::LanguageSpec {
            id: "procedural.procedural2d.document",
            extension: Some("procedural2d"),
            role: dsl::LanguageRole::Document,
            grammar: Some(crate::artifacts::procedural2d::dsl::COMPONENT_GRAMMAR_SEMIO),
            grammar_path: Some(crate::artifacts::procedural2d::dsl::COMPONENT_GRAMMAR_PATH),
            protocol: Some(crate::artifacts::procedural2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::procedural2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("procedural.procedural2d.document")},
        dsl::LanguageSpec {
            id: "procedural.procedural2d.op",
            extension: None,
            role: dsl::LanguageRole::Ops,
            grammar: Some(crate::artifacts::procedural2d::op::COMPONENT_GRAMMAR_SEMIO),
            grammar_path: Some(crate::artifacts::procedural2d::op::COMPONENT_GRAMMAR_PATH),
            protocol: Some(crate::artifacts::procedural2d::spr::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::procedural2d::spr::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("procedural.procedural2d.op")},
        dsl::LanguageSpec {
            id: "procedural.procedural2d.diff",
            extension: None,
            role: dsl::LanguageRole::Diff,
            grammar: Some(crate::artifacts::procedural2d::diff::COMPONENT_GRAMMAR_SEMIO),
            grammar_path: Some(crate::artifacts::procedural2d::diff::COMPONENT_GRAMMAR_PATH),
            protocol: None,
            protocol_path: None,
            hooks: dsl::passthrough_hooks("procedural.procedural2d.diff")},
        dsl::LanguageSpec {
            id: "procedural2d.pack",
            extension: None,
            role: dsl::LanguageRole::Pack,
            grammar: None,
            grammar_path: None,
            protocol: Some(crate::artifacts::procedural2d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::procedural2d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("procedural2d.pack")},
        dsl::LanguageSpec {
            id: "procedural2d.spr",
            extension: None,
            role: dsl::LanguageRole::Spr,
            grammar: None,
            grammar_path: None,
            protocol: Some(crate::artifacts::procedural2d::spr::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::procedural2d::spr::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("procedural2d.spr")},
    ]).as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four global registries directly from a plugin
/// `.setup()` callback. `crate::apps::procedural2d::config::schema::register_app_schema()` is the one
/// exception, still called from `🌀️procedural/🦀️component.rs`'s own `.setup()`: it registers the
/// `Procedural2dPlayApp` CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration`
/// deliberately has no field for (see that struct's own doc, and note's exemplar which documents the
/// same exception).
///
/// DEVIATION (26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g1): the `.composers(...)` argument is
/// qualified to `standards::v1::engine::io_registry::entries()` rather than left as the bare
/// `io_registry::entries()` this body used while it still lived in the `⚙️engine` file. Left bare it
/// would now resolve to THIS file's own `io_registry` module below, which has a different, incompatible
/// return type (`&'static [&'static ComposerEntry]`, wrapping the engine's owned entries) — not the
/// `&'static [ComposerEntry]` `.composers()` expects.
pub fn declaration() -> semio_framework_plugin::ArtifactDeclaration {
    semio_framework_plugin::ArtifactDeclaration::builder("s.procedural2d")
        .schema(crate::artifacts::procedural2d::schema::procedural2d_artifact_schema_descriptor())
        .inferences([crate::artifacts::procedural2d::standards::v1::subsets::any::schema::inferences::procedural2d_artifact_inference_descriptor()])
        .composers(crate::artifacts::procedural2d::standards::v1::engine::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::procedural2d::Procedural2dPlayApp>()
        .build()
}
//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_schema_matches_the_document_schema() {
        assert_eq!(artifact_kind().schema, PROCEDURAL_2D_SCHEMA);
    }

    #[test]
    fn widget_id_covers_every_widget_kind() {
        let widgets = vec![
            Widget::Neuron { id: "w-neuron".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: true },
            Widget::InputSlider { id: "w-slider".into(), value: 1.0, min: 0.0, max: 2.0, step: 0.5 },
            Widget::InputNote { id: "w-note".into(), text: String::new() },
            Widget::InputImage { id: "w-image".into(), src: String::new() },
            Widget::Variable { id: "w-variable".into(), name: "value".into(), schema: "dictionary".into() },
            Widget::OutputPreview { id: "w-preview".into(), preview: Default::default(), expanded: Default::default() },
            Widget::OutputAction { id: "w-action".into(), action: String::new() },
            Widget::OutputExport { id: "w-export".into(), format: "svg".into() },
            Widget::Cluster { id: "w-cluster".into(), name: String::new(), tree: Default::default(), flow: Default::default() },
        ];
        for widget in &widgets {
            assert!(!widget_id(widget).is_empty());
        }
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::procedural2d::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Procedural2dComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
