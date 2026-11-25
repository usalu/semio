//! 🧱️ EN 1993 diff rows (frame): the sparse field patch, key extractor and keyed row delta of 7 document row lists.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FrameNode`.
    pub En1993FrameNodePatch of semio_s_artifact_norm_en1993_model::FrameNode { set { label: String, x: f64, z: f64, support: semio_s_artifact_norm_en1993_model::FrameNodeSupport, load_x: f64, load_z: f64} }
}

/// 🔑️ Key extractor of the foreign `FrameNode` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FrameNodeDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FrameNode> for En1993FrameNodeDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FrameNode) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FrameNode` list.
    pub En1993FrameNodeDelta { removal: En1993FrameNodeRemoval, insertion: En1993FrameNodeInsertion, relocation: En1993FrameNodeRelocation, modification: En1993FrameNodeModification, row: semio_s_artifact_norm_en1993_model::FrameNode, patch: En1993FrameNodePatch, list: Vec<semio_s_artifact_norm_en1993_model::FrameNode>, key: String = by En1993FrameNodeDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FrameBar`.
    pub En1993FrameBarPatch of semio_s_artifact_norm_en1993_model::FrameBar { set { member_id: String, label: String, start_node_id: String, end_node_id: String, release_start: semio_s_artifact_norm_en1993_model::FrameBarRelease, release_end: semio_s_artifact_norm_en1993_model::FrameBarRelease} }
}

/// 🔑️ Key extractor of the foreign `FrameBar` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FrameBarDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FrameBar> for En1993FrameBarDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FrameBar) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FrameBar` list.
    pub En1993FrameBarDelta { removal: En1993FrameBarRemoval, insertion: En1993FrameBarInsertion, relocation: En1993FrameBarRelocation, modification: En1993FrameBarModification, row: semio_s_artifact_norm_en1993_model::FrameBar, patch: En1993FrameBarPatch, list: Vec<semio_s_artifact_norm_en1993_model::FrameBar>, key: String = by En1993FrameBarDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `MaterialDuctility`.
    pub En1993MaterialDuctilityPatch of semio_s_artifact_norm_en1993_model::MaterialDuctility { set { material_id: String, elongation: f64, ultimate_strain: f64 } }
}

/// 🔑️ Key extractor of the foreign rows of `En1993MaterialDuctilityDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993MaterialDuctilityDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::MaterialDuctility> for En1993MaterialDuctilityDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::MaterialDuctility) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `MaterialDuctility` list.
    pub En1993MaterialDuctilityDelta { removal: En1993MaterialDuctilityRemoval, insertion: En1993MaterialDuctilityInsertion, relocation: En1993MaterialDuctilityRelocation, modification: En1993MaterialDuctilityModification, row: semio_s_artifact_norm_en1993_model::MaterialDuctility, patch: En1993MaterialDuctilityPatch, list: Vec<semio_s_artifact_norm_en1993_model::MaterialDuctility>, key: String = by En1993MaterialDuctilityDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FormingGain`.
    pub En1993FormingGainPatch of semio_s_artifact_norm_en1993_model::FormingGain { set { member_id: String, forming: semio_s_artifact_norm_en1993_model::FormingProcess, ultimate_strength: f64, gross_area: f64, bends: u32 , used_yield: f64} }
}

/// 🔑️ Key extractor of the foreign rows of `En1993FormingGainDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FormingGainDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FormingGain> for En1993FormingGainDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FormingGain) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FormingGain` list.
    pub En1993FormingGainDelta { removal: En1993FormingGainRemoval, insertion: En1993FormingGainInsertion, relocation: En1993FormingGainRelocation, modification: En1993FormingGainModification, row: semio_s_artifact_norm_en1993_model::FormingGain, patch: En1993FormingGainPatch, list: Vec<semio_s_artifact_norm_en1993_model::FormingGain>, key: String = by En1993FormingGainDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `BucklingLength`.
    pub En1993BucklingLengthPatch of semio_s_artifact_norm_en1993_model::BucklingLength { set { member_id: String, plane: semio_s_artifact_norm_en1993_model::BucklingPlane, sway: semio_s_artifact_norm_en1993_model::FrameSway, restraint_a: semio_s_artifact_norm_en1993_model::NodeARestraint, column_stiffness_a: f64, beam_stiffness_a: f64, restraint_b: semio_s_artifact_norm_en1993_model::NodeBRestraint, column_stiffness_b: f64, beam_stiffness_b: f64 } }
}

/// 🔑️ Key extractor of the foreign rows of `En1993BucklingLengthDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993BucklingLengthDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::BucklingLength> for En1993BucklingLengthDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::BucklingLength) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `BucklingLength` list.
    pub En1993BucklingLengthDelta { removal: En1993BucklingLengthRemoval, insertion: En1993BucklingLengthInsertion, relocation: En1993BucklingLengthRelocation, modification: En1993BucklingLengthModification, row: semio_s_artifact_norm_en1993_model::BucklingLength, patch: En1993BucklingLengthPatch, list: Vec<semio_s_artifact_norm_en1993_model::BucklingLength>, key: String = by En1993BucklingLengthDeltaKey }
}

//#endregion 🔖️Rows

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FrameMemberBasis`.
    pub En1993FrameMemberBasisPatch of semio_s_artifact_norm_en1993_model::FrameMemberBasis { set { member_id: String, basis: semio_s_artifact_norm_en1993_model::MemberForceBasis } }
}

/// 🔑️ Key extractor of the foreign rows of `En1993FrameMemberBasisDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FrameMemberBasisDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FrameMemberBasis> for En1993FrameMemberBasisDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FrameMemberBasis) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FrameMemberBasis` list.
    pub En1993FrameMemberBasisDelta { removal: En1993FrameMemberBasisRemoval, insertion: En1993FrameMemberBasisInsertion, relocation: En1993FrameMemberBasisRelocation, modification: En1993FrameMemberBasisModification, row: semio_s_artifact_norm_en1993_model::FrameMemberBasis, patch: En1993FrameMemberBasisPatch, list: Vec<semio_s_artifact_norm_en1993_model::FrameMemberBasis>, key: String = by En1993FrameMemberBasisDeltaKey }
}

//#endregion 🔖️Rows

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FrameAction`.
    pub En1993FrameActionPatch of semio_s_artifact_norm_en1993_model::FrameAction { set { load_case_id: String, target_id: String, kind: semio_s_artifact_norm_en1993_model::FrameActionKind, fx: f64, fz: f64, my: f64, start_ratio: f64, end_ratio: f64 } }
}

/// 🔑️ Key extractor of the foreign rows of `En1993FrameActionDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FrameActionDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FrameAction> for En1993FrameActionDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FrameAction) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FrameAction` list.
    pub En1993FrameActionDelta { removal: En1993FrameActionRemoval, insertion: En1993FrameActionInsertion, relocation: En1993FrameActionRelocation, modification: En1993FrameActionModification, row: semio_s_artifact_norm_en1993_model::FrameAction, patch: En1993FrameActionPatch, list: Vec<semio_s_artifact_norm_en1993_model::FrameAction>, key: String = by En1993FrameActionDeltaKey }
}
