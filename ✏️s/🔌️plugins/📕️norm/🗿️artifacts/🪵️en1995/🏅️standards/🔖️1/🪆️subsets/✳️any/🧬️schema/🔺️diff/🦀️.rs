//! 🧬️ EN1995 diff schema — sparse scalar fields plus positional row deltas for every list the document owns.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;
use protocol::{ColdDiffAlgebra, DiffAlgebra, MutationDiff};

use semio_s_artifact_norm_en1995_model::En1995Snapshot;

//#region 🔖️Rows
/// 🔑️ Key extractor of the foreign `CharacteristicAction` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CharacteristicActionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::CharacteristicAction> for CharacteristicActionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::CharacteristicAction) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `ConnectionAction` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConnectionActionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::ConnectionAction> for ConnectionActionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::ConnectionAction) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `TimberMember` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimberMemberKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::TimberMember> for TimberMemberKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::TimberMember) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `TimberConnection` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimberConnectionKey;

/// 🔑️ Key extractor of the foreign `Placement` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlacementKey;

/// 🔑️ Key extractor of the foreign `PanelLayer` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelLayerKey;

/// 🔑️ Key extractor of the foreign `FastenerPosition` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FastenerPositionKey;

/// 🔑️ Key extractor of the foreign `Taper` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TaperKey;

/// 🔑️ Key extractor of the foreign `Cutout` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CutoutKey;

/// 🔑️ Key extractor of the foreign `Fastener` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FastenerKey;

/// 🔑️ Key extractor of the foreign `Panel` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanelKey;

/// 🔑️ Key extractor of the foreign `Floor` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FloorKey;

/// 🔑️ Key extractor of the foreign `Diaphragm` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DiaphragmKey;

/// 🔑️ Key extractor of the foreign `Truss` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrussKey;

/// 🔑️ Key extractor of the foreign `Bracing` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BracingKey;

/// 🔑️ Key extractor of the foreign `Deck` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeckKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::TimberConnection> for TimberConnectionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::TimberConnection) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::Placement> for PlacementKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::Placement) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::PanelLayer> for PanelLayerKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::PanelLayer) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::FastenerPosition> for FastenerPositionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::FastenerPosition) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::Taper> for TaperKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::Taper) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::Cutout> for CutoutKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::Cutout) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::Fastener> for FastenerKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::Fastener) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::Panel> for PanelKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::Panel) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::Floor> for FloorKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::Floor) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::Diaphragm> for DiaphragmKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::Diaphragm) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::Truss> for TrussKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::Truss) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::Bracing> for BracingKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::Bracing) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1995_model::Deck> for DeckKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1995_model::Deck) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `CharacteristicAction`.
    pub En1995MemberActionPatch of semio_s_artifact_norm_en1995_model::CharacteristicAction { set { kind: semio_s_artifact_norm_en1995_model::ActionKind, category: semio_s_artifact_norm_en1995_model::ImposedCategory, load_duration: semio_s_artifact_norm_en1995_model::LoadDurationClass, q_line_n_per_m: f64, f_point_n: f64, m_k_nm: f64, v_k_n: f64, n_k_n: f64, n_t_k_n: f64, f_c90_k_n: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `CharacteristicAction` list.
    pub En1995MemberActionDelta { removal: En1995MemberActionRemoval, insertion: En1995MemberActionInsertion, relocation: En1995MemberActionRelocation, modification: En1995MemberActionModification, row: semio_s_artifact_norm_en1995_model::CharacteristicAction, patch: En1995MemberActionPatch, list: Vec<semio_s_artifact_norm_en1995_model::CharacteristicAction>, key: String = by CharacteristicActionKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ConnectionAction`.
    pub En1995ConnectionActionPatch of semio_s_artifact_norm_en1995_model::ConnectionAction { set { kind: semio_s_artifact_norm_en1995_model::ActionKind, load_duration: semio_s_artifact_norm_en1995_model::LoadDurationClass, f_k_n: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `ConnectionAction` list.
    pub En1995ConnectionActionDelta { removal: En1995ConnectionActionRemoval, insertion: En1995ConnectionActionInsertion, relocation: En1995ConnectionActionRelocation, modification: En1995ConnectionActionModification, row: semio_s_artifact_norm_en1995_model::ConnectionAction, patch: En1995ConnectionActionPatch, list: Vec<semio_s_artifact_norm_en1995_model::ConnectionAction>, key: String = by ConnectionActionKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `TimberMember`.
    pub En1995MemberPatch of semio_s_artifact_norm_en1995_model::TimberMember { set { label_en: String, label_de: String, role: semio_s_artifact_norm_en1995_model::MemberRole, strength_class: semio_s_artifact_norm_en1995_model::StrengthClass, service_class: u8, support: semio_s_artifact_norm_en1995_model::SupportType, b_m: f64, h_m: f64, span_m: f64, support_length_m: f64, bearing_length_m: f64, buckling_length_y_m: f64, buckling_length_z_m: f64, lateral_restraint_spacing_m: f64, notch_depth_m: f64, notch_distance_m: f64, mass_kg_per_m: f64, mass_kg_per_m2: f64, damping_xi: f64, fire_duration_s: f64, bridge_n_obs: f64, bridge_t_l_years: f64, bridge_beta: f64, bridge_a: f64, bridge_b: f64, bridge_crowd_per_m2: f64 } nest { actions: En1995MemberActionDelta } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `TimberMember` list.
    pub En1995MemberDelta { removal: En1995MemberRemoval, insertion: En1995MemberInsertion, relocation: En1995MemberRelocation, modification: En1995MemberModification, row: semio_s_artifact_norm_en1995_model::TimberMember, patch: En1995MemberPatch, list: Vec<semio_s_artifact_norm_en1995_model::TimberMember>, key: String = by TimberMemberKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `TimberConnection`.
    pub En1995ConnectionPatch of semio_s_artifact_norm_en1995_model::TimberConnection { set { label_en: String, label_de: String, fastener_type: semio_s_artifact_norm_en1995_model::FastenerType, strength_class: semio_s_artifact_norm_en1995_model::StrengthClass, service_class: u8, diameter_m: f64, number: u32, rows: u32, spacing_m: f64, edge_distance_m: f64, end_distance_m: f64, t1_m: f64, t2_m: f64, steel_plate: bool, steel_plate_thickness_m: f64, shear_planes: u32, f_u_k: f64 } nest { actions: En1995ConnectionActionDelta } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Placement`.
    pub En1995PlacementPatch of semio_s_artifact_norm_en1995_model::Placement { set { x: f64, y: f64, z: f64, yaw: f64 } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Taper`.
    pub En1995TaperPatch of semio_s_artifact_norm_en1995_model::Taper { set { member_id: String, kind: semio_s_artifact_norm_en1995_model::TaperKind, taper_angle_rad: f64, apex_depth_m: f64, inner_radius_m: f64, lamella_thickness_m: f64, apex_angle_rad: f64, tension_side: semio_s_artifact_norm_en1995_model::TensionSide, rods: u32, rod_resistance_n: f64 } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Cutout`.
    pub En1995CutoutPatch of semio_s_artifact_norm_en1995_model::Cutout { set { member_id: String, kind: semio_s_artifact_norm_en1995_model::CutoutKind, face: semio_s_artifact_norm_en1995_model::CutoutFace, distance_m: f64, length_m: f64, depth_m: f64, centre_height_m: f64, slope: f64, rods: u32, rod_resistance_n: f64 } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Fastener`.
    pub En1995FastenerPatch of semio_s_artifact_norm_en1995_model::Fastener { set { connection_id: String, member_id: String, columns: u32, lines: u32, spacing_parallel_m: f64, spacing_perpendicular_m: f64, end_distance_m: f64, edge_distance_m: f64, offset_x_m: f64, offset_y_m: f64, load_angle_rad: f64 } nest { positions: En1995FastenerPositionDelta } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Panel`.
    pub En1995PanelPatch of semio_s_artifact_norm_en1995_model::Panel { set { strength_class: semio_s_artifact_norm_en1995_model::StrengthClass, span_m: f64, width_m: f64, load_pa: f64, fire_duration_s: f64 } nest { layers: En1995PanelLayerDelta } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Floor`.
    pub En1995FloorPatch of semio_s_artifact_norm_en1995_model::Floor { set { member_id: String, span_m: f64, width_m: f64, joist_spacing_m: f64, ei_l_n_m2_per_m: f64, ei_b_n_m2_per_m: f64, mass_kg_per_m2: f64, damping_ratio: f64, permanent_load_pa: f64, imposed_load_pa: f64, psi_2: f64 } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Diaphragm`.
    pub En1995DiaphragmPatch of semio_s_artifact_norm_en1995_model::Diaphragm { set { kind: semio_s_artifact_norm_en1995_model::DiaphragmKind, method: semio_s_artifact_norm_en1995_model::DiaphragmMethod, panel_width_m: f64, wall_height_m: f64, fastener_spacing_m: f64, panels: u32, fastener_capacity_n: f64, shear_ed_n: f64, hold_down_n: f64, diaphragm_force_n: f64, diaphragm_length_m: f64, shear_resistance_n_per_m: f64 } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Truss`.
    pub En1995TrussPatch of semio_s_artifact_norm_en1995_model::Truss { set { kind: semio_s_artifact_norm_en1995_model::TrussKind, product: semio_s_artifact_norm_en1995_model::BracingProduct, span_m: f64, height_m: f64, bays: u32, chord_width_m: f64, chord_depth_m: f64, web_width_m: f64, web_depth_m: f64, chord_force_n: f64, restraint_length_m: f64, bracing_capacity_n_per_m: f64 } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Bracing`.
    pub En1995BracingPatch of semio_s_artifact_norm_en1995_model::Bracing { set { kind: semio_s_artifact_norm_en1995_model::BracingKind, member_id: String, stabilised_count: u32, spacing_m: f64, length_m: f64, brace_width_m: f64, brace_depth_m: f64, compression_force_n: f64, resistance_n: f64, stiffness_n_per_m: f64 } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Deck`.
    pub En1995DeckPatch of semio_s_artifact_norm_en1995_model::Deck { set { lamination_product: semio_s_artifact_norm_en1995_model::LaminationProduct, bar_product: semio_s_artifact_norm_en1995_model::PrestressBarProduct, span_m: f64, deck_width_m: f64, deck_depth_m: f64, lamination_width_m: f64, bar_spacing_m: f64, bar_diameter_m: f64, plate_side_m: f64, initial_prestress_pa: f64, loss_relaxation: f64, loss_creep: f64, loss_moisture: f64, bar_resistance_pa: f64, shear_stress_pa: f64, compression_perpendicular_pa: f64, wheel_load_n: f64, wheel_contact_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `TimberConnection` list.
    pub En1995ConnectionDelta { removal: En1995ConnectionRemoval, insertion: En1995ConnectionInsertion, relocation: En1995ConnectionRelocation, modification: En1995ConnectionModification, row: semio_s_artifact_norm_en1995_model::TimberConnection, patch: En1995ConnectionPatch, list: Vec<semio_s_artifact_norm_en1995_model::TimberConnection>, key: String = by TimberConnectionKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Placement` list.
    pub En1995PlacementDelta { removal: En1995PlacementRemoval, insertion: En1995PlacementInsertion, relocation: En1995PlacementRelocation, modification: En1995PlacementModification, row: semio_s_artifact_norm_en1995_model::Placement, patch: En1995PlacementPatch, list: Vec<semio_s_artifact_norm_en1995_model::Placement>, key: String = by PlacementKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Taper` list.
    pub En1995TaperDelta { removal: En1995TaperRemoval, insertion: En1995TaperInsertion, relocation: En1995TaperRelocation, modification: En1995TaperModification, row: semio_s_artifact_norm_en1995_model::Taper, patch: En1995TaperPatch, list: Vec<semio_s_artifact_norm_en1995_model::Taper>, key: String = by TaperKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Cutout` list.
    pub En1995CutoutDelta { removal: En1995CutoutRemoval, insertion: En1995CutoutInsertion, relocation: En1995CutoutRelocation, modification: En1995CutoutModification, row: semio_s_artifact_norm_en1995_model::Cutout, patch: En1995CutoutPatch, list: Vec<semio_s_artifact_norm_en1995_model::Cutout>, key: String = by CutoutKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Fastener` list.
    pub En1995FastenerDelta { removal: En1995FastenerRemoval, insertion: En1995FastenerInsertion, relocation: En1995FastenerRelocation, modification: En1995FastenerModification, row: semio_s_artifact_norm_en1995_model::Fastener, patch: En1995FastenerPatch, list: Vec<semio_s_artifact_norm_en1995_model::Fastener>, key: String = by FastenerKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FastenerPosition`.
    pub En1995FastenerPositionPatch of semio_s_artifact_norm_en1995_model::FastenerPosition { set { x_m: f64, y_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `FastenerPosition` list.
    pub En1995FastenerPositionDelta { removal: En1995FastenerPositionRemoval, insertion: En1995FastenerPositionInsertion, relocation: En1995FastenerPositionRelocation, modification: En1995FastenerPositionModification, row: semio_s_artifact_norm_en1995_model::FastenerPosition, patch: En1995FastenerPositionPatch, list: Vec<semio_s_artifact_norm_en1995_model::FastenerPosition>, key: String = by FastenerPositionKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Panel` list.
    pub En1995PanelDelta { removal: En1995PanelRemoval, insertion: En1995PanelInsertion, relocation: En1995PanelRelocation, modification: En1995PanelModification, row: semio_s_artifact_norm_en1995_model::Panel, patch: En1995PanelPatch, list: Vec<semio_s_artifact_norm_en1995_model::Panel>, key: String = by PanelKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `PanelLayer`.
    pub En1995PanelLayerPatch of semio_s_artifact_norm_en1995_model::PanelLayer { set { thickness_m: f64, orientation: semio_s_artifact_norm_en1995_model::LayerOrientation, grade: String } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `PanelLayer` list.
    pub En1995PanelLayerDelta { removal: En1995PanelLayerRemoval, insertion: En1995PanelLayerInsertion, relocation: En1995PanelLayerRelocation, modification: En1995PanelLayerModification, row: semio_s_artifact_norm_en1995_model::PanelLayer, patch: En1995PanelLayerPatch, list: Vec<semio_s_artifact_norm_en1995_model::PanelLayer>, key: String = by PanelLayerKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Floor` list.
    pub En1995FloorDelta { removal: En1995FloorRemoval, insertion: En1995FloorInsertion, relocation: En1995FloorRelocation, modification: En1995FloorModification, row: semio_s_artifact_norm_en1995_model::Floor, patch: En1995FloorPatch, list: Vec<semio_s_artifact_norm_en1995_model::Floor>, key: String = by FloorKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Diaphragm` list.
    pub En1995DiaphragmDelta { removal: En1995DiaphragmRemoval, insertion: En1995DiaphragmInsertion, relocation: En1995DiaphragmRelocation, modification: En1995DiaphragmModification, row: semio_s_artifact_norm_en1995_model::Diaphragm, patch: En1995DiaphragmPatch, list: Vec<semio_s_artifact_norm_en1995_model::Diaphragm>, key: String = by DiaphragmKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Truss` list.
    pub En1995TrussDelta { removal: En1995TrussRemoval, insertion: En1995TrussInsertion, relocation: En1995TrussRelocation, modification: En1995TrussModification, row: semio_s_artifact_norm_en1995_model::Truss, patch: En1995TrussPatch, list: Vec<semio_s_artifact_norm_en1995_model::Truss>, key: String = by TrussKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Bracing` list.
    pub En1995BracingDelta { removal: En1995BracingRemoval, insertion: En1995BracingInsertion, relocation: En1995BracingRelocation, modification: En1995BracingModification, row: semio_s_artifact_norm_en1995_model::Bracing, patch: En1995BracingPatch, list: Vec<semio_s_artifact_norm_en1995_model::Bracing>, key: String = by BracingKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Deck` list.
    pub En1995DeckDelta { removal: En1995DeckRemoval, insertion: En1995DeckInsertion, relocation: En1995DeckRelocation, modification: En1995DeckModification, row: semio_s_artifact_norm_en1995_model::Deck, patch: En1995DeckPatch, list: Vec<semio_s_artifact_norm_en1995_model::Deck>, key: String = by DeckKey }
}

//#endregion 🔖️Rows

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1995 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Diff {
    #[state(artifact)]
    pub annex: Option<semio_s_plugin_norm_compliance::AnnexChoice>,
    #[state(artifact)]
    pub members: En1995MemberDelta,
    #[state(artifact)]
    pub connections: En1995ConnectionDelta,
    #[state(artifact)]
    pub placements: En1995PlacementDelta,
    #[state(artifact)]
    pub tapers: En1995TaperDelta,
    #[state(artifact)]
    pub cutouts: En1995CutoutDelta,
    #[state(artifact)]
    pub fasteners: En1995FastenerDelta,
    #[state(artifact)]
    pub panels: En1995PanelDelta,
    #[state(artifact)]
    pub floors: En1995FloorDelta,
    #[state(artifact)]
    pub diaphragms: En1995DiaphragmDelta,
    #[state(artifact)]
    pub trusses: En1995TrussDelta,
    #[state(artifact)]
    pub bracings: En1995BracingDelta,
    #[state(artifact)]
    pub decks: En1995DeckDelta,
    #[state(artifact)]
    pub uls_equation: Option<semio_s_artifact_norm_en1995_model::UlsEquation>,
    #[state(artifact)]
    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,
    #[state(artifact)]
    pub reliability_class: Option<semio_s_plugin_norm_compliance::ReliabilityClass>,
}
//#endregion 🔖️Diff

impl MutationDiff<En1995Snapshot> for En1995Diff {}

impl protocol::ColdMutationDiff<En1995Snapshot> for En1995Diff {
    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.reliability_class.is_some() {
            self.reliability_class = other.reliability_class;
        }
        self.members.absorb(other.members);
        self.connections.absorb(other.connections);
        self.placements.absorb(other.placements);
        self.tapers.absorb(other.tapers);
        self.cutouts.absorb(other.cutouts);
        self.fasteners.absorb(other.fasteners);
        self.panels.absorb(other.panels);
        self.floors.absorb(other.floors);
        self.diaphragms.absorb(other.diaphragms);
        self.trusses.absorb(other.trusses);
        self.bracings.absorb(other.bracings);
        self.decks.absorb(other.decks);
        if other.uls_equation.is_some() {
            self.uls_equation = other.uls_equation;
        }
        self.cases.absorb(other.cases);
    }

    fn apply(&self, unpublished: &mut En1995Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        *unpublished = En1995Snapshot {
            annex: self.annex.unwrap_or(base.annex),
            reliability_class: self.reliability_class.unwrap_or(base.reliability_class),
            members: self.members.commit_onto(&base.members, capability).map_err(|error| error.under(["members"]))?,
            connections: self.connections.commit_onto(&base.connections, capability).map_err(|error| error.under(["connections"]))?,
            uls_equation: self.uls_equation.clone().unwrap_or_else(|| base.uls_equation.clone()),
            cases: self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?,
            placements: self.placements.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?,
            tapers: self.tapers.commit_onto(&base.tapers, capability).map_err(|error| error.under(["tapers"]))?,
            cutouts: self.cutouts.commit_onto(&base.cutouts, capability).map_err(|error| error.under(["cutouts"]))?,
            fasteners: self.fasteners.commit_onto(&base.fasteners, capability).map_err(|error| error.under(["fasteners"]))?,
            panels: self.panels.commit_onto(&base.panels, capability).map_err(|error| error.under(["panels"]))?,
            floors: self.floors.commit_onto(&base.floors, capability).map_err(|error| error.under(["floors"]))?,
            diaphragms: self.diaphragms.commit_onto(&base.diaphragms, capability).map_err(|error| error.under(["diaphragms"]))?,
            trusses: self.trusses.commit_onto(&base.trusses, capability).map_err(|error| error.under(["trusses"]))?,
            bracings: self.bracings.commit_onto(&base.bracings, capability).map_err(|error| error.under(["bracings"]))?,
            decks: self.decks.commit_onto(&base.decks, capability).map_err(|error| error.under(["decks"]))?,
        };
        Ok(())
    }
}

impl DiffAlgebra<En1995Snapshot> for En1995Diff {
    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.reliability_class.is_none()
            && self.members.is_empty()
            && self.connections.is_empty()
            && self.placements.is_empty()
            && self.tapers.is_empty()
            && self.cutouts.is_empty()
            && self.fasteners.is_empty()
            && self.panels.is_empty()
            && self.floors.is_empty()
            && self.diaphragms.is_empty()
            && self.trusses.is_empty()
            && self.bracings.is_empty()
            && self.decks.is_empty()
            && self.uls_equation.is_none()
            && self.cases.is_empty()
    }
}

impl protocol::ColdDiffAlgebra<En1995Snapshot> for En1995Diff {
    fn inverse(&self, base: &En1995Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex),
            reliability_class: self.reliability_class.as_ref().map(|_| base.reliability_class),
            members: self.members.inverse(&base.members),
            connections: self.connections.inverse(&base.connections),
            placements: self.placements.inverse(&base.placements),
            tapers: self.tapers.inverse(&base.tapers),
            cutouts: self.cutouts.inverse(&base.cutouts),
            fasteners: self.fasteners.inverse(&base.fasteners),
            panels: self.panels.inverse(&base.panels),
            floors: self.floors.inverse(&base.floors),
            diaphragms: self.diaphragms.inverse(&base.diaphragms),
            trusses: self.trusses.inverse(&base.trusses),
            bracings: self.bracings.inverse(&base.bracings),
            decks: self.decks.inverse(&base.decks),
            uls_equation: self.uls_equation.as_ref().map(|_| base.uls_equation.clone()),
            cases: self.cases.inverse(&base.cases),
        }
    }
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves
