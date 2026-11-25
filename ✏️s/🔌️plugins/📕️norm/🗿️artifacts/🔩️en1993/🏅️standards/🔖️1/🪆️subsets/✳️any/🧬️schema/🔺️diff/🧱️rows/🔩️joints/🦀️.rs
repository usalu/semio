//! 🧱️ EN 1993 diff rows (joints): the sparse field patch, key extractor and keyed row delta of 7 document row lists.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `SteelJoint`.
    pub En1993JointPatch of semio_s_artifact_norm_en1993_model::SteelJoint { set { kind: semio_s_artifact_norm_en1993_model::JointKind, member_id: String, bolt_class: String, bolt_diameter: f64, bolt_rows: u32, bolts_per_row: u32, pitch: f64, gauge: f64, end_distance: f64, edge_distance: f64, shear_planes: u32, plate_thickness: f64, plate_fu: f64, weld_throat: f64, weld_length: f64, weld_fu: f64, weld_grade: String, actions: Vec<semio_s_artifact_norm_en1993_model::JointForceAction>, category: semio_s_artifact_norm_en1993_model::JointCategory, friction_mu: f64, preload_force: f64, slip_factor_ks: f64, friction_surfaces: u32 , plate_fy: f64, plate_width: f64, bolt_to_web: f64, extension_to_flange: f64, extension_edge: f64, alpha_factor: f64, bolt_stress_area: f64, bolt_grip_length: f64, column_flange_thickness: f64, column_web_thickness: f64, column_web_depth: f64, column_web_shear_area: f64, column_root_radius: f64, column_fy: f64, column_web_stress: f64, beam_flange_thickness: f64, compression_lever: f64, moment_demand: f64, beam_moment_resistance: f64, column_moment_resistance: f64, beam_inertia: f64, beam_span: f64, braced_frame: u32, stiffness_ratio: f64, transformation_parameter: f64, assumed_class: semio_s_artifact_norm_en1993_model::StiffnessClass, base_plate_length: f64, concrete_fcd: f64, concrete_modulus: f64, concrete_area_a0: f64, concrete_area_a1: f64, joint_coefficient: f64, anchor_area: f64, anchor_length: f64, base_normal_force: f64, base_moment: f64, base_lever_left: f64, base_lever_right: f64, beam_web_thickness: f64, beam_web_fy: f64, weld_force_longitudinal: f64, weld_force_transverse: f64} }
}

/// 🔑️ Key extractor of the foreign `SteelJoint` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993JointDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::SteelJoint> for En1993JointDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::SteelJoint) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `SteelJoint` list.
    pub En1993JointDelta { removal: En1993JointRemoval, insertion: En1993JointInsertion, relocation: En1993JointRelocation, modification: En1993JointModification, row: semio_s_artifact_norm_en1993_model::SteelJoint, patch: En1993JointPatch, list: Vec<semio_s_artifact_norm_en1993_model::SteelJoint>, key: String = by En1993JointDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WeldGroup`.
    pub En1993WeldGroupPatch of semio_s_artifact_norm_en1993_model::WeldGroup { set { member_id: String, label: String, pattern: semio_s_artifact_norm_en1993_model::WeldPattern, throat: f64, width: f64, depth: f64, shear_force: f64, eccentricity: f64, steel_fu: f64, beta_w: f64} }
}

/// 🔑️ Key extractor of the foreign `WeldGroup` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993WeldGroupDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::WeldGroup> for En1993WeldGroupDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::WeldGroup) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `WeldGroup` list.
    pub En1993WeldGroupDelta { removal: En1993WeldGroupRemoval, insertion: En1993WeldGroupInsertion, relocation: En1993WeldGroupRelocation, modification: En1993WeldGroupModification, row: semio_s_artifact_norm_en1993_model::WeldGroup, patch: En1993WeldGroupPatch, list: Vec<semio_s_artifact_norm_en1993_model::WeldGroup>, key: String = by En1993WeldGroupDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ParametricSection`.
    pub En1993ParametricSectionPatch of semio_s_artifact_norm_en1993_model::ParametricSection { set { section_id: String, label: String, shape: semio_s_artifact_norm_en1993_model::ParametricShape, height: f64, width: f64, web_thickness: f64, flange_thickness: f64, tolerance: f64} }
}

/// 🔑️ Key extractor of the foreign `ParametricSection` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993ParametricSectionDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::ParametricSection> for En1993ParametricSectionDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::ParametricSection) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `ParametricSection` list.
    pub En1993ParametricSectionDelta { removal: En1993ParametricSectionRemoval, insertion: En1993ParametricSectionInsertion, relocation: En1993ParametricSectionRelocation, modification: En1993ParametricSectionModification, row: semio_s_artifact_norm_en1993_model::ParametricSection, patch: En1993ParametricSectionPatch, list: Vec<semio_s_artifact_norm_en1993_model::ParametricSection>, key: String = by En1993ParametricSectionDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `HollowJoint`.
    pub En1993HollowJointPatch of semio_s_artifact_norm_en1993_model::HollowJoint { set { member_id: String, label: String, chord_width: f64, chord_height: f64, chord_thickness: f64, chord_fy: f64, chord_stress: f64, brace1_width: f64, brace1_height: f64, brace1_thickness: f64, brace1_angle: f64, brace1_force: f64, brace2_width: f64, brace2_height: f64, brace2_thickness: f64, brace2_angle: f64, brace2_force: f64, gap: f64, brace_fy: f64} }
}

/// 🔑️ Key extractor of the foreign `HollowJoint` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993HollowJointDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::HollowJoint> for En1993HollowJointDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::HollowJoint) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `HollowJoint` list.
    pub En1993HollowJointDelta { removal: En1993HollowJointRemoval, insertion: En1993HollowJointInsertion, relocation: En1993HollowJointRelocation, modification: En1993HollowJointModification, row: semio_s_artifact_norm_en1993_model::HollowJoint, patch: En1993HollowJointPatch, list: Vec<semio_s_artifact_norm_en1993_model::HollowJoint>, key: String = by En1993HollowJointDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `JointRow`.
    pub En1993JointRowPatch of semio_s_artifact_norm_en1993_model::JointRow { set { joint_id: String, label: String, row_type: semio_s_artifact_norm_en1993_model::JointRowType, lever: f64, bolt_to_web: f64, edge_distance: f64, extension_to_flange: f64, extension_edge: f64, alpha_factor: f64, pitch_to_previous: f64, bolts: u32, bolt_diameter: f64, bolt_stress_area: f64, grip_length: f64, bolt_class: String, group_with_previous: u32} }
}

/// 🔑️ Key extractor of the foreign `JointRow` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993JointRowDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::JointRow> for En1993JointRowDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::JointRow) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `JointRow` list.
    pub En1993JointRowDelta { removal: En1993JointRowRemoval, insertion: En1993JointRowInsertion, relocation: En1993JointRowRelocation, modification: En1993JointRowModification, row: semio_s_artifact_norm_en1993_model::JointRow, patch: En1993JointRowPatch, list: Vec<semio_s_artifact_norm_en1993_model::JointRow>, key: String = by En1993JointRowDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `JointPlate`.
    pub En1993JointPlatePatch of semio_s_artifact_norm_en1993_model::JointPlate { set { joint_id: String, label: String, role: String, plate_height: f64, plate_width: f64, plate_thickness: f64, plate_count: u32, plate_fy: f64, plate_fu: f64, bolt_class: String, bolt_diameter: f64, hole_diameter: f64, bolt_rows: u32, bolts_per_row: u32, pitch: f64, gauge: f64, end_distance: f64, edge_distance: f64, shear_planes: u32, eccentricity: f64, weld_throat: f64, weld_fu: f64, weld_beta_w: f64, group_method: semio_s_artifact_norm_en1993_model::BoltGroupMethod} }
}

/// 🔑️ Key extractor of the foreign rows of `En1993JointPlateDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993JointPlateDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::JointPlate> for En1993JointPlateDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::JointPlate) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `JointPlate` list.
    pub En1993JointPlateDelta { removal: En1993JointPlateRemoval, insertion: En1993JointPlateInsertion, relocation: En1993JointPlateRelocation, modification: En1993JointPlateModification, row: semio_s_artifact_norm_en1993_model::JointPlate, patch: En1993JointPlatePatch, list: Vec<semio_s_artifact_norm_en1993_model::JointPlate>, key: String = by En1993JointPlateDeltaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FastenedJoint`.
    pub En1993FastenedJointPatch of semio_s_artifact_norm_en1993_model::FastenedJoint { set { member_id: String, kind: semio_s_artifact_norm_en1993_model::FastenerKind, diameter: f64, washer_diameter: f64, count: u32, per_row: u32, head_thickness: f64, base_thickness: f64, head_strength: f64, base_strength: f64, end_distance: f64, edge_distance: f64, pitch: f64, gauge: f64, sheet_width: f64, shear_force: f64, tension_force: f64, shear_resistance: f64 } }
}

/// 🔑️ Key extractor of the foreign rows of `En1993FastenedJointDelta` (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct En1993FastenedJointDeltaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1993_model::FastenedJoint> for En1993FastenedJointDeltaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1993_model::FastenedJoint) -> String {
        row.id.clone()
    }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `FastenedJoint` list.
    pub En1993FastenedJointDelta { removal: En1993FastenedJointRemoval, insertion: En1993FastenedJointInsertion, relocation: En1993FastenedJointRelocation, modification: En1993FastenedJointModification, row: semio_s_artifact_norm_en1993_model::FastenedJoint, patch: En1993FastenedJointPatch, list: Vec<semio_s_artifact_norm_en1993_model::FastenedJoint>, key: String = by En1993FastenedJointDeltaKey }
}
