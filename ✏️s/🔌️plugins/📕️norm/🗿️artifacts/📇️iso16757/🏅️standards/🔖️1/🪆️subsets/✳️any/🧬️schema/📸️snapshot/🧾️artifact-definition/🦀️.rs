//! 🧾️ Artifact definition of the ISO 16757 snapshot: the language-neutral package declaration, the artifact kind, the dialect coordinate, the document schema and the capability rows of the definition. See [`super`].

use semio_s_plugin_norm_app_surface as app_surface;

/// 📜 Language-neutral package declaration owned by this artifact.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("../../../../../../../📜️artifact-definition.json");

/// 📦 Validates this artifact's independently compiled package identity.
pub fn package_descriptor() -> Result<semio_s_artifact_norm_contract::NormArtifactPackage, semio_s_artifact_norm_contract::PackageSchemaError> {
    semio_s_artifact_norm_contract::package_from_schema(ARTIFACT_DEFINITION_SCHEMA)
}

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("iso16757", "ISO 16757", ISO16757_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const ISO16757_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.iso16757", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const ISO16757_DOCUMENT_SCHEMA: &str = "semio.norm.iso16757/v1";

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    package_descriptor().map_err(|error| semio_framework_plugin::ArtifactDefinitionError::new("artifact.package-schema", error.to_string()))?;
    use semio_s_artifact_norm_contract::definition::{CapabilitySpec as C, ClaimSpec as Q, LocalizationSpec as L};
    const S: &[Q] = &[Q { namespace: "schema", value: "s.norm.iso16757" }];
    const I: &[Q] = &[Q { namespace: "schema", value: "s.norm.iso16757.inference" }];
    const M: &[Q] = &[Q { namespace: "dialect", value: "s.norm.iso16757@1/*" }];
    const K: &[Q] = &[Q { namespace: "codec", value: "semio.norm.iso16757/v1" }, Q { namespace: "codec-extension", value: "22:semio.norm.iso16757/v1:iso16757" }];
    const EN: &[L] = &[L { locale: "en", text: "ISO 16757 building-services product catalogue exchange" }];
    const DE: &[L] = &[L { locale: "de", text: "ISO 16757 Austausch von Produktdaten der technischen Gebäudeausrüstung" }];
    const ROWS: &[C] = &[
        C { identity: "s.norm.iso16757.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        C { identity: "s.norm.iso16757.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        C { identity: "s.norm.iso16757.schema.artifact", kind: "schema", descriptor: "s.norm.iso16757", claims: S, localizations: &[] },
        C { identity: "s.norm.iso16757.inference.outline", kind: "inference", descriptor: "s.norm.iso16757.inference", claims: I, localizations: &[] },
        C { identity: "s.norm.iso16757.composer.any", kind: "composer", descriptor: "s.norm.iso16757@1/*", claims: M, localizations: &[] },
        C { identity: "s.norm.iso16757.grammar.document", kind: "grammar", descriptor: "iso16757.document", claims: &[Q { namespace: "grammar", value: "iso16757.document" }], localizations: &[] },
        C { identity: "s.norm.iso16757.grammar.op", kind: "grammar", descriptor: "iso16757.op", claims: &[Q { namespace: "grammar", value: "iso16757.op" }], localizations: &[] },
        C { identity: "s.norm.iso16757.grammar.diff", kind: "grammar", descriptor: "iso16757.diff", claims: &[Q { namespace: "grammar", value: "iso16757.diff" }], localizations: &[] },
        C { identity: "s.norm.iso16757.grammar.pack", kind: "grammar", descriptor: "iso16757.pack", claims: &[Q { namespace: "grammar", value: "iso16757.pack" }], localizations: &[] },
        C { identity: "s.norm.iso16757.grammar.spr", kind: "grammar", descriptor: "iso16757.spr", claims: &[Q { namespace: "grammar", value: "iso16757.spr" }], localizations: &[] },
        C { identity: "s.norm.iso16757.codec.document.v1", kind: "codec", descriptor: "semio.norm.iso16757/v1:iso16757", claims: K, localizations: &[] },
        C { identity: "s.norm.iso16757.localization.en", kind: "localization", descriptor: "ISO 16757 building-services product catalogue exchange", claims: &[], localizations: EN },
        C { identity: "s.norm.iso16757.localization.de", kind: "localization", descriptor: "ISO 16757 Austausch von Produktdaten der technischen Gebäudeausrüstung", claims: &[], localizations: DE },
    ];
    semio_s_artifact_norm_contract::definition::assemble_definition("s.norm.iso16757", ROWS)
}
