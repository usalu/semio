//! 🎨️ Generation2d editor command — `set-active-example`.

use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;

use semio_s_artifact_procedural_2d_model::empty_generation2d_snapshot;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_plugin::Effect;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

fn example_document(example_id: &str) -> Option<Generation2dSnapshot> {
    if example_id.is_empty() {
        return Some(empty_generation2d_snapshot());
    }
    if example_id == semio_s_artifact_procedural_2d_examples::demo::ID {
        return semio_s_artifact_procedural_2d_io_text::snapshot::parse_dsl(semio_s_artifact_procedural_2d_examples::demo::PRIMARY_TEXT).ok();
    }
    None
}

/// 🧬️ The whole-document replacement that picking an example or importing a file emits: the artifact's load effect, outside undo history and
/// never a mutation row. `store::os_store::text_format::empty_document_spr` (never a minted `create_document_envelope`) keeps the guest off the
/// `terminal shell reached Drop before its app-owned bounded retirement authority detached` trap on this path.
pub fn reset_generation2d_document_effect(document: &Generation2dSnapshot) -> Effect {
    let pack = <Generation2dSnapshot as pack::snapshot::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("generation2d", GENERATION_2D_SCHEMA));
    Effect::LoadDocument { pack, spr }
}

/// 🎨️ Loads the published `demo` document, clears the graph for an empty id, and ignores an unknown id. The document rides the artifact's
/// load effect (outside undo history); the config lane follows with the concrete selection leaf when the loaded document selects differently.
pub fn emit(payload: &SetActiveExample, _doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let Some(target) = example_document(&payload.example_id) else {
        return Ok(Emit::default());
    };
    let effect = crate::set_active_example::reset_generation2d_document_effect(&target);
    let selected = target.generation.selected_generation_id.clone();
    target.retire_cold();
    let config_mutations = if cfg.snapshot.selected_generation_id == selected { Vec::new() } else { vec![Generation2dConfigMutation::SetSelectedGeneration(semio_s_artifact_procedural_2d_editor_config::SetSelectedGenerationSetting { selected_generation_id: selected })] };
    Ok(Emit { effects: vec![effect], config_mutations, ..Default::default() })
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    emit(payload, doc, cfg)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
