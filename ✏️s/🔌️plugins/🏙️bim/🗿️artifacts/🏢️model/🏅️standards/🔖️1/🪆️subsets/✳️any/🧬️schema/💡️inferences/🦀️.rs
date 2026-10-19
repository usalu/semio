//! 💡️ BIM model inference schema: everything derivable from a [`ModelSnapshot`]. One field per named inference under
//! `💡️inferences/`; the snapshot stores authored parameters only, so elevations, wall heights and quantities live here.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use schema::ArtifactSchema;
use std::collections::BTreeMap;

use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::storey_levels::StoreyLevel;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility::PhaseVisibility;
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::ModelQuantities;
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::{SchemeTotals, ZoneTotals};
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules::ScheduleTable;
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::spaces::SpaceRoom;
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::stair_runs::StairRun;
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::ramp_runs::RampRun;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::wall_layout::WallLayout;
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::inferences::curtain_layout::CurtainLayout;
use semio_s_artifact_bim_model_wall_layout::standards::v1::subsets::any::schema::inferences::opening_frames::OpeningFrame;

use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::{Diagnostic, DiagnosticIndex};
use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework::PlanLinework;
use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework;
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout::StoreyAnnotations;
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components::ComponentValue;
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::FamilyValue;
use semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep::MepValue;
use semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties::EffectiveProperties;
use semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::{EnergyTotals, EnvelopeSpace};
use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout::SheetLayout;
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets::ClashSetResult;
use semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results::RuleResult;

//#region 🔖️Inference
/// 💡️ Everything inferable from a model snapshot, keyed by element id.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[artifact_schema(id = "s.bim.model.inference")]
pub struct ModelInference {
    #[derived]
    pub analytical_members: BTreeMap<String, semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::analytical_members::AnalyticalMember>,
    #[derived]
    pub structural_analysis: semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::analytical_members::StructuralAnalysis,
    #[derived]
    pub option_scope: semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::option_scope::OptionScope,
    #[derived]
    pub storey_levels: BTreeMap<String, StoreyLevel>,
    #[derived]
    pub wall_layout: BTreeMap<String, WallLayout>,
    #[derived]
    pub curtain_layout: BTreeMap<String, CurtainLayout>,
    #[derived]
    pub stair_runs: BTreeMap<String, StairRun>,
    #[derived]
    pub ramp_runs: BTreeMap<String, RampRun>,
    #[derived]
    pub families: BTreeMap<String, FamilyValue>,
    #[derived]
    pub components: BTreeMap<String, ComponentValue>,
    #[derived]
    pub mep: BTreeMap<String, MepValue>,
    #[derived]
    pub spaces: BTreeMap<String, SpaceRoom>,
    #[derived]
    pub opening_frames: BTreeMap<String, OpeningFrame>,
    #[derived]
    pub element_solids: BTreeMap<String, ElementSolid>,
    #[derived]
    pub plan_linework: BTreeMap<String, PlanLinework>,
    #[derived]
    pub view_linework: BTreeMap<String, ViewLinework>,
    #[derived]
    pub sheet_layouts: BTreeMap<String, SheetLayout>,
    #[derived]
    pub diagnostics: Vec<Diagnostic>,
    #[derived]
    pub diagnostic_index: DiagnosticIndex,
    #[derived]
    pub quantities: ModelQuantities,
    #[derived]
    pub phase_visibility: BTreeMap<String, PhaseVisibility>,
    #[derived]
    pub zone_totals: BTreeMap<String, ZoneTotals>,
    #[derived]
    pub scheme_totals: BTreeMap<String, SchemeTotals>,
    #[derived]
    pub schedules: BTreeMap<String, ScheduleTable>,
    #[derived]
    pub annotations: BTreeMap<String, StoreyAnnotations>,
    #[derived]
    pub effective_properties: BTreeMap<String, EffectiveProperties>,
    #[derived]
    pub clash_sets: BTreeMap<String, ClashSetResult>,
    #[derived]
    pub rule_results: BTreeMap<String, RuleResult>,
    #[derived]
    pub energy_envelopes: BTreeMap<String, EnvelopeSpace>,
    #[derived]
    pub energy_totals: BTreeMap<String, EnergyTotals>,
}

impl protocol::Inference<ModelSnapshot> for ModelInference {
    fn infer(snapshot: &ModelSnapshot) -> Result<Self, semio_framework_value::ValueError> {
        crate::standards::v1::subsets::any::schema::inferences::model_graph::try_infer_selected::<{ crate::standards::v1::subsets::any::schema::inferences::model_graph::kinds::ALL }>(snapshot).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
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
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.analytical-members", reads: semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::analytical_members::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.option-scope", reads: semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::option_scope::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.storey-levels", reads: &["storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.wall-layout", reads: &["walls", "wall_types", "storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.curtain-layout", reads: &["curtain_walls", "curtain_wall_types", "curtain_panel_overrides", "storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.stair-runs", reads: &["stairs", "storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.ramp-runs", reads: &["ramps", "storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.families", reads: semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::families::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.components", reads: semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::components::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.mep", reads: semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::mep::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.quantities", reads: semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::quantities::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.phase-visibility", reads: semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::inferences::phase_visibility::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.spaces", reads: &["spaces", "walls", "wall_types", "curtain_walls", "curtain_wall_types", "columns", "column_types", "slabs", "slab_types", "storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.opening-frames", reads: &["openings", "walls", "curtain_walls", "curtain_wall_types", "wall_types", "window_types", "door_types", "storeys", "buildings", "sites"] },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.element-solids", reads: semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::inferences::element_solids::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.plan-linework", reads: semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::plan_linework::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.view-linework", reads: semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::view_linework::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.sheet-layout", reads: semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::sheet_layout::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.clash-sets", reads: semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::inferences::clash_sets::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.rule-results", reads: semio_s_artifact_bim_model_plan_linework::standards::v1::subsets::any::schema::inferences::rule_results::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.diagnostics", reads: semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.diagnostic-index", reads: semio_s_artifact_bim_model_components::standards::v1::subsets::any::schema::inferences::diagnostics::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.annotation-layout", reads: semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::annotation_layout::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.schedules", reads: semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::schedules::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.effective-properties", reads: semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::effective_properties::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.zones", reads: semio_s_artifact_bim_model_quantities::standards::v1::subsets::any::schema::inferences::zones::READS },
            protocol::InferenceFieldSpec { id: "s.bim.model.inference.energy-envelope", reads: semio_s_artifact_bim_model_spaces::standards::v1::subsets::any::schema::inferences::energy_envelope::READS },
        ]
    }
}
//#endregion 🔖️Inference

//#region 🔖️Descriptor
/// 💡️ The inference descriptor stitched into the artifact declaration: its facet leaves in every language.
pub fn bim_model_inference_descriptor() -> semio_framework_schema_registry::ArtifactInferenceDescriptor {
    ::semio_framework_schema_registry::register_schema_leaf_documents(&[
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::JsonSchema, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔣️.json" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model.inference", export: "inference", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🛰️.proto" },
    ]);
    #[cfg(target_arch = "wasm32")]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, "");
    #[cfg(not(target_arch = "wasm32"))]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, env!("CARGO_MANIFEST_DIR"));
    semio_framework_schema_registry::ArtifactInferenceDescriptor {
        id: "s.bim.model.inference",
        inference: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: "", graphql: "", json_schema: "", proto: "" },
    }
}
//#endregion 🔖️Descriptor


#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🧰️schedule-kit/🦀️.rs"]
pub mod schedule_kit;

