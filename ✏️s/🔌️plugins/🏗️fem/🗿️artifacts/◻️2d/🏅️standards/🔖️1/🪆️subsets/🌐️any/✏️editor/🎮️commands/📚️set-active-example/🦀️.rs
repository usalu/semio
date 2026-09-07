//! 📚️ 📚️ Fem2d play app commands command — `set-active-example`.

use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactDsl;

type Fem2dSnapshot = crate::artifacts::fem2d::Fem2dSnapshot;

//#region 🔖️SetActiveExample
//#endregion 🔖️SetActiveExample

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 📚️ The bundled example's own `ExampleSource` id (`📚️examples/🎬️demo`) loads that fixture; any other
/// id resets to an empty document. The id is not a local literal because the shell's navbar switcher
/// dispatches this action straight from `PluginManifest.examples`, whose entries carry exactly that id.
/// Also resets camera and result display to their defaults as two granular rows — never a whole-config
/// `Snapshot`, which would also wipe the session locale.
///
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetSnapshot` — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so this builds `editor::fem2d::reset_document_effect`
/// (a `Effect::LoadDocument`, outside undo history) instead of an `artifact_mutations` entry.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation>, Fault> {
    let document = if payload.example_id == crate::artifacts::fem2d::examples::demo::ID {
        Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).unwrap_or_else(|_| crate::artifacts::fem2d::schema::empty_fem2d_snapshot())
    } else {
        crate::artifacts::fem2d::schema::empty_fem2d_snapshot()
    };
    eprintln!("[DEBUG] fem2d setActiveExample '{}': loading nodes={} elements={} regions={} loadCases={}", payload.example_id, document.nodes.len(), document.elements.len(), document.regions.len(), document.load_cases.len());
    let defaults = Fem2dConfig::default();
    Ok(Emit {
        effects: vec![crate::editor::fem2d::reset_document_effect(&document)],
        config_mutations: vec![
            Fem2dConfigMutation::SetResultDisplay { source_id: defaults.result_source_id.clone(), mode: defaults.result_mode.clone(), mode_index: defaults.result_mode_index },
            Fem2dConfigMutation::SetCamera { camera: defaults.camera },
        ],
        ..Default::default()
    })
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
    fn set_active_example_loads_the_demo_fixture_2d() {
        let snapshot = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = Fem2dConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = handle(&SetActiveExample { example_id: crate::artifacts::fem2d::examples::demo::ID.into() }, &doc, &cfg).expect("handle");
        assert!(emit.artifact_mutations.is_empty());
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <crate::artifacts::fem2d::Fem2dSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(!loaded.nodes.is_empty(), "expected the default fixture's nodes");
    }

    /// 🗣️ The reset must leave `locale` alone: a German session switching examples stays German.
    #[test]
    fn set_active_example_keeps_the_session_locale_2d() {
        let snapshot = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = Fem2dConfig { locale: "de-DE".into(), ..Fem2dConfig::default() };
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = handle(&SetActiveExample { example_id: crate::artifacts::fem2d::examples::demo::ID.into() }, &doc, &cfg).expect("handle");
        assert_eq!(emit.config_mutations.len(), 2);
        assert!(emit.config_mutations.iter().all(|row| !matches!(row, Fem2dConfigMutation::Snapshot { .. } | Fem2dConfigMutation::SetLocale { .. })));
    }

    #[test]
    fn set_active_example_unknown_id_resets_to_empty_document_2d() {
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
    fn set_active_example_is_declared_as_operation_2d() {
        let definition = crate::editor::fem2d::create_fem2d_app();
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Mutation), "loading an example emits a document-replace effect, so it is a Mutation");
        assert!(!action.args.is_empty(), "the palette stages the example choice via a declared select arg");
    }

    /// 📚️ LAW: the declared select option, the handler's accepted id and the example the subset root
    /// publishes to `PluginManifest.examples` are one and the same string — the shell's navbar
    /// switcher dispatches that id verbatim, so any drift makes the switcher a no-op.
    #[test]
    fn the_declared_example_option_is_the_bundled_example_id_2d() {
        let definition = crate::editor::fem2d::create_fem2d_app();
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "setActiveExample").expect("setActiveExample declared");
        let example = action.args.iter().find(|arg| arg.id == "exampleId").expect("exampleId arg declared");
        match example.control() {
            semio_framework::ActionArgControl::Select { options } => {
                assert_eq!(options.len(), 1);
                assert_eq!(options[0].value, crate::artifacts::fem2d::examples::demo::ID);
            }
            other => panic!("expected a select control for exampleId, got {other:?}"),
        }
        assert_eq!(crate::artifacts::fem2d::examples::demo::source().id(), crate::artifacts::fem2d::examples::demo::ID);
    }
}
//#endregion 🧪️Tests
