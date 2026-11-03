//! 🧬️ EN 1998 artifact schema — mirrors the snapshot subject.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_en1998_model::{En1998Accelerogram, En1998Assessment, En1998Bearing, En1998Bridge, En1998Building, En1998Diaphragm, En1998Foundation, En1998Joint, En1998Member, En1998Pier, En1998Placement, En1998RetainingWall, En1998Silo, En1998Site, En1998Slope, En1998Snapshot, En1998SoilLayer, En1998Tank, En1998TieBeam, En1998Tower, MemberKind};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1998")]
pub struct En1998Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub site: En1998Site,
    #[state(artifact)]
    pub buildings: Vec<En1998Building>,
    #[state(artifact)]
    pub bridges: Vec<En1998Bridge>,
    #[state(artifact)]
    pub assessments: Vec<En1998Assessment>,
    #[state(artifact)]
    pub silos: Vec<En1998Silo>,
    #[state(artifact)]
    pub tanks: Vec<En1998Tank>,
    #[state(artifact)]
    pub foundations: Vec<En1998Foundation>,
    #[state(artifact)]
    pub retaining_walls: Vec<En1998RetainingWall>,
    #[state(artifact)]
    pub towers: Vec<En1998Tower>,
    #[state(artifact)]
    pub soil_layers: Vec<En1998SoilLayer>,
    #[state(artifact)]
    pub diaphragms: Vec<En1998Diaphragm>,
    #[state(artifact)]
    pub accelerograms: Vec<En1998Accelerogram>,
    #[state(artifact)]
    pub piers: Vec<En1998Pier>,
    #[state(artifact)]
    pub bearings: Vec<En1998Bearing>,
    #[state(artifact)]
    pub joints: Vec<En1998Joint>,
    #[state(artifact)]
    pub placements: Vec<En1998Placement>,
    #[state(artifact)]
    pub tie_beams: Vec<En1998TieBeam>,
    #[state(artifact)]
    pub slopes: Vec<En1998Slope>,
}
//#endregion 🔖️Artifact

impl En1998Artifact {
    pub fn to_snapshot(&self) -> semio_s_artifact_norm_en1998_model::En1998Snapshot {
        semio_s_artifact_norm_en1998_model::En1998Snapshot {
            annex: self.annex.clone(),
            site: self.site.clone(),
            buildings: self.buildings.clone(),
            bridges: self.bridges.clone(),
            assessments: self.assessments.clone(),
            silos: self.silos.clone(),
            tanks: self.tanks.clone(),
            foundations: self.foundations.clone(),
            retaining_walls: self.retaining_walls.clone(),
            towers: self.towers.clone(),
            soil_layers: self.soil_layers.clone(),
            diaphragms: self.diaphragms.clone(),
            accelerograms: self.accelerograms.clone(),
            piers: self.piers.clone(),
            bearings: self.bearings.clone(),
            joints: self.joints.clone(),
            tie_beams: self.tie_beams.clone(),
            slopes: self.slopes.clone(),
            placements: self.placements.clone(),
        }
    }

    pub fn from_snapshot(snapshot: semio_s_artifact_norm_en1998_model::En1998Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            site: snapshot.site,
            buildings: snapshot.buildings,
            bridges: snapshot.bridges,
            assessments: snapshot.assessments,
            silos: snapshot.silos,
            tanks: snapshot.tanks,
            foundations: snapshot.foundations,
            retaining_walls: snapshot.retaining_walls,
            towers: snapshot.towers,
            soil_layers: snapshot.soil_layers,
            diaphragms: snapshot.diaphragms,
            accelerograms: snapshot.accelerograms,
            piers: snapshot.piers,
            bearings: snapshot.bearings,
            joints: snapshot.joints,
            tie_beams: snapshot.tie_beams,
            slopes: snapshot.slopes,
            placements: snapshot.placements,
        }
    }

}

impl Default for En1998Artifact {
    fn default() -> Self {
        Self::from_snapshot(semio_s_artifact_norm_en1998_model::En1998Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))
    }
}

// Re-export formula modules (kept below for evaluate).

use semio_s_plugin_norm_compliance::AnnexChoice;

#[cfg(any(test, feature = "oracle-wiring"))]
#[path = "🧪️tests/🎯️oracle-wiring/🦀️.rs"]
pub mod oracle_wiring;

#[path = "🧱️horizontal/🦀️.rs"]
pub mod horizontal;

#[path = "🌉️bridge/🦀️.rs"]
pub mod bridge;

#[path = "🛢️tank/🦀️.rs"]
pub mod tank;

#[path = "🗼️tower/🦀️.rs"]
pub mod tower;

#[path = "🧱️wall-plan/🦀️.rs"]
pub mod wall_plan;

#[path = "🔩️joints/🦀️.rs"]
pub mod joints;

#[path = "🪨️cracked-stiffness/🦀️.rs"]
pub mod cracked_stiffness;

pub use semio_s_artifact_norm_en1998_schema_formulas::{isolation, liquefaction_hazard, na_de, part_1, part_2, part_3, part_3_state, part_4, part_5, part_6, soil_structure};

/// 🇪️🇺️ Resolved seismic-action model.
#[derive(Clone, Debug, PartialEq)]
pub enum AnnexParams {
    De { s_ap_r: f64, combo: na_de::GroundCombo },
    En { a_gr: f64, ground: char, spectrum_type1: bool },
}

impl AnnexParams {
    pub fn choice(&self) -> AnnexChoice {
        match self { Self::De { .. } => AnnexChoice::De, Self::En { .. } => AnnexChoice::En }
    }

    pub fn ground_params(&self) -> (f64, f64, f64, f64, f64) {
        match self {
            Self::De { s_ap_r, combo } => {
                let (s, tb, tc, td) = combo.spectrum_params();
                (na_de::a_gr_from_s_ap_r(*s_ap_r), s, tb, tc, td)
            }
            Self::En { a_gr, ground, spectrum_type1 } => {
                let (s, tb, tc, td) = en_ground_params(*ground, *spectrum_type1);
                (*a_gr, s, tb, tc, td)
            }
        }
    }

    /// 🔖️ Registry ids of the four spectrum parameters (S, T_B, T_C, T_D) this model reads.
    pub fn ndp_ids(&self) -> Vec<String> {
        let prefix = match self {
            Self::De { combo, .. } => format!("ndp.en1998.na4.{}", combo.key()),
            Self::En { ground, spectrum_type1, .. } => format!("ndp.en1998.{}.{}", if *spectrum_type1 { "t32" } else { "t33" }, en_ground_class(*ground)),
        };
        ["s", "tb", "tc", "td"].iter().map(|symbol| format!("{prefix}.{symbol}")).collect()
    }

    pub fn elastic_response_spectrum(&self, t: f64) -> f64 {
        let (a_g, s, tb, tc, td) = self.ground_params();
        part_1::elastic_response_spectrum(a_g, s, tb, tc, td, t)
    }
}

fn en_ground_class(ground: char) -> char {
    match ground { 'A' => 'a', 'C' => 'c', 'D' => 'd', 'E' => 'e', _ => 'b' }
}

fn en_ground_params(ground: char, type1: bool) -> (f64, f64, f64, f64) {
    let class = en_ground_class(ground);
    let table = if type1 { "t32" } else { "t33" };
    let n = |symbol: &str| semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1998.{table}.{class}.{symbol}"), AnnexChoice::En);
    (n("s"), n("tb"), n("tc"), n("td"))
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1998` — handcrafted schema facet leaves.
pub fn en1998_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.en1998",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_en1998_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_en1998_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_en1998_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

