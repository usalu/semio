//! 📚️ 📚️ FEM 3D app commands command — `set-active-example`.

use crate::op::Fem3dMutation;
use crate::Fem3dSnapshot;
use crate::editor::fem3d::config::{Fem3dConfig, Fem3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 📚️ `examples::demo::ID` (the id the shell switcher dispatches) loads the bundled `.fem3d` fixture; any other id resets to an empty document —
/// fem3d only ships the one example (mirrors the pre-migration `handle_action` behavior). Also resets
/// the view config back to its defaults, expressed as the two granular field mutations that together
/// cover every `Fem3dConfig` field: `Fem3dConfigPreparationFactory` refuses a whole-config `Snapshot`
/// row on the Config publication lane (unbounded envelope), so a `Snapshot` here would fault this tool
/// at publication now that it runs as a retained job.
///
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetSnapshot` — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so this builds `editor::fem3d::reset_document_effect`
/// (a `Effect::LoadDocument`, outside undo history) instead of an `artifact_mutations` entry.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Fem3dSnapshot>, _cfg: &ConfigView<'_, Fem3dConfig>) -> Result<Emit<Fem3dMutation, Fem3dConfigMutation>, Fault> {
    let document = if payload.example_id == crate::examples::demo::ID { <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(crate::dsl::FEM3D_EXAMPLE_TEXT).unwrap_or_default() } else { Fem3dSnapshot::default() };
    eprintln!("[DEBUG] fem3d setActiveExample id={} nodes={} elements={} solids={}", payload.example_id, document.nodes.len(), document.elements.len(), document.solids.len());
    let defaults = Fem3dConfig::default();
    Ok(Emit {
        effects: vec![crate::editor::fem3d::reset_document_effect(&document)],
        config_mutations: vec![
            Fem3dConfigMutation::SetResultDisplay { source_id: defaults.result_source_id.clone(), mode: defaults.result_mode.clone(), mode_index: defaults.result_mode_index },
            Fem3dConfigMutation::SetCamera { camera: defaults.camera },
        ],
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework::kernel::Effect;
    use semio_framework_plugin::ActionKind;

    fn empty_view() -> (Fem3dSnapshot, semio_framework_plugin::HistoryView) {
        (Fem3dSnapshot::default(), semio_framework_plugin::HistoryView::empty())
    }

    /// 🧬️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright —
    /// see `📓️taxonomy.md`'s forbidden vocabulary), so this now surfaces as a `Effect::LoadDocument`
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
        let emit = handle(&SetActiveExample { example_id: crate::examples::demo::ID.into() }, &doc, &cfg).expect("handle");
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <Fem3dSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(!loaded.nodes.is_empty(), "expected the default fixture's nodes");
    }

    /// 🧬️ `setActiveExample` replaces document content via a `Effect::LoadDocument`, so it MUST be
    /// declared as a Mutation, not a View/Shell action — the framework's "View/Shell actions must not
    /// emit operations" guard would otherwise reject it.
    #[test]
    fn set_active_example_is_declared_as_operation_3d() {
        let definition = crate::editor::fem3d::create_fem3d_app();
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, ActionKind::Mutation), "loading an example emits a document-replace effect, so it is a Mutation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
    }

    /// 📬️ LAW: the config reset must stay granular and must cover every `Fem3dConfig` field.
    /// `Fem3dConfigPreparationFactory::preflight` rejects `Fem3dConfigMutation::Snapshot` on the Config
    /// publication lane, so a whole-config row here would fault `setActiveExample` the moment it
    /// publishes as a retained job (the admissibility half of this law is asserted in the editor's own
    /// `retained_command_fixture_matches_exact_routes_and_value_codec_boundaries` neighbourhood, where
    /// the private factory type is in scope).
    #[test]
    fn set_active_example_resets_config_without_a_whole_snapshot_row() {
        let (snapshot, history) = empty_view();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = Fem3dConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = handle(&SetActiveExample { example_id: crate::examples::demo::ID.into() }, &doc, &cfg).expect("handle");
        assert_eq!(emit.config_mutations.len(), 2);
        assert!(!emit.config_mutations.iter().any(|mutation| matches!(mutation, Fem3dConfigMutation::Snapshot { .. })));
        let mut applied = Fem3dConfig { result_source_id: Some("dead".into()), result_mode: "modal".into(), result_mode_index: 3, camera: crate::FemCamera { json: "{\"x\":9}".into() } };
        for mutation in &emit.config_mutations {
            applied = protocol::Mutation::diff(mutation, &applied).diff().clone();
        }
        assert_eq!(applied, Fem3dConfig::default(), "the two granular rows together restore every config field");
    }

    #[test]
    fn set_active_example_unknown_id_resets_to_empty_document() {
        let (snapshot, history) = empty_view();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = Fem3dConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = handle(&SetActiveExample { example_id: "nonsense".into() }, &doc, &cfg).expect("handle");
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <Fem3dSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(loaded.nodes.is_empty());
    }
}
