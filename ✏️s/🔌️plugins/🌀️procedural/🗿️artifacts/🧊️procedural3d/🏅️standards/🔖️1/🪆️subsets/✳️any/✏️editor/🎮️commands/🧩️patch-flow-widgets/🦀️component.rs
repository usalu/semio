//! 🧩️ 🧩️ Procedural3d play app commands command — `patch-flow-widgets`.

use crate::artifacts::procedural3d::op::{procedural3d_fixture_operations, Procedural3dMutation};
use crate::artifacts::procedural3d::schema::host_from_fixture;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::{FlowEvalSession, Widget};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patch-flow-widgets")]
pub struct PatchFlowWidgets {
    pub widget_ids: Vec<String>,
    pub field: String,
    pub value: Option<f64>,
}

pub async fn handle(payload: &PatchFlowWidgets, doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut host = host_from_fixture(fixture);
    let baseline = host.fixture.clone();
    for widget in host.fixture.widgets.iter_mut() {
        if !payload.widget_ids.contains(&crate::artifacts::procedural3d::widget_id(widget).to_string()) {
            continue;
        }
        if let (Widget::InputSlider { value: slider_value, .. }, Some(new_value)) = (widget, payload.value) {
            if payload.field == "value" {
                *slider_value = new_value;
            }
        }
    }
    Ok(Emit::mutations(procedural3d_fixture_operations(&baseline, &host.fixture)))
}
