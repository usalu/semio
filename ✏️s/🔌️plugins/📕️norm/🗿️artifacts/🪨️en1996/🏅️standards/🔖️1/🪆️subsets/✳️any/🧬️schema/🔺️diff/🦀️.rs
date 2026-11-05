//! 🧬️ EN1996 diff schema — sparse scalar fields plus positional row deltas for every list the document owns.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;
use protocol::{ColdDiffAlgebra, DiffAlgebra, MutationDiff};

use semio_s_artifact_norm_en1996_model::En1996Snapshot;

//#region 🔖️Rows
/// 🔑️ Key extractor of the foreign `WallOpening` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallOpeningKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::WallOpening> for WallOpeningKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::WallOpening) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `ConcentratedLoad` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConcentratedLoadKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::ConcentratedLoad> for ConcentratedLoadKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::ConcentratedLoad) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `WallLoadCase` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallLoadCaseKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::WallLoadCase> for WallLoadCaseKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::WallLoadCase) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `MasonryWall` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MasonryWallKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::MasonryWall> for MasonryWallKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::MasonryWall) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `Placement` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlacementKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::Placement> for PlacementKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::Placement) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `MovementJoint` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovementJointKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::MovementJoint> for MovementJointKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::MovementJoint) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `WallChase` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallChaseKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::WallChase> for WallChaseKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::WallChase) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `WallPier` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallPierKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::WallPier> for WallPierKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::WallPier) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `WallLintel` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallLintelKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::WallLintel> for WallLintelKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::WallLintel) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `WallTie` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallTieKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::WallTie> for WallTieKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::WallTie) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `WallPrestress` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallPrestressKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::WallPrestress> for WallPrestressKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::WallPrestress) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `ConfinedElement` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConfinedElementKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::ConfinedElement> for ConfinedElementKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::ConfinedElement) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `ShearFlange` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShearFlangeKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::ShearFlange> for ShearFlangeKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::ShearFlange) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `WallBar` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallBarKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1996_model::WallBar> for WallBarKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1996_model::WallBar) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WallOpening`.
    pub En1996OpeningPatch of semio_s_artifact_norm_en1996_model::WallOpening { set { width_m: f64, height_m: f64, sill_height_m: f64, position_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `WallOpening` list.
    pub En1996OpeningDelta { removal: En1996OpeningRemoval, insertion: En1996OpeningInsertion, relocation: En1996OpeningRelocation, modification: En1996OpeningModification, row: semio_s_artifact_norm_en1996_model::WallOpening, patch: En1996OpeningPatch, list: Vec<semio_s_artifact_norm_en1996_model::WallOpening>, key: String = by WallOpeningKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ConcentratedLoad`.
    pub En1996ConcentratedPatch of semio_s_artifact_norm_en1996_model::ConcentratedLoad { set { force_n: f64, bearing_area_m2: f64, bearing_length_m: f64, distance_a1_m: f64, position_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `ConcentratedLoad` list.
    pub En1996ConcentratedDelta { removal: En1996ConcentratedRemoval, insertion: En1996ConcentratedInsertion, relocation: En1996ConcentratedRelocation, modification: En1996ConcentratedModification, row: semio_s_artifact_norm_en1996_model::ConcentratedLoad, patch: En1996ConcentratedPatch, list: Vec<semio_s_artifact_norm_en1996_model::ConcentratedLoad>, key: String = by ConcentratedLoadKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WallLoadCase`.
    pub En1996LoadCasePatch of semio_s_artifact_norm_en1996_model::WallLoadCase { set { design_situation: semio_s_artifact_norm_en1996_model::LoadCaseSituation, imposed_category: semio_s_artifact_norm_en1996_model::ImposedCategory, g_k_slab_n: f64, q_k_imposed_pa: f64, tributary_area_m2: f64, slab_span_m: f64, q_k_snow_pa: f64, q_p_wind_pa: f64, c_pe: f64, h_k_earth_n: f64 } nest { concentrated: En1996ConcentratedDelta } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `WallLoadCase` list.
    pub En1996LoadCaseDelta { removal: En1996LoadCaseRemoval, insertion: En1996LoadCaseInsertion, relocation: En1996LoadCaseRelocation, modification: En1996LoadCaseModification, row: semio_s_artifact_norm_en1996_model::WallLoadCase, patch: En1996LoadCasePatch, list: Vec<semio_s_artifact_norm_en1996_model::WallLoadCase>, key: String = by WallLoadCaseKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `MasonryWall`.
    pub En1996WallPatch of semio_s_artifact_norm_en1996_model::MasonryWall { set { label_en: String, label_de: String, wall_type: semio_s_artifact_norm_en1996_model::WallType, thickness_m: f64, height_m: f64, length_m: f64, support_sides: u8, slab_bearing_depth_m: f64, eccentricity_top_m: f64, eccentricity_bottom_m: f64, unit_group: semio_s_artifact_norm_en1996_model::UnitGroup, unit_material: semio_s_artifact_norm_en1996_model::UnitMaterial, f_b_pa: f64, unit_length_m: f64, unit_width_m: f64, unit_height_m: f64, mortar_type: semio_s_artifact_norm_en1996_model::MortarType, mortar_class: semio_s_artifact_norm_en1996_model::MortarClass, mortar_strength_pa: f64, bed_joint_thickness_m: f64, reinforced: bool, as_vertical_m2: f64, as_horizontal_m2: f64, f_yd_pa: f64, fire_rei_min: u32, exposure: semio_s_artifact_norm_en1996_model::ExposureClass, mu: f64, density_kg_m3: f64, phi_infinity: f64, is_basement: bool, frost_class: semio_s_artifact_norm_en1996_model::FrostClass, cover_m: f64, deflection_limit: f64, soil_height_m: f64, soil_density_kg_m3: f64, isotherm_depth_m: f64, plan_x_m: f64, plan_y_m: f64, plan_angle_deg: f64, in_plane_shear_n: f64, in_plane_moment_nm: f64, bracing_system: semio_s_artifact_norm_en1996_model::BracingSystem, diaphragm_stiffness_n_per_m: f64 , web_spacing_m: f64} nest { openings: En1996OpeningDelta, load_cases: En1996LoadCaseDelta, piers: En1996PierDelta, lintels: En1996LintelDelta, ties: En1996TieDelta, prestress: En1996PrestressDelta, confined: En1996ConfinedDelta, flanges: En1996FlangeDelta, bars: En1996BarDelta } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Placement`.
    pub En1996PlacementPatch of semio_s_artifact_norm_en1996_model::Placement { set { x: f64, y: f64, z: f64, yaw: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `MasonryWall` list.
    pub En1996WallDelta { removal: En1996WallRemoval, insertion: En1996WallInsertion, relocation: En1996WallRelocation, modification: En1996WallModification, row: semio_s_artifact_norm_en1996_model::MasonryWall, patch: En1996WallPatch, list: Vec<semio_s_artifact_norm_en1996_model::MasonryWall>, key: String = by MasonryWallKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Placement` list.
    pub En1996PlacementDelta { removal: En1996PlacementRemoval, insertion: En1996PlacementInsertion, relocation: En1996PlacementRelocation, modification: En1996PlacementModification, row: semio_s_artifact_norm_en1996_model::Placement, patch: En1996PlacementPatch, list: Vec<semio_s_artifact_norm_en1996_model::Placement>, key: String = by PlacementKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WallChase`.
    pub En1996ChasePatch of semio_s_artifact_norm_en1996_model::WallChase { set { wall_id: String, kind: semio_s_artifact_norm_en1996_model::ChaseKind, x_m: f64, y_m: f64, width_m: f64, length_m: f64, depth_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `WallChase` list.
    pub En1996ChaseDelta { removal: En1996ChaseRemoval, insertion: En1996ChaseInsertion, relocation: En1996ChaseRelocation, modification: En1996ChaseModification, row: semio_s_artifact_norm_en1996_model::WallChase, patch: En1996ChasePatch, list: Vec<semio_s_artifact_norm_en1996_model::WallChase>, key: String = by WallChaseKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `MovementJoint`.
    pub En1996JointPatch of semio_s_artifact_norm_en1996_model::MovementJoint { set { wall_id: String, position_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `MovementJoint` list.
    pub En1996JointDelta { removal: En1996JointRemoval, insertion: En1996JointInsertion, relocation: En1996JointRelocation, modification: En1996JointModification, row: semio_s_artifact_norm_en1996_model::MovementJoint, patch: En1996JointPatch, list: Vec<semio_s_artifact_norm_en1996_model::MovementJoint>, key: String = by MovementJointKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WallPier`.
    pub En1996PierPatch of semio_s_artifact_norm_en1996_model::WallPier { set { position_m: f64, width_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `WallPier` list.
    pub En1996PierDelta { removal: En1996PierRemoval, insertion: En1996PierInsertion, relocation: En1996PierRelocation, modification: En1996PierModification, row: semio_s_artifact_norm_en1996_model::WallPier, patch: En1996PierPatch, list: Vec<semio_s_artifact_norm_en1996_model::WallPier>, key: String = by WallPierKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WallLintel`.
    pub En1996LintelPatch of semio_s_artifact_norm_en1996_model::WallLintel { set { position_m: f64, span_m: f64, depth_m: f64, bearing_length_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `WallLintel` list.
    pub En1996LintelDelta { removal: En1996LintelRemoval, insertion: En1996LintelInsertion, relocation: En1996LintelRelocation, modification: En1996LintelModification, row: semio_s_artifact_norm_en1996_model::WallLintel, patch: En1996LintelPatch, list: Vec<semio_s_artifact_norm_en1996_model::WallLintel>, key: String = by WallLintelKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WallTie`.
    pub En1996TiePatch of semio_s_artifact_norm_en1996_model::WallTie { set { kind: semio_s_artifact_norm_en1996_model::TieKind, position_m: f64, level_m: f64, capacity_n: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `WallTie` list.
    pub En1996TieDelta { removal: En1996TieRemoval, insertion: En1996TieInsertion, relocation: En1996TieRelocation, modification: En1996TieModification, row: semio_s_artifact_norm_en1996_model::WallTie, patch: En1996TiePatch, list: Vec<semio_s_artifact_norm_en1996_model::WallTie>, key: String = by WallTieKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WallPrestress`.
    pub En1996PrestressPatch of semio_s_artifact_norm_en1996_model::WallPrestress { set { force_n: f64, area_m2: f64, f_pk_pa: f64, position_m: f64, eccentricity_m: f64, loss_ratio: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `WallPrestress` list.
    pub En1996PrestressDelta { removal: En1996PrestressRemoval, insertion: En1996PrestressInsertion, relocation: En1996PrestressRelocation, modification: En1996PrestressModification, row: semio_s_artifact_norm_en1996_model::WallPrestress, patch: En1996PrestressPatch, list: Vec<semio_s_artifact_norm_en1996_model::WallPrestress>, key: String = by WallPrestressKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ConfinedElement`.
    pub En1996ConfinedPatch of semio_s_artifact_norm_en1996_model::ConfinedElement { set { position_m: f64, width_m: f64, depth_m: f64, as_m2: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `ConfinedElement` list.
    pub En1996ConfinedDelta { removal: En1996ConfinedRemoval, insertion: En1996ConfinedInsertion, relocation: En1996ConfinedRelocation, modification: En1996ConfinedModification, row: semio_s_artifact_norm_en1996_model::ConfinedElement, patch: En1996ConfinedPatch, list: Vec<semio_s_artifact_norm_en1996_model::ConfinedElement>, key: String = by ConfinedElementKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ShearFlange`.
    pub En1996FlangePatch of semio_s_artifact_norm_en1996_model::ShearFlange { set { length_m: f64, thickness_m: f64, position_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `ShearFlange` list.
    pub En1996FlangeDelta { removal: En1996FlangeRemoval, insertion: En1996FlangeInsertion, relocation: En1996FlangeRelocation, modification: En1996FlangeModification, row: semio_s_artifact_norm_en1996_model::ShearFlange, patch: En1996FlangePatch, list: Vec<semio_s_artifact_norm_en1996_model::ShearFlange>, key: String = by ShearFlangeKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WallBar`.
    pub En1996BarPatch of semio_s_artifact_norm_en1996_model::WallBar { set { kind: semio_s_artifact_norm_en1996_model::BarKind, diameter_m: f64, length_m: f64, spacing_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `WallBar` list.
    pub En1996BarDelta { removal: En1996BarRemoval, insertion: En1996BarInsertion, relocation: En1996BarRelocation, modification: En1996BarModification, row: semio_s_artifact_norm_en1996_model::WallBar, patch: En1996BarPatch, list: Vec<semio_s_artifact_norm_en1996_model::WallBar>, key: String = by WallBarKey }
}

//#endregion 🔖️Rows

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1996 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1996")]
pub struct En1996Diff {
    #[state(artifact)]
    pub annex: Option<semio_s_plugin_norm_compliance::AnnexChoice>,
    #[state(artifact)]
    pub masonry_class: Option<semio_s_artifact_norm_en1996_model::MasonryClass>,
    #[state(artifact)]
    pub design_situation: Option<semio_s_plugin_norm_compliance::DesignSituation>,
    #[state(artifact)]
    pub storeys: Option<u32>,
    #[state(artifact)]
    pub walls: En1996WallDelta,
    #[state(artifact)]
    pub placements: En1996PlacementDelta,
    #[state(artifact)]
    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,
    #[state(artifact)]
    pub chases: En1996ChaseDelta,
    #[state(artifact)]
    pub movement_joints: En1996JointDelta,
    #[state(artifact)]
    pub reliability_class: Option<semio_s_plugin_norm_compliance::ReliabilityClass>,
}
//#endregion 🔖️Diff

impl MutationDiff<En1996Snapshot> for En1996Diff {}

impl protocol::ColdMutationDiff<En1996Snapshot> for En1996Diff {
    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.reliability_class.is_some() {
            self.reliability_class = other.reliability_class;
        }
        if other.masonry_class.is_some() {
            self.masonry_class = other.masonry_class;
        }
        if other.design_situation.is_some() {
            self.design_situation = other.design_situation;
        }
        if other.storeys.is_some() {
            self.storeys = other.storeys;
        }
        self.walls.absorb(other.walls);
        self.placements.absorb(other.placements);
        self.cases.absorb(other.cases);
        self.chases.absorb(other.chases);
        self.movement_joints.absorb(other.movement_joints);
    }

    fn apply(&self, unpublished: &mut En1996Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        *unpublished = En1996Snapshot {
            annex: self.annex.unwrap_or(base.annex),
            reliability_class: self.reliability_class.unwrap_or(base.reliability_class),
            masonry_class: self.masonry_class.unwrap_or(base.masonry_class),
            design_situation: self.design_situation.unwrap_or(base.design_situation),
            storeys: self.storeys.unwrap_or(base.storeys),
            walls: self.walls.commit_onto(&base.walls, capability).map_err(|error| error.under(["walls"]))?,
            placements: self.placements.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?,
            cases: self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?,
            chases: self.chases.commit_onto(&base.chases, capability).map_err(|error| error.under(["chases"]))?,
            movement_joints: self.movement_joints.commit_onto(&base.movement_joints, capability).map_err(|error| error.under(["movementJoints"]))?,
        };
        Ok(())
    }
}

impl DiffAlgebra<En1996Snapshot> for En1996Diff {
    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.reliability_class.is_none()
            && self.masonry_class.is_none()
            && self.design_situation.is_none()
            && self.storeys.is_none()
            && self.walls.is_empty()
            && self.placements.is_empty()
            && self.cases.is_empty()
            && self.chases.is_empty()
            && self.movement_joints.is_empty()
    }
}

impl protocol::ColdDiffAlgebra<En1996Snapshot> for En1996Diff {
    fn inverse(&self, base: &En1996Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex),
            reliability_class: self.reliability_class.as_ref().map(|_| base.reliability_class),
            masonry_class: self.masonry_class.as_ref().map(|_| base.masonry_class),
            design_situation: self.design_situation.as_ref().map(|_| base.design_situation),
            storeys: self.storeys.as_ref().map(|_| base.storeys),
            walls: self.walls.inverse(&base.walls),
            placements: self.placements.inverse(&base.placements),
            cases: self.cases.inverse(&base.cases),
            chases: self.chases.inverse(&base.chases),
            movement_joints: self.movement_joints.inverse(&base.movement_joints),
        }
    }
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1996_diff_typescript::LEAF, graphql: semio_s_artifact_norm_en1996_diff_graphql::LEAF, json_schema: semio_s_artifact_norm_en1996_diff_json_schema::LEAF, proto: semio_s_artifact_norm_en1996_diff_proto::LEAF };
//#endregion 🍃️FacetLeaves
