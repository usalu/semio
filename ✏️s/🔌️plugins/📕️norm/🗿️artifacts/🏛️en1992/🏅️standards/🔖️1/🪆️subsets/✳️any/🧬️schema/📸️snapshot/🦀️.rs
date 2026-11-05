//! 🧬️ En1992 snapshot — complete reinforced/prestressed concrete structure subject (SI).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};
pub use semio_s_plugin_norm_compliance::design_case::{CaseNumber, CaseRow, CaseText};

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

/// 🧱 Cement strength development class (EN 1992-1-1 3.1.2 (6)).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum CementClass {
    S,
    N,
    R,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
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
    pub cement_type: CementClass,
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
    #[dsl(table)]
    #[state(artifact)]
    pub placements: Vec<Placement>,
    #[dsl(table)]
    #[state(artifact)]
    pub cases: Vec<CaseRow>,
    #[state(artifact)]
    pub reliability_class: ReliabilityClass,
}

impl En1992Snapshot {
    /// 📸️ Reference EN 1992 subject under the explicitly chosen annex; the annex is never defaulted.
    pub fn example(annex: semio_s_plugin_norm_compliance::AnnexChoice) -> Self {
        Self { annex: annex, ..Self::compliant_office_frame() }
    }

    /// ⚖️ The partial and combination factors of the document (EN 1990 Annex A1 and B): its annex and reliability class (K_FI scales the partial factors); the one place every action combination of this artifact reads them from.
    pub fn action_factors(&self) -> semio_s_plugin_norm_compliance::ActionFactors {
        semio_s_plugin_norm_compliance::ActionFactors::new(self.annex, self.reliability_class, semio_s_plugin_norm_compliance::DesignSituation::Persistent)
    }

    /// 🏢 Realistic DE-NA compliant multi-member RC office frame (default subject).
    pub fn compliant_office_frame() -> Self {
        Self {
            annex: AnnexChoice::De,
            reliability_class: ReliabilityClass::RC2,
            title: "Office frame RC assessment".into(),
            design_working_life_years: 50.0,
            delta_c_dev: 0.010,
            cement_type: CementClass::N,
            concrete_grades: vec![
                ConcreteGrade::from_f_ck("c30", "C30/37", 30.0e6),
                ConcreteGrade::from_f_ck("c12", "C12/15", 12.0e6),
                ConcreteGrade::from_f_ck("c50", "C50/60", 50.0e6),
                ConcreteGrade::from_f_ck("c100", "C100/115", 100.0e6),
            ],
            reinforcement_grades: vec![ReinforcementGrade::b500b("b500"), ReinforcementGrade::b500a("b500a"), ReinforcementGrade::b500c("b500c")],
            prestress_steels: vec![PrestressSteel { id: "yp1860".into(), name: "Y1860S7".into(), f_pk: 1860.0e6, f_p0_1k: 1600.0e6 }, PrestressSteel { id: "yp1860b".into(), name: "Y1770S7".into(), f_pk: 1770.0e6, f_p0_1k: 1520.0e6 }],
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
                    longitudinal: vec![BarLayer { id: "bot".into(), diameter: 0.020, count: 4, position: BarPosition::Bottom, anchorage_length: 0.70, lap_length: 0.85, bond_condition: BondCondition::Good, aggregate_size: 0.016, lap_share: 1.0 }],
                    stirrups: Some(Stirrups { diameter: 0.008, spacing: 0.150, legs: 2 }),
                    punching: None,
                    prestress: None,
                    fire: Some(FireSpec { rating: FireRating::R60, axis_distance: 0.040, slab_system: SlabFireSystem::OneWay }),
                    actions: vec![
                        LoadCaseActions { id: "G".into(), kind: ActionKind::Permanent, category: PsiCategory::SelfWeight, source: ActionSource::Udl, g_k_line: 12.0e3, q_k_line: 0.0, point_force: 0.0, m_k: 0.0, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 0.0 },
                        LoadCaseActions { id: "Q-office".into(), kind: ActionKind::Imposed, category: PsiCategory::Office, source: ActionSource::Udl, g_k_line: 0.0, q_k_line: 8.0e3, point_force: 25.0e3, m_k: 0.0, n_k: 0.0, v_k: 0.0, t_k: 8.0e3, v_k_punch: 0.0 },
                        LoadCaseActions { id: "W".into(), kind: ActionKind::Wind, category: PsiCategory::Wind, source: ActionSource::Udl, g_k_line: 0.0, q_k_line: 2.0e3, point_force: 0.0, m_k: 0.0, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 0.0 },
                        LoadCaseActions { id: "A-impact".into(), kind: ActionKind::Accidental, category: PsiCategory::Accidental, source: ActionSource::External, g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 40.0e3, n_k: 0.0, v_k: 20.0e3, t_k: 0.0, v_k_punch: 0.0 },
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
                    relative_humidity: 0.5,
                    bearing_width: 0.0,
                    section_points: vec![],
                    bars: vec![],
                    tendon_points: vec![],
                    stud_rows: vec![],
                    openings: vec![],
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
                    longitudinal: vec![BarLayer { id: "vert".into(), diameter: 0.020, count: 8, position: BarPosition::Bottom, anchorage_length: 0.70, lap_length: 0.85, bond_condition: BondCondition::Good, aggregate_size: 0.016, lap_share: 1.0 }],
                    stirrups: Some(Stirrups { diameter: 0.008, spacing: 0.200, legs: 2 }),
                    punching: None,
                    prestress: None,
                    fire: Some(FireSpec { rating: FireRating::R60, axis_distance: 0.035, slab_system: SlabFireSystem::OneWay }),
                    actions: vec![
                        LoadCaseActions { id: "G".into(), kind: ActionKind::Permanent, category: PsiCategory::SelfWeight, source: ActionSource::External, g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 25.0e3, n_k: -500.0e3, v_k: 20.0e3, t_k: 0.0, v_k_punch: 0.0 },
                        LoadCaseActions { id: "Q-office".into(), kind: ActionKind::Imposed, category: PsiCategory::Office, source: ActionSource::External, g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 15.0e3, n_k: -200.0e3, v_k: 15.0e3, t_k: 0.0, v_k_punch: 0.0 },
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
                    relative_humidity: 0.5,
                    bearing_width: 0.0,
                    section_points: vec![],
                    bars: vec![],
                    tendon_points: vec![],
                    stud_rows: vec![],
                    openings: vec![],
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
                    longitudinal: vec![BarLayer { id: "bot".into(), diameter: 0.012, count: 8, position: BarPosition::Bottom, anchorage_length: 0.55, lap_length: 0.70, bond_condition: BondCondition::Good, aggregate_size: 0.016, lap_share: 1.0 }],
                    stirrups: None,
                    punching: Some(PunchingSpec { column_width: 0.30, column_depth: 0.30, column_position: ColumnPosition::Interior, asw: 0.0, beta_method: PunchingBetaMethod::Simplified, moment_c1: 0.0, moment_c2: 0.0 }),
                    prestress: None,
                    fire: Some(FireSpec { rating: FireRating::R60, axis_distance: 0.025, slab_system: SlabFireSystem::OneWay }),
                    actions: vec![
                        LoadCaseActions { id: "G".into(), kind: ActionKind::Permanent, category: PsiCategory::SelfWeight, source: ActionSource::External, g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 25.0e3, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 80.0e3 },
                        LoadCaseActions { id: "Q-office".into(), kind: ActionKind::Imposed, category: PsiCategory::Office, source: ActionSource::External, g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 15.0e3, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 45.0e3 },
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
                    relative_humidity: 0.5,
                    bearing_width: 0.0,
                    section_points: vec![],
                    bars: vec![],
                    tendon_points: vec![],
                    stud_rows: vec![],
                    openings: vec![],
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
                    LoadCaseActions { id: "G".into(), kind: ActionKind::Permanent, category: PsiCategory::SelfWeight, source: ActionSource::Udl, g_k_line: 200.0, q_k_line: 100.0, point_force: 0.0, m_k: 50.0, n_k: 6.0e3, v_k: 3.0e3, t_k: 20.0, v_k_punch: 10.0 },
                    LoadCaseActions { id: "Q-office".into(), kind: ActionKind::Imposed, category: PsiCategory::Office, source: ActionSource::Point, g_k_line: 0.0, q_k_line: 0.0, point_force: 50.0, m_k: 30.0, n_k: 4.0e3, v_k: 2.0e3, t_k: 15.0, v_k_punch: 5.0 },
                ],
                head_diameter: 0.0,
            }],
            placements: vec![],
            cases: vec![CaseRow {
                id: "case-m04".into(),
                label_en: "Strength development of the frame concrete".into(),
                label_de: "Festigkeitsentwicklung des Rahmenbetons".into(),
                entry_id: "M-04".into(),
                numbers: vec![CaseNumber { id: "fck".into(), value: 30.0 }, CaseNumber { id: "t".into(), value: 28.0 }],
                texts: vec![CaseText { id: "cement".into(), value: "N".into() }],
            }],
        }
    }

    /// ❌️ Non-compliant subject with multiple ULS/SLS/fire failures.
    pub fn failing_under_reinforced() -> Self {
        let mut snap = Self::compliant_office_frame();
        snap.title = "Failing under-reinforced frame".into();
        if let Some(beam) = snap.members.iter_mut().find(|m| m.id == "beam-B1") {
            beam.longitudinal = vec![BarLayer { id: "bot".into(), diameter: 0.012, count: 2, position: BarPosition::Bottom, anchorage_length: 0.20, lap_length: 0.25, bond_condition: BondCondition::Good, aggregate_size: 0.016, lap_share: 1.0 }];
            beam.stirrups = Some(Stirrups { diameter: 0.006, spacing: 0.300, legs: 2 });
            beam.cover = 0.015;
            beam.fire = Some(FireSpec { rating: FireRating::R90, axis_distance: 0.015, slab_system: SlabFireSystem::OneWay });
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
                pre.force = 3500.0e3;
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
            anc.head_diameter = 0.030;
        }
        snap
    }

    /// 🗜️ Failing prestressed member — transfer stress / losses exceed §5.10 limits.

    /// 🧵 Prestressed beam PS1 subject (used by compliant/failing prestressed examples).
    pub fn prestressed_beam_member() -> RcMember {
        use crate::*;
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
            longitudinal: vec![BarLayer { id: "bot".into(), diameter: 0.020, count: 8, position: BarPosition::Bottom, anchorage_length: 1.20, lap_length: 1.40, bond_condition: BondCondition::Good, aggregate_size: 0.016, lap_share: 1.0 }],
            stirrups: Some(Stirrups { diameter: 0.010, spacing: 0.120, legs: 2 }),
            punching: None,
            prestress: Some(PrestressSpec { force: 1200.0e3, area: 1500.0e-6, eccentricity: 0.150, loss_ratio: 0.12 }),
            fire: Some(FireSpec { rating: FireRating::R60, axis_distance: 0.050, slab_system: SlabFireSystem::OneWay }),
            actions: vec![
                LoadCaseActions { id: "G".into(), kind: ActionKind::Permanent, category: PsiCategory::SelfWeight, source: ActionSource::Udl, g_k_line: 5000.0, q_k_line: 0.0, point_force: 0.0, m_k: 0.0, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 0.0 },
                LoadCaseActions { id: "Q-office".into(), kind: ActionKind::Imposed, category: PsiCategory::Office, source: ActionSource::Udl, g_k_line: 0.0, q_k_line: 2500.0, point_force: 0.0, m_k: 0.0, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 0.0 },
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
            relative_humidity: 0.5,
            bearing_width: 0.0,
            section_points: vec![],
            bars: vec![],
            tendon_points: vec![],
            stud_rows: vec![],
            openings: vec![],
        }
    }

    pub fn failing_prestressed_beam() -> Self {
        let mut snap = Self::compliant_office_frame();
        snap.title = "Failing prestressed beam".into();
        snap.members = vec![Self::prestressed_beam_member()];
        if let Some(ps) = snap.members.first_mut() {
            if let Some(pre) = &mut ps.prestress {
                pre.force = 4000.0e3;
                pre.eccentricity = 0.300;
                pre.loss_ratio = 0.45;
            }
            ps.cover = 0.015;
            ps.longitudinal[0].count = 2;
        }
        snap.anchors.clear();
        snap
    }

    /// ✅ Compliant prestressed beam example (selectable).
    pub fn compliant_prestressed_beam() -> Self {
        let mut snap = Self::compliant_office_frame();
        snap.title = "Compliant prestressed beam".into();
        snap.members = vec![Self::prestressed_beam_member()];
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
            longitudinal: vec![BarLayer { id: "vert".into(), diameter: 0.016, count: 6, position: BarPosition::Bottom, anchorage_length: 0.70, lap_length: 0.85, bond_condition: BondCondition::Good, aggregate_size: 0.016, lap_share: 1.0 }],
            stirrups: Some(Stirrups { diameter: 0.008, spacing: 0.150, legs: 2 }),
            punching: None,
            prestress: None,
            fire: Some(FireSpec { rating: FireRating::R90, axis_distance: 0.040, slab_system: SlabFireSystem::OneWay }),
            actions: vec![
                LoadCaseActions { id: "G".into(), kind: ActionKind::Permanent, category: PsiCategory::SelfWeight, source: ActionSource::External, g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 100.0e3, n_k: 30.0e3, v_k: 50.0e3, t_k: 0.0, v_k_punch: 0.0 },
                LoadCaseActions { id: "Q-office".into(), kind: ActionKind::Imposed, category: PsiCategory::Office, source: ActionSource::External, g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 50.0e3, n_k: 15.0e3, v_k: 30.0e3, t_k: 0.0, v_k_punch: 0.0 },
            ],
            use_fem: true,
            udl: 24000.0,
            deflection_sensitive: false,
            tightness: Some(TightnessClass::Tc2),
            hd_over_h: 8.0,
            liquid_sigma_s: 280.0e6,
            liquid_rho_p_eff: 0.020,
            liquid_f_ct_eff: 3.2e6,
            liquid_s_r_max: 0.22,
            bridge_sigma_c: 0.0,
            bridge_delta_sigma_s: 0.0,
            relative_humidity: 0.5,
            bearing_width: 0.0,
            section_points: vec![],
            bars: vec![],
            tendon_points: vec![],
            stud_rows: vec![],
            openings: vec![],
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
                LoadCaseActions { id: "G".into(), kind: ActionKind::Permanent, category: PsiCategory::SelfWeight, source: ActionSource::Udl, g_k_line: 250.0, q_k_line: 120.0, point_force: 0.0, m_k: 40.0, n_k: 8.0e3, v_k: 4.0e3, t_k: 25.0, v_k_punch: 8.0 },
                LoadCaseActions { id: "Q-office".into(), kind: ActionKind::Imposed, category: PsiCategory::Office, source: ActionSource::Point, g_k_line: 0.0, q_k_line: 0.0, point_force: 80.0, m_k: 25.0, n_k: 5.0e3, v_k: 3.0e3, t_k: 18.0, v_k_punch: 6.0 },
            ],
            head_diameter: 0.0,
        }];
        snap
    }

    /// 🔘 Flat slab with punching reinforcement: a 300 mm slab of C30/37 on 6 m column spacing with an interior, an edge and a corner column of 300 x 300 mm, each with four persisted rings of studs Ø12 (12, 8 and 5 studs per ring); the characteristic column loads follow the tributary areas. The interior and edge layouts keep the layout limits (first ring 0.3 d to 0.5 d, ring spacing 0.75 d, tangential spacing 1.5 d within 2 d and 2 d beyond), the outermost ring of the corner column sits too far out (layout fails); the loads are plausible demo inputs, no normative value.
    pub fn flat_slab_punching() -> Self {
        let base = Self::compliant_office_frame();
        let slab = base.members.iter().find(|m| m.kind == MemberKind::FlatSlab).cloned().expect("the office frame has a flat slab");
        let column = |id: &str, en: &str, de: &str, position: ColumnPosition, g_punch: f64, q_punch: f64, rows: (u32, f64, f64, u32, f64, f64, f64)| RcMember {
            id: id.into(),
            label_en: en.into(),
            label_de: de.into(),
            height: 0.30,
            effective_depth: 0.255,
            span: 6.0,
            longitudinal: vec![BarLayer { id: "bot".into(), diameter: 0.014, count: 8, position: BarPosition::Bottom, anchorage_length: 0.65, lap_length: 0.85, bond_condition: BondCondition::Good, aggregate_size: 0.016, lap_share: 1.0 }],
            punching: Some(PunchingSpec { column_width: 0.30, column_depth: 0.30, column_position: position, asw: 0.0, beta_method: PunchingBetaMethod::Simplified, moment_c1: 0.0, moment_c2: 0.0 }),
            fire: Some(FireSpec { rating: FireRating::R60, axis_distance: 0.035, slab_system: SlabFireSystem::OneWay }),
            actions: vec![
                LoadCaseActions { id: "G".into(), kind: ActionKind::Permanent, category: PsiCategory::SelfWeight, source: ActionSource::External, g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 30.0e3, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: g_punch },
                LoadCaseActions { id: "Q-office".into(), kind: ActionKind::Imposed, category: PsiCategory::Office, source: ActionSource::External, g_k_line: 0.0, q_k_line: 0.0, point_force: 0.0, m_k: 15.0e3, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: q_punch },
            ],
            deflection_sensitive: false,
            stud_rows: (0..rows.0).map(|k| StudRow { id: format!("r{}", k + 1), angle: (rows.1 + (rows.2 - rows.1) * f64::from(k) / f64::from((rows.0 - 1).max(1))).to_radians(), count: rows.3, spacing_first: rows.4, spacing_radial: if k + 1 == rows.0 { rows.6 } else { rows.5 }, diameter: 0.012 }).collect(),
            openings: vec![],
            ..slab.clone()
        };
        Self {
            title: "Flat slab with punching reinforcement".into(),
            concrete_grades: vec![ConcreteGrade::from_f_ck("c30", "C30/37", 30.0e6)],
            reinforcement_grades: vec![ReinforcementGrade::b500b("b500")],
            prestress_steels: vec![],
            members: vec![
                column("slab-FS-interior", "Flat slab interior column", "Flachdecke Innenstütze", ColumnPosition::Interior, 300.0e3, 110.0e3, (12, 0.0, 330.0, 4, 0.10, 0.14, 0.14)),
                column("slab-FS-edge", "Flat slab edge column", "Flachdecke Randstütze", ColumnPosition::Edge, 160.0e3, 60.0e3, (8, 90.0, 270.0, 4, 0.10, 0.14, 0.14)),
                column("slab-FS-corner", "Flat slab corner column", "Flachdecke Eckstütze", ColumnPosition::Corner, 85.0e3, 30.0e3, (5, 180.0, 270.0, 4, 0.10, 0.14, 0.24)),
            ],
            anchors: vec![],
            placements: vec![],
            cases: vec![],
            ..base
        }
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

impl StudRow {
    /// 📍 Position of the k-th stud in the column frame (origin at the column centre, `x` along `c1`, `y` along `c2`; m): the ray at the row angle leaves the `c1 × c2` column face at the distance `face`, the stud sits `s_0 + k s_r` beyond it.
    pub fn position(&self, k: u32, c1: f64, c2: f64) -> (f64, f64) {
        let (cos, sin) = (self.angle.cos(), self.angle.sin());
        let face = (if cos.abs() > 1e-12 { c1 / 2.0 / cos.abs() } else { f64::MAX }).min(if sin.abs() > 1e-12 { c2 / 2.0 / sin.abs() } else { f64::MAX });
        let reach = face + self.spacing_first + f64::from(k) * self.spacing_radial;
        (reach * cos, reach * sin)
    }
}

//#endregion 🔖️Snapshot

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1992_model_typescript::LEAF, graphql: semio_s_artifact_norm_en1992_model_graphql::LEAF, json_schema: semio_s_artifact_norm_en1992_model_json_schema::LEAF, proto: semio_s_artifact_norm_en1992_model_proto::LEAF };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;
use semio_s_plugin_norm_compliance as document;

/// 🏗️ Fire resistance rating (EN 1992-1-2 tabulated data).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum FireRating {
    R30,
    R60,
    R90,
    R120,
}

/// 💧️ Tightness class per EN 1992-3 Table 7.105.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum TightnessClass {
    Tc0,
    Tc1,
    Tc2,
}

/// 🧱 Member typology for EN 1992 checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum MemberKind {
    Beam,
    Slab,
    Column,
    Wall,
    FlatSlab,
    RibbedSlab,
    TensionMember,
    Bridge,
    LiquidRetaining,
}

/// 📍 Support condition for span/deflection rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum SupportCondition {
    SimplySupported,
    Continuous,
    Cantilever,
    Fixed,
}

/// 🌦 Exposure class (EN 1992-1-1 Table 4.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum ExposureClass {
    X0,
    Xc1,
    Xc2,
    Xc3,
    Xc4,
    Xd1,
    Xd2,
    Xd3,
    Xs1,
    Xs2,
    Xs3,
    Xf1,
    Xf2,
    Xf3,
    Xf4,
    Xa1,
    Xa2,
    Xa3,
}

/// 🪨 Concrete grade — editable f_ck; Table 3.1 derived properties via methods.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ConcreteGrade {
    pub id: String,
    pub name: String,
    pub f_ck: f64,
}

impl ConcreteGrade {
    /// 📘 Construct from cylinder strength f_ck [Pa].
    pub fn from_f_ck(id: impl Into<String>, name: impl Into<String>, f_ck: f64) -> Self {
        Self { id: id.into(), name: name.into(), f_ck }
    }
    pub fn f_ck_cube(&self) -> f64 {
        let fck_mpa = self.f_ck / 1.0e6;
        match fck_mpa.round() as i32 {
            12 => 15.0e6, 16 => 20.0e6, 20 => 25.0e6, 25 => 30.0e6, 30 => 37.0e6,
            35 => 45.0e6, 40 => 50.0e6, 45 => 55.0e6, 50 => 60.0e6, 55 => 67.0e6,
            60 => 75.0e6, 70 => 85.0e6, 80 => 95.0e6, 90 => 105.0e6, 100 => 115.0e6,
            _ => self.f_ck * 1.25,
        }
    }
    pub fn f_cm(&self) -> f64 { self.f_ck + 8.0e6 }
    pub fn f_ctm(&self) -> f64 {
        let fck_mpa = self.f_ck / 1.0e6;
        if fck_mpa <= 50.0 { 0.30 * fck_mpa.powf(2.0 / 3.0) * 1.0e6 }
        else { 2.12 * (1.0 + (self.f_cm() / 1.0e6) / 10.0).ln() * 1.0e6 }
    }
    pub fn f_ctk_005(&self) -> f64 { 0.7 * self.f_ctm() }
    pub fn e_cm(&self) -> f64 { 22.0e3 * ((self.f_cm() / 1.0e6) / 10.0).powf(0.3) * 1.0e6 }
    pub fn eps_c2(&self) -> f64 {
        let fck = self.f_ck / 1.0e6;
        if fck <= 50.0 { 2.0e-3 } else { (2.0 + 0.085 * (fck - 50.0).powf(0.53)) * 1.0e-3 }
    }
    pub fn eps_cu2(&self) -> f64 {
        let fck = self.f_ck / 1.0e6;
        if fck <= 50.0 { 3.5e-3 } else { (2.6 + 35.0 * ((90.0 - fck) / 100.0).powf(4.0)) * 1.0e-3 }
    }
    pub fn eps_c3(&self) -> f64 {
        let fck = self.f_ck / 1.0e6;
        if fck <= 50.0 { 1.75e-3 } else { (1.75 + 0.55 * ((fck - 50.0) / 40.0)) * 1.0e-3 }
    }
    pub fn eps_cu3(&self) -> f64 { self.eps_cu2() }
    pub fn n_parabola(&self) -> f64 {
        let fck = self.f_ck / 1.0e6;
        if fck <= 50.0 { 2.0 } else { 1.4 + 23.4 * ((90.0 - fck) / 100.0).powf(4.0) }
    }
}

/// 🔩 Ductility class A/B/C (EN 1992-1-1 Annex C).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum DuctilityClass {
    A,
    B,
    C,
}

impl DuctilityClass {
    /// 📏 Annex C Table C.1 minima (k, ε_uk).
    pub fn table_c1_minima(self) -> (f64, f64) {
        match self {
            Self::A => (1.05, 0.025),
            Self::B => (1.08, 0.050),
            Self::C => (1.15, 0.075),
        }
    }
}

/// 🔗 Bond condition of a bar layer (EN 1992-1-1 8.4.2, Figure 8.2): good or poor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum BondCondition {
    Good,
    Poor,
}

/// 🔩 Reinforcing steel — editable f_yk, ductility class, k, ε_uk (checked vs Table C.1).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ReinforcementGrade {
    pub id: String,
    pub name: String,
    pub f_yk: f64,
    pub e_s: f64,
    pub ductility: DuctilityClass,
    pub k: f64,
    pub eps_uk: f64,
}

impl ReinforcementGrade {
    pub fn b500a(id: impl Into<String>) -> Self {
        Self { id: id.into(), name: "B500A".into(), f_yk: 500.0e6, e_s: 200.0e9, ductility: DuctilityClass::A, k: 1.05, eps_uk: 0.025 }
    }
    pub fn b500b(id: impl Into<String>) -> Self {
        Self { id: id.into(), name: "B500B".into(), f_yk: 500.0e6, e_s: 200.0e9, ductility: DuctilityClass::B, k: 1.08, eps_uk: 0.050 }
    }
    pub fn b500c(id: impl Into<String>) -> Self {
        Self { id: id.into(), name: "B500C".into(), f_yk: 500.0e6, e_s: 200.0e9, ductility: DuctilityClass::C, k: 1.15, eps_uk: 0.075 }
    }
}

/// 🧵 Prestressing steel (f_pk, f_p0_1k in Pa).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PrestressSteel {
    pub id: String,
    pub name: String,
    pub f_pk: f64,
    pub f_p0_1k: f64,
}

/// 📍 Position of a bar layer in the section (EN 1992-1-1 8.4.2, Figure 8.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum BarPosition {
    Bottom,
    Top,
    Tension,
    Compression,
}

/// ➖ Longitudinal bar layer (diameter / lengths in m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BarLayer {
    pub id: String,
    pub diameter: f64,
    pub count: u32,
    pub position: BarPosition,
    pub anchorage_length: f64,
    pub lap_length: f64,
    pub bond_condition: BondCondition,
    pub aggregate_size: f64,
    pub lap_share: f64,
}

/// ➰ Shear reinforcement (spacing/diameter in m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Stirrups {
    pub diameter: f64,
    pub spacing: f64,
    pub legs: u32,
}

/// 🕳️ Position of the column below a flat slab for the punching factor beta (EN 1992-1-1 6.4.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum ColumnPosition {
    Interior,
    Edge,
    Corner,
    WallEnd,
}

/// 🎯 Method of the punching load increase factor β (EN 1992-1-1 6.4.3): the fixed values of 6.4.3(6) or β from the unbalanced column moments of 6.4.3(3) to (5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum PunchingBetaMethod {
    Simplified,
    UnbalancedMoments,
}

/// 🕳️ Punching geometry — u₀/u₁/β derived from column size + position (lengths m, asw m²/m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PunchingSpec {
    pub column_width: f64,
    pub column_depth: f64,
    pub column_position: ColumnPosition,
    pub asw: f64,
    pub beta_method: PunchingBetaMethod,
    pub moment_c1: f64,
    pub moment_c2: f64,
}

/// 🗜️ Prestress — force N, area m², eccentricity m, long-term loss ratio → P_m,∞.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PrestressSpec {
    pub force: f64,
    pub area: f64,
    pub eccentricity: f64,
    pub loss_ratio: f64,
}

/// 🔥 Slab system of the EN 1992-1-2 tabulated fire data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum SlabFireSystem {
    OneWay,
    TwoWay,
}

/// 🔥 Fire exposure (axis distance in m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FireSpec {
    pub rating: FireRating,
    pub axis_distance: f64,
    pub slab_system: SlabFireSystem,
}

/// 🏋️ Kind of a characteristic action (EN 1990 4.1.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum ActionKind {
    Permanent,
    Imposed,
    Snow,
    Wind,
    Accidental,
    Prestress,
}

/// 🧮 Source of the characteristic internal forces of an action.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum ActionSource {
    Udl,
    Point,
    External,
}

/// 🏷️ Category of an action for the combination factors psi (EN 1990 Table A1.1); the variants are the rows of the psi table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum PsiCategory {
    SelfWeight,
    Office,
    Residential,
    Congregation,
    Shopping,
    Storage,
    Snow,
    SnowHigh,
    Wind,
    Temperature,
    Accidental,
}

/// 📊 Characteristic load case (EN 1990) — line loads N/m or external characteristic effects N / N·m.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LoadCaseActions {
    pub id: String,
    pub kind: ActionKind,
    pub category: PsiCategory,
    pub source: ActionSource,
    pub g_k_line: f64,
    pub q_k_line: f64,
    pub point_force: f64,
    pub m_k: f64,
    pub n_k: f64,
    pub v_k: f64,
    pub t_k: f64,
    pub v_k_punch: f64,
}

/// 📍 SectionPoint of an explicit geometry collection of a member (lengths in m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SectionPoint {
    pub id: String,
    pub y: f64,
    pub z: f64,
}

/// 🪝 Bend of an explicit bar: derived hooks, straight, one hook or hooks at both ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
pub enum BarBend {
    Auto,
    Straight,
    Hook,
    UHook,
}

/// 📍 BarPoint of an explicit geometry collection of a member (lengths in m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BarPoint {
    pub id: String,
    pub layer_id: String,
    pub y: f64,
    pub z: f64,
    pub diameter: f64,
    pub length: f64,
    pub bend: BarBend,
}

/// 📍 TendonPoint of an explicit geometry collection of a member (lengths in m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TendonPoint {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 📍 Shear stud row (rail) of the punching zone of a member: studs at s0 + k sr from the column face along the ray at the angle (lengths in m, angle in rad).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct StudRow {
    pub id: String,
    pub angle: f64,
    pub count: u32,
    pub spacing_first: f64,
    pub spacing_radial: f64,
    pub diameter: f64,
}

/// 📍 Opening near the column of a member (centre and size in the column frame, m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Opening {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub depth: f64,
}

/// 🏛️ Reinforced/prestressed concrete member.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RcMember {
    pub id: String,
    pub label_en: String,
    pub label_de: String,
    pub kind: MemberKind,
    pub concrete_grade_id: String,
    pub reinforcement_grade_id: String,
    pub prestress_steel_id: String,
    pub exposure: ExposureClass,
    pub width: f64,
    pub height: f64,
    pub effective_depth: f64,
    pub cover: f64,
    pub span: f64,
    pub support: SupportCondition,
    pub buckling_length: f64,
    #[dsl(table)]
    pub longitudinal: Vec<BarLayer>,
    pub stirrups: Option<Stirrups>,
    pub punching: Option<PunchingSpec>,
    pub prestress: Option<PrestressSpec>,
    pub fire: Option<FireSpec>,
    #[dsl(table)]
    pub actions: Vec<LoadCaseActions>,
    pub use_fem: bool,
    pub udl: f64,
    pub deflection_sensitive: bool,
    pub tightness: Option<TightnessClass>,
    pub hd_over_h: f64,
    pub liquid_sigma_s: f64,
    pub liquid_rho_p_eff: f64,
    pub liquid_f_ct_eff: f64,
    pub liquid_s_r_max: f64,
    pub bridge_sigma_c: f64,
    pub bridge_delta_sigma_s: f64,
    pub relative_humidity: f64,
    /// 🧱 Width a of the support and load bearing plates of a deep beam, the strut width at its nodes (EN 1992-1-1 6.5.2, 6.5.4); 0 = not given, no value is assumed and the strut and node checks are not applicable.
    #[dsl(unit = "m")]
    pub bearing_width: f64,
    #[dsl(table)]
    pub section_points: Vec<SectionPoint>,
    #[dsl(table)]
    pub bars: Vec<BarPoint>,
    #[dsl(table)]
    pub tendon_points: Vec<TendonPoint>,
    #[dsl(table)]
    pub stud_rows: Vec<StudRow>,
    #[dsl(table)]
    pub openings: Vec<Opening>,
}

/// 〰️ Parabolic interpolation through control points sorted by `x` (EN 1992-1-1 5.10 tendon profiles are parabolic between deviation points): two points are joined by a line, from three on each interval blends the parabola through the three points ending at its right end with the one starting at its left end by the position in the interval (Overhauser), so the curve passes through every point, equals the exact parabola when the points lie on one, and is continuous in value and slope; `x` outside the range is clamped.
pub fn parabolic_through(points: &[(f64, f64)], x: f64) -> f64 {
    let n = points.len();
    if n < 3 {
        return match n {
            0 => 0.0,
            1 => points[0].1,
            _ => {
                let t = if (points[1].0 - points[0].0).abs() < f64::EPSILON { 0.0 } else { ((x - points[0].0) / (points[1].0 - points[0].0)).clamp(0.0, 1.0) };
                points[0].1 + t * (points[1].1 - points[0].1)
            }
        };
    }
    let x = x.clamp(points[0].0, points[n - 1].0);
    let i = points.iter().position(|p| p.0 >= x).unwrap_or(n - 1).max(1);
    let through = |a: usize| -> f64 {
        let (p, q, r) = (points[a], points[a + 1], points[a + 2]);
        let (dpq, dpr, dqr) = (p.0 - q.0, p.0 - r.0, q.0 - r.0);
        if dpq.abs() < f64::EPSILON || dpr.abs() < f64::EPSILON || dqr.abs() < f64::EPSILON {
            return if (x - points[i - 1].0).abs() <= (points[i].0 - x).abs() { points[i - 1].1 } else { points[i].1 };
        }
        p.1 * (x - q.0) * (x - r.0) / (dpq * dpr) - q.1 * (x - p.0) * (x - r.0) / (dpq * dqr) + r.1 * (x - p.0) * (x - q.0) / (dpr * dqr)
    };
    let (a, b) = (points[i - 1], points[i]);
    let t = if (b.0 - a.0).abs() < f64::EPSILON { 0.0 } else { ((x - a.0) / (b.0 - a.0)).clamp(0.0, 1.0) };
    match (i >= 2, i + 1 < n) {
        (true, true) => (1.0 - t) * through(i - 2) + t * through(i - 1),
        (true, false) => through(i - 2),
        _ => through(i - 1),
    }
}

impl RcMember {
    /// ➕ Tension steel area (m²): layers tagged bottom or tension; a layer that has explicit bars contributes the sum of their own areas instead of count × area.
    pub fn a_s_tension(&self) -> f64 {
        let area = |d: f64| std::f64::consts::PI * (d * 0.5).powi(2);
        self.longitudinal
            .iter()
            .filter(|l| l.position == BarPosition::Bottom || l.position == BarPosition::Tension)
            .map(|l| {
                let own: Vec<f64> = self.bars.iter().filter(|b| b.layer_id == l.id).map(|b| area(b.diameter)).collect();
                if own.is_empty() { l.count as f64 * area(l.diameter) } else { own.iter().sum() }
            })
            .sum()
    }

    /// 🎯 Centroid (y, z) in m of the explicit outline in the frame of its section points, (0, 0) for the rectangle b × h; outline and bars are given in that frame and move to the centroid together; tendon points are measured from the centroid already.
    pub fn outline_centroid(&self) -> (f64, f64) {
        if self.section_points.len() < 3 { return (0.0, 0.0); }
        let p = &self.section_points;
        let (mut a, mut sy, mut sz) = (0.0, 0.0, 0.0);
        for i in 0..p.len() {
            let (q, r) = (&p[i], &p[(i + 1) % p.len()]);
            let c = q.y * r.z - r.y * q.z;
            a += c;
            sy += (q.y + r.y) * c;
            sz += (q.z + r.z) * c;
        }
        if a.abs() < 1e-18 { (0.0, 0.0) } else { (sy / (3.0 * a), sz / (3.0 * a)) }
    }

    /// 🔩 Position (y, z) in m of an explicit bar relative to the centroid of the outline.
    pub fn bar_position(&self, bar: &BarPoint) -> (f64, f64) {
        let (cy, cz) = self.outline_centroid();
        (bar.y - cy, bar.z - cz)
    }

    /// 📐 Section polygon in m (y across, z up from the centroid): the explicit outline moved to its centroid, else the rectangle b × h.
    pub fn outline(&self) -> Vec<(f64, f64)> {
        if self.section_points.len() >= 3 {
            let (cy, cz) = self.outline_centroid();
            self.section_points.iter().map(|p| (p.y - cy, p.z - cz)).collect()
        } else {
            let (hb, hh) = (self.width * 0.5, self.height * 0.5);
            vec![(-hb, -hh), (hb, -hh), (hb, hh), (-hb, hh)]
        }
    }

    /// 🧮 Gross area of the outline (m², shoelace).
    pub fn gross_area(&self) -> f64 {
        let o = self.outline();
        (0..o.len()).map(|i| { let (a, b) = (o[i], o[(i + 1) % o.len()]); a.0 * b.1 - b.0 * a.1 }).sum::<f64>().abs() * 0.5
    }

    /// 📏 Effective depth d (m) derived from explicit tension bars: distance from the top fibre to the area-weighted tension bar centroid; `None` without explicit bars.
    pub fn depth_from_bars(&self) -> Option<f64> {
        let tension: Vec<&BarPoint> = self.bars.iter().filter(|b| self.longitudinal.iter().any(|l| l.id == b.layer_id && (l.position == BarPosition::Bottom || l.position == BarPosition::Tension))).collect();
        let area: f64 = tension.iter().map(|b| b.diameter * b.diameter).sum();
        if area <= 0.0 { return None; }
        let zc = tension.iter().map(|b| self.bar_position(b).1 * b.diameter * b.diameter).sum::<f64>() / area;
        let top = self.outline().iter().map(|p| p.1).fold(f64::MIN, f64::max);
        Some(top - zc)
    }

    /// 🧵 Tendon height `z` (m, up from the centroid) at `x` along the span, parabolic through the control points (see [`parabolic_through`]); `None` without control points.
    pub fn tendon_z_at(&self, x: f64) -> Option<f64> {
        let mut pts: Vec<(f64, f64)> = self.tendon_points.iter().map(|p| (p.x, p.z)).collect();
        pts.sort_by(|a, b| a.0.total_cmp(&b.0));
        (!pts.is_empty()).then(|| parabolic_through(&pts, x))
    }

    /// 🧵 Tendon eccentricity at midspan (m, positive below the centroid) on the parabolic profile through the control points; `None` without control points.
    pub fn tendon_eccentricity_mid(&self) -> Option<f64> {
        self.tendon_z_at(self.span * 0.5).map(|z| -z)
    }

    /// 🧵 The tendon profile sampled at `per_interval` points between each pair of control points, ends included; empty without control points.
    pub fn tendon_profile(&self, per_interval: usize) -> Vec<(f64, f64, f64)> {
        let mut pts: Vec<&TendonPoint> = self.tendon_points.iter().collect();
        pts.sort_by(|a, b| a.x.total_cmp(&b.x));
        let steps = per_interval.max(1);
        pts.windows(2)
            .flat_map(|w| (0..steps).map(move |k| {
                let t = k as f64 / steps as f64;
                (w[0].x + t * (w[1].x - w[0].x), w[0].y + t * (w[1].y - w[0].y))
            }))
            .chain(pts.last().map(|p| (p.x, p.y)))
            .map(|(x, y)| (x, y, self.tendon_z_at(x).unwrap_or(0.0)))
            .collect()
    }

    /// 🔩 Shear stud area A_sw per perimeter (m²) of EN 1992-1-1 6.4.5: one stud of every row with studs crosses a perimeter, `None` without such a row.
    pub fn stud_asw(&self) -> Option<f64> {
        let area: f64 = self.stud_rows.iter().filter(|r| r.count > 0 && r.diameter > 0.0).map(|r| std::f64::consts::PI * (r.diameter * 0.5).powi(2)).sum();
        (area > 0.0).then_some(area)
    }

    /// 📏 Radial spacing s_r (m) of the stud perimeters of EN 1992-1-1 6.4.5 Eq. 6.52: the largest radial spacing of a row with at least two studs, `None` without such a row.
    pub fn stud_radial_spacing(&self) -> Option<f64> {
        self.stud_rows.iter().filter(|r| r.count > 1 && r.diameter > 0.0).map(|r| r.spacing_radial).reduce(f64::max).filter(|s| *s > 0.0)
    }

    /// 📐 Distance (m) of the outermost stud from the column face: `s_0 + (n - 1) s_r` of the longest row, `None` without studs.
    pub fn stud_reach(&self) -> Option<f64> {
        self.stud_rows.iter().filter(|r| r.count > 0 && r.diameter > 0.0).map(|r| r.spacing_first + f64::from(r.count - 1) * r.spacing_radial).reduce(f64::max)
    }

    /// ➗ Longitudinal reinforcement ratio ρ_l = A_s/(b·d).
    pub fn rho_l(&self) -> f64 {
        let bd = self.width * self.effective_depth;
        if bd <= 0.0 {
            0.0
        } else {
            self.a_s_tension() / bd
        }
    }

    /// ➰ A_sw / s from stirrups (m²/m).
    pub fn asw_per_s(&self) -> f64 {
        let Some(s) = &self.stirrups else {
            return 0.0;
        };
        if s.spacing <= 0.0 {
            return 0.0;
        }
        s.legs as f64 * std::f64::consts::PI * (s.diameter * 0.5).powi(2) / s.spacing
    }

    /// 🏷️ Localized subject label.
    pub fn subject_label(&self) -> document::LocalizedCopy {
        document::LocalizedCopy::new(self.label_en.clone(), self.label_de.clone())
    }
}

/// ⚓️ Fastening (EN 1992-4) — lengths m, stresses Pa, forces N.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Anchor {
    pub id: String,
    pub h_ef: f64,
    pub cracked: bool,
    pub f_uk: f64,
    pub f_yk: f64,
    pub a_s: f64,
    pub d: f64,
    pub c1: f64,
    pub f_ck: f64,
    #[dsl(table)]
    pub actions: Vec<LoadCaseActions>,
    pub head_diameter: f64,
}

/// 📍️ Persisted pose of one scene entity (`id` is the entity id; SI: m, rad). Entities without a placement follow the documented default layout.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
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

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1992", "EN 1992", EN1992_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1992_DIALECT: semio_plugin::app::Dialect = semio_plugin::app::Dialect { artifact_kind: "s.norm.en1992", standard: semio_plugin::app::StandardId("1"), subset: semio_plugin::app::SubsetId::ANY };

pub const EN1992_DOCUMENT_SCHEMA: &str = "semio.norm.en1992/v1";

#[path = "../../🚪️io/🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[path = "../../🚪️io/🪞️records/🦀️.rs"]
mod records;
