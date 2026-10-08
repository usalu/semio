//! 🧬️ Layout diff schema — sparse field delta over the artifact.

use crate::{CharacterStyle, GridSettings, ImageLink, ImageLinkPatch, LayoutDrawingChild, Page, PagePatch, ParagraphStyle, ParentPage, Spread, TextStory, TextStoryPatch};
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the layout artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.layout.layout")]
pub struct LayoutDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::standards::v1::subsets::any::schema::LayoutArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub name: Option<String>,
    #[state(artifact)]
    pub grid: Option<GridSettings>,
    #[state(artifact)]
    pub paragraph_styles: Option<LayoutParagraphStylesDelta>,
    #[state(artifact)]
    pub character_styles: Option<LayoutCharacterStylesDelta>,
    #[state(artifact)]
    pub stories: Option<LayoutStoriesDelta>,
    #[state(artifact)]
    pub links: Option<LayoutLinksDelta>,
    #[state(artifact)]
    pub parent_pages: Option<LayoutParentPagesDelta>,
    #[state(artifact)]
    pub spreads: Option<LayoutSpreadsDelta>,
    #[state(artifact)]
    pub pages: Option<LayoutPagesDelta>,
    #[state(artifact)]
    pub print_target: Option<Option<String>>,
    #[state(artifact)]
    pub data_fields: Option<FormDictionaryChange>,
    /// 🖇️ Optional composed-child slot: outer `Option` = "did the presence/identity change", inner
    /// `Option` = "is it now present" — the same double-`Option` shape `✳️object`'s own `mesh` diff
    /// already established, per the migration recipe's §8 diff-shape convention.
    #[state(artifact)]
    pub background_drawing: Option<Option<LayoutDrawingChild>>,
    /// 🔗️ Same double-`Option` shape as `background_drawing`, for the forward link slot.
    #[state(artifact)]
    pub referenced_model: Option<Option<store::ArtifactLink>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🧾️ A present change distinguishes clearing ownership from replacing it with an empty dictionary.
#[derive(Clone,Debug,PartialEq,ToValue,FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(deny_unknown_fields)]
pub struct FormDictionaryChange {pub dictionary:Option<crate::FormDictionary>}

/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutStringList {
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutPagesDelta {
    pub added: Vec<Page>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutPagePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutPagePatchEntry {
    pub id: String,
    pub patch: PagePatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutStoriesDelta {
    pub added: Vec<TextStory>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutStoryPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutStoryPatchEntry {
    pub id: String,
    pub patch: TextStoryPatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutLinksDelta {
    pub added: Vec<ImageLink>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutLinkPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutLinkPatchEntry {
    pub id: String,
    pub patch: ImageLinkPatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutParagraphStylesDelta {
    pub added: Vec<ParagraphStyle>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutParagraphStylePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutParagraphStylePatchEntry {
    pub id: String,
    pub patch: ParagraphStylePatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutCharacterStylesDelta {
    pub added: Vec<CharacterStyle>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutCharacterStylePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutCharacterStylePatchEntry {
    pub id: String,
    pub patch: CharacterStylePatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutParentPagesDelta {
    pub added: Vec<ParentPage>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutParentPagePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutParentPagePatchEntry {
    pub id: String,
    pub patch: ParentPagePatch,
}

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LayoutSpreadsDelta {
    pub added: Vec<Spread>,
    pub removed: Vec<String>,
    pub patched: Vec<LayoutSpreadPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct LayoutSpreadPatchEntry {
    pub id: String,
    pub patch: SpreadPatch,
}

/// 🩹 Sparse patch for a {@link ParagraphStyle}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct ParagraphStylePatch {
    pub name: Option<String>,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub font_weight: Option<u32>,
    pub leading: Option<f64>,
    pub tracking: Option<f64>,
    pub alignment: Option<String>,
}

/// 🩹 Sparse patch for a {@link CharacterStyle}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CharacterStylePatch {
    pub name: Option<Option<String>>,
    pub font_family: Option<Option<String>>,
    pub font_size: Option<Option<f64>>,
    pub font_weight: Option<Option<u32>>,
    pub italic: Option<Option<bool>>,
    pub color: Option<Option<[f32; 4]>>,
    pub tracking: Option<Option<f64>>,
}

/// 🩹 Sparse patch for a {@link ParentPage}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct ParentPagePatch {
    pub name: Option<String>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

/// 🩹 Sparse patch for a {@link Spread}.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct SpreadPatch {
    pub name: Option<String>,
}
//#endregion 🔖️DeltaHelpers

use crate::standards::v1::subsets::any::schema::LayoutArtifact;
use crate::LayoutSnapshot;
use protocol::Identified;
use protocol::MutationDiff;
use protocol::Patchable;

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

/// 🖼️ Whole-snapshot replacement diff.
pub fn diff_set_snapshot(snapshot: &LayoutSnapshot) -> LayoutDiff {
    LayoutDiff { artifact: Some(Box::new(LayoutArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
