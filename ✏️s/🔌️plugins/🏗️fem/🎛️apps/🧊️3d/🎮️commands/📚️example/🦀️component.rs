//! 📚️ FEM 3D app commands — loading a bundled example, replacing the whole document and resetting
//! view-state config back to its default.

use crate::apps::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use crate::artifacts::fem3d::op::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    /// 📚️ `"default"` loads the bundled `.fem3d` fixture; any other id resets to an empty document —
    /// fem3d only ships the one example (mirrors the pre-migration `handle_action` behavior). Also
    /// resets the whole config back to its default (camera, result display) via a `Snapshot`.
    ///
    /// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetSnapshot` — see
    /// `📓️taxonomy.md`'s forbidden vocabulary), so this builds `apps::fem3d::reset_document_effect`
    /// (a `HostEffect::LoadDocument`, outside undo history) instead of an `artifact_mutations` entry.
    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
        let document = if payload.example_id == "default" {
            <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::fem3d::dsl::FEM3D_EXAMPLE_TEXT).unwrap_or_default()
        } else {
            Fem3dSnapshot::default()
        };
        Ok(Emit { effects: vec![crate::apps::fem3d::reset_document_effect(&document)], config_mutations: vec![Fem3dConfigMutation::Snapshot { config: Fem3dConfig::default() }], ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework::kernel::HostEffect;
    use semio_framework_plugin::ActionKind;

    fn empty_view() -> (Fem3dSnapshot, semio_framework_plugin::HistoryView) {
        (Fem3dSnapshot::default(), semio_framework_plugin::HistoryView::empty())
    }

    /// 🧬️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright —
    /// see `📓️taxonomy.md`'s forbidden vocabulary), so this now surfaces as a `HostEffect::LoadDocument`
    /// carrying the replacement document's pack bytes, not an `artifact_mutations` entry — `dispatch`'s
    /// in-process `VcsArtifactApp` never applies `effects` to its own store (that's the real host's
    /// job), so this asserts directly on the `Emit` `import_media`-style rather than through
    /// `app.snapshot()`.
    #[test]
    fn set_active_example_loads_default_fixture_3d() {
        let (snapshot, history) = empty_view();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = Fem3dConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: "default".into() }, &doc, &cfg).expect("handle");
        let HostEffect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <Fem3dSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(!loaded.nodes.is_empty(), "expected the default fixture's nodes");
    }

    /// 🧬️ `setActiveExample` replaces document content via a `HostEffect::LoadDocument`, so it MUST be
    /// declared as a Mutation, not a View/Shell action — the framework's "View/Shell actions must not
    /// emit operations" guard would otherwise reject it.
    #[test]
    fn set_active_example_is_declared_as_operation_3d() {
        let definition = crate::apps::fem3d::create_fem3d_app().definition;
        let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, ActionKind::Mutation), "loading an example emits a document-replace effect, so it is a Mutation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
    }

    #[test]
    fn set_active_example_unknown_id_resets_to_empty_document() {
        let (snapshot, history) = empty_view();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = Fem3dConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: "nonsense".into() }, &doc, &cfg).expect("handle");
        let HostEffect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <Fem3dSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(loaded.nodes.is_empty());
    }
}
// #endregion 🧪️Tests
