//! 🔺️ Vdi3805 artifact — sparse field diff runtime.

use crate::document_schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::document_schema::Vdi3805Artifact;
use crate::Vdi3805Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl Vdi3805Diff {
    pub fn apply_to_artifact(&self, artifact: &Vdi3805Artifact) -> protocol::MutationApplyResult<Vdi3805Artifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(value) = &self.manufacturer_file {
                next.manufacturer_file = value.clone();
            }
            if let Some(value) = &self.catalog {
                next.catalog = value.clone();
            }
            if let Some(value) = &self.edition_profile {
                next.edition_profile = value.clone();
            }
            if let Some(value) = &self.correction_as_of {
                next.correction_as_of = *value;
            }
            if let Some(value) = &self.strict_mode {
                next.strict_mode = *value;
            }
            if let Some(value) = &self.index {
                next.index = value.clone();
            }
            if let Some(value) = &self.geometry {
                next.geometry = value.clone();
            }
            if let Some(value) = &self.curves {
                next.curves = value.clone();
            }
            if let Some(value) = &self.limits {
                next.limits = *value;
            }
            next
        })
    }
}

impl MutationDiff<Vdi3805Snapshot> for Vdi3805Diff {
    fn apply(&self, snapshot: &Vdi3805Snapshot) -> protocol::MutationApplyResult<Vdi3805Snapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(value) = &self.manufacturer_file {
                next.manufacturer_file = value.clone();
            }
            if let Some(value) = &self.catalog {
                next.catalog = value.clone();
            }
            if let Some(value) = &self.edition_profile {
                next.edition_profile = value.clone();
            }
            if let Some(value) = &self.correction_as_of {
                next.correction_as_of = *value;
            }
            if let Some(value) = &self.strict_mode {
                next.strict_mode = *value;
            }
            if let Some(value) = &self.index {
                next.index = value.clone();
            }
            if let Some(value) = &self.geometry {
                next.geometry = value.clone();
            }
            if let Some(value) = &self.curves {
                next.curves = value.clone();
            }
            if let Some(value) = &self.limits {
                next.limits = *value;
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(manufacturer_file);
        take!(catalog);
        take!(edition_profile);
        take!(correction_as_of);
        take!(strict_mode);
        take!(index);
        take!(geometry);
        take!(curves);
        take!(limits);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &Vdi3805Snapshot) -> Vdi3805Diff {
    Vdi3805Diff { artifact: Some(Box::new(Vdi3805Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Vdi3805DiffText = String;
//#endregion 🚚️Carrier
