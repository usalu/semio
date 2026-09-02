//! 🔺️ Layout artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::layout::schema::diff::{LayoutDiff, LayoutLinkPatchEntry, LayoutLinksDelta, LayoutPagePatchEntry, LayoutPagesDelta, LayoutStoriesDelta, LayoutStoryPatchEntry};
use crate::artifacts::layout::schema::LayoutArtifact;
use crate::artifacts::layout::{ImageLink, LayoutSnapshot, Page, TextStory};
use protocol::{Identified, MutationDiff, Patchable};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
async fn apply_identified_delta<T, P, E, F>(items: &[T], removed: &[String], added: &[T], patched: &[E], reordered: Option<&Vec<String>>, entry_parts: F) -> protocol::MutationApplyResult<Vec<T>>
where
    T: Clone + protocol::Identified<String> + Patchable<P>,
    P: Clone,
    F: Fn(&E) -> (&String, &P),
{
    let mut next = items.to_vec();
    let mut seen = std::collections::HashSet::new();
    for id in removed {
        if !seen.insert(id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item is removed more than once").at(["removed", id.as_str()]));
        }
        let position = next.iter().position(|item| item.id() == id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "removed item does not exist").at(["removed", id.as_str()]))?;
        next.remove(position);
    }
    seen.clear();
    for item in added {
        let id = item.id();
        if !seen.insert(id.clone()) || next.iter().any(|entry| entry.id() == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added item identity already exists").at(["added", id.as_str()]));
        }
        next.push(item.clone());
    }
    seen.clear();
    for entry in patched {
        let (id, patch) = entry_parts(entry);
        if !seen.insert(id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item is patched more than once").at(["patched", id.as_str()]));
        }
        let item = next.iter_mut().find(|item| item.id() == id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched item does not exist").at(["patched", id.as_str()]))?;
        item.apply_patch(patch);
    }
    if let Some(order) = reordered {
        if order.len() != next.len() {
            return Err(protocol::MutationApplyError::new("mutation.apply.incomplete-diff", format!("order has length {}, expected {}", order.len(), next.len())).at(["reordered"]));
        }
        seen.clear();
        for id in order {
            if !seen.insert(id.clone()) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item appears more than once in order").at(["reordered", id.as_str()]));
            }
            if !next.iter().any(|item| item.id() == id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "ordered item does not exist").at(["reordered", id.as_str()]));
            }
        }
        let mut ordered = Vec::with_capacity(next.len());
        for id in order {
            let position = next.iter().position(|item| item.id() == id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "ordered item does not exist").at(["reordered", id.as_str()]))?;
            ordered.push(next.remove(position));
        }
        next = ordered;
    }
    Ok(next)
}

pub async fn apply_pages_delta(items: &[Page], delta: &LayoutPagesDelta) -> protocol::MutationApplyResult<Vec<Page>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutPagePatchEntry| (&entry.id, &entry.patch))
}

pub async fn apply_stories_delta(items: &[TextStory], delta: &LayoutStoriesDelta) -> protocol::MutationApplyResult<Vec<TextStory>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutStoryPatchEntry| (&entry.id, &entry.patch))
}

pub async fn apply_links_delta(items: &[ImageLink], delta: &LayoutLinksDelta) -> protocol::MutationApplyResult<Vec<ImageLink>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutLinkPatchEntry| (&entry.id, &entry.patch))
}

impl LayoutDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub async fn apply_to_artifact(&self, artifact: &LayoutArtifact) -> protocol::MutationApplyResult<LayoutArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
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
                next.pages = apply_pages_delta(&next.pages, delta).map_err(|error| error.under(["pages"]))?;
            }
            if let Some(delta) = &self.stories {
                next.stories = apply_stories_delta(&next.stories, delta).map_err(|error| error.under(["stories"]))?;
            }
            if let Some(delta) = &self.links {
                next.links = apply_links_delta(&next.links, delta).map_err(|error| error.under(["links"]))?;
            }
            if let Some(value) = &self.print_target {
                next.print_target = value.clone();
            }
            if let Some(value) = &self.data_fields_json {
                next.data_fields_json = value.clone();
            }
            if let Some(value) = &self.background_drawing {
                next.background_drawing = value.clone();
            }
            if let Some(value) = &self.referenced_model {
                next.referenced_model = value.clone();
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
        })
    }
}

impl MutationDiff<LayoutSnapshot> for LayoutDiff {
    async fn apply(&self, snapshot: &LayoutSnapshot) -> protocol::MutationApplyResult<LayoutSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
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
                next.pages = apply_pages_delta(&next.pages, delta).map_err(|error| error.under(["pages"]))?;
            }
            if let Some(delta) = &self.stories {
                next.stories = apply_stories_delta(&next.stories, delta).map_err(|error| error.under(["stories"]))?;
            }
            if let Some(delta) = &self.links {
                next.links = apply_links_delta(&next.links, delta).map_err(|error| error.under(["links"]))?;
            }
            if let Some(value) = &self.print_target {
                next.print_target = value.clone();
            }
            if let Some(value) = &self.data_fields_json {
                next.data_fields_json = value.clone();
            }
            if let Some(value) = &self.background_drawing {
                next.background_drawing = value.clone();
            }
            if let Some(value) = &self.referenced_model {
                next.referenced_model = value.clone();
            }
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        async fn absorb_pages(target: &mut Option<LayoutPagesDelta>, incoming: Option<LayoutPagesDelta>) {
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
        take!(background_drawing);
        take!(referenced_model);
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
/// 🖼️ Whole-snapshot replacement diff.
pub async fn diff_set_snapshot(snapshot: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff { artifact: Some(Box::new(LayoutArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    #[semio_framework_async_macros::async_test]
    async fn set_data_fields_diff_applies_onto_the_base_snapshot() {
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
            background_drawing: None,
            referenced_model: None,
        };
        let operation = crate::artifacts::layout::mutations::LayoutMutation::ChangeDataFields(crate::artifacts::layout::mutations::change_data_fields::mutation::ChangeDataFields { new_json: Some("{}".into()) });
        let diff: LayoutDiff = operation.diff(&base).into_parts().0;
        let applied = diff.apply(&base).expect("valid mutation diff");
        assert_eq!(applied.data_fields_json.as_deref(), Some("{}"));
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_replaces_with_whole_artifact_diff() {
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
            background_drawing: None,
            referenced_model: None,
        };
        diff.absorb(diff_set_snapshot(&snap));
        assert!(diff.artifact.is_some());
    }
}
//#endregion 🧪️Tests
