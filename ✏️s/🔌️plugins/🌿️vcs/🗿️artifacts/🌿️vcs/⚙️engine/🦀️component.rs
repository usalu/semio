//! ⚙️ VCS artifact — headless compute (was: constitutional `engine`).

use crate::artifacts::vcs::{VcsDemoProjection, VCS_DEMO_SCHEMA};

//#region 🔖️DocumentHelpers
pub fn empty_vcs_demo_projection() -> VcsDemoProjection {
    VcsDemoProjection { schema: VCS_DEMO_SCHEMA.into(), title: "VCS Demo".into(), counter: 0, notes: String::new(), status: "new".into(), tags: Vec::new() }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers `VcsDemoProjection`'s pack<->dsl codec under its real `document_schema()` string so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse vcs-play
/// documents without depending on this crate's concrete `Projection`/`Mutation` types. Called by
/// `semio_plugin!`'s `setup:` hook — was the old bundle crate's `register_vcs_exports()`.
pub fn register() {
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::vcs::VcsPlayApp>(VCS_DEMO_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "vcs.document",
        extension: Some("vcsdemo"),
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
        grammar: Some(crate::artifacts::vcs::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::vcs::diff::COMPONENT_GRAMMAR_PATH),
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_projection_matches_schema() {
        let projection = empty_vcs_demo_projection();
        assert_eq!(projection.schema, VCS_DEMO_SCHEMA);
        assert_eq!(projection.status, "new");
    }
}
//#endregion 🧪️Tests

//#region 🔖️ArtifactEngine
pub struct VcsDemoEngine {
    projection: VcsDemoProjection,
}

impl VcsDemoEngine {
    pub fn new(projection: VcsDemoProjection) -> Self {
        Self { projection }
    }
}

impl protocol::ArtifactEngine for VcsDemoEngine {
    type Projection = VcsDemoProjection;
    type Mutation = crate::artifacts::vcs::mutations::VcsDemoMutation;
    type Diff = crate::artifacts::vcs::diff::VcsDemoDiff;

    fn projection(&self) -> &Self::Projection {
        &self.projection
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        crate::artifacts::vcs::mutations::apply_vcs_demo_mutation(&mut self.projection, mutation);
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Projection>>::inverse(mutation, &self.projection)
    }
}
//#endregion 🔖️ArtifactEngine
