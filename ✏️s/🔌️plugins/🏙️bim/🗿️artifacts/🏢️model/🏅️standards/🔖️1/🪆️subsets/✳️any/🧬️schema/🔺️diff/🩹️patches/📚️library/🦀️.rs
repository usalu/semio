//! 📚️ Sparse patches of the library records: property templates, classification systems, space conditions, families, components, MEP elements, materials and the element types.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::entities::*;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::*;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::*;

#[path = "🏷️templates-and-families/🦀️.rs"]
mod templates_and_families;
pub use templates_and_families::{
    ClassificationSystemPatch, FamilyParameterPatch, FamilyPatch, FamilySolidPatch,
    PropertyTemplatePatch, SpaceConditionsPatch,
};

#[path = "🧱️components-and-materials/🦀️.rs"]
mod components_and_materials;
pub use components_and_materials::{
    ComponentOverridePatch, ComponentPatch, MaterialPatch, MepElementPatch,
};

#[path = "🚪️element-types/🦀️.rs"]
mod element_types;
pub use element_types::{
    BeamTypePatch, CeilingTypePatch, ColumnTypePatch, CurtainWallTypePatch, DoorTypePatch,
    RoofTypePatch, SlabTypePatch, WallTypePatch, WindowTypePatch,
};
