//! 🧬️ En1994 artifact schema — every field of the artifact with its state class.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact

/// 🧬️ Full En1994 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub structure_kind: String,
    #[state(artifact)]
    pub steel_f_y_pa: f64,
    #[state(artifact)]
    pub beams: Vec<CompositeBeam>,
    #[state(artifact)]
    pub columns: Vec<CompositeColumn>,
    #[state(artifact)]
    pub slabs: Vec<CompositeSlab>,
    #[state(artifact)]
    pub fire_rating: String,
    #[state(artifact)]
    pub insulation_thickness_m: f64,
    #[state(artifact)]
    pub fatigue_detail: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions

impl En1994Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> semio_s_artifact_norm_en1994_model::En1994Snapshot {
        semio_s_artifact_norm_en1994_model::En1994Snapshot {
            annex: self.annex,
            structure_kind: self.structure_kind.clone(),
            steel_f_y_pa: self.steel_f_y_pa,
            beams: self.beams.clone(),
            columns: self.columns.clone(),
            slabs: self.slabs.clone(),
            fire_rating: self.fire_rating.clone(),
            insulation_thickness_m: self.insulation_thickness_m,
            fatigue_detail: self.fatigue_detail.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: semio_s_artifact_norm_en1994_model::En1994Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            structure_kind: snapshot.structure_kind,
            steel_f_y_pa: snapshot.steel_f_y_pa,
            beams: snapshot.beams,
            columns: snapshot.columns,
            slabs: snapshot.slabs,
            fire_rating: snapshot.fire_rating,
            insulation_thickness_m: snapshot.insulation_thickness_m,
            fatigue_detail: snapshot.fatigue_detail,
        }
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1994` — twenty handcrafted schema leaves.
pub fn en1994_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.en1994",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_en1994_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_en1994_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_en1994_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

