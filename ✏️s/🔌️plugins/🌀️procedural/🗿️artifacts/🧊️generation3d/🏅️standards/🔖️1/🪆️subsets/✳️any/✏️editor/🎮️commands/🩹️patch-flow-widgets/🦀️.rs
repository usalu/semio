//! 🧩️ 🧩️ Generation3d play app commands command — `patch-flow-widgets`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::with_host;
use crate::standards::v1::subsets::any::schema::mutations::text::{generation3d_fixture_operations, Generation3dMutation};
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_os_flow::FlowEvalSession;
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
    Ok(Emit::mutations(with_host(fixture, |host| {
        let baseline = host.fixture.clone();
        for widget in host.fixture.widgets.iter_mut() {
            if !payload.widget_ids.contains(&crate::widget_id(widget).to_string()) {
                continue;
            }
            if let (Widget::InputSlider { value: slider_value, .. }, Some(new_value)) = (widget, payload.value) {
                if payload.field == "value" {
                    *slider_value = new_value;
                }
            }
        }
        let operations = generation3d_fixture_operations(&baseline, &host.fixture);
        baseline.retire_cold();
        operations
    })))
}
