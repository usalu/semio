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
    pub artifact: Option<Box<crate::schema::VcsArtifact>>,
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
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct VcsStringList {
    pub values: Vec<String>,
}

/// 🏷️ Tag-list sparse delta (added/removed tag strings).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct VcsTagsDelta {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}
//#endregion 🔖️DeltaHelpers

use crate::schema::VcsArtifact;
use crate::VcsSnapshot;
use protocol::MutationDiff;

pub fn apply_tags_delta(tags: &[String], delta: &VcsTagsDelta) -> protocol::MutationApplyResult<Vec<String>> {
    for (index, tag) in delta.removed.iter().enumerate() {
        if !tags.contains(tag) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed tag does not exist").at(["removed".to_string(), index.to_string()]));
        }
        if delta.removed[..index].contains(tag) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "tag is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
    }
    for (index, tag) in delta.added.iter().enumerate() {
        if tags.contains(tag) || delta.added[..index].contains(tag) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added tag already exists").at(["added".to_string(), index.to_string()]));
        }
        if delta.removed.contains(tag) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "tag cannot be removed and added").at(["added".to_string(), index.to_string()]));
        }
    }
    let mut next = tags.to_vec();
    for tag in &delta.removed {
        next.retain(|e| e != tag);
    }
    for tag in &delta.added {
        next.push(tag.clone());
    }
    Ok(next)
}

fn absorb_tags_delta(target: &mut Option<VcsTagsDelta>, incoming: Option<VcsTagsDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
            }
            None => *target = Some(src),
        }
    }
}

impl VcsDiff {
    /// 🧬️ Applies sparse document fields onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &VcsArtifact) -> protocol::MutationApplyResult<VcsArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
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
                next.tags = apply_tags_delta(&next.tags, delta).map_err(|error| error.under(["tags"]))?;
            }
            next
        })
    }
}

impl MutationDiff<VcsSnapshot> for VcsDiff {
    fn apply(&self, snapshot: &VcsSnapshot) -> protocol::MutationApplyResult<VcsSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
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
                next.tags = apply_tags_delta(&next.tags, delta).map_err(|error| error.under(["tags"]))?;
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        absorb_tags_delta(&mut self.tags, other.tags);
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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
