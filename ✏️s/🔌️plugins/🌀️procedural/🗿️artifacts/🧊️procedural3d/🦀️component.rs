//! 📐️ Procedural3d artifact — snapshot re-exports, widget id helper, and artifact kind.


pub use crate::artifacts::procedural3d::schema::snapshot::Procedural3dSnapshot;
pub use crate::artifacts::procedural3d::schema::mutations::Procedural3dMutation;
pub use crate::artifacts::procedural3d::schema::diff::Procedural3dDiff;

use flow::Widget;
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, };



pub const PROCEDURAL_3D_SCHEMA: &str = "procedural.3d";


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
/// `crate::apps::procedural3d::create_procedural3d_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.procedural".into(),
        name: "3D Procedural".into(),
        source_format: "procedural.3d".into(),
        component_kind: "procedural3d".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Flow },
        schema: "procedural.3d".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
        import_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"],
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
            id: "procedural.procedural3d.document",
            extension: Some("procedural3d"),
            role: dsl::LanguageRole::Document,
            grammar: Some(crate::artifacts::procedural3d::dsl::COMPONENT_GRAMMAR_SEMIO),
            grammar_path: Some(crate::artifacts::procedural3d::dsl::COMPONENT_GRAMMAR_PATH),
            protocol: Some(crate::artifacts::procedural3d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::procedural3d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("procedural.procedural3d.document")},
        dsl::LanguageSpec {
            id: "procedural.procedural3d.op",
            extension: None,
            role: dsl::LanguageRole::Ops,
            grammar: Some(crate::artifacts::procedural3d::op::COMPONENT_GRAMMAR_SEMIO),
            grammar_path: Some(crate::artifacts::procedural3d::op::COMPONENT_GRAMMAR_PATH),
            protocol: Some(crate::artifacts::procedural3d::spr::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::procedural3d::spr::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("procedural.procedural3d.op")},
        dsl::LanguageSpec {
            id: "procedural.procedural3d.diff",
            extension: None,
            role: dsl::LanguageRole::Diff,
            grammar: Some(crate::artifacts::procedural3d::diff::COMPONENT_GRAMMAR_SEMIO),
            grammar_path: Some(crate::artifacts::procedural3d::diff::COMPONENT_GRAMMAR_PATH),
            protocol: None,
            protocol_path: None,
            hooks: dsl::passthrough_hooks("procedural.procedural3d.diff")},
        dsl::LanguageSpec {
            id: "procedural3d.pack",
            extension: None,
            role: dsl::LanguageRole::Pack,
            grammar: None,
            grammar_path: None,
            protocol: Some(crate::artifacts::procedural3d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::procedural3d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("procedural3d.pack")},
        dsl::LanguageSpec {
            id: "procedural3d.spr",
            extension: None,
            role: dsl::LanguageRole::Spr,
            grammar: None,
            grammar_path: None,
            protocol: Some(crate::artifacts::procedural3d::spr::COMPONENT_PROTOCOL_SEMIO),
            protocol_path: Some(crate::artifacts::procedural3d::spr::COMPONENT_PROTOCOL_PATH),
            hooks: dsl::passthrough_hooks("procedural3d.spr")},
    ]).as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`. `register_dwg_mesh_bridge()` (still in the `⚙️engine` file) and
/// `crate::apps::procedural3d::config::schema::register_app_schema()` both stay live via
/// `🌀️procedural/🦀️component.rs`'s own `.setup()` — neither has an `ArtifactDeclaration` field (one
/// app-scope config/presence schema, the same exception note's exemplar documents; the other the
/// genuine DWG-bridge gap `register_dwg_mesh_bridge`'s own doc names).
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
        ("s.procedural3d.schema.artifact", "schema", "s.procedural.procedural3d", &[("schema", "s.procedural.procedural3d")], None),
        ("s.procedural3d.inference.artifact", "inference", "s.procedural.procedural3d.inference", &[("schema", "s.procedural.procedural3d.inference")], None),
        ("s.procedural3d.composer.las", "composer", "s.stdio.las@1.0/*", &[("dialect", "s.stdio.las@1.0/*")], None),
        ("s.procedural3d.composer.ply", "composer", "s.stdio.ply@1.0/*", &[("dialect", "s.stdio.ply@1.0/*")], None),
        ("s.procedural3d.composer.png", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.procedural3d.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.procedural3d.composer.dwg", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.procedural3d.composer.stl", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.procedural3d.composer.gltf", "composer", "s.stdio.gltf@2.0/*", &[("dialect", "s.stdio.gltf@2.0/*")], None),
        ("s.procedural3d.composer.obj", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.procedural3d.grammar.document", "grammar", "procedural.procedural3d.document", &[("grammar", "procedural.procedural3d.document")], None),
        ("s.procedural3d.grammar.op", "grammar", "procedural.procedural3d.op", &[("grammar", "procedural.procedural3d.op")], None),
        ("s.procedural3d.grammar.diff", "grammar", "procedural.procedural3d.diff", &[("grammar", "procedural.procedural3d.diff")], None),
        ("s.procedural3d.grammar.pack", "grammar", "procedural3d.pack", &[("grammar", "procedural3d.pack")], None),
        ("s.procedural3d.grammar.spr", "grammar", "procedural3d.spr", &[("grammar", "procedural3d.spr")], None),
        ("s.procedural3d.codec.document", "codec", "procedural.3d:procedural3d", &[("codec", "procedural.3d"), ("extension", "procedural3d")], None),
        ("s.procedural3d.localization.en", "localization", "3D Procedural", &[], Some(("en", "3D Procedural"))),
        ("s.procedural3d.localization.de", "localization", "3D Prozedural", &[], Some(("de", "3D Prozedural"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.procedural3d")?);
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
        .schema(crate::artifacts::procedural3d::schema::procedural3d_artifact_schema_descriptor())
        .inferences([crate::artifacts::procedural3d::standards::v1::subsets::any::schema::inferences::procedural3d_artifact_inference_descriptor()])
        .composers(crate::artifacts::procedural3d::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<crate::apps::procedural3d::Procedural3dPlayApp>()
        .try_build()
}
//#endregion 🔖️Declaration

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_schema_matches_the_document_schema() {
        assert_eq!(artifact_kind().schema, PROCEDURAL_3D_SCHEMA);
    }

    #[test]
    fn widget_id_covers_all_widget_kinds() {
        let widgets: Vec<Widget> = vec![
            Widget::Neuron { id: "neuron-1".into(), neuron_kind: "math.add".into(), params: Default::default(), input_ports: vec![], output_ports: vec![], preview: true },
            Widget::InputSlider { id: "slider-1".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 },
            Widget::InputNote { id: "note-1".into(), text: String::new() },
            Widget::InputImage { id: "image-1".into(), src: String::new() },
            Widget::Variable { id: "variable-1".into(), name: "x".into(), schema: "number".into() },
            Widget::OutputPreview { id: "preview-1".into(), preview: Default::default(), expanded: Default::default() },
            Widget::OutputAction { id: "action-1".into(), action: "run".into() },
            Widget::OutputExport { id: "export-1".into(), format: "gltf".into() },
            Widget::Cluster { id: "cluster-1".into(), name: "c".into(), tree: Default::default(), flow: Default::default() },
        ];
        let ids: Vec<&str> = widgets.iter().map(widget_id).collect();
        assert_eq!(ids, vec!["neuron-1", "slider-1", "note-1", "image-1", "variable-1", "preview-1", "action-1", "export-1", "cluster-1"]);
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::procedural3d::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("Procedural3dComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
