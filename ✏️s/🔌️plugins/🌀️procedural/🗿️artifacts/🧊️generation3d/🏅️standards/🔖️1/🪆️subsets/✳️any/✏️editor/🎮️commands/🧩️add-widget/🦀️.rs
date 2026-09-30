//! 🧩️ Typed catalogue creation with deterministic automatic placement.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::standards::v1::subsets::any::schema::{commit_host_snapshot, with_host};
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

pub const AUTOMATIC_GAP: f64 = 48.0;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-widget")]
pub struct AddWidget {
    pub kind: String,
    pub neuron_kind: Option<String>,
    pub format: Option<String>,
    pub action: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

fn creation_fault(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("generation3d.widget.add"), message.into())
}

impl AddWidget {
    /// 🛍️ One descriptor source for catalogue activation and canvas drag.
    pub(crate) fn from_catalogue(item: &semio_framework_os_flow::CatalogueItem) -> Self {
        Self { kind: item.kind.clone(), neuron_kind: item.neuron_kind.clone(), format: item.format.clone(), action: item.action.clone(), x: None, y: None }
    }

    pub(crate) fn descriptor_fields(&self) -> Vec<(&'static str, &str)> {
        let mut fields = vec![("kind", self.kind.as_str())];
        for (name, value) in [("neuronKind", &self.neuron_kind), ("format", &self.format), ("action", &self.action)] {
            if let Some(value) = value { fields.push((name, value.as_str())); }
        }
        fields
    }

    /// 🧬️ Validates the schema's discriminated fields before building the host descriptor.
    pub(crate) fn descriptor_json(&self) -> Result<String, Fault> {
        if !matches!(self.kind.as_str(), "neuron" | "inputSlider" | "inputNote" | "inputImage" | "outputPreview" | "outputExport" | "outputAction" | "variable") {
            return Err(creation_fault(format!("unknown widget kind: {}", self.kind)));
        }
        for (kind, name, value) in [("neuron", "neuronKind", &self.neuron_kind), ("outputExport", "format", &self.format), ("outputAction", "action", &self.action)] {
            if (self.kind == kind) != value.is_some() || value.as_ref().is_some_and(|value| value.is_empty()) {
                return Err(creation_fault(format!("{} requires only its own descriptor fields; invalid {name}", self.kind)));
            }
        }
        if [self.x, self.y].into_iter().flatten().any(|number| !number.is_finite()) {
            return Err(creation_fault("widget coordinates must be finite"));
        }
        Ok(dsl::json::to_json_string(&dsl::DslValue::object(self.descriptor_fields().into_iter().map(|(name, value)| (name.into(), dsl::DslValue::String(value.into()))))))
    }
}

/// 📍️ Sweeps along the unspecified axis using centered rendered rectangles and a visible gap.
fn automatic_position(x: Option<f64>, y: Option<f64>, size: [f64; 2], occupied: &[[f64; 4]]) -> [f64; 2] {
    let mut position = [x.unwrap_or(120.0), y.unwrap_or(120.0)];
    if x.is_some() && y.is_some() { return position; }
    let axis = usize::from(y.is_none());
    for _ in 0..=occupied.len() {
        let next = occupied.iter().filter(|rect| (position[0] - rect[0]).abs() < (size[0] + rect[2]) / 2.0 + AUTOMATIC_GAP && (position[1] - rect[1]).abs() < (size[1] + rect[3]) / 2.0 + AUTOMATIC_GAP)
            .map(|rect| rect[axis] + (size[axis] + rect[axis + 2]) / 2.0 + AUTOMATIC_GAP).fold(position[axis], f64::max);
        if next == position[axis] { break; }
        position[axis] = next;
    }
    position
}

/// 🕹️ Creates one event-sourced widget; rejected descriptors produce a fault and no mutation.
pub fn handle(payload: &AddWidget, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let host_snapshot = &doc.snapshot.host_snapshot;
    let descriptor = payload.descriptor_json()?;
    if let Some(kind) = &payload.neuron_kind {
        if !semio_framework_os_flow::flow_neuron_kind_info_map().contains_key(kind) && !matches!(kind.as_str(), semio_framework_artifact_flow_flow::neural::INPUT_KIND | semio_framework_artifact_flow_flow::neural::OUTPUT_KIND) {
            return Err(creation_fault(format!("unknown neuron kind: {kind}")));
        }
    }
    with_host(host_snapshot, |host| {
        let id = host.add_widget(&descriptor, payload.x.unwrap_or(120.0), payload.y.unwrap_or(120.0)).map_err(|error| creation_fault(error.to_string()))?;
        let node = host.dag.host_snapshot.nodes.iter().find(|node| node.id == id).ok_or_else(|| creation_fault("created widget has no rendered node"))?;
        let occupied: Vec<[f64; 4]> = host.dag.host_snapshot.nodes.iter().filter(|node| node.id != id).map(|node| [node.x, node.y, node.width, node.height]).collect();
        let [x, y] = automatic_position(payload.x, payload.y, [node.width, node.height], &occupied);
        host.move_widget(&id, x, y).map_err(|error| creation_fault(error.to_string()))?;
        Ok(Emit { artifact_mutations: commit_host_snapshot(host_snapshot, &host.host_snapshot), ..Default::default() })
    })
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
