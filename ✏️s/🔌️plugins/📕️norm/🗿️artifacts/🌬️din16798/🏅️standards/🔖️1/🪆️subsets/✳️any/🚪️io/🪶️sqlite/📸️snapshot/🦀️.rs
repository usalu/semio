//! 🌬️ Handwritten complete building, comfort-zone and ventilation-system snapshot entities.
use crate::standards::v1::subsets::any::schema::snapshot::Din16798Snapshot;
use crate::{ZoneDocument,VentSystemDocument,document::AnnexChoice};
use std::collections::BTreeMap;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SnapshotEncoding,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,validate_sqlite_database_schema,artifact::{Cell,FloatColumn,FloatRow,NativeEncodingBound,RowWriter,reconstruct_text}}};
use semio_framework_value::{ValueError, ValueRefusalKind};
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }


const DOCUMENT_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];
const ZONE_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(6),FloatColumn::Binary64(11),FloatColumn::Binary64(12),FloatColumn::Binary64(13),FloatColumn::Binary64(14),FloatColumn::Binary64(15),FloatColumn::Binary64(16),FloatColumn::Binary64(17),FloatColumn::Binary64(18),FloatColumn::Binary64(19),FloatColumn::Binary64(20),FloatColumn::Binary64(21)];
const SYSTEM_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(6),FloatColumn::Binary64(8),FloatColumn::Binary64(12),FloatColumn::Binary64(13),FloatColumn::Binary64(14),FloatColumn::Binary64(15),FloatColumn::Binary64(17),FloatColumn::Binary64(18),FloatColumn::Binary64(19)];
type Entities<'a>=BTreeMap<i64,FloatRow<'a>>;

fn annex(value:AnnexChoice)-> &'static str{match value{AnnexChoice::En=>"En",AnnexChoice::De=>"De"}}
fn read_annex(value:&str)->Result<AnnexChoice, ValueError>{match value{"En"=>Ok(AnnexChoice::En),"De"=>Ok(AnnexChoice::De),_=>Err(invalid("DIN16798 unsupported annex"))}}
fn ordinal(value:usize)->Result<Cell<'static>, ValueError>{Ok(Cell::Integer(i64::try_from(value).map_err(|error| invalid(error.to_string()))?))}
fn checkpoint(control:&mut SqliteSnapshotControl<'_>,position:usize,total:usize)->Result<(), ValueError>{if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,total)?;}Ok(())}
fn entities<'a>(database:&'a SqliteDatabase,table:&str,columns:usize,floats:&'static[FloatColumn],control:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>, ValueError>{let rows=&database.table(table)?.rows;let mut result=Entities::new();for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;let row=FloatRow::new(row,floats)?;if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid||result.insert(row.rowid,row).is_some(){return Err(invalid(format!("{table} requires exact fields and unique positive aliased identities")));}}Ok(result)}
fn ordered<'a>(rows:impl Iterator<Item=FloatRow<'a>>,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<FloatRow<'a>>, ValueError>{let mut result=BTreeMap::new();for(position,row)in rows.enumerate(){checkpoint(control,position,0)?;let ordinal=usize::try_from(row.integer(2)?).map_err(|error| invalid(error.to_string()))?;if row.integer(1)?!=1||result.insert(ordinal,row).is_some(){return Err(invalid("DIN16798 entity has a foreign document or duplicate ordinal"));}}for(position,ordinal)in result.keys().copied().enumerate(){checkpoint(control,position,result.len())?;if position!=ordinal{return Err(invalid("DIN16798 ordinals must be dense"));}}Ok(result.into_values().collect())}
fn read_u32(row:FloatRow<'_>,column:usize)->Result<u32, ValueError>{u32::try_from(row.integer(column)?).map_err(|error| invalid(error.to_string()))}
fn read_u8(row:FloatRow<'_>,column:usize)->Result<u8, ValueError>{u8::try_from(row.integer(column)?).map_err(|error| invalid(error.to_string()))}


impl Din16798Snapshot {
    fn write_sqlite_rows(&self, out: &mut RowWriter<'_,'_>) -> Result<(), ValueError> {
        out.insert_key_float("din16798_document",1,&[Cell::Text(annex(self.annex)),Cell::Real(self.theta_rm_c),Cell::Real(self.outdoor_co2_ppm),Cell::Real(self.envelope_n50_h_inv),Cell::Real(self.envelope_volume_m3),Cell::Real(self.cellar_area_m2),Cell::Real(self.cellar_ventilation_m3_h),Cell::Real(self.night_setback_k)],DOCUMENT_FLOATS)?;
        for(position,z)in self.zones.iter().enumerate(){out.insert_float("din16798_zone",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&z.id),Cell::Text(&z.name),Cell::Text(&z.usage_type),Cell::Real(z.floor_area_m2),Cell::Integer(i64::from(z.occupants)),Cell::Text(&z.comfort_category),Cell::Text(&z.pollution_class),Cell::Text(&z.comfort_model),Cell::Real(z.t_op_winter_c),Cell::Real(z.t_op_summer_c),Cell::Real(z.air_speed_m_s),Cell::Real(z.clothing_clo),Cell::Real(z.metabolic_rate_met),Cell::Real(z.rh_percent),Cell::Real(z.outdoor_air_supplied_m3_h),Cell::Real(z.co2_ppm),Cell::Real(z.illuminance_lx),Cell::Real(z.noise_db),Cell::Real(z.turbulence_intensity_percent),Cell::Text(&z.vent_method),Cell::Text(&z.vent_system_id)],ZONE_FLOATS)?;}
        for(position,v)in self.vent_systems.iter().enumerate(){out.insert_float("din16798_vent_system",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&v.id),Cell::Text(&v.name),Cell::Text(&v.system_type),Cell::Real(v.sfp_w_m3_s),Cell::Integer(i64::from(v.sfp_required_class)),Cell::Real(v.heat_recovery_eta),Cell::Text(&v.oda_class),Cell::Text(&v.filter_sup_class),Cell::Integer(i64::from(v.years_since_inspection)),Cell::Real(v.humidification_required_kg_h),Cell::Real(v.humidification_provided_kg_h),Cell::Real(v.fan_q_v_m3_s),Cell::Real(v.fan_t_run_h),Cell::Text(&v.duct_class),Cell::Real(v.duct_test_pressure_pa),Cell::Real(v.duct_leakage_m3_s_m2),Cell::Real(v.design_airflow_m3_h)],SYSTEM_FLOATS)?;}
        Ok(())
    }
    fn admit_sqlite_values(&self, control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {semio_s_artifact_norm_contract::sqlite_native::schema(Din16798Snapshot::SQLITE_SCHEMA,3,48,control)?; let mut out = RowWriter::borrowed(control, phase)?; self.write_sqlite_rows(&mut out)?; out.finish_borrowed() }
}
impl ArtifactSqliteSnapshot for Din16798Snapshot{
 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{let limits=control.limits();store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {admission::admit(record,native,limits)?;Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)}
    const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload, ValueError>{
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let add=|count:usize,size:usize|count.checked_add(size).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Native semantic row count overflow"));let rows=add(add(1,self.zones.len())?,self.vent_systems.len())?;control.check_rows(rows)?;
  self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control,native_owner)
 }

 fn preflight_sqlite_snapshot_encoding(&self,_:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(), ValueError>{self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
        let mut bound=NativeEncodingBound::file_only(control)?;bound.add(16384)?;
        for zone in &self.zones{bound.add(16384)?;for text in [&zone.id,&zone.name,&zone.usage_type,&zone.comfort_category,&zone.pollution_class,&zone.comfort_model,&zone.vent_method,&zone.vent_system_id]{bound.repeated(text.len(),24)?;}}
        for system in &self.vent_systems{bound.add(16384)?;for text in [&system.id,&system.name,&system.system_type,&system.oda_class,&system.filter_sup_class,&system.duct_class]{bound.repeated(text.len(),24)?;}}
        bound.finish()
    }
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> { let mut out = RowWriter::new(Self::SQLITE_SCHEMA, control)?; self.write_sqlite_rows(&mut out)?; out.finish() }

    fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self, ValueError>{
        validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
        let documents=entities(database,"din16798_document",9,DOCUMENT_FLOATS,control)?;if documents.len()!=1||!documents.contains_key(&1){return Err(invalid("DIN16798 requires exactly document identity one"));}let d=documents[&1];
        let mut zones=Vec::new();for z in ordered(entities(database,"din16798_zone",24,ZONE_FLOATS,control)?.into_values(),control)?{checkpoint(control,zones.len(),0)?;zones.push(ZoneDocument{id:reconstruct_text(control,z.text(3)?)?,name:reconstruct_text(control,z.text(4)?)?,usage_type:reconstruct_text(control,z.text(5)?)?,floor_area_m2:z.real(6)?,occupants:read_u32(z,7)?,comfort_category:reconstruct_text(control,z.text(8)?)?,pollution_class:reconstruct_text(control,z.text(9)?)?,comfort_model:reconstruct_text(control,z.text(10)?)?,t_op_winter_c:z.real(11)?,t_op_summer_c:z.real(12)?,air_speed_m_s:z.real(13)?,clothing_clo:z.real(14)?,metabolic_rate_met:z.real(15)?,rh_percent:z.real(16)?,outdoor_air_supplied_m3_h:z.real(17)?,co2_ppm:z.real(18)?,illuminance_lx:z.real(19)?,noise_db:z.real(20)?,turbulence_intensity_percent:z.real(21)?,vent_method:reconstruct_text(control,z.text(22)?)?,vent_system_id:reconstruct_text(control,z.text(23)?)?});}
        let mut vent_systems=Vec::new();for v in ordered(entities(database,"din16798_vent_system",20,SYSTEM_FLOATS,control)?.into_values(),control)?{checkpoint(control,vent_systems.len(),0)?;vent_systems.push(VentSystemDocument{id:reconstruct_text(control,v.text(3)?)?,name:reconstruct_text(control,v.text(4)?)?,system_type:reconstruct_text(control,v.text(5)?)?,sfp_w_m3_s:v.real(6)?,sfp_required_class:read_u8(v,7)?,heat_recovery_eta:v.real(8)?,oda_class:reconstruct_text(control,v.text(9)?)?,filter_sup_class:reconstruct_text(control,v.text(10)?)?,years_since_inspection:read_u32(v,11)?,humidification_required_kg_h:v.real(12)?,humidification_provided_kg_h:v.real(13)?,fan_q_v_m3_s:v.real(14)?,fan_t_run_h:v.real(15)?,duct_class:reconstruct_text(control,v.text(16)?)?,duct_test_pressure_pa:v.real(17)?,duct_leakage_m3_s_m2:v.real(18)?,design_airflow_m3_h:v.real(19)?});}
        let snapshot=Self{annex:read_annex(d.text(1)?)?,theta_rm_c:d.real(2)?,outdoor_co2_ppm:d.real(3)?,zones,vent_systems,envelope_n50_h_inv:d.real(4)?,envelope_volume_m3:d.real(5)?,cellar_area_m2:d.real(6)?,cellar_ventilation_m3_h:d.real(7)?,night_setback_k:d.real(8)?};control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(snapshot)
    }
}

pub fn sqlite_codec()->store::ArtifactSqliteSnapshotCodec{<Din16798Snapshot as store::ArtifactSqliteSnapshot>::sqlite_codec()}

#[path="🛂️admission/🦀️.rs"]
pub(in crate::standards::v1::subsets::any)mod admission;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
