//! 🔺️ Layout artifact — sparse field-delta diff codec and apply/absorb.

use crate::standards::v1::subsets::any::schema::diff::{LayoutCharacterStylePatchEntry, LayoutCharacterStylesDelta, LayoutDiff, LayoutLinkPatchEntry, LayoutLinksDelta, LayoutPagePatchEntry, LayoutPagesDelta, LayoutParagraphStylePatchEntry, LayoutParagraphStylesDelta, LayoutParentPagePatchEntry, LayoutParentPagesDelta, LayoutSpreadPatchEntry, LayoutSpreadsDelta, LayoutStoriesDelta, LayoutStoryPatchEntry};
use crate::standards::v1::subsets::any::schema::LayoutArtifact;
use crate::{CharacterStyle, ImageLink, LayoutSnapshot, Page, ParagraphStyle, ParentPage, Spread, TextStory};
use protocol::{Identified, MutationDiff, Patchable};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
fn apply_identified_delta<T, P, E, F>(items: &[T], removed: &[String], added: &[T], patched: &[E], reordered: Option<&Vec<String>>, entry_parts: F) -> protocol::MutationApplyResult<Vec<T>>
where
    T: Clone + Identified<String> + Patchable<P>,
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

pub fn apply_pages_delta(items: &[Page], delta: &LayoutPagesDelta) -> protocol::MutationApplyResult<Vec<Page>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutPagePatchEntry| (&entry.id, &entry.patch))
}

pub fn apply_stories_delta(items: &[TextStory], delta: &LayoutStoriesDelta) -> protocol::MutationApplyResult<Vec<TextStory>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutStoryPatchEntry| (&entry.id, &entry.patch))
}

pub fn apply_links_delta(items: &[ImageLink], delta: &LayoutLinksDelta) -> protocol::MutationApplyResult<Vec<ImageLink>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutLinkPatchEntry| (&entry.id, &entry.patch))
}

pub fn apply_paragraph_styles_delta(items: &[ParagraphStyle], delta: &LayoutParagraphStylesDelta) -> protocol::MutationApplyResult<Vec<ParagraphStyle>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutParagraphStylePatchEntry| (&entry.id, &entry.patch))
}

pub fn apply_character_styles_delta(items: &[CharacterStyle], delta: &LayoutCharacterStylesDelta) -> protocol::MutationApplyResult<Vec<CharacterStyle>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutCharacterStylePatchEntry| (&entry.id, &entry.patch))
}

pub fn apply_parent_pages_delta(items: &[ParentPage], delta: &LayoutParentPagesDelta) -> protocol::MutationApplyResult<Vec<ParentPage>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutParentPagePatchEntry| (&entry.id, &entry.patch))
}

pub fn apply_spreads_delta(items: &[Spread], delta: &LayoutSpreadsDelta) -> protocol::MutationApplyResult<Vec<Spread>> {
    apply_identified_delta(items, &delta.removed, &delta.added, &delta.patched, delta.reordered.as_ref(), |entry: &LayoutSpreadPatchEntry| (&entry.id, &entry.patch))
}

impl LayoutDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &LayoutArtifact) -> protocol::MutationApplyResult<LayoutArtifact> {
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
            if let Some(delta) = &self.paragraph_styles {
                next.paragraph_styles = apply_paragraph_styles_delta(&next.paragraph_styles, delta).map_err(|error| error.under(["paragraphStyles"]))?;
            }
            if let Some(delta) = &self.character_styles {
                next.character_styles = apply_character_styles_delta(&next.character_styles, delta).map_err(|error| error.under(["characterStyles"]))?;
            }
            if let Some(delta) = &self.parent_pages {
                next.parent_pages = apply_parent_pages_delta(&next.parent_pages, delta).map_err(|error| error.under(["parentPages"]))?;
            }
            if let Some(delta) = &self.spreads {
                next.spreads = apply_spreads_delta(&next.spreads, delta).map_err(|error| error.under(["spreads"]))?;
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
            if let Some(value) = &self.data_fields {
                next.data_fields = value.dictionary.clone();
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
}

impl MutationDiff<LayoutSnapshot> for LayoutDiff {
    fn apply(&self, snapshot: &LayoutSnapshot) -> protocol::MutationApplyResult<LayoutSnapshot> {
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
            if let Some(delta) = &self.paragraph_styles {
                next.paragraph_styles = apply_paragraph_styles_delta(&next.paragraph_styles, delta).map_err(|error| error.under(["paragraphStyles"]))?;
            }
            if let Some(delta) = &self.character_styles {
                next.character_styles = apply_character_styles_delta(&next.character_styles, delta).map_err(|error| error.under(["characterStyles"]))?;
            }
            if let Some(delta) = &self.parent_pages {
                next.parent_pages = apply_parent_pages_delta(&next.parent_pages, delta).map_err(|error| error.under(["parentPages"]))?;
            }
            if let Some(delta) = &self.spreads {
                next.spreads = apply_spreads_delta(&next.spreads, delta).map_err(|error| error.under(["spreads"]))?;
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
            if let Some(value) = &self.data_fields {
                next.data_fields = value.dictionary.clone();
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
        macro_rules! absorb_collection {
            ($field:ident) => {
                if let Some(src) = other.$field {
                    match &mut self.$field {
                        Some(dst) => {
                            dst.added.extend(src.added);
                            dst.removed.extend(src.removed);
                            dst.patched.extend(src.patched);
                            if src.reordered.is_some() {
                                dst.reordered = src.reordered;
                            }
                        }
                        None => self.$field = Some(src),
                    }
                }
            };
        }
        absorb_collection!(paragraph_styles);
        absorb_collection!(character_styles);
        absorb_collection!(parent_pages);
        absorb_collection!(spreads);
        take!(print_target);
        take!(data_fields);
        take!(background_drawing);
        take!(referenced_model);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
/// 🖼️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff { artifact: Some(Box::new(LayoutArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type LayoutDiffText = String;
//#endregion 🚚️Carrier
