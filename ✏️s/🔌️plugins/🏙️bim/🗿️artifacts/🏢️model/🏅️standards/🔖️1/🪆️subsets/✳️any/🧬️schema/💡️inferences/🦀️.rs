//! 💡️ BIM model inference schema: everything derivable from a [`ModelSnapshot`]. One field per named inference under
//! `💡️inferences/`; the snapshot stores authored parameters only, so elevations, wall heights and quantities live here.

use crate::ModelSnapshot;
use schema::ArtifactSchema;
use std::collections::BTreeMap;

use super::storey_levels::StoreyLevel;
use super::quantities::ModelQuantities;
use super::spaces::SpaceRoom;
use super::stair_runs::StairRun;
use super::element_solids::ElementSolid;
use super::wall_layout::WallLayout;
use super::curtain_layout::CurtainLayout;
use super::opening_frames::OpeningFrame;

use super::diagnostics::Diagnostic;
use super::plan_linework::PlanLinework;

//#region 🔖️Inference
/// 💡️ Everything inferable from a model snapshot, keyed by element id.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[artifact_schema(id = "s.bim.model.inference")]
pub struct ModelInference {
    #[derived]
    pub storey_levels: BTreeMap<String, StoreyLevel>,
    #[derived]
    pub wall_layout: BTreeMap<String, WallLayout>,
    #[derived]
    pub curtain_layout: BTreeMap<String, CurtainLayout>,
    #[derived]
    pub stair_runs: BTreeMap<String, StairRun>,
    #[derived]
    pub spaces: BTreeMap<String, SpaceRoom>,
    #[derived]
    pub opening_frames: BTreeMap<String, OpeningFrame>,
    #[derived]
    pub element_solids: BTreeMap<String, ElementSolid>,
    #[derived]
    pub plan_linework: BTreeMap<String, PlanLinework>,
    #[derived]
    pub diagnostics: Vec<Diagnostic>,
    #[derived]
    pub quantities: ModelQuantities,
}

impl protocol::Inference<ModelSnapshot> for ModelInference {
    fn infer(snapshot: &ModelSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        Ok(super::model_graph::infer_selected::<{ super::model_graph::kinds::ALL }>(snapshot))
    }
}

impl protocol::InferenceSpec<ModelSnapshot> for ModelInference {
    fn inference_schema_id() -> &'static str {
        "s.bim.model.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.storey-levels", reads: &["storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.wall-layout", reads: &["walls", "wall_types", "storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.curtain-layout", reads: &["curtain_walls", "storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.stair-runs", reads: &["stairs", "storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.quantities", reads: super::quantities::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.spaces", reads: &["spaces", "walls", "wall_types", "curtain_walls", "columns", "column_types", "slabs", "slab_types", "storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.opening-frames", reads: &["openings", "walls", "curtain_walls", "wall_types", "window_types", "door_types", "storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.element-solids", reads: super::element_solids::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.plan-linework", reads: super::plan_linework::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.diagnostics", reads: super::diagnostics::READS },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ The inference descriptor stitched into the artifact declaration: its facet leaves in every language.
pub fn bim_model_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.bim.model.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
