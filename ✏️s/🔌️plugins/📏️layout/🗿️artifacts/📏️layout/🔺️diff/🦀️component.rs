//! 🔺️ Layout artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::layout::diff::schema::{
    LayoutDiff, LayoutLinkPatchEntry, LayoutLinksDelta, LayoutPagePatchEntry, LayoutPagesDelta, LayoutStoriesDelta,
    LayoutStoryPatchEntry, LayoutStringList,
};
use crate::artifacts::layout::schema::LayoutArtifact;
use crate::artifacts::layout::{
    ImageLink, ImageLinkPatch, Page, PagePatch, TextStory, TextStoryPatch, LayoutSnapshot,
};
use protocol::{CollectionMutation, Identified, MutationDiff, Patchable};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

//#region 🔖️Apply
fn apply_identified_delta<T, P, E, F>(
    items: &[T],
    removed: &[String],
    added: &[T],
    patched: &[E],
    reordered: Option<&Vec<String>>,
    entry_parts: F,
) -> Vec<T>
where
    T: Clone + Identified<String> + Patchable<P>,
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
        let mut by_id: std::collections::BTreeMap<_, _> =
            next.into_iter().map(|item| (item.id().clone(), item)).collect();
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

pub fn apply_pages_delta(items: &[Page], delta: &LayoutPagesDelta) -> Vec<Page> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutPagePatchEntry| {
        (&entry.id, &entry.patch)
    })
}

pub fn apply_stories_delta(items: &[TextStory], delta: &LayoutStoriesDelta) -> Vec<TextStory> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutStoryPatchEntry| {
        (&entry.id, &entry.patch)
    })
}

pub fn apply_links_delta(items: &[ImageLink], delta: &LayoutLinksDelta) -> Vec<ImageLink> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutLinkPatchEntry| {
        (&entry.id, &entry.patch)
    })
}

impl LayoutDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &LayoutArtifact) -> LayoutArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(name) = &self.name {
            next.name = name.clone();
        }
        if let Some(grid) = &self.grid {
            next.grid = grid.clone();
        }
        if let Some(delta) = &self.pages {
            next.pages = apply_pages_delta(&next.pages, delta);
        }
        if let Some(delta) = &self.stories {
            next.stories = apply_stories_delta(&next.stories, delta);
        }
        if let Some(delta) = &self.links {
            next.links = apply_links_delta(&next.links, delta);
        }
        if let Some(value) = &self.print_target {
            next.print_target = value.clone();
        }
        if let Some(value) = &self.data_fields_json {
            next.data_fields_json = value.clone();
        }
        if let Some(list) = &self.selected_ids {
            next.selected_ids = list.values.clone();
        }
        if let Some(value) = &self.active_page_id {
            next.active_page_id = value.clone();
        }
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
        }
        if let Some(value) = self.camera_x {
            next.camera_x = value;
        }
        if let Some(value) = self.camera_y {
            next.camera_y = value;
        }
        if let Some(value) = self.camera_zoom {
            next.camera_zoom = value;
        }
        if let Some(value) = self.preview_camera_x {
            next.preview_camera_x = value;
        }
        if let Some(value) = self.preview_camera_y {
            next.preview_camera_y = value;
        }
        if let Some(value) = self.preview_camera_zoom {
            next.preview_camera_zoom = value;
        }
        if let Some(value) = &self.drop_preview {
            next.drop_preview = value.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        if let Some(value) = &self.hovered_id {
            next.hovered_id = value.clone();
        }
        next
    }
}

impl MutationDiff<LayoutSnapshot> for LayoutDiff {
    fn apply(&self, snapshot: &LayoutSnapshot) -> LayoutSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(name) = &self.name {
            next.name = name.clone();
        }
        if let Some(grid) = &self.grid {
            next.grid = grid.clone();
        }
        if let Some(delta) = &self.pages {
            next.pages = apply_pages_delta(&next.pages, delta);
        }
        if let Some(delta) = &self.stories {
            next.stories = apply_stories_delta(&next.stories, delta);
        }
        if let Some(delta) = &self.links {
            next.links = apply_links_delta(&next.links, delta);
        }
        if let Some(value) = &self.print_target {
            next.print_target = value.clone();
        }
        if let Some(value) = &self.data_fields_json {
            next.data_fields_json = value.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        fn absorb_pages(target: &mut Option<LayoutPagesDelta>, incoming: Option<LayoutPagesDelta>) {
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
        absorb_pages(&mut self.pages, other.pages);
        if let Some(src) = other.stories {
            match &mut self.stories {
                Some(dst) => {
                    dst.added.extend(src.added);
                    dst.removed.extend(src.removed);
                    dst.patched.extend(src.patched);
                    if src.reordered.is_some() {
                        dst.reordered = src.reordered;
                    }
                }
                None => self.stories = Some(src),
            }
        }
        if let Some(src) = other.links {
            match &mut self.links {
                Some(dst) => {
                    dst.added.extend(src.added);
                    dst.removed.extend(src.removed);
                    dst.patched.extend(src.patched);
                    if src.reordered.is_some() {
                        dst.reordered = src.reordered;
                    }
                }
                None => self.links = Some(src),
            }
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(name);
        take!(grid);
        take!(print_target);
        take!(data_fields_json);
        take!(selected_ids);
        take!(active_page_id);
        take!(engagement_input);
        take!(camera_x);
        take!(camera_y);
        take!(camera_zoom);
        take!(preview_camera_x);
        take!(preview_camera_y);
        take!(preview_camera_zoom);
        take!(drop_preview);
        take!(locale);
        take!(hovered_id);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn pages_delta_from_collection_mutation(
    base: &[Page],
    op: &CollectionMutation<String, Page, PagePatch>,
) -> LayoutPagesDelta {
    match op {
        CollectionMutation::Add { item, .. } => LayoutPagesDelta {
            added: vec![item.clone()],
            ..Default::default()
        },
        CollectionMutation::Remove { id } => LayoutPagesDelta {
            removed: vec![id.clone()],
            ..Default::default()
        },
        CollectionMutation::Patch { id, patch } => LayoutPagesDelta {
            patched: vec![LayoutPagePatchEntry { id: id.clone(), patch: patch.clone() }],
            ..Default::default()
        },
        CollectionMutation::Move { id, to_index } => {
            let mut ids: Vec<String> = base.iter().map(|item| item.id.clone()).collect();
            if let Some(from) = ids.iter().position(|x| x == id) {
                let item = ids.remove(from);
                let to = (*to_index).min(ids.len());
                ids.insert(to, item);
            }
            LayoutPagesDelta {
                reordered: Some(ids),
                ..Default::default()
            }
        }
    }
}

pub fn stories_delta_from_collection_mutation(
    base: &[TextStory],
    op: &CollectionMutation<String, TextStory, TextStoryPatch>,
) -> LayoutStoriesDelta {
    match op {
        CollectionMutation::Add { item, .. } => LayoutStoriesDelta {
            added: vec![item.clone()],
            ..Default::default()
        },
        CollectionMutation::Remove { id } => LayoutStoriesDelta {
            removed: vec![id.clone()],
            ..Default::default()
        },
        CollectionMutation::Patch { id, patch } => LayoutStoriesDelta {
            patched: vec![LayoutStoryPatchEntry { id: id.clone(), patch: patch.clone() }],
            ..Default::default()
        },
        CollectionMutation::Move { id, to_index } => {
            let mut ids: Vec<String> = base.iter().map(|item| item.id.clone()).collect();
            if let Some(from) = ids.iter().position(|x| x == id) {
                let item = ids.remove(from);
                let to = (*to_index).min(ids.len());
                ids.insert(to, item);
            }
            LayoutStoriesDelta {
                reordered: Some(ids),
                ..Default::default()
            }
        }
    }
}

pub fn links_delta_from_collection_mutation(
    base: &[ImageLink],
    op: &CollectionMutation<String, ImageLink, ImageLinkPatch>,
) -> LayoutLinksDelta {
    match op {
        CollectionMutation::Add { item, .. } => LayoutLinksDelta {
            added: vec![item.clone()],
            ..Default::default()
        },
        CollectionMutation::Remove { id } => LayoutLinksDelta {
            removed: vec![id.clone()],
            ..Default::default()
        },
        CollectionMutation::Patch { id, patch } => LayoutLinksDelta {
            patched: vec![LayoutLinkPatchEntry { id: id.clone(), patch: patch.clone() }],
            ..Default::default()
        },
        CollectionMutation::Move { id, to_index } => {
            let mut ids: Vec<String> = base.iter().map(|item| item.id.clone()).collect();
            if let Some(from) = ids.iter().position(|x| x == id) {
                let item = ids.remove(from);
                let to = (*to_index).min(ids.len());
                ids.insert(to, item);
            }
            LayoutLinksDelta {
                reordered: Some(ids),
                ..Default::default()
            }
        }
    }
}

pub fn pages_replace_delta(before: &[Page], after: &[Page]) -> LayoutPagesDelta {
    LayoutPagesDelta {
        removed: before.iter().map(|p| p.id.clone()).collect(),
        added: after.to_vec(),
        ..Default::default()
    }
}

/// 🖼️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff {
        artifact: Some(Box::new(LayoutArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    #[test]
    fn set_data_fields_diff_applies_onto_the_base_snapshot() {
        let base = LayoutSnapshot {
            schema: crate::artifacts::layout::LAYOUT_DOCUMENT_SCHEMA.into(),
            name: "t".into(),
            grid: crate::artifacts::layout::GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
            paragraph_styles: Vec::new(),
            character_styles: Vec::new(),
            stories: Vec::new(),
            links: Vec::new(),
            parent_pages: Vec::new(),
            spreads: Vec::new(),
            pages: Vec::new(),
            print_target: None,
            data_fields_json: None,
        };
        let operation = crate::artifacts::layout::mutations::LayoutMutation::SetDataFields { json: Some("{}".into()) };
        let diff: LayoutDiff = operation.diff(&base);
        let applied = diff.apply(&base);
        assert_eq!(applied.data_fields_json.as_deref(), Some("{}"));
    }

    #[test]
    fn absorb_replaces_with_whole_artifact_diff() {
        let mut diff = LayoutDiff::default();
        let snap = LayoutSnapshot {
            schema: crate::artifacts::layout::LAYOUT_DOCUMENT_SCHEMA.into(),
            name: "x".into(),
            grid: crate::artifacts::layout::GridSettings { baseline_grid: 12.0, baseline_offset: 0.0, snap_to_baseline: false },
            paragraph_styles: Vec::new(),
            character_styles: Vec::new(),
            stories: Vec::new(),
            links: Vec::new(),
            parent_pages: Vec::new(),
            spreads: Vec::new(),
            pages: Vec::new(),
            print_target: None,
            data_fields_json: None,
        };
        diff.absorb(diff_set_snapshot(&snap));
        assert!(diff.artifact.is_some());
    }
}
//#endregion 🧪️Tests
