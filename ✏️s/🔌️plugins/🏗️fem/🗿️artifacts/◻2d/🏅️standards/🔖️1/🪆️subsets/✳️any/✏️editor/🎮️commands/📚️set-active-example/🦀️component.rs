//! 📚️ 📚️ Fem2d play app commands command — `set-active-example`.

use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::artifacts::fem2d::op::Fem2dMutation;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use store::ArtifactDsl;

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️SetActiveExample
//#endregion 🔖️SetActiveExample

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 📚️ `"default"` loads the bundled example fixture; any other id resets to an empty document.
/// Also resets the whole config back to its default (camera, result display) via a `Snapshot`.
///
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetSnapshot` — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so this builds `editor::fem2d::reset_document_effect`
/// (a `Effect::LoadDocument`, outside undo history) instead of an `artifact_mutations` entry.
pub async fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let document = if payload.example_id == "default" {
        Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).unwrap_or_else(|_| crate::artifacts::fem2d::schema::empty_fem2d_snapshot())
    } else {
        crate::artifacts::fem2d::schema::empty_fem2d_snapshot()
    };
    Ok(Emit { effects: vec![crate::editor::fem2d::reset_document_effect(&document)], config_mutations: vec![Fem2dConfigMutation::Snapshot { config: Fem2dConfig::default() }], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework::kernel::Effect;

    /// 🧬️ Driven directly through `handle` (not `dispatch`, which routes through `VcsArtifactApp` and
    /// never applies `effects` to its own store — that's the real host's job): asserts on the `Emit`
    /// itself, the same shape `commands::set_active_example`'s fem3d sibling tests use.
    #[test]
    async fn set_active_example_loads_default_fixture_2d() {
        let snapshot = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = Fem2dConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = handle(&SetActiveExample { example_id: "default".into() }, &doc, &cfg).expect("handle");
        assert!(emit.artifact_mutations.is_empty());
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <crate::artifacts::fem2d::Fem2dSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(!loaded.nodes.is_empty(), "expected the default fixture's nodes");
    }

    #[test]
    async fn set_active_example_unknown_id_resets_to_empty_document_2d() {
        let snapshot = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = Fem2dConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = handle(&SetActiveExample { example_id: "nonsense".into() }, &doc, &cfg).expect("handle");
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <crate::artifacts::fem2d::Fem2dSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert_eq!(loaded, crate::artifacts::fem2d::schema::empty_fem2d_snapshot());
    }

    /// 🧬️ `setActiveExample` replaces document content via a `Effect::LoadDocument`, so it MUST be
    /// declared as a Mutation, not a View/Shell action — the framework's "View/Shell actions must not
    /// emit operations" guard would otherwise reject it.
    #[test]
    async fn set_active_example_is_declared_as_operation_2d() {
        let definition = crate::editor::fem2d::create_fem2d_app();
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Mutation), "loading an example emits a document-replace effect, so it is a Mutation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
    }
}
//#endregion 🧪️Tests
