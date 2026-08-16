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
/// DEVIATION (26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): the `⚙️engine` file this
/// `io_registry` (and its `.composers(...)` argument) used to live in is now deleted — the registry
/// moved into `🚪️io/🦀️component.rs` alongside the rest of this artifact's IO surface. The
/// `.composers(...)` argument stays fully qualified to `io::io_registry::entries()` rather than the
/// bare `io_registry::entries()` name: left bare it would resolve to THIS file's own `io_registry`
/// module below, which has a different, incompatible return type (`&'static [&'static ComposerEntry]`,
/// wrapping the owned entries) — not the `&'static [ComposerEntry]` `.composers()` expects.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.procedural2d.schema.artifact", "schema", "s.procedural.procedural2d", &[("schema", "s.procedural.procedural2d")], None),
        ("s.procedural2d.inference.artifact", "inference", "s.procedural.procedural2d.inference", &[("schema", "s.procedural.procedural2d.inference")], None),
        ("s.procedural2d.composer.svg", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.procedural2d.composer.pdf", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.procedural2d.composer.png", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.procedural2d.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.procedural2d.composer.dwg", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.procedural2d.composer.dxf", "composer", "s.stdio.dxf@r12/*", &[("dialect", "s.stdio.dxf@r12/*")], None),
        ("s.procedural2d.grammar.document", "grammar", "procedural.procedural2d.document", &[("grammar", "procedural.procedural2d.document")], None),
        ("s.procedural2d.grammar.op", "grammar", "procedural.procedural2d.op", &[("grammar", "procedural.procedural2d.op")], None),
        ("s.procedural2d.grammar.diff", "grammar", "procedural.procedural2d.diff", &[("grammar", "procedural.procedural2d.diff")], None),
        ("s.procedural2d.grammar.pack", "grammar", "procedural2d.pack", &[("grammar", "procedural2d.pack")], None),
        ("s.procedural2d.grammar.spr", "grammar", "procedural2d.spr", &[("grammar", "procedural2d.spr")], None),
        ("s.procedural2d.codec.document", "codec", "procedural.2d:procedural2d", &[("codec", "procedural.2d"), ("extension", "procedural2d")], None),
        ("s.procedural2d.localization.en", "localization", "2D Procedural", &[], Some(("en", "2D Procedural"))),
        ("s.procedural2d.localization.de", "localization", "2D Prozedural", &[], Some(("de", "2D Prozedural"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.procedural2d")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::procedural2d::schema::procedural2d_artifact_schema_descriptor())
        .inferences([crate::artifacts::procedural2d::standards::v1::subsets::any::schema::inferences::procedural2d_artifact_inference_descriptor()])
        .composers(crate::artifacts::procedural2d::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::procedural2d::Procedural2dPlayApp>()
        .try_build()
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
    use crate::artifacts::procedural2d::standards::v1::subsets::any::io::io_registry as v1;

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
