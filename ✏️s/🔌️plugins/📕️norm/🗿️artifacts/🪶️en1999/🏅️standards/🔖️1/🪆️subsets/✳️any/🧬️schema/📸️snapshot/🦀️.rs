//! ✨️ EN 1999 snapshot — aluminium structure subject (SI: m, N, Pa, °C).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};
use framework_schema::ArtifactSchema;

//#region 🔖️SubjectEntities

/// 🏭️ Product form of EN 1999-1-1 Table 3.2.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum ProductForm {
    Sheet,
    ExtrudedProfile,
    ExtrudedTube,
    ExtrudedBar,
    DrawnTube,
    Forging,
}

/// 🔩 Alloy temper catalogue entry (EN 1999-1-1 Table 3.2 properties resolved at evaluate).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AluminiumMaterial {
    pub id: String,
    /// 🆔️ Designation key, e.g. `aw6060-t6`, `aw6082-t6`, `aw5083-o`.
    pub designation: String,
    /// 🏭️ Product form of Table 3.2: `sheet` | `extrudedProfile` | `extrudedTube` | `extrudedBar` | `drawnTube` | `forging`.
    pub product_form: ProductForm,
}

/// 🔥 Welding process of a welded plate element or weld run (EN 1999-1-1 6.1.6).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum WeldProcess {
    Mig,
    Tig,
}

/// 🧵 Direction of a weld against the member axis (heat-affected zone pattern, EN 1999-1-1 6.1.6.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum WeldDirection {
    Longitudinal,
    Transverse,
}

/// 🧱 Plate element of an extruded section (b/t classification input).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PlateElement {
    pub id: String,
    /// ↔️ Flat width b [m].
#[cfg_attr(any(test, feature = "test-support"), serde(default, skip_serializing_if = "is_zero_f64"))]
    pub width: f64,
    /// 📏️ Thickness t [m].
    pub thickness: f64,
    /// ✔️ Outstand flange/leg when true; internal web/flange when false.
    pub outstand: bool,
    /// 🪛️ Element contains a weld (applies η and HAZ).
    pub welded: bool,
    /// 🔧️ Weld position along element from root [m] (0 = at junction); omitted when not welded.
    #[cfg_attr(any(test, feature = "test-support"), serde(default, skip_serializing_if = "is_zero_f64"))]
    pub weld_position: f64,
    /// 🔥️ Welding process `mig` | `tig` (HAZ extent factor).
    pub weld_process: WeldProcess,
    /// 🔀️ The weld sits in a T-junction (HAZ extent × 1.33).
    pub weld_t_junction: bool,
    /// 🧭️ `longitudinal` (HAZ strips on both sides of the weld line) | `transverse` (HAZ across the whole element width).
    pub weld_direction: WeldDirection,
    /// 📐️ Edge stiffener (lip) depth [m]; 0 = none. Only outstands can carry one.
    pub edge_stiffener_depth: f64,
    /// 📏️ Edge stiffener thickness [m]; 0 = none.
    pub edge_stiffener_thickness: f64,
}

/// 📐 Kind of a cross-section: its shape family decides the resistance formulas and the outline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum SectionKind {
    ExtrudedI,
    Channel,
    Angle,
    Rhs,
    Box,
    Chs,
    Free,
}

/// 📐 Extruded aluminium cross-section.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AluminiumSection {
    pub id: String,
    /// 🧊️ Discriminated geometry: `extrudedI` | `channel` | `angle` | `rhs` | `box` | `tube`/`chs`.
    /// I/H/channel/angle/rhs/box use height/width/flangeThickness/webThickness; CHS/tube use outerDiameter + webThickness (wall).
    pub kind: SectionKind,
    /// ↕️ Zero for CHS/tube (discriminated — omitted when 0).
    #[cfg_attr(any(test, feature = "test-support"), serde(default, skip_serializing_if = "is_zero_f64"))]
    pub height: f64,
    #[cfg_attr(any(test, feature = "test-support"), serde(default, skip_serializing_if = "is_zero_f64"))]
    pub width: f64,
    #[cfg_attr(any(test, feature = "test-support"), serde(default, skip_serializing_if = "is_zero_f64"))]
    pub flange_thickness: f64,
    pub web_thickness: f64,
    /// 📤️ CHS/tube outer diameter D [m]; omitted for non-tube kinds (discriminated geometry).
    #[cfg_attr(any(test, feature = "test-support"), serde(default, skip_serializing_if = "is_zero_f64"))]
    pub outer_diameter: f64,
    #[dsl(table)]
    pub elements: Vec<PlateElement>,
}

/// 🏋️ Kind of a characteristic action (EN 1990 4.1.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum ActionKind {
    Permanent,
    Imposed,
    Snow,
    Wind,
    Temperature,
    Fire,
}

/// 🏷️ Category of an action for the combination factors psi (EN 1990 Table A1.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum ActionCategory {
    SelfWeight,
    Office,
    Snow,
    Wind,
}

/// 🧮 Source of the characteristic internal forces of an action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum ActionSource {
    Udl,
    External,
}

/// 🏋️ Characteristic action / load case on a member (EN 1990; SI).
///
/// Either `source = "udl"` (line loads converted via span/support) or `source = "external"`
/// (characteristic internal forces from structural analysis).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MemberAction {
    pub id: String,
    /// 📌️ permanent | imposed | snow | wind | temperature | fire
    pub kind: ActionKind,
    /// 🚦️ EN 1990 ψ-category (office, snow, wind, self, …).
    pub category: ActionCategory,
    /// 🪢️ udl | external
    pub source: ActionSource,
    /// 🔤️ Characteristic permanent line load g_k [N/m] (udl).
    pub g_k_line: f64,
    /// 🔡️ Characteristic variable line load q_k [N/m] (udl).
    pub q_k_line: f64,
    /// ⚖️ Characteristic axial force N_k [N] (external).
    pub n_k: f64,
    /// 📉️ Characteristic shear V_y,k [N].
    pub v_y_k: f64,
    /// 🧬️ Characteristic shear V_z,k [N].
    pub v_z_k: f64,
    /// 🧲️ Characteristic moment M_y,k [N·m].
    pub m_y_k: f64,
    /// 🔋️ Characteristic moment M_z,k [N·m].
    pub m_z_k: f64,
    /// 🌀️ Characteristic torque T_k [N·m] (EN 1999-1-1 §6.2.7).
    pub torsion_k: f64,
    /// 📍️ Characteristic transverse patch load F_k [N] on the flange (EN 1999-1-1 §6.7.5).
    pub local_load_k: f64,
}

/// 📍️ Support idealisation of a member for characteristic → internal force derivation and the effective buckling length.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum SupportCondition {
    SimplySupported,
    Continuous,
    Cantilever,
}

/// 🔩 Distribution of an eccentric shear over a bolt group: elastic method or instantaneous-centre method.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum FastenerForceMethod {
    Elastic,
    InstantaneousCentre,
}

impl FastenerForceMethod {
    /// 🔤 Wire token of the variant.
    pub fn code(self) -> &'static str {
        match self {
            Self::Elastic => "elastic",
            Self::InstantaneousCentre => "instantaneousCentre",
        }
    }
}

impl SupportCondition {
    /// 🏷️ The wire value (`simplySupported` | `continuous` | `cantilever`).
    pub fn code(self) -> &'static str {
        match self {
            Self::SimplySupported => "simplySupported",
            Self::Continuous => "continuous",
            Self::Cantilever => "cantilever",
        }
    }
}

/// 🏗️ Aluminium member with buckling lengths and restraints.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AluminiumMember {
    pub id: String,
    pub section_id: String,
    pub material_id: String,
    /// 🔢️ System length L [m] (span for UDL conversion; default L_cr when buckling lengths ≤ 0).
    pub length: f64,
    pub support: SupportCondition,
    pub buckling_length_y: f64,
    pub buckling_length_z: f64,
    pub buckling_length_t: f64,
    pub ltb_length: f64,
    pub c1: f64,
    pub restrained_ltb: bool,
    /// 📏️ Stiff bearing length s_s of the patch load [m].
    pub bearing_length: f64,
    /// 🔗️ A localized transverse weld lies on the member (EN 1999-1-1 §6.3.3.3 ω_x).
    pub transverse_weld: bool,
    /// 📍️ Distance x_s of that weld from the nearest member end [m].
    pub transverse_weld_offset: f64,
    #[dsl(table)]
    pub actions: Vec<MemberAction>,
}

/// 🔩 Bolted connection parameters (EN 1999-1-1 §8.5).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BoltGroup {
    pub material: String,
    pub diameter: f64,
    pub rows: u32,
    pub bolts_per_row: u32,
    pub edge_distance: f64,
    pub pitch: f64,
    pub gauge: f64,
    pub plate_thickness: f64,
}

/// 🔥️ Fillet weld parameters (EN 1999-1-1 §8.6).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct WeldGroup {
    pub filler_alloy: String,
    pub throat: f64,
    pub length: f64,
    pub beta_w: f64,
    pub haz_extent: f64,
}

/// 🔗 Kind of a connection (EN 1999-1-1 Section 8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum ConnectionKind {
    Bolted,
    Welded,
    Combined,
    Pinned,
}

/// 🔗 Connection (bolted and/or welded) with design forces.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AluminiumConnection {
    pub id: String,
    pub member_id: String,
    pub material_id: String,
    /// ⚙️ `bolted` | `welded` | `combined` | `pinned` (pins use `bolts`: diameter, rows × boltsPerRow pins, plateThickness).
    pub kind: ConnectionKind,
    /// 🎮️ Characteristic connection load cases (same EN 1990 model as members).
    #[dsl(table)]
    pub actions: Vec<MemberAction>,
    pub bolts: BoltGroup,
    pub welds: WeldGroup,
    /// 🔩 Lever arm e of the in-plane shear force from the centroid of the fastener group (EN 1999-1-1 8.5); 0 = concentric, the shear is shared equally.
    #[dsl(unit = "m")]
    pub load_eccentricity: f64,
    /// 🕳️ Longitudinal offset s of the holes of every second bolt line across the load (EN 1999-1-1 6.2.2: staggered holes reduce the net-section deduction by s²·t/(4·p) per diagonal); 0 = all bolt lines aligned.
    #[dsl(unit = "m")]
    pub hole_stagger: f64,
    /// 🔩 Method that distributes an eccentric in-plane shear over the fasteners of a bolt group (EN 1999-1-1 8.5): the elastic method (rigid plate, forces proportional to the distance from the centroid) or the instantaneous-centre method (ductile fasteners rotating about the instantaneous centre of rotation).
    pub force_method: FastenerForceMethod,
}

/// 🔥️ Fire assessment scenario (EN 1999-1-2).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FireScenario {
    pub id: String,
    pub member_id: String,
    pub theta_a: f64,
    pub duration_s: f64,
}

/// 🔄️ Fatigue detail (EN 1999-1-3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueDetail {
    pub id: String,
    pub member_id: String,
    /// 🎛️ Annex J detail category id (e.g. `71`, `40-weld`).
    pub detail_category: String,
    /// 🔺️ Detail category Δσ_C [Pa] — derived from detailCategory (one source of truth; not serialized).
    #[cfg_attr(any(test, feature = "test-support"), serde(default, skip_serializing))]
    pub delta_sigma_c: f64,
    /// ✏️ Constant-amplitude stress range Δσ [Pa] (single-range spectrum entry).
    pub delta_sigma_ed: f64,
    /// 🔁️ Cycles in the spectrum block n_i.
    pub n_cycles: f64,
    /// 🌳️ First slope m1 (N ≤ N_D).
    pub m1: f64,
    /// 📜️ Second slope m2 (N_D < N ≤ N_L).
    pub m2: f64,
}

/// 📄 Cold-formed aluminium sheeting (EN 1999-1-4).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ColdFormedSheet {
    pub id: String,
    pub material_id: String,
    /// 🔭️ Design thickness t [m].
    pub thickness: f64,
    /// ⬜️ Flat width between webs / supports b [m].
    pub width: f64,
    /// 🔬️ Span L [m].
    pub span: f64,
    pub welded: bool,
    /// ⌨️ Characteristic EN 1990 actions; design N_Ed/M_Ed from governing ULS/SLS.
    pub actions: Vec<MemberAction>,
}

/// 🏗️ Fabrication quality class of a shell (EN 1999-1-5 Table 5.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum FabricationClass {
    A,
    B,
    C,
}

/// 🫙 Aluminium shell of revolution (EN 1999-1-5).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AluminiumShell {
    pub id: String,
    pub material_id: String,
    /// ⭕️ Mid-surface radius r [m].
    pub radius: f64,
    /// 🚧️ Wall thickness t [m].
    pub thickness: f64,
    /// ➕️ Meridian length L [m].
    pub length: f64,
    /// 🔥️ A circumferential weld lies in the shell wall (HAZ reduces the axial resistance, EN 1999-1-5).
    pub welded: bool,
    /// 🏭️ Fabrication quality class `A` | `B` | `C` (imperfection amplitude Q = 40 | 25 | 16).
    pub fabrication_class: FabricationClass,
    /// 🕹️ Characteristic membrane actions (n_k→σ_x,k, m_y_k→σ_θ,k, v_z_k→τ_k [Pa]).
    pub actions: Vec<MemberAction>,
}

/// 📍️ Persisted pose of a member, sheet, shell or connection in the 3D scene (`id` is the entity id; SI: m, rad). Entities without a placement follow the documented default layout.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Placement {
    pub id: String,
    /// ↔️ Origin x [m].
    pub x: f64,
    /// ↕️ Origin y [m].
    pub y: f64,
    /// ⬆️ Origin z [m].
    pub z: f64,
    /// 🧭️ Yaw about +Z [rad].
    pub yaw: f64,
}

/// 🧵️ Explicit weld run along a member (EN 1999-1-1 §6.1.6, §6.3.3.3, §8.6): a longitudinal run lies between `start` and `start + length` along the member axis, a transverse run sits at `start` (a localized weld, `length` is ignored). It overrides the weld flags of the named plate element in the member's checks and drives the localized-weld factor ω_x.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct WeldRun {
    pub id: String,
    /// 🏗️ Member the run lies on.
    pub member_id: String,
    /// 🧱️ Plate element of the member's section carrying the weld; empty = no element (only a transverse run affects the member).
    pub element_id: String,
    /// 📍️ Start (longitudinal run) or position (transverse run) x along the member axis [m].
    pub start: f64,
    /// 📏️ Run length along the member axis [m]; ignored for a transverse run.
    pub length: f64,
    /// 🔀️ Weld position across the plate element from its root [m] (0 = at the junction).
    pub offset: f64,
    /// 🔥️ Fillet weld throat thickness a [m].
    pub throat: f64,
    /// ⚙️ Welding process `mig` | `tig` (HAZ extent factor).
    pub process: WeldProcess,
    /// 🔱️ The run sits in a T-junction (HAZ extent × 1.33).
    pub t_junction: bool,
    /// 🧭️ `longitudinal` (HAZ strips beside the line) | `transverse` (HAZ across the whole element width, localized weld on the member).
    pub direction: WeldDirection,
    /// ♨️ HAZ extent b_haz per side [m]; 0 = table of EN 1999-1-1 §6.1.6.3 by thickness, process and junction.
    pub haz_extent: f64,
}

/// ⭕ Ring of an outline vertex: the outer boundary or a hole.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum OutlineRing {
    Outer,
    Hole,
}

/// 🔷️ Vertex of the free section outline (EN 1999-1-1 §6.1): the vertices of a section in list order form its closed rings; ring `outer` is the outer boundary, every other ring name is a hole. A section of kind `free` takes its gross properties from the polygon.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SectionVertex {
    pub id: String,
    /// 📏️ Section the vertex belongs to.
    pub section_id: String,
    /// ⭕️ Ring name: `outer` for the outer boundary, any other name for a hole.
    pub ring: OutlineRing,
    /// 🪵️ Plate element carried by the edge from this vertex to the next vertex of the ring (empty = none).
    pub element_id: String,
    /// ↗️ Horizontal coordinate y of the vertex in the section plane [m].
    pub y: f64,
    /// 🔝️ Vertical coordinate z of the vertex in the section plane [m].
    pub z: f64,
}

/// 🔩 Type of a fatigue detail (EN 1999-1-3 Annex J).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum DetailType {
    Plain,
    ButtWeld,
    FilletWeld,
    Bolted,
    Riveted,
    Other,
}

/// 🎯️ Fatigue-critical detail of a member (EN 1999-1-3 §6, Annex J): the site on the member where one detail category governs. The `fatigueDetails` rows with the same member and detail category are its stress-range spectrum blocks; the Δσ_C of the category is derived from `detailCategory`, the slopes `m1` and `m2` override the block slopes when positive.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueCategory {
    pub id: String,
    /// 🏗️ Member carrying the detail.
    pub member_id: String,
    /// 🪵️ Plate element of the member's section at the detail (empty = whole member).
    pub element_id: String,
    /// 🎛️ Annex J detail category id (e.g. `71`, `40-weld`).
    pub detail_category: String,
    /// 🧷️ Detail type: `plain` | `buttWeld` | `filletWeld` | `bolted` | `riveted` | `other`.
    pub detail_type: DetailType,
    /// 📍️ Location of the detail along the member axis [m].
    pub location_x: f64,
    /// 🌳️ First S-N slope m1; 0 = slope of the first stress block.
    pub m1: f64,
    /// 📜️ Second S-N slope m2; 0 = m1 + 2.
    pub m2: f64,
}

/// 📍 Position of a patch load along the girder (EN 1999-1-5 6).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum PatchLoadPosition {
    End,
    Interior,
}

/// ⬇️ Transverse patch load on a web (EN 1999-1-1 §6.7.5 for members, EN 1999-1-4 §6.1.7 for cold-formed sheets): a force introduced through a bearing length into the web of a member or a sheet. Web height, web slope and inner corner radius of 0 are taken from the section.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PatchLoad {
    pub id: String,
    /// 🎯️ Member or cold-formed sheet whose web carries the load.
    pub subject_id: String,
    /// ↔️ Load position `end` (one-flange, near a support) | `interior`.
    pub position: PatchLoadPosition,
    /// 📍️ Position of the load along the member or sheet [m].
    pub x: f64,
    /// 📏️ Bearing length s_s of the load on the flange [m].
    pub bearing_length: f64,
    /// ↕️ Web height h_w between flange mid-lines [m]; 0 = from the section.
    pub web_height: f64,
    /// 📐️ Web slope φ against the flange [rad]; 0 = 90 degrees.
    pub web_angle: f64,
    /// ⭕️ Inner corner radius r between web and flange [m].
    pub corner_radius: f64,
    /// ⬇️ Design transverse force F_Ed on the web [N].
    pub load: f64,
}

/// ⭕️ Ring stiffener of a cylindrical shell (EN 1999-1-5 §5, §8): a flat-bar ring at axial position `x` that divides the shell into bays; its flat-bar depth and thickness give the ring section that is checked against the required stiffness.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RingStiffener {
    pub id: String,
    /// 🫙 Shell the ring belongs to.
    pub shell_id: String,
    /// 📍️ Axial position of the ring from the shell base [m].
    pub x: f64,
    /// ↕️ Radial depth of the flat-bar ring [m].
    pub depth: f64,
    /// 🧱️ Thickness of the flat-bar ring [m].
    pub thickness: f64,
    /// 🔄️ The ring stands inside the shell (otherwise outside).
    pub internal: bool,
}

/// 🪗️ 🪗 Trapezoidal profile of a cold-formed sheet (EN 1999-1-4 5.5, 6.1): one rib of two webs between a top and a bottom flange, so that the bending, shear and deflection checks use the profiled section instead of the flat strip.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SheetProfile {
    pub id: String,
    /// 🔗 Id of the cold-formed sheet that carries this profile; the sheet thickness is the profile thickness.
    pub sheet_id: String,
    /// ↔️ Pitch p [m] of one rib (top flange, web, bottom flange, web), measured between the mid-lines.
    #[dsl(unit = "m")]
    pub pitch: f64,
    /// ⬜️ Flat width b_t [m] of the top flange between the web corners (mid-line).
    #[dsl(unit = "m")]
    pub top_flange: f64,
    /// ⬜️ Flat width b_b [m] of the bottom flange between the web corners (mid-line).
    #[dsl(unit = "m")]
    pub bottom_flange: f64,
    /// ↕️ Vertical distance h [m] between the mid-lines of the flanges; the web inclination follows from the pitch and the flange widths.
    #[dsl(unit = "m")]
    pub web_height: f64,
}

/// 🧱️ 🧱 Fire protection (insulation) of the member of a fire scenario (EN 1999-1-2 4.3.4): thickness, conductivity, specific heat and density of the insulating material for the temperature of a protected member.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FireInsulation {
    pub id: String,
    /// 🔗 Id of the fire scenario whose member is protected.
    pub fire_id: String,
    /// 📏 Thickness d_p [m] of the insulation.
    #[dsl(unit = "m")]
    pub thickness: f64,
    /// 🌡️ Thermal conductivity λ_p [W/(m·K)] of the insulation.
    pub conductivity: f64,
    /// 🔥 Specific heat c_p [J/(kg·K)] of the insulation.
    pub specific_heat: f64,
    /// ⚖️ Density ρ_p [kg/m³] of the insulation.
    pub density: f64,
}

/// 🦴 Sides of the web a transverse stiffener is welded to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum StiffenerSides {
    One,
    Two,
}

/// 🦴️ Transverse web stiffener of a plate girder (EN 1999-1-1 §6.7.6, §6.7.7; spec A-28): a flat bar welded across the web of the girder member at `position` along its axis, on one or on both sides of the web. The stiffener with an effective web strip (15·ε·t_w per side) must keep the required stiffness and carry the shear-induced axial force as a strut; the spacing of the stiffeners of one girder gives the panel widths a.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TransverseStiffener {
    pub id: String,
    /// 🏗️ Plate girder member the stiffener stands on.
    pub girder_id: String,
    /// 📍️ Position of the stiffener along the girder axis from its start [m].
    pub position: f64,
    /// ↔️ Outstand width b_st of the flat bar, measured from the web face [m].
    pub width: f64,
    /// 🧱️ Thickness t_st of the flat bar [m].
    pub thickness: f64,
    /// 🪞️ Arrangement `one` (single-sided) | `two` (double-sided, symmetric about the web).
    pub sides: StiffenerSides,
}

//#endregion 🔖️SubjectEntities

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1999 aluminium-structure subject.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1999", layout = "lines")]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[dsl(table)]
    #[state(artifact)]
    pub materials: Vec<AluminiumMaterial>,
    #[dsl(table)]
    #[state(artifact)]
    pub sections: Vec<AluminiumSection>,
    #[dsl(table)]
    #[state(artifact)]
    pub members: Vec<AluminiumMember>,
    #[dsl(table)]
    #[state(artifact)]
    pub connections: Vec<AluminiumConnection>,
    #[dsl(table)]
    #[state(artifact)]
    pub fire_scenarios: Vec<FireScenario>,
    #[dsl(table)]
    #[state(artifact)]
    pub fatigue_details: Vec<FatigueDetail>,
    #[dsl(table)]
    #[state(artifact)]
    pub cold_formed: Vec<ColdFormedSheet>,
    #[dsl(table)]
    #[state(artifact)]
    pub shells: Vec<AluminiumShell>,
    #[dsl(table)]
    #[state(artifact)]
    pub placements: Vec<Placement>,
    #[dsl(table)]
    #[state(artifact)]
    pub weld_runs: Vec<WeldRun>,
    #[dsl(table)]
    #[state(artifact)]
    pub outline_vertices: Vec<SectionVertex>,
    #[dsl(table)]
    #[state(artifact)]
    pub fatigue_categories: Vec<FatigueCategory>,
    #[dsl(table)]
    #[state(artifact)]
    pub patch_loads: Vec<PatchLoad>,
    #[dsl(table)]
    #[state(artifact)]
    pub ring_stiffeners: Vec<RingStiffener>,
    #[dsl(table)]
    #[state(artifact)]
    pub transverse_stiffeners: Vec<TransverseStiffener>,
    #[state(artifact)]
    pub reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,
    #[dsl(table)]
    #[state(artifact)]
    pub sheet_profiles: Vec<SheetProfile>,
    #[dsl(table)]
    #[state(artifact)]
    pub fire_insulations: Vec<FireInsulation>,
}

//#endregion 🔖️Snapshot

fn is_zero_f64(v: &f64) -> bool { *v == 0.0 }

fn default_plate(id: &str, width: f64, thickness: f64, outstand: bool, welded: bool) -> PlateElement {
    PlateElement { id: id.into(), width, thickness, outstand, welded, weld_position: if welded { 0.005 } else { 0.0 }, weld_process: WeldProcess::Mig, weld_t_junction: false, weld_direction: WeldDirection::Longitudinal, edge_stiffener_depth: 0.0, edge_stiffener_thickness: 0.0 }
}

fn default_bolts() -> BoltGroup {
    BoltGroup {
        material: "8.8".into(),
        diameter: 0.012,
        rows: 2,
        bolts_per_row: 2,
        edge_distance: 0.024,
        pitch: 0.036,
        gauge: 0.036,
        plate_thickness: 0.008,
    }
}

fn default_welds() -> WeldGroup {
    WeldGroup { filler_alloy: "4043".into(), throat: 0.004, length: 0.120, beta_w: 0.63, haz_extent: 0.025 }
}

impl En1999Snapshot {
    /// 📸️ Reference EN 1999 subject under the explicitly chosen annex; the annex is never defaulted.
    pub fn example(annex: semio_s_plugin_norm_compliance::AnnexChoice) -> Self {
        Self { annex: annex, ..Self::compliant_roof_purlin() }
    }

    /// 🎪️ The full showcase: one document that populates every entity collection and runs every check family of the spec trace, decoded from the committed fixture `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json` (generated by the ticket script `🧰️scripts/showcase`).
    pub fn full_showcase() -> Self {
        semio_framework_pack_json::from_json_str(include_str!("../../🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the committed showcase fixture decodes")
    }

    /// 📭 Empty subject shell (tests / set-snapshot baseline).
    pub fn empty() -> Self {
        Self {
            annex: AnnexChoice::En,
            reliability_class: ReliabilityClass::RC2,
            materials: Vec::new(),
            sections: Vec::new(),
            members: Vec::new(),
            connections: Vec::new(),
            fire_scenarios: Vec::new(),
            fatigue_details: Vec::new(),
            cold_formed: Vec::new(),
            shells: Vec::new(),
            placements: Vec::new(),
            weld_runs: Vec::new(),
            outline_vertices: Vec::new(),
            fatigue_categories: Vec::new(),
            patch_loads: Vec::new(),
            ring_stiffeners: Vec::new(),
            sheet_profiles: Vec::new(),
            fire_insulations: Vec::new(),
            transverse_stiffeners: Vec::new(),
        }
    }

    /// ✅ Realistic DE-annex roof purlin that complies under ULS + connection checks.
    pub fn compliant_roof_purlin() -> Self {
        let h = 0.140;
        let b = 0.070;
        let tf = 0.010;
        let tw = 0.008;
        Self {
            annex: AnnexChoice::De,
            reliability_class: ReliabilityClass::RC2,
            materials: vec![AluminiumMaterial { id: "mat-6082".into(), designation: "aw6082-t6".into(), product_form: ProductForm::ExtrudedProfile }],
            sections: vec![AluminiumSection {
                id: "sec-i120".into(),
                kind: SectionKind::ExtrudedI,
                height: h,
                width: b,
                flange_thickness: tf,
                web_thickness: tw,
                outer_diameter: 0.0,
                elements: vec![
                    default_plate("flange-top", (b - tw) * 0.5, tf, true, true),
                    default_plate("flange-bot", (b - tw) * 0.5, tf, true, true),
                    default_plate("web", h - 2.0 * tf, tw, false, true),
                ],
            },
            AluminiumSection {
                id: "sec-chs168".into(),
                kind: SectionKind::Chs,
                height: 0.0,
                width: 0.0,
                flange_thickness: 0.0,
                web_thickness: 0.005,
                outer_diameter: 0.1683,
                elements: vec![],
            }],
            members: vec![AluminiumMember {
                id: "purlin-1".into(),
                section_id: "sec-i120".into(),
                material_id: "mat-6082".into(),
                length: 6.0,
                support: SupportCondition::SimplySupported,
                buckling_length_y: 1.5,
                buckling_length_z: 1.5,
                buckling_length_t: 1.5,
                ltb_length: 1.5,
                c1: 1.132,
                restrained_ltb: false, bearing_length: 0.0, transverse_weld: false, transverse_weld_offset: 0.0,
                actions: vec![
                    MemberAction {
                        id: "G".into(),
                        kind: ActionKind::Permanent,
                        category: ActionCategory::SelfWeight,
                        source: ActionSource::Udl,
                        g_k_line: 300.0, q_k_line: 0.0,
                        n_k: 2_000.0, v_y_k: 0.0, v_z_k: 0.0, m_y_k: 0.0, m_z_k: 0.0, torsion_k: 0.0, local_load_k: 0.0,
                    },
                    MemberAction {
                        id: "Q-snow".into(),
                        kind: ActionKind::Snow,
                        category: ActionCategory::Snow,
                        source: ActionSource::Udl,
                        g_k_line: 0.0, q_k_line: 500.0,
                        n_k: 1_200.0, v_y_k: 50.0, v_z_k: 200.0, m_y_k: 100.0, m_z_k: 40.0, torsion_k: 0.0, local_load_k: 0.0,
                    },
                ],
            },
            AluminiumMember {
                id: "post-chs".into(),
                section_id: "sec-chs168".into(),
                material_id: "mat-6082".into(),
                length: 3.0,
                support: SupportCondition::Cantilever,
                buckling_length_y: 3.0,
                buckling_length_z: 3.0,
                buckling_length_t: 3.0,
                ltb_length: 3.0,
                c1: 1.0,
                restrained_ltb: false, bearing_length: 0.0, transverse_weld: false, transverse_weld_offset: 0.0,
                actions: vec![
                    MemberAction {
                        id: "G-post".into(), kind: ActionKind::Permanent, category: ActionCategory::SelfWeight, source: ActionSource::External,
                        g_k_line: 50.0, q_k_line: 0.0, n_k: -8_000.0, v_y_k: 20.0, v_z_k: 80.0, m_y_k: 120.0, m_z_k: 15.0, torsion_k: 0.0, local_load_k: 0.0,
                    },
                    MemberAction {
                        id: "Q-post".into(), kind: ActionKind::Imposed, category: ActionCategory::Office, source: ActionSource::External,
                        g_k_line: 0.0, q_k_line: 40.0, n_k: -2_000.0, v_y_k: 15.0, v_z_k: 40.0, m_y_k: 50.0, m_z_k: 10.0, torsion_k: 0.0, local_load_k: 0.0,
                    },
                ],
            }],
            connections: vec![AluminiumConnection {
                id: "support-weld".into(),
                member_id: "purlin-1".into(),
                material_id: "mat-6082".into(),
                kind: ConnectionKind::Combined,
                actions: vec![
                    MemberAction {
                        id: "G-conn".into(), kind: ActionKind::Permanent, category: ActionCategory::SelfWeight, source: ActionSource::External,
                        g_k_line: 800.0, q_k_line: 0.0, n_k: 1_500.0, v_y_k: 200.0, v_z_k: 3_500.0, m_y_k: 400.0, m_z_k: 150.0, torsion_k: 0.0, local_load_k: 0.0,
                    },
                    MemberAction {
                        id: "Q-conn".into(), kind: ActionKind::Snow, category: ActionCategory::Snow, source: ActionSource::External,
                        g_k_line: 0.0, q_k_line: 600.0, n_k: 800.0, v_y_k: 150.0, v_z_k: 2_500.0, m_y_k: 250.0, m_z_k: 100.0, torsion_k: 0.0, local_load_k: 0.0,
                    },
                ],
                bolts: default_bolts(),
                welds: WeldGroup { filler_alloy: "4043".into(), throat: 0.005, length: 0.160, beta_w: 0.63, haz_extent: 0.025 },
                load_eccentricity: 0.0,
                hole_stagger: 0.0,
                force_method: FastenerForceMethod::Elastic,
            }],
            fire_scenarios: vec![FireScenario { id: "fire-1".into(), member_id: "purlin-1".into(), theta_a: 20.0, duration_s: 30.0 }],
            fatigue_details: vec![FatigueDetail {
                id: "fat-1".into(),
                member_id: "purlin-1".into(),
                detail_category: "71".into(),
                delta_sigma_c: 71.0e6,
                delta_sigma_ed: 35.0e6,
                n_cycles: 6_000_000.0,
                m1: 4.3,
                m2: 6.3,
            }],
            cold_formed: vec![ColdFormedSheet {
                id: "sheet-pass".into(),
                material_id: "mat-6082".into(),
                thickness: 0.005,
                width: 0.080,
                span: 1.0,
                welded: false,
                actions: vec![
                    MemberAction {
                    id: "G-sheet".into(),
                    kind: ActionKind::Permanent,
                    category: ActionCategory::SelfWeight,
                    source: ActionSource::External,
                    g_k_line: 0.0,
                    q_k_line: 0.0,
                    n_k: 5926.0,
                    v_y_k: 0.0,
                    v_z_k: 0.0,
                    m_y_k: 29.6,
                    m_z_k: 0.0, torsion_k: 0.0, local_load_k: 0.0,
                },
                ],
            }],
            shells: vec![AluminiumShell {
                id: "shell-pass".into(),
                material_id: "mat-6082".into(),
                radius: 0.50,
                thickness: 0.010,
                length: 2.0,
                welded: false,
                fabrication_class: FabricationClass::B,
                actions: vec![
                    MemberAction {
                    id: "G-shell".into(),
                    kind: ActionKind::Permanent,
                    category: ActionCategory::SelfWeight,
                    source: ActionSource::External,
                    g_k_line: 800.0,
                    q_k_line: 0.0,
                    n_k: 29600000.0,
                    v_y_k: 0.0,
                    v_z_k: 3700000.0,
                    m_y_k: 12000000.0,
                    m_z_k: 0.0, torsion_k: 0.0, local_load_k: 0.0,
                },
                ],
            }],
            placements: Vec::new(),
            weld_runs: Vec::new(),
            outline_vertices: Vec::new(),
            fatigue_categories: Vec::new(),
            patch_loads: Vec::new(),
            ring_stiffeners: Vec::new(),
            sheet_profiles: Vec::new(),
            fire_insulations: Vec::new(),
            transverse_stiffeners: Vec::new(),
        }
    }

    /// ❌ Multi-failure subject: undersized section, long LTB, thin weld, high fatigue, HAZ flange.
    pub fn noncompliant_multi_fail() -> Self {
        let h = 0.080;
        let b = 0.050;
        let tf = 0.003;
        let tw = 0.003;
        Self {
            annex: AnnexChoice::De,
            reliability_class: ReliabilityClass::RC2,
            materials: vec![AluminiumMaterial { id: "mat-6060".into(), designation: "aw6060-t6".into(), product_form: ProductForm::ExtrudedProfile }],
            sections: vec![AluminiumSection {
                id: "sec-weak".into(),
                kind: SectionKind::ExtrudedI,
                height: h,
                width: b,
                flange_thickness: tf,
                web_thickness: tw,
                outer_diameter: 0.0,
                elements: vec![
                    default_plate("flange-top", (b - tw) * 0.5, tf, true, true),
                    default_plate("flange-bot", (b - tw) * 0.5, tf, true, true),
                    default_plate("web", h - 2.0 * tf, tw, false, true),
                ],
            }],
            members: vec![AluminiumMember {
                id: "beam-fail".into(),
                section_id: "sec-weak".into(),
                material_id: "mat-6060".into(),
                length: 8.0,
                support: SupportCondition::SimplySupported,
                buckling_length_y: 8.0,
                buckling_length_z: 8.0,
                buckling_length_t: 8.0,
                ltb_length: 8.0,
                c1: 1.0,
                restrained_ltb: false, bearing_length: 0.0, transverse_weld: false, transverse_weld_offset: 0.0,
                actions: vec![
                    MemberAction {
                        id: "G".into(),
                        kind: ActionKind::Permanent,
                        category: ActionCategory::SelfWeight,
                        source: ActionSource::External,
                        g_k_line: 300.0, q_k_line: 0.0,
                        n_k: 20_000.0, v_y_k: 0.0, v_z_k: 8_000.0, m_y_k: 4_000.0, m_z_k: 500.0, torsion_k: 0.0, local_load_k: 0.0,
                    },
                    MemberAction {
                        id: "Q-imposed".into(),
                        kind: ActionKind::Imposed,
                        category: ActionCategory::Office,
                        source: ActionSource::External,
                        g_k_line: 0.0, q_k_line: 500.0,
                        n_k: 40_000.0, v_y_k: 5_000.0, v_z_k: 12_000.0, m_y_k: 6_000.0, m_z_k: 1_000.0, torsion_k: 0.0, local_load_k: 0.0,
                    },
                ],
            }],
            connections: vec![
                AluminiumConnection {
                    id: "weld-thin".into(),
                    member_id: "beam-fail".into(),
                    material_id: "mat-6060".into(),
                    kind: ConnectionKind::Combined,
                    actions: vec![MemberAction {
                        id: "Q-weld".into(), kind: ActionKind::Imposed, category: ActionCategory::Office, source: ActionSource::External,
                        g_k_line: 0.0, q_k_line: 800.0, n_k: 0.0, v_y_k: 0.0, v_z_k: 40_000.0, m_y_k: 0.0, m_z_k: 0.0, torsion_k: 0.0, local_load_k: 0.0,
                    }],
                    bolts: default_bolts(),
                    welds: WeldGroup { filler_alloy: "4043".into(), throat: 0.002, length: 0.060, beta_w: 0.63, haz_extent: 0.025 },
                    load_eccentricity: 0.0,
                    hole_stagger: 0.0,
                    force_method: FastenerForceMethod::Elastic,
                },
                AluminiumConnection {
                    id: "bolt-short".into(),
                    member_id: "beam-fail".into(),
                    material_id: "mat-6060".into(),
                    kind: ConnectionKind::Combined,
                    actions: vec![
                        MemberAction {
                            id: "G-bolt".into(), kind: ActionKind::Permanent, category: ActionCategory::SelfWeight, source: ActionSource::External,
                            g_k_line: 400.0, q_k_line: 0.0, n_k: 10_000.0, v_y_k: 0.0, v_z_k: 5_000.0, m_y_k: 0.0, m_z_k: 0.0, torsion_k: 0.0, local_load_k: 0.0,
                        },
                        MemberAction {
                            id: "Q-bolt".into(), kind: ActionKind::Imposed, category: ActionCategory::Office, source: ActionSource::External,
                            g_k_line: 0.0, q_k_line: 600.0, n_k: 20_000.0, v_y_k: 0.0, v_z_k: 15_000.0, m_y_k: 0.0, m_z_k: 0.0, torsion_k: 0.0, local_load_k: 0.0,
                        },
                    ],
                    bolts: BoltGroup {
                        material: "8.8".into(),
                        diameter: 0.010,
                        rows: 1,
                        bolts_per_row: 2,
                        edge_distance: 0.012,
                        pitch: 0.030,
                        gauge: 0.014,
                        plate_thickness: 0.004,
                    },
                    welds: default_welds(),
                    load_eccentricity: 0.0,
                    hole_stagger: 0.0,
                    force_method: FastenerForceMethod::Elastic,
                },
            ],
            fire_scenarios: vec![FireScenario { id: "fire-hot".into(), member_id: "beam-fail".into(), theta_a: 320.0, duration_s: 3600.0 }],
            fatigue_details: vec![FatigueDetail {
                id: "fat-hot".into(),
                member_id: "beam-fail".into(),
                detail_category: "40-weld".into(),
                delta_sigma_c: 40.0e6,
                delta_sigma_ed: 70.0e6,
                n_cycles: 800_000.0,
                m1: 3.4,
                m2: 5.4,
            }],
            cold_formed: vec![ColdFormedSheet {
                id: "sheet-fail".into(),
                material_id: "mat-6060".into(),
                thickness: 0.0012,
                width: 0.250,
                span: 1.5,
                welded: true,
                actions: vec![
                    MemberAction {
                    id: "G-sheet".into(),
                    kind: ActionKind::Permanent,
                    category: ActionCategory::SelfWeight,
                    source: ActionSource::External,
                    g_k_line: 90.0,
                    q_k_line: 0.0,
                    n_k: 2000.0,
                    v_y_k: 0.0,
                    v_z_k: 0.0,
                    m_y_k: 800.0,
                    m_z_k: 0.0, torsion_k: 0.0, local_load_k: 0.0,
                },
                    MemberAction {
                    id: "Q-sheet".into(),
                    kind: ActionKind::Imposed,
                    category: ActionCategory::Office,
                    source: ActionSource::External,
                    g_k_line: 0.0,
                    q_k_line: 140.0,
                    n_k: 3000.0,
                    v_y_k: 0.0,
                    v_z_k: 0.0,
                    m_y_k: 1700.0,
                    m_z_k: 0.0, torsion_k: 0.0, local_load_k: 0.0,
                },
                ],
            }],
            shells: vec![AluminiumShell {
                id: "shell-fail".into(),
                material_id: "mat-6060".into(),
                radius: 0.80,
                thickness: 0.003,
                length: 3.0,
                welded: false,
                fabrication_class: FabricationClass::B,
                actions: vec![
                    MemberAction {
                    id: "G-shell".into(),
                    kind: ActionKind::Permanent,
                    category: ActionCategory::SelfWeight,
                    source: ActionSource::External,
                    g_k_line: 5000.0,
                    q_k_line: 0.0,
                    n_k: 80000000.0,
                    v_y_k: 0.0,
                    v_z_k: 20000000.0,
                    m_y_k: 50000000.0,
                    m_z_k: 0.0, torsion_k: 0.0, local_load_k: 0.0,
                },
                    MemberAction {
                    id: "Q-shell".into(),
                    kind: ActionKind::Imposed,
                    category: ActionCategory::Office,
                    source: ActionSource::External,
                    g_k_line: 0.0,
                    q_k_line: 9000.0,
                    n_k: 100000000.0,
                    v_y_k: 0.0,
                    v_z_k: 80000000.0,
                    m_y_k: 70000000.0,
                    m_z_k: 0.0, torsion_k: 0.0, local_load_k: 0.0,
                },
                ],
            }],
            placements: Vec::new(),
            weld_runs: Vec::new(),
            outline_vertices: Vec::new(),
            fatigue_categories: Vec::new(),
            patch_loads: Vec::new(),
            ring_stiffeners: Vec::new(),
            sheet_profiles: Vec::new(),
            fire_insulations: Vec::new(),
            transverse_stiffeners: Vec::new(),
        }
    }
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1999_model_typescript::LEAF, graphql: semio_s_artifact_norm_en1999_model_graphql::LEAF, json_schema: semio_s_artifact_norm_en1999_model_json_schema::LEAF, proto: semio_s_artifact_norm_en1999_model_proto::LEAF };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;

pub fn artifact_kind() -> semio_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1999", "EN 1999", EN1999_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1999_DIALECT: semio_plugin::app::Dialect = semio_plugin::app::Dialect { artifact_kind: "s.norm.en1999", standard: semio_plugin::app::StandardId("1"), subset: semio_plugin::app::SubsetId::ANY };

pub const EN1999_DOCUMENT_SCHEMA: &str = "semio.norm.en1999/v1";

#[path = "../../🚪️io/🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[path = "../../🚪️io/🪞️records/🦀️.rs"]
mod records;
