//! 🏋️ Handwritten EN1991 site, load assumptions and nested accidental-case identities.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::En1991Snapshot;
use crate::{CaseNumber, CaseRow, CaseText, AccidentalCase, AccidentalExplosion, AccidentalImpact, FireLoadBasis, FireMode, FloorArea, RoofArea, SelfWeightElement, StructureKind, WindFace, BridgeLane, ConstructionStage, RoofSurface, Placement, SpecialVehicle, WindBridge, DynamicElement, ImpactAction, InternalExplosion, ThermalSection, HelicopterLanding, TieRun, ConstructionLoad, FallingLoad, ExecutionImperfection, DerailmentCase, CraneHoist, Footbridge, RailDynamic, LoadDispersal, BulkSolid, RotatingMachine, FireOccupancy, FatigueModel, SupportCollision, LocalFire, SnowGuard, RailDerailment, RailLoadGroup, DustExplosion};
use semio_s_plugin_norm_compliance::AnnexChoice;
use crate::part_1_2::FireCurve;
use std::collections::BTreeMap;
use store::{os_store::serialization::ArtifactSqliteSnapshotHost, sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, validate_sqlite_database_schema, artifact::{Cell, FloatColumn, FloatRow, NativeEncodingBound, RowWriter, reconstruct_text}}};
use semio_framework_value::{ValueError, ValueRefusalKind};
fn fire_occupancy_text(a:FireOccupancy)->&'static str{match a{FireOccupancy::Dwelling=>"dwelling",FireOccupancy::Office=>"office",FireOccupancy::Hospital=>"hospital",FireOccupancy::School=>"school",FireOccupancy::Hotel=>"hotel",FireOccupancy::Library=>"library",FireOccupancy::Shopping=>"shopping",FireOccupancy::Theatre=>"theatre",FireOccupancy::Transport=>"transport",}}
fn read_fire_occupancy(a:&str)->Result<FireOccupancy, ValueError>{match a{"dwelling"=>Ok(FireOccupancy::Dwelling),"office"=>Ok(FireOccupancy::Office),"hospital"=>Ok(FireOccupancy::Hospital),"school"=>Ok(FireOccupancy::School),"hotel"=>Ok(FireOccupancy::Hotel),"library"=>Ok(FireOccupancy::Library),"shopping"=>Ok(FireOccupancy::Shopping),"theatre"=>Ok(FireOccupancy::Theatre),"transport"=>Ok(FireOccupancy::Transport),_=>Err(invalid("EN1991 fire occupancy differs"))}}
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }

const DOCUMENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(3), FloatColumn::Binary64(4), FloatColumn::Binary64(7), FloatColumn::Binary64(10), FloatColumn::Binary64(11), FloatColumn::Binary64(13), FloatColumn::Binary64(14), FloatColumn::Binary64(15), FloatColumn::Binary64(16), FloatColumn::Binary64(17), FloatColumn::Binary64(18), FloatColumn::Binary64(19), FloatColumn::Binary64(20), FloatColumn::Binary64(23), FloatColumn::Binary64(27), FloatColumn::Binary64(28), FloatColumn::Binary64(29), FloatColumn::Binary64(30), FloatColumn::Binary64(31), FloatColumn::Binary64(32), FloatColumn::Binary64(33), FloatColumn::Binary64(35), FloatColumn::Binary64(36), FloatColumn::Binary64(38), FloatColumn::Binary64(41), FloatColumn::Binary64(42), FloatColumn::Binary64(43), FloatColumn::Binary64(44), FloatColumn::Binary64(45), FloatColumn::Binary64(46), FloatColumn::Binary64(47), FloatColumn::Binary64(48), FloatColumn::Binary64(53), FloatColumn::Binary64(54), FloatColumn::Binary64(55), FloatColumn::Binary64(58), FloatColumn::Binary64(59), FloatColumn::Binary64(60), FloatColumn::Binary64(61), FloatColumn::Binary64(62), FloatColumn::Binary64(63), FloatColumn::Binary64(64), FloatColumn::Binary64(65), FloatColumn::Binary64(66), FloatColumn::Binary64(67), FloatColumn::Binary64(68), FloatColumn::Binary64(69), FloatColumn::Binary64(70), FloatColumn::Binary64(71), FloatColumn::Binary64(72), FloatColumn::Binary64(73), FloatColumn::Binary64(74), FloatColumn::Binary64(75), FloatColumn::Binary64(76), FloatColumn::Binary64(77), FloatColumn::Binary64(79)];
const FLOOR_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8)];
const SELF_WEIGHT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6)];
const ROOF_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(12)];
const WIND_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(11), FloatColumn::Binary64(12)];
const ACCIDENTAL_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(3), FloatColumn::Binary64(4), FloatColumn::Binary64(5)];
const ROTATING_MACHINE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6)];
const FATIGUE_MODEL_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6),FloatColumn::Binary64(7)];
const SUPPORT_COLLISION_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];
const BULK_SOLID_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4)];
const LOAD_DISPERSAL_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7)];
const RAIL_DYNAMIC_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4)];
const FOOTBRIDGE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6),FloatColumn::Binary64(7)];
const CRANE_HOIST_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5)];
const DERAILMENT_CASE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9)];
const EXECUTION_IMPERFECTION_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9)];
const FALLING_LOAD_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9)];
const CONSTRUCTION_LOAD_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6)];
const TIE_RUN_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(11), FloatColumn::Binary64(12)];
const HELICOPTER_LANDING_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8)];
const CASE_NUMBER_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4)];
const THERMAL_SECTION_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9)];
const INTERNAL_EXPLOSION_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7)];
const IMPACT_ACTION_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(11)];
const DYNAMIC_ELEMENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10)];
const WIND_BRIDGE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(7), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(11)];
const SPECIAL_VEHICLE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6)];
const ROOF_SURFACE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6), FloatColumn::Binary64(7)];
const PLACEMENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7)];
const CONSTRUCTION_STAGE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7)];
const BRIDGE_LANE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5)];
const LOCAL_FIRE: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];
const SNOW_GUARD: &[FloatColumn] = &[FloatColumn::Binary64(5),FloatColumn::Binary64(6)];
const RAIL_DERAILMENT: &[FloatColumn] = &[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];
const RAIL_LOAD_GROUP: &[FloatColumn] = &[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11)];
const DUST_EXPLOSION: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9)];
type Entities<'a> = BTreeMap<i64, FloatRow<'a>>;

fn checkpoint(control: &mut SqliteSnapshotControl<'_>, position: usize, total: usize) -> Result<(), ValueError> {
    if position % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, position, total)?; }
    Ok(())
}
fn entities<'a>(database: &'a SqliteDatabase, table: &str, columns: usize, floats: &'static [FloatColumn], control: &mut SqliteSnapshotControl<'_>) -> Result<Entities<'a>, ValueError> {
    let rows = &database.table(table)?.rows;
    let mut result = Entities::new();
    for (position, row) in rows.iter().enumerate() {
        checkpoint(control, position, rows.len())?;
        let row = FloatRow::new(row, floats)?;
        if row.values.len() != columns || row.rowid <= 0 || row.integer(0)? != row.rowid || result.insert(row.rowid, row).is_some() { return Err(invalid(format!("{table} requires exact fields and unique positive aliased identities"))); }
    }
    Ok(result)
}
fn ordered<'a>(rows: impl Iterator<Item = FloatRow<'a>>, parent: i64, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<FloatRow<'a>>, ValueError> {
    let mut result = BTreeMap::new();
    for (position, row) in rows.enumerate() {
        checkpoint(control, position, 0)?;
        let ordinal = usize::try_from(row.integer(2)?).map_err(|error| invalid(error.to_string()))?;
        if row.integer(1)? != parent || result.insert(ordinal, row).is_some() { return Err(invalid("EN1991 entity has a foreign parent or duplicate ordinal")); }
    }
    for (position, ordinal) in result.keys().copied().enumerate() { checkpoint(control, position, result.len())?; if position != ordinal { return Err(invalid("EN1991 ordinals must be dense")); } }
    Ok(result.into_values().collect())
}
fn child_groups<'a>(rows: Entities<'a>, parents: &Entities<'_>, control: &mut SqliteSnapshotControl<'_>) -> Result<BTreeMap<i64, Vec<FloatRow<'a>>>, ValueError> {
    let mut result = BTreeMap::<i64, Vec<FloatRow<'a>>>::new();
    for (position, row) in rows.into_values().enumerate() {
        checkpoint(control, position, 0)?;
        let parent = row.integer(1)?;
        if !parents.contains_key(&parent) { return Err(invalid("EN1991 accidental child has an unknown owning case")); }
        result.entry(parent).or_default().push(row);
    }
    Ok(result)
}
fn ordinal(value: usize) -> Result<Cell<'static>, ValueError> { Ok(Cell::Integer(i64::try_from(value).map_err(|error| invalid(error.to_string()))?)) }
fn flag(value: bool) -> Cell<'static> { Cell::Integer(i64::from(value)) }
fn read_flag(row: FloatRow<'_>, column: usize) -> Result<bool, ValueError> { match row.integer(column)? { 0 => Ok(false), 1 => Ok(true), _ => Err(invalid("EN1991 boolean requires exactly zero or one")) } }
fn read_u8(row: FloatRow<'_>, column: usize) -> Result<u8, ValueError> { u8::try_from(row.integer(column)?).map_err(|error| invalid(error.to_string())) }

impl En1991Snapshot {
    fn write_sqlite_rows(&self, out: &mut RowWriter<'_,'_>) -> Result<(), ValueError> {
        
        out.insert_key_float( "en1991_document", 1, &[
            Cell::Text(match self.annex { AnnexChoice::En => "En", AnnexChoice::De => "De" }), Cell::Text(&self.snow_zone), Cell::Real(self.altitude), Cell::Real(self.en_sk), flag(self.north_german_lowland_snow), Cell::Integer(i64::from(self.wind_zone)), Cell::Real(self.en_vb), Cell::Integer(i64::from(self.terrain_category)), Cell::Integer(i64::from(self.mixed_terrain_upwind)), Cell::Real(self.mixed_terrain_distance), Cell::Real(self.orography_factor), flag(self.coast_or_island), Cell::Real(self.air_density), Cell::Real(self.height), Cell::Real(self.width), Cell::Real(self.depth), Cell::Real(self.assumed_delta_t), Cell::Real(self.t_max), Cell::Real(self.t_min), Cell::Real(self.t_0), Cell::Text(&self.thermal_element_type), Cell::Integer(i64::from(self.thermal_bridge_type)), Cell::Real(self.delta_t_m), Cell::Integer(i64::from(self.storey_count)),
            Cell::Text(match self.fire_mode { FireMode::None => "none", FireMode::Nominal => "nominal", FireMode::Parametric => "parametric" }), Cell::Text(match self.fire_curve { FireCurve::Standard => "standard", FireCurve::External => "external", FireCurve::Hydrocarbon => "hydrocarbon", FireCurve::Parametric => "parametric" }),
            Cell::Real(self.fire_duration), Cell::Real(self.assumed_gas_temperature), Cell::Real(self.assumed_h_net), Cell::Real(self.fire_compartment_area), Cell::Real(self.fire_compartment_height), Cell::Real(self.fire_opening_factor), Cell::Real(self.fire_thermal_inertia), Cell::Text(fire_occupancy_text(self.fire_occupancy)), Cell::Real(self.fire_load_density_qf), Cell::Real(self.assumed_qf_d), Cell::Text(&self.construction_activity), Cell::Real(self.assumed_construction_qk), Cell::Text(match self.structure_kind { StructureKind::Building => "building", StructureKind::Bridge => "bridge" }), Cell::Integer(i64::from(self.bridge_lane)), Cell::Real(self.bridge_span), Cell::Real(self.bridge_lane_width), Cell::Real(self.assumed_bridge_tandem), Cell::Real(self.assumed_bridge_udl), Cell::Real(self.assumed_bridge_lm2), Cell::Real(self.assumed_bridge_footway), Cell::Real(self.assumed_bridge_lm3), Cell::Real(self.assumed_bridge_lm4), Cell::Text(&self.bridge_load_group), flag(self.crane_claimed), Cell::Text(&self.crane_class), Cell::Text(&self.hoist_class), Cell::Real(self.hoisting_speed), Cell::Real(self.assumed_crane_wheel), Cell::Real(self.assumed_crane_horizontal), flag(self.silo_claimed), Cell::Text(&self.silo_kind), Cell::Real(self.silo_bulk_density), Cell::Real(self.silo_height), Cell::Real(self.silo_hydraulic_radius), Cell::Real(self.silo_mu), Cell::Real(self.silo_k), Cell::Real(self.assumed_silo_pressure), Cell::Real(self.assumed_silo_patch), Cell::Real(self.assumed_silo_wall_friction), Cell::Real(self.bridge_tandem_x), Cell::Real(self.rail_alpha), Cell::Real(self.assumed_rail_lm71), Cell::Real(self.assumed_rail_lm71_udl), Cell::Real(self.assumed_rail_sw0), Cell::Real(self.assumed_rail_sw2), Cell::Real(self.crane_x), Cell::Real(self.crane_runway_length), Cell::Real(self.crane_wheel_base), Cell::Real(self.silo_diameter), Cell::Real(self.silo_eccentricity), Cell::Real(self.silo_hopper_angle_deg), Cell::Text(match self.fire_load_basis { FireLoadBasis::OccupancyTable => "occupancyTable", FireLoadBasis::UserSpecified => "userSpecified" }), Cell::Real(self.bridge_curve_radius)
        ], DOCUMENT_FLOATS)?;
        for (position, floor) in self.floors.iter().enumerate() { out.insert_float( "en1991_floor", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&floor.id), Cell::Text(&floor.category), Cell::Real(floor.area), Cell::Real(floor.assumed_qk), Cell::Real(floor.assumed_qk_concentrated), Cell::Real(floor.assumed_partitions)], FLOOR_FLOATS)?; }
        for (position, element) in self.self_weight_elements.iter().enumerate() { out.insert_float( "en1991_self_weight", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&element.id), Cell::Text(&element.material), Cell::Real(element.thickness), Cell::Real(element.assumed_gk)], SELF_WEIGHT_FLOATS)?; }
        for (position, roof) in self.roofs.iter().enumerate() { out.insert_float( "en1991_roof", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&roof.id), Cell::Text(&roof.roof_type), Cell::Real(roof.pitch_deg), Cell::Real(roof.c_e), Cell::Real(roof.c_t), flag(roof.has_parapet), Cell::Real(roof.parapet_height), Cell::Real(roof.drift_obstruction_height), flag(roof.multi_span), Cell::Real(roof.assumed_sk)], ROOF_FLOATS)?; }
        for (position, face) in self.wind_faces.iter().enumerate() { out.insert_float( "en1991_wind_face", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&face.id), Cell::Text(&face.zone), Cell::Real(face.z), Cell::Real(face.c_pe10), Cell::Real(face.c_pe1), Cell::Real(face.c_pi), Cell::Real(face.c_s), Cell::Real(face.c_d), Cell::Real(face.loaded_area), Cell::Real(face.assumed_wp)], WIND_FLOATS)?; }
        for (position, case) in self.accidental_cases.iter().enumerate() {
            let id = out.insert("en1991_accidental_case", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&case.id)])?;
            for (position, impact) in case.impact.iter().enumerate() { out.insert_float( "en1991_impact", &[Cell::Integer(id), ordinal(position)?, Cell::Real(impact.vehicle_mass), Cell::Real(impact.vehicle_speed), Cell::Real(impact.assumed_force)], ACCIDENTAL_FLOATS)?; }
            for (position, explosion) in case.explosion.iter().enumerate() { out.insert_float( "en1991_explosion", &[Cell::Integer(id), ordinal(position)?, Cell::Real(explosion.explosion_mass), Cell::Real(explosion.standoff), Cell::Real(explosion.assumed_pressure)], ACCIDENTAL_FLOATS)?; }
        }
        for (position, row) in self.rotating_machines.iter().enumerate() { out.insert_float( "en1991_rotating_machine", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.mass), Cell::Real(row.eccentricity), Cell::Real(row.frequency)], ROTATING_MACHINE_FLOATS)?; }
        for (position, row) in self.fatigue_models.iter().enumerate() { out.insert_float( "en1991_fatigue_model", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(&row.model), Cell::Integer(i64::from(row.lane)), Cell::Real(row.assumed_axle_load), Cell::Real(row.assumed_udl)], FATIGUE_MODEL_FLOATS)?; }
        for (position, row) in self.support_collisions.iter().enumerate() { out.insert_float( "en1991_support_collision", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(&row.location), Cell::Real(row.assumed_force_x), Cell::Real(row.assumed_force_y), Cell::Real(row.height), Cell::Real(row.position_x)], SUPPORT_COLLISION_FLOATS)?; }
        for (position, row) in self.local_fires.iter().enumerate() { out.insert_float( "en1991_local_fire", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.fire_diameter), Cell::Real(row.heat_release_rate), Cell::Real(row.ceiling_height), Cell::Real(row.horizontal_distance), Cell::Real(row.assumed_flux)], LOCAL_FIRE)?; }
        for (position, row) in self.snow_guards.iter().enumerate() { out.insert_float( "en1991_snow_guard", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(&row.roof_id), Cell::Real(row.slope_length), Cell::Real(row.assumed_force)], SNOW_GUARD)?; }
        for (position, row) in self.rail_derailments.iter().enumerate() { out.insert_float( "en1991_rail_derailment", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(&row.situation), Cell::Real(row.assumed_line_load), Cell::Real(row.loaded_length), Cell::Real(row.offset)], RAIL_DERAILMENT)?; }
        for (position, row) in self.rail_load_groups.iter().enumerate() { out.insert_float( "en1991_rail_load_group", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(&row.load_group), Cell::Real(row.factor_vertical), Cell::Real(row.factor_longitudinal), Cell::Real(row.factor_lateral), Cell::Real(row.assumed_axle_load), Cell::Real(row.assumed_udl), Cell::Real(row.assumed_longitudinal), Cell::Real(row.assumed_lateral)], RAIL_LOAD_GROUP)?; }
        for (position, row) in self.dust_explosions.iter().enumerate() { out.insert_float( "en1991_dust_explosion", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.kst), Cell::Real(row.p_max), Cell::Real(row.volume), Cell::Real(row.vent_area), Cell::Real(row.static_opening_pressure), Cell::Real(row.design_pressure)], DUST_EXPLOSION)?; }
        for (position, row) in self.bulk_solids.iter().enumerate() { out.insert_float( "en1991_bulk_solid", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.internal_friction)], BULK_SOLID_FLOATS)?; }
        for (position, row) in self.load_dispersals.iter().enumerate() { out.insert_float( "en1991_load_dispersal", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.wheel_length), Cell::Real(row.wheel_width), Cell::Real(row.pavement_thickness), Cell::Real(row.slab_thickness)], LOAD_DISPERSAL_FLOATS)?; }
        for (position, row) in self.rail_dynamics.iter().enumerate() { out.insert_float( "en1991_rail_dynamic", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.determinant_length), Cell::Text(&row.maintenance)], RAIL_DYNAMIC_FLOATS)?; }
        for (position, row) in self.footbridges.iter().enumerate() { out.insert_float( "en1991_footbridge", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.loaded_length), Cell::Real(row.deck_width), Cell::Real(row.natural_frequency),Cell::Real(row.assumed_service_axle)], FOOTBRIDGE_FLOATS)?; }
        for (position, row) in self.crane_hoists.iter().enumerate() { out.insert_float( "en1991_crane_hoist", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.rated_load), Cell::Real(row.release_fraction), Cell::Text(&row.release_device)], CRANE_HOIST_FLOATS)?; }
        for (position, row) in self.derailment_cases.iter().enumerate() { out.insert_float( "en1991_derailment_case", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.distance), Cell::Real(row.design_force_x), Cell::Real(row.design_force_y), Cell::Real(row.x), Cell::Real(row.y), Cell::Real(row.assumed_force)], DERAILMENT_CASE_FLOATS)?; }
        for (position, row) in self.execution_imperfections.iter().enumerate() { out.insert_float( "en1991_execution_imperfection", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.height), Cell::Real(row.tolerance), Cell::Real(row.vertical_load), Cell::Real(row.x), Cell::Real(row.y), Cell::Real(row.assumed_force)], EXECUTION_IMPERFECTION_FLOATS)?; }
        for (position, row) in self.falling_loads.iter().enumerate() { out.insert_float( "en1991_falling_load", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.mass), Cell::Real(row.drop_height), Cell::Real(row.stopping_distance), Cell::Real(row.x), Cell::Real(row.y), Cell::Real(row.assumed_force)], FALLING_LOAD_FLOATS)?; }
        for (position, row) in self.construction_loads.iter().enumerate() { out.insert_float( "en1991_construction_load", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(&row.stage), Cell::Integer(i64::from(row.kind)), Cell::Real(row.magnitude)], CONSTRUCTION_LOAD_FLOATS)?; }
        for (position, row) in self.tie_runs.iter().enumerate() { out.insert_float( "en1991_tie_run", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Integer(i64::from(row.kind)), Cell::Real(row.spacing), Cell::Real(row.span), Cell::Real(row.dead_load), Cell::Real(row.imposed_load), Cell::Real(row.psi), Cell::Real(row.x), Cell::Real(row.y), Cell::Real(row.provided_force)], TIE_RUN_FLOATS)?; }
        for (position, row) in self.helicopter_landings.iter().enumerate() { out.insert_float( "en1991_helicopter_landing", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.mass), Cell::Real(row.x), Cell::Real(row.y), Cell::Real(row.height), Cell::Real(row.assumed_force)], HELICOPTER_LANDING_FLOATS)?; }
        for (position, case) in self.cases.iter().enumerate() {
            let id = out.insert_float("en1991_case", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&case.id), Cell::Text(&case.label_en), Cell::Text(&case.label_de), Cell::Text(&case.entry_id)], &[])?;
            for (index, number) in case.numbers.iter().enumerate() { out.insert_float("en1991_case_number", &[Cell::Integer(id), ordinal(index)?, Cell::Text(&number.id), Cell::Real(number.value)], CASE_NUMBER_FLOATS)?; }
            for (index, text) in case.texts.iter().enumerate() { out.insert_float("en1991_case_text", &[Cell::Integer(id), ordinal(index)?, Cell::Text(&text.id), Cell::Text(&text.value)], &[])?; }
        }
        for (position, row) in self.thermal_sections.iter().enumerate() { out.insert_float( "en1991_thermal_section", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Integer(i64::from(row.deck_type)), Cell::Integer(i64::from(row.surfacing)), Cell::Real(row.length), Cell::Real(row.depth), Cell::Real(row.alpha_micro), Cell::Real(row.assumed_range)], THERMAL_SECTION_FLOATS)?; }
        for (position, row) in self.internal_explosions.iter().enumerate() { out.insert_float( "en1991_internal_explosion", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.volume), Cell::Real(row.vent_area), Cell::Real(row.vent_pressure), Cell::Real(row.assumed_pressure)], INTERNAL_EXPLOSION_FLOATS)?; }
        for (position, row) in self.impact_actions.iter().enumerate() { out.insert_float( "en1991_impact_action", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Integer(i64::from(row.kind)), Cell::Real(row.mass), Cell::Real(row.speed), Cell::Real(row.stiffness), Cell::Real(row.height), Cell::Real(row.x), Cell::Real(row.y), Cell::Real(row.assumed_force)], IMPACT_ACTION_FLOATS)?; }
        for (position, row) in self.dynamic_elements.iter().enumerate() { out.insert_float( "en1991_dynamic_element", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.height), Cell::Real(row.width), Cell::Real(row.first_frequency), Cell::Real(row.log_decrement), Cell::Real(row.mass_per_length), Cell::Real(row.strouhal), Cell::Real(row.assumed_cscd)], DYNAMIC_ELEMENT_FLOATS)?; }
        for (position, row) in self.wind_bridges.iter().enumerate() { out.insert_float( "en1991_wind_bridge", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.deck_width), Cell::Real(row.deck_depth), Cell::Integer(i64::from(row.parapet_kind)), Cell::Real(row.parapet_height), Cell::Integer(i64::from(row.traffic_kind)), Cell::Real(row.height_above_ground), Cell::Real(row.span), Cell::Real(row.assumed_force)], WIND_BRIDGE_FLOATS)?; }
        for (position, row) in self.special_vehicles.iter().enumerate() { out.insert_float( "en1991_special_vehicle", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(&row.designation), Cell::Real(row.x), Cell::Real(row.y)], SPECIAL_VEHICLE_FLOATS)?; }
        for (position, row) in self.roof_surfaces.iter().enumerate() { out.insert_float( "en1991_roof_surface", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(&row.face), Cell::Text(&row.shape), Cell::Real(row.pitch_deg), Cell::Real(row.extent)], ROOF_SURFACE_FLOATS)?; }
        for (position, row) in self.construction_stages.iter().enumerate() { out.insert_float( "en1991_construction_stage", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(&row.activity), Cell::Real(row.duration_days), Cell::Real(row.return_years), Cell::Real(row.assumed_qk)], CONSTRUCTION_STAGE_FLOATS)?; }
        for (position, row) in self.bridge_lanes.iter().enumerate() { out.insert_float( "en1991_bridge_lane", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.width), Cell::Real(row.offset), Cell::Integer(i64::from(row.lm1_lane))], BRIDGE_LANE_FLOATS)?; }
        for (position, row) in self.placements.iter().enumerate() { out.insert_float( "en1991_placement", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Real(row.x), Cell::Real(row.y), Cell::Real(row.z), Cell::Real(row.yaw)], PLACEMENT_FLOATS)?; }
        Ok(())
    }
    fn admit_sqlite_values(&self, control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {semio_s_artifact_norm_contract::sqlite_native::schema(En1991Snapshot::SQLITE_SCHEMA,27,192,control)?; let mut out = RowWriter::borrowed(control, phase)?; self.write_sqlite_rows(&mut out)?; out.finish_borrowed() }
}
impl ArtifactSqliteSnapshotHost for En1991Snapshot {
 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{let limits=control.limits();store::decode_sqlite_snapshot_record_native(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, ValueError> = (|| {admission::admit(record,native,limits)?;Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)}
    

    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload, ValueError>{
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let add=|count:usize,size:usize|count.checked_add(size).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Native semantic row count overflow"));let mut rows=1usize;for size in[self.floors.len(),self.self_weight_elements.len(),self.roofs.len(),self.wind_faces.len(),self.accidental_cases.len(),self.bridge_lanes.len(),self.construction_stages.len(),self.roof_surfaces.len(),self.placements.len(),self.special_vehicles.len(),self.wind_bridges.len(),self.dynamic_elements.len(),self.impact_actions.len(),self.internal_explosions.len(),self.thermal_sections.len(),self.cases.len(),self.helicopter_landings.len(),self.tie_runs.len(),self.construction_loads.len(),self.falling_loads.len(),self.execution_imperfections.len(),self.derailment_cases.len(),self.crane_hoists.len(),self.footbridges.len(),self.rail_dynamics.len(),self.load_dispersals.len(),self.bulk_solids.len(),self.rotating_machines.len(),self.fatigue_models.len(),self.support_collisions.len(),self.local_fires.len(),self.snow_guards.len(),self.rail_derailments.len(),self.rail_load_groups.len(),self.dust_explosions.len()]{rows=add(rows,size)?}for case in &self.cases{rows=add(add(rows,case.numbers.len())?,case.texts.len())?}control.check_rows(rows)?;for(index,case)in self.accidental_cases.iter().enumerate(){rows=add(add(rows,case.impact.len())?,case.explosion.len())?;control.check_rows(rows)?;if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,index+1,self.accidental_cases.len())?}}
  self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control,native_owner)
 }

 fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
        let mut bound = NativeEncodingBound::file_only(control)?;
        bound.add(65536)?;
        for text in [&self.snow_zone, &self.thermal_element_type, fire_occupancy_text(self.fire_occupancy), &self.construction_activity, &self.bridge_load_group, &self.crane_class, &self.hoist_class, &self.silo_kind] { bound.repeated(text.len(), 24)?; }
        for floor in &self.floors { bound.add(4096)?; bound.repeated(floor.id.len(), 24)?; bound.repeated(floor.category.len(), 24)?; }
        for element in &self.self_weight_elements { bound.add(2048)?; bound.repeated(element.id.len(), 24)?; bound.repeated(element.material.len(), 24)?; }
        for roof in &self.roofs { bound.add(8192)?; bound.repeated(roof.id.len(), 24)?; bound.repeated(roof.roof_type.len(), 24)?; }
        for face in &self.wind_faces { bound.add(8192)?; bound.repeated(face.id.len(), 24)?; bound.repeated(face.zone.len(), 24)?; }
        for row in &self.bridge_lanes { bound.add(7168)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.construction_stages { bound.add(8192)?; bound.repeated(row.id.len(), 24)?; bound.repeated(row.activity.len(), 24)?; }
        for row in &self.roof_surfaces { bound.add(8192)?; bound.repeated(row.id.len(), 24)?; bound.repeated(row.face.len(), 24)?; bound.repeated(row.shape.len(), 24)?; }
        for row in &self.placements { bound.add(4096)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.special_vehicles { bound.add(7168)?; bound.repeated(row.id.len(), 24)?; bound.repeated(row.designation.len(), 24)?; }
        for row in &self.wind_bridges { bound.add(12288)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.dynamic_elements { bound.add(11264)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.impact_actions { bound.add(12288)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.internal_explosions { bound.add(8192)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.thermal_sections { bound.add(10240)?; bound.repeated(row.id.len(), 24)?; }
        for case in &self.cases { bound.add(4096)?; for value in [&case.id, &case.label_en, &case.label_de, &case.entry_id] { bound.repeated(value.len(), 24)?; } for number in &case.numbers { bound.add(2048)?; bound.repeated(number.id.len(), 24)?; } for text in &case.texts { bound.add(2048)?; for value in [&text.id, &text.value] { bound.repeated(value.len(), 24)?; } } }
        for row in &self.helicopter_landings { bound.add(9216)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.tie_runs { bound.add(13312)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.construction_loads { bound.add(7168)?; bound.repeated(row.id.len(), 24)?; bound.repeated(row.stage.len(), 24)?; }
        for row in &self.falling_loads { bound.add(10240)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.execution_imperfections { bound.add(10240)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.derailment_cases { bound.add(10240)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.crane_hoists { bound.add(7168)?; bound.repeated(row.id.len(), 24)?; bound.repeated(row.release_device.len(), 24)?; }
        for row in &self.footbridges { bound.add(7168)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.rail_dynamics { bound.add(6144)?; bound.repeated(row.id.len(), 24)?; bound.repeated(row.maintenance.len(), 24)?; }
        for row in &self.load_dispersals { bound.add(8192)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.bulk_solids { bound.add(5120)?; bound.repeated(row.id.len(), 24)?; }
        for row in &self.rotating_machines { bound.add(7168)?; bound.repeated(row.id.len(), 24)?; }
        for case in &self.accidental_cases {
            bound.add(2048)?; bound.repeated(case.id.len(), 24)?;
            for _ in &case.impact { bound.add(4096)?; }
            for _ in &case.explosion { bound.add(4096)?; }
        }
        bound.finish()
    }

    

    
}
impl pack::snapshot::ArtifactSqliteSnapshot for En1991Snapshot {
const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> { let mut out = RowWriter::new(Self::SQLITE_SCHEMA, control)?; self.write_sqlite_rows(&mut out)?; out.finish() }
fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let documents = entities(database, "en1991_document", 80, DOCUMENT_FLOATS, control)?;
        if documents.len() != 1 || !documents.contains_key(&1) { return Err(invalid("EN1991 requires exactly document identity one")); }
        let document = documents[&1];
        let floor_rows = entities(database, "en1991_floor", 9, FLOOR_FLOATS, control)?;
        let weight_rows = entities(database, "en1991_self_weight", 7, SELF_WEIGHT_FLOATS, control)?;
        let roof_rows = entities(database, "en1991_roof", 13, ROOF_FLOATS, control)?;
        let wind_rows = entities(database, "en1991_wind_face", 13, WIND_FLOATS, control)?;
        let case_rows = entities(database, "en1991_accidental_case", 4, &[], control)?;
        let rotating_machine_rows = entities(database, "en1991_rotating_machine", 7, ROTATING_MACHINE_FLOATS, control)?;
        let fatigue_model_rows = entities(database, "en1991_fatigue_model", 8, FATIGUE_MODEL_FLOATS, control)?;
        let support_collision_rows = entities(database, "en1991_support_collision", 9, SUPPORT_COLLISION_FLOATS, control)?;
        let local_fire_rows = entities(database, "en1991_local_fire", 9, LOCAL_FIRE, control)?;
        let snow_guard_rows = entities(database, "en1991_snow_guard", 7, SNOW_GUARD, control)?;
        let rail_derailment_rows = entities(database, "en1991_rail_derailment", 8, RAIL_DERAILMENT, control)?;
        let rail_load_group_rows = entities(database, "en1991_rail_load_group", 12, RAIL_LOAD_GROUP, control)?;
        let dust_explosion_rows = entities(database, "en1991_dust_explosion", 10, DUST_EXPLOSION, control)?;
        let bulk_solid_rows = entities(database, "en1991_bulk_solid", 5, BULK_SOLID_FLOATS, control)?;
        let load_dispersal_rows = entities(database, "en1991_load_dispersal", 8, LOAD_DISPERSAL_FLOATS, control)?;
        let rail_dynamic_rows = entities(database, "en1991_rail_dynamic", 6, RAIL_DYNAMIC_FLOATS, control)?;
        let footbridge_rows = entities(database, "en1991_footbridge", 8, FOOTBRIDGE_FLOATS, control)?;
        let crane_hoist_rows = entities(database, "en1991_crane_hoist", 7, CRANE_HOIST_FLOATS, control)?;
        let derailment_case_rows = entities(database, "en1991_derailment_case", 10, DERAILMENT_CASE_FLOATS, control)?;
        let execution_imperfection_rows = entities(database, "en1991_execution_imperfection", 10, EXECUTION_IMPERFECTION_FLOATS, control)?;
        let falling_load_rows = entities(database, "en1991_falling_load", 10, FALLING_LOAD_FLOATS, control)?;
        let construction_load_rows = entities(database, "en1991_construction_load", 7, CONSTRUCTION_LOAD_FLOATS, control)?;
        let tie_run_rows = entities(database, "en1991_tie_run", 13, TIE_RUN_FLOATS, control)?;
        let helicopter_landing_rows = entities(database, "en1991_helicopter_landing", 9, HELICOPTER_LANDING_FLOATS, control)?;
        let thermal_section_rows = entities(database, "en1991_thermal_section", 10, THERMAL_SECTION_FLOATS, control)?;
        let design_rows = entities(database, "en1991_case", 7, &[], control)?;
        let mut design_numbers = child_groups(entities(database, "en1991_case_number", 5, CASE_NUMBER_FLOATS, control)?, &design_rows, control)?;
        let mut design_texts = child_groups(entities(database, "en1991_case_text", 5, &[], control)?, &design_rows, control)?;
        let internal_explosion_rows = entities(database, "en1991_internal_explosion", 8, INTERNAL_EXPLOSION_FLOATS, control)?;
        let impact_action_rows = entities(database, "en1991_impact_action", 12, IMPACT_ACTION_FLOATS, control)?;
        let dynamic_element_rows = entities(database, "en1991_dynamic_element", 11, DYNAMIC_ELEMENT_FLOATS, control)?;
        let wind_bridge_rows = entities(database, "en1991_wind_bridge", 12, WIND_BRIDGE_FLOATS, control)?;
        let special_vehicle_rows = entities(database, "en1991_special_vehicle", 7, SPECIAL_VEHICLE_FLOATS, control)?;
        let roof_surface_rows = entities(database, "en1991_roof_surface", 8, ROOF_SURFACE_FLOATS, control)?;
        let construction_stage_rows = entities(database, "en1991_construction_stage", 8, CONSTRUCTION_STAGE_FLOATS, control)?;
        let placement_rows = entities(database, "en1991_placement", 8, PLACEMENT_FLOATS, control)?;
        let bridge_lane_rows = entities(database, "en1991_bridge_lane", 7, BRIDGE_LANE_FLOATS, control)?;
        let mut impacts = child_groups(entities(database, "en1991_impact", 6, ACCIDENTAL_FLOATS, control)?, &case_rows, control)?;
        let mut explosions = child_groups(entities(database, "en1991_explosion", 6, ACCIDENTAL_FLOATS, control)?, &case_rows, control)?;
        let mut floors = Vec::new();
        for row in ordered(floor_rows.into_values(), 1, control)? { checkpoint(control, floors.len(), 0)?; floors.push(FloorArea { id: reconstruct_text(control, row.text(3)?)?, category: reconstruct_text(control, row.text(4)?)?, area: row.real(5)?, assumed_qk: row.real(6)?, assumed_qk_concentrated: row.real(7)?, assumed_partitions: row.real(8)? }); }
        let mut self_weight_elements = Vec::new();
        for row in ordered(weight_rows.into_values(), 1, control)? { checkpoint(control, self_weight_elements.len(), 0)?; self_weight_elements.push(SelfWeightElement { id: reconstruct_text(control, row.text(3)?)?, material: reconstruct_text(control, row.text(4)?)?, thickness: row.real(5)?, assumed_gk: row.real(6)? }); }
        let mut roofs = Vec::new();
        for row in ordered(roof_rows.into_values(), 1, control)? { checkpoint(control, roofs.len(), 0)?; roofs.push(RoofArea { id: reconstruct_text(control, row.text(3)?)?, roof_type: reconstruct_text(control, row.text(4)?)?, pitch_deg: row.real(5)?, c_e: row.real(6)?, c_t: row.real(7)?, has_parapet: read_flag(row, 8)?, parapet_height: row.real(9)?, drift_obstruction_height: row.real(10)?, multi_span: read_flag(row, 11)?, assumed_sk: row.real(12)? }); }
        let mut wind_faces = Vec::new();
        for row in ordered(wind_rows.into_values(), 1, control)? { checkpoint(control, wind_faces.len(), 0)?; wind_faces.push(WindFace { id: reconstruct_text(control, row.text(3)?)?, zone: reconstruct_text(control, row.text(4)?)?, z: row.real(5)?, c_pe10: row.real(6)?, c_pe1: row.real(7)?, c_pi: row.real(8)?, c_s: row.real(9)?, c_d: row.real(10)?, loaded_area: row.real(11)?, assumed_wp: row.real(12)? }); }
        let mut accidental_cases = Vec::new();
        for row in ordered(case_rows.values().copied(), 1, control)? {
            checkpoint(control, accidental_cases.len(), 0)?;
            let mut impact = Vec::new();
            for child in ordered(impacts.remove(&row.rowid).unwrap_or_default().into_iter(), row.rowid, control)? { checkpoint(control, impact.len(), 0)?; impact.push(AccidentalImpact { vehicle_mass: child.real(3)?, vehicle_speed: child.real(4)?, assumed_force: child.real(5)? }); }
            let mut explosion = Vec::new();
            for child in ordered(explosions.remove(&row.rowid).unwrap_or_default().into_iter(), row.rowid, control)? { checkpoint(control, explosion.len(), 0)?; explosion.push(AccidentalExplosion { explosion_mass: child.real(3)?, standoff: child.real(4)?, assumed_pressure: child.real(5)? }); }
            accidental_cases.push(AccidentalCase { id: reconstruct_text(control, row.text(3)?)?, impact, explosion });
        }
        let mut bridge_lanes = Vec::new();
        for row in ordered(bridge_lane_rows.into_values(), 1, control)? { checkpoint(control, bridge_lanes.len(), 0)?; bridge_lanes.push(BridgeLane { id: reconstruct_text(control, row.text(3)?)?, width: row.real(4)?, offset: row.real(5)?, lm1_lane: read_u8(row, 6)? }); }
        let mut construction_stages = Vec::new();
        for row in ordered(construction_stage_rows.into_values(), 1, control)? { checkpoint(control, construction_stages.len(), 0)?; construction_stages.push(ConstructionStage { id: reconstruct_text(control, row.text(3)?)?, activity: reconstruct_text(control, row.text(4)?)?, duration_days: row.real(5)?, return_years: row.real(6)?, assumed_qk: row.real(7)? }); }
        let mut roof_surfaces = Vec::new();
        for row in ordered(roof_surface_rows.into_values(), 1, control)? { checkpoint(control, roof_surfaces.len(), 0)?; roof_surfaces.push(RoofSurface { id: reconstruct_text(control, row.text(3)?)?, face: reconstruct_text(control, row.text(4)?)?, shape: reconstruct_text(control, row.text(5)?)?, pitch_deg: row.real(6)?, extent: row.real(7)? }); }
        let mut placements = Vec::new();
        for row in ordered(placement_rows.into_values(), 1, control)? { checkpoint(control, placements.len(), 0)?; placements.push(Placement { id: reconstruct_text(control, row.text(3)?)?, x: row.real(4)?, y: row.real(5)?, z: row.real(6)?, yaw: row.real(7)? }); }
        let mut special_vehicles = Vec::new();
        for row in ordered(special_vehicle_rows.into_values(), 1, control)? { checkpoint(control, special_vehicles.len(), 0)?; special_vehicles.push(SpecialVehicle { id: reconstruct_text(control, row.text(3)?)?, designation: reconstruct_text(control, row.text(4)?)?, x: row.real(5)?, y: row.real(6)? }); }
        let mut wind_bridges = Vec::new();
        for row in ordered(wind_bridge_rows.into_values(), 1, control)? { checkpoint(control, wind_bridges.len(), 0)?; wind_bridges.push(WindBridge { id: reconstruct_text(control, row.text(3)?)?, deck_width: row.real(4)?, deck_depth: row.real(5)?, parapet_kind: read_u8(row, 6)?, parapet_height: row.real(7)?, traffic_kind: read_u8(row, 8)?, height_above_ground: row.real(9)?, span: row.real(10)?, assumed_force: row.real(11)? }); }
        let mut dynamic_elements = Vec::new();
        for row in ordered(dynamic_element_rows.into_values(), 1, control)? { checkpoint(control, dynamic_elements.len(), 0)?; dynamic_elements.push(DynamicElement { id: reconstruct_text(control, row.text(3)?)?, height: row.real(4)?, width: row.real(5)?, first_frequency: row.real(6)?, log_decrement: row.real(7)?, mass_per_length: row.real(8)?, strouhal: row.real(9)?, assumed_cscd: row.real(10)? }); }
        let mut impact_actions = Vec::new();
        for row in ordered(impact_action_rows.into_values(), 1, control)? { checkpoint(control, impact_actions.len(), 0)?; impact_actions.push(ImpactAction { id: reconstruct_text(control, row.text(3)?)?, kind: read_u8(row, 4)?, mass: row.real(5)?, speed: row.real(6)?, stiffness: row.real(7)?, height: row.real(8)?, x: row.real(9)?, y: row.real(10)?, assumed_force: row.real(11)? }); }
        let mut internal_explosions = Vec::new();
        for row in ordered(internal_explosion_rows.into_values(), 1, control)? { checkpoint(control, internal_explosions.len(), 0)?; internal_explosions.push(InternalExplosion { id: reconstruct_text(control, row.text(3)?)?, volume: row.real(4)?, vent_area: row.real(5)?, vent_pressure: row.real(6)?, assumed_pressure: row.real(7)? }); }
        let mut thermal_sections = Vec::new();
        for row in ordered(thermal_section_rows.into_values(), 1, control)? { checkpoint(control, thermal_sections.len(), 0)?; thermal_sections.push(ThermalSection { id: reconstruct_text(control, row.text(3)?)?, deck_type: read_u8(row, 4)?, surfacing: read_u8(row, 5)?, length: row.real(6)?, depth: row.real(7)?, alpha_micro: row.real(8)?, assumed_range: row.real(9)? }); }
        let mut cases = Vec::new();
        for row in ordered(design_rows.into_values(), 1, control)? {
            checkpoint(control, cases.len(), 0)?;
            let mut numbers = Vec::new();
            for number in ordered(design_numbers.remove(&row.rowid).unwrap_or_default().into_iter(), row.rowid, control)? { numbers.push(CaseNumber { id: reconstruct_text(control, number.text(3)?)?, value: number.real(4)? }); }
            let mut texts = Vec::new();
            for text in ordered(design_texts.remove(&row.rowid).unwrap_or_default().into_iter(), row.rowid, control)? { texts.push(CaseText { id: reconstruct_text(control, text.text(3)?)?, value: reconstruct_text(control, text.text(4)?)? }); }
            cases.push(CaseRow { id: reconstruct_text(control, row.text(3)?)?, label_en: reconstruct_text(control, row.text(4)?)?, label_de: reconstruct_text(control, row.text(5)?)?, entry_id: reconstruct_text(control, row.text(6)?)?, numbers, texts });
        }
        let mut helicopter_landings = Vec::new();
        for row in ordered(helicopter_landing_rows.into_values(), 1, control)? { checkpoint(control, helicopter_landings.len(), 0)?; helicopter_landings.push(HelicopterLanding { id: reconstruct_text(control, row.text(3)?)?, mass: row.real(4)?, x: row.real(5)?, y: row.real(6)?, height: row.real(7)?, assumed_force: row.real(8)? }); }
        let mut tie_runs = Vec::new();
        for row in ordered(tie_run_rows.into_values(), 1, control)? { checkpoint(control, tie_runs.len(), 0)?; tie_runs.push(TieRun { id: reconstruct_text(control, row.text(3)?)?, kind: read_u8(row, 4)?, spacing: row.real(5)?, span: row.real(6)?, dead_load: row.real(7)?, imposed_load: row.real(8)?, psi: row.real(9)?, x: row.real(10)?, y: row.real(11)?, provided_force: row.real(12)? }); }
        let mut construction_loads = Vec::new();
        for row in ordered(construction_load_rows.into_values(), 1, control)? { checkpoint(control, construction_loads.len(), 0)?; construction_loads.push(ConstructionLoad { id: reconstruct_text(control, row.text(3)?)?, stage: reconstruct_text(control, row.text(4)?)?, kind: read_u8(row, 5)?, magnitude: row.real(6)? }); }
        let mut falling_loads = Vec::new();
        for row in ordered(falling_load_rows.into_values(), 1, control)? { checkpoint(control, falling_loads.len(), 0)?; falling_loads.push(FallingLoad { id: reconstruct_text(control, row.text(3)?)?, mass: row.real(4)?, drop_height: row.real(5)?, stopping_distance: row.real(6)?, x: row.real(7)?, y: row.real(8)?, assumed_force: row.real(9)? }); }
        let mut execution_imperfections = Vec::new();
        for row in ordered(execution_imperfection_rows.into_values(), 1, control)? { checkpoint(control, execution_imperfections.len(), 0)?; execution_imperfections.push(ExecutionImperfection { id: reconstruct_text(control, row.text(3)?)?, height: row.real(4)?, tolerance: row.real(5)?, vertical_load: row.real(6)?, x: row.real(7)?, y: row.real(8)?, assumed_force: row.real(9)? }); }
        let mut derailment_cases = Vec::new();
        for row in ordered(derailment_case_rows.into_values(), 1, control)? { checkpoint(control, derailment_cases.len(), 0)?; derailment_cases.push(DerailmentCase { id: reconstruct_text(control, row.text(3)?)?, distance: row.real(4)?, design_force_x: row.real(5)?, design_force_y: row.real(6)?, x: row.real(7)?, y: row.real(8)?, assumed_force: row.real(9)? }); }
        let mut crane_hoists = Vec::new();
        for row in ordered(crane_hoist_rows.into_values(), 1, control)? { checkpoint(control, crane_hoists.len(), 0)?; crane_hoists.push(CraneHoist { id: reconstruct_text(control, row.text(3)?)?, rated_load: row.real(4)?, release_fraction: row.real(5)?, release_device: reconstruct_text(control, row.text(6)?)? }); }
        let mut footbridges = Vec::new();
        for row in ordered(footbridge_rows.into_values(), 1, control)? { checkpoint(control, footbridges.len(), 0)?; footbridges.push(Footbridge { id: reconstruct_text(control, row.text(3)?)?, loaded_length: row.real(4)?, deck_width: row.real(5)?, natural_frequency: row.real(6)?,assumed_service_axle: row.real(7)? }); }
        let mut rail_dynamics = Vec::new();
        for row in ordered(rail_dynamic_rows.into_values(), 1, control)? { checkpoint(control, rail_dynamics.len(), 0)?; rail_dynamics.push(RailDynamic { id: reconstruct_text(control, row.text(3)?)?, determinant_length: row.real(4)?, maintenance: reconstruct_text(control, row.text(5)?)? }); }
        let mut load_dispersals = Vec::new();
        for row in ordered(load_dispersal_rows.into_values(), 1, control)? { checkpoint(control, load_dispersals.len(), 0)?; load_dispersals.push(LoadDispersal { id: reconstruct_text(control, row.text(3)?)?, wheel_length: row.real(4)?, wheel_width: row.real(5)?, pavement_thickness: row.real(6)?, slab_thickness: row.real(7)? }); }
        let mut bulk_solids = Vec::new();
        for row in ordered(bulk_solid_rows.into_values(), 1, control)? { checkpoint(control, bulk_solids.len(), 0)?; bulk_solids.push(BulkSolid { id: reconstruct_text(control, row.text(3)?)?, internal_friction: row.real(4)? }); }
        let mut rotating_machines = Vec::new();
        let mut fatigue_models = Vec::new();
        let mut support_collisions = Vec::new();
        let mut local_fires = Vec::new();
        let mut snow_guards = Vec::new();
        let mut rail_derailments = Vec::new();
        let mut rail_load_groups = Vec::new();
        let mut dust_explosions = Vec::new();
        for row in ordered(rotating_machine_rows.into_values(), 1, control)? { checkpoint(control, rotating_machines.len(), 0)?; rotating_machines.push(RotatingMachine { id: reconstruct_text(control, row.text(3)?)?, mass: row.real(4)?, eccentricity: row.real(5)?, frequency: row.real(6)? }); }
        for row in ordered(fatigue_model_rows.into_values(), 1, control)? { checkpoint(control, fatigue_models.len(), 0)?; fatigue_models.push(FatigueModel { id: reconstruct_text(control, row.text(3)?)?, model: reconstruct_text(control, row.text(4)?)?, lane: read_u8(row, 5)?, assumed_axle_load: row.real(6)?, assumed_udl: row.real(7)? }); }
        for row in ordered(support_collision_rows.into_values(), 1, control)? { checkpoint(control, support_collisions.len(), 0)?; support_collisions.push(SupportCollision { id: reconstruct_text(control, row.text(3)?)?, location: reconstruct_text(control, row.text(4)?)?, assumed_force_x: row.real(5)?, assumed_force_y: row.real(6)?, height: row.real(7)?, position_x: row.real(8)? }); }
        for row in ordered(local_fire_rows.into_values(), 1, control)? { checkpoint(control, local_fires.len(), 0)?; local_fires.push(LocalFire { id: reconstruct_text(control, row.text(3)?)?, fire_diameter: row.real(4)?, heat_release_rate: row.real(5)?, ceiling_height: row.real(6)?, horizontal_distance: row.real(7)?, assumed_flux: row.real(8)? }); }
        for row in ordered(snow_guard_rows.into_values(), 1, control)? { checkpoint(control, snow_guards.len(), 0)?; snow_guards.push(SnowGuard { id: reconstruct_text(control, row.text(3)?)?, roof_id: reconstruct_text(control, row.text(4)?)?, slope_length: row.real(5)?, assumed_force: row.real(6)? }); }
        for row in ordered(rail_derailment_rows.into_values(), 1, control)? { checkpoint(control, rail_derailments.len(), 0)?; rail_derailments.push(RailDerailment { id: reconstruct_text(control, row.text(3)?)?, situation: reconstruct_text(control, row.text(4)?)?, assumed_line_load: row.real(5)?, loaded_length: row.real(6)?, offset: row.real(7)? }); }
        for row in ordered(rail_load_group_rows.into_values(), 1, control)? { checkpoint(control, rail_load_groups.len(), 0)?; rail_load_groups.push(RailLoadGroup { id: reconstruct_text(control, row.text(3)?)?, load_group: reconstruct_text(control, row.text(4)?)?, factor_vertical: row.real(5)?, factor_longitudinal: row.real(6)?, factor_lateral: row.real(7)?, assumed_axle_load: row.real(8)?, assumed_udl: row.real(9)?, assumed_longitudinal: row.real(10)?, assumed_lateral: row.real(11)? }); }
        for row in ordered(dust_explosion_rows.into_values(), 1, control)? { checkpoint(control, dust_explosions.len(), 0)?; dust_explosions.push(DustExplosion { id: reconstruct_text(control, row.text(3)?)?, kst: row.real(4)?, p_max: row.real(5)?, volume: row.real(6)?, vent_area: row.real(7)?, static_opening_pressure: row.real(8)?, design_pressure: row.real(9)? }); }
        let result = Self {
            annex: match document.text(1)? { "En" => AnnexChoice::En, "De" => AnnexChoice::De, _ => return Err(invalid("EN1991 annex choice is not owned")) }, snow_zone: reconstruct_text(control, document.text(2)?)?, altitude: document.real(3)?, en_sk: document.real(4)?, north_german_lowland_snow: read_flag(document, 5)?, wind_zone: read_u8(document, 6)?, en_vb: document.real(7)?, terrain_category: read_u8(document, 8)?, mixed_terrain_upwind: read_u8(document, 9)?, mixed_terrain_distance: document.real(10)?, orography_factor: document.real(11)?, coast_or_island: read_flag(document, 12)?, air_density: document.real(13)?, height: document.real(14)?, width: document.real(15)?, depth: document.real(16)?, assumed_delta_t: document.real(17)?, t_max: document.real(18)?, t_min: document.real(19)?, t_0: document.real(20)?, thermal_element_type: reconstruct_text(control, document.text(21)?)?, thermal_bridge_type: read_u8(document, 22)?, delta_t_m: document.real(23)?, storey_count: read_u8(document, 24)?,
            fire_mode: match document.text(25)? { "none" => FireMode::None, "nominal" => FireMode::Nominal, "parametric" => FireMode::Parametric, _ => return Err(invalid("EN1991 fire mode is not owned")) }, fire_curve: match document.text(26)? { "standard" => FireCurve::Standard, "external" => FireCurve::External, "hydrocarbon" => FireCurve::Hydrocarbon, "parametric" => FireCurve::Parametric, _ => return Err(invalid("EN1991 fire curve is not owned")) },
            fire_duration: document.real(27)?, assumed_gas_temperature: document.real(28)?, assumed_h_net: document.real(29)?, fire_compartment_area: document.real(30)?, fire_compartment_height: document.real(31)?, fire_opening_factor: document.real(32)?, fire_thermal_inertia: document.real(33)?, fire_occupancy: read_fire_occupancy(document.text(34)?)?, fire_load_density_qf: document.real(35)?, assumed_qf_d: document.real(36)?, construction_activity: reconstruct_text(control, document.text(37)?)?, assumed_construction_qk: document.real(38)?, structure_kind: match document.text(39)? { "building" => StructureKind::Building, "bridge" => StructureKind::Bridge, _ => return Err(invalid("EN1991 structure kind is not owned")) }, bridge_lane: read_u8(document, 40)?, bridge_span: document.real(41)?, bridge_lane_width: document.real(42)?, assumed_bridge_tandem: document.real(43)?, assumed_bridge_udl: document.real(44)?, assumed_bridge_lm2: document.real(45)?, assumed_bridge_footway: document.real(46)?, assumed_bridge_lm3: document.real(47)?, assumed_bridge_lm4: document.real(48)?, bridge_load_group: reconstruct_text(control, document.text(49)?)?, crane_claimed: read_flag(document, 50)?, crane_class: reconstruct_text(control, document.text(51)?)?, hoist_class: reconstruct_text(control, document.text(52)?)?, hoisting_speed: document.real(53)?, assumed_crane_wheel: document.real(54)?, assumed_crane_horizontal: document.real(55)?, silo_claimed: read_flag(document, 56)?, silo_kind: reconstruct_text(control, document.text(57)?)?, silo_bulk_density: document.real(58)?, silo_height: document.real(59)?, silo_hydraulic_radius: document.real(60)?, silo_mu: document.real(61)?, silo_k: document.real(62)?, assumed_silo_pressure: document.real(63)?, assumed_silo_patch: document.real(64)?, assumed_silo_wall_friction: document.real(65)?, bridge_tandem_x: document.real(66)?, rail_alpha: document.real(67)?, assumed_rail_lm71: document.real(68)?, assumed_rail_lm71_udl: document.real(69)?, assumed_rail_sw0: document.real(70)?, assumed_rail_sw2: document.real(71)?, crane_x: document.real(72)?, crane_runway_length: document.real(73)?, crane_wheel_base: document.real(74)?, silo_diameter: document.real(75)?, silo_eccentricity: document.real(76)?, silo_hopper_angle_deg: document.real(77)?, fire_load_basis: match document.text(78)? { "occupancyTable" => FireLoadBasis::OccupancyTable, "userSpecified" => FireLoadBasis::UserSpecified, _ => return Err(invalid("EN1991 fire load basis is not owned")) }, bridge_curve_radius: document.real(79)?, floors, self_weight_elements, roofs, wind_faces, accidental_cases,
            bridge_lanes,
            construction_stages,
            roof_surfaces,
            placements,
            special_vehicles,
            wind_bridges,
            dynamic_elements,
            impact_actions,
            internal_explosions,
            thermal_sections,
            cases,
            helicopter_landings,
            tie_runs,
            construction_loads,
            falling_loads,
            execution_imperfections,
            derailment_cases,
            crane_hoists,
            footbridges,
            rail_dynamics,
            load_dispersals,
            bulk_solids,
            rotating_machines,
            fatigue_models,
            support_collisions,
            local_fires,
            snow_guards,
            rail_derailments,
            rail_load_groups,
            dust_explosions,
        };
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(result)
    }
}

/// 🪶️ Exposes only the explicitly authored EN1991 relational provider.
pub fn sqlite_codec() -> store::os_store::serialization::ArtifactSqliteSnapshotCodec { <En1991Snapshot as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec() }

#[path="🛂️admission/🦀️.rs"]
pub mod admission;

