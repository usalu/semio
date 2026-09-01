//! 🧩️ 🧩️ Procedural3d play app commands command — `add-widget`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde_json::json;
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
        json!({ "kind": "inputSlider", "label": "" }).to_string()
    } else if let Some((base, neuron)) = payload.kind.split_once('|') {
        if base == "neuron" {
            json!({ "kind": "neuron", "neuronKind": neuron }).to_string()
        } else {
            json!({ "kind": payload.kind }).to_string()
        }
    } else {
        json!({ "kind": payload.kind }).to_string()
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
