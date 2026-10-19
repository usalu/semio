//! 🗂️ Entity table of the BIM entity table: the one static table of every entity kind. See [`super`].

use crate::editor::bim::entities::*;
use semio_s_artifact_bim_model_viewer::editor::bim::entities::row_types::EntityKind;
use semio_s_artifact_bim_model_viewer::editor::bim::entities::create_rows::{create_site, create_building, create_storey, create_grid, create_wall, create_curtain_wall, create_column, create_beam, create_slab, create_roof, create_opening, create_stair, create_railing, create_space, create_material, create_wall_type, create_slab_type, create_roof_type, create_column_type, create_beam_type, create_window_type, create_door_type};
use semio_s_artifact_bim_model_viewer::editor::bim::entities::frame;
use semio_s_artifact_bim_model_mut_any_101::standards::v1::subsets::any::schema::mutations::delete_area_scheme;
use semio_s_artifact_bim_model_mut_any_67::standards::v1::subsets::any::schema::mutations::delete_beam;
use semio_s_artifact_bim_model_mut_any_71::standards::v1::subsets::any::schema::mutations::delete_beam_type;
use semio_s_artifact_bim_model_mut_any_59::standards::v1::subsets::any::schema::mutations::delete_building;
use semio_s_artifact_bim_model_mut_any_90::standards::v1::subsets::any::schema::mutations::delete_ceiling;
use semio_s_artifact_bim_model_mut_any_89::standards::v1::subsets::any::schema::mutations::delete_ceiling_type;
use semio_s_artifact_bim_model_mut_any_148::standards::v1::subsets::any::schema::mutations::delete_classification_system;
use semio_s_artifact_bim_model_mut_any_65::standards::v1::subsets::any::schema::mutations::delete_column;
use semio_s_artifact_bim_model_mut_any_70::standards::v1::subsets::any::schema::mutations::delete_column_type;
use semio_s_artifact_bim_model_mut_any_83::standards::v1::subsets::any::schema::mutations::delete_curtain_wall;
use semio_s_artifact_bim_model_mut_any_73::standards::v1::subsets::any::schema::mutations::delete_door_type;
use semio_s_artifact_bim_model_mut_any_149::standards::v1::subsets::any::schema::mutations::delete_family;
use semio_s_artifact_bim_model_mut_any_110::standards::v1::subsets::any::schema::mutations::delete_family_solid;
use semio_s_artifact_bim_model_mut_any_82::standards::v1::subsets::any::schema::mutations::delete_grid_line;
use semio_s_artifact_bim_model_mut_any_74::standards::v1::subsets::any::schema::mutations::delete_material;
use semio_s_artifact_bim_model_mut_any_78::standards::v1::subsets::any::schema::mutations::delete_opening;
use semio_s_artifact_bim_model_mut_any_108::standards::v1::subsets::any::schema::mutations::delete_property_template;
use semio_s_artifact_bim_model_mut_any_62::standards::v1::subsets::any::schema::mutations::delete_railing;
use semio_s_artifact_bim_model_mut_any_92::standards::v1::subsets::any::schema::mutations::delete_ramp;
use semio_s_artifact_bim_model_mut_any_69::standards::v1::subsets::any::schema::mutations::delete_roof;
use semio_s_artifact_bim_model_mut_any_77::standards::v1::subsets::any::schema::mutations::delete_roof_type;
use semio_s_artifact_bim_model_mut_any_102::standards::v1::subsets::any::schema::mutations::delete_schedule;
use semio_s_artifact_bim_model_mut_any_58::standards::v1::subsets::any::schema::mutations::delete_site;
use semio_s_artifact_bim_model_mut_any_68::standards::v1::subsets::any::schema::mutations::delete_slab;
use semio_s_artifact_bim_model_mut_any_76::standards::v1::subsets::any::schema::mutations::delete_slab_type;
use semio_s_artifact_bim_model_mut_any_64::standards::v1::subsets::any::schema::mutations::delete_space;
use semio_s_artifact_bim_model_mut_any_80::standards::v1::subsets::any::schema::mutations::delete_stair;
use semio_s_artifact_bim_model_mut_any_60::standards::v1::subsets::any::schema::mutations::delete_storey;
use semio_s_artifact_bim_model_mut_any_93::standards::v1::subsets::any::schema::mutations::delete_view;
use semio_s_artifact_bim_model_mut_any_61::standards::v1::subsets::any::schema::mutations::delete_wall;
use semio_s_artifact_bim_model_mut_any_111::standards::v1::subsets::any::schema::mutations::delete_wall_sweep;
use semio_s_artifact_bim_model_mut_any_75::standards::v1::subsets::any::schema::mutations::delete_wall_type;
use semio_s_artifact_bim_model_mut_any_72::standards::v1::subsets::any::schema::mutations::delete_window_type;
use semio_s_artifact_bim_model_mut_any_99::standards::v1::subsets::any::schema::mutations::delete_zone;
use semio_s_artifact_bim_model_mut_any_10::standards::v1::subsets::any::schema::mutations::set_beam_type;
use semio_s_artifact_bim_model_mut_any_25::standards::v1::subsets::any::schema::mutations::set_ceiling_type;
use semio_s_artifact_bim_model_mut_any_41::standards::v1::subsets::any::schema::mutations::set_classification_system;
use semio_s_artifact_bim_model_mut_any_9::standards::v1::subsets::any::schema::mutations::set_column_type;
use semio_s_artifact_bim_model_mut_any_12::standards::v1::subsets::any::schema::mutations::set_door_type;
use semio_s_artifact_bim_model_mut_any_42::standards::v1::subsets::any::schema::mutations::set_family;
use semio_s_artifact_bim_model_mut_any_43::standards::v1::subsets::any::schema::mutations::set_family_solid;
use semio_s_artifact_bim_model_mut_any_13::standards::v1::subsets::any::schema::mutations::set_material;
use semio_s_artifact_bim_model_mut_any_40::standards::v1::subsets::any::schema::mutations::set_property_template;
use semio_s_artifact_bim_model_mut_any_16::standards::v1::subsets::any::schema::mutations::set_roof_type;
use semio_s_artifact_bim_model_mut_any_35::standards::v1::subsets::any::schema::mutations::set_schedule;
use semio_s_artifact_bim_model_mut_any_15::standards::v1::subsets::any::schema::mutations::set_slab_type;
use semio_s_artifact_bim_model_mut_any_28::standards::v1::subsets::any::schema::mutations::set_view;
use semio_s_artifact_bim_model_mut_any_14::standards::v1::subsets::any::schema::mutations::set_wall_type;
use semio_s_artifact_bim_model_mut_any_11::standards::v1::subsets::any::schema::mutations::set_window_type;

//#region 🔖️Table
/// 🧩️ Every entity kind, structure first (site to space), then the library (materials and types). A new mutation kind is one row.
pub static ENTITIES: &[EntityKind] = &[
    options::OPTION_GROUP,
    options::DESIGN_OPTION,
    options::WORKSET,
    kind!("site", "map", false, kind_site, group_sites, sites . name, parent: |_, _| None,
        delete: delete!(delete_site::DeleteSite), rename: Some(rename_element), create: Some(create_site), fields: SITE_FIELDS, inferred: &[]),
    kind!("building", "building", false, kind_building, group_buildings, buildings . name, parent: |s, id| s.buildings.get(id).map(|row| row.site.clone()),
        delete: delete!(delete_building::DeleteBuilding), rename: Some(rename_element), create: Some(create_building), fields: BUILDING_FIELDS, inferred: energy::BUILDING_INFERRED),
    kind!("storey", "layers", false, kind_storey, group_storeys, storeys . name, parent: |s, id| s.storeys.get(id).map(|row| row.building.clone()),
        delete: delete!(delete_storey::DeleteStorey), rename: Some(rename_element), create: Some(create_storey), fields: STOREY_FIELDS, inferred: STOREY_INFERRED),
    kind!("grid", "grid", false, kind_grid, group_grids, grids . label, parent: |s, id| s.grids.get(id).map(|row| row.building.clone()),
        delete: delete!(delete_grid_line::DeleteGridLine), rename: Some(rename_element), create: Some(create_grid), fields: GRID_FIELDS, inferred: &[]),
    kind!("wall", "square", false, kind_wall, group_walls, walls . name, parent: |s, id| s.walls.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_wall::DeleteWall), rename: Some(rename_element), create: Some(create_wall), fields: WALL_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("curtain-wall", "panels-top-left", false, kind_curtain_wall, group_curtain_walls, curtain_walls . name, parent: |s, id| s.curtain_walls.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_curtain_wall::DeleteCurtainWall), rename: Some(rename_element), create: Some(create_curtain_wall), fields: CURTAIN_WALL_FIELDS, inferred: QUANTITY_INFERRED),
    semio_s_artifact_bim_model_viewer::editor::bim::entities::frame::CURTAIN_PANEL_OVERRIDE,
    kind!("column", "columns", false, kind_column, group_columns, columns . name, parent: |s, id| s.columns.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_column::DeleteColumn), rename: Some(rename_element), create: Some(create_column), fields: COLUMN_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("beam", "minus", false, kind_beam, group_beams, beams . name, parent: |s, id| s.beams.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_beam::DeleteBeam), rename: Some(rename_element), create: Some(create_beam), fields: BEAM_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("slab", "layout-panel-top", false, kind_slab, group_slabs, slabs . name, parent: |s, id| s.slabs.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_slab::DeleteSlab), rename: Some(rename_element), create: Some(create_slab), fields: SLAB_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("ceiling", "panel-top", false, kind_ceiling, group_ceilings, ceilings . name, parent: |s, id| s.ceilings.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_ceiling::DeleteCeiling), rename: Some(rename_element), create: Some(ceilings::create_ceiling), fields: ceilings::CEILING_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("roof", "house", false, kind_roof, group_roofs, roofs . name, parent: |s, id| s.roofs.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_roof::DeleteRoof), rename: Some(rename_element), create: Some(create_roof), fields: ROOF_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("opening", "door-open", false, kind_opening, group_openings, openings . name, parent: |s, id| s.openings.get(id).map(|row| row.host.clone()),
        delete: delete!(delete_opening::DeleteOpening), rename: Some(rename_element), create: Some(create_opening), fields: OPENING_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("wall-sweep", "baseline", false, kind_wall_sweep, group_wall_sweeps, wall_sweeps . name, parent: |s, id| s.wall_sweeps.get(id).map(|row| row.host.clone()),
        delete: delete!(delete_wall_sweep::DeleteWallSweep), rename: Some(rename_element), create: Some(wall_sweeps::create_wall_sweep), fields: wall_sweeps::SWEEP_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("stair", "footprints", false, kind_stair, group_stairs, stairs . name, parent: |s, id| s.stairs.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_stair::DeleteStair), rename: Some(rename_element), create: Some(create_stair), fields: STAIR_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("railing", "fence", false, kind_railing, group_railings, railings . name, parent: |s, id| s.railings.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_railing::DeleteRailing), rename: Some(rename_element), create: Some(create_railing), fields: RAILING_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("ramp", "trending-up", false, kind_ramp, group_ramps, ramps . name, parent: |s, id| s.ramps.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_ramp::DeleteRamp), rename: Some(rename_element), create: Some(ramps::create_ramp), fields: ramps::RAMP_FIELDS, inferred: ramps::RAMP_INFERRED),
    components::COMPONENT,
    components::COMPONENT_OVERRIDE,
    mep::MEP_ELEMENT,
    kind!("space", "square-dashed", false, kind_space, group_spaces, spaces . name, parent: |s, id| s.spaces.get(id).map(|row| row.storey.clone()),
        delete: delete!(delete_space::DeleteSpace), rename: Some(rename_element), create: Some(create_space), fields: SPACE_FIELDS, inferred: QUANTITY_INFERRED),
    kind!("zone", "group", false, kind_zone, group_zones, zones . name, parent: |_, _| None,
        delete: delete!(delete_zone::DeleteZone), rename: Some(rename_element), create: Some(zoning::create_zone), fields: zoning::ZONE_FIELDS, inferred: zoning::ZONE_INFERRED),
    kind!("area-scheme", "ruler", false, kind_area_scheme, group_area_schemes, area_schemes . name, parent: |_, _| None,
        delete: delete!(delete_area_scheme::DeleteAreaScheme), rename: Some(rename_element), create: Some(zoning::create_area_scheme), fields: zoning::AREA_SCHEME_FIELDS, inferred: zoning::AREA_SCHEME_INFERRED),
    kind!("view", "scan-eye", false, kind_view, group_views, views . name, parent: |s, id| s.views.get(id).map(|row| row.storey.clone().unwrap_or_else(|| row.building.clone())),
        delete: delete!(delete_view::DeleteView), rename: renaming!(set_view::SetView), create: Some(views::create_view), fields: views::VIEW_FIELDS, inferred: &[]),
    notations::DIMENSION,
    notations::TAG,
    notations::TEXT_NOTE,
    notations::LEADER,
    sheets::SHEET,
    sheets::VIEWPORT,
    sheets::REVISION,
    coordination::CLASH_SET,
    coordination::RULE,
    coordination::ISSUE,
    coordination::ISSUE_COMMENT,
    kind!("family", "shapes", true, kind_family, group_families, families . name, parent: |_, _| None,
        delete: delete!(delete_family::DeleteFamily), rename: renaming!(set_family::SetFamily), create: Some(families::create_family), fields: families::FAMILY_FIELDS, inferred: families::FAMILY_INFERRED),
    kind!("family-solid", "box", false, kind_family_solid, group_family_solids, family_solids . name, parent: |s, id| s.family_solids.get(id).map(|row| row.family.clone()),
        delete: delete!(delete_family_solid::DeleteFamilySolid), rename: renaming!(set_family_solid::SetFamilySolid), create: Some(families::create_family_solid), fields: families::FAMILY_SOLID_FIELDS, inferred: families::FAMILY_SOLID_INFERRED),
    kind!("material", "palette", true, kind_material, group_materials, materials . name, parent: |_, _| None,
        delete: delete!(delete_material::DeleteMaterial), rename: renaming!(set_material::SetMaterial), create: Some(create_material), fields: MATERIAL_FIELDS, inferred: &[]),
    kind!("wall-type", "brick-wall", true, kind_wall_type, group_wall_types, wall_types . name, parent: |_, _| None,
        delete: delete!(delete_wall_type::DeleteWallType), rename: renaming!(set_wall_type::SetWallType), create: Some(create_wall_type), fields: WALL_TYPE_FIELDS, inferred: &[]),
    kind!("slab-type", "layout-panel-top", true, kind_slab_type, group_slab_types, slab_types . name, parent: |_, _| None,
        delete: delete!(delete_slab_type::DeleteSlabType), rename: renaming!(set_slab_type::SetSlabType), create: Some(create_slab_type), fields: SLAB_TYPE_FIELDS, inferred: &[]),
    kind!("ceiling-type", "panel-top", true, kind_ceiling_type, group_ceiling_types, ceiling_types . name, parent: |_, _| None,
        delete: delete!(delete_ceiling_type::DeleteCeilingType), rename: renaming!(set_ceiling_type::SetCeilingType), create: Some(ceilings::create_ceiling_type), fields: ceilings::CEILING_TYPE_FIELDS, inferred: &[]),
    kind!("roof-type", "house", true, kind_roof_type, group_roof_types, roof_types . name, parent: |_, _| None,
        delete: delete!(delete_roof_type::DeleteRoofType), rename: renaming!(set_roof_type::SetRoofType), create: Some(create_roof_type), fields: ROOF_TYPE_FIELDS, inferred: &[]),
    kind!("column-type", "columns", true, kind_column_type, group_column_types, column_types . name, parent: |_, _| None,
        delete: delete!(delete_column_type::DeleteColumnType), rename: renaming!(set_column_type::SetColumnType), create: Some(create_column_type), fields: COLUMN_TYPE_FIELDS, inferred: &[]),
    semio_s_artifact_bim_model_viewer::editor::bim::entities::frame::CURTAIN_WALL_TYPE,
    kind!("beam-type", "minus", true, kind_beam_type, group_beam_types, beam_types . name, parent: |_, _| None,
        delete: delete!(delete_beam_type::DeleteBeamType), rename: renaming!(set_beam_type::SetBeamType), create: Some(create_beam_type), fields: BEAM_TYPE_FIELDS, inferred: &[]),
    kind!("window-type", "app-window", true, kind_window_type, group_window_types, window_types . name, parent: |_, _| None,
        delete: delete!(delete_window_type::DeleteWindowType), rename: renaming!(set_window_type::SetWindowType), create: Some(create_window_type), fields: WINDOW_TYPE_FIELDS, inferred: &[]),
    kind!("door-type", "door-closed", true, kind_door_type, group_door_types, door_types . name, parent: |_, _| None,
        delete: delete!(delete_door_type::DeleteDoorType), rename: renaming!(set_door_type::SetDoorType), create: Some(create_door_type), fields: DOOR_TYPE_FIELDS, inferred: &[]),
    notations::ANNOTATION_STYLE,
    kind!("schedule", "table", true, kind_schedule, group_schedules, schedules . name, parent: |_, _| None,
        delete: delete!(delete_schedule::DeleteSchedule), rename: renaming!(set_schedule::SetSchedule), create: Some(schedules::create_schedule), fields: schedules::SCHEDULE_FIELDS, inferred: &[]),
    kind!("property-template", "list-checks", true, kind_property_template, group_property_templates, property_templates . name, parent: |_, _| None,
        delete: delete!(delete_property_template::DeletePropertyTemplate), rename: renaming!(set_property_template::SetPropertyTemplate), create: Some(psets::create_property_template), fields: psets::PROPERTY_TEMPLATE_FIELDS, inferred: psets::PROPERTY_TEMPLATE_INFERRED),
    kind!("classification-system", "library", true, kind_classification_system, group_classification_systems, classification_systems . name, parent: |_, _| None,
        delete: delete!(delete_classification_system::DeleteClassificationSystem), rename: renaming!(set_classification_system::SetClassificationSystem), create: Some(psets::create_classification_system), fields: psets::CLASSIFICATION_SYSTEM_FIELDS, inferred: psets::CLASSIFICATION_SYSTEM_INFERRED),
];
//#endregion 🔖️Table
