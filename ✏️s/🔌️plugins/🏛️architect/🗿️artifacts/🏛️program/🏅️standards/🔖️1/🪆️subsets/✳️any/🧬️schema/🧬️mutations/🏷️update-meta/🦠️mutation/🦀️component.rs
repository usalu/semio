//! 🦠️ ProgramSnapshot mutation — `update_meta` leaf: `RenameMeta`/`ReplaceMeta`. `ProgramMeta`
//! is a document-level scalar facet (`program.meta: ProgramMeta`, no id-keyed collection) per
//! `📓️derivation-rules.md` rule 1: `rename-<artifact>` for the identity-like field (`title`),
//! `replace-<facet>` for the rest as one whole-value swap (`program.meta`'s 15 fields are a rich,
//! independently-editable document, not an "inseparable ≥2-field facet" — full per-field
//! `change-meta-<field>` decomposition is a fair follow-up, noted in the wave2 report, not blocking).
//! Supersedes the banned raw-Patch-payload `UpdateMeta { patch: ProgramMetaPatch }`.

use crate::artifacts::program::registers::ProgramMeta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️RenameMeta
/// ✏️ Sets `program.meta.title`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameMeta {
    pub new_title: String,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for RenameMeta {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "meta", kind: "rename-meta", record: "RenamedMeta" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_rename(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_rename(self, base)
    }
    fn label(&self) -> String {
        format!("Rename document to \"{}\"", self.new_title)
    }
}
//#endregion 🔖️RenameMeta

//#region 🔖️ReplaceMeta
/// 🔁️ Whole-value swap of `program.meta`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceMeta {
    pub new_meta: ProgramMeta,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ReplaceMeta {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "meta", kind: "replace-meta", record: "ReplacedMeta" };
    fn diff(&self, base: &ProgramSnapshot) -> ProgramDiff {
        super::diff::diff_replace(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        super::inverse::inverse_replace(self, base)
    }
    fn label(&self) -> String {
        format!("Replace document metadata \"{}\"", self.new_meta.title)
    }
}
//#endregion 🔖️ReplaceMeta
