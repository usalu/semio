//! 🧩️ 🧩️ Procedural3d play app commands command — `add-widget`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-widget")]
pub struct AddWidget {
    pub kind: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

/// 🕹️ No longer auto-selects the newly-added widget — no `Emit` channel writes `graph`'s selection
/// directly anymore (the framework owns it exclusively; ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(payload: &AddWidget, doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let descriptor = if payload.kind == "inputSlider" {
        dsl::json::to_json_string(&dsl::DslValue::object([("kind".to_string(), dsl::DslValue::String("inputSlider".into())), ("label".to_string(), dsl::DslValue::String(String::new()))]))
    } else if let Some((base, neuron)) = payload.kind.split_once('|') {
        if base == "neuron" {
            dsl::json::to_json_string(&dsl::DslValue::object([("kind".to_string(), dsl::DslValue::String("neuron".into())), ("neuronKind".to_string(), dsl::DslValue::String(neuron.into()))]))
        } else {
            dsl::json::to_json_string(&dsl::DslValue::object([("kind".to_string(), dsl::DslValue::String(payload.kind.clone()))]))
        }
    } else {
        dsl::json::to_json_string(&dsl::DslValue::object([("kind".to_string(), dsl::DslValue::String(payload.kind.clone()))]))
    };
    let x = payload.x.unwrap_or(120.0);
    let y = payload.y.unwrap_or(120.0);
    let mut host = host_from_fixture(fixture);
    if host.add_widget(&descriptor, x, y).is_ok() {
        let operations = commit_fixture(fixture, &host.fixture);
        Ok(Emit { artifact_mutations: operations, ..Default::default() })
    } else {
        Ok(Emit::default())
    }
}
