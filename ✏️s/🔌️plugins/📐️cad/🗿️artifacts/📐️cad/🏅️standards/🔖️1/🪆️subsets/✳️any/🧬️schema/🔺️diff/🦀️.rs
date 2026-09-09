//! 🧬️ Cad diff schema — sparse field delta over the artifact.

use crate::mutations::CadNodePatch;
use crate::{CadDrawingChild, CadModelChild, CadNode, CadReferenceList};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the cad artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::CadArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub shape_model: Option<Option<CadModelChild>>,
    #[state(artifact)]
    pub building_model: Option<Option<CadModelChild>>,
    #[state(artifact)]
    pub energy_model: Option<Option<CadModelChild>>,
    #[state(artifact)]
    pub structure_classic_model: Option<Option<CadModelChild>>,
    #[state(artifact)]
    pub drawings: Option<CadDrawingChildList>,
    #[state(artifact)]
    pub references_by_model_definition_id: Option<BTreeMap<String, CadReferenceList>>,
    #[state(artifact)]
    pub nodes: Option<CadNodesDelta>,
    #[state(artifact)]
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CadStringList {
    pub values: Vec<String>,
}

/// 🧩️ Whole-list wrapper for the `drawings` composed CHILD COLLECTION diff field — same `RunList`
/// shape `✳️text`/`✳️kit` use for their own Vec-of-child diff fields (kit's own
/// `SemioKitModelChildList` is the direct precedent for a `Vec<ArtifactChild<S>>` diff wrapper).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CadDrawingChildList {
    pub values: Vec<CadDrawingChild>,
}

/// 🧩 Identified-collection delta for nodes.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CadNodesDelta {
    pub added: Vec<CadNode>,
    pub removed: Vec<String>,
    pub patched: Vec<CadNodePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched node entry.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CadNodePatchEntry {
    pub id: String,
    pub patch: CadNodePatch,
}
//#endregion 🔖️DeltaHelpers
