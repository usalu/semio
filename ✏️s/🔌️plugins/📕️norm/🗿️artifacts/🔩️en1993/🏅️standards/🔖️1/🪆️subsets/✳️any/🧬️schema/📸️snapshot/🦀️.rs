//! 🧬️ En1993 snapshot schema — complete steel-structure subject (SI base units).

use crate::document::AnnexChoice;
use crate::{
    BridgeFatigue, ColdFormedMember, CraneRunway, DesignAction, FatigueBand, FatigueDetail, FireExposure, ForceAction, JointForceAction, LoadCase,
    MemberAction, PlatedPanel, SiloShell, SteelJoint, SteelMaterial, SteelMember, SteelPile, SteelSection, TensionComponent, TowerLeg,
};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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
//#region 🔖️HandcraftedArtifactCodecs
crate::impl_norm_artifact_record!(En1993Snapshot, extension = "en1993", envelope_id = "norm.en1993");
//#endregion 🔖️HandcraftedArtifactCodecs

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

//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`En1993Snapshot`] — the surface
/// `../../../../../🧪️tests/🔩️mutate-en1993-1` is compared through under `ordered-json-v1`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1993_snapshot_json(snapshot: &En1993Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_en1993_snapshot_json`] — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`En1993Snapshot`] values, so the case adapter reads the committed
/// fixture instead of re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible — the generated test host links only this crate — which is why the bridge
/// belongs here.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1993_snapshot_json(text: &str) -> Result<En1993Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`En1993Snapshot`]. Calls the `ArtifactDsl`
/// trait method directly rather than the `📝️text` facet's async wrapper, because a test host has no
/// async runtime to drive one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1993_dsl(text: &str) -> Result<En1993Snapshot, String> {
    <En1993Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`En1993Snapshot`] back to its canonical `.dsl.semio` body. Canonical is the operative
/// word: the committed example assets ARE this function's own output, which is why the identity
/// scenario asserts byte-exactness rather than the no-byte-pass-through inequality.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1993_dsl(snapshot: &En1993Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`En1993Snapshot`] from the binary `.pack.semio` envelope — an independently written
/// codec from the DSL grammar above, which is what makes their agreement evidence that the document
/// was parsed rather than copied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_en1993_pack(bytes: &[u8]) -> Result<En1993Snapshot, String> {
    <En1993Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`En1993Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_en1993_pack(snapshot: &En1993Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
