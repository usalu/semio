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
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct VcsStringList {
    pub values: Vec<String>,
}

/// 🏷️ Tag-list sparse delta: removed tags, added tags and the complete resulting order when it differs from "remaining tags, then added tags".
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct VcsTagsDelta {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub reordered: Option<Vec<String>>,
}
//#endregion 🔖️DeltaHelpers

use crate::VcsSnapshot;
use protocol::MutationDiff;

/// 🏷️ Applies a tag delta: the removed tags leave, the added tags append in order (so remove-then-append of one tag moves it to the end).
pub fn apply_tags_delta(tags: &[String], delta: &VcsTagsDelta) -> protocol::MutationApplyResult<Vec<String>> {
    for (index, tag) in delta.removed.iter().enumerate() {
        if !tags.contains(tag) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed tag does not exist").at(["removed".to_string(), index.to_string()]));
        }
        if delta.removed[..index].contains(tag) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "tag is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
    }
    let mut next: Vec<String> = tags.iter().filter(|tag| !delta.removed.contains(tag)).cloned().collect();
    for (index, tag) in delta.added.iter().enumerate() {
        if next.contains(tag) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added tag already exists").at(["added".to_string(), index.to_string()]));
        }
        next.push(tag.clone());
    }
    let Some(order) = &delta.reordered else { return Ok(next) };
    if order.len() != next.len() || order.iter().enumerate().any(|(index, tag)| order[..index].contains(tag) || !next.contains(tag)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered".to_string()]));
    }
    Ok(order.clone())
}

impl VcsTagsDelta {
    /// 🧮️ Canonical form: removed sorted and a `reordered` that merely restates "remaining, then added" is dropped.
    fn canonical(mut self) -> Self {
        self.removed.sort();
        self.removed.dedup();
        if let Some(order) = &self.reordered {
            let tail = self.added.len();
            if order.len() >= tail && order.len() <= tail + 1 && order[order.len() - tail..] == self.added[..] {
                self.reordered = None;
            }
        }
        self
    }

    /// ➕️ Composes `self` then `later`: add∘remove cancels, remove∘add moves the tag to the end, the later order wins.
    fn absorb(&mut self, later: Self) {
        let kept: Option<Vec<String>> = self.reordered.take().map(|order| order.into_iter().filter(|tag| !later.removed.contains(tag)).chain(later.added.iter().cloned()).collect());
        for tag in &later.removed {
            match self.added.iter().position(|added| added == tag) {
                Some(position) => {
                    self.added.remove(position);
                }
                None if !self.removed.contains(tag) => self.removed.push(tag.clone()),
                None => {}
            }
        }
        self.added.extend(later.added);
        self.reordered = later.reordered.or(kept);
        *self = std::mem::take(self).canonical();
    }

    /// 🔁️ Puts back what the delta removed, takes away what it appended, and restores the base order.
    fn inverse(&self, base: &[String]) -> Self {
        let added: Vec<String> = base.iter().filter(|tag| self.removed.contains(tag)).cloned().collect();
        let after: Vec<String> = match &self.reordered {
            Some(order) => order.clone(),
            None => base.iter().filter(|tag| !self.removed.contains(tag)).chain(self.added.iter()).cloned().collect(),
        };
        let natural: Vec<String> = after.iter().filter(|tag| !self.added.contains(tag)).chain(added.iter()).cloned().collect();
        Self { added, removed: self.added.clone(), reordered: (natural != base).then(|| base.to_vec()) }.canonical()
    }

    /// 🧭️ The delta turning `base` into `other`.
    fn between(base: &[String], other: &[String]) -> Self {
        let removed: Vec<String> = base.iter().filter(|tag| !other.contains(tag)).cloned().collect();
        let added: Vec<String> = other.iter().filter(|tag| !base.contains(tag)).cloned().collect();
        let natural: Vec<String> = base.iter().filter(|tag| !removed.contains(tag)).chain(added.iter()).cloned().collect();
        Self { added, removed, reordered: (natural != other).then(|| other.to_vec()) }.canonical()
    }

    fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.reordered.is_none()
    }
}

impl MutationDiff<VcsSnapshot> for VcsDiff {
    fn apply(&self, snapshot: &VcsSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<VcsSnapshot> {
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
    fn between(base: &VcsSnapshot, other: &VcsSnapshot) -> Self {
        let tags = VcsTagsDelta::between(&base.tags, &other.tags);
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            title: (base.title != other.title).then(|| other.title.clone()),
            counter: (base.counter != other.counter).then_some(other.counter),
            notes: (base.notes != other.notes).then(|| other.notes.clone()),
            status: (base.status != other.status).then(|| other.status.clone()),
            tags: (!tags.is_empty()).then_some(tags),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.title.is_none() && self.counter.is_none() && self.notes.is_none() && self.status.is_none() && self.tags.as_ref().is_none_or(VcsTagsDelta::is_empty)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
