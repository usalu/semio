//! 🧩️ 🧩️ Procedural2d play app commands command — `add-widget`.

use crate::editor::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::schema::host_from_fixture;
use crate::artifacts::procedural2d::op::{procedural2d_fixture_operations, Procedural2dMutation};
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-widget")]
pub struct AddWidget {
    pub kind: String,
    pub neuron_kind: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>}

/// 🕹️ No longer auto-selects the newly-added widget — no `Emit` channel writes `graph`'s selection
/// directly anymore (the framework owns it exclusively; ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(payload: &AddWidget, doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let descriptor = match payload.kind.as_str() {
        "neuron" => json!({ "kind": "neuron", "neuronKind": payload.neuron_kind.clone().unwrap_or_else(|| "math.add".into()) }).to_string(),
        other => json!({ "kind": other }).to_string()};
    let mut host = host_from_fixture(fixture);
    let baseline = host.fixture.clone();
    if host.add_widget(&descriptor, payload.x.unwrap_or(120.0), payload.y.unwrap_or(120.0)).is_ok() {
        return Ok(Emit { artifact_mutations: procedural2d_fixture_operations(&baseline, &host.fixture), ..Default::default() });
    }
    Ok(Emit::default())
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::testkit::{app, dispatch};
    use crate::editor::procedural2d::Procedural2dCommand;

    #[test]
    fn add_widget_emits_op_and_grows_document() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot").fixture.widgets.len();
        dispatch(&mut app, Procedural2dCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: None, y: None }));
        assert_eq!(app.snapshot().expect("snapshot").fixture.widgets.len(), before + 1);
    }
}
//#endregion 🧪️Tests
