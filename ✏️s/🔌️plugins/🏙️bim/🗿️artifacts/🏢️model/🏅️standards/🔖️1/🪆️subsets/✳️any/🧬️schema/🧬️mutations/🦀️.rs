//! 🧬️ BIM model semantic mutation dispatch enum.
//!
//! `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<ModelSnapshot>` and `impl protocol::SemanticMutation<ModelSnapshot>`
//! for [`ModelMutation`] by delegating each variant to its payload's `protocol::MutationKind` impl; the handcrafted logic lives in
//! the triad leaves (`<slug>/{🦠️mutation,🔺️diff,↩️inverse}`). This file is dispatch-only.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};

#[cfg(test)]
#[path = "🧪️tests/🧰️kit/🦀️.rs"]
pub mod kit;

#[cfg(test)]
#[path = "🧪️tests/🗂️fixture-cases/🦀️.rs"]
mod fixture_cases;

pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements;

pub use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade;


pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::mutations::annotating;

pub use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::mutations::family_rules;

pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::component_rules;

pub use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::placement;

pub use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_depth;


//#region 🔖️Operations
/// 🧬️ Every variant wraps exactly one `protocol::MutationKind<ModelSnapshot, ModelMutation>` payload struct declared in the
/// corresponding leaf's `🦠️mutation/🦀️.rs`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = ModelSnapshot, diff = ModelDiff, schema = "bim.model")]
pub enum ModelMutation {
    SetElementWorkset(crate::standards::v1::subsets::any::schema::mutations::set_element_workset::SetElementWorkset),
    SetElementOption(crate::standards::v1::subsets::any::schema::mutations::set_element_option::SetElementOption),
    DeleteWorkset(crate::standards::v1::subsets::any::schema::mutations::delete_workset::DeleteWorkset),
    SetWorkset(crate::standards::v1::subsets::any::schema::mutations::set_workset::SetWorkset),
    CreateWorkset(crate::standards::v1::subsets::any::schema::mutations::create_workset::CreateWorkset),
    DeleteDesignOption(crate::standards::v1::subsets::any::schema::mutations::delete_design_option::DeleteDesignOption),
    SetDesignOption(crate::standards::v1::subsets::any::schema::mutations::set_design_option::SetDesignOption),
    CreateDesignOption(crate::standards::v1::subsets::any::schema::mutations::create_design_option::CreateDesignOption),
    DeleteOptionGroup(crate::standards::v1::subsets::any::schema::mutations::delete_option_group::DeleteOptionGroup),
    SetOptionGroup(crate::standards::v1::subsets::any::schema::mutations::set_option_group::SetOptionGroup),
    CreateOptionGroup(crate::standards::v1::subsets::any::schema::mutations::create_option_group::CreateOptionGroup),
    CreateSupport(crate::standards::v1::subsets::any::schema::mutations::create_support::CreateSupport),
    SetSupport(crate::standards::v1::subsets::any::schema::mutations::set_support::SetSupport),
    DeleteSupport(crate::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport),
    CreateLoadCase(crate::standards::v1::subsets::any::schema::mutations::create_load_case::CreateLoadCase),
    SetLoadCase(crate::standards::v1::subsets::any::schema::mutations::set_load_case::SetLoadCase),
    DeleteLoadCase(crate::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase),
    CreateLoad(crate::standards::v1::subsets::any::schema::mutations::create_load::CreateLoad),
    SetLoad(crate::standards::v1::subsets::any::schema::mutations::set_load::SetLoad),
    DeleteLoad(crate::standards::v1::subsets::any::schema::mutations::delete_load::DeleteLoad),

    SetCurtainWall(crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall::SetCurtainWall),
    DeleteCurtainWall(crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall::DeleteCurtainWall),
    CreateCurtainWall(crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall::CreateCurtainWall),
    SplitWall(crate::standards::v1::subsets::any::schema::mutations::split_wall::SplitWall),
    FlipWall(crate::standards::v1::subsets::any::schema::mutations::flip_wall::FlipWall),
    SetWallLocation(crate::standards::v1::subsets::any::schema::mutations::set_wall_location::SetWallLocation),
    SetWallTypeOf(crate::standards::v1::subsets::any::schema::mutations::set_wall_type_of::SetWallTypeOf),
    SetWallBaseOffset(crate::standards::v1::subsets::any::schema::mutations::set_wall_base_offset::SetWallBaseOffset),
    SetWallAxis(crate::standards::v1::subsets::any::schema::mutations::set_wall_axis::SetWallAxis),
    CreateSite(crate::standards::v1::subsets::any::schema::mutations::create_site::CreateSite),
    DeleteSite(crate::standards::v1::subsets::any::schema::mutations::delete_site::DeleteSite),
    CreateBuilding(crate::standards::v1::subsets::any::schema::mutations::create_building::CreateBuilding),
    DeleteBuilding(crate::standards::v1::subsets::any::schema::mutations::delete_building::DeleteBuilding),
    CreateStorey(crate::standards::v1::subsets::any::schema::mutations::create_storey::CreateStorey),
    RenameStorey(crate::standards::v1::subsets::any::schema::mutations::rename_storey::RenameStorey),
    SetStoreyHeight(crate::standards::v1::subsets::any::schema::mutations::set_storey_height::SetStoreyHeight),
    SetStoreyCutHeight(crate::standards::v1::subsets::any::schema::mutations::set_storey_cut_height::SetStoreyCutHeight),
    SetElementPhase(crate::standards::v1::subsets::any::schema::mutations::set_element_phase::SetElementPhase),
    SetElementStorey(crate::standards::v1::subsets::any::schema::mutations::set_element_storey::SetElementStorey),
    SetStoreyLevel(crate::standards::v1::subsets::any::schema::mutations::set_storey_level::SetStoreyLevel),
    DeleteStorey(crate::standards::v1::subsets::any::schema::mutations::delete_storey::DeleteStorey),
    CreateWall(crate::standards::v1::subsets::any::schema::mutations::create_wall::CreateWall),
    DeleteWall(crate::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall),
    SetWallTop(crate::standards::v1::subsets::any::schema::mutations::set_wall_top::SetWallTop),
    SetProjectInfo(crate::standards::v1::subsets::any::schema::mutations::set_project_info::SetProjectInfo),
    SetSite(crate::standards::v1::subsets::any::schema::mutations::set_site::SetSite),
    SetBuilding(crate::standards::v1::subsets::any::schema::mutations::set_building::SetBuilding),
    CreateGridLine(crate::standards::v1::subsets::any::schema::mutations::create_grid_line::CreateGridLine),
    DeleteGridLine(crate::standards::v1::subsets::any::schema::mutations::delete_grid_line::DeleteGridLine),
    SetGridLine(crate::standards::v1::subsets::any::schema::mutations::set_grid_line::SetGridLine),
    CreateColumnType(crate::standards::v1::subsets::any::schema::mutations::create_column_type::CreateColumnType),
    DeleteColumnType(crate::standards::v1::subsets::any::schema::mutations::delete_column_type::DeleteColumnType),
    SetColumnType(crate::standards::v1::subsets::any::schema::mutations::set_column_type::SetColumnType),
    CreateBeamType(crate::standards::v1::subsets::any::schema::mutations::create_beam_type::CreateBeamType),
    DeleteBeamType(crate::standards::v1::subsets::any::schema::mutations::delete_beam_type::DeleteBeamType),
    SetBeamType(crate::standards::v1::subsets::any::schema::mutations::set_beam_type::SetBeamType),
    CreateWindowType(crate::standards::v1::subsets::any::schema::mutations::create_window_type::CreateWindowType),
    DeleteWindowType(crate::standards::v1::subsets::any::schema::mutations::delete_window_type::DeleteWindowType),
    SetWindowType(crate::standards::v1::subsets::any::schema::mutations::set_window_type::SetWindowType),
    CreateDoorType(crate::standards::v1::subsets::any::schema::mutations::create_door_type::CreateDoorType),
    DeleteDoorType(crate::standards::v1::subsets::any::schema::mutations::delete_door_type::DeleteDoorType),
    SetDoorType(crate::standards::v1::subsets::any::schema::mutations::set_door_type::SetDoorType),
    CreateMaterial(crate::standards::v1::subsets::any::schema::mutations::create_material::CreateMaterial),
    DeleteMaterial(crate::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial),
    SetMaterial(crate::standards::v1::subsets::any::schema::mutations::set_material::SetMaterial),
    CreateWallType(crate::standards::v1::subsets::any::schema::mutations::create_wall_type::CreateWallType),
    DeleteWallType(crate::standards::v1::subsets::any::schema::mutations::delete_wall_type::DeleteWallType),
    SetWallType(crate::standards::v1::subsets::any::schema::mutations::set_wall_type::SetWallType),
    CreateSlabType(crate::standards::v1::subsets::any::schema::mutations::create_slab_type::CreateSlabType),
    DeleteSlabType(crate::standards::v1::subsets::any::schema::mutations::delete_slab_type::DeleteSlabType),
    SetSlabType(crate::standards::v1::subsets::any::schema::mutations::set_slab_type::SetSlabType),
    CreateRoofType(crate::standards::v1::subsets::any::schema::mutations::create_roof_type::CreateRoofType),
    DeleteRoofType(crate::standards::v1::subsets::any::schema::mutations::delete_roof_type::DeleteRoofType),
    SetRoofType(crate::standards::v1::subsets::any::schema::mutations::set_roof_type::SetRoofType),
    CreateColumn(crate::standards::v1::subsets::any::schema::mutations::create_column::CreateColumn),
    DeleteColumn(crate::standards::v1::subsets::any::schema::mutations::delete_column::DeleteColumn),
    SetColumn(crate::standards::v1::subsets::any::schema::mutations::set_column::SetColumn),
    CreateBeam(crate::standards::v1::subsets::any::schema::mutations::create_beam::CreateBeam),
    DeleteBeam(crate::standards::v1::subsets::any::schema::mutations::delete_beam::DeleteBeam),
    SetBeam(crate::standards::v1::subsets::any::schema::mutations::set_beam::SetBeam),
    CreateRailing(crate::standards::v1::subsets::any::schema::mutations::create_railing::CreateRailing),
    DeleteRailing(crate::standards::v1::subsets::any::schema::mutations::delete_railing::DeleteRailing),
    SetRailing(crate::standards::v1::subsets::any::schema::mutations::set_railing::SetRailing),
    CreateRamp(crate::standards::v1::subsets::any::schema::mutations::create_ramp::CreateRamp),
    SetRamp(crate::standards::v1::subsets::any::schema::mutations::set_ramp::SetRamp),
    DeleteRamp(crate::standards::v1::subsets::any::schema::mutations::delete_ramp::DeleteRamp),
    CreateSpace(crate::standards::v1::subsets::any::schema::mutations::create_space::CreateSpace),
    DeleteSpace(crate::standards::v1::subsets::any::schema::mutations::delete_space::DeleteSpace),
    SetSpace(crate::standards::v1::subsets::any::schema::mutations::set_space::SetSpace),
    CreateZone(crate::standards::v1::subsets::any::schema::mutations::create_zone::CreateZone),
    SetZone(crate::standards::v1::subsets::any::schema::mutations::set_zone::SetZone),
    DeleteZone(crate::standards::v1::subsets::any::schema::mutations::delete_zone::DeleteZone),
    CreateAreaScheme(crate::standards::v1::subsets::any::schema::mutations::create_area_scheme::CreateAreaScheme),
    SetAreaScheme(crate::standards::v1::subsets::any::schema::mutations::set_area_scheme::SetAreaScheme),
    DeleteAreaScheme(crate::standards::v1::subsets::any::schema::mutations::delete_area_scheme::DeleteAreaScheme),
    CreateSlab(crate::standards::v1::subsets::any::schema::mutations::create_slab::CreateSlab),
    DeleteSlab(crate::standards::v1::subsets::any::schema::mutations::delete_slab::DeleteSlab),
    SetSlabBoundary(crate::standards::v1::subsets::any::schema::mutations::set_slab_boundary::SetSlabBoundary),
    SetSlab(crate::standards::v1::subsets::any::schema::mutations::set_slab::SetSlab),
    CreateCeilingType(crate::standards::v1::subsets::any::schema::mutations::create_ceiling_type::CreateCeilingType),
    DeleteCeilingType(crate::standards::v1::subsets::any::schema::mutations::delete_ceiling_type::DeleteCeilingType),
    SetCeilingType(crate::standards::v1::subsets::any::schema::mutations::set_ceiling_type::SetCeilingType),
    CreateCeiling(crate::standards::v1::subsets::any::schema::mutations::create_ceiling::CreateCeiling),
    CreateDimension(crate::standards::v1::subsets::any::schema::mutations::create_dimension::CreateDimension),
    DeleteDimension(crate::standards::v1::subsets::any::schema::mutations::delete_dimension::DeleteDimension),
    SetDimension(crate::standards::v1::subsets::any::schema::mutations::set_dimension::SetDimension),
    CreateTag(crate::standards::v1::subsets::any::schema::mutations::create_tag::CreateTag),
    DeleteTag(crate::standards::v1::subsets::any::schema::mutations::delete_tag::DeleteTag),
    SetTag(crate::standards::v1::subsets::any::schema::mutations::set_tag::SetTag),
    CreateTextNote(crate::standards::v1::subsets::any::schema::mutations::create_text_note::CreateTextNote),
    DeleteTextNote(crate::standards::v1::subsets::any::schema::mutations::delete_text_note::DeleteTextNote),
    SetTextNote(crate::standards::v1::subsets::any::schema::mutations::set_text_note::SetTextNote),
    CreateLeader(crate::standards::v1::subsets::any::schema::mutations::create_leader::CreateLeader),
    DeleteLeader(crate::standards::v1::subsets::any::schema::mutations::delete_leader::DeleteLeader),
    SetLeader(crate::standards::v1::subsets::any::schema::mutations::set_leader::SetLeader),
    CreateAnnotationStyle(crate::standards::v1::subsets::any::schema::mutations::create_annotation_style::CreateAnnotationStyle),
    DeleteMepElement(crate::standards::v1::subsets::any::schema::mutations::delete_mep_element::DeleteMepElement),
    SetMepElement(crate::standards::v1::subsets::any::schema::mutations::set_mep_element::SetMepElement),
    CreateMepElement(crate::standards::v1::subsets::any::schema::mutations::create_mep_element::CreateMepElement),
    RemoveComponentOverride(crate::standards::v1::subsets::any::schema::mutations::remove_component_override::RemoveComponentOverride),
    SetComponentOverride(crate::standards::v1::subsets::any::schema::mutations::set_component_override::SetComponentOverride),
    DeleteComponent(crate::standards::v1::subsets::any::schema::mutations::delete_component::DeleteComponent),
    SetComponent(crate::standards::v1::subsets::any::schema::mutations::set_component::SetComponent),
    CreateComponent(crate::standards::v1::subsets::any::schema::mutations::create_component::CreateComponent),
    SetFamilySolid(crate::standards::v1::subsets::any::schema::mutations::set_family_solid::SetFamilySolid),
    DeleteFamilySolid(crate::standards::v1::subsets::any::schema::mutations::delete_family_solid::DeleteFamilySolid),
    CreateFamilySolid(crate::standards::v1::subsets::any::schema::mutations::create_family_solid::CreateFamilySolid),
    RemoveFamilyParameter(crate::standards::v1::subsets::any::schema::mutations::remove_family_parameter::RemoveFamilyParameter),
    SetFamilyParameter(crate::standards::v1::subsets::any::schema::mutations::set_family_parameter::SetFamilyParameter),
    SetFamily(crate::standards::v1::subsets::any::schema::mutations::set_family::SetFamily),
    DeleteFamily(crate::standards::v1::subsets::any::schema::mutations::delete_family::DeleteFamily),
    CreateFamily(crate::standards::v1::subsets::any::schema::mutations::create_family::CreateFamily),
    DeleteAnnotationStyle(crate::standards::v1::subsets::any::schema::mutations::delete_annotation_style::DeleteAnnotationStyle),
    SetAnnotationStyle(crate::standards::v1::subsets::any::schema::mutations::set_annotation_style::SetAnnotationStyle),
    DeleteCeiling(crate::standards::v1::subsets::any::schema::mutations::delete_ceiling::DeleteCeiling),
    SetCeilingBoundary(crate::standards::v1::subsets::any::schema::mutations::set_ceiling_boundary::SetCeilingBoundary),
    SetCeiling(crate::standards::v1::subsets::any::schema::mutations::set_ceiling::SetCeiling),
    CreateRoof(crate::standards::v1::subsets::any::schema::mutations::create_roof::CreateRoof),
    DeleteRoof(crate::standards::v1::subsets::any::schema::mutations::delete_roof::DeleteRoof),
    SetRoofFootprint(crate::standards::v1::subsets::any::schema::mutations::set_roof_footprint::SetRoofFootprint),
    SetRoofShape(crate::standards::v1::subsets::any::schema::mutations::set_roof_shape::SetRoofShape),
    CreateOpening(crate::standards::v1::subsets::any::schema::mutations::create_opening::CreateOpening),
    DeleteOpening(crate::standards::v1::subsets::any::schema::mutations::delete_opening::DeleteOpening),
    MoveOpening(crate::standards::v1::subsets::any::schema::mutations::move_opening::MoveOpening),
    SetOpening(crate::standards::v1::subsets::any::schema::mutations::set_opening::SetOpening),
    CreateStair(crate::standards::v1::subsets::any::schema::mutations::create_stair::CreateStair),
    DeleteStair(crate::standards::v1::subsets::any::schema::mutations::delete_stair::DeleteStair),
    SetStair(crate::standards::v1::subsets::any::schema::mutations::set_stair::SetStair),
    MoveElements(crate::standards::v1::subsets::any::schema::mutations::move_elements::MoveElements),
    RotateElements(crate::standards::v1::subsets::any::schema::mutations::rotate_elements::RotateElements),
    PlaceElements(crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements),
    DeleteElements(crate::standards::v1::subsets::any::schema::mutations::delete_elements::DeleteElements),
    RenameElement(crate::standards::v1::subsets::any::schema::mutations::rename_element::RenameElement),
    SetElementProperty(crate::standards::v1::subsets::any::schema::mutations::set_element_property::SetElementProperty),
    RemoveElementProperty(crate::standards::v1::subsets::any::schema::mutations::remove_element_property::RemoveElementProperty),
    SetElementClassification(crate::standards::v1::subsets::any::schema::mutations::set_element_classification::SetElementClassification),
    RemoveElementClassification(crate::standards::v1::subsets::any::schema::mutations::remove_element_classification::RemoveElementClassification),
    CreatePropertyTemplate(crate::standards::v1::subsets::any::schema::mutations::create_property_template::CreatePropertyTemplate),
    SetPropertyTemplate(crate::standards::v1::subsets::any::schema::mutations::set_property_template::SetPropertyTemplate),
    DeletePropertyTemplate(crate::standards::v1::subsets::any::schema::mutations::delete_property_template::DeletePropertyTemplate),
    CreateClassificationSystem(crate::standards::v1::subsets::any::schema::mutations::create_classification_system::CreateClassificationSystem),
    SetClassificationSystem(crate::standards::v1::subsets::any::schema::mutations::set_classification_system::SetClassificationSystem),
    DeleteClassificationSystem(crate::standards::v1::subsets::any::schema::mutations::delete_classification_system::DeleteClassificationSystem),
    CreateSchedule(crate::standards::v1::subsets::any::schema::mutations::create_schedule::CreateSchedule),
    SetSchedule(crate::standards::v1::subsets::any::schema::mutations::set_schedule::SetSchedule),
    DeleteSchedule(crate::standards::v1::subsets::any::schema::mutations::delete_schedule::DeleteSchedule),
    CreateView(crate::standards::v1::subsets::any::schema::mutations::create_view::CreateView),
    SetView(crate::standards::v1::subsets::any::schema::mutations::set_view::SetView),
    DeleteView(crate::standards::v1::subsets::any::schema::mutations::delete_view::DeleteView),
    CreateSheet(crate::standards::v1::subsets::any::schema::mutations::create_sheet::CreateSheet),
    SetSheet(crate::standards::v1::subsets::any::schema::mutations::set_sheet::SetSheet),
    DeleteSheet(crate::standards::v1::subsets::any::schema::mutations::delete_sheet::DeleteSheet),
    CreateViewport(crate::standards::v1::subsets::any::schema::mutations::create_viewport::CreateViewport),
    SetViewport(crate::standards::v1::subsets::any::schema::mutations::set_viewport::SetViewport),
    DeleteViewport(crate::standards::v1::subsets::any::schema::mutations::delete_viewport::DeleteViewport),
    CreateSheetRevision(crate::standards::v1::subsets::any::schema::mutations::create_sheet_revision::CreateSheetRevision),
    SetSheetRevision(crate::standards::v1::subsets::any::schema::mutations::set_sheet_revision::SetSheetRevision),
    DeleteSheetRevision(crate::standards::v1::subsets::any::schema::mutations::delete_sheet_revision::DeleteSheetRevision),
    SetSpaceConditions(crate::standards::v1::subsets::any::schema::mutations::set_space_conditions::SetSpaceConditions),
    RemoveSpaceConditions(crate::standards::v1::subsets::any::schema::mutations::remove_space_conditions::RemoveSpaceConditions),
    SetTypeThermalData(crate::standards::v1::subsets::any::schema::mutations::set_type_thermal_data::SetTypeThermalData),
    CreateClashSet(crate::standards::v1::subsets::any::schema::mutations::create_clash_set::CreateClashSet),
    SetClashSet(crate::standards::v1::subsets::any::schema::mutations::set_clash_set::SetClashSet),
    DeleteClashSet(crate::standards::v1::subsets::any::schema::mutations::delete_clash_set::DeleteClashSet),
    CreateRule(crate::standards::v1::subsets::any::schema::mutations::create_rule::CreateRule),
    SetRule(crate::standards::v1::subsets::any::schema::mutations::set_rule::SetRule),
    DeleteRule(crate::standards::v1::subsets::any::schema::mutations::delete_rule::DeleteRule),
    CreateIssue(crate::standards::v1::subsets::any::schema::mutations::create_issue::CreateIssue),
    SetIssue(crate::standards::v1::subsets::any::schema::mutations::set_issue::SetIssue),
    DeleteIssue(crate::standards::v1::subsets::any::schema::mutations::delete_issue::DeleteIssue),
    CreateIssueComment(crate::standards::v1::subsets::any::schema::mutations::create_issue_comment::CreateIssueComment),
    SetIssueComment(crate::standards::v1::subsets::any::schema::mutations::set_issue_comment::SetIssueComment),
    DeleteIssueComment(crate::standards::v1::subsets::any::schema::mutations::delete_issue_comment::DeleteIssueComment),
    SetWallEndJoin(crate::standards::v1::subsets::any::schema::mutations::set_wall_end_join::SetWallEndJoin),
    CopyElements(crate::standards::v1::subsets::any::schema::mutations::copy_elements::CopyElements),
    MirrorElements(crate::standards::v1::subsets::any::schema::mutations::mirror_elements::MirrorElements),
    ArrayElements(crate::standards::v1::subsets::any::schema::mutations::array_elements::ArrayElements),
    AlignElements(crate::standards::v1::subsets::any::schema::mutations::align_elements::AlignElements),
    OffsetWall(crate::standards::v1::subsets::any::schema::mutations::offset_wall::OffsetWall),
    TrimExtendWall(crate::standards::v1::subsets::any::schema::mutations::trim_extend_wall::TrimExtendWall),
    SplitSlab(crate::standards::v1::subsets::any::schema::mutations::split_slab::SplitSlab),
    SplitBeam(crate::standards::v1::subsets::any::schema::mutations::split_beam::SplitBeam),
    CreateWallSweep(crate::standards::v1::subsets::any::schema::mutations::create_wall_sweep::CreateWallSweep),
    SetWallSweep(crate::standards::v1::subsets::any::schema::mutations::set_wall_sweep::SetWallSweep),
    DeleteWallSweep(crate::standards::v1::subsets::any::schema::mutations::delete_wall_sweep::DeleteWallSweep),
    SetWallBaseSlab(crate::standards::v1::subsets::any::schema::mutations::set_wall_base_slab::SetWallBaseSlab),
    SetBeamAxis(crate::standards::v1::subsets::any::schema::mutations::set_beam_axis::SetBeamAxis),
    SetColumnTilt(crate::standards::v1::subsets::any::schema::mutations::set_column_tilt::SetColumnTilt),
    CreateCurtainWallType(crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall_type::CreateCurtainWallType),
    SetCurtainWallType(crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type::SetCurtainWallType),
    DeleteCurtainWallType(crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall_type::DeleteCurtainWallType),
    SetCurtainWallTypeOf(crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type_of::SetCurtainWallTypeOf),
    SetCurtainWallGrid(crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall_grid::SetCurtainWallGrid),
    CreateCurtainPanelOverride(crate::standards::v1::subsets::any::schema::mutations::create_curtain_panel_override::CreateCurtainPanelOverride),
    SetCurtainPanelOverride(crate::standards::v1::subsets::any::schema::mutations::set_curtain_panel_override::SetCurtainPanelOverride),
    DeleteCurtainPanelOverride(crate::standards::v1::subsets::any::schema::mutations::delete_curtain_panel_override::DeleteCurtainPanelOverride),
}

/// 🏷️ The kebab spelling of every [`ModelMutation`] variant, in declaration order: the one list the language-neutral test platform
/// is measured against (this subset's oracle catalog and the `mutate-model-1` adapter repeat it on purpose).
pub const KINDS: &[&str] = &[
    "set-element-workset",
    "set-element-option",
    "delete-workset",
    "set-workset",
    "create-workset",
    "delete-design-option",
    "set-design-option",
    "create-design-option",
    "delete-option-group",
    "set-option-group",
    "create-option-group",
    "create-support",
    "set-support",
    "delete-support",
    "create-load-case",
    "set-load-case",
    "delete-load-case",
    "create-load",
    "set-load",
    "delete-load",

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
    "set-element-phase",
    "set-element-storey",
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
    "create-ramp",
    "set-ramp",
    "delete-ramp",
    "create-space",
    "delete-space",
    "set-space",
    "create-zone",
    "set-zone",
    "delete-zone",
    "create-area-scheme",
    "set-area-scheme",
    "delete-area-scheme",
    "create-slab",
    "delete-slab",
    "set-slab-boundary",
    "set-slab",
    "create-ceiling-type",
    "delete-ceiling-type",
    "set-ceiling-type",
    "create-ceiling",
    "create-dimension",
    "delete-dimension",
    "set-dimension",
    "create-tag",
    "delete-tag",
    "set-tag",
    "create-text-note",
    "delete-text-note",
    "set-text-note",
    "create-leader",
    "delete-leader",
    "set-leader",
    "create-annotation-style",
    "delete-mep-element",
    "set-mep-element",
    "create-mep-element",
    "remove-component-override",
    "set-component-override",
    "delete-component",
    "set-component",
    "create-component",
    "set-family-solid",
    "delete-family-solid",
    "create-family-solid",
    "remove-family-parameter",
    "set-family-parameter",
    "set-family",
    "delete-family",
    "create-family",
    "delete-annotation-style",
    "set-annotation-style",
    "delete-ceiling",
    "set-ceiling-boundary",
    "set-ceiling",
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
    "create-property-template",
    "set-property-template",
    "delete-property-template",
    "create-classification-system",
    "set-classification-system",
    "delete-classification-system",
    "create-schedule",
    "set-schedule",
    "delete-schedule",
    "create-view",
    "set-view",
    "delete-view",
    "create-sheet",
    "set-sheet",
    "delete-sheet",
    "create-viewport",
    "set-viewport",
    "delete-viewport",
    "create-sheet-revision",
    "set-sheet-revision",
    "delete-sheet-revision",
    "set-space-conditions",
    "remove-space-conditions",
    "set-type-thermal-data",
    "create-clash-set",
    "set-clash-set",
    "delete-clash-set",
    "create-rule",
    "set-rule",
    "delete-rule",
    "create-issue",
    "set-issue",
    "delete-issue",
    "create-issue-comment",
    "set-issue-comment",
    "delete-issue-comment",
    "set-wall-end-join",
    "copy-elements",
    "mirror-elements",
    "array-elements",
    "align-elements",
    "offset-wall",
    "trim-extend-wall",
    "split-slab",
    "split-beam",
    "create-wall-sweep",
    "set-wall-sweep",
    "delete-wall-sweep",
    "set-wall-base-slab",
    "set-beam-axis",
    "set-column-tilt",
    "create-curtain-wall-type",
    "set-curtain-wall-type",
    "delete-curtain-wall-type",
    "set-curtain-wall-type-of",
    "set-curtain-wall-grid",
    "create-curtain-panel-override",
    "set-curtain-panel-override",
    "delete-curtain-panel-override",
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

//#region 🧩️Modules
pub use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::mutations::horizontal_rules;
//#region 🔖️Leaves
pub use semio_s_artifact_bim_model_mut_any_124::standards::v1::subsets::any::schema::mutations::create_site;
pub use semio_s_artifact_bim_model_mut_any_58::standards::v1::subsets::any::schema::mutations::delete_site;
pub use semio_s_artifact_bim_model_mut_any_125::standards::v1::subsets::any::schema::mutations::create_building;
pub use semio_s_artifact_bim_model_mut_any_59::standards::v1::subsets::any::schema::mutations::delete_building;
pub use semio_s_artifact_bim_model_mut_any_126::standards::v1::subsets::any::schema::mutations::create_storey;
pub use semio_s_artifact_bim_model_mut_any::standards::v1::subsets::any::schema::mutations::rename_storey;
pub use semio_s_artifact_bim_model_mut_any_2::standards::v1::subsets::any::schema::mutations::set_storey_height;
pub use semio_s_artifact_bim_model_mut_any_3::standards::v1::subsets::any::schema::mutations::set_storey_level;
pub use semio_s_artifact_bim_model_mut_any_60::standards::v1::subsets::any::schema::mutations::delete_storey;
pub use semio_s_artifact_bim_model_mut_any_173::standards::v1::subsets::any::schema::mutations::create_wall;
pub use semio_s_artifact_bim_model_mut_any_61::standards::v1::subsets::any::schema::mutations::delete_wall;
pub use semio_s_artifact_bim_model_mut_any_174::standards::v1::subsets::any::schema::mutations::set_wall_top;
pub use semio_s_artifact_bim_model_mut_any_127::standards::v1::subsets::any::schema::mutations::create_railing;
pub use semio_s_artifact_bim_model_mut_any_62::standards::v1::subsets::any::schema::mutations::delete_railing;
pub use semio_s_artifact_bim_model_mut_any_63::standards::v1::subsets::any::schema::mutations::set_railing;
pub use semio_s_artifact_bim_model_mut_any_128::standards::v1::subsets::any::schema::mutations::create_space;
pub use semio_s_artifact_bim_model_mut_any_64::standards::v1::subsets::any::schema::mutations::delete_space;
pub use semio_s_artifact_bim_model_mut_any_4::standards::v1::subsets::any::schema::mutations::set_space;
pub use semio_s_artifact_bim_model_mut_any_159::standards::v1::subsets::any::schema::mutations::create_column;
pub use semio_s_artifact_bim_model_mut_any_65::standards::v1::subsets::any::schema::mutations::delete_column;
pub use semio_s_artifact_bim_model_mut_any_66::standards::v1::subsets::any::schema::mutations::set_column;
pub use semio_s_artifact_bim_model_mut_any_160::standards::v1::subsets::any::schema::mutations::create_beam;
pub use semio_s_artifact_bim_model_mut_any_67::standards::v1::subsets::any::schema::mutations::delete_beam;
pub use semio_s_artifact_bim_model_mut_any_129::standards::v1::subsets::any::schema::mutations::create_slab;
pub use semio_s_artifact_bim_model_mut_any_68::standards::v1::subsets::any::schema::mutations::delete_slab;
pub use semio_s_artifact_bim_model_mut_any_5::standards::v1::subsets::any::schema::mutations::set_slab_boundary;
pub use semio_s_artifact_bim_model_mut_any_6::standards::v1::subsets::any::schema::mutations::set_slab;
pub use semio_s_artifact_bim_model_mut_any_130::standards::v1::subsets::any::schema::mutations::create_roof;
pub use semio_s_artifact_bim_model_mut_any_69::standards::v1::subsets::any::schema::mutations::delete_roof;
pub use semio_s_artifact_bim_model_mut_any_7::standards::v1::subsets::any::schema::mutations::set_roof_footprint;
pub use semio_s_artifact_bim_model_mut_any_8::standards::v1::subsets::any::schema::mutations::set_roof_shape;
pub use semio_s_artifact_bim_model_mut_any_70::standards::v1::subsets::any::schema::mutations::create_column_type;
pub use semio_s_artifact_bim_model_mut_any_70::standards::v1::subsets::any::schema::mutations::delete_column_type;
pub use semio_s_artifact_bim_model_mut_any_9::standards::v1::subsets::any::schema::mutations::set_column_type;
pub use semio_s_artifact_bim_model_mut_any_71::standards::v1::subsets::any::schema::mutations::create_beam_type;
pub use semio_s_artifact_bim_model_mut_any_71::standards::v1::subsets::any::schema::mutations::delete_beam_type;
pub use semio_s_artifact_bim_model_mut_any_10::standards::v1::subsets::any::schema::mutations::set_beam_type;
pub use semio_s_artifact_bim_model_mut_any_72::standards::v1::subsets::any::schema::mutations::create_window_type;
pub use semio_s_artifact_bim_model_mut_any_72::standards::v1::subsets::any::schema::mutations::delete_window_type;
pub use semio_s_artifact_bim_model_mut_any_11::standards::v1::subsets::any::schema::mutations::set_window_type;
pub use semio_s_artifact_bim_model_mut_any_73::standards::v1::subsets::any::schema::mutations::create_door_type;
pub use semio_s_artifact_bim_model_mut_any_73::standards::v1::subsets::any::schema::mutations::delete_door_type;
pub use semio_s_artifact_bim_model_mut_any_12::standards::v1::subsets::any::schema::mutations::set_door_type;
pub use semio_s_artifact_bim_model_mut_any_74::standards::v1::subsets::any::schema::mutations::create_material;
pub use semio_s_artifact_bim_model_mut_any_74::standards::v1::subsets::any::schema::mutations::delete_material;
pub use semio_s_artifact_bim_model_mut_any_13::standards::v1::subsets::any::schema::mutations::set_material;
pub use semio_s_artifact_bim_model_mut_any_75::standards::v1::subsets::any::schema::mutations::create_wall_type;
pub use semio_s_artifact_bim_model_mut_any_75::standards::v1::subsets::any::schema::mutations::delete_wall_type;
pub use semio_s_artifact_bim_model_mut_any_14::standards::v1::subsets::any::schema::mutations::set_wall_type;
pub use semio_s_artifact_bim_model_mut_any_76::standards::v1::subsets::any::schema::mutations::create_slab_type;
pub use semio_s_artifact_bim_model_mut_any_76::standards::v1::subsets::any::schema::mutations::delete_slab_type;
pub use semio_s_artifact_bim_model_mut_any_15::standards::v1::subsets::any::schema::mutations::set_slab_type;
pub use semio_s_artifact_bim_model_mut_any_77::standards::v1::subsets::any::schema::mutations::create_roof_type;
pub use semio_s_artifact_bim_model_mut_any_77::standards::v1::subsets::any::schema::mutations::delete_roof_type;
pub use semio_s_artifact_bim_model_mut_any_16::standards::v1::subsets::any::schema::mutations::set_roof_type;
pub use semio_s_artifact_bim_model_mut_any_175::standards::v1::subsets::any::schema::mutations::create_opening;
pub use semio_s_artifact_bim_model_mut_any_78::standards::v1::subsets::any::schema::mutations::delete_opening;
pub use semio_s_artifact_bim_model_mut_any_79::standards::v1::subsets::any::schema::mutations::move_opening;
pub use semio_s_artifact_bim_model_mut_any_176::standards::v1::subsets::any::schema::mutations::set_opening;
pub use semio_s_artifact_bim_model_mut_any_131::standards::v1::subsets::any::schema::mutations::create_stair;
pub use semio_s_artifact_bim_model_mut_any_80::standards::v1::subsets::any::schema::mutations::delete_stair;
pub use semio_s_artifact_bim_model_mut_any_81::standards::v1::subsets::any::schema::mutations::set_stair;
pub use semio_s_artifact_bim_model_mut_any_17::standards::v1::subsets::any::schema::mutations::set_project_info;
pub use semio_s_artifact_bim_model_mut_any_18::standards::v1::subsets::any::schema::mutations::set_site;
pub use semio_s_artifact_bim_model_mut_any_19::standards::v1::subsets::any::schema::mutations::set_building;
pub use semio_s_artifact_bim_model_mut_any_132::standards::v1::subsets::any::schema::mutations::create_grid_line;
pub use semio_s_artifact_bim_model_mut_any_82::standards::v1::subsets::any::schema::mutations::delete_grid_line;
pub use semio_s_artifact_bim_model_mut_any_20::standards::v1::subsets::any::schema::mutations::set_grid_line;
pub use semio_s_artifact_bim_model_mut_any_161::standards::v1::subsets::any::schema::mutations::set_wall_axis;
pub use semio_s_artifact_bim_model_mut_any_21::standards::v1::subsets::any::schema::mutations::set_wall_base_offset;
pub use semio_s_artifact_bim_model_mut_any_22::standards::v1::subsets::any::schema::mutations::set_wall_type_of;
pub use semio_s_artifact_bim_model_mut_any_23::standards::v1::subsets::any::schema::mutations::set_wall_location;
pub use semio_s_artifact_bim_model_mut_any_162::standards::v1::subsets::any::schema::mutations::flip_wall;
pub use semio_s_artifact_bim_model_mut_any_177::standards::v1::subsets::any::schema::mutations::split_wall;
pub use semio_s_artifact_bim_model_mut_any_83::standards::v1::subsets::any::schema::mutations::delete_curtain_wall;
pub use semio_s_artifact_bim_model_mut_any_133::standards::v1::subsets::any::schema::mutations::move_elements;
pub use semio_s_artifact_bim_model_mut_any_134::standards::v1::subsets::any::schema::mutations::rotate_elements;
pub use semio_s_artifact_bim_model_mut_any_84::standards::v1::subsets::any::schema::mutations::place_elements;
pub use semio_s_artifact_bim_model_mut_any_85::standards::v1::subsets::any::schema::mutations::delete_elements;
pub use semio_s_artifact_bim_model_mut_any_86::standards::v1::subsets::any::schema::mutations::rename_element;
pub use semio_s_artifact_bim_model_mut_any_87::standards::v1::subsets::any::schema::mutations::set_element_property;
pub use semio_s_artifact_bim_model_mut_any_87::standards::v1::subsets::any::schema::mutations::remove_element_property;
pub use semio_s_artifact_bim_model_mut_any_88::standards::v1::subsets::any::schema::mutations::set_element_classification;
pub use semio_s_artifact_bim_model_mut_any_88::standards::v1::subsets::any::schema::mutations::remove_element_classification;
pub use semio_s_artifact_bim_model_mut_any_24::standards::v1::subsets::any::schema::mutations::set_storey_cut_height;
pub use semio_s_artifact_bim_model_mut_any_89::standards::v1::subsets::any::schema::mutations::create_ceiling_type;
pub use semio_s_artifact_bim_model_mut_any_89::standards::v1::subsets::any::schema::mutations::delete_ceiling_type;
pub use semio_s_artifact_bim_model_mut_any_25::standards::v1::subsets::any::schema::mutations::set_ceiling_type;
pub use semio_s_artifact_bim_model_mut_any_135::standards::v1::subsets::any::schema::mutations::create_ceiling;
pub use semio_s_artifact_bim_model_mut_any_90::standards::v1::subsets::any::schema::mutations::delete_ceiling;
pub use semio_s_artifact_bim_model_mut_any_26::standards::v1::subsets::any::schema::mutations::set_ceiling_boundary;
pub use semio_s_artifact_bim_model_mut_any_27::standards::v1::subsets::any::schema::mutations::set_ceiling;
pub use semio_s_artifact_bim_model_mut_any_136::standards::v1::subsets::any::schema::mutations::create_ramp;
pub use semio_s_artifact_bim_model_mut_any_91::standards::v1::subsets::any::schema::mutations::set_ramp;
pub use semio_s_artifact_bim_model_mut_any_92::standards::v1::subsets::any::schema::mutations::delete_ramp;
pub use semio_s_artifact_bim_model_mut_any_137::standards::v1::subsets::any::schema::mutations::create_view;
pub use semio_s_artifact_bim_model_mut_any_28::standards::v1::subsets::any::schema::mutations::set_view;
pub use semio_s_artifact_bim_model_mut_any_93::standards::v1::subsets::any::schema::mutations::delete_view;
pub use semio_s_artifact_bim_model_mut_any_94::standards::v1::subsets::any::schema::mutations::create_annotation_style;
pub use semio_s_artifact_bim_model_mut_any_94::standards::v1::subsets::any::schema::mutations::delete_annotation_style;
pub use semio_s_artifact_bim_model_mut_any_29::standards::v1::subsets::any::schema::mutations::set_annotation_style;
pub use semio_s_artifact_bim_model_mut_any_138::standards::v1::subsets::any::schema::mutations::create_dimension;
pub use semio_s_artifact_bim_model_mut_any_95::standards::v1::subsets::any::schema::mutations::delete_dimension;
pub use semio_s_artifact_bim_model_mut_any_30::standards::v1::subsets::any::schema::mutations::set_dimension;
pub use semio_s_artifact_bim_model_mut_any_139::standards::v1::subsets::any::schema::mutations::create_tag;
pub use semio_s_artifact_bim_model_mut_any_96::standards::v1::subsets::any::schema::mutations::delete_tag;
pub use semio_s_artifact_bim_model_mut_any_31::standards::v1::subsets::any::schema::mutations::set_tag;
pub use semio_s_artifact_bim_model_mut_any_140::standards::v1::subsets::any::schema::mutations::create_text_note;
pub use semio_s_artifact_bim_model_mut_any_97::standards::v1::subsets::any::schema::mutations::delete_text_note;
pub use semio_s_artifact_bim_model_mut_any_32::standards::v1::subsets::any::schema::mutations::set_text_note;
pub use semio_s_artifact_bim_model_mut_any_141::standards::v1::subsets::any::schema::mutations::create_leader;
pub use semio_s_artifact_bim_model_mut_any_98::standards::v1::subsets::any::schema::mutations::delete_leader;
pub use semio_s_artifact_bim_model_mut_any_33::standards::v1::subsets::any::schema::mutations::set_leader;
pub use semio_s_artifact_bim_model_mut_any_142::standards::v1::subsets::any::schema::mutations::create_zone;
pub use semio_s_artifact_bim_model_mut_any_34::standards::v1::subsets::any::schema::mutations::set_zone;
pub use semio_s_artifact_bim_model_mut_any_99::standards::v1::subsets::any::schema::mutations::delete_zone;
pub use semio_s_artifact_bim_model_mut_any_143::standards::v1::subsets::any::schema::mutations::create_area_scheme;
pub use semio_s_artifact_bim_model_mut_any_100::standards::v1::subsets::any::schema::mutations::set_area_scheme;
pub use semio_s_artifact_bim_model_mut_any_101::standards::v1::subsets::any::schema::mutations::delete_area_scheme;
pub use semio_s_artifact_bim_model_mut_any_102::standards::v1::subsets::any::schema::mutations::create_schedule;
pub use semio_s_artifact_bim_model_mut_any_35::standards::v1::subsets::any::schema::mutations::set_schedule;
pub use semio_s_artifact_bim_model_mut_any_102::standards::v1::subsets::any::schema::mutations::delete_schedule;
pub use semio_s_artifact_bim_model_mut_any_144::standards::v1::subsets::any::schema::mutations::set_element_storey;
pub use semio_s_artifact_bim_model_mut_any_103::standards::v1::subsets::any::schema::mutations::set_element_phase;
pub use semio_s_artifact_bim_model_mut_any_178::standards::v1::subsets::any::schema::mutations::set_wall_end_join;
pub use semio_s_artifact_bim_model_mut_any_179::standards::v1::subsets::any::schema::mutations::copy_elements;
pub use semio_s_artifact_bim_model_mut_any_189::standards::v1::subsets::any::schema::mutations::mirror_elements;
pub use semio_s_artifact_bim_model_mut_any_180::standards::v1::subsets::any::schema::mutations::array_elements;
pub use semio_s_artifact_bim_model_mut_any_181::standards::v1::subsets::any::schema::mutations::align_elements;
pub use semio_s_artifact_bim_model_mut_any_182::standards::v1::subsets::any::schema::mutations::offset_wall;
pub use semio_s_artifact_bim_model_mut_any_183::standards::v1::subsets::any::schema::mutations::trim_extend_wall;
pub use semio_s_artifact_bim_model_mut_any_184::standards::v1::subsets::any::schema::mutations::split_slab;
pub use semio_s_artifact_bim_model_mut_any_185::standards::v1::subsets::any::schema::mutations::split_beam;
pub use semio_s_artifact_bim_model_mut_any_145::standards::v1::subsets::any::schema::mutations::create_sheet;
pub use semio_s_artifact_bim_model_mut_any_36::standards::v1::subsets::any::schema::mutations::set_sheet;
pub use semio_s_artifact_bim_model_mut_any_104::standards::v1::subsets::any::schema::mutations::delete_sheet;
pub use semio_s_artifact_bim_model_mut_any_146::standards::v1::subsets::any::schema::mutations::create_viewport;
pub use semio_s_artifact_bim_model_mut_any_37::standards::v1::subsets::any::schema::mutations::set_viewport;
pub use semio_s_artifact_bim_model_mut_any_105::standards::v1::subsets::any::schema::mutations::delete_viewport;
pub use semio_s_artifact_bim_model_mut_any_147::standards::v1::subsets::any::schema::mutations::create_sheet_revision;
pub use semio_s_artifact_bim_model_mut_any_38::standards::v1::subsets::any::schema::mutations::set_sheet_revision;
pub use semio_s_artifact_bim_model_mut_any_106::standards::v1::subsets::any::schema::mutations::delete_sheet_revision;
pub use semio_s_artifact_bim_model_mut_any_163::standards::v1::subsets::any::schema::mutations::set_beam_axis;
pub use semio_s_artifact_bim_model_mut_any_164::standards::v1::subsets::any::schema::mutations::set_column_tilt;
pub use semio_s_artifact_bim_model_mut_any_165::standards::v1::subsets::any::schema::mutations::create_curtain_wall_type;
pub use semio_s_artifact_bim_model_mut_any_166::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type;
pub use semio_s_artifact_bim_model_mut_any_165::standards::v1::subsets::any::schema::mutations::delete_curtain_wall_type;
pub use semio_s_artifact_bim_model_mut_any_39::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type_of;
pub use semio_s_artifact_bim_model_mut_any_167::standards::v1::subsets::any::schema::mutations::set_curtain_wall_grid;
pub use semio_s_artifact_bim_model_mut_any_168::standards::v1::subsets::any::schema::mutations::create_curtain_panel_override;
pub use semio_s_artifact_bim_model_mut_any_169::standards::v1::subsets::any::schema::mutations::set_curtain_panel_override;
pub use semio_s_artifact_bim_model_mut_any_107::standards::v1::subsets::any::schema::mutations::delete_curtain_panel_override;
pub use semio_s_artifact_bim_model_mut_any_170::standards::v1::subsets::any::schema::mutations::set_beam;
pub use semio_s_artifact_bim_model_mut_any_171::standards::v1::subsets::any::schema::mutations::set_curtain_wall;
pub use semio_s_artifact_bim_model_mut_any_172::standards::v1::subsets::any::schema::mutations::create_curtain_wall;
pub use semio_s_artifact_bim_model_mut_any_108::standards::v1::subsets::any::schema::mutations::create_property_template;
pub use semio_s_artifact_bim_model_mut_any_40::standards::v1::subsets::any::schema::mutations::set_property_template;
pub use semio_s_artifact_bim_model_mut_any_108::standards::v1::subsets::any::schema::mutations::delete_property_template;
pub use semio_s_artifact_bim_model_mut_any_148::standards::v1::subsets::any::schema::mutations::create_classification_system;
pub use semio_s_artifact_bim_model_mut_any_41::standards::v1::subsets::any::schema::mutations::set_classification_system;
pub use semio_s_artifact_bim_model_mut_any_148::standards::v1::subsets::any::schema::mutations::delete_classification_system;
pub use semio_s_artifact_bim_model_mut_any_149::standards::v1::subsets::any::schema::mutations::create_family;
pub use semio_s_artifact_bim_model_mut_any_149::standards::v1::subsets::any::schema::mutations::delete_family;
pub use semio_s_artifact_bim_model_mut_any_42::standards::v1::subsets::any::schema::mutations::set_family;
pub use semio_s_artifact_bim_model_mut_any_109::standards::v1::subsets::any::schema::mutations::set_family_parameter;
pub use semio_s_artifact_bim_model_mut_any_109::standards::v1::subsets::any::schema::mutations::remove_family_parameter;
pub use semio_s_artifact_bim_model_mut_any_110::standards::v1::subsets::any::schema::mutations::create_family_solid;
pub use semio_s_artifact_bim_model_mut_any_110::standards::v1::subsets::any::schema::mutations::delete_family_solid;
pub use semio_s_artifact_bim_model_mut_any_43::standards::v1::subsets::any::schema::mutations::set_family_solid;
pub use semio_s_artifact_bim_model_mut_any_186::standards::v1::subsets::any::schema::mutations::create_wall_sweep;
pub use semio_s_artifact_bim_model_mut_any_187::standards::v1::subsets::any::schema::mutations::set_wall_sweep;
pub use semio_s_artifact_bim_model_mut_any_111::standards::v1::subsets::any::schema::mutations::delete_wall_sweep;
pub use semio_s_artifact_bim_model_mut_any_188::standards::v1::subsets::any::schema::mutations::set_wall_base_slab;
pub use semio_s_artifact_bim_model_mut_any_112::standards::v1::subsets::any::schema::mutations::create_support;
pub use semio_s_artifact_bim_model_mut_any_44::standards::v1::subsets::any::schema::mutations::set_support;
pub use semio_s_artifact_bim_model_mut_any_112::standards::v1::subsets::any::schema::mutations::delete_support;
pub use semio_s_artifact_bim_model_mut_any_113::standards::v1::subsets::any::schema::mutations::create_load_case;
pub use semio_s_artifact_bim_model_mut_any_45::standards::v1::subsets::any::schema::mutations::set_load_case;
pub use semio_s_artifact_bim_model_mut_any_113::standards::v1::subsets::any::schema::mutations::delete_load_case;
pub use semio_s_artifact_bim_model_mut_any_114::standards::v1::subsets::any::schema::mutations::create_load;
pub use semio_s_artifact_bim_model_mut_any_46::standards::v1::subsets::any::schema::mutations::set_load;
pub use semio_s_artifact_bim_model_mut_any_114::standards::v1::subsets::any::schema::mutations::delete_load;
pub use semio_s_artifact_bim_model_mut_any_115::standards::v1::subsets::any::schema::mutations::create_option_group;
pub use semio_s_artifact_bim_model_mut_any_47::standards::v1::subsets::any::schema::mutations::set_option_group;
pub use semio_s_artifact_bim_model_mut_any_115::standards::v1::subsets::any::schema::mutations::delete_option_group;
pub use semio_s_artifact_bim_model_mut_any_116::standards::v1::subsets::any::schema::mutations::create_design_option;
pub use semio_s_artifact_bim_model_mut_any_48::standards::v1::subsets::any::schema::mutations::set_design_option;
pub use semio_s_artifact_bim_model_mut_any_116::standards::v1::subsets::any::schema::mutations::delete_design_option;
pub use semio_s_artifact_bim_model_mut_any_117::standards::v1::subsets::any::schema::mutations::create_workset;
pub use semio_s_artifact_bim_model_mut_any_49::standards::v1::subsets::any::schema::mutations::set_workset;
pub use semio_s_artifact_bim_model_mut_any_117::standards::v1::subsets::any::schema::mutations::delete_workset;
pub use semio_s_artifact_bim_model_mut_any_50::standards::v1::subsets::any::schema::mutations::set_element_option;
pub use semio_s_artifact_bim_model_mut_any_51::standards::v1::subsets::any::schema::mutations::set_element_workset;
pub use semio_s_artifact_bim_model_mut_any_150::standards::v1::subsets::any::schema::mutations::create_clash_set;
pub use semio_s_artifact_bim_model_mut_any_52::standards::v1::subsets::any::schema::mutations::set_clash_set;
pub use semio_s_artifact_bim_model_mut_any_118::standards::v1::subsets::any::schema::mutations::delete_clash_set;
pub use semio_s_artifact_bim_model_mut_any_151::standards::v1::subsets::any::schema::mutations::create_rule;
pub use semio_s_artifact_bim_model_mut_any_53::standards::v1::subsets::any::schema::mutations::set_rule;
pub use semio_s_artifact_bim_model_mut_any_119::standards::v1::subsets::any::schema::mutations::delete_rule;
pub use semio_s_artifact_bim_model_mut_any_152::standards::v1::subsets::any::schema::mutations::create_issue;
pub use semio_s_artifact_bim_model_mut_any_54::standards::v1::subsets::any::schema::mutations::set_issue;
pub use semio_s_artifact_bim_model_mut_any_120::standards::v1::subsets::any::schema::mutations::delete_issue;
pub use semio_s_artifact_bim_model_mut_any_153::standards::v1::subsets::any::schema::mutations::create_issue_comment;
pub use semio_s_artifact_bim_model_mut_any_55::standards::v1::subsets::any::schema::mutations::set_issue_comment;
pub use semio_s_artifact_bim_model_mut_any_121::standards::v1::subsets::any::schema::mutations::delete_issue_comment;
pub use semio_s_artifact_bim_model_mut_any_154::standards::v1::subsets::any::schema::mutations::create_component;
pub use semio_s_artifact_bim_model_mut_any_155::standards::v1::subsets::any::schema::mutations::set_component;
pub use semio_s_artifact_bim_model_mut_any_122::standards::v1::subsets::any::schema::mutations::delete_component;
pub use semio_s_artifact_bim_model_mut_any_156::standards::v1::subsets::any::schema::mutations::set_component_override;
pub use semio_s_artifact_bim_model_mut_any_156::standards::v1::subsets::any::schema::mutations::remove_component_override;
pub use semio_s_artifact_bim_model_mut_any_157::standards::v1::subsets::any::schema::mutations::create_mep_element;
pub use semio_s_artifact_bim_model_mut_any_158::standards::v1::subsets::any::schema::mutations::set_mep_element;
pub use semio_s_artifact_bim_model_mut_any_123::standards::v1::subsets::any::schema::mutations::delete_mep_element;
pub use semio_s_artifact_bim_model_mut_any_56::standards::v1::subsets::any::schema::mutations::set_space_conditions;
pub use semio_s_artifact_bim_model_mut_any_56::standards::v1::subsets::any::schema::mutations::remove_space_conditions;
pub use semio_s_artifact_bim_model_mut_any_57::standards::v1::subsets::any::schema::mutations::set_type_thermal_data;
//#endregion 🔖️Leaves
//#endregion 🧩️Modules
