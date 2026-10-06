//! 🧬️ Generation3d diff schema — sparse field delta over the artifact.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Generation3dDiff
/// 🧬️ Generation3dDiff facet type.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.procedural.generation3d")]
pub struct Generation3dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Generation3dArtifact>>,
    #[state(artifact)]
    pub host_snapshot: Option<FlowHostSnapshot>,
    #[state(artifact)]
    pub generation: Option<GenerationPlayRoot>,
}
//#endregion 🔖️Generation3dDiff

impl Generation3dDiff {
    /// 🧊️ Explicit cold-only disposal of a detached sparse delta. Every inhabited side carries the
    /// same retirement law the projection does — `host_snapshot.layout` is an `OrderedMap<WidgetLayout>`
    /// whose root must be retired (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`) and
    /// `generation` owns its own ladder — so an owned diff is CLOSED, never dropped.
    pub fn retire_cold(self) {
        let Self { artifact, host_snapshot, generation } = self;
        if let Some(artifact) = artifact {
            let Generation3dArtifact { host_snapshot, generation } = *artifact;
            host_snapshot.retire_cold();
            generation.retire_cold();
        }
        if let Some(host_snapshot) = host_snapshot {
            host_snapshot.retire_cold();
        }
        if let Some(generation) = generation {
            generation.retire_cold();
        }
    }
}

/// 🔺️ A sparse-delta read that CLOSES itself — the ONE shape a test holds an owned
/// [`Generation3dDiff`] in, the diff twin of `Generation3dSnapshotRead`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[cfg(test)]
pub struct Generation3dDiffRead(Option<Generation3dDiff>);

#[cfg(test)]
impl Generation3dDiffRead {
    pub fn new(diff: Generation3dDiff) -> Self {
        Self(Some(diff))
    }
}

#[cfg(test)]
impl std::ops::Deref for Generation3dDiffRead {
    type Target = Generation3dDiff;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("a delta read is inhabited until it is taken or dropped")
    }
}

#[cfg(test)]
impl Drop for Generation3dDiffRead {
    fn drop(&mut self) {
        if let Some(diff) = self.0.take() {
            diff.retire_cold();
        }
    }
}

#[cfg(test)]
impl std::fmt::Debug for Generation3dDiffRead {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&**self, formatter)
    }
}

//#region 🔖️Helpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation3dStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️Helpers

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v1::subsets::any::schema::Generation3dArtifact;
//#endregion 🔁️Re-exports


use crate::widget_id;
use crate::Generation3dSnapshot;
use protocol::MutationDiff;
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_artifact_flow_flow::SynapseSpec;
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_artifact_flow_flow::WidgetLayout;
use semio_framework_artifact_playbook_playbook::apply_generation_mutation;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_artifact_playbook_playbook::GenerationPlayState;

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct WidgetsDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, Widget)>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SynapsesDiff {
    pub removed: Vec<String>,
    pub set: Vec<(usize, SynapseSpec)>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
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

fn apply_layout_diff(layout: &mut semio_framework_artifact_flow_flow::OrderedMap<WidgetLayout>, diff: &LayoutDiff) {
    for id in &diff.removed {
        layout.remove(id);
    }
    for (id, entry) in &diff.set {
        layout.insert(id.clone(), entry.clone());
    }
}

/// 🧩 Applies sparse fixture-collection helpers onto a cloned host_snapshot.
pub fn apply_host_snapshot_helpers(host_snapshot: &FlowHostSnapshot, widgets: &WidgetsDiff, synapses: &SynapsesDiff, layout: &LayoutDiff, camera: Option<&CameraJson>, schema: Option<&str>) -> FlowHostSnapshot {
    let mut next = host_snapshot.clone();
    apply_widgets_diff(&mut next.widgets, widgets);
    apply_synapses_diff(&mut next.synapses, synapses);
    apply_layout_diff(&mut next.layout, layout);
    if let Some(camera) = camera {
        next.camera = camera.clone();
    }
    if let Some(schema) = schema {
        next.schema = schema.to_string();
    }
    next
}

/// 🧩 Applies generation mutations onto a cloned play state.
pub fn apply_generation_helpers(state: &GenerationPlayState, ops: &[GenerationMutation]) -> GenerationPlayState {
    let mut next = state.clone();
    for operation in ops {
        apply_generation_mutation(&mut next, operation);
    }
    next
}

impl Generation3dDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &Generation3dArtifact) -> protocol::MutationApplyResult<Generation3dArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(host_snapshot) = &self.host_snapshot {
                std::mem::replace(&mut next.host_snapshot, host_snapshot.clone()).retire_cold();
            }
            if let Some(generation) = &self.generation {
                std::mem::replace(&mut next.generation, generation.clone()).retire_cold();
            }
            next
        })
    }
}

impl MutationDiff<Generation3dSnapshot> for Generation3dDiff {
    fn apply(&self, snapshot: &Generation3dSnapshot) -> protocol::MutationApplyResult<Generation3dSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(host_snapshot) = &self.host_snapshot {
                std::mem::replace(&mut next.host_snapshot, host_snapshot.clone()).retire_cold();
            }
            if let Some(generation) = &self.generation {
                std::mem::replace(&mut next.generation, generation.clone()).retire_cold();
            }
            next
        })
    }
    /// ➕️ Sequential coalesce. Every side this overwrites is RETIRED, never dropped: an inhabited
    /// `fixture`/`generation` owns an `OrderedMap` root and a generation ladder that reject a bare
    /// drop (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`).
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            std::mem::replace(self, other).retire_cold();
            return;
        }
        let Self { artifact: _, host_snapshot, generation } = other;
        if let Some(replacement) = host_snapshot {
            if let Some(displaced) = self.host_snapshot.replace(replacement) {
                displaced.retire_cold();
            }
        }
        if let Some(generation) = generation {
            if let Some(displaced) = self.generation.replace(generation) {
                displaced.retire_cold();
            }
        }
    }

    /// 🧊️ The generic replay seams (`os_vcs::apply_mutation`, the store's history folds) build a
    /// delta and throw it away; an inhabited `fixture` owns an `OrderedMap` root that aborts the
    /// process on a bare drop, so the contract routes here.
    fn retire_cold(self) {
        Generation3dDiff::retire_cold(self);
    }

    /// 🧊️ Same law for the scratch projections a history fold displaces between steps.
    fn retire_projection(projection: Generation3dSnapshot) {
        projection.retire_cold();
    }
}

/// 🏗️ Whole-fixture field delta after applying sparse collection helpers.
pub fn diff_snapshot_from_helpers(base: &Generation3dSnapshot, widgets: &WidgetsDiff, synapses: &SynapsesDiff, layout: &LayoutDiff, camera: Option<&CameraJson>, schema: Option<&str>) -> Generation3dDiff {
    let host_snapshot = apply_host_snapshot_helpers(&base.host_snapshot, widgets, synapses, layout, camera, schema);
    Generation3dDiff { host_snapshot: Some(host_snapshot), ..Generation3dDiff::default() }
}

/// 🏗️ Generation field delta after applying ordered generation mutations.
pub fn diff_generation_from_ops(base: &Generation3dSnapshot, ops: &[GenerationMutation]) -> Generation3dDiff {
    let generation = apply_generation_helpers(&base.generation, ops);
    Generation3dDiff { generation: Some(generation.into()), ..Generation3dDiff::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
