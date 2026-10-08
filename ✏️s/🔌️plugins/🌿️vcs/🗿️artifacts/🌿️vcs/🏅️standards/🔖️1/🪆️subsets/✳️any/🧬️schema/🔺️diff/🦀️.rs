//! 🧬️ VCS diff schema — sparse field delta over the artifact.

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the VCS artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.vcs.vcs")]
pub struct VcsDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    pub counter: Option<i64>,
    #[state(artifact)]
    pub notes: Option<String>,
    #[state(artifact)]
    pub status: Option<String>,
    #[state(artifact)]
    pub tags: Option<VcsTagsDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
protocol::plain_list_delta! {
    /// 🏷️ Tag-list sparse delta: positional rows — `removed: [{id, index}]` (base index), `inserted: [{index, row}]` (after index), `moved: [{id, from, to}]`.
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    pub VcsTagsDelta { removal: VcsTagRemoval, insertion: VcsTagInsertion, relocation: VcsTagRelocation, row: String }
}
//#endregion 🔖️DeltaHelpers

use crate::VcsSnapshot;
use protocol::MutationDiff;

impl MutationDiff<VcsSnapshot> for VcsDiff {
    fn apply(&self, snapshot: &VcsSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<VcsSnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(title) = &self.title {
            next.title = title.clone();
        }
        if let Some(counter) = self.counter {
            next.counter = counter;
        }
        if let Some(notes) = &self.notes {
            next.notes = notes.clone();
        }
        if let Some(status) = &self.status {
            next.status = status.clone();
        }
        if let Some(delta) = &self.tags {
            next.tags = delta.commit_onto(&next.tags, capability).map_err(|error| error.under(["tags"]))?;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if let Some(later) = other.tags {
            match &mut self.tags {
                Some(first) => first.absorb(later),
                None => self.tags = Some(later),
            }
        }
        if self.tags.as_ref().is_some_and(VcsTagsDelta::is_empty) {
            self.tags = None;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(title);
        take!(counter);
        take!(notes);
        take!(status);
    }
}

impl protocol::DiffAlgebra<VcsSnapshot> for VcsDiff {
    fn inverse(&self, base: &VcsSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            title: self.title.as_ref().map(|_| base.title.clone()),
            counter: self.counter.map(|_| base.counter),
            notes: self.notes.as_ref().map(|_| base.notes.clone()),
            status: self.status.as_ref().map(|_| base.status.clone()),
            tags: self.tags.as_ref().map(|delta| delta.inverse(&base.tags)),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.title.is_none() && self.counter.is_none() && self.notes.is_none() && self.status.is_none() && self.tags.as_ref().is_none_or(VcsTagsDelta::is_empty)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
