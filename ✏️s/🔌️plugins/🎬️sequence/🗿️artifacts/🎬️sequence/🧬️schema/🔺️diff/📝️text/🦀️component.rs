//! 🔺️ Sequence artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::sequence::schema::SequenceArtifact;
use crate::artifacts::sequence::{SequenceEdge, SequenceEdgePatch, SequenceSnapshot, SequenceStep, SequenceStepPatch};
use protocol::{CollectionMutation, MutationDiff, Patchable};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


//#region 🔖️Apply
pub fn apply_steps_delta(items: &[SequenceStep], delta: &SequenceStepsDelta) -> Vec<SequenceStep> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &SequenceStepPatchEntry| {
        (&entry.id, &entry.patch)
    })
}

pub fn apply_edges_delta(items: &[SequenceEdge], delta: &SequenceEdgesDelta) -> Vec<SequenceEdge> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &SequenceEdgePatchEntry| {
        (&entry.id, &entry.patch)
    })
}

fn apply_identified_delta<T, P, E, F>(
    items: &[T],
    removed: &[String],
    added: &[T],
    patched: &[E],
    reordered: Option<&Vec<String>>,
    entry_parts: F,
) -> Vec<T>
where
    T: Clone + protocol::Identified<String> + Patchable<P>,
    P: Clone,
    F: Fn(&E) -> (&String, &P),
{
    let mut next = items.to_vec();
    for id in removed {
        next.retain(|item| item.id() != id);
    }
    for item in added {
        next.push(item.clone());
    }
    for entry in patched {
        let (id, patch) = entry_parts(entry);
        if let Some(item) = next.iter_mut().find(|item| item.id() == id) {
            item.apply_patch(patch);
        }
    }
    if let Some(order) = reordered {
        let mut by_id: std::collections::BTreeMap<_, _> = next.into_iter().map(|item| (item.id().clone(), item)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(item) = by_id.remove(id) {
                ordered.push(item);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

fn absorb_steps_delta(target: &mut Option<SequenceStepsDelta>, incoming: Option<SequenceStepsDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            None => *target = Some(src),
        }
    }
}

fn absorb_edges_delta(target: &mut Option<SequenceEdgesDelta>, incoming: Option<SequenceEdgesDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            None => *target = Some(src),
        }
    }
}

impl SequenceDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &SequenceArtifact) -> SequenceArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.steps {
            next.steps = apply_steps_delta(&next.steps, delta);
        }
        if let Some(delta) = &self.edges {
            next.edges = apply_edges_delta(&next.edges, delta);
        }
        if let Some(list) = &self.selected_step_ids {
            next.selected_step_ids = list.values.clone();
        }
        if let Some(value) = &self.last_run_json {
            next.last_run_json = value.clone();
        }
        if let Some(value) = &self.orientation {
            next.orientation = value.clone();
        }
        if let Some(value) = &self.camera {
            next.camera = value.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        next
    }
}

impl MutationDiff<SequenceSnapshot> for SequenceDiff {
    fn apply(&self, snapshot: &SequenceSnapshot) -> SequenceSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.steps {
            next.steps = apply_steps_delta(&next.steps, delta);
        }
        if let Some(delta) = &self.edges {
            next.edges = apply_edges_delta(&next.edges, delta);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        absorb_steps_delta(&mut self.steps, other.steps);
        absorb_edges_delta(&mut self.edges, other.edges);
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(selected_step_ids);
        take!(last_run_json);
        take!(orientation);
        take!(camera);
        take!(locale);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn steps_delta_from_collection_mutation(
    base: &[SequenceStep],
    op: &CollectionMutation<String, SequenceStep, SequenceStepPatch>,
) -> SequenceStepsDelta {
    match op {
        CollectionMutation::Add { item, .. } => SequenceStepsDelta {
            added: vec![item.clone()],
            ..Default::default()
        },
        CollectionMutation::Remove { id } => SequenceStepsDelta {
            removed: vec![id.clone()],
            ..Default::default()
        },
        CollectionMutation::Patch { id, patch } => SequenceStepsDelta {
            patched: vec![SequenceStepPatchEntry { id: id.clone(), patch: patch.clone() }],
            ..Default::default()
        },
        CollectionMutation::Move { id, to_index } => {
            let mut ids: Vec<String> = base.iter().map(|item| item.id.clone()).collect();
            if let Some(from) = ids.iter().position(|x| x == id) {
                let item = ids.remove(from);
                let to = (*to_index).min(ids.len());
                ids.insert(to, item);
            }
            SequenceStepsDelta {
                reordered: Some(ids),
                ..Default::default()
            }
        }
    }
}

pub fn edges_delta_from_collection_mutation(
    base: &[SequenceEdge],
    op: &CollectionMutation<String, SequenceEdge, SequenceEdgePatch>,
) -> SequenceEdgesDelta {
    match op {
        CollectionMutation::Add { item, .. } => SequenceEdgesDelta {
            added: vec![item.clone()],
            ..Default::default()
        },
        CollectionMutation::Remove { id } => SequenceEdgesDelta {
            removed: vec![id.clone()],
            ..Default::default()
        },
        CollectionMutation::Patch { id, patch } => SequenceEdgesDelta {
            patched: vec![SequenceEdgePatchEntry { id: id.clone(), patch: patch.clone() }],
            ..Default::default()
        },
        CollectionMutation::Move { id, to_index } => {
            let mut ids: Vec<String> = base.iter().map(|item| item.id.clone()).collect();
            if let Some(from) = ids.iter().position(|x| x == id) {
                let item = ids.remove(from);
                let to = (*to_index).min(ids.len());
                ids.insert(to, item);
            }
            SequenceEdgesDelta {
                reordered: Some(ids),
                ..Default::default()
            }
        }
    }
}

/// 🖼️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &SequenceSnapshot) -> SequenceDiff {
    SequenceDiff {
        artifact: Some(Box::new(SequenceArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::mutations::SequenceMutation;
    use crate::artifacts::sequence::{default_snapshot, StepParams};
    use protocol::Mutation;

    #[test]
    fn steps_add_diff_applies_onto_the_base_snapshot() {
        let base = default_snapshot();
        let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
        let operation = SequenceMutation::StepsAdd { index: 2, item: step };
        let diff: SequenceDiff = operation.diff(&base);
        assert!(diff.steps.is_some(), "StepsAdd must produce a steps diff: {diff:?}");
        assert!(diff.edges.is_none(), "StepsAdd must touch only the steps slot: {diff:?}");
        assert_eq!(diff.apply(&base).steps.len(), base.steps.len() + 1);
    }
}
//#endregion 🧪️Tests
