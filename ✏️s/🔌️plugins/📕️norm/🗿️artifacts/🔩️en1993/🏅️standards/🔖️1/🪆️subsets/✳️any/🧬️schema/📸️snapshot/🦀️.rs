//! 🧬️ En1993 snapshot schema — complete steel-structure subject (SI base units).

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
#[dsl(id = "norm.en1993", layout = "lines")]
#[artifact_schema(id = "s.norm.en1993")]
pub struct En1993Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[dsl(table)]
    #[state(artifact)]
    pub materials: Vec<SteelMaterial>,
    #[dsl(table)]
    #[state(artifact)]
    pub sections: Vec<SteelSection>,
    #[dsl(table)]
    #[state(artifact)]
    pub members: Vec<SteelMember>,
    #[dsl(table)]
    #[state(artifact)]
    pub load_cases: Vec<LoadCase>,
    #[dsl(table)]
    #[state(artifact)]
    pub member_actions: Vec<MemberAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub joints: Vec<SteelJoint>,
    #[dsl(table)]
    #[state(artifact)]
    pub fatigue_details: Vec<FatigueDetail>,
    #[dsl(table)]
    #[state(artifact)]
    pub fire_exposures: Vec<FireExposure>,
    #[dsl(table)]
    #[state(artifact)]
    pub cold_formed_members: Vec<ColdFormedMember>,
    #[dsl(table)]
    #[state(artifact)]
    pub plated_panels: Vec<PlatedPanel>,
    #[dsl(table)]
    #[state(artifact)]
    pub silo_shells: Vec<SiloShell>,
    #[dsl(table)]
    #[state(artifact)]
    pub tension_components: Vec<TensionComponent>,
    #[dsl(table)]
    #[state(artifact)]
    pub bridge_fatigue: Vec<BridgeFatigue>,
    #[dsl(table)]
    #[state(artifact)]
    pub tower_legs: Vec<TowerLeg>,
    #[dsl(table)]
    #[state(artifact)]
    pub piles: Vec<SteelPile>,
    #[dsl(table)]
    #[state(artifact)]
    pub crane_runways: Vec<CraneRunway>,
}

impl Default for En1993Snapshot {
    fn default() -> Self {
        Self::compliant_heb240_frame()
    }
}

impl En1993Snapshot {
    /// ✅️ Realistic multi-part compliant steel subject (DE-NA) exercising parts 1–6.
    pub fn compliant_heb240_frame() -> Self {
        Self {
            annex: AnnexChoice::De,
            materials: vec![SteelMaterial {
                id: "mat-s355".into(),
                grade: "S355".into(),
                fy: 355.0e6,
                fu: 510.0e6,
                e_modulus: 210.0e9,
                g_modulus: 81.0e9,
                subgrade: "J2".into(),
                kind: "carbon".into(),
            }],
            sections: vec![catalogue_heb240()],
            members: vec![SteelMember {
                id: "member-b1".into(),
                label: "Beam-column B1".into(),
                member_type: "beamColumn".into(),
                section_id: "sec-heb240".into(),
                material_id: "mat-s355".into(),
                length: 4.0,
                buckling_length_y: 4.0,
                buckling_length_z: 4.0,
                ltb_length: 4.0,
                ltb_restraint_spacing: 2.0,
                load_application: "shearCenter".into(),
                end_moment_ratio_psi: -1.0,
                moment_diagram: "linear".into(),
                deflection_limit_ratio: 300.0,
                analysis: "elastic".into(),
            }],
            load_cases: vec![
                LoadCase { id: "g-permanent".into(), name: "Permanent G".into(), kind: "permanent".into(), category: "self".into() },
                LoadCase { id: "q-imposed".into(), name: "Imposed Q".into(), kind: "imposed".into(), category: "office".into() },
                LoadCase { id: "w-wind".into(), name: "Wind".into(), kind: "wind".into(), category: "wind".into() },
            ],
            member_actions: vec![
                MemberAction {
                    id: "act-b1-g".into(),
                    member_id: "member-b1".into(),
                    load_case_id: "g-permanent".into(),
                    action: DesignAction { n: 100_000.0, vy: 0.0, vz: 20_000.0, my: 40_000.0, mz: 0.0, t: 0.0 },
                },
                MemberAction {
                    id: "act-b1-q".into(),
                    member_id: "member-b1".into(),
                    load_case_id: "q-imposed".into(),
                    action: DesignAction { n: 43_333.0, vy: 0.0, vz: 15_333.0, my: 17_333.0, mz: 0.0, t: 0.0 },
                },
            ],
            joints: vec![SteelJoint {
                id: "joint-j1".into(),
                kind: "bolted".into(),
                member_id: "member-b1".into(),
                bolt_class: "8.8".into(),
                bolt_diameter: 0.020,
                bolt_rows: 2,
                bolts_per_row: 2,
                pitch: 0.060,
                gauge: 0.060,
                end_distance: 0.040,
                edge_distance: 0.040,
                shear_planes: 1,
                plate_thickness: 0.010,
                plate_fu: 510.0e6,
                weld_throat: 0.0,
                weld_length: 0.0,
                weld_fu: 510.0e6,
                weld_grade: "S355".into(),
                actions: vec![
                    JointForceAction { id: "jf-g".into(), load_case_id: "g-permanent".into(), shear: 30_000.0, tension: 0.0 },
                    JointForceAction { id: "jf-q".into(), load_case_id: "q-imposed".into(), shear: 23_333.0, tension: 0.0 },
                ],
                category: "A".into(),
                friction_mu: 0.50,
                preload_force: 0.0,
                slip_factor_ks: 1.0,
                friction_surfaces: 1,
            }],
            fatigue_details: vec![FatigueDetail {
                id: "fat-1".into(),
                member_id: "member-b1".into(),
                category: 71,
                method: "damage_tolerant".into(),
                spectrum: vec![
                    FatigueBand { id: "fb-1".into(), delta_sigma: 50.0e6, cycles: 1.0e6 },
                    FatigueBand { id: "fb-2".into(), delta_sigma: 40.0e6, cycles: 1.0e6 },
                ],
            }],
            fire_exposures: vec![FireExposure {
                id: "fire-1".into(),
                member_id: "member-b1".into(),
                rating: "r60".into(),
                protection_thickness: 0.020,
                section_factor: 150.0,
                mu0: 0.50,
                design_temperature: 550.0,
                protection_conductivity: 0.20,
                protection_density: 800.0,
                protection_specific_heat: 1700.0,
            }],
            cold_formed_members: vec![ColdFormedMember {
                id: "cf-1".into(),
                b_bar: 0.080,
                thickness: 0.003,
                k_sigma: 4.0,
                psi: 1.0,
                fy: 350.0e6,
                gross_resistance: 80_000.0,
                actions: vec![
                    ForceAction { id: "cf-g".into(), load_case_id: "g-permanent".into(), force: 15_000.0 },
                    ForceAction { id: "cf-q".into(), load_case_id: "q-imposed".into(), force: 10_000.0 },
                ],
            }],
            plated_panels: vec![PlatedPanel {
                id: "pp-1".into(),
                a: 1.2,
                b: 0.6,
                thickness: 0.012,
                fy: 355.0e6,
                k_sigma: 4.0,
                actions: vec![
                    ForceAction { id: "pp-g".into(), load_case_id: "g-permanent".into(), force: 80.0e6 },
                    ForceAction { id: "pp-q".into(), load_case_id: "q-imposed".into(), force: 40.0e6 },
                ],
            }],
            silo_shells: vec![SiloShell {
                id: "silo-1".into(),
                thickness: 0.008,
                radius: 3.0,
                depth: 6.0,
                k: 0.5,
                gamma: 9_000.0,
                fy: 235.0e6,
            }],
            tension_components: vec![TensionComponent {
                id: "ten-1".into(),
                f_uk: 650_000.0,
                f_k: 480_000.0,
                actions: vec![
                    ForceAction { id: "ten-g".into(), load_case_id: "g-permanent".into(), force: 120_000.0 },
                    ForceAction { id: "ten-q".into(), load_case_id: "q-imposed".into(), force: 80_000.0 },
                ],
            }],
            bridge_fatigue: vec![BridgeFatigue {
                id: "br-1".into(),
                member_id: "member-b1".into(),
                lambda: 1.0,
                phi2: 1.0,
                delta_sigma_p: 35.0e6,
                category: 71,
                method: "damage_tolerant".into(),
            }],
            tower_legs: vec![TowerLeg {
                id: "tw-1".into(),
                member_id: "member-b1".into(),
                force_coefficient: 1.2,
                dynamic_factor: 1.0,
                actions: vec![
                    ForceAction { id: "tw-g".into(), load_case_id: "g-permanent".into(), force: 80_000.0 },
                    ForceAction { id: "tw-w".into(), load_case_id: "w-wind".into(), force: 40_000.0 },
                ],
            }],
            piles: vec![SteelPile {
                id: "pile-1".into(),
                section_id: "sec-heb240".into(),
                material_id: "mat-s355".into(),
                driving_stress: 200.0e6,
                embedded_length: 8.0,
                shaft_perimeter: 0.96,
                actions: vec![
                    ForceAction { id: "pile-g".into(), load_case_id: "g-permanent".into(), force: 200_000.0 },
                    ForceAction { id: "pile-q".into(), load_case_id: "q-imposed".into(), force: 100_000.0 },
                ],
            }],
            crane_runways: vec![CraneRunway {
                id: "crane-1".into(),
                member_id: "member-b1".into(),
                wheel_contact_length: 0.10,
                dispersion: 0.05,
                web_thickness: 0.010,
                fy: 355.0e6,
                phi: 1.25,
                actions: vec![
                    ForceAction { id: "cr-g".into(), load_case_id: "g-permanent".into(), force: 20_000.0 },
                    ForceAction { id: "cr-q".into(), load_case_id: "q-imposed".into(), force: 30_000.0 },
                ],
            }],
        }
    }

    /// ❌️ Non-compliant multi-part subject with failures across parts.
    pub fn noncompliant_overloaded_frame() -> Self {
        let mut doc = Self::compliant_heb240_frame();
        if let Some(m) = doc.members.iter_mut().find(|m| m.id == "member-b1") {
            m.buckling_length_y = 12.0;
            m.buckling_length_z = 12.0;
            m.ltb_length = 12.0;
            m.ltb_restraint_spacing = 12.0;
            m.analysis = "plastic".into();
        }
        if let Some(act) = doc.member_actions.iter_mut().find(|a| a.id == "act-b1-g") {
            act.action.n = 2_100_000.0;
            act.action.my = 333_000.0;
            act.action.vz = 296_000.0;
        }
        if let Some(act) = doc.member_actions.iter_mut().find(|a| a.id == "act-b1-q") {
            act.action.n = 0.0;
            act.action.my = 0.0;
            act.action.vz = 0.0;
        }
        if let Some(j) = doc.joints.iter_mut().find(|j| j.id == "joint-j1") {
            j.actions = vec![
                JointForceAction { id: "jf-g".into(), load_case_id: "g-permanent".into(), shear: 100_000.0, tension: 0.0 },
                JointForceAction { id: "jf-q".into(), load_case_id: "q-imposed".into(), shear: 66_667.0, tension: 0.0 },
            ];
            j.bolt_rows = 1;
            j.end_distance = 0.025;
        }
        if let Some(f) = doc.fire_exposures.iter_mut().find(|f| f.id == "fire-1") {
            f.mu0 = 0.85;
            f.protection_thickness = 0.008;
        }
        if let Some(fat) = doc.fatigue_details.iter_mut().find(|f| f.id == "fat-1") {
            fat.spectrum = vec![FatigueBand { id: "fb-1".into(), delta_sigma: 90.0e6, cycles: 2.0e6 }];
        }
        if let Some(cf) = doc.cold_formed_members.iter_mut().find(|c| c.id == "cf-1") {
            cf.actions = vec![ForceAction { id: "cf-g".into(), load_case_id: "g-permanent".into(), force: 120_000.0 }];
            cf.gross_resistance = 40_000.0;
        }
        if let Some(pp) = doc.plated_panels.iter_mut().find(|p| p.id == "pp-1") {
            pp.actions = vec![ForceAction { id: "pp-g".into(), load_case_id: "g-permanent".into(), force: 300.0e6 }];
            pp.thickness = 0.006;
        }
        if let Some(s) = doc.silo_shells.iter_mut().find(|s| s.id == "silo-1") {
            s.thickness = 0.003;
            s.depth = 20.0;
        }
        if let Some(t) = doc.tension_components.iter_mut().find(|t| t.id == "ten-1") {
            t.actions = vec![ForceAction { id: "ten-g".into(), load_case_id: "g-permanent".into(), force: 500_000.0 }];
            t.f_k = 200_000.0;
        }
        if let Some(b) = doc.bridge_fatigue.iter_mut().find(|b| b.id == "br-1") {
            b.delta_sigma_p = 90.0e6;
        }
        if let Some(tw) = doc.tower_legs.iter_mut().find(|t| t.id == "tw-1") {
            tw.actions = vec![
                ForceAction { id: "tw-g".into(), load_case_id: "g-permanent".into(), force: 1_500_000.0 },
                ForceAction { id: "tw-w".into(), load_case_id: "w-wind".into(), force: 800_000.0 },
            ];
        }
        if let Some(p) = doc.piles.iter_mut().find(|p| p.id == "pile-1") {
            p.driving_stress = 400.0e6;
            p.actions = vec![ForceAction { id: "pile-g".into(), load_case_id: "g-permanent".into(), force: 3_000_000.0 }];
        }
        if let Some(c) = doc.crane_runways.iter_mut().find(|c| c.id == "crane-1") {
            c.web_thickness = 0.004;
            c.actions = vec![ForceAction { id: "cr-q".into(), load_case_id: "q-imposed".into(), force: 200_000.0 }];
            c.phi = 1.35;
        }
        doc
    }
}

pub fn catalogue_heb240() -> SteelSection {
    SteelSection {
        id: "sec-heb240".into(),
        designation: "HEB 240".into(),
        kind: "rolledI".into(),
        h: 0.240,
        b: 0.240,
        tw: 0.010,
        tf: 0.017,
        r: 0.021,
        area: 0.0106,
        shear_area_y: 0.00512,
        shear_area_z: 0.00240,
        iy: 1.126e-4,
        iz: 3.923e-5,
        it: 6.53e-7,
        iw: 4.87e-6,
        w_el_y: 9.38e-4,
        w_el_z: 3.27e-4,
        w_pl_y: 1.053e-3,
        w_pl_z: 5.01e-4,
        area_net: 0.0095,
    }
}

/// 📒 HEB 260 rolled I-section (next larger after HEB 240).
pub fn catalogue_heb260() -> SteelSection {
    SteelSection {
        id: "sec-heb260".into(),
        designation: "HEB 260".into(),
        kind: "rolledI".into(),
        h: 0.260,
        b: 0.260,
        tw: 0.010,
        tf: 0.0175,
        r: 0.024,
        area: 0.0118,
        shear_area_y: 0.00570,
        shear_area_z: 0.00260,
        iy: 1.492e-4,
        iz: 5.135e-5,
        it: 8.25e-7,
        iw: 7.45e-6,
        w_el_y: 1.148e-3,
        w_el_z: 3.95e-4,
        w_pl_y: 1.283e-3,
        w_pl_z: 6.02e-4,
        area_net: 0.0106,
    }
}

/// 📒 HEB 280 rolled I-section.
pub fn catalogue_heb280() -> SteelSection {
    SteelSection {
        id: "sec-heb280".into(),
        designation: "HEB 280".into(),
        kind: "rolledI".into(),
        h: 0.280,
        b: 0.280,
        tw: 0.0105,
        tf: 0.018,
        r: 0.024,
        area: 0.0131,
        shear_area_y: 0.00635,
        shear_area_z: 0.00294,
        iy: 1.926e-4,
        iz: 6.595e-5,
        it: 1.02e-6,
        iw: 1.10e-5,
        w_el_y: 1.376e-3,
        w_el_z: 4.71e-4,
        w_pl_y: 1.532e-3,
        w_pl_z: 7.17e-4,
        area_net: 0.0118,
    }
}

/// 📒 HEB 300 rolled I-section.
pub fn catalogue_heb300() -> SteelSection {
    SteelSection {
        id: "sec-heb300".into(),
        designation: "HEB 300".into(),
        kind: "rolledI".into(),
        h: 0.300,
        b: 0.300,
        tw: 0.011,
        tf: 0.019,
        r: 0.027,
        area: 0.0149,
        shear_area_y: 0.00720,
        shear_area_z: 0.00330,
        iy: 2.517e-4,
        iz: 8.563e-5,
        it: 1.37e-6,
        iw: 1.64e-5,
        w_el_y: 1.678e-3,
        w_el_z: 5.71e-4,
        w_pl_y: 1.869e-3,
        w_pl_z: 8.70e-4,
        area_net: 0.0134,
    }
}

/// 📒 Ordered HEB catalogue for remediation section upsizing.
pub fn rolled_heb_catalogue() -> Vec<SteelSection> {
    vec![catalogue_heb240(), catalogue_heb260(), catalogue_heb280(), catalogue_heb300()]
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves


/// 🧱 Steel material grade entry (SI: fy/fu/E/G in Pa).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelMaterial {
    pub id: String,
    pub grade: String,
    pub fy: f64,
    pub fu: f64,
    pub e_modulus: f64,
    pub g_modulus: f64,
    pub subgrade: String,
    pub kind: String,
}

/// 📐️ Steel section with full property table (SI: m, m², m³, m⁴).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelSection {
    pub id: String,
    pub designation: String,
    pub kind: String,
    pub h: f64,
    pub b: f64,
    pub tw: f64,
    pub tf: f64,
    pub r: f64,
    pub area: f64,
    pub shear_area_y: f64,
    pub shear_area_z: f64,
    pub iy: f64,
    pub iz: f64,
    pub it: f64,
    pub iw: f64,
    pub w_el_y: f64,
    pub w_el_z: f64,
    pub w_pl_y: f64,
    pub w_pl_z: f64,
    pub area_net: f64,
}

/// 🏗️ Structural member (lengths in m; ψ dimensionless).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelMember {
    pub id: String,
    pub label: String,
    pub member_type: String,
    pub section_id: String,
    pub material_id: String,
    pub length: f64,
    pub buckling_length_y: f64,
    pub buckling_length_z: f64,
    pub ltb_length: f64,
    pub ltb_restraint_spacing: f64,
    pub load_application: String,
    pub end_moment_ratio_psi: f64,
    pub moment_diagram: String,
    pub deflection_limit_ratio: f64,
    pub analysis: String,
}

/// 🏋️ Design action effects (SI: N, N·m).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct DesignAction {
    pub n: f64,
    pub vy: f64,
    pub vz: f64,
    pub my: f64,
    pub mz: f64,
    pub t: f64,
}

/// 📋 Load case identity.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LoadCase {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub category: String,
}

/// 🔗 Member action under a load case.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MemberAction {
    pub id: String,
    pub member_id: String,
    pub load_case_id: String,
    pub action: DesignAction,
}

/// 📉 Characteristic scalar force under a load case (SI N).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ForceAction {
    pub id: String,
    pub load_case_id: String,
    pub force: f64,
}

/// 🔩 Characteristic joint forces under a load case (SI N).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct JointForceAction {
    pub id: String,
    pub load_case_id: String,
    pub shear: f64,
    pub tension: f64,
}

/// 📊 Fatigue stress-range band (Δσ_i, n_i) for Palmgren-Miner.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueBand {
    pub id: String,
    pub delta_sigma: f64,
    pub cycles: f64,
}

/// 🔩 Bolted or welded joint.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelJoint {
    pub id: String,
    pub kind: String,
    pub member_id: String,
    pub bolt_class: String,
    pub bolt_diameter: f64,
    pub bolt_rows: u32,
    pub bolts_per_row: u32,
    pub pitch: f64,
    pub gauge: f64,
    pub end_distance: f64,
    pub edge_distance: f64,
    pub shear_planes: u32,
    pub plate_thickness: f64,
    pub plate_fu: f64,
    pub weld_throat: f64,
    pub weld_length: f64,
    pub weld_fu: f64,
    pub weld_grade: String,
    #[dsl(table)]
    pub actions: Vec<JointForceAction>,
    pub category: String,
    pub friction_mu: f64,
    pub preload_force: f64,
    pub slip_factor_ks: f64,
    pub friction_surfaces: u32,
}

/// 🔄 Fatigue detail category assessment.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FatigueDetail {
    pub id: String,
    pub member_id: String,
    pub category: u8,
    pub method: String,
    #[dsl(table)]
    pub spectrum: Vec<FatigueBand>,
}

/// 🔥 Fire exposure on a member.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FireExposure {
    pub id: String,
    pub member_id: String,
    pub rating: String,
    pub protection_thickness: f64,
    pub section_factor: f64,
    pub mu0: f64,
    pub design_temperature: f64,
    pub protection_conductivity: f64,
    pub protection_density: f64,
    pub protection_specific_heat: f64,
}

/// ❄️ Cold-formed thin-gauge member (1-3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ColdFormedMember {
    pub id: String,
    pub b_bar: f64,
    pub thickness: f64,
    pub k_sigma: f64,
    pub psi: f64,
    pub fy: f64,
    pub gross_resistance: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
}

/// 🧱 Plated panel for EN 1993-1-5.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PlatedPanel {
    pub id: String,
    pub a: f64,
    pub b: f64,
    pub thickness: f64,
    pub fy: f64,
    pub k_sigma: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
}

/// 🛢️ Silo / shell of revolution.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SiloShell {
    pub id: String,
    pub thickness: f64,
    pub radius: f64,
    pub depth: f64,
    pub k: f64,
    pub gamma: f64,
    pub fy: f64,
}

/// 🪢 Tension component / cable (1-11).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TensionComponent {
    pub id: String,
    pub f_uk: f64,
    pub f_k: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
}

/// 🌉 Bridge fatigue parameters (EN 1993-2).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BridgeFatigue {
    pub id: String,
    pub member_id: String,
    pub lambda: f64,
    pub phi2: f64,
    pub delta_sigma_p: f64,
    pub category: u8,
    pub method: String,
}

/// 🗼 Tower leg (EN 1993-3-1).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TowerLeg {
    pub id: String,
    pub member_id: String,
    pub force_coefficient: f64,
    pub dynamic_factor: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
}

/// 🪵 Steel pile (EN 1993-5).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SteelPile {
    pub id: String,
    pub section_id: String,
    pub material_id: String,
    pub driving_stress: f64,
    pub embedded_length: f64,
    pub shaft_perimeter: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
}

/// 🏗️ Crane runway local web check (EN 1993-6).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CraneRunway {
    pub id: String,
    pub member_id: String,
    pub wheel_contact_length: f64,
    pub dispersion: f64,
    pub web_thickness: f64,
    pub fy: f64,
    pub phi: f64,
    #[dsl(table)]
    pub actions: Vec<ForceAction>,
}

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_s_plugin_norm_app_surface::artifact_kind_spec("en1993", "EN 1993", EN1993_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1993_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1993", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const EN1993_DOCUMENT_SCHEMA: &str = "semio.norm.en1993/v1";
