//! 📄️ Layout artifact — the document entity the layout app edits (constitutional: general).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

use semio_s_artifact_layout_layout_model::set_drawing_text;
use semio_s_artifact_layout_layout_model::set_frame_layer;
extern crate semio_framework_schema as schema;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;


use protocol::{Identified, Patchable};
use {semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Constants

//#endregion 🔖️Constants

//#region 🔖️ComposedTypes


//#endregion 🔖️ComposedTypes


//#region 🔖️DropPreview
//#endregion 🔖️DropPreview

//#region 🔖️Types


//#endregion 🔖️Types

#[cfg(test)] use semio_s_artifact_layout_layout_diff::LayoutDiff;

#[cfg(test)] use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;


//#region 🔖️ArtifactKind

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention used by `standards::v1::subsets::any::io::io_registry`.
/// Relocated from `⚙️engine/🦀️.rs` alongside `declaration()` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE) — `declaration()`'s only caller, kept private.
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "layout.document",
                    extension: Some("layout"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_layout_layout_model::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_layout_layout_model::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_layout_layout_model::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_layout_layout_model::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("layout.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "layout.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_layout_layout_mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_layout_layout_mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_layout_layout_mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_layout_layout_mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("layout.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "layout.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_layout_layout_diff::io::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_layout_layout_diff::io::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("layout.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "layout.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_layout_layout_model::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_layout_layout_model::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("layout.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "layout.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_layout_layout_mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_layout_layout_mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("layout.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from
/// a plugin `.setup()` callback, including `crate::io_registry::register()` — a
/// DUPLICATE registration of the exact `standards::v1::subsets::any::io::io_registry::entries()` slice the
/// `.composers(…)` call below now registers once.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.layout.layout.standard.v1", "standard", "1", &[], None),
        ("s.layout.layout.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.layout.layout.schema.artifact", "schema", "s.layout.layout", &[("schema", "s.layout.layout")], None),
        ("s.layout.layout.inference.artifact", "inference", "s.layout.layout.inference", &[("schema", "s.layout.layout.inference")], None),
        // 🐛️ D2-capability-claim-repairs: `io_registry::entries()` registers THREE composer rows, not
        // two — the two below plus `composer_entry_of::<LayoutAnyComposer>()` (`🚪️io/🦀️.rs`),
        // whose `writes` is this artifact's own native dialect (`LAYOUT_DIALECT`, `s.layout@1/*`), the
        // same gap class `🗒️note` hit first (see that file's own `definition()` doc comment).
        ("s.layout.layout.composer.layout", "composer", "s.layout.layout@1/*", &[("dialect", "s.layout.layout@1/*")], None),
        ("s.layout.layout.composer.svg", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.layout.layout.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.layout.layout.composer.png", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.layout.layout.composer.dxf", "composer", "s.stdio.dxf@r12/*", &[("dialect", "s.stdio.dxf@r12/*")], None),
        ("s.layout.layout.composer.dwg", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.layout.layout.grammar.document", "grammar", "layout.document", &[("grammar", "layout.document")], None),
        ("s.layout.layout.grammar.op", "grammar", "layout.op", &[("grammar", "layout.op")], None),
        ("s.layout.layout.grammar.diff", "grammar", "layout.diff", &[("grammar", "layout.diff")], None),
        ("s.layout.layout.grammar.pack", "grammar", "layout.pack", &[("grammar", "layout.pack")], None),
        ("s.layout.layout.grammar.spr", "grammar", "layout.spr", &[("grammar", "layout.spr")], None),
        ("s.layout.layout.codec.document.v1", "codec", "layout.layout:layout", &[("codec", "layout.layout"), ("codec-extension", "13:layout.layout:layout")], None),
        ("s.layout.layout.localization.en", "localization", "Layout", &[], Some(("en", "Layout"))),
        ("s.layout.layout.localization.de", "localization", "Layout", &[], Some(("de", "Layout"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.layout.layout")?);
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
        .schema(semio_s_artifact_layout_layout_descriptor::layout_artifact_schema_descriptor())
        .inferences([semio_s_artifact_layout_layout_model::inferences::layout_artifact_inference_descriptor()])
        .composers(semio_s_artifact_layout_layout_io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_layout_layout_editor::LayoutPlayApp>>()
        .try_build()
}
//#endregion 🔖️ArtifactKind

//#region 🔖️CollectionSupport


//#endregion 🔖️CollectionSupport

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests


// ---- Shims: keep pre-migration module paths resolving for external callers ----


pub use crate::standards::v1::subsets::any::io::{LayoutBuilderFacets, LayoutBuilder, LayoutAnalyzer, LayoutComposer};

