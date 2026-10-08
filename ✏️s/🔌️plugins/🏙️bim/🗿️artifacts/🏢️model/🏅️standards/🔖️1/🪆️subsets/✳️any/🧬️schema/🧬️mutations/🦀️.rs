//! 🧬️ BIM model semantic mutation dispatch enum.
//!
//! `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<ModelSnapshot>` and `impl protocol::SemanticMutation<ModelSnapshot>`
//! for [`ModelMutation`] by delegating each variant to its payload's `protocol::MutationKind` impl; the handcrafted logic lives in
//! the triad leaves (`<slug>/{🦠️mutation,🔺️diff,↩️inverse}`). This file is dispatch-only.

use crate::{ModelDiff, ModelSnapshot};

#[cfg(test)]
#[path = "🧪️tests/🧰️kit/🦀️.rs"]
pub mod kit;

#[path = "🧵️elements/🦀️.rs"]
pub mod elements;

#[path = "🌊️cascade/🦀️.rs"]
pub mod cascade;

#[path = "📍️placement/🦀️.rs"]
pub mod placement;

//#region 🔖️Operations
/// 🧬️ Every variant wraps exactly one `protocol::MutationKind<ModelSnapshot, ModelMutation>` payload struct declared in the
/// corresponding leaf's `🦠️mutation/🦀️.rs`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = ModelSnapshot, diff = ModelDiff, schema = "bim.model")]
pub enum ModelMutation {
    SetCurtainWall(super::set_curtain_wall::SetCurtainWall),
    DeleteCurtainWall(super::delete_curtain_wall::DeleteCurtainWall),
    CreateCurtainWall(super::create_curtain_wall::CreateCurtainWall),
    SplitWall(super::split_wall::SplitWall),
    FlipWall(super::flip_wall::FlipWall),
    SetWallLocation(super::set_wall_location::SetWallLocation),
    SetWallTypeOf(super::set_wall_type_of::SetWallTypeOf),
    SetWallBaseOffset(super::set_wall_base_offset::SetWallBaseOffset),
    SetWallAxis(super::set_wall_axis::SetWallAxis),
    CreateSite(super::create_site::CreateSite),
    DeleteSite(super::delete_site::DeleteSite),
    CreateBuilding(super::create_building::CreateBuilding),
    DeleteBuilding(super::delete_building::DeleteBuilding),
    CreateStorey(super::create_storey::CreateStorey),
    RenameStorey(super::rename_storey::RenameStorey),
    SetStoreyHeight(super::set_storey_height::SetStoreyHeight),
    SetStoreyCutHeight(super::set_storey_cut_height::SetStoreyCutHeight),
    SetStoreyLevel(super::set_storey_level::SetStoreyLevel),
    DeleteStorey(super::delete_storey::DeleteStorey),
    CreateWall(super::create_wall::CreateWall),
    DeleteWall(super::delete_wall::DeleteWall),
    SetWallTop(super::set_wall_top::SetWallTop),
    SetProjectInfo(super::set_project_info::SetProjectInfo),
    SetSite(super::set_site::SetSite),
    SetBuilding(super::set_building::SetBuilding),
    CreateGridLine(super::create_grid_line::CreateGridLine),
    DeleteGridLine(super::delete_grid_line::DeleteGridLine),
    SetGridLine(super::set_grid_line::SetGridLine),
    CreateColumnType(super::create_column_type::CreateColumnType),
    DeleteColumnType(super::delete_column_type::DeleteColumnType),
    SetColumnType(super::set_column_type::SetColumnType),
    CreateBeamType(super::create_beam_type::CreateBeamType),
    DeleteBeamType(super::delete_beam_type::DeleteBeamType),
    SetBeamType(super::set_beam_type::SetBeamType),
    CreateWindowType(super::create_window_type::CreateWindowType),
    DeleteWindowType(super::delete_window_type::DeleteWindowType),
    SetWindowType(super::set_window_type::SetWindowType),
    CreateDoorType(super::create_door_type::CreateDoorType),
    DeleteDoorType(super::delete_door_type::DeleteDoorType),
    SetDoorType(super::set_door_type::SetDoorType),
    CreateMaterial(super::create_material::CreateMaterial),
    DeleteMaterial(super::delete_material::DeleteMaterial),
    SetMaterial(super::set_material::SetMaterial),
    CreateWallType(super::create_wall_type::CreateWallType),
    DeleteWallType(super::delete_wall_type::DeleteWallType),
    SetWallType(super::set_wall_type::SetWallType),
    CreateSlabType(super::create_slab_type::CreateSlabType),
    DeleteSlabType(super::delete_slab_type::DeleteSlabType),
    SetSlabType(super::set_slab_type::SetSlabType),
    CreateRoofType(super::create_roof_type::CreateRoofType),
    DeleteRoofType(super::delete_roof_type::DeleteRoofType),
    SetRoofType(super::set_roof_type::SetRoofType),
    CreateColumn(super::create_column::CreateColumn),
    DeleteColumn(super::delete_column::DeleteColumn),
    SetColumn(super::set_column::SetColumn),
    CreateBeam(super::create_beam::CreateBeam),
    DeleteBeam(super::delete_beam::DeleteBeam),
    SetBeam(super::set_beam::SetBeam),
    CreateRailing(super::create_railing::CreateRailing),
    DeleteRailing(super::delete_railing::DeleteRailing),
    SetRailing(super::set_railing::SetRailing),
    CreateSpace(super::create_space::CreateSpace),
    DeleteSpace(super::delete_space::DeleteSpace),
    SetSpace(super::set_space::SetSpace),
    CreateSlab(super::create_slab::CreateSlab),
    DeleteSlab(super::delete_slab::DeleteSlab),
    SetSlabBoundary(super::set_slab_boundary::SetSlabBoundary),
    SetSlab(super::set_slab::SetSlab),
    CreateRoof(super::create_roof::CreateRoof),
    DeleteRoof(super::delete_roof::DeleteRoof),
    SetRoofFootprint(super::set_roof_footprint::SetRoofFootprint),
    SetRoofShape(super::set_roof_shape::SetRoofShape),
    CreateOpening(super::create_opening::CreateOpening),
    DeleteOpening(super::delete_opening::DeleteOpening),
    MoveOpening(super::move_opening::MoveOpening),
    SetOpening(super::set_opening::SetOpening),
    CreateStair(super::create_stair::CreateStair),
    DeleteStair(super::delete_stair::DeleteStair),
    SetStair(super::set_stair::SetStair),
    MoveElements(super::move_elements::MoveElements),
    RotateElements(super::rotate_elements::RotateElements),
    PlaceElements(super::place_elements::PlaceElements),
    DeleteElements(super::delete_elements::DeleteElements),
    RenameElement(super::rename_element::RenameElement),
    SetElementProperty(super::set_element_property::SetElementProperty),
    RemoveElementProperty(super::remove_element_property::RemoveElementProperty),
    SetElementClassification(super::set_element_classification::SetElementClassification),
    RemoveElementClassification(super::remove_element_classification::RemoveElementClassification),
}

/// 🏷️ The kebab spelling of every [`ModelMutation`] variant, in declaration order: the one list the language-neutral test platform
/// is measured against (this subset's oracle catalog and the `mutate-model-1` adapter repeat it on purpose).
pub const KINDS: &[&str] = &[
    "set-curtain-wall",
    "delete-curtain-wall",
    "create-curtain-wall",
    "split-wall",
    "flip-wall",
    "set-wall-location",
    "set-wall-type-of",
    "set-wall-base-offset",
    "set-wall-axis",
    "create-site",
    "delete-site",
    "create-building",
    "delete-building",
    "create-storey",
    "rename-storey",
    "set-storey-height",
    "set-storey-cut-height",
    "set-storey-level",
    "delete-storey",
    "create-wall",
    "delete-wall",
    "set-wall-top",
    "set-project-info",
    "set-site",
    "set-building",
    "create-grid-line",
    "delete-grid-line",
    "set-grid-line",
    "create-column-type",
    "delete-column-type",
    "set-column-type",
    "create-beam-type",
    "delete-beam-type",
    "set-beam-type",
    "create-window-type",
    "delete-window-type",
    "set-window-type",
    "create-door-type",
    "delete-door-type",
    "set-door-type",
    "create-material",
    "delete-material",
    "set-material",
    "create-wall-type",
    "delete-wall-type",
    "set-wall-type",
    "create-slab-type",
    "delete-slab-type",
    "set-slab-type",
    "create-roof-type",
    "delete-roof-type",
    "set-roof-type",
    "create-column",
    "delete-column",
    "set-column",
    "create-beam",
    "delete-beam",
    "set-beam",
    "create-railing",
    "delete-railing",
    "set-railing",
    "create-space",
    "delete-space",
    "set-space",
    "create-slab",
    "delete-slab",
    "set-slab-boundary",
    "set-slab",
    "create-roof",
    "delete-roof",
    "set-roof-footprint",
    "set-roof-shape",
    "create-opening",
    "delete-opening",
    "move-opening",
    "set-opening",
    "create-stair",
    "delete-stair",
    "set-stair",
    "move-elements",
    "rotate-elements",
    "place-elements",
    "delete-elements",
    "rename-element",
    "set-element-property",
    "remove-element-property",
    "set-element-classification",
    "remove-element-classification",
];
//#endregion 🔖️Operations

//#region 🔖️Apply
/// 📦️ Applies `mutation` onto `snapshot`, returning the resulting snapshot, through the central applier.
pub fn apply_model_mutation(snapshot: &ModelSnapshot, mutation: &ModelMutation) -> protocol::MutationApplyResult<ModelSnapshot> {
    store::apply_mutation(snapshot, mutation).map(|(next, _messages)| next)
}

/// ↩️ Computes `mutation`'s concrete inverse mutations against `snapshot` (pre-state), in storage order.
pub fn inverse_model_mutation(snapshot: &ModelSnapshot, mutation: &ModelMutation) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
    <ModelMutation as protocol::Mutation<ModelSnapshot>>::inverse(mutation, snapshot)
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
