//! 🌍️ EN 1997 artifact schema — geotechnical project subject + clause helpers.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1997_model::{En1997Snapshot, FoundationLoadCase, Pile, RetainingWall, Slope, SoilLayer, SpreadFoundation, UpliftCase};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ EN 1997 document artifact state (mirrors snapshot).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1997")]
pub struct En1997Artifact {
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
    #[state(artifact)]
    pub groundwater_level: f64,
    #[state(artifact)]
    pub investigation_depth: f64,
    #[state(artifact)]
    pub layers: Vec<SoilLayer>,
    #[state(artifact)]
    pub footings: Vec<SpreadFoundation>,
    #[state(artifact)]
    pub piles: Vec<Pile>,
    #[state(artifact)]
    pub retaining_walls: Vec<RetainingWall>,
    #[state(artifact)]
    pub slopes: Vec<Slope>,
    #[state(artifact)]
    pub uplift_cases: Vec<UpliftCase>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1997Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1997Snapshot::default())
    }
}

impl From<En1997Snapshot> for En1997Artifact {
    fn from(snapshot: En1997Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1997Artifact {
    pub fn to_snapshot(&self) -> En1997Snapshot {
        En1997Snapshot {
            structure_id: self.structure_id.clone(),
            geotechnical_category: self.geotechnical_category,
            design_situation: self.design_situation.clone(),
            design_approach: self.design_approach.clone(),
            annex: self.annex,
            groundwater_level: self.groundwater_level,
            investigation_depth: self.investigation_depth,
            layers: self.layers.clone(),
            footings: self.footings.clone(),
            piles: self.piles.clone(),
            retaining_walls: self.retaining_walls.clone(),
            slopes: self.slopes.clone(),
            uplift_cases: self.uplift_cases.clone(),
        }
    }

    pub fn from_snapshot(snapshot: En1997Snapshot) -> Self {
        Self {
            structure_id: snapshot.structure_id,
            geotechnical_category: snapshot.geotechnical_category,
            design_situation: snapshot.design_situation,
            design_approach: snapshot.design_approach,
            annex: snapshot.annex,
            groundwater_level: snapshot.groundwater_level,
            investigation_depth: snapshot.investigation_depth,
            layers: snapshot.layers,
            footings: snapshot.footings,
            piles: snapshot.piles,
            retaining_walls: snapshot.retaining_walls,
            slopes: snapshot.slopes,
            uplift_cases: snapshot.uplift_cases,
        }
    }

}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1997_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.en1997",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_en1997_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_en1997_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_en1997_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

