//! 🧬️ EN1991 diff schema — sparse scalar fields plus positional row deltas for every list the document owns.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;
use protocol::{ColdDiffAlgebra, DiffAlgebra, MutationDiff};

use semio_s_artifact_norm_en1991_model::En1991Snapshot;

//#region 🔖️Rows
/// 🔑️ Key extractor of the foreign `FloorArea` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FloorAreaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::FloorArea> for FloorAreaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::FloorArea) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `SelfWeightElement` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelfWeightElementKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::SelfWeightElement> for SelfWeightElementKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::SelfWeightElement) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `RoofArea` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofAreaKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::RoofArea> for RoofAreaKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::RoofArea) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `WindFace` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindFaceKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::WindFace> for WindFaceKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::WindFace) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `AccidentalCase` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccidentalCaseKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::AccidentalCase> for AccidentalCaseKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::AccidentalCase) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `BridgeLane` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BridgeLaneKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::BridgeLane> for BridgeLaneKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::BridgeLane) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `ConstructionStage` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConstructionStageKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::ConstructionStage> for ConstructionStageKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::ConstructionStage) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `RoofSurface` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofSurfaceKey;

/// 🔑️ Key extractor of the foreign `Placement` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlacementKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::RoofSurface> for RoofSurfaceKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::RoofSurface) -> String {
        row.id.clone()
    }
}

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::Placement> for PlacementKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::Placement) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `SpecialVehicle` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpecialVehicleKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::SpecialVehicle> for SpecialVehicleKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::SpecialVehicle) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `WindBridge` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindBridgeKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::WindBridge> for WindBridgeKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::WindBridge) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `DynamicElement` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DynamicElementKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::DynamicElement> for DynamicElementKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::DynamicElement) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `ImpactAction` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImpactActionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::ImpactAction> for ImpactActionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::ImpactAction) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `InternalExplosion` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InternalExplosionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::InternalExplosion> for InternalExplosionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::InternalExplosion) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `ThermalSection` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThermalSectionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::ThermalSection> for ThermalSectionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::ThermalSection) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `HelicopterLanding` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HelicopterLandingKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::HelicopterLanding> for HelicopterLandingKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::HelicopterLanding) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `TieRun` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TieRunKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::TieRun> for TieRunKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::TieRun) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `ConstructionLoad` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConstructionLoadKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::ConstructionLoad> for ConstructionLoadKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::ConstructionLoad) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `FallingLoad` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FallingLoadKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::FallingLoad> for FallingLoadKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::FallingLoad) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `ExecutionImperfection` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExecutionImperfectionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::ExecutionImperfection> for ExecutionImperfectionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::ExecutionImperfection) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `DerailmentCase` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DerailmentCaseKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::DerailmentCase> for DerailmentCaseKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::DerailmentCase) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `CraneHoist` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CraneHoistKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::CraneHoist> for CraneHoistKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::CraneHoist) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `Footbridge` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FootbridgeKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::Footbridge> for FootbridgeKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::Footbridge) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `RailDynamic` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RailDynamicKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::RailDynamic> for RailDynamicKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::RailDynamic) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `LoadDispersal` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoadDispersalKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::LoadDispersal> for LoadDispersalKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::LoadDispersal) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `BulkSolid` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BulkSolidKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::BulkSolid> for BulkSolidKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::BulkSolid) -> String {
        row.id.clone()
    }
}

/// 🔑️ Key extractor of the foreign `RotatingMachine` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RotatingMachineKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::RotatingMachine> for RotatingMachineKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::RotatingMachine) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FloorArea`.
    pub En1991FloorPatch of semio_s_artifact_norm_en1991_model::FloorArea { set { category: String, area: f64, assumed_qk: f64, assumed_qk_concentrated: f64, assumed_partitions: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `FloorArea` list.
    pub En1991FloorDelta { removal: En1991FloorRemoval, insertion: En1991FloorInsertion, relocation: En1991FloorRelocation, modification: En1991FloorModification, row: semio_s_artifact_norm_en1991_model::FloorArea, patch: En1991FloorPatch, list: Vec<semio_s_artifact_norm_en1991_model::FloorArea>, key: String = by FloorAreaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `SelfWeightElement`.
    pub En1991SelfWeightElementPatch of semio_s_artifact_norm_en1991_model::SelfWeightElement { set { material: String, thickness: f64, assumed_gk: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `SelfWeightElement` list.
    pub En1991SelfWeightElementDelta { removal: En1991SelfWeightElementRemoval, insertion: En1991SelfWeightElementInsertion, relocation: En1991SelfWeightElementRelocation, modification: En1991SelfWeightElementModification, row: semio_s_artifact_norm_en1991_model::SelfWeightElement, patch: En1991SelfWeightElementPatch, list: Vec<semio_s_artifact_norm_en1991_model::SelfWeightElement>, key: String = by SelfWeightElementKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `RoofArea`.
    pub En1991RoofPatch of semio_s_artifact_norm_en1991_model::RoofArea { set { roof_type: String, pitch_deg: f64, c_e: f64, c_t: f64, has_parapet: bool, parapet_height: f64, drift_obstruction_height: f64, multi_span: bool, assumed_sk: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `RoofArea` list.
    pub En1991RoofDelta { removal: En1991RoofRemoval, insertion: En1991RoofInsertion, relocation: En1991RoofRelocation, modification: En1991RoofModification, row: semio_s_artifact_norm_en1991_model::RoofArea, patch: En1991RoofPatch, list: Vec<semio_s_artifact_norm_en1991_model::RoofArea>, key: String = by RoofAreaKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WindFace`.
    pub En1991WindFacePatch of semio_s_artifact_norm_en1991_model::WindFace { set { zone: String, z: f64, c_pe10: f64, c_pe1: f64, c_pi: f64, c_s: f64, c_d: f64, loaded_area: f64, assumed_wp: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `WindFace` list.
    pub En1991WindFaceDelta { removal: En1991WindFaceRemoval, insertion: En1991WindFaceInsertion, relocation: En1991WindFaceRelocation, modification: En1991WindFaceModification, row: semio_s_artifact_norm_en1991_model::WindFace, patch: En1991WindFacePatch, list: Vec<semio_s_artifact_norm_en1991_model::WindFace>, key: String = by WindFaceKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `AccidentalCase`.
    pub En1991AccidentalCasePatch of semio_s_artifact_norm_en1991_model::AccidentalCase { set { impact: Vec<semio_s_artifact_norm_en1991_model::AccidentalImpact>, explosion: Vec<semio_s_artifact_norm_en1991_model::AccidentalExplosion> } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `AccidentalCase` list.
    pub En1991AccidentalCaseDelta { removal: En1991AccidentalCaseRemoval, insertion: En1991AccidentalCaseInsertion, relocation: En1991AccidentalCaseRelocation, modification: En1991AccidentalCaseModification, row: semio_s_artifact_norm_en1991_model::AccidentalCase, patch: En1991AccidentalCasePatch, list: Vec<semio_s_artifact_norm_en1991_model::AccidentalCase>, key: String = by AccidentalCaseKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `BridgeLane`.
    pub En1991BridgeLanePatch of semio_s_artifact_norm_en1991_model::BridgeLane { set { width: f64, offset: f64, lm1_lane: u8 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `BridgeLane` list.
    pub En1991BridgeLaneDelta { removal: En1991BridgeLaneRemoval, insertion: En1991BridgeLaneInsertion, relocation: En1991BridgeLaneRelocation, modification: En1991BridgeLaneModification, row: semio_s_artifact_norm_en1991_model::BridgeLane, patch: En1991BridgeLanePatch, list: Vec<semio_s_artifact_norm_en1991_model::BridgeLane>, key: String = by BridgeLaneKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ConstructionStage`.
    pub En1991ConstructionStagePatch of semio_s_artifact_norm_en1991_model::ConstructionStage { set { activity: String, duration_days: f64, return_years: f64, assumed_qk: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `ConstructionStage` list.
    pub En1991ConstructionStageDelta { removal: En1991ConstructionStageRemoval, insertion: En1991ConstructionStageInsertion, relocation: En1991ConstructionStageRelocation, modification: En1991ConstructionStageModification, row: semio_s_artifact_norm_en1991_model::ConstructionStage, patch: En1991ConstructionStagePatch, list: Vec<semio_s_artifact_norm_en1991_model::ConstructionStage>, key: String = by ConstructionStageKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `RoofSurface`.
    pub En1991RoofSurfacePatch of semio_s_artifact_norm_en1991_model::RoofSurface { set { face: String, shape: String, pitch_deg: f64, extent: f64 } }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Placement`.
    pub En1991PlacementPatch of semio_s_artifact_norm_en1991_model::Placement { set { x: f64, y: f64, z: f64, yaw: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `RoofSurface` list.
    pub En1991RoofSurfaceDelta { removal: En1991RoofSurfaceRemoval, insertion: En1991RoofSurfaceInsertion, relocation: En1991RoofSurfaceRelocation, modification: En1991RoofSurfaceModification, row: semio_s_artifact_norm_en1991_model::RoofSurface, patch: En1991RoofSurfacePatch, list: Vec<semio_s_artifact_norm_en1991_model::RoofSurface>, key: String = by RoofSurfaceKey }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Placement` list.
    pub En1991PlacementDelta { removal: En1991PlacementRemoval, insertion: En1991PlacementInsertion, relocation: En1991PlacementRelocation, modification: En1991PlacementModification, row: semio_s_artifact_norm_en1991_model::Placement, patch: En1991PlacementPatch, list: Vec<semio_s_artifact_norm_en1991_model::Placement>, key: String = by PlacementKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `SpecialVehicle`.
    pub En1991SpecialVehiclePatch of semio_s_artifact_norm_en1991_model::SpecialVehicle { set { designation: String, x: f64, y: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `SpecialVehicle` list.
    pub En1991SpecialVehicleDelta { removal: En1991SpecialVehicleRemoval, insertion: En1991SpecialVehicleInsertion, relocation: En1991SpecialVehicleRelocation, modification: En1991SpecialVehicleModification, row: semio_s_artifact_norm_en1991_model::SpecialVehicle, patch: En1991SpecialVehiclePatch, list: Vec<semio_s_artifact_norm_en1991_model::SpecialVehicle>, key: String = by SpecialVehicleKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WindBridge`.
    pub En1991WindBridgePatch of semio_s_artifact_norm_en1991_model::WindBridge { set { deck_width: f64, deck_depth: f64, parapet_kind: u8, parapet_height: f64, traffic_kind: u8, height_above_ground: f64, span: f64, assumed_force: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `WindBridge` list.
    pub En1991WindBridgeDelta { removal: En1991WindBridgeRemoval, insertion: En1991WindBridgeInsertion, relocation: En1991WindBridgeRelocation, modification: En1991WindBridgeModification, row: semio_s_artifact_norm_en1991_model::WindBridge, patch: En1991WindBridgePatch, list: Vec<semio_s_artifact_norm_en1991_model::WindBridge>, key: String = by WindBridgeKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `DynamicElement`.
    pub En1991DynamicElementPatch of semio_s_artifact_norm_en1991_model::DynamicElement { set { height: f64, width: f64, first_frequency: f64, log_decrement: f64, mass_per_length: f64, strouhal: f64, assumed_cscd: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `DynamicElement` list.
    pub En1991DynamicElementDelta { removal: En1991DynamicElementRemoval, insertion: En1991DynamicElementInsertion, relocation: En1991DynamicElementRelocation, modification: En1991DynamicElementModification, row: semio_s_artifact_norm_en1991_model::DynamicElement, patch: En1991DynamicElementPatch, list: Vec<semio_s_artifact_norm_en1991_model::DynamicElement>, key: String = by DynamicElementKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ImpactAction`.
    pub En1991ImpactActionPatch of semio_s_artifact_norm_en1991_model::ImpactAction { set { kind: u8, mass: f64, speed: f64, stiffness: f64, height: f64, x: f64, y: f64, assumed_force: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `ImpactAction` list.
    pub En1991ImpactActionDelta { removal: En1991ImpactActionRemoval, insertion: En1991ImpactActionInsertion, relocation: En1991ImpactActionRelocation, modification: En1991ImpactActionModification, row: semio_s_artifact_norm_en1991_model::ImpactAction, patch: En1991ImpactActionPatch, list: Vec<semio_s_artifact_norm_en1991_model::ImpactAction>, key: String = by ImpactActionKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `InternalExplosion`.
    pub En1991InternalExplosionPatch of semio_s_artifact_norm_en1991_model::InternalExplosion { set { volume: f64, vent_area: f64, vent_pressure: f64, assumed_pressure: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `InternalExplosion` list.
    pub En1991InternalExplosionDelta { removal: En1991InternalExplosionRemoval, insertion: En1991InternalExplosionInsertion, relocation: En1991InternalExplosionRelocation, modification: En1991InternalExplosionModification, row: semio_s_artifact_norm_en1991_model::InternalExplosion, patch: En1991InternalExplosionPatch, list: Vec<semio_s_artifact_norm_en1991_model::InternalExplosion>, key: String = by InternalExplosionKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ThermalSection`.
    pub En1991ThermalSectionPatch of semio_s_artifact_norm_en1991_model::ThermalSection { set { deck_type: u8, surfacing: u8, length: f64, depth: f64, alpha_micro: f64, assumed_range: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `ThermalSection` list.
    pub En1991ThermalSectionDelta { removal: En1991ThermalSectionRemoval, insertion: En1991ThermalSectionInsertion, relocation: En1991ThermalSectionRelocation, modification: En1991ThermalSectionModification, row: semio_s_artifact_norm_en1991_model::ThermalSection, patch: En1991ThermalSectionPatch, list: Vec<semio_s_artifact_norm_en1991_model::ThermalSection>, key: String = by ThermalSectionKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `HelicopterLanding`.
    pub En1991HelicopterLandingPatch of semio_s_artifact_norm_en1991_model::HelicopterLanding { set { mass: f64, x: f64, y: f64, height: f64, assumed_force: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `HelicopterLanding` list.
    pub En1991HelicopterLandingDelta { removal: En1991HelicopterLandingRemoval, insertion: En1991HelicopterLandingInsertion, relocation: En1991HelicopterLandingRelocation, modification: En1991HelicopterLandingModification, row: semio_s_artifact_norm_en1991_model::HelicopterLanding, patch: En1991HelicopterLandingPatch, list: Vec<semio_s_artifact_norm_en1991_model::HelicopterLanding>, key: String = by HelicopterLandingKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `TieRun`.
    pub En1991TieRunPatch of semio_s_artifact_norm_en1991_model::TieRun { set { kind: u8, spacing: f64, span: f64, dead_load: f64, imposed_load: f64, psi: f64, x: f64, y: f64, provided_force: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `TieRun` list.
    pub En1991TieRunDelta { removal: En1991TieRunRemoval, insertion: En1991TieRunInsertion, relocation: En1991TieRunRelocation, modification: En1991TieRunModification, row: semio_s_artifact_norm_en1991_model::TieRun, patch: En1991TieRunPatch, list: Vec<semio_s_artifact_norm_en1991_model::TieRun>, key: String = by TieRunKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ConstructionLoad`.
    pub En1991ConstructionLoadPatch of semio_s_artifact_norm_en1991_model::ConstructionLoad { set { stage: String, kind: u8, magnitude: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `ConstructionLoad` list.
    pub En1991ConstructionLoadDelta { removal: En1991ConstructionLoadRemoval, insertion: En1991ConstructionLoadInsertion, relocation: En1991ConstructionLoadRelocation, modification: En1991ConstructionLoadModification, row: semio_s_artifact_norm_en1991_model::ConstructionLoad, patch: En1991ConstructionLoadPatch, list: Vec<semio_s_artifact_norm_en1991_model::ConstructionLoad>, key: String = by ConstructionLoadKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FallingLoad`.
    pub En1991FallingLoadPatch of semio_s_artifact_norm_en1991_model::FallingLoad { set { mass: f64, drop_height: f64, stopping_distance: f64, x: f64, y: f64, assumed_force: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `FallingLoad` list.
    pub En1991FallingLoadDelta { removal: En1991FallingLoadRemoval, insertion: En1991FallingLoadInsertion, relocation: En1991FallingLoadRelocation, modification: En1991FallingLoadModification, row: semio_s_artifact_norm_en1991_model::FallingLoad, patch: En1991FallingLoadPatch, list: Vec<semio_s_artifact_norm_en1991_model::FallingLoad>, key: String = by FallingLoadKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ExecutionImperfection`.
    pub En1991ExecutionImperfectionPatch of semio_s_artifact_norm_en1991_model::ExecutionImperfection { set { height: f64, tolerance: f64, vertical_load: f64, x: f64, y: f64, assumed_force: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `ExecutionImperfection` list.
    pub En1991ExecutionImperfectionDelta { removal: En1991ExecutionImperfectionRemoval, insertion: En1991ExecutionImperfectionInsertion, relocation: En1991ExecutionImperfectionRelocation, modification: En1991ExecutionImperfectionModification, row: semio_s_artifact_norm_en1991_model::ExecutionImperfection, patch: En1991ExecutionImperfectionPatch, list: Vec<semio_s_artifact_norm_en1991_model::ExecutionImperfection>, key: String = by ExecutionImperfectionKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `DerailmentCase`.
    pub En1991DerailmentCasePatch of semio_s_artifact_norm_en1991_model::DerailmentCase { set { distance: f64, design_force_x: f64, design_force_y: f64, x: f64, y: f64, assumed_force: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `DerailmentCase` list.
    pub En1991DerailmentCaseDelta { removal: En1991DerailmentCaseRemoval, insertion: En1991DerailmentCaseInsertion, relocation: En1991DerailmentCaseRelocation, modification: En1991DerailmentCaseModification, row: semio_s_artifact_norm_en1991_model::DerailmentCase, patch: En1991DerailmentCasePatch, list: Vec<semio_s_artifact_norm_en1991_model::DerailmentCase>, key: String = by DerailmentCaseKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `CraneHoist`.
    pub En1991CraneHoistPatch of semio_s_artifact_norm_en1991_model::CraneHoist { set { rated_load: f64, release_fraction: f64, release_device: String } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `CraneHoist` list.
    pub En1991CraneHoistDelta { removal: En1991CraneHoistRemoval, insertion: En1991CraneHoistInsertion, relocation: En1991CraneHoistRelocation, modification: En1991CraneHoistModification, row: semio_s_artifact_norm_en1991_model::CraneHoist, patch: En1991CraneHoistPatch, list: Vec<semio_s_artifact_norm_en1991_model::CraneHoist>, key: String = by CraneHoistKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `Footbridge`.
    pub En1991FootbridgePatch of semio_s_artifact_norm_en1991_model::Footbridge { set { loaded_length: f64, deck_width: f64, natural_frequency: f64 , assumed_service_axle: f64} }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `Footbridge` list.
    pub En1991FootbridgeDelta { removal: En1991FootbridgeRemoval, insertion: En1991FootbridgeInsertion, relocation: En1991FootbridgeRelocation, modification: En1991FootbridgeModification, row: semio_s_artifact_norm_en1991_model::Footbridge, patch: En1991FootbridgePatch, list: Vec<semio_s_artifact_norm_en1991_model::Footbridge>, key: String = by FootbridgeKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `RailDynamic`.
    pub En1991RailDynamicPatch of semio_s_artifact_norm_en1991_model::RailDynamic { set { determinant_length: f64, maintenance: String } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `RailDynamic` list.
    pub En1991RailDynamicDelta { removal: En1991RailDynamicRemoval, insertion: En1991RailDynamicInsertion, relocation: En1991RailDynamicRelocation, modification: En1991RailDynamicModification, row: semio_s_artifact_norm_en1991_model::RailDynamic, patch: En1991RailDynamicPatch, list: Vec<semio_s_artifact_norm_en1991_model::RailDynamic>, key: String = by RailDynamicKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `LoadDispersal`.
    pub En1991LoadDispersalPatch of semio_s_artifact_norm_en1991_model::LoadDispersal { set { wheel_length: f64, wheel_width: f64, pavement_thickness: f64, slab_thickness: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `LoadDispersal` list.
    pub En1991LoadDispersalDelta { removal: En1991LoadDispersalRemoval, insertion: En1991LoadDispersalInsertion, relocation: En1991LoadDispersalRelocation, modification: En1991LoadDispersalModification, row: semio_s_artifact_norm_en1991_model::LoadDispersal, patch: En1991LoadDispersalPatch, list: Vec<semio_s_artifact_norm_en1991_model::LoadDispersal>, key: String = by LoadDispersalKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `BulkSolid`.
    pub En1991BulkSolidPatch of semio_s_artifact_norm_en1991_model::BulkSolid { set { internal_friction: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `BulkSolid` list.
    pub En1991BulkSolidDelta { removal: En1991BulkSolidRemoval, insertion: En1991BulkSolidInsertion, relocation: En1991BulkSolidRelocation, modification: En1991BulkSolidModification, row: semio_s_artifact_norm_en1991_model::BulkSolid, patch: En1991BulkSolidPatch, list: Vec<semio_s_artifact_norm_en1991_model::BulkSolid>, key: String = by BulkSolidKey }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `RotatingMachine`.
    pub En1991RotatingMachinePatch of semio_s_artifact_norm_en1991_model::RotatingMachine { set { mass: f64, eccentricity: f64, frequency: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `RotatingMachine` list.
    pub En1991RotatingMachineDelta { removal: En1991RotatingMachineRemoval, insertion: En1991RotatingMachineInsertion, relocation: En1991RotatingMachineRelocation, modification: En1991RotatingMachineModification, row: semio_s_artifact_norm_en1991_model::RotatingMachine, patch: En1991RotatingMachinePatch, list: Vec<semio_s_artifact_norm_en1991_model::RotatingMachine>, key: String = by RotatingMachineKey }
}

/// 🔑️ Key extractor of the foreign `DustExplosion` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DustExplosionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::DustExplosion> for DustExplosionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::DustExplosion) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `DustExplosion`.
    pub En1991DustExplosionPatch of semio_s_artifact_norm_en1991_model::DustExplosion { set { kst: f64, p_max: f64, volume: f64, vent_area: f64, static_opening_pressure: f64, design_pressure: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `DustExplosion` list.
    pub En1991DustExplosionDelta { removal: En1991DustExplosionRemoval, insertion: En1991DustExplosionInsertion, relocation: En1991DustExplosionRelocation, modification: En1991DustExplosionModification, row: semio_s_artifact_norm_en1991_model::DustExplosion, patch: En1991DustExplosionPatch, list: Vec<semio_s_artifact_norm_en1991_model::DustExplosion>, key: String = by DustExplosionKey }
}

/// 🔑️ Key extractor of the foreign `RailLoadGroup` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RailLoadGroupKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::RailLoadGroup> for RailLoadGroupKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::RailLoadGroup) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `RailLoadGroup`.
    pub En1991RailLoadGroupPatch of semio_s_artifact_norm_en1991_model::RailLoadGroup { set { load_group: String, factor_vertical: f64, factor_longitudinal: f64, factor_lateral: f64, assumed_axle_load: f64, assumed_udl: f64, assumed_longitudinal: f64, assumed_lateral: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `RailLoadGroup` list.
    pub En1991RailLoadGroupDelta { removal: En1991RailLoadGroupRemoval, insertion: En1991RailLoadGroupInsertion, relocation: En1991RailLoadGroupRelocation, modification: En1991RailLoadGroupModification, row: semio_s_artifact_norm_en1991_model::RailLoadGroup, patch: En1991RailLoadGroupPatch, list: Vec<semio_s_artifact_norm_en1991_model::RailLoadGroup>, key: String = by RailLoadGroupKey }
}

/// 🔑️ Key extractor of the foreign `RailDerailment` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RailDerailmentKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::RailDerailment> for RailDerailmentKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::RailDerailment) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `RailDerailment`.
    pub En1991RailDerailmentPatch of semio_s_artifact_norm_en1991_model::RailDerailment { set { situation: String, assumed_line_load: f64, loaded_length: f64, offset: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `RailDerailment` list.
    pub En1991RailDerailmentDelta { removal: En1991RailDerailmentRemoval, insertion: En1991RailDerailmentInsertion, relocation: En1991RailDerailmentRelocation, modification: En1991RailDerailmentModification, row: semio_s_artifact_norm_en1991_model::RailDerailment, patch: En1991RailDerailmentPatch, list: Vec<semio_s_artifact_norm_en1991_model::RailDerailment>, key: String = by RailDerailmentKey }
}

/// 🔑️ Key extractor of the foreign `FatigueModel` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FatigueModelKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::FatigueModel> for FatigueModelKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::FatigueModel) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `FatigueModel`.
    pub En1991FatigueModelPatch of semio_s_artifact_norm_en1991_model::FatigueModel { set { model: String, lane: u8, assumed_axle_load: f64, assumed_udl: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `FatigueModel` list.
    pub En1991FatigueModelDelta { removal: En1991FatigueModelRemoval, insertion: En1991FatigueModelInsertion, relocation: En1991FatigueModelRelocation, modification: En1991FatigueModelModification, row: semio_s_artifact_norm_en1991_model::FatigueModel, patch: En1991FatigueModelPatch, list: Vec<semio_s_artifact_norm_en1991_model::FatigueModel>, key: String = by FatigueModelKey }
}

/// 🔑️ Key extractor of the foreign `SupportCollision` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SupportCollisionKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::SupportCollision> for SupportCollisionKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::SupportCollision) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `SupportCollision`.
    pub En1991SupportCollisionPatch of semio_s_artifact_norm_en1991_model::SupportCollision { set { location: String, assumed_force_x: f64, assumed_force_y: f64, height: f64, position_x: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `SupportCollision` list.
    pub En1991SupportCollisionDelta { removal: En1991SupportCollisionRemoval, insertion: En1991SupportCollisionInsertion, relocation: En1991SupportCollisionRelocation, modification: En1991SupportCollisionModification, row: semio_s_artifact_norm_en1991_model::SupportCollision, patch: En1991SupportCollisionPatch, list: Vec<semio_s_artifact_norm_en1991_model::SupportCollision>, key: String = by SupportCollisionKey }
}

/// 🔑️ Key extractor of the foreign `LocalFire` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocalFireKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::LocalFire> for LocalFireKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::LocalFire) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `LocalFire`.
    pub En1991LocalFirePatch of semio_s_artifact_norm_en1991_model::LocalFire { set { fire_diameter: f64, heat_release_rate: f64, ceiling_height: f64, horizontal_distance: f64, assumed_flux: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `LocalFire` list.
    pub En1991LocalFireDelta { removal: En1991LocalFireRemoval, insertion: En1991LocalFireInsertion, relocation: En1991LocalFireRelocation, modification: En1991LocalFireModification, row: semio_s_artifact_norm_en1991_model::LocalFire, patch: En1991LocalFirePatch, list: Vec<semio_s_artifact_norm_en1991_model::LocalFire>, key: String = by LocalFireKey }
}

/// 🔑️ Key extractor of the foreign `SnowGuard` rows (`id`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnowGuardKey;

impl protocol::list_delta::KeyOf<semio_s_artifact_norm_en1991_model::SnowGuard> for SnowGuardKey {
    type Key = String;
    fn key_of(row: &semio_s_artifact_norm_en1991_model::SnowGuard) -> String {
        row.id.clone()
    }
}

protocol::row_patch! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `SnowGuard`.
    pub En1991SnowGuardPatch of semio_s_artifact_norm_en1991_model::SnowGuard { set { roof_id: String, slope_length: f64, assumed_force: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `SnowGuard` list.
    pub En1991SnowGuardDelta { removal: En1991SnowGuardRemoval, insertion: En1991SnowGuardInsertion, relocation: En1991SnowGuardRelocation, modification: En1991SnowGuardModification, row: semio_s_artifact_norm_en1991_model::SnowGuard, patch: En1991SnowGuardPatch, list: Vec<semio_s_artifact_norm_en1991_model::SnowGuard>, key: String = by SnowGuardKey }
}

//#endregion 🔖️Rows

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1991 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1991")]
pub struct En1991Diff {
    #[state(artifact)]
    pub annex: Option<semio_s_plugin_norm_compliance::AnnexChoice>,
    #[state(artifact)]
    pub snow_zone: Option<String>,
    #[state(artifact)]
    pub altitude: Option<f64>,
    #[state(artifact)]
    pub en_sk: Option<f64>,
    #[state(artifact)]
    pub north_german_lowland_snow: Option<bool>,
    #[state(artifact)]
    pub wind_zone: Option<u8>,
    #[state(artifact)]
    pub en_vb: Option<f64>,
    #[state(artifact)]
    pub terrain_category: Option<u8>,
    #[state(artifact)]
    pub mixed_terrain_upwind: Option<u8>,
    #[state(artifact)]
    pub mixed_terrain_distance: Option<f64>,
    #[state(artifact)]
    pub orography_factor: Option<f64>,
    #[state(artifact)]
    pub coast_or_island: Option<bool>,
    #[state(artifact)]
    pub air_density: Option<f64>,
    #[state(artifact)]
    pub height: Option<f64>,
    #[state(artifact)]
    pub width: Option<f64>,
    #[state(artifact)]
    pub depth: Option<f64>,
    #[state(artifact)]
    pub assumed_delta_t: Option<f64>,
    #[state(artifact)]
    pub t_max: Option<f64>,
    #[state(artifact)]
    pub t_min: Option<f64>,
    #[state(artifact)]
    pub t_0: Option<f64>,
    #[state(artifact)]
    pub thermal_element_type: Option<String>,
    #[state(artifact)]
    pub thermal_bridge_type: Option<u8>,
    #[state(artifact)]
    pub delta_t_m: Option<f64>,
    #[state(artifact)]
    pub storey_count: Option<u8>,
    #[state(artifact)]
    pub fire_mode: Option<semio_s_artifact_norm_en1991_model::FireMode>,
    #[state(artifact)]
    pub fire_curve: Option<semio_s_artifact_norm_en1991_model::part_1_2::FireCurve>,
    #[state(artifact)]
    pub fire_duration: Option<f64>,
    #[state(artifact)]
    pub assumed_gas_temperature: Option<f64>,
    #[state(artifact)]
    pub assumed_h_net: Option<f64>,
    #[state(artifact)]
    pub fire_compartment_area: Option<f64>,
    #[state(artifact)]
    pub fire_compartment_height: Option<f64>,
    #[state(artifact)]
    pub fire_opening_factor: Option<f64>,
    #[state(artifact)]
    pub fire_thermal_inertia: Option<f64>,
    #[state(artifact)]
    pub fire_occupancy: Option<semio_s_artifact_norm_en1991_model::FireOccupancy>,
    #[state(artifact)]
    pub fire_load_density_qf: Option<f64>,
    #[state(artifact)]
    pub assumed_qf_d: Option<f64>,
    #[state(artifact)]
    pub construction_activity: Option<String>,
    #[state(artifact)]
    pub assumed_construction_qk: Option<f64>,
    #[state(artifact)]
    pub structure_kind: Option<semio_s_artifact_norm_en1991_model::StructureKind>,
    #[state(artifact)]
    pub bridge_lane: Option<u8>,
    #[state(artifact)]
    pub bridge_span: Option<f64>,
    #[state(artifact)]
    pub bridge_lane_width: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_tandem: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_udl: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_lm2: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_footway: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_lm3: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_lm4: Option<f64>,
    #[state(artifact)]
    pub bridge_load_group: Option<String>,
    #[state(artifact)]
    pub crane_claimed: Option<bool>,
    #[state(artifact)]
    pub crane_class: Option<String>,
    #[state(artifact)]
    pub hoist_class: Option<String>,
    #[state(artifact)]
    pub hoisting_speed: Option<f64>,
    #[state(artifact)]
    pub assumed_crane_wheel: Option<f64>,
    #[state(artifact)]
    pub assumed_crane_horizontal: Option<f64>,
    #[state(artifact)]
    pub silo_claimed: Option<bool>,
    #[state(artifact)]
    pub silo_kind: Option<String>,
    #[state(artifact)]
    pub silo_bulk_density: Option<f64>,
    #[state(artifact)]
    pub silo_height: Option<f64>,
    #[state(artifact)]
    pub silo_hydraulic_radius: Option<f64>,
    #[state(artifact)]
    pub silo_mu: Option<f64>,
    #[state(artifact)]
    pub silo_k: Option<f64>,
    #[state(artifact)]
    pub assumed_silo_pressure: Option<f64>,
    #[state(artifact)]
    pub assumed_silo_patch: Option<f64>,
    #[state(artifact)]
    pub assumed_silo_wall_friction: Option<f64>,
    #[state(artifact)]
    pub bridge_tandem_x: Option<f64>,
    #[state(artifact)]
    pub rail_alpha: Option<f64>,
    #[state(artifact)]
    pub assumed_rail_lm71: Option<f64>,
    #[state(artifact)]
    pub assumed_rail_lm71_udl: Option<f64>,
    #[state(artifact)]
    pub assumed_rail_sw0: Option<f64>,
    #[state(artifact)]
    pub assumed_rail_sw2: Option<f64>,
    #[state(artifact)]
    pub crane_x: Option<f64>,
    #[state(artifact)]
    pub crane_runway_length: Option<f64>,
    #[state(artifact)]
    pub crane_wheel_base: Option<f64>,
    #[state(artifact)]
    pub silo_diameter: Option<f64>,
    #[state(artifact)]
    pub silo_eccentricity: Option<f64>,
    #[state(artifact)]
    pub silo_hopper_angle_deg: Option<f64>,
    #[state(artifact)]
    pub bridge_curve_radius: Option<f64>,
    #[state(artifact)]
    pub fire_load_basis: Option<semio_s_artifact_norm_en1991_model::FireLoadBasis>,
    #[state(artifact)]
    pub floors: En1991FloorDelta,
    #[state(artifact)]
    pub self_weight_elements: En1991SelfWeightElementDelta,
    #[state(artifact)]
    pub roofs: En1991RoofDelta,
    #[state(artifact)]
    pub wind_faces: En1991WindFaceDelta,
    #[state(artifact)]
    pub accidental_cases: En1991AccidentalCaseDelta,
    #[state(artifact)]
    pub bridge_lanes: En1991BridgeLaneDelta,
    #[state(artifact)]
    pub construction_stages: En1991ConstructionStageDelta,
    #[state(artifact)]
    pub roof_surfaces: En1991RoofSurfaceDelta,
    #[state(artifact)]
    pub placements: En1991PlacementDelta,
    #[state(artifact)]
    pub special_vehicles: En1991SpecialVehicleDelta,
    #[state(artifact)]
    pub wind_bridges: En1991WindBridgeDelta,
    #[state(artifact)]
    pub dynamic_elements: En1991DynamicElementDelta,
    #[state(artifact)]
    pub impact_actions: En1991ImpactActionDelta,
    #[state(artifact)]
    pub internal_explosions: En1991InternalExplosionDelta,
    #[state(artifact)]
    pub thermal_sections: En1991ThermalSectionDelta,
    #[state(artifact)]
    pub cases: semio_s_plugin_norm_compliance::design_case::CaseRowDelta,
    #[state(artifact)]
    pub helicopter_landings: En1991HelicopterLandingDelta,
    #[state(artifact)]
    pub tie_runs: En1991TieRunDelta,
    #[state(artifact)]
    pub construction_loads: En1991ConstructionLoadDelta,
    #[state(artifact)]
    pub falling_loads: En1991FallingLoadDelta,
    #[state(artifact)]
    pub execution_imperfections: En1991ExecutionImperfectionDelta,
    #[state(artifact)]
    pub derailment_cases: En1991DerailmentCaseDelta,
    #[state(artifact)]
    pub crane_hoists: En1991CraneHoistDelta,
    #[state(artifact)]
    pub footbridges: En1991FootbridgeDelta,
    #[state(artifact)]
    pub rail_dynamics: En1991RailDynamicDelta,
    #[state(artifact)]
    pub load_dispersals: En1991LoadDispersalDelta,
    #[state(artifact)]
    pub bulk_solids: En1991BulkSolidDelta,
    #[state(artifact)]
    pub rotating_machines: En1991RotatingMachineDelta,
    #[state(artifact)]
    pub dust_explosions: En1991DustExplosionDelta,
    #[state(artifact)]
    pub rail_load_groups: En1991RailLoadGroupDelta,
    #[state(artifact)]
    pub rail_derailments: En1991RailDerailmentDelta,
    #[state(artifact)]
    pub fatigue_models: Option<En1991FatigueModelDelta>,
    #[state(artifact)]
    pub support_collisions: Option<En1991SupportCollisionDelta>,
    #[state(artifact)]
    pub local_fires: Option<En1991LocalFireDelta>,
    #[state(artifact)]
    pub snow_guards: Option<En1991SnowGuardDelta>,
}
//#endregion 🔖️Diff

impl MutationDiff<En1991Snapshot> for En1991Diff {}

impl protocol::ColdMutationDiff<En1991Snapshot> for En1991Diff {
    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.snow_zone.is_some() {
            self.snow_zone = other.snow_zone;
        }
        if other.altitude.is_some() {
            self.altitude = other.altitude;
        }
        if other.en_sk.is_some() {
            self.en_sk = other.en_sk;
        }
        if other.north_german_lowland_snow.is_some() {
            self.north_german_lowland_snow = other.north_german_lowland_snow;
        }
        if other.wind_zone.is_some() {
            self.wind_zone = other.wind_zone;
        }
        if other.en_vb.is_some() {
            self.en_vb = other.en_vb;
        }
        if other.terrain_category.is_some() {
            self.terrain_category = other.terrain_category;
        }
        if other.mixed_terrain_upwind.is_some() {
            self.mixed_terrain_upwind = other.mixed_terrain_upwind;
        }
        if other.mixed_terrain_distance.is_some() {
            self.mixed_terrain_distance = other.mixed_terrain_distance;
        }
        if other.orography_factor.is_some() {
            self.orography_factor = other.orography_factor;
        }
        if other.coast_or_island.is_some() {
            self.coast_or_island = other.coast_or_island;
        }
        if other.air_density.is_some() {
            self.air_density = other.air_density;
        }
        if other.height.is_some() {
            self.height = other.height;
        }
        if other.width.is_some() {
            self.width = other.width;
        }
        if other.depth.is_some() {
            self.depth = other.depth;
        }
        if other.assumed_delta_t.is_some() {
            self.assumed_delta_t = other.assumed_delta_t;
        }
        if other.t_max.is_some() {
            self.t_max = other.t_max;
        }
        if other.t_min.is_some() {
            self.t_min = other.t_min;
        }
        if other.t_0.is_some() {
            self.t_0 = other.t_0;
        }
        if other.thermal_element_type.is_some() {
            self.thermal_element_type = other.thermal_element_type;
        }
        if other.thermal_bridge_type.is_some() {
            self.thermal_bridge_type = other.thermal_bridge_type;
        }
        if other.delta_t_m.is_some() {
            self.delta_t_m = other.delta_t_m;
        }
        if other.storey_count.is_some() {
            self.storey_count = other.storey_count;
        }
        if other.fire_mode.is_some() {
            self.fire_mode = other.fire_mode;
        }
        if other.fire_curve.is_some() {
            self.fire_curve = other.fire_curve;
        }
        if other.fire_duration.is_some() {
            self.fire_duration = other.fire_duration;
        }
        if other.assumed_gas_temperature.is_some() {
            self.assumed_gas_temperature = other.assumed_gas_temperature;
        }
        if other.assumed_h_net.is_some() {
            self.assumed_h_net = other.assumed_h_net;
        }
        if other.fire_compartment_area.is_some() {
            self.fire_compartment_area = other.fire_compartment_area;
        }
        if other.fire_compartment_height.is_some() {
            self.fire_compartment_height = other.fire_compartment_height;
        }
        if other.fire_opening_factor.is_some() {
            self.fire_opening_factor = other.fire_opening_factor;
        }
        if other.fire_thermal_inertia.is_some() {
            self.fire_thermal_inertia = other.fire_thermal_inertia;
        }
        if other.fire_occupancy.is_some() {
            self.fire_occupancy = other.fire_occupancy;
        }
        if other.fire_load_density_qf.is_some() {
            self.fire_load_density_qf = other.fire_load_density_qf;
        }
        if other.assumed_qf_d.is_some() {
            self.assumed_qf_d = other.assumed_qf_d;
        }
        if other.construction_activity.is_some() {
            self.construction_activity = other.construction_activity;
        }
        if other.assumed_construction_qk.is_some() {
            self.assumed_construction_qk = other.assumed_construction_qk;
        }
        if other.structure_kind.is_some() {
            self.structure_kind = other.structure_kind;
        }
        if other.bridge_lane.is_some() {
            self.bridge_lane = other.bridge_lane;
        }
        if other.bridge_span.is_some() {
            self.bridge_span = other.bridge_span;
        }
        if other.bridge_lane_width.is_some() {
            self.bridge_lane_width = other.bridge_lane_width;
        }
        if other.assumed_bridge_tandem.is_some() {
            self.assumed_bridge_tandem = other.assumed_bridge_tandem;
        }
        if other.assumed_bridge_udl.is_some() {
            self.assumed_bridge_udl = other.assumed_bridge_udl;
        }
        if other.assumed_bridge_lm2.is_some() {
            self.assumed_bridge_lm2 = other.assumed_bridge_lm2;
        }
        if other.assumed_bridge_footway.is_some() {
            self.assumed_bridge_footway = other.assumed_bridge_footway;
        }
        if other.assumed_bridge_lm3.is_some() {
            self.assumed_bridge_lm3 = other.assumed_bridge_lm3;
        }
        if other.assumed_bridge_lm4.is_some() {
            self.assumed_bridge_lm4 = other.assumed_bridge_lm4;
        }
        if other.bridge_load_group.is_some() {
            self.bridge_load_group = other.bridge_load_group;
        }
        if other.crane_claimed.is_some() {
            self.crane_claimed = other.crane_claimed;
        }
        if other.crane_class.is_some() {
            self.crane_class = other.crane_class;
        }
        if other.hoist_class.is_some() {
            self.hoist_class = other.hoist_class;
        }
        if other.hoisting_speed.is_some() {
            self.hoisting_speed = other.hoisting_speed;
        }
        if other.assumed_crane_wheel.is_some() {
            self.assumed_crane_wheel = other.assumed_crane_wheel;
        }
        if other.assumed_crane_horizontal.is_some() {
            self.assumed_crane_horizontal = other.assumed_crane_horizontal;
        }
        if other.silo_claimed.is_some() {
            self.silo_claimed = other.silo_claimed;
        }
        if other.silo_kind.is_some() {
            self.silo_kind = other.silo_kind;
        }
        if other.silo_bulk_density.is_some() {
            self.silo_bulk_density = other.silo_bulk_density;
        }
        if other.silo_height.is_some() {
            self.silo_height = other.silo_height;
        }
        if other.silo_hydraulic_radius.is_some() {
            self.silo_hydraulic_radius = other.silo_hydraulic_radius;
        }
        if other.silo_mu.is_some() {
            self.silo_mu = other.silo_mu;
        }
        if other.silo_k.is_some() {
            self.silo_k = other.silo_k;
        }
        if other.assumed_silo_pressure.is_some() {
            self.assumed_silo_pressure = other.assumed_silo_pressure;
        }
        if other.assumed_silo_patch.is_some() {
            self.assumed_silo_patch = other.assumed_silo_patch;
        }
        if other.assumed_silo_wall_friction.is_some() {
            self.assumed_silo_wall_friction = other.assumed_silo_wall_friction;
        }
        if other.bridge_tandem_x.is_some() {
            self.bridge_tandem_x = other.bridge_tandem_x;
        }
        if other.rail_alpha.is_some() {
            self.rail_alpha = other.rail_alpha;
        }
        if other.assumed_rail_lm71.is_some() {
            self.assumed_rail_lm71 = other.assumed_rail_lm71;
        }
        if other.assumed_rail_lm71_udl.is_some() {
            self.assumed_rail_lm71_udl = other.assumed_rail_lm71_udl;
        }
        if other.assumed_rail_sw0.is_some() {
            self.assumed_rail_sw0 = other.assumed_rail_sw0;
        }
        if other.assumed_rail_sw2.is_some() {
            self.assumed_rail_sw2 = other.assumed_rail_sw2;
        }
        if other.crane_x.is_some() {
            self.crane_x = other.crane_x;
        }
        if other.crane_runway_length.is_some() {
            self.crane_runway_length = other.crane_runway_length;
        }
        if other.crane_wheel_base.is_some() {
            self.crane_wheel_base = other.crane_wheel_base;
        }
        if other.silo_diameter.is_some() {
            self.silo_diameter = other.silo_diameter;
        }
        if other.silo_eccentricity.is_some() {
            self.silo_eccentricity = other.silo_eccentricity;
        }
        if other.silo_hopper_angle_deg.is_some() {
            self.silo_hopper_angle_deg = other.silo_hopper_angle_deg;
        }
        if other.bridge_curve_radius.is_some() {
            self.bridge_curve_radius = other.bridge_curve_radius;
        }
        if other.fire_load_basis.is_some() {
            self.fire_load_basis = other.fire_load_basis;
        }
        self.floors.absorb(other.floors);
        self.self_weight_elements.absorb(other.self_weight_elements);
        self.roofs.absorb(other.roofs);
        self.wind_faces.absorb(other.wind_faces);
        self.accidental_cases.absorb(other.accidental_cases);
        self.bridge_lanes.absorb(other.bridge_lanes);
        self.construction_stages.absorb(other.construction_stages);
        self.roof_surfaces.absorb(other.roof_surfaces);
        self.placements.absorb(other.placements);
        self.special_vehicles.absorb(other.special_vehicles);
        self.wind_bridges.absorb(other.wind_bridges);
        self.dynamic_elements.absorb(other.dynamic_elements);
        self.impact_actions.absorb(other.impact_actions);
        self.internal_explosions.absorb(other.internal_explosions);
        self.thermal_sections.absorb(other.thermal_sections);
        self.cases.absorb(other.cases);
        self.helicopter_landings.absorb(other.helicopter_landings);
        self.tie_runs.absorb(other.tie_runs);
        self.construction_loads.absorb(other.construction_loads);
        self.falling_loads.absorb(other.falling_loads);
        self.execution_imperfections.absorb(other.execution_imperfections);
        self.derailment_cases.absorb(other.derailment_cases);
        self.crane_hoists.absorb(other.crane_hoists);
        self.footbridges.absorb(other.footbridges);
        self.rail_dynamics.absorb(other.rail_dynamics);
        self.load_dispersals.absorb(other.load_dispersals);
        self.bulk_solids.absorb(other.bulk_solids);
        self.rotating_machines.absorb(other.rotating_machines);
        self.dust_explosions.absorb(other.dust_explosions);
        self.rail_load_groups.absorb(other.rail_load_groups);
        self.rail_derailments.absorb(other.rail_derailments);
        self.fatigue_models.absorb(other.fatigue_models);
        self.support_collisions.absorb(other.support_collisions);
        self.local_fires.absorb(other.local_fires);
        self.snow_guards.absorb(other.snow_guards);
    }

    fn apply(&self, unpublished: &mut En1991Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        *unpublished = En1991Snapshot {
            annex: self.annex.unwrap_or(base.annex),
            snow_zone: self.snow_zone.clone().unwrap_or_else(|| base.snow_zone.clone()),
            altitude: self.altitude.unwrap_or(base.altitude),
            en_sk: self.en_sk.unwrap_or(base.en_sk),
            north_german_lowland_snow: self.north_german_lowland_snow.unwrap_or(base.north_german_lowland_snow),
            wind_zone: self.wind_zone.unwrap_or(base.wind_zone),
            en_vb: self.en_vb.unwrap_or(base.en_vb),
            terrain_category: self.terrain_category.unwrap_or(base.terrain_category),
            mixed_terrain_upwind: self.mixed_terrain_upwind.unwrap_or(base.mixed_terrain_upwind),
            mixed_terrain_distance: self.mixed_terrain_distance.unwrap_or(base.mixed_terrain_distance),
            orography_factor: self.orography_factor.unwrap_or(base.orography_factor),
            coast_or_island: self.coast_or_island.unwrap_or(base.coast_or_island),
            air_density: self.air_density.unwrap_or(base.air_density),
            height: self.height.unwrap_or(base.height),
            width: self.width.unwrap_or(base.width),
            depth: self.depth.unwrap_or(base.depth),
            assumed_delta_t: self.assumed_delta_t.unwrap_or(base.assumed_delta_t),
            t_max: self.t_max.unwrap_or(base.t_max),
            t_min: self.t_min.unwrap_or(base.t_min),
            t_0: self.t_0.unwrap_or(base.t_0),
            thermal_element_type: self.thermal_element_type.clone().unwrap_or_else(|| base.thermal_element_type.clone()),
            thermal_bridge_type: self.thermal_bridge_type.unwrap_or(base.thermal_bridge_type),
            delta_t_m: self.delta_t_m.unwrap_or(base.delta_t_m),
            storey_count: self.storey_count.unwrap_or(base.storey_count),
            fire_mode: self.fire_mode.unwrap_or(base.fire_mode),
            fire_curve: self.fire_curve.unwrap_or(base.fire_curve),
            fire_duration: self.fire_duration.unwrap_or(base.fire_duration),
            assumed_gas_temperature: self.assumed_gas_temperature.unwrap_or(base.assumed_gas_temperature),
            assumed_h_net: self.assumed_h_net.unwrap_or(base.assumed_h_net),
            fire_compartment_area: self.fire_compartment_area.unwrap_or(base.fire_compartment_area),
            fire_compartment_height: self.fire_compartment_height.unwrap_or(base.fire_compartment_height),
            fire_opening_factor: self.fire_opening_factor.unwrap_or(base.fire_opening_factor),
            fire_thermal_inertia: self.fire_thermal_inertia.unwrap_or(base.fire_thermal_inertia),
            fire_occupancy: self.fire_occupancy.clone().unwrap_or_else(|| base.fire_occupancy.clone()),
            fire_load_density_qf: self.fire_load_density_qf.unwrap_or(base.fire_load_density_qf),
            assumed_qf_d: self.assumed_qf_d.unwrap_or(base.assumed_qf_d),
            construction_activity: self.construction_activity.clone().unwrap_or_else(|| base.construction_activity.clone()),
            assumed_construction_qk: self.assumed_construction_qk.unwrap_or(base.assumed_construction_qk),
            structure_kind: self.structure_kind.unwrap_or(base.structure_kind),
            bridge_lane: self.bridge_lane.unwrap_or(base.bridge_lane),
            bridge_span: self.bridge_span.unwrap_or(base.bridge_span),
            bridge_lane_width: self.bridge_lane_width.unwrap_or(base.bridge_lane_width),
            assumed_bridge_tandem: self.assumed_bridge_tandem.unwrap_or(base.assumed_bridge_tandem),
            assumed_bridge_udl: self.assumed_bridge_udl.unwrap_or(base.assumed_bridge_udl),
            assumed_bridge_lm2: self.assumed_bridge_lm2.unwrap_or(base.assumed_bridge_lm2),
            assumed_bridge_footway: self.assumed_bridge_footway.unwrap_or(base.assumed_bridge_footway),
            assumed_bridge_lm3: self.assumed_bridge_lm3.unwrap_or(base.assumed_bridge_lm3),
            assumed_bridge_lm4: self.assumed_bridge_lm4.unwrap_or(base.assumed_bridge_lm4),
            bridge_load_group: self.bridge_load_group.clone().unwrap_or_else(|| base.bridge_load_group.clone()),
            crane_claimed: self.crane_claimed.unwrap_or(base.crane_claimed),
            crane_class: self.crane_class.clone().unwrap_or_else(|| base.crane_class.clone()),
            hoist_class: self.hoist_class.clone().unwrap_or_else(|| base.hoist_class.clone()),
            hoisting_speed: self.hoisting_speed.unwrap_or(base.hoisting_speed),
            assumed_crane_wheel: self.assumed_crane_wheel.unwrap_or(base.assumed_crane_wheel),
            assumed_crane_horizontal: self.assumed_crane_horizontal.unwrap_or(base.assumed_crane_horizontal),
            silo_claimed: self.silo_claimed.unwrap_or(base.silo_claimed),
            silo_kind: self.silo_kind.clone().unwrap_or_else(|| base.silo_kind.clone()),
            silo_bulk_density: self.silo_bulk_density.unwrap_or(base.silo_bulk_density),
            silo_height: self.silo_height.unwrap_or(base.silo_height),
            silo_hydraulic_radius: self.silo_hydraulic_radius.unwrap_or(base.silo_hydraulic_radius),
            silo_mu: self.silo_mu.unwrap_or(base.silo_mu),
            silo_k: self.silo_k.unwrap_or(base.silo_k),
            assumed_silo_pressure: self.assumed_silo_pressure.unwrap_or(base.assumed_silo_pressure),
            assumed_silo_patch: self.assumed_silo_patch.unwrap_or(base.assumed_silo_patch),
            assumed_silo_wall_friction: self.assumed_silo_wall_friction.unwrap_or(base.assumed_silo_wall_friction),
            bridge_tandem_x: self.bridge_tandem_x.unwrap_or(base.bridge_tandem_x),
            rail_alpha: self.rail_alpha.unwrap_or(base.rail_alpha),
            assumed_rail_lm71: self.assumed_rail_lm71.unwrap_or(base.assumed_rail_lm71),
            assumed_rail_lm71_udl: self.assumed_rail_lm71_udl.unwrap_or(base.assumed_rail_lm71_udl),
            assumed_rail_sw0: self.assumed_rail_sw0.unwrap_or(base.assumed_rail_sw0),
            assumed_rail_sw2: self.assumed_rail_sw2.unwrap_or(base.assumed_rail_sw2),
            crane_x: self.crane_x.unwrap_or(base.crane_x),
            crane_runway_length: self.crane_runway_length.unwrap_or(base.crane_runway_length),
            crane_wheel_base: self.crane_wheel_base.unwrap_or(base.crane_wheel_base),
            silo_diameter: self.silo_diameter.unwrap_or(base.silo_diameter),
            silo_eccentricity: self.silo_eccentricity.unwrap_or(base.silo_eccentricity),
            silo_hopper_angle_deg: self.silo_hopper_angle_deg.unwrap_or(base.silo_hopper_angle_deg),
            bridge_curve_radius: self.bridge_curve_radius.unwrap_or(base.bridge_curve_radius),
            fire_load_basis: self.fire_load_basis.unwrap_or(base.fire_load_basis),
            floors: self.floors.commit_onto(&base.floors, capability).map_err(|error| error.under(["floors"]))?,
            self_weight_elements: self.self_weight_elements.commit_onto(&base.self_weight_elements, capability).map_err(|error| error.under(["self_weight_elements"]))?,
            roofs: self.roofs.commit_onto(&base.roofs, capability).map_err(|error| error.under(["roofs"]))?,
            wind_faces: self.wind_faces.commit_onto(&base.wind_faces, capability).map_err(|error| error.under(["wind_faces"]))?,
            accidental_cases: self.accidental_cases.commit_onto(&base.accidental_cases, capability).map_err(|error| error.under(["accidental_cases"]))?,
            bridge_lanes: self.bridge_lanes.commit_onto(&base.bridge_lanes, capability).map_err(|error| error.under(["bridge_lanes"]))?,
            construction_stages: self.construction_stages.commit_onto(&base.construction_stages, capability).map_err(|error| error.under(["construction_stages"]))?,
            roof_surfaces: self.roof_surfaces.commit_onto(&base.roof_surfaces, capability).map_err(|error| error.under(["roof_surfaces"]))?,
            placements: self.placements.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?,
            special_vehicles: self.special_vehicles.commit_onto(&base.special_vehicles, capability).map_err(|error| error.under(["special_vehicles"]))?,
            wind_bridges: self.wind_bridges.commit_onto(&base.wind_bridges, capability).map_err(|error| error.under(["wind_bridges"]))?,
            dynamic_elements: self.dynamic_elements.commit_onto(&base.dynamic_elements, capability).map_err(|error| error.under(["dynamic_elements"]))?,
            impact_actions: self.impact_actions.commit_onto(&base.impact_actions, capability).map_err(|error| error.under(["impact_actions"]))?,
            internal_explosions: self.internal_explosions.commit_onto(&base.internal_explosions, capability).map_err(|error| error.under(["internal_explosions"]))?,
            thermal_sections: self.thermal_sections.commit_onto(&base.thermal_sections, capability).map_err(|error| error.under(["thermal_sections"]))?,
            cases: self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?,
            helicopter_landings: self.helicopter_landings.commit_onto(&base.helicopter_landings, capability).map_err(|error| error.under(["helicopter_landings"]))?,
            tie_runs: self.tie_runs.commit_onto(&base.tie_runs, capability).map_err(|error| error.under(["tie_runs"]))?,
            construction_loads: self.construction_loads.commit_onto(&base.construction_loads, capability).map_err(|error| error.under(["construction_loads"]))?,
            falling_loads: self.falling_loads.commit_onto(&base.falling_loads, capability).map_err(|error| error.under(["falling_loads"]))?,
            execution_imperfections: self.execution_imperfections.commit_onto(&base.execution_imperfections, capability).map_err(|error| error.under(["execution_imperfections"]))?,
            derailment_cases: self.derailment_cases.commit_onto(&base.derailment_cases, capability).map_err(|error| error.under(["derailment_cases"]))?,
            crane_hoists: self.crane_hoists.commit_onto(&base.crane_hoists, capability).map_err(|error| error.under(["crane_hoists"]))?,
            footbridges: self.footbridges.commit_onto(&base.footbridges, capability).map_err(|error| error.under(["footbridges"]))?,
            rail_dynamics: self.rail_dynamics.commit_onto(&base.rail_dynamics, capability).map_err(|error| error.under(["rail_dynamics"]))?,
            load_dispersals: self.load_dispersals.commit_onto(&base.load_dispersals, capability).map_err(|error| error.under(["load_dispersals"]))?,
            bulk_solids: self.bulk_solids.commit_onto(&base.bulk_solids, capability).map_err(|error| error.under(["bulk_solids"]))?,
            rotating_machines: self.rotating_machines.commit_onto(&base.rotating_machines, capability).map_err(|error| error.under(["rotating_machines"]))?,
            dust_explosions: self.dust_explosions.commit_onto(&base.dust_explosions, capability).map_err(|error| error.under(["dust_explosions"]))?,
            rail_load_groups: self.rail_load_groups.commit_onto(&base.rail_load_groups, capability).map_err(|error| error.under(["rail_load_groups"]))?,
            rail_derailments: self.rail_derailments.commit_onto(&base.rail_derailments, capability).map_err(|error| error.under(["rail_derailments"]))?,
            fatigue_models: self.fatigue_models.commit_onto(&base.fatigue_models, capability).map_err(|error| error.under(["fatigue_models"]))?,
            support_collisions: self.support_collisions.commit_onto(&base.support_collisions, capability).map_err(|error| error.under(["support_collisions"]))?,
            local_fires: self.local_fires.commit_onto(&base.local_fires, capability).map_err(|error| error.under(["local_fires"]))?,
            snow_guards: self.snow_guards.commit_onto(&base.snow_guards, capability).map_err(|error| error.under(["snow_guards"]))?,
        };
        Ok(())
    }
}

impl DiffAlgebra<En1991Snapshot> for En1991Diff {
    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.snow_zone.is_none()
            && self.altitude.is_none()
            && self.en_sk.is_none()
            && self.north_german_lowland_snow.is_none()
            && self.wind_zone.is_none()
            && self.en_vb.is_none()
            && self.terrain_category.is_none()
            && self.mixed_terrain_upwind.is_none()
            && self.mixed_terrain_distance.is_none()
            && self.orography_factor.is_none()
            && self.coast_or_island.is_none()
            && self.air_density.is_none()
            && self.height.is_none()
            && self.width.is_none()
            && self.depth.is_none()
            && self.assumed_delta_t.is_none()
            && self.t_max.is_none()
            && self.t_min.is_none()
            && self.t_0.is_none()
            && self.thermal_element_type.is_none()
            && self.thermal_bridge_type.is_none()
            && self.delta_t_m.is_none()
            && self.storey_count.is_none()
            && self.fire_mode.is_none()
            && self.fire_curve.is_none()
            && self.fire_duration.is_none()
            && self.assumed_gas_temperature.is_none()
            && self.assumed_h_net.is_none()
            && self.fire_compartment_area.is_none()
            && self.fire_compartment_height.is_none()
            && self.fire_opening_factor.is_none()
            && self.fire_thermal_inertia.is_none()
            && self.fire_occupancy.is_none()
            && self.fire_load_density_qf.is_none()
            && self.assumed_qf_d.is_none()
            && self.construction_activity.is_none()
            && self.assumed_construction_qk.is_none()
            && self.structure_kind.is_none()
            && self.bridge_lane.is_none()
            && self.bridge_span.is_none()
            && self.bridge_lane_width.is_none()
            && self.assumed_bridge_tandem.is_none()
            && self.assumed_bridge_udl.is_none()
            && self.assumed_bridge_lm2.is_none()
            && self.assumed_bridge_footway.is_none()
            && self.assumed_bridge_lm3.is_none()
            && self.assumed_bridge_lm4.is_none()
            && self.bridge_load_group.is_none()
            && self.crane_claimed.is_none()
            && self.crane_class.is_none()
            && self.hoist_class.is_none()
            && self.hoisting_speed.is_none()
            && self.assumed_crane_wheel.is_none()
            && self.assumed_crane_horizontal.is_none()
            && self.silo_claimed.is_none()
            && self.silo_kind.is_none()
            && self.silo_bulk_density.is_none()
            && self.silo_height.is_none()
            && self.silo_hydraulic_radius.is_none()
            && self.silo_mu.is_none()
            && self.silo_k.is_none()
            && self.assumed_silo_pressure.is_none()
            && self.assumed_silo_patch.is_none()
            && self.assumed_silo_wall_friction.is_none()
            && self.bridge_tandem_x.is_none()
            && self.rail_alpha.is_none()
            && self.assumed_rail_lm71.is_none()
            && self.assumed_rail_lm71_udl.is_none()
            && self.assumed_rail_sw0.is_none()
            && self.assumed_rail_sw2.is_none()
            && self.crane_x.is_none()
            && self.crane_runway_length.is_none()
            && self.crane_wheel_base.is_none()
            && self.silo_diameter.is_none()
            && self.silo_eccentricity.is_none()
            && self.silo_hopper_angle_deg.is_none()
            && self.bridge_curve_radius.is_none()
            && self.fire_load_basis.is_none()
            && self.floors.is_empty()
            && self.self_weight_elements.is_empty()
            && self.roofs.is_empty()
            && self.wind_faces.is_empty()
            && self.accidental_cases.is_empty()
            && self.bridge_lanes.is_empty()
            && self.construction_stages.is_empty()
            && self.roof_surfaces.is_empty()
            && self.placements.is_empty()
            && self.special_vehicles.is_empty()
            && self.wind_bridges.is_empty()
            && self.dynamic_elements.is_empty()
            && self.impact_actions.is_empty()
            && self.internal_explosions.is_empty()
            && self.thermal_sections.is_empty()
            && self.cases.is_empty()
            && self.helicopter_landings.is_empty()
            && self.tie_runs.is_empty()
            && self.construction_loads.is_empty()
            && self.falling_loads.is_empty()
            && self.execution_imperfections.is_empty()
            && self.derailment_cases.is_empty()
            && self.crane_hoists.is_empty()
            && self.footbridges.is_empty()
            && self.rail_dynamics.is_empty()
            && self.load_dispersals.is_empty()
            && self.bulk_solids.is_empty()
            && self.rotating_machines.is_empty()
            && self.dust_explosions.is_empty()
            && self.rail_load_groups.is_empty()
            && self.rail_derailments.is_empty()
            && self.fatigue_models.is_empty()
            && self.support_collisions.is_empty()
            && self.local_fires.is_empty()
            && self.snow_guards.is_empty()
    }
}

impl protocol::ColdDiffAlgebra<En1991Snapshot> for En1991Diff {
    fn inverse(&self, base: &En1991Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex),
            snow_zone: self.snow_zone.as_ref().map(|_| base.snow_zone.clone()),
            altitude: self.altitude.as_ref().map(|_| base.altitude),
            en_sk: self.en_sk.as_ref().map(|_| base.en_sk),
            north_german_lowland_snow: self.north_german_lowland_snow.as_ref().map(|_| base.north_german_lowland_snow),
            wind_zone: self.wind_zone.as_ref().map(|_| base.wind_zone),
            en_vb: self.en_vb.as_ref().map(|_| base.en_vb),
            terrain_category: self.terrain_category.as_ref().map(|_| base.terrain_category),
            mixed_terrain_upwind: self.mixed_terrain_upwind.as_ref().map(|_| base.mixed_terrain_upwind),
            mixed_terrain_distance: self.mixed_terrain_distance.as_ref().map(|_| base.mixed_terrain_distance),
            orography_factor: self.orography_factor.as_ref().map(|_| base.orography_factor),
            coast_or_island: self.coast_or_island.as_ref().map(|_| base.coast_or_island),
            air_density: self.air_density.as_ref().map(|_| base.air_density),
            height: self.height.as_ref().map(|_| base.height),
            width: self.width.as_ref().map(|_| base.width),
            depth: self.depth.as_ref().map(|_| base.depth),
            assumed_delta_t: self.assumed_delta_t.as_ref().map(|_| base.assumed_delta_t),
            t_max: self.t_max.as_ref().map(|_| base.t_max),
            t_min: self.t_min.as_ref().map(|_| base.t_min),
            t_0: self.t_0.as_ref().map(|_| base.t_0),
            thermal_element_type: self.thermal_element_type.as_ref().map(|_| base.thermal_element_type.clone()),
            thermal_bridge_type: self.thermal_bridge_type.as_ref().map(|_| base.thermal_bridge_type),
            delta_t_m: self.delta_t_m.as_ref().map(|_| base.delta_t_m),
            storey_count: self.storey_count.as_ref().map(|_| base.storey_count),
            fire_mode: self.fire_mode.as_ref().map(|_| base.fire_mode),
            fire_curve: self.fire_curve.as_ref().map(|_| base.fire_curve),
            fire_duration: self.fire_duration.as_ref().map(|_| base.fire_duration),
            assumed_gas_temperature: self.assumed_gas_temperature.as_ref().map(|_| base.assumed_gas_temperature),
            assumed_h_net: self.assumed_h_net.as_ref().map(|_| base.assumed_h_net),
            fire_compartment_area: self.fire_compartment_area.as_ref().map(|_| base.fire_compartment_area),
            fire_compartment_height: self.fire_compartment_height.as_ref().map(|_| base.fire_compartment_height),
            fire_opening_factor: self.fire_opening_factor.as_ref().map(|_| base.fire_opening_factor),
            fire_thermal_inertia: self.fire_thermal_inertia.as_ref().map(|_| base.fire_thermal_inertia),
            fire_occupancy: self.fire_occupancy.as_ref().map(|_| base.fire_occupancy.clone()),
            fire_load_density_qf: self.fire_load_density_qf.as_ref().map(|_| base.fire_load_density_qf),
            assumed_qf_d: self.assumed_qf_d.as_ref().map(|_| base.assumed_qf_d),
            construction_activity: self.construction_activity.as_ref().map(|_| base.construction_activity.clone()),
            assumed_construction_qk: self.assumed_construction_qk.as_ref().map(|_| base.assumed_construction_qk),
            structure_kind: self.structure_kind.as_ref().map(|_| base.structure_kind),
            bridge_lane: self.bridge_lane.as_ref().map(|_| base.bridge_lane),
            bridge_span: self.bridge_span.as_ref().map(|_| base.bridge_span),
            bridge_lane_width: self.bridge_lane_width.as_ref().map(|_| base.bridge_lane_width),
            assumed_bridge_tandem: self.assumed_bridge_tandem.as_ref().map(|_| base.assumed_bridge_tandem),
            assumed_bridge_udl: self.assumed_bridge_udl.as_ref().map(|_| base.assumed_bridge_udl),
            assumed_bridge_lm2: self.assumed_bridge_lm2.as_ref().map(|_| base.assumed_bridge_lm2),
            assumed_bridge_footway: self.assumed_bridge_footway.as_ref().map(|_| base.assumed_bridge_footway),
            assumed_bridge_lm3: self.assumed_bridge_lm3.as_ref().map(|_| base.assumed_bridge_lm3),
            assumed_bridge_lm4: self.assumed_bridge_lm4.as_ref().map(|_| base.assumed_bridge_lm4),
            bridge_load_group: self.bridge_load_group.as_ref().map(|_| base.bridge_load_group.clone()),
            crane_claimed: self.crane_claimed.as_ref().map(|_| base.crane_claimed),
            crane_class: self.crane_class.as_ref().map(|_| base.crane_class.clone()),
            hoist_class: self.hoist_class.as_ref().map(|_| base.hoist_class.clone()),
            hoisting_speed: self.hoisting_speed.as_ref().map(|_| base.hoisting_speed),
            assumed_crane_wheel: self.assumed_crane_wheel.as_ref().map(|_| base.assumed_crane_wheel),
            assumed_crane_horizontal: self.assumed_crane_horizontal.as_ref().map(|_| base.assumed_crane_horizontal),
            silo_claimed: self.silo_claimed.as_ref().map(|_| base.silo_claimed),
            silo_kind: self.silo_kind.as_ref().map(|_| base.silo_kind.clone()),
            silo_bulk_density: self.silo_bulk_density.as_ref().map(|_| base.silo_bulk_density),
            silo_height: self.silo_height.as_ref().map(|_| base.silo_height),
            silo_hydraulic_radius: self.silo_hydraulic_radius.as_ref().map(|_| base.silo_hydraulic_radius),
            silo_mu: self.silo_mu.as_ref().map(|_| base.silo_mu),
            silo_k: self.silo_k.as_ref().map(|_| base.silo_k),
            assumed_silo_pressure: self.assumed_silo_pressure.as_ref().map(|_| base.assumed_silo_pressure),
            assumed_silo_patch: self.assumed_silo_patch.as_ref().map(|_| base.assumed_silo_patch),
            assumed_silo_wall_friction: self.assumed_silo_wall_friction.as_ref().map(|_| base.assumed_silo_wall_friction),
            bridge_tandem_x: self.bridge_tandem_x.as_ref().map(|_| base.bridge_tandem_x),
            rail_alpha: self.rail_alpha.as_ref().map(|_| base.rail_alpha),
            assumed_rail_lm71: self.assumed_rail_lm71.as_ref().map(|_| base.assumed_rail_lm71),
            assumed_rail_lm71_udl: self.assumed_rail_lm71_udl.as_ref().map(|_| base.assumed_rail_lm71_udl),
            assumed_rail_sw0: self.assumed_rail_sw0.as_ref().map(|_| base.assumed_rail_sw0),
            assumed_rail_sw2: self.assumed_rail_sw2.as_ref().map(|_| base.assumed_rail_sw2),
            crane_x: self.crane_x.as_ref().map(|_| base.crane_x),
            crane_runway_length: self.crane_runway_length.as_ref().map(|_| base.crane_runway_length),
            crane_wheel_base: self.crane_wheel_base.as_ref().map(|_| base.crane_wheel_base),
            silo_diameter: self.silo_diameter.as_ref().map(|_| base.silo_diameter),
            silo_eccentricity: self.silo_eccentricity.as_ref().map(|_| base.silo_eccentricity),
            silo_hopper_angle_deg: self.silo_hopper_angle_deg.as_ref().map(|_| base.silo_hopper_angle_deg),
            bridge_curve_radius: self.bridge_curve_radius.as_ref().map(|_| base.bridge_curve_radius),
            fire_load_basis: self.fire_load_basis.as_ref().map(|_| base.fire_load_basis),
            floors: self.floors.inverse(&base.floors),
            self_weight_elements: self.self_weight_elements.inverse(&base.self_weight_elements),
            roofs: self.roofs.inverse(&base.roofs),
            wind_faces: self.wind_faces.inverse(&base.wind_faces),
            accidental_cases: self.accidental_cases.inverse(&base.accidental_cases),
            bridge_lanes: self.bridge_lanes.inverse(&base.bridge_lanes),
            construction_stages: self.construction_stages.inverse(&base.construction_stages),
            roof_surfaces: self.roof_surfaces.inverse(&base.roof_surfaces),
            placements: self.placements.inverse(&base.placements),
            special_vehicles: self.special_vehicles.inverse(&base.special_vehicles),
            wind_bridges: self.wind_bridges.inverse(&base.wind_bridges),
            dynamic_elements: self.dynamic_elements.inverse(&base.dynamic_elements),
            impact_actions: self.impact_actions.inverse(&base.impact_actions),
            internal_explosions: self.internal_explosions.inverse(&base.internal_explosions),
            thermal_sections: self.thermal_sections.inverse(&base.thermal_sections),
            cases: self.cases.inverse(&base.cases),
            helicopter_landings: self.helicopter_landings.inverse(&base.helicopter_landings),
            tie_runs: self.tie_runs.inverse(&base.tie_runs),
            construction_loads: self.construction_loads.inverse(&base.construction_loads),
            falling_loads: self.falling_loads.inverse(&base.falling_loads),
            execution_imperfections: self.execution_imperfections.inverse(&base.execution_imperfections),
            derailment_cases: self.derailment_cases.inverse(&base.derailment_cases),
            crane_hoists: self.crane_hoists.inverse(&base.crane_hoists),
            footbridges: self.footbridges.inverse(&base.footbridges),
            rail_dynamics: self.rail_dynamics.inverse(&base.rail_dynamics),
            load_dispersals: self.load_dispersals.inverse(&base.load_dispersals),
            bulk_solids: self.bulk_solids.inverse(&base.bulk_solids),
            rotating_machines: self.rotating_machines.inverse(&base.rotating_machines),
            dust_explosions: self.dust_explosions.inverse(&base.dust_explosions),
            rail_load_groups: self.rail_load_groups.inverse(&base.rail_load_groups),
            rail_derailments: self.rail_derailments.inverse(&base.rail_derailments),
            fatigue_models: self.fatigue_models.inverse(&base.fatigue_models),
            support_collisions: self.support_collisions.inverse(&base.support_collisions),
            local_fires: self.local_fires.inverse(&base.local_fires),
            snow_guards: self.snow_guards.inverse(&base.snow_guards),
        }
    }
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `diff` schema facet.
pub const DIFF_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: semio_s_artifact_norm_en1991_diff_typescript::LEAF, graphql: semio_s_artifact_norm_en1991_diff_graphql::LEAF, json_schema: semio_s_artifact_norm_en1991_diff_json_schema::LEAF, proto: semio_s_artifact_norm_en1991_diff_proto::LEAF };
//#endregion 🍃️FacetLeaves

#[path = "../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
pub mod text;

#[path = "../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
pub mod binary;
