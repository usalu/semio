//! 🌍️ EN 1997 snapshot — complete geotechnical project subject (SI).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
pub use semio_s_plugin_norm_compliance::design_case::{CaseNumber, CaseRow, CaseText};

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// ⚖️ DIN 1054 design situation BS-P, BS-T or BS-A of a geotechnical check.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum SituationChoice {
    Persistent,
    Transient,
    Accidental,
}

/// 🧭 Design approach of EN 1997-1 2.4.7.3.4 (DA1 combinations, DA2, DA2*, DA3, GEO-3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum ApproachChoice {
    Da1Str,
    Da1Geo,
    Da2,
    Da2Star,
    Da3,
    Geo3,
}

/// 📸️ Persisted EN 1997 geotechnical design package.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1997", layout = "lines")]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Snapshot {
    #[state(artifact)]
    pub structure_id: String,
    #[state(artifact)]
    pub geotechnical_category: u8,
    #[state(artifact)]
    pub design_situation: SituationChoice,
    #[state(artifact)]
    pub design_approach: ApproachChoice,
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub groundwater_level: f64,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub investigation_depth: f64,
    #[dsl(table)]
    #[state(artifact)]
    pub layers: Vec<SoilLayer>,
    #[dsl(table)]
    #[state(artifact)]
    pub footings: Vec<SpreadFoundation>,
    #[dsl(table)]
    #[state(artifact)]
    pub piles: Vec<Pile>,
    #[dsl(table)]
    #[state(artifact)]
    pub retaining_walls: Vec<RetainingWall>,
    #[dsl(table)]
    #[state(artifact)]
    pub slopes: Vec<Slope>,
    #[dsl(table)]
    #[state(artifact)]
    pub uplift_cases: Vec<UpliftCase>,
    #[dsl(table)]
    #[state(artifact)]
    pub anchors: Vec<Anchor>,
    #[dsl(table)]
    #[state(artifact)]
    pub embedded_walls: Vec<EmbeddedWall>,
    #[dsl(table)]
    #[state(artifact)]
    pub pile_groups: Vec<PileGroup>,
    #[dsl(table)]
    #[state(artifact)]
    pub lab_tests: Vec<LabTest>,
    #[dsl(table)]
    #[state(artifact)]
    pub layer_waters: Vec<LayerWater>,
    #[dsl(table)]
    #[state(artifact)]
    pub placements: Vec<Placement>,
    #[dsl(table)]
    #[state(artifact)]
    pub cases: Vec<CaseRow>,
    #[dsl(table)]
    #[state(artifact)]
    pub investigations: Vec<Investigation>,
    #[dsl(table)]
    #[state(artifact)]
    pub characteristic_bases: Vec<CharacteristicBasis>,
}
//#endregion 🔖️Snapshot

impl En1997Snapshot {
    /// 📸️ Reference EN 1997 subject under the explicitly chosen annex; the annex is never defaulted.
    pub fn example(annex: semio_s_plugin_norm_compliance::AnnexChoice) -> Self {
        Self { annex, ..compliant_demo() }
    }
}

/// 🏗 Realistic compliant DE / DA2 geotechnical package (spread footing + bored pile + wall + slope).
pub fn compliant_demo() -> En1997Snapshot {
    use crate::{AnchorTest, FoundationLoadCase, LabPoint, PileTestProfile};
    En1997Snapshot {
        structure_id: "geo-demo-compliant".into(),
        geotechnical_category: 2,
        design_situation: SituationChoice::Persistent,
        design_approach: ApproachChoice::Da2,
        annex: AnnexChoice::De,
        groundwater_level: 4.0,
        investigation_depth: 20.0,
        layers: vec![
            SoilLayer {
                id: "layer-sand".into(),
                soil_type: SoilType::Sand,
                depth_top: 0.0,
                depth_bottom: 8.0,
                gamma: 19_000.0,
                gamma_prime: 10_000.0,
                phi_prime_deg: 32.0,
                cohesion_effective: 0.0,
                cohesion_undrained: 0.0,
                oedometric_modulus: 40_000_000.0,
                poisson_ratio: 0.3,
                cpt_qc: 8_000_000.0,
                spt_n: 18.0,
                cpt_nkt: 0.0,
            },
            SoilLayer {
                id: "layer-gravel".into(),
                soil_type: SoilType::Gravel,
                depth_top: 8.0,
                depth_bottom: 20.0,
                gamma: 20_000.0,
                gamma_prime: 11_000.0,
                phi_prime_deg: 35.0,
                cohesion_effective: 0.0,
                cohesion_undrained: 0.0,
                oedometric_modulus: 60_000_000.0,
                poisson_ratio: 0.28,
                cpt_qc: 12_000_000.0,
                spt_n: 28.0,
                cpt_nkt: 0.0,
            },
        ],
        footings: vec![SpreadFoundation {
            id: "footing-F1".into(),
            width: 2.5,
            length: 2.5,
            embedment: 1.5,
            base_inclination_deg: 0.0,
            settlement_limit: 0.025,
            load_cases: vec![FoundationLoadCase {
                id: "lc-bsP".into(),
                design_situation: SituationChoice::Persistent,
                vertical_permanent: 400_000.0,
                vertical_variable: 150_000.0,
                horizontal_permanent: 20_000.0,
                horizontal_variable: 40_000.0,
                moment_permanent: 30_000.0,
                moment_variable: 50_000.0,
            }],
        }],
        piles: vec![Pile {
            id: "pile-P1".into(),
            pile_type: PileType::Bored,
            diameter: 0.6,
            length: 14.0,
            count: 1,
            alpha_s: 0.7,
            unit_shaft_resistance: 90_000.0,
            unit_base_resistance: 3_000_000.0,
            compression_permanent: 500_000.0,
            compression_variable: 100_000.0,
            tension_permanent: 0.0,
            tension_variable: 0.0,
            bending_stiffness_ei: 197_200_000.0,
            horizontal_subgrade_modulus: 20_000_000.0,
            lateral_head_load: 50_000.0,
            elastic_modulus: 0.0,
            settling_thickness: 0.0,
            negative_friction_beta: 0.0,
            dynamic_test_count: 0.0,
            dynamic_resistance_mean: 0.0,
            dynamic_resistance_min: 0.0,
            hammer_weight: 0.0,
            drop_height: 0.0,
            set_per_blow: 0.0,
            elastic_compression: 0.0,
            hammer_efficiency: 0.0,
            concrete_class: PileConcreteClass::C3037,
            test_profiles: vec![
                PileTestProfile { id: "test-1".into(), shaft_resistance: 1_600_000.0, base_resistance: 850_000.0 },
                PileTestProfile { id: "test-2".into(), shaft_resistance: 1_550_000.0, base_resistance: 820_000.0 },
                PileTestProfile { id: "test-3".into(), shaft_resistance: 1_650_000.0, base_resistance: 880_000.0 },
            ],
        }],
        retaining_walls: vec![RetainingWall {
            id: "wall-W1".into(),
            height: 4.0,
            embedment: 1.2,
            base_width: 2.5,
            stem_thickness: 0.35,
            backfill_phi_deg: 30.0,
            backfill_gamma: 18_000.0,
            wall_friction_deg: 20.0,
            earth_pressure_mode: EarthPressureMode::Active,
            wall_movement: WallMovement::Free,
            ocr: 1.0,
            concrete_gamma: 25_000.0,
            surcharge: 10_000.0,
            vertical_permanent: 320_000.0,
            horizontal_permanent: 40_000.0,
            seismic_kh: 0.0,
            seismic_kv: 0.0,
            reinforcement_spacing: 0.0,
            reinforcement_width: 0.0,
            reinforcement_friction: 0.0,
        }],
        slopes: vec![Slope {
            id: "slope-S1".into(),
            angle_deg: 25.0,
            height: 6.0,
            length: 14.0,
            governing_layer_id: "layer-sand".into(),
            circle_centre_x: 0.0,
            circle_centre_z: 0.0,
            circle_radius: 0.0,
            width: 0.0,
        }],
        uplift_cases: vec![UpliftCase {
            id: "upl-U1".into(),
            permanent_stabilizing: 900_000.0,
            permanent_destabilizing: 500_000.0,
            variable_destabilizing: 50_000.0,
            pore_pressure: 40_000.0,
            total_stress: 120_000.0,
        }],
        anchors: vec![Anchor {
            id: "anchor-A1".into(),
            wall_id: "ewall-E1".into(),
            tendon: TendonType::Strand,
            head_depth: 1.0,
            inclination_deg: 15.0,
            free_length: 8.0,
            fixed_length: 6.0,
            fixed_diameter: 0.15,
            bond_stress: 200_000.0,
            design_force: 300_000.0,
            tendon_yield_force: 700_000.0,
            min_free_length: 4.5,
            test_factor: 1.10,
            tests: vec![
                AnchorTest { id: "atest-1".into(), pull_out_resistance: 650_000.0 },
                AnchorTest { id: "atest-2".into(), pull_out_resistance: 640_000.0 },
                AnchorTest { id: "atest-3".into(), pull_out_resistance: 660_000.0 },
            ],
        }],
        embedded_walls: vec![EmbeddedWall {
            id: "ewall-E1".into(),
            kind: EmbeddedWallKind::FreeEarth,
            retained_height: 4.0,
            embedment: 6.0,
            prop_depth: 1.0,
            thickness: 0.6,
            backfill_gamma: 19_000.0,
            phi_deg: 32.0,
            delta_active_deg: 16.0,
            delta_passive_deg: 10.0,
            surcharge: 10_000.0,
            undrained_strength: 0.0,
        }],
        pile_groups: vec![PileGroup {
            id: "group-G1".into(),
            pile_id: "pile-P1".into(),
            rows: 2,
            columns: 2,
            spacing: 1.8,
            cap_vertical: 1_800_000.0,
            cap_moment_x: 100_000.0,
            cap_moment_y: 60_000.0,
            single_resistance: 1_400_000.0,
            shaft_cohesion_sum: 400_000.0,
            base_cohesion: 120_000.0,
        }],
        lab_tests: vec![LabTest {
            id: "lab-T1".into(),
            layer_id: "layer-sand".into(),
            kind: LabTestKind::DirectShear,
            void_ratio: 0.65,
            preload_stress: 100_000.0,
            drainage_height: 0.01,
            consolidation_time: 120.0,
            compression_index: 0.0,
            swelling_index: 0.0,
            stress_increment: 0.0,
            secondary_compression_index: 0.0,
            design_life: 0.0,
            points: vec![
                LabPoint { id: "pt-1".into(), normal_stress: 50_000.0, shear_stress: 32_500.0 },
                LabPoint { id: "pt-2".into(), normal_stress: 100_000.0, shear_stress: 65_000.0 },
                LabPoint { id: "pt-3".into(), normal_stress: 200_000.0, shear_stress: 130_000.0 },
            ],
        }],
        layer_waters: vec![LayerWater {
            id: "water-gravel".into(),
            layer_id: "layer-gravel".into(),
            piezometric_depth: 4.0,
            permeability: 1.0e-3,
            head_difference: 1.0,
            flow_length: 6.0,
            well_radius: 0.0,
            well_water_depth: 0.0,
            influence_radius: 0.0,
            well_capacity: 0.0,
        }],
        placements: vec![],
        cases: vec![CaseRow {
            id: "case-equ-01".into(),
            label_en: "Overturning of the retaining wall".into(),
            label_de: "Kippen der Stützwand".into(),
            entry_id: "EQU-01".into(),
            numbers: vec![CaseNumber { id: "m_dst_g".into(), value: 450_000.0 }, CaseNumber { id: "m_dst_q".into(), value: 150_000.0 }, CaseNumber { id: "m_stb_g".into(), value: 900_000.0 }],
            texts: vec![CaseText { id: "situation".into(), value: "bsP".into() }],
        }],
        investigations: vec![],
        characteristic_bases: vec![],
    }
}

/// 🎬️ The compliant package with three placed footings and a 6 m excavation at the embedded wall: the water table at 4 m depth leaves a 2 m head across the wall, so piping (HYD-02), the seepage streamlines and the exit-gradient diagram show in the demo document.
pub fn showcase_demo() -> En1997Snapshot {
    let mut s = compliant_demo();
    for wall in &mut s.embedded_walls {
        wall.retained_height = 6.0;
    }
    for wall in &mut s.retaining_walls {
        wall.seismic_kh = 0.15;
        wall.seismic_kv = 0.05;
    }
    for (id, factor, x, y) in [("footing-F2", 0.6, 9.0, 0.0), ("footing-F3", 0.8, 0.0, 9.0)] {
        let mut footing = s.footings[0].clone();
        footing.id = id.into();
        for lc in &mut footing.load_cases {
            lc.vertical_permanent *= factor;
            lc.vertical_variable *= factor;
            lc.horizontal_permanent *= factor;
            lc.horizontal_variable *= factor;
            lc.moment_permanent *= factor;
            lc.moment_variable *= factor;
        }
        s.footings.push(footing);
        s.placements.push(Placement { id: id.into(), x, y, z: 0.0, yaw: 0.0 });
    }
    let first = s.footings[0].id.clone();
    s.placements.push(Placement { id: first, x: 0.0, y: 0.0, z: 0.0, yaw: 0.0 });
    s.investigations = vec![
        investigation_point("cpt-1", InvestigationKind::Cpt, (4.5, 4.5), 20.0, &[2.0e6, 3.0e6, 4.5e6, 6.0e6, 8.0e6, 9.0e6, 10.0e6, 12.0e6, 13.0e6, 14.0e6, 16.0e6]),
        investigation_point("spt-1", InvestigationKind::Spt, (-4.5, 4.5), 9.0, &[8.0, 12.0, 15.0, 18.0, 22.0, 26.0, 30.0, 34.0]),
    ];
    s
}

fn investigation_point(id: &str, kind: InvestigationKind, at: (f64, f64), depth: f64, readings: &[f64]) -> Investigation {
    Investigation {
        id: id.into(),
        position_x: at.0,
        position_y: at.1,
        ground_level: 0.0,
        depth,
        kind,
        readings: readings.iter().enumerate().map(|(k, value)| InvestigationReading { id: format!("{id}-r{}", k + 1), depth: k as f64 + 1.0, measurement: *value }).collect(),
    }
}

/// 🚨 Non-compliant package with multiple GEO/UPL failures for remedy tests.
pub fn noncompliant_demo() -> En1997Snapshot {
    use crate::{AnchorTest, LabPoint, PileTestProfile};
    let mut s = compliant_demo();
    s.structure_id = "geo-demo-failing".into();
    s.investigation_depth = 3.0;
    if let Some(f) = s.footings.first_mut() {
        f.width = 1.2;
        f.length = 1.2;
        f.settlement_limit = 0.010;
        if let Some(lc) = f.load_cases.first_mut() {
            lc.vertical_permanent = 1_200_000.0;
            lc.vertical_variable = 400_000.0;
            lc.horizontal_permanent = 150_000.0;
            lc.horizontal_variable = 100_000.0;
            lc.moment_permanent = 200_000.0;
            lc.moment_variable = 150_000.0;
        }
    }
    if let Some(layer) = s.layers.first_mut() {
        layer.oedometric_modulus = 5_000_000.0;
        layer.phi_prime_deg = 28.0;
    }
    if let Some(p) = s.piles.first_mut() {
        p.length = 6.0;
        p.compression_permanent = 1_200_000.0;
        p.compression_variable = 400_000.0;
        p.tension_permanent = 200_000.0;
        p.tension_variable = 50_000.0;
        p.test_profiles = vec![PileTestProfile { id: "test-1".into(), shaft_resistance: 400_000.0, base_resistance: 200_000.0 }];
    }
    if let Some(w) = s.retaining_walls.first_mut() {
        w.base_width = 0.8;
        w.height = 5.5;
        w.surcharge = 40_000.0;
    }
    if let Some(sl) = s.slopes.first_mut() {
        sl.angle_deg = 42.0;
    }
    if let Some(u) = s.uplift_cases.first_mut() {
        u.permanent_stabilizing = 400_000.0;
        u.permanent_destabilizing = 600_000.0;
        u.variable_destabilizing = 200_000.0;
        u.pore_pressure = 100_000.0;
        u.total_stress = 90_000.0;
    }
    if let Some(w) = s.embedded_walls.first_mut() {
        w.embedment = 1.0;
    }
    if let Some(a) = s.anchors.first_mut() {
        a.design_force = 900_000.0;
        a.free_length = 2.0;
        a.tests = vec![AnchorTest { id: "atest-1".into(), pull_out_resistance: 400_000.0 }];
    }
    if let Some(g) = s.pile_groups.first_mut() {
        g.cap_vertical = 9_000_000.0;
        g.single_resistance = 1_000_000.0;
    }
    if let Some(t) = s.lab_tests.first_mut() {
        t.points = vec![
            LabPoint { id: "pt-1".into(), normal_stress: 50_000.0, shear_stress: 20_000.0 },
            LabPoint { id: "pt-2".into(), normal_stress: 100_000.0, shear_stress: 38_000.0 },
            LabPoint { id: "pt-3".into(), normal_stress: 200_000.0, shear_stress: 76_000.0 },
        ];
    }
    if let Some(w) = s.layer_waters.first_mut() {
        w.head_difference = 6.0;
        w.flow_length = 2.0;
    }
    s
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1997_model_typescript::LEAF, graphql: semio_s_artifact_norm_en1997_model_graphql::LEAF, json_schema: semio_s_artifact_norm_en1997_model_json_schema::LEAF, proto: semio_s_artifact_norm_en1997_model_proto::LEAF };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;

pub fn artifact_kind() -> semio_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1997", "EN 1997", EN1997_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1997_DIALECT: semio_plugin::app::Dialect = semio_plugin::app::Dialect { artifact_kind: "s.norm.en1997", standard: semio_plugin::app::StandardId("1"), subset: semio_plugin::app::SubsetId::ANY };

pub const EN1997_DOCUMENT_SCHEMA: &str = "semio.norm.en1997/v1";

/// 🪨 Soil class of a layer (GK1 presumptive bearing table).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum SoilType {
    Sand,
    Gravel,
    Clay,
    Silt,
    Fill,
}

/// 🪨 Stratigraphic soil layer with characteristic parameters (SI).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SoilLayer {
    pub id: String,
    pub soil_type: SoilType,
    #[dsl(unit = "m")]
    pub depth_top: f64,
    #[dsl(unit = "m")]
    pub depth_bottom: f64,
    pub gamma: f64,
    pub gamma_prime: f64,
    pub phi_prime_deg: f64,
    #[dsl(unit = "Pa")]
    pub cohesion_effective: f64,
    #[dsl(unit = "Pa")]
    pub cohesion_undrained: f64,
    #[dsl(unit = "Pa")]
    pub oedometric_modulus: f64,
    pub poisson_ratio: f64,
    #[dsl(unit = "Pa")]
    pub cpt_qc: f64,
    pub spt_n: f64,
    /// 🥞 Cone factor N_kt of the CPT-derived undrained shear strength c_u = (q_c - sigma_v0) / N_kt (GI-03); 0 = not given, no value is assumed.
    pub cpt_nkt: f64,
}

/// 🏋️ Characteristic permanent/variable actions for one design situation.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FoundationLoadCase {
    pub id: String,
    pub design_situation: SituationChoice,
    #[dsl(unit = "N")]
    pub vertical_permanent: f64,
    #[dsl(unit = "N")]
    pub vertical_variable: f64,
    #[dsl(unit = "N")]
    pub horizontal_permanent: f64,
    #[dsl(unit = "N")]
    pub horizontal_variable: f64,
    #[dsl(unit = "J")]
    pub moment_permanent: f64,
    #[dsl(unit = "J")]
    pub moment_variable: f64,
}

/// 🦶️ Spread foundation (pad/strip) geometry and actions.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SpreadFoundation {
    pub id: String,
    #[dsl(unit = "m")]
    pub width: f64,
    #[dsl(unit = "m")]
    pub length: f64,
    #[dsl(unit = "m")]
    pub embedment: f64,
    pub base_inclination_deg: f64,
    #[dsl(unit = "m")]
    pub settlement_limit: f64,
    #[dsl(table)]
    pub load_cases: Vec<FoundationLoadCase>,
}

/// 🧪 Pile load-test or CPT/table-derived calculated resistance profile.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PileTestProfile {
    pub id: String,
    #[dsl(unit = "N")]
    pub shaft_resistance: f64,
    #[dsl(unit = "N")]
    pub base_resistance: f64,
}

/// 🧱 Concrete strength class of a pile shaft (EN 1992-1-1 Table 3.1, C12/15 to C50/60); `NotGiven` leaves the explicit modulus as the only source of E_c.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum PileConcreteClass {
    NotGiven,
    C1215,
    C1620,
    C2025,
    C2530,
    C3037,
    C3545,
    C4050,
    C4555,
    C5060,
}

impl PileConcreteClass {
    /// 🆔 The class designation of EN 1992-1-1 Table 3.1 (`C30/37`) the shared registry rows `ndp.en1992.t3-1.ecm.*` are keyed by; `None` for `NotGiven`.
    pub const fn designation(self) -> Option<&'static str> {
        match self {
            Self::NotGiven => None,
            Self::C1215 => Some("C12/15"),
            Self::C1620 => Some("C16/20"),
            Self::C2025 => Some("C20/25"),
            Self::C2530 => Some("C25/30"),
            Self::C3037 => Some("C30/37"),
            Self::C3545 => Some("C35/45"),
            Self::C4050 => Some("C40/50"),
            Self::C4555 => Some("C45/55"),
            Self::C5060 => Some("C50/60"),
        }
    }

    /// 🏷️ The wire token (`c3037`) the JSON, SQLite and editor choice lists use.
    pub const fn token(self) -> &'static str {
        match self {
            Self::NotGiven => "notGiven",
            Self::C1215 => "c1215",
            Self::C1620 => "c1620",
            Self::C2025 => "c2025",
            Self::C2530 => "c2530",
            Self::C3037 => "c3037",
            Self::C3545 => "c3545",
            Self::C4050 => "c4050",
            Self::C4555 => "c4555",
            Self::C5060 => "c5060",
        }
    }

    /// 🧾 Every class in declaration order (the editor choice list and the SQLite check constraint follow it).
    pub const ALL: [Self; 10] = [Self::NotGiven, Self::C1215, Self::C1620, Self::C2025, Self::C2530, Self::C3037, Self::C3545, Self::C4050, Self::C4555, Self::C5060];
}

/// 🏗 Pile installation type (bored, driven or CFA).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum PileType {
    Bored,
    Driven,
    Cfa,
}

/// 🏗️ Deep foundation pile or pile group.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Pile {
    pub id: String,
    pub pile_type: PileType,
    #[dsl(unit = "m")]
    pub diameter: f64,
    #[dsl(unit = "m")]
    pub length: f64,
    pub count: u32,
    pub alpha_s: f64,
    #[dsl(unit = "Pa")]
    pub unit_shaft_resistance: f64,
    #[dsl(unit = "Pa")]
    pub unit_base_resistance: f64,
    #[dsl(unit = "N")]
    pub compression_permanent: f64,
    #[dsl(unit = "N")]
    pub compression_variable: f64,
    #[dsl(unit = "N")]
    pub tension_permanent: f64,
    #[dsl(unit = "N")]
    pub tension_variable: f64,
    /// 🪵 Pile bending stiffness EI [N·m²] for the lateral Winkler beam and the buckling interface (PILE-04, PILE-07); 0 = not given (cracking is not applied).
    pub bending_stiffness_ei: f64,
    /// 🪵 Horizontal modulus of subgrade reaction k_h [N/m³] (constant with depth) of the Winkler foundation of the lateral pile; 0 = not given.
    pub horizontal_subgrade_modulus: f64,
    /// 🪵 Characteristic horizontal load at the pile head [N] for the lateral deflection line; 0 = none.
    #[dsl(unit = "N")]
    pub lateral_head_load: f64,
    /// 🪵 Elastic modulus E_c of the pile material [Pa] for the elastic shortening of the resistance-settlement curve (PILE-09); an input of the pile (EN 1992-1-1 Table 3.1 gives E_cm per concrete class), 0 = not given.
    #[dsl(unit = "Pa")]
    pub elastic_modulus: f64,
    /// 🪵 Thickness h₁ [m] of the settling ground from the surface along the shaft for the negative skin friction (PILE-05); 0 = no negative skin friction.
    #[dsl(unit = "m")]
    pub settling_thickness: f64,
    /// 🪵 Shaft factor β of the negative skin friction τ = β·σ′v (PILE-05); 0 = default 0.25 of the catalogue.
    pub negative_friction_beta: f64,
    /// 🪵 Number of dynamic pile tests n (integer, ξ5/ξ6 of PILE-08); 0 = no dynamic tests.
    pub dynamic_test_count: f64,
    /// 🪵 Mean measured resistance of the dynamic tests [N] (PILE-08).
    #[dsl(unit = "N")]
    pub dynamic_resistance_mean: f64,
    /// 🪵 Lowest measured resistance of the dynamic tests [N] (PILE-08).
    #[dsl(unit = "N")]
    pub dynamic_resistance_min: f64,
    /// 🪵 Hammer weight W [N] of the driving formula (PILE-08); 0 = no driving record.
    #[dsl(unit = "N")]
    pub hammer_weight: f64,
    /// 🪵 Drop height h [m] of the driving formula (PILE-08).
    #[dsl(unit = "m")]
    pub drop_height: f64,
    /// 🪵 Permanent set s per blow [m] of the driving formula (PILE-08).
    #[dsl(unit = "m")]
    pub set_per_blow: f64,
    /// 🪵 Temporary elastic compression s_e [m] of pile and ground per blow (PILE-08).
    #[dsl(unit = "m")]
    pub elastic_compression: f64,
    /// 🪵 Hammer efficiency η_h of the driving formula (PILE-08).
    pub hammer_efficiency: f64,
    /// 🪵 Concrete strength class of the pile shaft (EN 1992-1-1 Table 3.1) for the elastic shortening of the resistance-settlement curve (PILE-09): E_cm from the shared rows `ndp.en1992.t3-1.ecm.*`; `notGiven` leaves the explicit modulus `elasticModulus` as the only source, a modulus above 0 overrides the class.
    pub concrete_class: PileConcreteClass,
    #[dsl(table)]
    pub test_profiles: Vec<PileTestProfile>,
}

/// 🧱 Earth pressure regime behind a retaining wall (DIN 4085).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum EarthPressureMode {
    Active,
    AtRest,
    IncreasedActive,
}

/// 🕳️ Ground investigation kind (EN 1997-2): borehole, cone penetration test or standard penetration test.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum InvestigationKind {
    Borehole,
    Cpt,
    Spt,
}

/// ↔️ Wall movement regime of a retaining wall (DIN 4085).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum WallMovement {
    Free,
    Rigid,
    Propped,
}

/// 🧱 Retaining wall geometry and surcharge.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RetainingWall {
    pub id: String,
    #[dsl(unit = "m")]
    pub height: f64,
    #[dsl(unit = "m")]
    pub embedment: f64,
    #[dsl(unit = "m")]
    pub base_width: f64,
    #[dsl(unit = "m")]
    pub stem_thickness: f64,
    pub backfill_phi_deg: f64,
    pub backfill_gamma: f64,
    pub wall_friction_deg: f64,
    /// 🧱 active | atRest | increasedActive (DIN 4085 earth-pressure regime).
    pub earth_pressure_mode: EarthPressureMode,
    /// 🧱 free | rigid | propped (DIN 4085 wall-movement class).
    pub wall_movement: WallMovement,
    /// 🧱 Overconsolidation ratio for Jáky K₀.
    pub ocr: f64,
    /// 🧱 Stem/base concrete unit weight γ_c [N/m³].
    pub concrete_gamma: f64,
    #[dsl(unit = "Pa")]
    pub surcharge: f64,
    #[dsl(unit = "N")]
    pub vertical_permanent: f64,
    #[dsl(unit = "N")]
    pub horizontal_permanent: f64,
    /// 🌋 Horizontal seismic coefficient k_h of the pseudo-static Mononobe-Okabe thrust (RW-08, EN 1998-5 §7.3); 0 = no seismic situation.
    pub seismic_kh: f64,
    /// 🌋 Vertical seismic coefficient k_v of the pseudo-static Mononobe-Okabe thrust (RW-08); positive downward, both signs are checked.
    pub seismic_kv: f64,
    /// 🧱 Vertical spacing S_v [m] of the soil reinforcement layers for the internal stability (RW-09); 0 = unreinforced.
    #[dsl(unit = "m")]
    pub reinforcement_spacing: f64,
    /// 🧱 Width b [m] of one reinforcement element (strip width, or 1 m for sheets) (RW-09).
    #[dsl(unit = "m")]
    pub reinforcement_width: f64,
    /// 🧱 Apparent friction coefficient F* between soil and reinforcement (RW-09).
    pub reinforcement_friction: f64,
}

/// ⛰️ Slope geometry for overall stability (GEO-3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Slope {
    pub id: String,
    pub angle_deg: f64,
    #[dsl(unit = "m")]
    pub height: f64,
    #[dsl(unit = "m")]
    pub length: f64,
    pub governing_layer_id: String,
    /// ⛰️ Optional slip-circle override: centre abscissa from the crest [m]; used only when the radius is positive.
    #[dsl(unit = "m")]
    pub circle_centre_x: f64,
    /// ⛰️ Optional slip-circle override: centre elevation above the toe [m].
    #[dsl(unit = "m")]
    pub circle_centre_z: f64,
    /// ⛰️ Optional slip-circle override: radius [m]; 0 searches the critical circle.
    #[dsl(unit = "m")]
    pub circle_radius: f64,
    /// ⛰️ Width of the slope along its axis [m] for the extruded 3D slip body (SLOPE-05); 0 keeps the plane-strain check and draws the body over the slope length.
    #[dsl(unit = "m")]
    pub width: f64,
}

/// 💧 Uplift / heave verification case (§10 UPL/HYD).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct UpliftCase {
    pub id: String,
    #[dsl(unit = "N")]
    pub permanent_stabilizing: f64,
    #[dsl(unit = "N")]
    pub permanent_destabilizing: f64,
    #[dsl(unit = "N")]
    pub variable_destabilizing: f64,
    #[dsl(unit = "Pa")]
    pub pore_pressure: f64,
    #[dsl(unit = "Pa")]
    pub total_stress: f64,
}

/// 📌 Suitability-test pull-out resistance of one anchor (ANC-01).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AnchorTest {
    pub id: String,
    #[dsl(unit = "N")]
    pub pull_out_resistance: f64,
}

/// 🔗 Tendon type of a ground anchor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum TendonType {
    Strand,
    Bar,
}

/// ⚓ Ground anchor of an embedded wall: geometry, bond, tendon and the suitability-test series (ANC-01…ANC-04).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Anchor {
    pub id: String,
    /// ⚓ Id of the supported embedded wall.
    pub wall_id: String,
    /// ⚓ strand | bar.
    pub tendon: TendonType,
    #[dsl(unit = "m")]
    pub head_depth: f64,
    pub inclination_deg: f64,
    #[dsl(unit = "m")]
    pub free_length: f64,
    #[dsl(unit = "m")]
    pub fixed_length: f64,
    #[dsl(unit = "m")]
    pub fixed_diameter: f64,
    /// ⚓ Ultimate skin friction τ_f of the grout body.
    #[dsl(unit = "Pa")]
    pub bond_stress: f64,
    /// ⚓ Design anchor force A_d.
    #[dsl(unit = "N")]
    pub design_force: f64,
    /// ⚓ Characteristic tendon force P_t,k.
    #[dsl(unit = "N")]
    pub tendon_yield_force: f64,
    #[dsl(unit = "m")]
    pub min_free_length: f64,
    /// ⚓ Correlation factor ξ_a.
    pub test_factor: f64,
    #[dsl(table)]
    pub tests: Vec<AnchorTest>,
}

/// 🧱 Support condition of an embedded wall.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum EmbeddedWallKind {
    FreeEarth,
    Cantilever,
}

/// 🧮 Source of a characteristic parameter: the declared layer value or the value derived from the ground investigations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum ParameterBasis {
    Declared,
    Derived,
}

/// 🎯 Soil parameter whose characteristic value has a typed basis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum CharacteristicParameter {
    UndrainedStrength,
    FrictionAngle,
}

/// 🧱 Embedded (sheet-pile or diaphragm) wall for the free-earth-support and cantilever verification (RW-07).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct EmbeddedWall {
    pub id: String,
    /// 🧱 freeEarth | cantilever.
    pub kind: EmbeddedWallKind,
    #[dsl(unit = "m")]
    pub retained_height: f64,
    /// 🧱 Provided embedment below the dredge level.
    #[dsl(unit = "m")]
    pub embedment: f64,
    #[dsl(unit = "m")]
    pub prop_depth: f64,
    #[dsl(unit = "m")]
    pub thickness: f64,
    pub backfill_gamma: f64,
    pub phi_deg: f64,
    pub delta_active_deg: f64,
    pub delta_passive_deg: f64,
    #[dsl(unit = "Pa")]
    pub surcharge: f64,
    /// 🧱 Undrained strength c_u of the base soil for the basal-heave check; 0 = drained.
    #[dsl(unit = "Pa")]
    pub undrained_strength: f64,
}

/// 🏗️ Pile group on a rigid cap (PILE-06).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PileGroup {
    pub id: String,
    /// 🏗️ Id of the pile type repeated in the group.
    pub pile_id: String,
    pub rows: u32,
    pub columns: u32,
    #[dsl(unit = "m")]
    pub spacing: f64,
    #[dsl(unit = "N")]
    pub cap_vertical: f64,
    #[dsl(unit = "J")]
    pub cap_moment_x: f64,
    #[dsl(unit = "J")]
    pub cap_moment_y: f64,
    /// 🏗️ Design compression resistance R_c,d of one pile.
    #[dsl(unit = "N")]
    pub single_resistance: f64,
    /// 🏗️ Σ c_u,i·h_i along the block perimeter [Pa·m].
    #[dsl(unit = "Pa")]
    pub shaft_cohesion_sum: f64,
    #[dsl(unit = "Pa")]
    pub base_cohesion: f64,
}

/// 📈 One stress/strength pair of a laboratory series.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LabPoint {
    pub id: String,
    #[dsl(unit = "Pa")]
    pub normal_stress: f64,
    #[dsl(unit = "Pa")]
    pub shear_stress: f64,
}

/// 🧪 Laboratory test kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum LabTestKind {
    Oedometer,
    Triaxial,
    DirectShear,
}

/// 🧪 Laboratory test series of one layer (EN 1997-2 oedometer, triaxial and direct shear; GI-10…GI-12).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LabTest {
    pub id: String,
    pub layer_id: String,
    /// 🧪 oedometer | triaxial | directShear.
    pub kind: LabTestKind,
    pub void_ratio: f64,
    #[dsl(unit = "Pa")]
    pub preload_stress: f64,
    #[dsl(unit = "m")]
    pub drainage_height: f64,
    pub consolidation_time: f64,
    /// 🧪 Compression index C_c of the oedometer test for the primary consolidation settlement (SLS-02); 0 = not given.
    pub compression_index: f64,
    /// 🧪 Swelling index C_s of the oedometer test (SLS-02); 0 = 0.1·C_c.
    pub swelling_index: f64,
    /// 🧪 Vertical stress increment Δσ [Pa] at the layer mid-depth from the structure (SLS-02); 0 = not given.
    #[dsl(unit = "Pa")]
    pub stress_increment: f64,
    /// 🧪 Secondary compression index C_α of the oedometer test for the creep settlement (SLS-03); 0 = not given.
    pub secondary_compression_index: f64,
    /// 🧪 Design life t₂ [s] of the structure for the time settlement and the creep (SLS-02, SLS-03); 0 = not given.
    pub design_life: f64,
    #[dsl(table)]
    pub points: Vec<LabPoint>,
}

/// 💧 Water regime of one soil layer: piezometric level, permeability and hydraulic gradient (HYD-01, GI-15).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct LayerWater {
    pub id: String,
    pub layer_id: String,
    #[dsl(unit = "m")]
    pub piezometric_depth: f64,
    pub permeability: f64,
    #[dsl(unit = "m")]
    pub head_difference: f64,
    #[dsl(unit = "m")]
    pub flow_length: f64,
    /// 💧 Radius r_w [m] of a dewatering well in this layer (DEW-01); 0 = no well.
    #[dsl(unit = "m")]
    pub well_radius: f64,
    /// 💧 Depth [m] of the water level in the pumped well below the surface (DEW-01).
    #[dsl(unit = "m")]
    pub well_water_depth: f64,
    /// 💧 Radius of influence R [m] of the well (DEW-01); 0 = Sichardt.
    #[dsl(unit = "m")]
    pub influence_radius: f64,
    /// 💧 Capacity of one well [m³/s] for the required number of wells (DEW-01); 0 = not given.
    pub well_capacity: f64,
}

/// 📈 One depth-resolved reading of an investigation point: cone resistance q_c [Pa] for a CPT, blow count N for an SPT, or the sample value of a borehole.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InvestigationReading {
    pub id: String,
    /// ⬇️ Depth [m] below the ground level of the reading.
    #[dsl(unit = "m")]
    pub depth: f64,
    /// 📈 Measured value at the depth: q_c [Pa] (CPT), N (SPT) or the sample value (borehole).
    pub measurement: f64,
}

/// 🕳️ Ground investigation point of EN 1997-2 (borehole, cone penetration test or standard penetration test) with its position, ground level, depth and depth-resolved readings (GI-01, GI-03).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Investigation {
    pub id: String,
    /// ↔️ Easting x [m] of the investigation point in the site frame.
    #[dsl(unit = "m")]
    pub position_x: f64,
    /// ↕️ Northing y [m] of the investigation point in the site frame.
    #[dsl(unit = "m")]
    pub position_y: f64,
    /// ⬆️ Ground level z [m] at the investigation point.
    #[dsl(unit = "m")]
    pub ground_level: f64,
    /// ⬇️ Depth [m] below the ground level reached by the investigation.
    #[dsl(unit = "m")]
    pub depth: f64,
    /// 🕳️ borehole | cpt | spt.
    pub kind: InvestigationKind,
    #[dsl(table)]
    pub readings: Vec<InvestigationReading>,
}

/// 🧮 Typed basis of one characteristic soil parameter of a layer: the declared value of the layer or the value derived from the ground investigations (GI-03, GI-13). A parameter without a row keeps its declared value; nothing is overridden silently.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CharacteristicBasis {
    pub id: String,
    /// 🪨 Id of the soil layer whose parameter this row governs.
    pub layer_id: String,
    /// 🎯 undrainedStrength | frictionAngle.
    pub parameter: CharacteristicParameter,
    /// 🧮 declared | derived.
    pub basis: ParameterBasis,
}

/// 📍️ Persisted pose of one scene entity (`id` is the entity id; SI: m, rad). Entities without a placement follow the documented default layout.
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

#[path = "../../🚪️io/🪶️sqlite/🦀️.rs"]
pub mod sqlite;

#[path = "../../🚪️io/🪞️records/🦀️.rs"]
mod records;

impl SituationChoice {
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Persistent => "persistent",
            Self::Transient => "transient",
            Self::Accidental => "accidental",
        }
    }
}

impl std::fmt::Display for SituationChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ApproachChoice {
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Da1Str => "da1str",
            Self::Da1Geo => "da1geo",
            Self::Da2 => "da2",
            Self::Da2Star => "da2star",
            Self::Da3 => "da3",
            Self::Geo3 => "geo3",
        }
    }
}

impl std::fmt::Display for ApproachChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl SoilType {
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sand => "sand",
            Self::Gravel => "gravel",
            Self::Clay => "clay",
            Self::Silt => "silt",
            Self::Fill => "fill",
        }
    }
}

impl std::fmt::Display for SoilType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PileType {
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bored => "bored",
            Self::Driven => "driven",
            Self::Cfa => "cfa",
        }
    }
}

impl std::fmt::Display for PileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl EarthPressureMode {
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::AtRest => "atRest",
            Self::IncreasedActive => "increasedActive",
        }
    }
}

impl std::fmt::Display for EarthPressureMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl InvestigationKind {
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Borehole => "borehole",
            Self::Cpt => "cpt",
            Self::Spt => "spt",
        }
    }
}

impl std::fmt::Display for InvestigationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl WallMovement {
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Free => "free",
            Self::Rigid => "rigid",
            Self::Propped => "propped",
        }
    }
}

impl std::fmt::Display for WallMovement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TendonType {
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Strand => "strand",
            Self::Bar => "bar",
        }
    }
}

impl std::fmt::Display for TendonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl EmbeddedWallKind {
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreeEarth => "freeEarth",
            Self::Cantilever => "cantilever",
        }
    }
}

impl std::fmt::Display for EmbeddedWallKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ParameterBasis {
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Declared => "declared",
            Self::Derived => "derived",
        }
    }
}

impl std::fmt::Display for ParameterBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl CharacteristicParameter {
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UndrainedStrength => "undrainedStrength",
            Self::FrictionAngle => "frictionAngle",
        }
    }
}

impl std::fmt::Display for CharacteristicParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl LabTestKind {
    /// 🔤 Camel-case wire token of the variant, for the string-keyed registry and case parameters.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Oedometer => "oedometer",
            Self::Triaxial => "triaxial",
            Self::DirectShear => "directShear",
        }
    }
}

impl std::fmt::Display for LabTestKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
