//! 🌍️ EN 1997 snapshot — complete geotechnical project subject (SI).

use crate::document::AnnexChoice;
use crate::{Pile, RetainingWall, Slope, SoilLayer, SpreadFoundation, UpliftCase};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1997 geotechnical design package.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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

//#region 🔖️HandcraftedArtifactCodecs
crate::impl_norm_artifact_record!(En1997Snapshot, extension = "en1997", envelope_id = "norm.en1997");
//#endregion 🔖️HandcraftedArtifactCodecs

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

//#region 🌉️ExternalCodecBridge
pub fn encode_en1997_snapshot_json(snapshot: &En1997Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}
pub fn decode_en1997_snapshot_json(text: &str) -> Result<En1997Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}
pub fn decode_en1997_dsl(text: &str) -> Result<En1997Snapshot, String> {
    <En1997Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}
pub fn encode_en1997_dsl(snapshot: &En1997Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
pub fn decode_en1997_pack(bytes: &[u8]) -> Result<En1997Snapshot, String> {
    <En1997Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
pub fn encode_en1997_pack(snapshot: &En1997Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge



#[cfg(test)]
mod regen_assets {
    use super::{compliant_demo, encode_en1997_dsl, encode_en1997_pack, noncompliant_demo};

    #[test]
    fn regen_example_assets_when_env_set() {
        if std::env::var("SEMIO_REGEN_EN1997_ASSETS").ok().as_deref() != Some("1") {
            return;
        }
        let assets = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");
        assert!(assets.is_dir(), "assets dir missing: {}", assets.display());
        for (name, snap) in [
            ("🎬️demo", compliant_demo()),
            ("🏗compliant", compliant_demo()),
            ("🚨noncompliant", noncompliant_demo()),
        ] {
            let dir = assets.join(name);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join("🗣️.dsl.semio"), encode_en1997_dsl(&snap)).unwrap();
            std::fs::write(dir.join("📦️.pack.semio"), encode_en1997_pack(&snap)).unwrap();
            eprintln!("[DEBUG] wrote {}", dir.display());
        }
    }
}
