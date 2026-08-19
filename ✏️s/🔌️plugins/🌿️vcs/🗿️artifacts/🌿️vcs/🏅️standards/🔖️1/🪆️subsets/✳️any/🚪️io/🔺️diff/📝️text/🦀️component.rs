//! 🔺️ VCS artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::vcs::schema::VcsArtifact;
use crate::artifacts::vcs::VcsSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::vcs::schema::diff::*;

//#region 🔖️Apply
pub async fn apply_tags_delta(
    tags: &[String],
    delta: &VcsTagsDelta,
) -> protocol::MutationApplyResult<Vec<String>> {
    for (index, tag) in delta.removed.iter().enumerate() {
        if !tags.contains(tag) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.missing-target",
                "removed tag does not exist",
            )
            .at(["removed".to_string(), index.to_string()]));
        }
        if delta.removed[..index].contains(tag) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.duplicate-target",
                "tag is removed more than once",
            )
            .at(["removed".to_string(), index.to_string()]));
        }
    }
    for (index, tag) in delta.added.iter().enumerate() {
        if tags.contains(tag) || delta.added[..index].contains(tag) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.duplicate-target",
                "added tag already exists",
            )
            .at(["added".to_string(), index.to_string()]));
        }
        if delta.removed.contains(tag) {
            return Err(protocol::MutationApplyError::new(
                "mutation.apply.conflicting-target",
                "tag cannot be removed and added",
            )
            .at(["added".to_string(), index.to_string()]));
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

async fn absorb_tags_delta(target: &mut Option<VcsTagsDelta>, incoming: Option<VcsTagsDelta>) {
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
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub async fn apply_to_artifact(&self, artifact: &VcsArtifact) -> protocol::MutationApplyResult<VcsArtifact> {
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
                next.tags = apply_tags_delta(&next.tags, delta)
                    .map_err(|error| error.under(["tags"]))?;
            }
            if let Some(list) = &self.selected_checkpoint_ids {
                next.selected_checkpoint_ids = list.values.clone();
            }
            if let Some(value) = &self.locale {
                next.locale = value.clone();
            }
            next
        })
    }
}

impl MutationDiff<VcsSnapshot> for VcsDiff {
    async fn apply(&self, snapshot: &VcsSnapshot) -> protocol::MutationApplyResult<VcsSnapshot> {
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
                next.tags = apply_tags_delta(&next.tags, delta)
                    .map_err(|error| error.under(["tags"]))?;
            }
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
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
        take!(selected_checkpoint_ids);
        take!(locale);
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn empty_diff_is_a_no_operation() {
        let base = crate::artifacts::vcs::standards::v1::subsets::any::schema::empty_vcs_snapshot();
        let diff = VcsDiff::default();
        assert_eq!(diff.apply(&base).expect("valid mutation diff"), base);
    }
}
//#endregion 🧪️Tests
