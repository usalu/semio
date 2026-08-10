//! ⚙️ VCS artifact — headless compute (was: constitutional `engine`).

use crate::artifacts::vcs::{VcsSnapshot, VCS_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
pub fn empty_vcs_snapshot() -> VcsSnapshot {
    VcsSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers `VcsSnapshot`'s pack<->dsl codec under its real `document_schema()` string so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse vcs-play
/// documents without depending on this crate's concrete snapshot/mutation types. Called by
/// `semio_plugin!`'s `setup:` hook — was the old bundle crate's `register_vcs_exports()`.
pub fn register() {
    crate::artifacts::vcs::composer::register();

    register_artifact_schema();
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::vcs::VcsPlayApp>(VCS_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.document",
        extension: Some("vcs"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::vcs::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::vcs::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::vcs::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::vcs::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("vcs.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::vcs::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::vcs::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::vcs::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::vcs::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("vcs.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::vcs::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::vcs::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("vcs.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::vcs::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::vcs::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("vcs.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::vcs::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::vcs::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("vcs.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️SchemaRegistry
/// 📌️ Registers the twenty handcrafted schema leaves for `s.vcs.vcs`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::vcs::schema::vcs_artifact_schema_descriptor());
}
//#endregion 🔖️SchemaRegistry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_vcs_snapshot();
        assert_eq!(snapshot.schema, VCS_DOCUMENT_SCHEMA);
        assert_eq!(snapshot.status, "new");
    }
}
//#endregion 🧪️Tests

//#region 🔖️ArtifactEngine
pub struct VcsDemoEngine {
    artifact: crate::artifacts::vcs::schema::VcsArtifact,
    snapshot: VcsSnapshot,
}

impl VcsDemoEngine {
    pub fn new(snapshot: VcsSnapshot) -> Self {
        let artifact = crate::artifacts::vcs::schema::VcsArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }
}
//#endregion 🔖️ArtifactEngine
