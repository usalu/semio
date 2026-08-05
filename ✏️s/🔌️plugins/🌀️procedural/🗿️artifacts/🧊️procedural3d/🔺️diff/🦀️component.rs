//! 🔺️ Procedural3d artifact — the operation diff (constitutional: diff).

use crate::artifacts::procedural3d::{widget_id, Procedural3dDocument};
use flow_core::{CameraJson, SynapseSpec, Widget, WidgetLayout};
use playbook::{apply_generation_operation, GenerationOperation};
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Collections
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Widget)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SynapsesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, SynapseSpec)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutDiff {
    pub removed: Vec<String>,
    pub set: Vec<(String, WidgetLayout)>,
}

pub(crate) fn apply_widgets_diff(widgets: &mut Vec<Widget>, diff: &WidgetsDiff) {
    for id in &diff.removed {
        widgets.retain(|widget| widget_id(widget) != id);
    }
    for (index, widget) in &diff.set {
        if let Some(pos) = widgets.iter().position(|entry| widget_id(entry) == widget_id(widget)) {
            widgets[pos] = widget.clone();
        } else {
            widgets.insert((*index).min(widgets.len()), widget.clone());
        }
    }
}

pub(crate) fn apply_synapses_diff(synapses: &mut Vec<SynapseSpec>, diff: &SynapsesDiff) {
    for id in &diff.removed {
        synapses.retain(|synapse| synapse.id != *id);
    }
    for (index, synapse) in &diff.set {
        if let Some(pos) = synapses.iter().position(|entry| entry.id == synapse.id) {
            synapses[pos] = synapse.clone();
        } else {
            synapses.insert((*index).min(synapses.len()), synapse.clone());
        }
    }
}
//#endregion 🔖️Collections

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dDiff {
    pub widgets: WidgetsDiff,
    pub synapses: SynapsesDiff,
    pub layout: LayoutDiff,
    pub camera: Option<CameraJson>,
    pub schema: Option<String>,
    #[serde(default)]
    pub generation: Vec<GenerationOperation>,
}

impl OperationDiff<Procedural3dDocument> for Procedural3dDiff {
    fn apply(&self, projection: &Procedural3dDocument) -> Procedural3dDocument {
        let mut next = projection.clone();
        apply_widgets_diff(&mut next.fixture.widgets, &self.widgets);
        apply_synapses_diff(&mut next.fixture.synapses, &self.synapses);
        for id in &self.layout.removed {
            next.fixture.layout.remove(id);
        }
        for (id, layout) in &self.layout.set {
            next.fixture.layout.insert(id.clone(), layout.clone());
        }
        if let Some(camera) = &self.camera {
            next.fixture.camera = camera.clone();
        }
        if let Some(schema) = &self.schema {
            next.fixture.schema = schema.clone();
        }
        for operation in &self.generation {
            apply_generation_operation(&mut next.generation, operation);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        self.widgets.removed.extend(other.widgets.removed);
        self.widgets.set.extend(other.widgets.set);
        self.synapses.removed.extend(other.synapses.removed);
        self.synapses.set.extend(other.synapses.set);
        self.layout.removed.extend(other.layout.removed);
        self.layout.set.extend(other.layout.set);
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        self.generation.extend(other.generation);
    }
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_absorb_merges_collections_and_prefers_incoming_scalars() {
        let mut first = Procedural3dDiff::default();
        first.widgets.removed.push("w-a".into());
        first.widgets.set.push((0, Widget::InputNote { id: "w-b".into(), text: String::new() }));
        first.synapses.removed.push("s-a".into());
        first.layout.removed.push("l-a".into());
        first.camera = Some(CameraJson { x: 1.0, y: 1.0, zoom: 1.0 });
        first.schema = Some("schema-1".into());
        first.generation.push(GenerationOperation::Rename { id: "generation-1".into(), name: "First".into() });

        let mut second = Procedural3dDiff::default();
        second.widgets.removed.push("w-c".into());
        second.synapses.set.push((0, SynapseSpec { id: "s-b".into(), from: "a".into(), to: "b".into(), from_port: "out".into(), to_port: "in".into() }));
        second.layout.set.push(("l-b".into(), WidgetLayout { x: 2.0, y: 2.0 }));
        second.camera = Some(CameraJson { x: 9.0, y: 9.0, zoom: 9.0 });
        second.schema = None;
        second.generation.push(GenerationOperation::Rename { id: "generation-1".into(), name: "Second".into() });

        first.absorb(second);

        assert_eq!(first.widgets.removed, vec!["w-a".to_string(), "w-c".to_string()]);
        assert_eq!(first.widgets.set.len(), 1);
        assert_eq!(first.synapses.removed, vec!["s-a".to_string()]);
        assert_eq!(first.synapses.set.len(), 1);
        assert_eq!(first.layout.removed, vec!["l-a".to_string()]);
        assert_eq!(first.layout.set.len(), 1);
        assert_eq!(first.camera, Some(CameraJson { x: 9.0, y: 9.0, zoom: 9.0 }));
        assert_eq!(first.schema, Some("schema-1".to_string()));
        assert_eq!(first.generation.len(), 2);
    }
}
//#endregion 🧪️Tests
