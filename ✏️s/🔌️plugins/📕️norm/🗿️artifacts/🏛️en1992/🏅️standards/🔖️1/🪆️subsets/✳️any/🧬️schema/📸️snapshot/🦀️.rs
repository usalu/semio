//! 🧬️ En1992 snapshot — complete reinforced/prestressed concrete structure subject (SI).

use crate::document::AnnexChoice;
use crate::{
    Anchor, BarLayer, ConcreteGrade, ExposureClass, FireRating, FireSpec, LoadCaseActions, MemberKind, PrestressSpec, PrestressSteel, PunchingSpec, RcMember, ReinforcementGrade, Stirrups, SupportCondition,
    TightnessClass,
};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1992", layout = "lines")]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub title: String,
    #[state(artifact)]
    pub design_working_life_years: f64,
    #[state(artifact)]
    pub delta_c_dev: f64,
    #[state(artifact)]
    pub cement_type: String,
    #[dsl(table)]
    #[state(artifact)]
    pub concrete_grades: Vec<ConcreteGrade>,
    #[dsl(table)]
    #[state(artifact)]
    pub reinforcement_grades: Vec<ReinforcementGrade>,
    #[dsl(table)]
    #[state(artifact)]
    pub prestress_steels: Vec<PrestressSteel>,
    #[dsl(table)]
    #[state(artifact)]
    pub members: Vec<RcMember>,
    #[dsl(table)]
    #[state(artifact)]
    pub anchors: Vec<Anchor>,
}

crate::impl_norm_artifact_record!(En1992Snapshot, extension = "en1992", envelope_id = "norm.en1992");

impl Default for En1992Snapshot {
    fn default() -> Self {
        Self::compliant_office_frame()
    }
}

impl En1992Snapshot {
    /// 🏢 Realistic DE-NA compliant multi-member RC office frame (default subject).
    pub fn compliant_office_frame() -> Self {
        Self {
            annex: AnnexChoice::De,
            title: "Office frame RC assessment".into(),
            design_working_life_years: 50.0,
            delta_c_dev: 0.010,
            cement_type: "N".into(),
            concrete_grades: vec![
                ConcreteGrade::from_f_ck("c30", "C30/37", 30.0e6),
                ConcreteGrade::from_f_ck("c12", "C12/15", 12.0e6),
                ConcreteGrade::from_f_ck("c50", "C50/60", 50.0e6),
                ConcreteGrade::from_f_ck("c100", "C100/115", 100.0e6),
            ],
            reinforcement_grades: vec![ReinforcementGrade::b500b("b500"), ReinforcementGrade::b500a("b500a")],
            prestress_steels: vec![PrestressSteel { id: "yp1860".into(), name: "Y1860S7".into(), f_pk: 1860.0e6, f_p0_1k: 1640.0e6 }],
            members: vec![
                RcMember {
                    id: "beam-B1".into(),
                    label_en: "Beam B1".into(),
                    label_de: "Träger B1".into(),
                    kind: MemberKind::Beam,
                    concrete_grade_id: "c30".into(),
                    reinforcement_grade_id: "b500".into(),
                    prestress_steel_id: String::new(),
                    exposure: ExposureClass::Xc3,
                    width: 0.30,
                    height: 0.50,
                    effective_depth: 0.450,
                    cover: 0.035,
                    span: 6.0,
                    support: SupportCondition::SimplySupported,
                    buckling_length: 0.0,
                    longitudinal: vec![BarLayer { id: "bot".into(), diameter: 0.020, count: 4, position: "bottom".into(), anchorage_length: 0.70, lap_length: 0.85, bond_condition: "good".into(), aggregate_size: 0.016 }],
                    stirrups: Some(Stirrups { diameter: 0.008, spacing: 0.150, legs: 2 }),
                    punching: None,
                    prestress: None,
                    fire: Some(FireSpec { rating: FireRating::R60, axis_distance: 0.040, column_method: "A".into(), slab_system: "one-way".into() }),
                    actions: vec![
                        LoadCaseActions { id: "G".into(), kind: "permanent".into(), category: "self".into(), source: "udl".into(), g_k_line: 12.0e3, q_k_line: 0.0, point_force: 0.0, m_k: 0.0, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 0.0 },
                        LoadCaseActions { id: "Q-office".into(), kind: "imposed".into(), category: "office".into(), source: "udl".into(), g_k_line: 0.0, q_k_line: 8.0e3, point_force: 25.0e3, m_k: 0.0, n_k: 0.0, v_k: 0.0, t_k: 8.0e3, v_k_punch: 0.0 },
                        LoadCaseActions { id: "W".into(), kind: "wind".into(), category: "wind".into(), source: "udl".into(), g_k_line: 0.0, q_k_line: 2.0e3, point_force: 0.0, m_k: 0.0, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 0.0 },
                        LoadCaseActions { id: "A-impact".into(), kind: "accidental".into(), category: "accidental".into(), source: "external".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 40.0e3, n_k: 0.0, v_k: 20.0e3, t_k: 0.0, v_k_punch: 0.0 },
                    ],
                    use_fem: false,
                    udl: 0.0,
                    deflection_sensitive: true,
                    tightness: None,
                    hd_over_h: 0.0,
                    liquid_sigma_s: 0.0,
                    liquid_rho_p_eff: 0.0,
                    liquid_f_ct_eff: 0.0,
                    liquid_s_r_max: 0.0,
                    bridge_sigma_c: 0.0,
                    bridge_delta_sigma_s: 0.0,
                },
                RcMember {
                    id: "col-C1".into(),
                    label_en: "Column C1".into(),
                    label_de: "Stütze C1".into(),
                    kind: MemberKind::Column,
                    concrete_grade_id: "c30".into(),
                    reinforcement_grade_id: "b500".into(),
                    prestress_steel_id: String::new(),
                    exposure: ExposureClass::Xc1,
                    width: 0.30,
                    height: 0.30,
                    effective_depth: 0.260,
                    cover: 0.030,
                    span: 3.5,
                    support: SupportCondition::Fixed,
                    buckling_length: 2.0,
                    longitudinal: vec![BarLayer { id: "vert".into(), diameter: 0.020, count: 8, position: "bottom".into(), anchorage_length: 0.70, lap_length: 0.85, bond_condition: "good".into(), aggregate_size: 0.016 }],
                    stirrups: Some(Stirrups { diameter: 0.008, spacing: 0.200, legs: 2 }),
                    punching: None,
                    prestress: None,
                    fire: Some(FireSpec { rating: FireRating::R60, axis_distance: 0.035, column_method: "A".into(), slab_system: "".into() }),
                    actions: vec![
                        LoadCaseActions { id: "G".into(), kind: "permanent".into(), category: "self".into(), source: "external".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 25.0e3, n_k: -500.0e3, v_k: 20.0e3, t_k: 0.0, v_k_punch: 0.0 },
                        LoadCaseActions { id: "Q-office".into(), kind: "imposed".into(), category: "office".into(), source: "external".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 15.0e3, n_k: -200.0e3, v_k: 15.0e3, t_k: 0.0, v_k_punch: 0.0 },
                    ],
                    use_fem: true,
                    udl: 0.0,
                    deflection_sensitive: false,
                    tightness: None,
                    hd_over_h: 0.0,
                    liquid_sigma_s: 0.0,
                    liquid_rho_p_eff: 0.0,
                    liquid_f_ct_eff: 0.0,
                    liquid_s_r_max: 0.0,
                    bridge_sigma_c: 0.0,
                    bridge_delta_sigma_s: 0.0,
                },
                RcMember {
                    id: "slab-S1".into(),
                    label_en: "Flat slab S1 punching".into(),
                    label_de: "Flachdecke S1 Durchstanzen".into(),
                    kind: MemberKind::FlatSlab,
                    concrete_grade_id: "c30".into(),
                    reinforcement_grade_id: "b500".into(),
                    prestress_steel_id: String::new(),
                    exposure: ExposureClass::Xc1,
                    width: 1.0,
                    height: 0.25,
                    effective_depth: 0.210,
                    cover: 0.025,
                    span: 7.0,
                    support: SupportCondition::Continuous,
                    buckling_length: 0.0,
                    longitudinal: vec![BarLayer { id: "bot".into(), diameter: 0.012, count: 8, position: "bottom".into(), anchorage_length: 0.55, lap_length: 0.70, bond_condition: "good".into(), aggregate_size: 0.016 }],
                    stirrups: None,
                    punching: Some(PunchingSpec { column_width: 0.30, column_depth: 0.30, column_position: "interior".into(), asw: 0.0 }),
                    prestress: None,
                    fire: Some(FireSpec { rating: FireRating::R60, axis_distance: 0.025, column_method: "".into(), slab_system: "flat".into() }),
                    actions: vec![
                        LoadCaseActions { id: "G".into(), kind: "permanent".into(), category: "self".into(), source: "external".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 25.0e3, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 80.0e3 },
                        LoadCaseActions { id: "Q-office".into(), kind: "imposed".into(), category: "office".into(), source: "external".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 15.0e3, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 45.0e3 },
                    ],
                    use_fem: true,
                    udl: 0.0,
                    deflection_sensitive: true,
                    tightness: None,
                    hd_over_h: 0.0,
                    liquid_sigma_s: 0.0,
                    liquid_rho_p_eff: 0.0,
                    liquid_f_ct_eff: 0.0,
                    liquid_s_r_max: 0.0,
                    bridge_sigma_c: 0.0,
                    bridge_delta_sigma_s: 0.0,
                },
                RcMember {
                    id: "beam-PS1".into(),
                    label_en: "Prestressed beam PS1".into(),
                    label_de: "Vorgespannter Träger PS1".into(),
                    kind: MemberKind::Beam,
                    concrete_grade_id: "c50".into(),
                    reinforcement_grade_id: "b500".into(),
                    prestress_steel_id: "yp1860".into(),
                    exposure: ExposureClass::Xc1,
                    width: 0.35,
                    height: 0.70,
                    effective_depth: 0.620,
                    cover: 0.040,
                    span: 12.0,
                    support: SupportCondition::SimplySupported,
                    buckling_length: 0.0,
                    longitudinal: vec![BarLayer { id: "bot".into(), diameter: 0.016, count: 4, position: "bottom".into(), anchorage_length: 0.80, lap_length: 1.00, bond_condition: "good".into(), aggregate_size: 0.016 }],
                    stirrups: Some(Stirrups { diameter: 0.010, spacing: 0.200, legs: 2 }),
                    punching: None,
                    prestress: Some(PrestressSpec { force: 1200.0e3, area: 1500.0e-6, eccentricity: 0.180, loss_ratio: 0.20 }),
                    fire: Some(FireSpec { rating: FireRating::R90, axis_distance: 0.055, column_method: "".into(), slab_system: "".into() }),
                    actions: vec![
                        LoadCaseActions { id: "G".into(), kind: "permanent".into(), category: "self".into(), source: "udl".into(), g_k_line: 15.0e3, q_k_line: 0.0, point_force: 0.0, m_k: 0.0, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 0.0 },
                        LoadCaseActions { id: "Q-office".into(), kind: "imposed".into(), category: "office".into(), source: "udl".into(), g_k_line: 0.0, q_k_line: 8.0e3, point_force: 0.0, m_k: 0.0, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 0.0 },
                    ],
                    use_fem: false,
                    udl: 0.0,
                    deflection_sensitive: false,
                    tightness: None,
                    hd_over_h: 0.0,
                    liquid_sigma_s: 0.0,
                    liquid_rho_p_eff: 0.0,
                    liquid_f_ct_eff: 0.0,
                    liquid_s_r_max: 0.0,
                    bridge_sigma_c: 0.0,
                    bridge_delta_sigma_s: 0.0,
                },
            ],
            anchors: vec![Anchor {
                id: "anc-1".into(),
                h_ef: 0.080,
                cracked: false,
                f_uk: 800.0e6,
                f_yk: 640.0e6,
                a_s: 84.3e-6,
                d: 0.012,
                c1: 0.100,
                f_ck: 30.0e6,
                actions: vec![
                    LoadCaseActions { id: "G".into(), kind: "permanent".into(), category: "self".into(), source: "external".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 0.0, n_k: 6.0e3, v_k: 3.0e3, t_k: 0.0, v_k_punch: 0.0 },
                    LoadCaseActions { id: "Q-office".into(), kind: "imposed".into(), category: "office".into(), source: "external".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 0.0, n_k: 4.0e3, v_k: 2.0e3, t_k: 0.0, v_k_punch: 0.0 },
                ],
            }],
        }
    }

    /// ❌️ Non-compliant subject with multiple ULS/SLS/fire failures.
    pub fn failing_under_reinforced() -> Self {
        let mut snap = Self::compliant_office_frame();
        snap.title = "Failing under-reinforced frame".into();
        if let Some(beam) = snap.members.iter_mut().find(|m| m.id == "beam-B1") {
            beam.longitudinal = vec![BarLayer { id: "bot".into(), diameter: 0.012, count: 2, position: "bottom".into(), anchorage_length: 0.20, lap_length: 0.25, bond_condition: "good".into(), aggregate_size: 0.016 }];
            beam.stirrups = Some(Stirrups { diameter: 0.006, spacing: 0.300, legs: 2 });
            beam.cover = 0.015;
            beam.fire = Some(FireSpec { rating: FireRating::R90, axis_distance: 0.015, column_method: "".into(), slab_system: "".into() });
            for a in &mut beam.actions {
                if a.id == "Q-office" {
                    a.q_k_line = 25.0e3;
                    a.point_force = 40.0e3;
                    a.t_k = 20.0e3;
                }
            }
        }
        if let Some(slab) = snap.members.iter_mut().find(|m| m.id == "slab-S1") {
            for a in &mut slab.actions {
                if a.id == "Q-office" {
                    a.v_k_punch = 300.0e3;
                }
            }
        }
        if let Some(ps) = snap.members.iter_mut().find(|m| m.id == "beam-PS1") {
            if let Some(pre) = &mut ps.prestress {
                pre.force = 3500.0e3; // overstress at transfer
                pre.eccentricity = 0.280;
                pre.loss_ratio = 0.05;
            }
        }
        if let Some(anc) = snap.anchors.first_mut() {
            for a in &mut anc.actions {
                if a.id == "Q-office" {
                    a.n_k = 35.0e3;
                    a.v_k = 15.0e3;
                }
            }
            anc.h_ef = 0.050;
        }
        snap
    }

    /// 🗜️ Failing prestressed member — transfer stress / losses exceed §5.10 limits.
    pub fn failing_prestressed_beam() -> Self {
        let mut snap = Self::compliant_office_frame();
        snap.title = "Failing prestressed beam".into();
        snap.members.retain(|m| m.id == "beam-PS1");
        if let Some(ps) = snap.members.first_mut() {
            if let Some(pre) = &mut ps.prestress {
                pre.force = 4000.0e3;
                pre.eccentricity = 0.300;
                pre.loss_ratio = 0.02;
            }
            ps.cover = 0.015;
        }
        snap.anchors.clear();
        snap
    }

    /// 🛢️ Liquid-retaining wall example with FEM characteristic effects + anchor.
    pub fn liquid_retaining_fem_anchor() -> Self {
        let mut snap = Self::compliant_office_frame();
        snap.title = "Liquid-retaining tank with FEM and anchor".into();
        snap.concrete_grades = vec![ConcreteGrade::from_f_ck("c35", "C35/45", 35.0e6)];
        snap.members = vec![RcMember {
            id: "wall-W1".into(),
            label_en: "Tank wall W1".into(),
            label_de: "Behälterwand W1".into(),
            kind: MemberKind::LiquidRetaining,
            concrete_grade_id: "c35".into(),
            reinforcement_grade_id: "b500".into(),
            prestress_steel_id: String::new(),
            exposure: ExposureClass::Xc4,
            width: 0.35,
            height: 0.50,
            effective_depth: 0.450,
            cover: 0.040,
            span: 7.5,
            support: SupportCondition::Continuous,
            buckling_length: 0.0,
            longitudinal: vec![BarLayer { id: "vert".into(), diameter: 0.016, count: 6, position: "bottom".into(), anchorage_length: 0.70, lap_length: 0.85, bond_condition: "good".into(), aggregate_size: 0.016 }],
            stirrups: Some(Stirrups { diameter: 0.008, spacing: 0.150, legs: 2 }),
            punching: None,
            prestress: None,
            fire: Some(FireSpec { rating: FireRating::R90, axis_distance: 0.040, column_method: "".into(), slab_system: "".into() }),
            actions: vec![
                LoadCaseActions { id: "G".into(), kind: "permanent".into(), category: "self".into(), source: "external".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 100.0e3, n_k: 30.0e3, v_k: 50.0e3, t_k: 0.0, v_k_punch: 0.0 },
                LoadCaseActions { id: "Q-office".into(), kind: "imposed".into(), category: "office".into(), source: "external".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 50.0e3, n_k: 15.0e3, v_k: 30.0e3, t_k: 0.0, v_k_punch: 0.0 },
            ],
            use_fem: true,
            udl: 24000.0,
            deflection_sensitive: false,
            tightness: Some(TightnessClass::Tc2),
            hd_over_h: 8.0,
            liquid_sigma_s: 180.0e6,
            liquid_rho_p_eff: 0.014,
            liquid_f_ct_eff: 3.2e6,
            liquid_s_r_max: 0.22,
            bridge_sigma_c: 0.0,
            bridge_delta_sigma_s: 0.0,
        }];
        snap.anchors = vec![Anchor {
            id: "anc-1".into(),
            h_ef: 0.100,
            cracked: true,
            f_uk: 900.0e6,
            f_yk: 720.0e6,
            a_s: 100.0e-6,
            d: 0.012,
            c1: 0.120,
            f_ck: 35.0e6,
            actions: vec![
                LoadCaseActions { id: "G".into(), kind: "permanent".into(), category: "self".into(), source: "external".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 0.0, n_k: 8.0e3, v_k: 4.0e3, t_k: 0.0, v_k_punch: 0.0 },
                LoadCaseActions { id: "Q-office".into(), kind: "imposed".into(), category: "office".into(), source: "external".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 0.0, n_k: 5.0e3, v_k: 3.0e3, t_k: 0.0, v_k_punch: 0.0 },
            ],
        }];
        snap
    }

    /// 🔎 Resolve concrete grade.
    pub fn concrete(&self, id: &str) -> Option<&ConcreteGrade> {
        self.concrete_grades.iter().find(|g| g.id == id)
    }

    /// 🔎 Resolve reinforcement grade.
    pub fn reinforcement(&self, id: &str) -> Option<&ReinforcementGrade> {
        self.reinforcement_grades.iter().find(|g| g.id == id)
    }
}

//#endregion 🔖️Snapshot

//#region 🌉️ExternalCodecBridge
/// 📤️ Canonical JSON projection of [`En1992Snapshot`].
pub fn encode_en1992_snapshot_json(snapshot: &En1992Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ Inverse of [`encode_en1992_snapshot_json`].
pub fn decode_en1992_snapshot_json(text: &str) -> Result<En1992Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses committed `.dsl.semio` into [`En1992Snapshot`].
pub fn decode_en1992_dsl(text: &str) -> Result<En1992Snapshot, String> {
    <En1992Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints [`En1992Snapshot`] to canonical `.dsl.semio`.
pub fn encode_en1992_dsl(snapshot: &En1992Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes `.pack.semio` envelope.
pub fn decode_en1992_pack(bytes: &[u8]) -> Result<En1992Snapshot, String> {
    <En1992Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes `.pack.semio` envelope.
pub fn encode_en1992_pack(snapshot: &En1992Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
