//! 🏋️ Handwritten EN1991 site, load assumptions and nested accidental-case identities.
use super::En1991Snapshot;
use crate::{AccidentalCase, AccidentalExplosion, AccidentalImpact, FireMode, FloorArea, RoofArea, SelfWeightElement, StructureKind, WindFace, document::AnnexChoice, part_1_2::FireCurve};
use std::collections::BTreeMap;
use store::{ArtifactSqliteSnapshot, sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, validate_sqlite_database_schema, artifact::{Cell, FloatColumn, FloatRow, NativeEncodingBound, RowWriter, reconstruct_text}}};
use semio_framework_value::{ValueError, ValueRefusalKind};
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }


const DOCUMENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(3), FloatColumn::Binary64(4), FloatColumn::Binary64(7), FloatColumn::Binary64(10), FloatColumn::Binary64(11), FloatColumn::Binary64(13), FloatColumn::Binary64(14), FloatColumn::Binary64(15), FloatColumn::Binary64(16), FloatColumn::Binary64(17), FloatColumn::Binary64(18), FloatColumn::Binary64(19), FloatColumn::Binary64(20), FloatColumn::Binary64(23), FloatColumn::Binary64(27), FloatColumn::Binary64(28), FloatColumn::Binary64(29), FloatColumn::Binary64(30), FloatColumn::Binary64(31), FloatColumn::Binary64(32), FloatColumn::Binary64(33), FloatColumn::Binary64(35), FloatColumn::Binary64(36), FloatColumn::Binary64(38), FloatColumn::Binary64(41), FloatColumn::Binary64(42), FloatColumn::Binary64(43), FloatColumn::Binary64(44), FloatColumn::Binary64(45), FloatColumn::Binary64(46), FloatColumn::Binary64(47), FloatColumn::Binary64(48), FloatColumn::Binary64(53), FloatColumn::Binary64(54), FloatColumn::Binary64(55), FloatColumn::Binary64(58), FloatColumn::Binary64(59), FloatColumn::Binary64(60), FloatColumn::Binary64(61), FloatColumn::Binary64(62), FloatColumn::Binary64(63), FloatColumn::Binary64(64), FloatColumn::Binary64(65)];
const FLOOR_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8)];
const SELF_WEIGHT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6)];
const ROOF_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(12)];
const WIND_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(11), FloatColumn::Binary64(12)];
const ACCIDENTAL_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(3), FloatColumn::Binary64(4), FloatColumn::Binary64(5)];
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
            Cell::Real(self.fire_duration), Cell::Real(self.assumed_gas_temperature), Cell::Real(self.assumed_h_net), Cell::Real(self.fire_compartment_area), Cell::Real(self.fire_compartment_height), Cell::Real(self.fire_opening_factor), Cell::Real(self.fire_thermal_inertia), Cell::Text(&self.fire_occupancy), Cell::Real(self.fire_load_density_qf), Cell::Real(self.assumed_qf_d), Cell::Text(&self.construction_activity), Cell::Real(self.assumed_construction_qk), Cell::Text(match self.structure_kind { StructureKind::Building => "building", StructureKind::Bridge => "bridge" }), Cell::Integer(i64::from(self.bridge_lane)), Cell::Real(self.bridge_span), Cell::Real(self.bridge_lane_width), Cell::Real(self.assumed_bridge_tandem), Cell::Real(self.assumed_bridge_udl), Cell::Real(self.assumed_bridge_lm2), Cell::Real(self.assumed_bridge_footway), Cell::Real(self.assumed_bridge_lm3), Cell::Real(self.assumed_bridge_lm4), Cell::Text(&self.bridge_load_group), flag(self.crane_claimed), Cell::Text(&self.crane_class), Cell::Text(&self.hoist_class), Cell::Real(self.hoisting_speed), Cell::Real(self.assumed_crane_wheel), Cell::Real(self.assumed_crane_horizontal), flag(self.silo_claimed), Cell::Text(&self.silo_kind), Cell::Real(self.silo_bulk_density), Cell::Real(self.silo_height), Cell::Real(self.silo_hydraulic_radius), Cell::Real(self.silo_mu), Cell::Real(self.silo_k), Cell::Real(self.assumed_silo_pressure), Cell::Real(self.assumed_silo_patch), Cell::Real(self.assumed_silo_wall_friction)
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
        Ok(())
    }
    fn admit_sqlite_values(&self, control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> { let mut out = RowWriter::borrowed(control, phase)?; self.write_sqlite_rows(&mut out)?; out.finish_borrowed() }
}
impl ArtifactSqliteSnapshot for En1991Snapshot {
 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self, ValueError>{let snapshot=store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, native| Self::__dsl_from_record_controlled(record, native),control)?;snapshot.admit_sqlite_values(control,SqliteSnapshotPhase::DecodeNative)?;Ok(snapshot)}
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload, ValueError>{
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let add=|count:usize,size:usize|count.checked_add(size).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Native semantic row count overflow"));let mut rows=1usize;for size in[self.floors.len(),self.self_weight_elements.len(),self.roofs.len(),self.wind_faces.len(),self.accidental_cases.len()]{rows=add(rows,size)?}control.check_rows(rows)?;for(index,case)in self.accidental_cases.iter().enumerate(){rows=add(add(rows,case.impact.len())?,case.explosion.len())?;control.check_rows(rows)?;if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,index+1,self.accidental_cases.len())?}}
  self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }

 fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
        let mut bound = NativeEncodingBound::new(control)?;
        bound.add(65536)?;
        for text in [&self.snow_zone, &self.thermal_element_type, &self.fire_occupancy, &self.construction_activity, &self.bridge_load_group, &self.crane_class, &self.hoist_class, &self.silo_kind] { bound.repeated(text.len(), 24)?; }
        for floor in &self.floors { bound.add(4096)?; bound.repeated(floor.id.len(), 24)?; bound.repeated(floor.category.len(), 24)?; }
        for element in &self.self_weight_elements { bound.add(2048)?; bound.repeated(element.id.len(), 24)?; bound.repeated(element.material.len(), 24)?; }
        for roof in &self.roofs { bound.add(8192)?; bound.repeated(roof.id.len(), 24)?; bound.repeated(roof.roof_type.len(), 24)?; }
        for face in &self.wind_faces { bound.add(8192)?; bound.repeated(face.id.len(), 24)?; bound.repeated(face.zone.len(), 24)?; }
        for case in &self.accidental_cases {
            bound.add(2048)?; bound.repeated(case.id.len(), 24)?;
            for _ in &case.impact { bound.add(4096)?; }
            for _ in &case.explosion { bound.add(4096)?; }
        }
        bound.finish()
    }

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> { let mut out = RowWriter::new(Self::SQLITE_SCHEMA, control)?; self.write_sqlite_rows(&mut out)?; out.finish() }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let documents = entities(database, "en1991_document", 66, DOCUMENT_FLOATS, control)?;
        if documents.len() != 1 || !documents.contains_key(&1) { return Err(invalid("EN1991 requires exactly document identity one")); }
        let document = documents[&1];
        let floor_rows = entities(database, "en1991_floor", 9, FLOOR_FLOATS, control)?;
        let weight_rows = entities(database, "en1991_self_weight", 7, SELF_WEIGHT_FLOATS, control)?;
        let roof_rows = entities(database, "en1991_roof", 13, ROOF_FLOATS, control)?;
        let wind_rows = entities(database, "en1991_wind_face", 13, WIND_FLOATS, control)?;
        let case_rows = entities(database, "en1991_accidental_case", 4, &[], control)?;
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
        let result = Self {
            annex: match document.text(1)? { "En" => AnnexChoice::En, "De" => AnnexChoice::De, _ => return Err(invalid("EN1991 annex choice is not owned")) }, snow_zone: reconstruct_text(control, document.text(2)?)?, altitude: document.real(3)?, en_sk: document.real(4)?, north_german_lowland_snow: read_flag(document, 5)?, wind_zone: read_u8(document, 6)?, en_vb: document.real(7)?, terrain_category: read_u8(document, 8)?, mixed_terrain_upwind: read_u8(document, 9)?, mixed_terrain_distance: document.real(10)?, orography_factor: document.real(11)?, coast_or_island: read_flag(document, 12)?, air_density: document.real(13)?, height: document.real(14)?, width: document.real(15)?, depth: document.real(16)?, assumed_delta_t: document.real(17)?, t_max: document.real(18)?, t_min: document.real(19)?, t_0: document.real(20)?, thermal_element_type: reconstruct_text(control, document.text(21)?)?, thermal_bridge_type: read_u8(document, 22)?, delta_t_m: document.real(23)?, storey_count: read_u8(document, 24)?,
            fire_mode: match document.text(25)? { "none" => FireMode::None, "nominal" => FireMode::Nominal, "parametric" => FireMode::Parametric, _ => return Err(invalid("EN1991 fire mode is not owned")) }, fire_curve: match document.text(26)? { "standard" => FireCurve::Standard, "external" => FireCurve::External, "hydrocarbon" => FireCurve::Hydrocarbon, "parametric" => FireCurve::Parametric, _ => return Err(invalid("EN1991 fire curve is not owned")) },
            fire_duration: document.real(27)?, assumed_gas_temperature: document.real(28)?, assumed_h_net: document.real(29)?, fire_compartment_area: document.real(30)?, fire_compartment_height: document.real(31)?, fire_opening_factor: document.real(32)?, fire_thermal_inertia: document.real(33)?, fire_occupancy: reconstruct_text(control, document.text(34)?)?, fire_load_density_qf: document.real(35)?, assumed_qf_d: document.real(36)?, construction_activity: reconstruct_text(control, document.text(37)?)?, assumed_construction_qk: document.real(38)?, structure_kind: match document.text(39)? { "building" => StructureKind::Building, "bridge" => StructureKind::Bridge, _ => return Err(invalid("EN1991 structure kind is not owned")) }, bridge_lane: read_u8(document, 40)?, bridge_span: document.real(41)?, bridge_lane_width: document.real(42)?, assumed_bridge_tandem: document.real(43)?, assumed_bridge_udl: document.real(44)?, assumed_bridge_lm2: document.real(45)?, assumed_bridge_footway: document.real(46)?, assumed_bridge_lm3: document.real(47)?, assumed_bridge_lm4: document.real(48)?, bridge_load_group: reconstruct_text(control, document.text(49)?)?, crane_claimed: read_flag(document, 50)?, crane_class: reconstruct_text(control, document.text(51)?)?, hoist_class: reconstruct_text(control, document.text(52)?)?, hoisting_speed: document.real(53)?, assumed_crane_wheel: document.real(54)?, assumed_crane_horizontal: document.real(55)?, silo_claimed: read_flag(document, 56)?, silo_kind: reconstruct_text(control, document.text(57)?)?, silo_bulk_density: document.real(58)?, silo_height: document.real(59)?, silo_hydraulic_radius: document.real(60)?, silo_mu: document.real(61)?, silo_k: document.real(62)?, assumed_silo_pressure: document.real(63)?, assumed_silo_patch: document.real(64)?, assumed_silo_wall_friction: document.real(65)?, floors, self_weight_elements, roofs, wind_faces, accidental_cases
        };
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(result)
    }
}

/// 🪶️ Exposes only the explicitly authored EN1991 relational provider.
pub fn sqlite_codec() -> store::ArtifactSqliteSnapshotCodec { <En1991Snapshot as ArtifactSqliteSnapshot>::sqlite_codec() }
