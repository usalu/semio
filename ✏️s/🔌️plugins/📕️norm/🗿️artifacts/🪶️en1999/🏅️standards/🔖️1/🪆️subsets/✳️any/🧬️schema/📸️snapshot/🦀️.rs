//! ✨️ EN 1999 snapshot — aluminium structure subject (SI: m, N, Pa, °C).

use crate::document::AnnexChoice;
use framework_schema::ArtifactSchema;

//#region 🔖️SubjectEntities

/// 🔩 Alloy temper catalogue entry (EN 1999-1-1 Table 3.2 properties resolved at evaluate).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AluminiumMaterial {
    pub id: String,
    /// Designation key, e.g. `aw6060-t6`, `aw6082-t6`, `aw5083-o`.
    pub designation: String,
}

/// 🧱 Plate element of an extruded section (b/t classification input).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PlateElement {
    pub id: String,
    /// Flat width b [m].
#[cfg_attr(test, serde(default, skip_serializing_if = "is_zero_f64"))]
    pub width: f64,
    /// Thickness t [m].
    pub thickness: f64,
    /// Outstand flange/leg when true; internal web/flange when false.
    pub outstand: bool,
    /// Element contains a weld (applies η and HAZ).
    pub welded: bool,
    /// Weld position along element from root [m] (0 = at junction); omitted when not welded.
    #[cfg_attr(test, serde(default, skip_serializing_if = "is_zero_f64"))]
    pub weld_position: f64,
}

/// 📐 Extruded aluminium cross-section.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AluminiumSection {
    pub id: String,
    /// Discriminated geometry: `extrudedI` | `channel` | `angle` | `rhs` | `box` | `tube`/`chs`.
    /// I/H/channel/angle/rhs/box use height/width/flangeThickness/webThickness; CHS/tube use outerDiameter + webThickness (wall).
    pub kind: String,
    /// Zero for CHS/tube (discriminated — omitted when 0).
    #[cfg_attr(test, serde(default, skip_serializing_if = "is_zero_f64"))]
    pub height: f64,
    #[cfg_attr(test, serde(default, skip_serializing_if = "is_zero_f64"))]
    pub width: f64,
    #[cfg_attr(test, serde(default, skip_serializing_if = "is_zero_f64"))]
    pub flange_thickness: f64,
    pub web_thickness: f64,
    /// CHS/tube outer diameter D [m]; omitted for non-tube kinds (discriminated geometry).
    #[cfg_attr(test, serde(default, skip_serializing_if = "is_non_tube_od"))]
    pub outer_diameter: f64,
    #[dsl(table)]
    pub elements: Vec<PlateElement>,
}

/// 🏋️ Characteristic action / load case on a member (EN 1990; SI).
///
/// Either `source = "udl"` (line loads converted via span/support) or `source = "external"`
/// (characteristic internal forces from structural analysis).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MemberAction {
    pub id: String,
    /// permanent | imposed | snow | wind | temperature | fire
    pub kind: String,
    /// EN 1990 ψ-category (office, snow, wind, self, …).
    pub category: String,
    /// udl | external
    pub source: String,
    /// Characteristic permanent line load g_k [N/m] (udl).
    pub g_k_line: f64,
    /// Characteristic variable line load q_k [N/m] (udl).
    pub q_k_line: f64,
    /// Characteristic axial force N_k [N] (external).
    pub n_k: f64,
    /// Characteristic shear V_y,k [N].
    pub v_y_k: f64,
    /// Characteristic shear V_z,k [N].
    pub v_z_k: f64,
    /// Characteristic moment M_y,k [N·m].
    pub m_y_k: f64,
    /// Characteristic moment M_z,k [N·m].
    pub m_z_k: f64,
}

/// 🏗️ Aluminium member with buckling lengths and restraints.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AluminiumMember {
    pub id: String,
    pub section_id: String,
    pub material_id: String,
    /// System length L [m] (span for UDL conversion; default L_cr when buckling lengths ≤ 0).
    pub length: f64,
    /// simplySupported | continuous | cantilever
    pub support: String,
    pub buckling_length_y: f64,
    pub buckling_length_z: f64,
    pub buckling_length_t: f64,
    pub ltb_length: f64,
    pub c1: f64,
    pub restrained_ltb: bool,
    #[dsl(table)]
    pub actions: Vec<MemberAction>,
}

/// 🔩 Bolted connection parameters (EN 1999-1-1 §8.5).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct WeldGroup {
    pub filler_alloy: String,
    pub throat: f64,
    pub length: f64,
    pub beta_w: f64,
    pub haz_extent: f64,
}

/// 🔗 Connection (bolted and/or welded) with design forces.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AluminiumConnection {
    pub id: String,
    pub member_id: String,
    pub material_id: String,
    /// `bolted` | `welded` | `combined`.
    pub kind: String,
    /// Characteristic connection load cases (same EN 1990 model as members).
    #[dsl(table)]
    pub actions: Vec<MemberAction>,
    pub bolts: BoltGroup,
    pub welds: WeldGroup,
}

/// 🔥️ Fire assessment scenario (EN 1999-1-2).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FireScenario {
    pub id: String,
    pub member_id: String,
    pub theta_a: f64,
    pub duration_s: f64,
}

/// 🔄️ Fatigue detail (EN 1999-1-3).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueDetail {
    pub id: String,
    pub member_id: String,
    /// Annex J detail category id (e.g. `71`, `40-weld`).
    pub detail_category: String,
    /// Detail category Δσ_C [Pa] — derived from detailCategory (one source of truth; not serialized).
    #[cfg_attr(test, serde(default, skip_serializing))]
    pub delta_sigma_c: f64,
    /// Constant-amplitude stress range Δσ [Pa] (single-range spectrum entry).
    pub delta_sigma_ed: f64,
    /// Cycles in the spectrum block n_i.
    pub n_cycles: f64,
    /// First slope m1 (N ≤ N_D).
    pub m1: f64,
    /// Second slope m2 (N_D < N ≤ N_L).
    pub m2: f64,
}

/// 📄 Cold-formed aluminium sheeting (EN 1999-1-4).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ColdFormedSheet {
    pub id: String,
    pub material_id: String,
    /// Design thickness t [m].
    pub thickness: f64,
    /// Flat width between webs / supports b [m].
    pub width: f64,
    /// Span L [m].
    pub span: f64,
    pub welded: bool,
    /// Characteristic EN 1990 actions; design N_Ed/M_Ed from governing ULS/SLS.
    pub actions: Vec<MemberAction>,
}

/// 🫙 Aluminium shell of revolution (EN 1999-1-5).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AluminiumShell {
    pub id: String,
    pub material_id: String,
    /// Mid-surface radius r [m].
    pub radius: f64,
    /// Wall thickness t [m].
    pub thickness: f64,
    /// Meridian length L [m].
    pub length: f64,
    /// Characteristic membrane actions (n_k→σ_x,k, m_y_k→σ_θ,k, v_z_k→τ_k [Pa]).
    pub actions: Vec<MemberAction>,
}

//#endregion 🔖️SubjectEntities

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1999 aluminium-structure subject.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1999", layout = "lines")]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    #[dsl(table)]
    pub materials: Vec<AluminiumMaterial>,
    #[state(artifact)]
    #[dsl(table)]
    pub sections: Vec<AluminiumSection>,
    #[state(artifact)]
    #[dsl(table)]
    pub members: Vec<AluminiumMember>,
    #[state(artifact)]
    #[dsl(table)]
    pub connections: Vec<AluminiumConnection>,
    #[state(artifact)]
    #[dsl(table)]
    pub fire_scenarios: Vec<FireScenario>,
    #[state(artifact)]
    #[dsl(table)]
    pub fatigue_details: Vec<FatigueDetail>,
    #[state(artifact)]
    #[dsl(table)]
    pub cold_formed: Vec<ColdFormedSheet>,
    #[state(artifact)]
    #[dsl(table)]
    pub shells: Vec<AluminiumShell>,
}

//#endregion 🔖️Snapshot

crate::impl_norm_artifact_record!(En1999Snapshot, extension = "en1999", envelope_id = "norm.en1999");

fn is_zero_f64(v: &f64) -> bool { *v == 0.0 }
fn is_non_tube_od(v: &f64) -> bool { *v == 0.0 }

fn default_plate(id: &str, width: f64, thickness: f64, outstand: bool, welded: bool) -> PlateElement {
    PlateElement { id: id.into(), width, thickness, outstand, welded, weld_position: if welded { 0.005 } else { 0.0 } }
}

fn default_bolts() -> BoltGroup {
    BoltGroup {
        material: "8.8".into(),
        diameter: 0.012,
        rows: 2,
        bolts_per_row: 2,
        edge_distance: 0.024,
        pitch: 0.036,
        gauge: 0.016,
        plate_thickness: 0.008,
    }
}

fn default_welds() -> WeldGroup {
    WeldGroup { filler_alloy: "4043".into(), throat: 0.004, length: 0.120, beta_w: 0.63, haz_extent: 0.025 }
}

impl Default for En1999Snapshot {
    fn default() -> Self {
        Self::compliant_roof_purlin()
    }
}

impl En1999Snapshot {
    /// 📭 Empty subject shell (tests / set-snapshot baseline).
    pub fn empty() -> Self {
        Self {
            annex: crate::document::AnnexChoice::En,
            materials: Vec::new(),
            sections: Vec::new(),
            members: Vec::new(),
            connections: Vec::new(),
            fire_scenarios: Vec::new(),
            fatigue_details: Vec::new(),
            cold_formed: Vec::new(),
            shells: Vec::new(),
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
            materials: vec![AluminiumMaterial { id: "mat-6082".into(), designation: "aw6082-t6".into() }],
            sections: vec![AluminiumSection {
                id: "sec-i120".into(),
                kind: "extrudedI".into(),
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
                kind: "tube".into(),
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
                support: "simplySupported".into(),
                buckling_length_y: 1.5,
                buckling_length_z: 1.5,
                buckling_length_t: 1.5,
                ltb_length: 1.5,
                c1: 1.132,
                restrained_ltb: false,
                actions: vec![
                    MemberAction {
                        id: "G".into(),
                        kind: "permanent".into(),
                        category: "self".into(),
                        source: "udl".into(),
                        g_k_line: 400.0, q_k_line: 0.0,
                        n_k: 2_000.0, v_y_k: 0.0, v_z_k: 0.0, m_y_k: 0.0, m_z_k: 0.0,
                    },
                    MemberAction {
                        id: "Q-snow".into(),
                        kind: "snow".into(),
                        category: "snow".into(),
                        source: "udl".into(),
                        g_k_line: 0.0, q_k_line: 900.0,
                        n_k: 1_200.0, v_y_k: 50.0, v_z_k: 200.0, m_y_k: 100.0, m_z_k: 40.0,
                    },
                ],
            },
            AluminiumMember {
                id: "post-chs".into(),
                section_id: "sec-chs168".into(),
                material_id: "mat-6082".into(),
                length: 3.0,
                support: "cantilever".into(),
                buckling_length_y: 3.0,
                buckling_length_z: 3.0,
                buckling_length_t: 3.0,
                ltb_length: 3.0,
                c1: 1.0,
                restrained_ltb: false,
                actions: vec![
                    MemberAction {
                        id: "G-post".into(), kind: "permanent".into(), category: "self".into(), source: "external".into(),
                        g_k_line: 50.0, q_k_line: 0.0, n_k: -8_000.0, v_y_k: 20.0, v_z_k: 80.0, m_y_k: 120.0, m_z_k: 15.0,
                    },
                    MemberAction {
                        id: "Q-post".into(), kind: "imposed".into(), category: "office".into(), source: "external".into(),
                        g_k_line: 0.0, q_k_line: 40.0, n_k: -2_000.0, v_y_k: 15.0, v_z_k: 40.0, m_y_k: 50.0, m_z_k: 10.0,
                    },
                ],
            }],
            connections: vec![AluminiumConnection {
                id: "support-weld".into(),
                member_id: "purlin-1".into(),
                material_id: "mat-6082".into(),
                kind: "combined".into(),
                actions: vec![
                    MemberAction {
                        id: "G-conn".into(), kind: "permanent".into(), category: "self".into(), source: "external".into(),
                        g_k_line: 800.0, q_k_line: 0.0, n_k: 1_500.0, v_y_k: 200.0, v_z_k: 3_500.0, m_y_k: 400.0, m_z_k: 150.0,
                    },
                    MemberAction {
                        id: "Q-conn".into(), kind: "snow".into(), category: "snow".into(), source: "external".into(),
                        g_k_line: 0.0, q_k_line: 600.0, n_k: 800.0, v_y_k: 150.0, v_z_k: 2_500.0, m_y_k: 250.0, m_z_k: 100.0,
                    },
                ],
                bolts: default_bolts(),
                welds: WeldGroup { filler_alloy: "4043".into(), throat: 0.005, length: 0.160, beta_w: 0.63, haz_extent: 0.025 },
            }],
            fire_scenarios: vec![FireScenario { id: "fire-1".into(), member_id: "purlin-1".into(), theta_a: 180.0, duration_s: 600.0 }],
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
                    kind: "permanent".into(),
                    category: "self".into(),
                    source: "external".into(),
                    g_k_line: 120.0,
                    q_k_line: 0.0,
                    n_k: 5926.0,
                    v_y_k: 0.0,
                    v_z_k: 0.0,
                    m_y_k: 29.6,
                    m_z_k: 0.0,
                },
                ],
            }],
            shells: vec![AluminiumShell {
                id: "shell-pass".into(),
                material_id: "mat-6082".into(),
                radius: 0.50,
                thickness: 0.010,
                length: 2.0,
                actions: vec![
                    MemberAction {
                    id: "G-shell".into(),
                    kind: "permanent".into(),
                    category: "self".into(),
                    source: "external".into(),
                    g_k_line: 800.0,
                    q_k_line: 0.0,
                    n_k: 29600000.0,
                    v_y_k: 0.0,
                    v_z_k: 3700000.0,
                    m_y_k: 22200000.0,
                    m_z_k: 0.0,
                },
                ],
            }],
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
            materials: vec![AluminiumMaterial { id: "mat-6060".into(), designation: "aw6060-t6".into() }],
            sections: vec![AluminiumSection {
                id: "sec-weak".into(),
                kind: "extrudedI".into(),
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
                support: "simplySupported".into(),
                buckling_length_y: 8.0,
                buckling_length_z: 8.0,
                buckling_length_t: 8.0,
                ltb_length: 8.0,
                c1: 1.0,
                restrained_ltb: false,
                actions: vec![
                    MemberAction {
                        id: "G".into(),
                        kind: "permanent".into(),
                        category: "self".into(),
                        source: "external".into(),
                        g_k_line: 300.0, q_k_line: 0.0,
                        n_k: 20_000.0, v_y_k: 0.0, v_z_k: 8_000.0, m_y_k: 4_000.0, m_z_k: 500.0,
                    },
                    MemberAction {
                        id: "Q-imposed".into(),
                        kind: "imposed".into(),
                        category: "office".into(),
                        source: "external".into(),
                        g_k_line: 0.0, q_k_line: 500.0,
                        n_k: 40_000.0, v_y_k: 5_000.0, v_z_k: 12_000.0, m_y_k: 6_000.0, m_z_k: 1_000.0,
                    },
                ],
            }],
            connections: vec![
                AluminiumConnection {
                    id: "weld-thin".into(),
                    member_id: "beam-fail".into(),
                    material_id: "mat-6060".into(),
                    kind: "combined".into(),
                    actions: vec![MemberAction {
                        id: "Q-weld".into(), kind: "imposed".into(), category: "office".into(), source: "external".into(),
                        g_k_line: 0.0, q_k_line: 800.0, n_k: 0.0, v_y_k: 0.0, v_z_k: 40_000.0, m_y_k: 0.0, m_z_k: 0.0,
                    }],
                    bolts: default_bolts(),
                    welds: WeldGroup { filler_alloy: "4043".into(), throat: 0.002, length: 0.060, beta_w: 0.63, haz_extent: 0.025 },
                },
                AluminiumConnection {
                    id: "bolt-short".into(),
                    member_id: "beam-fail".into(),
                    material_id: "mat-6060".into(),
                    kind: "combined".into(),
                    actions: vec![
                        MemberAction {
                            id: "G-bolt".into(), kind: "permanent".into(), category: "self".into(), source: "external".into(),
                            g_k_line: 400.0, q_k_line: 0.0, n_k: 10_000.0, v_y_k: 0.0, v_z_k: 5_000.0, m_y_k: 0.0, m_z_k: 0.0,
                        },
                        MemberAction {
                            id: "Q-bolt".into(), kind: "imposed".into(), category: "office".into(), source: "external".into(),
                            g_k_line: 0.0, q_k_line: 600.0, n_k: 20_000.0, v_y_k: 0.0, v_z_k: 15_000.0, m_y_k: 0.0, m_z_k: 0.0,
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
                    kind: "permanent".into(),
                    category: "self".into(),
                    source: "external".into(),
                    g_k_line: 90.0,
                    q_k_line: 0.0,
                    n_k: 2000.0,
                    v_y_k: 0.0,
                    v_z_k: 0.0,
                    m_y_k: 800.0,
                    m_z_k: 0.0,
                },
                    MemberAction {
                    id: "Q-sheet".into(),
                    kind: "imposed".into(),
                    category: "office".into(),
                    source: "external".into(),
                    g_k_line: 0.0,
                    q_k_line: 140.0,
                    n_k: 3000.0,
                    v_y_k: 0.0,
                    v_z_k: 0.0,
                    m_y_k: 1700.0,
                    m_z_k: 0.0,
                },
                ],
            }],
            shells: vec![AluminiumShell {
                id: "shell-fail".into(),
                material_id: "mat-6060".into(),
                radius: 0.80,
                thickness: 0.003,
                length: 3.0,
                actions: vec![
                    MemberAction {
                    id: "G-shell".into(),
                    kind: "permanent".into(),
                    category: "self".into(),
                    source: "external".into(),
                    g_k_line: 5000.0,
                    q_k_line: 0.0,
                    n_k: 80000000.0,
                    v_y_k: 0.0,
                    v_z_k: 20000000.0,
                    m_y_k: 50000000.0,
                    m_z_k: 0.0,
                },
                    MemberAction {
                    id: "Q-shell".into(),
                    kind: "imposed".into(),
                    category: "office".into(),
                    source: "external".into(),
                    g_k_line: 0.0,
                    q_k_line: 9000.0,
                    n_k: 100000000.0,
                    v_y_k: 0.0,
                    v_z_k: 80000000.0,
                    m_y_k: 70000000.0,
                    m_z_k: 0.0,
                },
                ],
            }],
        }
    }
}

//#region 🌉️ExternalCodecBridge
pub fn encode_en1999_snapshot_json(snapshot: &En1999Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

pub fn decode_en1999_snapshot_json(text: &str) -> Result<En1999Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

pub fn decode_en1999_dsl(text: &str) -> Result<En1999Snapshot, String> {
    <En1999Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

pub fn encode_en1999_dsl(snapshot: &En1999Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

pub fn decode_en1999_pack(bytes: &[u8]) -> Result<En1999Snapshot, String> {
    <En1999Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

pub fn encode_en1999_pack(snapshot: &En1999Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
