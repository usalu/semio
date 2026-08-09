//! 🔺️ Animate present artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::present::schema::PresentArtifact;
use crate::artifacts::present::{FigureTileDraft, FigureTileDraftPatch, PresentSnapshot};
use protocol::{CollectionMutation, MutationDiff, Patchable};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

//#region 🔖️Apply
pub fn apply_tiles_delta(items: &[FigureTileDraft], delta: &PresentTilesDelta) -> Vec<FigureTileDraft> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &PresentTilePatchEntry| {
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

fn absorb_tiles_delta(target: &mut Option<PresentTilesDelta>, incoming: Option<PresentTilesDelta>) {
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

impl PresentDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &PresentArtifact) -> PresentArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(source) = &self.source {
            next.source = source.clone();
        }
        if let Some(delta) = &self.tiles {
            next.tiles = apply_tiles_delta(&next.tiles, delta);
        }
        if let Some(list) = &self.selected_ids {
            next.selected_ids = list.values.clone();
        }
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        next
    }
}

impl MutationDiff<PresentSnapshot> for PresentDiff {
    fn apply(&self, snapshot: &PresentSnapshot) -> PresentSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(source) = &self.source {
            next.source = source.clone();
        }
        if let Some(delta) = &self.tiles {
            next.tiles = apply_tiles_delta(&next.tiles, delta);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        absorb_tiles_delta(&mut self.tiles, other.tiles);
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(source);
        take!(selected_ids);
        take!(engagement_input);
        take!(locale);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn tiles_delta_from_collection_mutation(
    base: &[FigureTileDraft],
    op: &CollectionMutation<String, FigureTileDraft, FigureTileDraftPatch>,
) -> PresentTilesDelta {
    match op {
        CollectionMutation::Add { item, .. } => PresentTilesDelta {
            added: vec![item.clone()],
            ..Default::default()
        },
        CollectionMutation::Remove { id } => PresentTilesDelta {
            removed: vec![id.clone()],
            ..Default::default()
        },
        CollectionMutation::Patch { id, patch } => PresentTilesDelta {
            patched: vec![PresentTilePatchEntry { id: id.clone(), patch: patch.clone() }],
            ..Default::default()
        },
        CollectionMutation::Move { id, to_index } => {
            let mut ids: Vec<String> = base.iter().map(|item| item.id.clone()).collect();
            if let Some(from) = ids.iter().position(|x| x == id) {
                let item = ids.remove(from);
                let to = (*to_index).min(ids.len());
                ids.insert(to, item);
            }
            PresentTilesDelta {
                reordered: Some(ids),
                ..Default::default()
            }
        }
    }
}

pub fn tiles_delta_from_set_tiles(base: &[FigureTileDraft], tiles: &[FigureTileDraft]) -> PresentTilesDelta {
    PresentTilesDelta {
        removed: base.iter().map(|t| t.id.clone()).collect(),
        added: tiles.to_vec(),
        ..Default::default()
    }
}

pub fn diff_set_snapshot(snapshot: &PresentSnapshot) -> PresentDiff {
    PresentDiff {
        artifact: Some(Box::new(PresentArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::present::default_present_snapshot;
    use crate::artifacts::present::op::PresentMutation;
    use protocol::Mutation;

    #[test]
    fn set_source_diff_applies_onto_the_base_snapshot() {
        let base = default_present_snapshot();
        let mut next_source = base.source.clone();
        next_source.kind = "video".into();
        let operation = PresentMutation::SetSource { source: next_source.clone() };
        let diff: PresentDiff = operation.diff(&base);
        assert_eq!(diff.source, Some(next_source));
        assert!(diff.artifact.is_none() && diff.tiles.is_none());
        assert_eq!(diff.apply(&base).source.kind, "video");
    }
}
//#endregion 🧪️Tests
