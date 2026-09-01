//! 🧬️ 🧬️ Procedural3d play app commands command — `select-generation`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::schema::evaluate_generation_preview;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::playbook::{select_generation, selected_generation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "select-generation")]
pub struct SelectGeneration {
    pub id: String,
}

pub fn handle(payload: &SelectGeneration, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut state = doc.snapshot.generation.as_state().clone();
    state.selected_generation_id = cfg.snapshot.selected_generation_id.clone();
    select_generation(&mut state, &payload.id);
    let generation_preview_text = selected_generation(&state).map(|selected| evaluate_generation_preview(fixture, &selected.values));
    Ok(Emit::config(vec![Procedural3dConfigMutation::SetGeneration { selected_generation_id: state.selected_generation_id.clone(), generation_preview_text }]))
}
