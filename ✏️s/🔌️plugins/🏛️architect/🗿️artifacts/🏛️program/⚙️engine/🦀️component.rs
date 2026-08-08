//! ⚙️ Architect program artifact engine — headless compute over the program projection.
//!
//! The engine is genuinely multi-topic (the ex-`🦴️spine` crate's ten compute domains), so each topic
//! keeps its own sibling `🦀️<topic>.rs` file and this node is the hub: the plugin-runtime `register()`
//! hook plus a flat re-export of every topic, so `crate::artifacts::program::engine::*` reaches all of
//! them without a caller needing to know which topic file owns a given function.

pub use crate::artifacts::program::engine::adjacency::*;
pub use crate::artifacts::program::engine::analyze::*;
pub use crate::artifacts::program::engine::exchange::*;
pub use crate::artifacts::program::engine::outputs::*;
pub use crate::artifacts::program::engine::report::*;
pub use crate::artifacts::program::engine::search::*;
pub use crate::artifacts::program::engine::status_summary::*;
pub use crate::artifacts::program::engine::template::*;
pub use crate::artifacts::program::engine::trace::*;
pub use crate::artifacts::program::engine::validate::*;

//#region 🔖️Register
/// 🗂️ Registers `Program`'s pack↔dsl codec under `ARCHITECT_PROGRAM_SCHEMA`. Called from the plugin
/// root's `semio_plugin!{ setup: … }`.
pub fn register() {
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::architect::ArchitectPlayApp>(crate::artifacts::program::ARCHITECT_PROGRAM_SCHEMA);
}

/// 🗂️ Plugin setup entry — same as `register`, named for `Plugin::builder(...).setup(...)`.
pub fn register_architect_exports() {
    register();
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "architect.program",
        extension: Some("architect"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::program::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::program::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::program::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::program::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("architect.program"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "architect.program.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::program::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::program::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::program::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::program::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("architect.program.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "architect.program.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::program::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::program::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("architect.program.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "program.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::program::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::program::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("program.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "program.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::program::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::program::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("program.spr"),
    });
}

//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::artifacts::program::sample_plugin;

    /// 🧭️ The hub's flat re-export reaches every topic module — one representative entry point per
    /// topic file, so a dropped `pub use` fails here rather than at some distant call site.
    #[test]
    fn the_hub_re_exports_every_engine_topic() {
        use super::*;
        let program = sample_plugin();
        let _ = undirected_edges(&program);
        let _ = run_analysis(&program, crate::artifacts::program::registers::AnalysisKind::Gap);
        let _ = export_registers_csv(&program);
        let _ = build_report(&program, crate::artifacts::program::registers::ReportKind::ExecutiveSummary);
        let _ = search_plugin(&program, &SearchQuery::default(), None, None);
        let _ = status_summary(&program);
        let _ = audit_trail(&program, None);
        let _ = validate_plugin(&program);
    }
}
//#endregion 🧪️Tests


//#region 🔖️ArtifactEngine
/// @emoji ⚙️ UI-independent program artifact engine — owns the projection; every transition is a mutation.
pub struct ProgramEngine {
    projection: crate::artifacts::program::Program,
}

impl ProgramEngine {
    pub fn new(projection: crate::artifacts::program::Program) -> Self {
        Self { projection }
    }
    pub fn into_projection(self) -> crate::artifacts::program::Program {
        self.projection
    }
}

impl protocol::ArtifactEngine for ProgramEngine {
    type Projection = crate::artifacts::program::Program;
    type Mutation = crate::artifacts::program::mutations::ProgramMutation;
    type Diff = crate::artifacts::program::diff::ProgramDiff;

    fn projection(&self) -> &Self::Projection { &self.projection }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        crate::artifacts::program::mutations::apply_program_mutation(&mut self.projection, mutation);
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Projection>>::inverse(mutation, &self.projection)
    }
}
//#endregion 🔖️ArtifactEngine
