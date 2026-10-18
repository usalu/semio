//! 🧮️ Field tables of the BIM entity table: the authored parameter rows and inferred rows of every entity kind. See [`super`].

use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, snapshot::OpeningKind, mutations::ModelMutation};
use crate::standards::v1::subsets::any::schema::diff::Assigned;
use crate::editor::bim::entities::*;

//#region 🔖️Fields
pub static SITE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.sites.get(id).map(|row| row.name.clone()), rename),
    field!("latitude", field_latitude, Number, |s, id| s.sites.get(id).map(|row| number(row.latitude)), parse_number => set_site::SetSite),
    field!("longitude", field_longitude, Number, |s, id| s.sites.get(id).map(|row| number(row.longitude)), parse_number => set_site::SetSite),
    field!("elevation", field_elevation, Number, |s, id| s.sites.get(id).map(|row| number(row.elevation)), parse_number => set_site::SetSite),
    field!("true_north", field_true_north, Number, |s, id| s.sites.get(id).map(|row| number(row.true_north)), parse_number => set_site::SetSite),
];

pub static BUILDING_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.buildings.get(id).map(|row| row.name.clone()), rename),
    field!("site", field_site, Text, |s, id| s.buildings.get(id).map(|row| row.site.clone())),
    field!("origin", field_origin, Text, |s, id| s.buildings.get(id).map(|row| point(row.origin.x, row.origin.y)), parse_point => set_building::SetBuilding),
    field!("rotation", field_rotation, Number, |s, id| s.buildings.get(id).map(|row| number(row.rotation)), parse_number => set_building::SetBuilding),
    field!("elevation", field_elevation, Number, |s, id| s.buildings.get(id).map(|row| number(row.elevation)), parse_number => set_building::SetBuilding),
];

pub static STOREY_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.storeys.get(id).map(|row| row.name.clone()), rename),
    field!("building", field_building, Text, |s, id| s.storeys.get(id).map(|row| row.building.clone())),
    field!("level", field_level, Number, |s, id| s.storeys.get(id).map(|row| row.level.to_string()), parse_int => set_storey_level::SetStoreyLevel),
    field!("height", field_height, Number, |s, id| s.storeys.get(id).map(|row| number(row.height)), parse_number => set_storey_height::SetStoreyHeight),
    field!("cut_height", field_cut_height, Number, |s, id| s.storeys.get(id).map(|row| number(row.cut_height.unwrap_or(crate::standards::v1::subsets::any::schema::snapshot::DEFAULT_CUT_HEIGHT))), |_, id, value| set!(set_storey_cut_height::SetStoreyCutHeight, id, "cut_height", &Assigned::new(parse_optional_number(value)?))),
];

pub static GRID_FIELDS: &[FieldRow] = &[
    field!("label", field_label, Text, |s, id| s.grids.get(id).map(|row| row.label.clone()), rename),
    field!("building", field_building, Text, |s, id| s.grids.get(id).map(|row| row.building.clone())),
    field!("start", field_start, Text, |s, id| s.grids.get(id).map(|row| point(row.start.x, row.start.y)), parse_point => set_grid_line::SetGridLine),
    field!("end", field_end, Text, |s, id| s.grids.get(id).map(|row| point(row.end.x, row.end.y)), parse_point => set_grid_line::SetGridLine),
];

pub static WALL_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.walls.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.walls.get(id).map(|row| row.storey.clone()), choices: phasing::storey_choices, write: phasing::write_storey),
    field!("phase", field_phase, Text, |s, id| s.walls.get(id).map(|row| format!("{:?}", row.phase)), choices: phasing::phase_choices, write: phasing::write_phase),
    field!("wall_type", field_wall_type, Text, |s, id| s.walls.get(id).map(|row| row.wall_type.clone()), parse_text => set_wall_type_of::SetWallTypeOf),
    field!("axis", field_axis, Text, |s, id| s.walls.get(id).map(|row| axis_text(&row.axis)), parse_axis => set_wall_axis::SetWallAxis),
    field!("location", field_location, Text, |s, id| s.walls.get(id).map(|row| format!("{:?}", row.location)), parse_location => set_wall_location::SetWallLocation),
    field!("base_offset", field_base_offset, Number, |s, id| s.walls.get(id).map(|row| number(row.base_offset)), parse_number => set_wall_base_offset::SetWallBaseOffset),
    field!("top", field_top, Text, |s, id| s.walls.get(id).map(|row| top_text(&row.top)), parse_top => set_wall_top::SetWallTop),
    field!("top_attach", field_top_attach, Text, |s, id| wall_sweeps::top_attach_text(s, id), choices: wall_sweeps::surface_choices, write: wall_sweeps::write_top_attach),
    field!("base_slab", field_base_slab, Text, |s, id| wall_sweeps::base_slab_text(s, id), choices: wall_sweeps::slab_choices, write: wall_sweeps::write_base_slab),
    field!("start_join",field_start_join, Text, |s, id| s.walls.get(id).map(|row| join_text(row.start_join)), |_, id, value| Some(ModelMutation::SetWallEndJoin(crate::standards::v1::subsets::any::schema::mutations::set_wall_end_join::SetWallEndJoin { id: id.into(), end: crate::standards::v1::subsets::any::schema::mutations::modify::WallEnd::Start, join: parse_join(value)? }))),
    field!("end_join", field_end_join, Text, |s, id| s.walls.get(id).map(|row| join_text(row.end_join)), |_, id, value| Some(ModelMutation::SetWallEndJoin(crate::standards::v1::subsets::any::schema::mutations::set_wall_end_join::SetWallEndJoin { id: id.into(), end: crate::standards::v1::subsets::any::schema::mutations::modify::WallEnd::End, join: parse_join(value)? }))),
];

pub static CURTAIN_WALL_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.curtain_walls.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.curtain_walls.get(id).map(|row| row.storey.clone()), choices: phasing::storey_choices, write: phasing::write_storey),
    field!("phase", field_phase, Text, |s, id| s.curtain_walls.get(id).map(|row| format!("{:?}", row.phase)), choices: phasing::phase_choices, write: phasing::write_phase),
    field!("axis", field_axis, Text, |s, id| s.curtain_walls.get(id).map(|row| axis_text(&row.axis)), parse_axis => set_curtain_wall::SetCurtainWall),
    field!("base_offset", field_base_offset, Number, |s, id| s.curtain_walls.get(id).map(|row| number(row.base_offset)), parse_number => set_curtain_wall::SetCurtainWall),
    field!("top", field_top, Text, |s, id| s.curtain_walls.get(id).map(|row| top_text(&row.top)), parse_top => set_curtain_wall::SetCurtainWall),
    field!("curtain_wall_type", field_curtain_wall_type, Text, |s, id| s.curtain_walls.get(id).map(|row| row.curtain_wall_type.clone()), choices: frame::type_choices, write: frame::write_curtain_wall_type),
    field!("u_grid", field_u_grid, Text, |s, id| s.curtain_walls.get(id).map(|row| frame::override_text(row.u_grid.as_ref())), |s, id, value| frame::write_u_grid(s, id, value)),
    field!("v_grid", field_v_grid, Text, |s, id| s.curtain_walls.get(id).map(|row| frame::override_text(row.v_grid.as_ref())), |s, id, value| frame::write_v_grid(s, id, value)),
];

pub static COLUMN_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.columns.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.columns.get(id).map(|row| row.storey.clone()), choices: phasing::storey_choices, write: phasing::write_storey),
    field!("phase", field_phase, Text, |s, id| s.columns.get(id).map(|row| format!("{:?}", row.phase)), choices: phasing::phase_choices, write: phasing::write_phase),
    field!("column_type", field_column_type, Text, |s, id| s.columns.get(id).map(|row| row.column_type.clone()), parse_text => set_column::SetColumn),
    field!("position", field_position, Text, |s, id| s.columns.get(id).map(|row| point(row.position.x, row.position.y)), parse_point => set_column::SetColumn),
    field!("rotation", field_rotation, Number, |s, id| s.columns.get(id).map(|row| number(row.rotation)), parse_number => set_column::SetColumn),
    field!("tilt", field_tilt, Text, |s, id| s.columns.get(id).map(|row| slope_text(row.tilt)), |s, id, value| frame::write_tilt(s, id, value)),
    field!("base_offset", field_base_offset, Number, |s, id| s.columns.get(id).map(|row| number(row.base_offset)), parse_number => set_column::SetColumn),
    field!("top", field_top, Text, |s, id| s.columns.get(id).map(|row| top_text(&row.top)), parse_top => set_column::SetColumn),
];

pub static BEAM_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.beams.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.beams.get(id).map(|row| row.storey.clone()), choices: phasing::storey_choices, write: phasing::write_storey),
    field!("phase", field_phase, Text, |s, id| s.beams.get(id).map(|row| format!("{:?}", row.phase)), choices: phasing::phase_choices, write: phasing::write_phase),
    field!("beam_type", field_beam_type, Text, |s, id| s.beams.get(id).map(|row| row.beam_type.clone()), parse_text => set_beam::SetBeam),
    field!("axis", field_axis, Text, |s, id| s.beams.get(id).map(|row| axis_text(&row.axis)), |s, id, value| frame::write_beam_axis(s, id, value)),
    field!("top_offset", field_offset, Number, |s, id| s.beams.get(id).map(|row| number(row.top_offset)), parse_number => set_beam::SetBeam),
    field!("end_top_offset", field_end_offset, Number, |s, id| s.beams.get(id).map(|row| row.end_top_offset.map_or_else(String::new, number)), |s, id, value| frame::write_end_offset(s, id, value)),
];

pub static SLAB_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.slabs.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.slabs.get(id).map(|row| row.storey.clone()), choices: phasing::storey_choices, write: phasing::write_storey),
    field!("phase", field_phase, Text, |s, id| s.slabs.get(id).map(|row| format!("{:?}", row.phase)), choices: phasing::phase_choices, write: phasing::write_phase),
    field!("slab_type", field_slab_type, Text, |s, id| s.slabs.get(id).map(|row| row.slab_type.clone()), parse_text => set_slab::SetSlab),
    field!("offset", field_offset, Number, |s, id| s.slabs.get(id).map(|row| number(row.offset)), parse_number => set_slab::SetSlab),
    field!("boundary", field_boundary, Text, |s, id| s.slabs.get(id).map(|row| loop_text(&row.boundary)), |s, id, value| Some(ModelMutation::SetSlabBoundary(crate::standards::v1::subsets::any::schema::mutations::set_slab_boundary::SetSlabBoundary { id: id.into(), boundary: parse_loop(value)?, holes: s.slabs.get(id)?.holes.clone() }))),
    field!("holes", field_holes, Text, |s, id| s.slabs.get(id).map(|row| holes_text(&row.holes)), |s, id, value| Some(ModelMutation::SetSlabBoundary(crate::standards::v1::subsets::any::schema::mutations::set_slab_boundary::SetSlabBoundary { id: id.into(), boundary: s.slabs.get(id)?.boundary.clone(), holes: parse_holes(value)? }))),
];

pub static ROOF_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.roofs.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.roofs.get(id).map(|row| row.storey.clone()), choices: phasing::storey_choices, write: phasing::write_storey),
    field!("phase", field_phase, Text, |s, id| s.roofs.get(id).map(|row| format!("{:?}", row.phase)), choices: phasing::phase_choices, write: phasing::write_phase),
    field!("roof_type", field_roof_type, Text, |s, id| s.roofs.get(id).map(|row| row.roof_type.clone())),
    field!("shape", field_shape, Text, |s, id| s.roofs.get(id).map(|row| roof_shape_text(&row.shape)), parse_roof_shape => set_roof_shape::SetRoofShape),
    field!("footprint", field_footprint, Text, |s, id| s.roofs.get(id).map(|row| loop_text(&row.footprint)), parse_loop => set_roof_footprint::SetRoofFootprint),
    field!("overhang", field_overhang, Number, |s, id| s.roofs.get(id).map(|row| number(row.overhang)), parse_number => set_roof_shape::SetRoofShape),
    field!("base_offset", field_base_offset, Number, |s, id| s.roofs.get(id).map(|row| number(row.base_offset)), parse_number => set_roof_shape::SetRoofShape),
];

pub fn opening_kind(snapshot: &ModelSnapshot, id: &str, value: &str) -> Option<ModelMutation> {
    let kind = match &snapshot.openings.get(id)?.kind {
        OpeningKind::Window { .. } => OpeningKind::Window { window_type: value.trim().to_string() },
        OpeningKind::Door { .. } => OpeningKind::Door { door_type: value.trim().to_string() },
        OpeningKind::Void { .. } => {
            let (width, height) = value.split_once('×')?;
            OpeningKind::Void { width: parse_number(width)?, height: parse_number(height)? }
        }
    };
    set!(set_opening::SetOpening, id, "kind", &kind)
}

pub static OPENING_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.openings.get(id).map(|row| row.name.clone()), rename),
    field!("host", field_host, Text, |s, id| s.openings.get(id).map(|row| row.host.clone()), |s, id, value| Some(ModelMutation::MoveOpening(crate::standards::v1::subsets::any::schema::mutations::move_opening::MoveOpening { id: id.into(), offset: s.openings.get(id)?.offset, host: Some(value.trim().into()) }))),
    field!("kind", field_kind, Text, |s, id| s.openings.get(id).map(|row| opening_text(&row.kind)), |s, id, value| opening_kind(s, id, value)),
    field!("offset", field_offset, Number, |s, id| s.openings.get(id).map(|row| number(row.offset)), parse_number => move_opening::MoveOpening),
    field!("sill_override", field_sill, Number, |s, id| s.openings.get(id).map(|row| number(row.sill_override.unwrap_or_else(|| type_sill(s, &row.kind)))), |_, id, value| set!(set_opening::SetOpening, id, "sill_override", &Assigned::new(parse_optional_number(value)?))),
    field!("width", field_width, Number, |s, id| s.openings.get(id).and_then(|row| row.width).map(number), |_, id, value| set!(set_opening::SetOpening, id, "width", &Assigned::new(Some(parse_number(value)?)))),
    field!("height", field_height, Number, |s, id| s.openings.get(id).and_then(|row| row.height).map(number), |_, id, value| set!(set_opening::SetOpening, id, "height", &Assigned::new(Some(parse_number(value)?)))),
    field!("flip_hand", field_flip_hand, Text, |s, id| s.openings.get(id).map(|row| row.flip_hand.to_string()), parse_flag => set_opening::SetOpening),
    field!("flip_facing", field_flip_facing, Text, |s, id| s.openings.get(id).map(|row| row.flip_facing.to_string()), parse_flag => set_opening::SetOpening),
    field!("reveal_depth", field_reveal_depth, Number, |s, id| wall_sweeps::reveal_depth_text(s, id), |s, id, value| wall_sweeps::write_reveal_depth(s, id, value)),
    field!("reveal_material", field_reveal_material, Text, |s, id| wall_sweeps::reveal_material_text(s, id), choices: zoning::material_choices, write: wall_sweeps::write_reveal_material),
];

pub static STAIR_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.stairs.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.stairs.get(id).map(|row| row.storey.clone()), choices: phasing::storey_choices, write: phasing::write_storey),
    field!("phase", field_phase, Text, |s, id| s.stairs.get(id).map(|row| format!("{:?}", row.phase)), choices: phasing::phase_choices, write: phasing::write_phase),
    field!("start", field_start, Text, |s, id| s.stairs.get(id).map(|row| point(row.start.x, row.start.y)), parse_point => set_stair::SetStair),
    field!("direction", field_rotation, Number, |s, id| s.stairs.get(id).map(|row| number(row.direction)), parse_number => set_stair::SetStair),
    field!("width", field_width, Number, |s, id| s.stairs.get(id).map(|row| number(row.width)), parse_number => set_stair::SetStair),
    field!("flight", field_flight, Text, |s, id| s.stairs.get(id).map(|row| flight_text(&row.flight)), parse_flight => set_stair::SetStair),
    field!("top", field_top, Text, |s, id| s.stairs.get(id).map(|row| top_text(&row.top)), parse_top => set_stair::SetStair),
    field!("max_riser", field_max_riser, Number, |s, id| s.stairs.get(id).map(|row| number(row.max_riser)), parse_number => set_stair::SetStair),
    field!("min_tread", field_min_tread, Number, |s, id| s.stairs.get(id).map(|row| number(row.min_tread)), parse_number => set_stair::SetStair),
    field!("stringer", field_stringer, Text, |s, id| s.stairs.get(id).map(|row| stringer_text(&row.stringer)), parse_stringer => set_stair::SetStair),
    field!("nosing", field_nosing, Number, |s, id| s.stairs.get(id).map(|row| number(row.nosing)), parse_number => set_stair::SetStair),
    field!("tread_thickness", field_tread_thickness, Number, |s, id| s.stairs.get(id).map(|row| number(row.tread_thickness)), parse_number => set_stair::SetStair),
    field!("riser", field_riser, Text, |s, id| s.stairs.get(id).map(|row| format!("{:?}", row.riser)), parse_riser => set_stair::SetStair),
    field!("landing_depth", field_landing_depth, Number, |s, id| s.stairs.get(id).map(|row| number(row.landing_depth)), parse_number => set_stair::SetStair),
];

pub static RAILING_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.railings.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.railings.get(id).map(|row| row.storey.clone()), choices: phasing::storey_choices, write: phasing::write_storey),
    field!("phase", field_phase, Text, |s, id| s.railings.get(id).map(|row| format!("{:?}", row.phase)), choices: phasing::phase_choices, write: phasing::write_phase),
    field!("height", field_height, Number, |s, id| s.railings.get(id).map(|row| number(row.height)), parse_number => set_railing::SetRailing),
    field!("post_spacing", field_post_spacing, Number, |s, id| s.railings.get(id).map(|row| number(row.post_spacing)), parse_number => set_railing::SetRailing),
    field!("profile", field_rail_profile, Text, |s, id| s.railings.get(id).map(|row| profile_text(&row.profile)), parse_profile => set_railing::SetRailing),
    field!("post_profile", field_post_profile, Text, |s, id| s.railings.get(id).map(|row| profile_text(&row.post_profile)), parse_profile => set_railing::SetRailing),
    field!("baluster", field_baluster, Text, |s, id| s.railings.get(id).map(|row| baluster_text(row.baluster.as_ref())), |_, id, value| set!(set_railing::SetRailing, id, "baluster", &Assigned::new(parse_baluster(value)?))),
    field!("infill", field_infill, Text, |s, id| s.railings.get(id).map(|row| infill_text(&row.infill)), parse_infill => set_railing::SetRailing),
    field!("material", field_material, Text, |s, id| s.railings.get(id).map(|row| row.material.clone()), parse_text => set_railing::SetRailing),
    field!("base_offset", field_base_offset, Number, |s, id| s.railings.get(id).map(|row| number(row.base_offset)), parse_number => set_railing::SetRailing),
    field!("host", field_host, Text, |s, id| s.railings.get(id).map(|row| row.host.as_ref().map(|host| host.element.clone()).unwrap_or_default()), choices: ramps::host_choices, write: ramps::write_host),
    field!("host_side", field_host_side, Text, |s, id| s.railings.get(id).map(|row| row.host.as_ref().map(|host| format!("{:?}", host.side)).unwrap_or_default()), choices: ramps::side_choices, write: ramps::write_host_side),
    field!("host_edge", field_host_edge, Number, |s, id| s.railings.get(id).map(|row| row.host.as_ref().map(|host| host.edge.to_string()).unwrap_or_default()), |s, id, value| ramps::write_host_edge(s, id, value)),
    field!("host_inset", field_host_inset, Number, |s, id| s.railings.get(id).map(|row| row.host.as_ref().map(|host| number(host.inset)).unwrap_or_default()), |s, id, value| ramps::write_host_inset(s, id, value)),
];

pub static SPACE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.spaces.get(id).map(|row| row.name.clone()), rename),
    field!("storey", field_storey, Text, |s, id| s.spaces.get(id).map(|row| row.storey.clone()), choices: phasing::storey_choices, write: phasing::write_storey),
    field!("phase", field_phase, Text, |s, id| s.spaces.get(id).map(|row| format!("{:?}", row.phase)), choices: phasing::phase_choices, write: phasing::write_phase),
    field!("number", field_number, Text, |s, id| s.spaces.get(id).map(|row| row.number.clone()), parse_text => set_space::SetSpace),
    field!("usage", field_usage, Text, |s, id| s.spaces.get(id).map(|row| row.usage.clone()), parse_text => set_space::SetSpace),
    field!("zone", field_zone, Text, |s, id| s.spaces.get(id).map(|row| row.zone.clone().unwrap_or_default()), choices: zoning::zone_choices, write: zoning::write_zone),
    field!("floor_finish", field_floor_finish, Text, |s, id| s.spaces.get(id).map(|row| row.floor_finish.clone().unwrap_or_default()), choices: zoning::material_choices, write: zoning::write_floor_finish),
    field!("wall_finish", field_wall_finish, Text, |s, id| s.spaces.get(id).map(|row| row.wall_finish.clone().unwrap_or_default()), choices: zoning::material_choices, write: zoning::write_wall_finish),
    field!("ceiling_finish", field_ceiling_finish, Text, |s, id| s.spaces.get(id).map(|row| row.ceiling_finish.clone().unwrap_or_default()), choices: zoning::material_choices, write: zoning::write_ceiling_finish),
    field!("occupancy", field_occupancy_type, Text, |s, id| energy::condition(s, id, |c| c.occupancy.clone()), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "occupancy", &Assigned::new(energy::stated(value)))),
    field!("occupancy_density", field_occupancy_density, Number, |s, id| energy::condition(s, id, |c| c.occupancy_density.map(number)), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "occupancy_density", &Assigned::new(parse_optional_number(value)?))),
    field!("heating_setpoint", field_heating_setpoint, Number, |s, id| energy::condition(s, id, |c| c.heating_setpoint.map(number)), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "heating_setpoint", &Assigned::new(parse_optional_number(value)?))),
    field!("cooling_setpoint", field_cooling_setpoint, Number, |s, id| energy::condition(s, id, |c| c.cooling_setpoint.map(number)), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "cooling_setpoint", &Assigned::new(parse_optional_number(value)?))),
    field!("ventilation_rate", field_ventilation_rate, Number, |s, id| energy::condition(s, id, |c| c.ventilation_rate.map(number)), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "ventilation_rate", &Assigned::new(parse_optional_number(value)?))),
    field!("lighting_power_density", field_lighting_power_density, Number, |s, id| energy::condition(s, id, |c| c.lighting_power_density.map(number)), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "lighting_power_density", &Assigned::new(parse_optional_number(value)?))),
    field!("equipment_power_density", field_equipment_power_density, Number, |s, id| energy::condition(s, id, |c| c.equipment_power_density.map(number)), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "equipment_power_density", &Assigned::new(parse_optional_number(value)?))),
    field!("schedule", field_schedule_profile, Text, |s, id| energy::condition(s, id, |c| c.schedule.clone()), |_, id, value| set!(set_space_conditions::SetSpaceConditions, id, "schedule", &Assigned::new(energy::stated(value)))),
];

pub static MATERIAL_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.materials.get(id).map(|row| row.name.clone()), parse_text => set_material::SetMaterial),
    field!("category", field_category, Text, |s, id| s.materials.get(id).map(|row| format!("{:?}", row.category)), parse_category => set_material::SetMaterial),
    field!("density", field_density, Number, |s, id| s.materials.get(id).map(|row| number(row.density)), parse_number => set_material::SetMaterial),
    field!("conductivity", field_conductivity, Number, |s, id| s.materials.get(id).map(|row| number(row.conductivity)), parse_number => set_material::SetMaterial),
    field!("specific_heat", field_specific_heat, Number, |s, id| s.materials.get(id).map(|row| number(row.specific_heat)), parse_number => set_material::SetMaterial),
];

pub static WALL_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.wall_types.get(id).map(|row| row.name.clone()), parse_text => set_wall_type::SetWallType),
    field!("layers", field_layers, Text, |s, id| s.wall_types.get(id).map(|row| row.layers.len().to_string())),
    field!("thickness", field_thickness, Number, |s, id| s.wall_types.get(id).map(|row| number(row.layers.iter().map(|layer| layer.thickness).sum()))),
];

pub static SLAB_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.slab_types.get(id).map(|row| row.name.clone()), parse_text => set_slab_type::SetSlabType),
    field!("layers", field_layers, Text, |s, id| s.slab_types.get(id).map(|row| row.layers.len().to_string())),
    field!("thickness", field_thickness, Number, |s, id| s.slab_types.get(id).map(|row| number(row.layers.iter().map(|layer| layer.thickness).sum()))),
];

pub static ROOF_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.roof_types.get(id).map(|row| row.name.clone()), parse_text => set_roof_type::SetRoofType),
    field!("layers", field_layers, Text, |s, id| s.roof_types.get(id).map(|row| row.layers.len().to_string())),
    field!("thickness", field_thickness, Number, |s, id| s.roof_types.get(id).map(|row| number(row.layers.iter().map(|layer| layer.thickness).sum()))),
];

pub static COLUMN_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.column_types.get(id).map(|row| row.name.clone()), parse_text => set_column_type::SetColumnType),
    field!("material", field_material, Text, |s, id| s.column_types.get(id).map(|row| row.material.clone()), parse_text => set_column_type::SetColumnType),
];

pub static BEAM_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.beam_types.get(id).map(|row| row.name.clone()), parse_text => set_beam_type::SetBeamType),
    field!("material", field_material, Text, |s, id| s.beam_types.get(id).map(|row| row.material.clone()), parse_text => set_beam_type::SetBeamType),
];

pub static WINDOW_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.window_types.get(id).map(|row| row.name.clone()), parse_text => set_window_type::SetWindowType),
    field!("width", field_width, Number, |s, id| s.window_types.get(id).map(|row| number(row.width)), parse_number => set_window_type::SetWindowType),
    field!("height", field_height, Number, |s, id| s.window_types.get(id).map(|row| number(row.height)), parse_number => set_window_type::SetWindowType),
    field!("sill", field_sill, Number, |s, id| s.window_types.get(id).map(|row| number(row.sill)), parse_number => set_window_type::SetWindowType),
    field!("frame_width", field_frame_width, Number, |s, id| s.window_types.get(id).map(|row| number(row.frame_width)), parse_number => set_window_type::SetWindowType),
    field!("frame_depth", field_frame_depth, Number, |s, id| s.window_types.get(id).map(|row| number(row.frame_depth)), parse_number => set_window_type::SetWindowType),
    field!("panes", field_panes, Number, |s, id| s.window_types.get(id).map(|row| row.panes.to_string()), parse_count => set_window_type::SetWindowType),
    field!("material", field_material, Text, |s, id| s.window_types.get(id).map(|row| row.material.clone()), parse_text => set_window_type::SetWindowType),
    field!("u_value", field_u_value, Number, |s, id| s.window_types.get(id).map(|row| row.u_value.map_or_else(String::new, number)), |_, id, value| set!(set_type_thermal_data::SetTypeThermalData, id, "u_value", &Assigned::new(parse_optional_number(value)?))),
    field!("g_value", field_g_value, Number, |s, id| s.window_types.get(id).map(|row| row.g_value.map_or_else(String::new, number)), |_, id, value| set!(set_type_thermal_data::SetTypeThermalData, id, "g_value", &Assigned::new(parse_optional_number(value)?))),
    field!("frame_fraction", field_frame_fraction, Number, |s, id| s.window_types.get(id).map(|row| row.frame_fraction.map_or_else(String::new, number)), |_, id, value| set!(set_type_thermal_data::SetTypeThermalData, id, "frame_fraction", &Assigned::new(parse_optional_number(value)?))),
];

pub static DOOR_TYPE_FIELDS: &[FieldRow] = &[
    field!("name", field_name, Text, |s, id| s.door_types.get(id).map(|row| row.name.clone()), parse_text => set_door_type::SetDoorType),
    field!("width", field_width, Number, |s, id| s.door_types.get(id).map(|row| number(row.width)), parse_number => set_door_type::SetDoorType),
    field!("height", field_height, Number, |s, id| s.door_types.get(id).map(|row| number(row.height)), parse_number => set_door_type::SetDoorType),
    field!("frame_width", field_frame_width, Number, |s, id| s.door_types.get(id).map(|row| number(row.frame_width)), parse_number => set_door_type::SetDoorType),
    field!("frame_depth", field_frame_depth, Number, |s, id| s.door_types.get(id).map(|row| number(row.frame_depth)), parse_number => set_door_type::SetDoorType),
    field!("leaves", field_leaves, Text, |s, id| s.door_types.get(id).map(|row| format!("{:?}", row.leaves)), parse_leaves => set_door_type::SetDoorType),
    field!("swing", field_swing, Text, |s, id| s.door_types.get(id).map(|row| format!("{:?}", row.swing)), parse_swing => set_door_type::SetDoorType),
    field!("material", field_material, Text, |s, id| s.door_types.get(id).map(|row| row.material.clone()), parse_text => set_door_type::SetDoorType),
    field!("u_value", field_u_value, Number, |s, id| s.door_types.get(id).map(|row| row.u_value.map_or_else(String::new, number)), |_, id, value| set!(set_type_thermal_data::SetTypeThermalData, id, "u_value", &Assigned::new(parse_optional_number(value)?))),
];
//#endregion 🔖️Fields

//#region 🔖️Inferred
pub static STOREY_INFERRED: &[InferredRow] = &[
    inferred!("elevation", field_elevation, |_, inference, id| inference.storey_levels.get(id).map(|row| number(row.elevation))),
    inferred!("top_elevation", field_top_elevation, |_, inference, id| inference.storey_levels.get(id).map(|row| number(row.top_elevation))),
];

/// 💡️ The quantities of a placed element, read off the `quantities` inference; a measure that does not apply to the kind is 0 and shows no row.
pub static QUANTITY_INFERRED: &[InferredRow] = &[
    inferred!("elevation", field_elevation, |_, inference, id| inference.wall_layout.get(id).map(|row| number(row.base_z))),
    inferred!("top_elevation", field_top_elevation, |_, inference, id| inference.wall_layout.get(id).map(|row| number(row.top_z))),
    inferred!("length", field_length, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.length > 0.0).map(|row| number(row.length))),
    inferred!("width", field_width, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.width > 0.0).map(|row| number(row.width))),
    inferred!("height", field_height, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.height > 0.0).map(|row| number(row.height))),
    inferred!("area", field_area, |_, inference, id| inference.quantities.elements.get(id).map(|row| row.area()).filter(|area| *area > 0.0).map(number)),
    inferred!("volume", field_volume, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.net_volume > 0.0).map(|row| number(row.net_volume))),
    inferred!("mass", field_mass, |_, inference, id| inference.quantities.elements.get(id).filter(|row| row.mass > 0.0).map(|row| number(row.mass))),
    inferred!("floor_finish_area", field_floor_finish_area, |_, inference, id| zoning::finish_area(inference, id, zoning::FinishSurface::Floor)),
    inferred!("wall_finish_area", field_wall_finish_area, |_, inference, id| zoning::finish_area(inference, id, zoning::FinishSurface::Wall)),
    inferred!("ceiling_finish_area", field_ceiling_finish_area, |_, inference, id| zoning::finish_area(inference, id, zoning::FinishSurface::Ceiling)),
    inferred!("envelope_surfaces", field_envelope_surfaces, |_, inference, id| energy::envelope(inference, id, |_, facts| facts.surfaces.to_string())),
    inferred!("envelope_area", field_envelope_area, |_, inference, id| energy::envelope(inference, id, |_, facts| energy::fixed(facts.envelope_area))),
    inferred!("glazing_area", field_glazing_area, |_, inference, id| energy::envelope(inference, id, |_, facts| energy::fixed(facts.glazing_area))),
    inferred!("glazing_ratio", field_glazing_ratio, |_, inference, id| energy::envelope(inference, id, |_, facts| energy::fixed(facts.glazing_ratio))),
    inferred!("open_length", field_open_length, |_, inference, id| energy::envelope(inference, id, |space, _| energy::fixed(space.open_length)).filter(|length| length != "0.00")),
    inferred!("envelope_findings", field_envelope_findings, |_, inference, id| energy::envelope(inference, id, |space, _| space.issues.len().to_string()).filter(|count| count != "0")),
    inferred!("zone_h_t_prime", field_zone_h_t_prime, |s, inference, id| energy::zone_of_space(s, inference, id, |totals| totals.h_t_prime)),
    inferred!("zone_a_over_v", field_zone_a_over_v, |s, inference, id| energy::zone_of_space(s, inference, id, |totals| totals.a_over_v)),
    inferred!("zone_envelope_area", field_zone_envelope_area, |s, inference, id| energy::zone_of_space(s, inference, id, |totals| totals.envelope_area)),
    inferred!("zone_glazing_ratio", field_zone_glazing_ratio, |s, inference, id| energy::zone_of_space(s, inference, id, |totals| totals.glazing_ratio)),
];
//#endregion 🔖️Inferred
