//! 📋️ 🧩️ Flow play app commands command — `duplicate-widget`. The one UI-reachable emitter of the
//! `duplicate-widget` COMPOSITE mutation (ticket
//! 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS): unlike every sibling
//! command here, it does NOT go through `host_operations`' stateful-host diffing — the composite is
//! constructed directly and handed to `Emit::mutations`, so its `diff`/`inverse` (folded from its
//! plan by `protocol::fold_plan_diff`/`fold_plan_inverse`) are what the store actually applies.
use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::artifacts::flow::schema::mutations::duplicate_widget::mutation::DuplicateWidget as DuplicateWidgetMutation;
use crate::artifacts::flow::{flow_working_scene, op::FlowMutation, FlowSnapshot, FlowWorkingScene};
use flow::FlowEvalSession;
use protocol::Identified;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct DuplicateWidget {
    pub widget_id: String,
}

pub fn handle(payload: &DuplicateWidget, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let scene = flow_working_scene(doc.snapshot);
    if !scene.widgets.iter().any(|widget| widget.id() == &payload.widget_id) {
        return Ok(Emit::mutations(Vec::new()));
    }
    let new_id = unique_widget_id(&scene, &payload.widget_id);
    let synapse_id = unique_synapse_id(&scene, &payload.widget_id, &new_id);
    Ok(Emit::mutations(vec![FlowMutation::DuplicateWidget(DuplicateWidgetMutation {
        source_id: payload.widget_id.clone(),
        new_id,
        synapse_id,
        from_port: String::new(),
        to_port: String::new(),
    })]))
}

/// 🏷️ Mints `"{source_id}-copy"`, bumping a numeric suffix until the id is free.
fn unique_widget_id(scene: &FlowWorkingScene, source_id: &str) -> String {
    let mut candidate = format!("{source_id}-copy");
    let mut suffix = 2;
    while scene.widgets.iter().any(|widget| widget.id() == &candidate) {
        candidate = format!("{source_id}-copy-{suffix}");
        suffix += 1;
    }
    candidate
}

/// 🏷️ Mints `"{from}-to-{to}"`, bumping a numeric suffix until the id is free.
fn unique_synapse_id(scene: &FlowWorkingScene, from: &str, to: &str) -> String {
    let mut candidate = format!("{from}-to-{to}");
    let mut suffix = 2;
    while scene.synapses.iter().any(|synapse| synapse.id == candidate) {
        candidate = format!("{from}-to-{to}-{suffix}");
        suffix += 1;
    }
    candidate
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn duplicate_widget_grows_widgets_and_synapses_and_leaves_the_source_untouched() {
        let mut app = flow_app();
        let before_widgets = app.snapshot().expect("snapshot").to_fixture().widgets.len();
        let before_synapses = app.snapshot().expect("snapshot").to_fixture().synapses.len();

        let result = dispatch(&mut app, FlowCommand::DuplicateWidget(DuplicateWidget { widget_id: "slider".into() }));
        assert!(!result.mutations.is_empty(), "duplicateWidget must emit operations");

        let after = app.snapshot().expect("snapshot").to_fixture();
        assert_eq!(after.widgets.len(), before_widgets + 1);
        assert_eq!(after.synapses.len(), before_synapses + 1);
        assert!(after.widgets.iter().any(|widget| widget.id() == "slider"), "source widget must survive");
        assert!(after.widgets.iter().any(|widget| widget.id() == "slider-copy"), "copy must land at the deterministic id");
        assert!(after.synapses.iter().any(|synapse| synapse.from == "slider" && synapse.to == "slider-copy"), "copy must be wired to its source");
    }

    #[test]
    fn duplicate_widget_of_an_unknown_id_is_a_no_operation() {
        let mut app = flow_app();
        let before = app.snapshot().expect("snapshot").to_fixture().widgets.len();
        let result = dispatch(&mut app, FlowCommand::DuplicateWidget(DuplicateWidget { widget_id: "does-not-exist".into() }));
        assert!(result.mutations.is_empty(), "duplicating an unknown widget must be a no-op");
        assert_eq!(app.snapshot().expect("snapshot").to_fixture().widgets.len(), before);
    }

    #[test]
    fn duplicate_widget_twice_mints_distinct_ids() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::DuplicateWidget(DuplicateWidget { widget_id: "slider".into() }));
        dispatch(&mut app, FlowCommand::DuplicateWidget(DuplicateWidget { widget_id: "slider".into() }));
        let after = app.snapshot().expect("snapshot").to_fixture();
        assert!(after.widgets.iter().any(|widget| widget.id() == "slider-copy"));
        assert!(after.widgets.iter().any(|widget| widget.id() == "slider-copy-2"));
    }
}
//#endregion 🧪️Tests
