//! 🧮️ Apply and algebra of the layout diff: the diff absorption, the mutation diff application and the diff algebra implementation. See [`super`].

use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
use super::*;

//#region 🔖️Apply
macro_rules! absorb_lists {
    ($first:ident, $second:ident, $($field:ident),+ $(,)?) => {$(
        if let Some(later) = $second.$field {
            let mut merged = $first.$field.take().unwrap_or_default();
            merged.absorb(later);
            $first.$field = (!merged.is_empty()).then_some(merged);
        }
    )+};
}

impl LayoutDiff {
    fn apply_data_fields(&self, base: &Option<semio_framework_value_dictionary::QuestionDictionary>, capability: ApplyCapability) -> MutationApplyResult<Option<semio_framework_value_dictionary::QuestionDictionary>> {
        let Some(delta) = &self.data_fields else { return Ok(base.clone()) };
        let entries = delta.entries.clone().unwrap_or_default();
        let commit = |held: &[semio_framework_value_dictionary::QuestionDictionaryEntry]| entries.commit_onto(held, capability).map_err(|error| error.under(["dataFields", "entries"]));
        Ok(match (base, delta.presence) {
            (_, Some(DataFieldsPresence::Deleted)) => None,
            (None, Some(DataFieldsPresence::Created)) | (Some(_), Some(DataFieldsPresence::Replaced)) | (None, Some(DataFieldsPresence::Replaced)) => Some(semio_framework_value_dictionary::QuestionDictionary { entries: commit(&[])? }),
            (Some(dictionary), _) => Some(semio_framework_value_dictionary::QuestionDictionary { entries: commit(&dictionary.entries)? }),
            (None, None) => {
                if entries.is_empty() {
                    None
                } else {
                    return Err(MutationApplyError::new("mutation.apply.missing-target", "data-field rows need an existing dictionary").at(["dataFields"]));
                }
            }
        })
    }
}

impl MutationDiff<LayoutSnapshot> for LayoutDiff {}

impl protocol::ColdMutationDiff<LayoutSnapshot> for LayoutDiff {
    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.name.is_some() {
            self.name = other.name;
        }
        if let Some(later) = other.grid {
            match &mut self.grid {
                Some(earlier) => earlier.composed(later),
                None => self.grid = Some(later),
            }
        }
        absorb_lists!(self, other, paragraph_styles, character_styles, parent_pages, spreads, pages, stories, links);
        if other.print_target.is_some() {
            self.print_target = other.print_target;
        }
        if let Some(later) = other.data_fields {
            self.data_fields = absorb_data_fields(self.data_fields.take(), later);
        }
        if other.background_drawing.is_some() {
            self.background_drawing = other.background_drawing;
        }
        if other.referenced_model.is_some() {
            self.referenced_model = other.referenced_model;
        }
    }

    fn apply(&self, unpublished: &mut LayoutSnapshot, capability: ApplyCapability) -> MutationApplyResult<()> {
        let applied = (|snapshot: &LayoutSnapshot| -> MutationApplyResult<LayoutSnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(name) = &self.name {
            next.name = name.clone();
        }
        if let Some(patch) = &self.grid {
            next.grid = patch.applied(&next.grid);
        }
        if let Some(delta) = &self.paragraph_styles {
            next.paragraph_styles = delta.commit_onto(&next.paragraph_styles, capability).map_err(|error| error.under(["paragraphStyles"]))?;
        }
        if let Some(delta) = &self.character_styles {
            next.character_styles = delta.commit_onto(&next.character_styles, capability).map_err(|error| error.under(["characterStyles"]))?;
        }
        if let Some(delta) = &self.parent_pages {
            next.parent_pages = delta.commit_onto(&next.parent_pages, capability).map_err(|error| error.under(["parentPages"]))?;
        }
        if let Some(delta) = &self.spreads {
            next.spreads = delta.commit_onto(&next.spreads, capability).map_err(|error| error.under(["spreads"]))?;
        }
        if let Some(delta) = &self.pages {
            next.pages = delta.commit_onto(&next.pages, capability).map_err(|error| error.under(["pages"]))?;
        }
        if let Some(delta) = &self.stories {
            next.stories = delta.commit_onto(&next.stories, capability).map_err(|error| error.under(["stories"]))?;
        }
        if let Some(delta) = &self.links {
            next.links = delta.commit_onto(&next.links, capability).map_err(|error| error.under(["links"]))?;
        }
        if let Some(change) = &self.print_target {
            next.print_target = change.target.clone();
        }
        next.data_fields = self.apply_data_fields(&next.data_fields, capability)?;
        if let Some(value) = &self.background_drawing {
            next.background_drawing = value.clone();
        }
        if let Some(value) = &self.referenced_model {
            next.referenced_model = value.clone();
        }
        Ok(next)
    
        })(&*unpublished)?;
        *unpublished = applied;
        Ok(())
    }
}

fn absorb_data_fields(first: Option<LayoutDataFieldsDelta>, second: LayoutDataFieldsDelta) -> Option<LayoutDataFieldsDelta> {
    use crate::DataFieldsPresence::{Created, Deleted, Replaced};
    let first = first.unwrap_or_default();
    let entries = |first: Option<LayoutDataEntriesDelta>, second: Option<LayoutDataEntriesDelta>| match (first, second) {
        (Some(mut first), Some(second)) => {
            first.absorb(second);
            (!first.is_empty()).then_some(first)
        }
        (first, None) => first,
        (None, second) => second,
    };
    let merged = match (first.presence, second.presence) {
        (_, Some(Deleted)) if first.presence == Some(Created) => LayoutDataFieldsDelta::default(),
        (_, Some(Deleted)) => LayoutDataFieldsDelta { presence: Some(Deleted), entries: None },
        (Some(Deleted), Some(Created | Replaced)) => LayoutDataFieldsDelta { presence: Some(Replaced), entries: second.entries },
        (_, Some(presence)) => LayoutDataFieldsDelta { presence: Some(presence), entries: second.entries },
        (Some(Deleted), None) => LayoutDataFieldsDelta { presence: Some(Deleted), entries: None },
        (presence, None) => LayoutDataFieldsDelta { presence, entries: entries(first.entries, second.entries) },
    };
    (merged != LayoutDataFieldsDelta::default()).then_some(merged)
}

impl DiffAlgebra<LayoutSnapshot> for LayoutDiff {
    fn is_empty(&self) -> bool {
        self.schema.is_none()
            && self.name.is_none()
            && self.grid.as_ref().is_none_or(|patch| patch == &GridPatch::default())
            && self.paragraph_styles.as_ref().is_none_or(|delta| delta.is_empty())
            && self.character_styles.as_ref().is_none_or(|delta| delta.is_empty())
            && self.stories.as_ref().is_none_or(|delta| delta.is_empty())
            && self.links.as_ref().is_none_or(|delta| delta.is_empty())
            && self.parent_pages.as_ref().is_none_or(|delta| delta.is_empty())
            && self.spreads.as_ref().is_none_or(|delta| delta.is_empty())
            && self.pages.as_ref().is_none_or(|delta| delta.is_empty())
            && self.print_target.is_none()
            && self.data_fields.as_ref().is_none_or(|delta| delta == &LayoutDataFieldsDelta::default())
            && self.background_drawing.is_none()
            && self.referenced_model.is_none()
    }
}

impl protocol::ColdDiffAlgebra<LayoutSnapshot> for LayoutDiff {
    fn inverse(&self, base: &LayoutSnapshot) -> Self {
        let data_fields = self.data_fields.as_ref().map(|delta| {
            let base_entries = base.data_fields.as_ref().map(|dictionary| dictionary.entries.as_slice()).unwrap_or_default();
            let all_base_rows = || LayoutDataEntriesDelta { inserted: base_entries.iter().cloned().enumerate().map(|(index, row)| LayoutDataEntryInsertion { index, row }).collect(), ..Default::default() };
            match delta.presence {
                Some(DataFieldsPresence::Created) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Deleted), entries: None },
                Some(DataFieldsPresence::Deleted) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Created), entries: Some(all_base_rows()) },
                Some(DataFieldsPresence::Replaced) => LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Replaced), entries: Some(all_base_rows()) },
                None => LayoutDataFieldsDelta { presence: None, entries: delta.entries.as_ref().map(|entries| entries.inverse(base_entries)) },
            }
        });
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            name: self.name.as_ref().map(|_| base.name.clone()),
            grid: self.grid.as_ref().map(|patch| patch.inverse_against(&base.grid)),
            paragraph_styles: self.paragraph_styles.as_ref().map(|delta| delta.inverse(&base.paragraph_styles)),
            character_styles: self.character_styles.as_ref().map(|delta| delta.inverse(&base.character_styles)),
            stories: self.stories.as_ref().map(|delta| delta.inverse(&base.stories)),
            links: self.links.as_ref().map(|delta| delta.inverse(&base.links)),
            parent_pages: self.parent_pages.as_ref().map(|delta| delta.inverse(&base.parent_pages)),
            spreads: self.spreads.as_ref().map(|delta| delta.inverse(&base.spreads)),
            pages: self.pages.as_ref().map(|delta| delta.inverse(&base.pages)),
            print_target: self.print_target.as_ref().map(|_| PrintTargetChange { target: base.print_target.clone() }),
            data_fields,
            background_drawing: self.background_drawing.as_ref().map(|_| base.background_drawing.clone()),
            referenced_model: self.referenced_model.as_ref().map(|_| base.referenced_model.clone()),
        }
    }
}
//#endregion 🔖️Apply
