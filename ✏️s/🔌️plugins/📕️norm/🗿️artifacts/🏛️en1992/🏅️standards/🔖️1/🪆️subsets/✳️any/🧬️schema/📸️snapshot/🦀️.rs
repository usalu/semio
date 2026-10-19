//! 🧬️ En1992 snapshot — complete reinforced/prestressed concrete structure subject (SI).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
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
                    LoadCaseActions { id: "G".into(), kind: "permanent".into(), category: "self".into(), source: "udl".into(), g_k_line: 200.0, q_k_line: 100.0, point_force: 0.0, m_k: 50.0, n_k: 6.0e3, v_k: 3.0e3, t_k: 20.0, v_k_punch: 10.0 },
                    LoadCaseActions { id: "Q-office".into(), kind: "imposed".into(), category: "office".into(), source: "point".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 50.0, m_k: 30.0, n_k: 4.0e3, v_k: 2.0e3, t_k: 15.0, v_k_punch: 5.0 },
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
            longitudinal: vec![BarLayer { id: "bot".into(), diameter: 0.020, count: 8, position: "bottom".into(), anchorage_length: 1.20, lap_length: 1.40, bond_condition: "good".into(), aggregate_size: 0.016 }],
            stirrups: Some(Stirrups { diameter: 0.010, spacing: 0.120, legs: 2 }),
            punching: None,
            prestress: Some(PrestressSpec { force: 1200.0e3, area: 1500.0e-6, eccentricity: 0.150, loss_ratio: 0.12 }),
            fire: Some(FireSpec { rating: FireRating::R60, axis_distance: 0.050, column_method: "A".into(), slab_system: "one-way".into() }),
            actions: vec![
                LoadCaseActions { id: "G".into(), kind: "permanent".into(), category: "self".into(), source: "udl".into(), g_k_line: 5000.0, q_k_line: 0.0, point_force: 0.0, m_k: 0.0, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 0.0 },
                LoadCaseActions { id: "Q-office".into(), kind: "imposed".into(), category: "office".into(), source: "udl".into(), g_k_line: 0.0, q_k_line: 2500.0, point_force: 0.0, m_k: 0.0, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 0.0 },
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
            liquid_sigma_s: 280.0e6,
            liquid_rho_p_eff: 0.020,
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
                LoadCaseActions { id: "G".into(), kind: "permanent".into(), category: "self".into(), source: "udl".into(), g_k_line: 250.0, q_k_line: 120.0, point_force: 0.0, m_k: 40.0, n_k: 8.0e3, v_k: 4.0e3, t_k: 25.0, v_k_punch: 8.0 },
                LoadCaseActions { id: "Q-office".into(), kind: "imposed".into(), category: "office".into(), source: "point".into(), g_k_line: 0.0, q_k_line: 0.0, point_force: 80.0, m_k: 25.0, n_k: 5.0e3, v_k: 3.0e3, t_k: 18.0, v_k_punch: 6.0 },
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

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;
use semio_s_plugin_norm_compliance as document;

/// 🏗️ Fire resistance rating (EN 1992-1-2 tabulated data).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum FireRating {
    R30,
    R60,
    R90,
    R120,
}

/// 💧️ Tightness class per EN 1992-3 Table 7.105.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum TightnessClass {
    Tc0,
    Tc1,
    Tc2,
}

/// 🧱 Member typology for EN 1992 checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum SupportCondition {
    SimplySupported,
    Continuous,
    Cantilever,
    Fixed,
}

/// 🌦 Exposure class (EN 1992-1-1 Table 4.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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

/// 🔩 Reinforcing steel — editable f_yk, ductility class, k, ε_uk (checked vs Table C.1).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
        Self { id: id.into(), name: "B500C".into(), f_yk: 550.0e6, e_s: 200.0e9, ductility: DuctilityClass::C, k: 1.15, eps_uk: 0.075 }
    }
}

/// 🧵 Prestressing steel (f_pk, f_p0_1k in Pa).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PrestressSteel {
    pub id: String,
    pub name: String,
    pub f_pk: f64,
    pub f_p0_1k: f64,
}

/// ➖ Longitudinal bar layer (diameter / lengths in m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BarLayer {
    pub id: String,
    pub diameter: f64,
    pub count: u32,
    pub position: String,
    pub anchorage_length: f64,
    pub lap_length: f64,
    pub bond_condition: String,
    pub aggregate_size: f64,
}

/// ➰ Shear reinforcement (spacing/diameter in m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Stirrups {
    pub diameter: f64,
    pub spacing: f64,
    pub legs: u32,
}

/// 🕳️ Punching geometry — u₀/u₁/β derived from column size + position (lengths m, asw m²/m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PunchingSpec {
    pub column_width: f64,
    pub column_depth: f64,
    pub column_position: String,
    pub asw: f64,
}

/// 🗜️ Prestress — force N, area m², eccentricity m, long-term loss ratio → P_m,∞.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PrestressSpec {
    pub force: f64,
    pub area: f64,
    pub eccentricity: f64,
    pub loss_ratio: f64,
}

/// 🔥 Fire exposure (axis distance in m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FireSpec {
    pub rating: FireRating,
    pub axis_distance: f64,
    pub column_method: String,
    pub slab_system: String,
}

/// 📊 Characteristic load case (EN 1990) — line loads N/m or external characteristic effects N / N·m.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LoadCaseActions {
    pub id: String,
    pub kind: String,
    pub category: String,
    pub source: String,
    pub g_k_line: f64,
    pub q_k_line: f64,
    pub point_force: f64,
    pub m_k: f64,
    pub n_k: f64,
    pub v_k: f64,
    pub t_k: f64,
    pub v_k_punch: f64,
}

/// 🏛️ Reinforced/prestressed concrete member.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
}

impl RcMember {
    /// ➕ Tension steel area from bottom/top layers tagged bottom (m²).
    pub fn a_s_tension(&self) -> f64 {
        self.longitudinal
            .iter()
            .filter(|l| l.position == "bottom" || l.position == "tension")
            .map(|l| l.count as f64 * std::f64::consts::PI * (l.diameter * 0.5).powi(2))
            .sum()
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
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
}

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1992", "EN 1992", EN1992_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1992_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1992", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const EN1992_DOCUMENT_SCHEMA: &str = "semio.norm.en1992/v1";
