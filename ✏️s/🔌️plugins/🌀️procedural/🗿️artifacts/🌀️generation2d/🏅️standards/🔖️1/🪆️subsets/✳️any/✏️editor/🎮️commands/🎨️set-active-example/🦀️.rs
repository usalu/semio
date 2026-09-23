//! 🎨️ Generation2d editor command — `set-active-example`.

use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::{generation2d_host_snapshot_operations, generation_mutation_to_generation2d, Generation2dMutation};
use crate::standards::v1::subsets::any::schema::empty_generation2d_snapshot;
use crate::Generation2dSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

fn example_document(example_id: &str) -> Option<Generation2dSnapshot> {
    if example_id.is_empty() {
        return Some(empty_generation2d_snapshot());
    }
    if example_id == crate::examples::demo::ID {
        return crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(crate::examples::demo::PRIMARY_TEXT).ok();
    }
    None
}

/// 🎨️ Loads the published `demo` document, clears the graph for an empty id, and ignores an unknown id.
pub fn emit(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    let Some(target) = example_document(&payload.example_id) else {
        return Ok(Emit::default());
    };
    let mut operations: Vec<Generation2dMutation> = doc.snapshot.generation.generations.iter().map(|generation| generation_mutation_to_generation2d(GenerationMutation::Remove { id: generation.id.clone() })).collect();
    operations.extend(generation2d_host_snapshot_operations(&doc.snapshot.host_snapshot, &target.host_snapshot));
    let config = Generation2dConfig { show_mode: cfg.snapshot.show_mode.clone(), selected_generation_id: None };
    target.retire_cold();
    Ok(Emit { artifact_mutations: operations, config_mutations: vec![Generation2dConfigMutation::Snapshot { config }], ..Default::default() })
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    emit(payload, doc, cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::HistoryView;

    #[test]
    fn set_active_example_loads_the_demo_document() {
        let empty = empty_generation2d_snapshot();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&empty, &history);
        let config = Generation2dConfig::default();
        let cfg = ConfigView { snapshot: &config, window: None };
        let emitted = emit(&SetActiveExample { example_id: crate::examples::demo::ID.into() }, &doc, &cfg).expect("demo loads");
        assert!(!emitted.artifact_mutations.is_empty(), "demo must change the empty document");
    }
}
