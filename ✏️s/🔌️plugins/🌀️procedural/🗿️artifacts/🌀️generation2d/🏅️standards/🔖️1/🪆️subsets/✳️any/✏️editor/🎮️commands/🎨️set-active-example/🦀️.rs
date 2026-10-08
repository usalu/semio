//! 🎨️ Generation2d editor command — `set-active-example`.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::Generation2dMutation;

use crate::standards::v1::subsets::any::schema::empty_generation2d_snapshot;
use crate::Generation2dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

fn example_document(example_id: &str) -> Option<Generation2dSnapshot> {
    if example_id.is_empty() {
        return Some(empty_generation2d_snapshot());
    }
    if example_id == crate::examples::demo::ID {
        return crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::examples::demo::PRIMARY_TEXT).ok();
    }
    None
}

/// 🎨️ Loads the published `demo` document, clears the graph for an empty id, and ignores an unknown id. The document rides the artifact's
/// load effect (outside undo history); the config lane follows with the concrete selection leaf when the loaded document selects differently.
pub fn emit(payload: &SetActiveExample, _doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let Some(target) = example_document(&payload.example_id) else {
        return Ok(Emit::default());
    };
    let effect = crate::editor::generation2d::reset_generation2d_document_effect(&target);
    let selected = target.generation.selected_generation_id.clone();
    target.retire_cold();
    let config_mutations = if cfg.snapshot.selected_generation_id == selected { Vec::new() } else { vec![Generation2dConfigMutation::SetSelectedGeneration { selected_generation_id: selected }] };
    Ok(Emit { effects: vec![effect], config_mutations, ..Default::default() })
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    emit(payload, doc, cfg)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
