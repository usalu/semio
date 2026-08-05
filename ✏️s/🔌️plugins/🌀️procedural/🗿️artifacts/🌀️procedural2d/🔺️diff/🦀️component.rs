//! 🔺️ Procedural2d artifact — the operation diff (constitutional: diff).

use crate::artifacts::procedural2d::{widget_id, Procedural2dDocument};
use flow_core::{CameraJson, SynapseSpec, Widget, WidgetLayout};
use playbook::{apply_generation_operation, GenerationOperation};
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Collections
/// 🩹️ Sparse id-keyed collection diff — removals plus id-or-index `set`s (replace when the id already
/// exists, else insert at the recorded index). Disjoint `set`s on different ids merge cleanly, which
/// is what lets two backbone peers converge on concurrent edits to different widgets/synapses.
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
/// 🩹️ Sparse procedural-2d diff over the flow fixture's collections plus scalar canvas/schema fields
/// and an ordered list of generation edits applied in sequence.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDiff {
    pub widgets: WidgetsDiff,
    pub synapses: SynapsesDiff,
    pub layout: LayoutDiff,
    pub camera: Option<CameraJson>,
    pub schema: Option<String>,
    #[serde(default)]
    pub generation: Vec<GenerationOperation>,
}

impl OperationDiff<Procedural2dDocument> for Procedural2dDiff {
    fn apply(&self, projection: &Procedural2dDocument) -> Procedural2dDocument {
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
    use crate::artifacts::procedural2d::engine::empty_procedural2d_projection;

    #[test]
    fn diff_absorb_merges_vecs_and_updates_scalars_when_present() {
        let mut diff = Procedural2dDiff { camera: Some(CameraJson { x: 1.0, y: 1.0, zoom: 1.0 }), ..Default::default() };
        diff.widgets.removed.push("w1".into());

        diff.absorb(Procedural2dDiff {
            widgets: WidgetsDiff { removed: vec!["w2".into()], set: vec![(0, Widget::InputNote { id: "note".into(), text: String::new() })] },
            synapses: SynapsesDiff { removed: vec!["s1".into()], set: vec![] },
            layout: LayoutDiff { removed: vec![], set: vec![("l1".into(), WidgetLayout { x: 3.0, y: 4.0 })] },
            camera: Some(CameraJson { x: 9.0, y: 9.0, zoom: 2.0 }),
            schema: Some("flow.fixture".into()),
            generation: vec![GenerationOperation::Remove { id: "g1".into() }],
        });

        assert_eq!(diff.widgets.removed, vec!["w1".to_string(), "w2".to_string()]);
        assert_eq!(diff.widgets.set.len(), 1);
        assert_eq!(diff.synapses.removed, vec!["s1".to_string()]);
        assert_eq!(diff.layout.set.len(), 1);
        assert_eq!(diff.camera, Some(CameraJson { x: 9.0, y: 9.0, zoom: 2.0 }));
        assert_eq!(diff.schema, Some("flow.fixture".to_string()));
        assert_eq!(diff.generation.len(), 1);
    }

    #[test]
    fn diff_absorb_keeps_scalar_when_incoming_is_none() {
        let mut diff = Procedural2dDiff { camera: Some(CameraJson { x: 1.0, y: 2.0, zoom: 1.0 }), schema: Some("flow.fixture".into()), ..Default::default() };
        diff.absorb(Procedural2dDiff::default());
        assert_eq!(diff.camera, Some(CameraJson { x: 1.0, y: 2.0, zoom: 1.0 }));
        assert_eq!(diff.schema, Some("flow.fixture".to_string()));
    }

    #[test]
    fn diff_apply_inserts_new_widget_and_replaces_existing_by_id() {
        let projection = empty_procedural2d_projection();
        let existing_id = widget_id(&projection.fixture.widgets[1]).to_string();
        let diff = Procedural2dDiff {
            widgets: WidgetsDiff { removed: vec![], set: vec![(0, Widget::InputNote { id: existing_id.clone(), text: "replaced".into() }), (999, Widget::InputNote { id: "brand-new".into(), text: "new".into() })] },
            ..Default::default()
        };
        let next = diff.apply(&projection);
        assert_eq!(next.fixture.widgets.len(), projection.fixture.widgets.len() + 1);
        let replaced = next.fixture.widgets.iter().find(|w| widget_id(w) == existing_id.as_str()).expect("replaced widget present");
        assert_eq!(replaced, &Widget::InputNote { id: existing_id, text: "replaced".into() });
        assert_eq!(widget_id(next.fixture.widgets.last().expect("inserted widget")), "brand-new");
    }

    #[test]
    fn diff_apply_updates_camera_and_schema_only_when_present() {
        let projection = empty_procedural2d_projection();
        let untouched = Procedural2dDiff::default().apply(&projection);
        assert_eq!(untouched.fixture.camera, projection.fixture.camera);
        assert_eq!(untouched.fixture.schema, projection.fixture.schema);

        let changed = Procedural2dDiff { camera: Some(CameraJson { x: 5.0, y: 6.0, zoom: 3.0 }), schema: Some("other.schema".into()), ..Default::default() }.apply(&projection);
        assert_eq!(changed.fixture.camera, CameraJson { x: 5.0, y: 6.0, zoom: 3.0 });
        assert_eq!(changed.fixture.schema, "other.schema");
    }
}
//#endregion 🧪️Tests
