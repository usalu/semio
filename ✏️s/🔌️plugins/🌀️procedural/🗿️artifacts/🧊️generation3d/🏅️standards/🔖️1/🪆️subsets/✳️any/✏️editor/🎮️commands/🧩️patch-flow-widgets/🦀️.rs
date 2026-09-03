//! 🧩️ 🧩️ Generation3d play app commands command — `patch-flow-widgets`.

use crate::artifacts::generation3d::op::{generation3d_fixture_operations, Generation3dMutation};
use crate::artifacts::generation3d::schema::host_from_fixture;
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::{FlowEvalSession, Widget};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-flow-widgets")]
pub struct PatchFlowWidgets {
    pub widget_ids: Vec<String>,
    pub field: String,
    pub value: Option<f64>,
}

pub fn handle(payload: &PatchFlowWidgets, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut host = host_from_fixture(fixture);
    let baseline = host.fixture.clone();
    for widget in host.fixture.widgets.iter_mut() {
        if !payload.widget_ids.contains(&crate::artifacts::generation3d::widget_id(widget).to_string()) {
            continue;
        }
        if let (Widget::InputSlider { value: slider_value, .. }, Some(new_value)) = (widget, payload.value) {
            if payload.field == "value" {
                *slider_value = new_value;
            }
        }
    }
    Ok(Emit::mutations(generation3d_fixture_operations(&baseline, &host.fixture)))
}
