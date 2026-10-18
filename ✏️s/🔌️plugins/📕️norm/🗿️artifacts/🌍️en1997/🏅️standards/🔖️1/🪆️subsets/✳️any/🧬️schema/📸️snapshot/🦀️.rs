//! 🌍️ EN 1997 snapshot — complete geotechnical project subject (SI).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1997 geotechnical design package.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1997", layout = "lines")]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Snapshot {
    #[state(artifact)]
    pub structure_id: String,
    #[state(artifact)]
    pub geotechnical_category: u8,
    #[state(artifact)]
    pub design_situation: String,
    #[state(artifact)]
    pub design_approach: String,
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
}
//#endregion 🔖️Snapshot

impl Default for En1997Snapshot {
    fn default() -> Self {
        compliant_demo()
    }
}

/// 🏗 Realistic compliant DE / DA2 geotechnical package (spread footing + bored pile + wall + slope).
pub fn compliant_demo() -> En1997Snapshot {
    use crate::{FoundationLoadCase, PileTestProfile};
    En1997Snapshot {
        structure_id: "geo-demo-compliant".into(),
        geotechnical_category: 2,
        design_situation: "bsP".into(),
        design_approach: "da2".into(),
        annex: AnnexChoice::De,
        groundwater_level: 4.0,
        investigation_depth: 12.0,
        layers: vec![
            SoilLayer {
                id: "layer-sand".into(),
                soil_type: "sand".into(),
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
            },
            SoilLayer {
                id: "layer-gravel".into(),
                soil_type: "gravel".into(),
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
                design_situation: "".into(),
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
            pile_type: "bored".into(),
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
            earth_pressure_mode: "active".into(),
            wall_movement: "free".into(),
            ocr: 1.0,
            concrete_gamma: 25_000.0,
            surcharge: 10_000.0,
            vertical_permanent: 320_000.0,
            horizontal_permanent: 40_000.0,
        }],
        slopes: vec![Slope {
            id: "slope-S1".into(),
            angle_deg: 25.0,
            height: 6.0,
            length: 14.0,
            governing_layer_id: "layer-sand".into(),
        }],
        uplift_cases: vec![UpliftCase {
            id: "upl-U1".into(),
            permanent_stabilizing: 900_000.0,
            permanent_destabilizing: 500_000.0,
            variable_destabilizing: 50_000.0,
            pore_pressure: 40_000.0,
            total_stress: 120_000.0,
        }],
    }
}

/// 🚨 Non-compliant package with multiple GEO/UPL failures for remedy tests.
pub fn noncompliant_demo() -> En1997Snapshot {
    use crate::PileTestProfile;
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
    s
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;

pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1997", "EN 1997", EN1997_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1997_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1997", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const EN1997_DOCUMENT_SCHEMA: &str = "semio.norm.en1997/v1";

/// 🪨 Stratigraphic soil layer with characteristic parameters (SI).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SoilLayer {
    pub id: String,
    pub soil_type: String,
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
}

/// 🏋️ Characteristic permanent/variable actions for one design situation.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct FoundationLoadCase {
    pub id: String,
    pub design_situation: String,
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
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PileTestProfile {
    pub id: String,
    #[dsl(unit = "N")]
    pub shaft_resistance: f64,
    #[dsl(unit = "N")]
    pub base_resistance: f64,
}

/// 🏗️ Deep foundation pile or pile group.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Pile {
    pub id: String,
    pub pile_type: String,
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
    #[dsl(table)]
    pub test_profiles: Vec<PileTestProfile>,
}

/// 🧱 Retaining wall geometry and surcharge.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
    pub earth_pressure_mode: String,
    /// 🧱 free | rigid | propped (DIN 4085 wall-movement class).
    pub wall_movement: String,
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
}

/// ⛰️ Slope geometry for overall stability (GEO-3).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Slope {
    pub id: String,
    pub angle_deg: f64,
    #[dsl(unit = "m")]
    pub height: f64,
    #[dsl(unit = "m")]
    pub length: f64,
    pub governing_layer_id: String,
}

/// 💧 Uplift / heave verification case (§10 UPL/HYD).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
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
