//! ⚙️ Playground artifact — headless compute owning a real `PlaygroundArtifact`.

use crate::artifacts::playground::PlaygroundSnapshot;

//#region 🔖️DocumentHelpers
/// 🏗️ Empty default playground snapshot.
pub fn empty_playground_snapshot() -> PlaygroundSnapshot {
    PlaygroundSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers playground codecs and the fifteen handcrafted schema leaves.
pub fn register() {
    register_artifact_schema();
    register_pilot_languages();
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "playground.document",
        extension: Some("playground"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::playground::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::playground::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::playground::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::playground::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("playground.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "playground.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::playground::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::playground::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::playground::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::playground::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("playground.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "playground.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::playground::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::playground::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("playground.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "playground.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::playground::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::playground::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("playground.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "playground.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::playground::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::playground::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("playground.spr"),
    });
}
//#endregion 🔖️Register

//#region 🔖️SchemaRegistry
/// 📌️ Registers the fifteen handcrafted schema leaves for `s.demonstrator.playground`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::playground::schema::playground_artifact_schema_descriptor());
}
//#endregion 🔖️SchemaRegistry

//#region 🔖️ArtifactEngine
/// ⚙️ UI-independent artifact engine — owns the full artifact; `snapshot()` is its persisted subset.
pub struct PlaygroundArtifactEngine {
    artifact: crate::artifacts::playground::schema::PlaygroundArtifact,
    snapshot: PlaygroundSnapshot,
}

impl PlaygroundArtifactEngine {
    /// 🏗️ Seeds the engine from a persisted snapshot.
    pub fn new(snapshot: PlaygroundSnapshot) -> Self {
        let artifact = crate::artifacts::playground::schema::PlaygroundArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    /// 📸️ Consumes the engine and returns its persisted snapshot.
    pub fn into_snapshot(self) -> PlaygroundSnapshot {
        self.snapshot
    }
}

impl protocol::ArtifactEngine for PlaygroundArtifactEngine {
    type Artifact = crate::artifacts::playground::schema::PlaygroundArtifact;
    type Snapshot = PlaygroundSnapshot;
    type Mutation = crate::artifacts::playground::mutations::PlaygroundMutation;
    type Diff = crate::artifacts::playground::diff::PlaygroundDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot);
        self.snapshot = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot);
        self.artifact.set_snapshot(self.snapshot.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot)
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playground::PLAYGROUND_DOCUMENT_SCHEMA;
    use protocol::ArtifactEngine;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_playground_snapshot();
        assert_eq!(snapshot.schema, PLAYGROUND_DOCUMENT_SCHEMA);
    }

    #[test]
    fn engine_owns_real_artifact() {
        let engine = PlaygroundArtifactEngine::new(empty_playground_snapshot());
        assert_eq!(engine.artifact().schema, PLAYGROUND_DOCUMENT_SCHEMA);
        assert_eq!(engine.snapshot().schema, PLAYGROUND_DOCUMENT_SCHEMA);
    }
}
//#endregion 🧪️Tests
