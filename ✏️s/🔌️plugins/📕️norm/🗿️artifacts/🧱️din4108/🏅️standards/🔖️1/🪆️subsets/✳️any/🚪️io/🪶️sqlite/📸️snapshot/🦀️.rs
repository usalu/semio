//! 🧱️ Handwritten complete envelope, window, layer, material-segment and bridge entities.
use crate::standards::v1::subsets::any::schema::snapshot::Din4108Snapshot;
use crate::{EnvelopeElement,LayerDocument,LayerSegment,ThermalBridge,ThermalZone,ZoneWindow,document::ClimateZoneDe};
use std::collections::BTreeMap;
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SnapshotEncoding,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,validate_sqlite_database_schema,artifact::{Cell,FloatColumn,FloatRow,NativeEncodingBound,RowWriter,reconstruct_text}}};
use semio_framework_value::{ValueError, ValueRefusalKind};
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }


const DOCUMENT_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(6)];
const ZONE_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(4)];
const WINDOW_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];
const ELEMENT_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),FloatColumn::Binary64(12)];
const LAYER_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];
const SEGMENT_FLOATS:&[FloatColumn]=LAYER_FLOATS;
const BRIDGE_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
type Entities<'a>=BTreeMap<i64,FloatRow<'a>>;
type Groups<'a>=BTreeMap<i64,Vec<FloatRow<'a>>>;

fn climate(value:ClimateZoneDe)-> &'static str{match value{ClimateZoneDe::Zone1=>"Zone1",ClimateZoneDe::Zone2=>"Zone2",ClimateZoneDe::Zone3=>"Zone3",ClimateZoneDe::Zone4=>"Zone4"}}
fn read_climate(value:&str)->Result<ClimateZoneDe, ValueError>{match value{"Zone1"=>Ok(ClimateZoneDe::Zone1),"Zone2"=>Ok(ClimateZoneDe::Zone2),"Zone3"=>Ok(ClimateZoneDe::Zone3),"Zone4"=>Ok(ClimateZoneDe::Zone4),_=>Err(invalid("DIN4108 unsupported climate zone"))}}
fn boolean(row:FloatRow<'_>,column:usize)->Result<bool, ValueError>{match row.integer(column)?{0=>Ok(false),1=>Ok(true),_=>Err(invalid("DIN4108 requires a boolean integer"))}}
fn ordinal(value:usize)->Result<Cell<'static>, ValueError>{Ok(Cell::Integer(i64::try_from(value).map_err(|error| invalid(error.to_string()))?))}
fn checkpoint(control:&mut SqliteSnapshotControl<'_>,position:usize,total:usize)->Result<(), ValueError>{if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,total)?;}Ok(())}
fn entities<'a>(database:&'a SqliteDatabase,table:&str,columns:usize,floats:&'static[FloatColumn],control:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>, ValueError>{let rows=&database.table(table)?.rows;let mut result=Entities::new();for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;let row=FloatRow::new(row,floats)?;if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid||result.insert(row.rowid,row).is_some(){return Err(invalid(format!("{table} requires exact fields and unique positive aliased identities")));}}Ok(result)}
fn groups<'a>(rows:&Entities<'a>,parents:&Entities<'a>,control:&mut SqliteSnapshotControl<'_>)->Result<Groups<'a>, ValueError>{let mut grouped:BTreeMap<i64,BTreeMap<usize,FloatRow<'a>>>=BTreeMap::new();for(position,row)in rows.values().copied().enumerate(){checkpoint(control,position,rows.len())?;let parent=row.integer(1)?;let ordinal=usize::try_from(row.integer(2)?).map_err(|error| invalid(error.to_string()))?;if !parents.contains_key(&parent)||grouped.entry(parent).or_default().insert(ordinal,row).is_some(){return Err(invalid("DIN4108 entity has a foreign parent or duplicate ordinal"));}}let mut result=Groups::new();for(parent,siblings)in grouped{for(position,ordinal)in siblings.keys().copied().enumerate(){checkpoint(control,position,siblings.len())?;if position!=ordinal{return Err(invalid("DIN4108 ordinals must be dense"));}}result.insert(parent,siblings.into_values().collect());}Ok(result)}
fn siblings<'a>(groups:&'a Groups<'a>,parent:i64)-> &'a[FloatRow<'a>]{groups.get(&parent).map(Vec::as_slice).unwrap_or_default()}


impl Din4108Snapshot {
    fn write_sqlite_rows(&self, out: &mut RowWriter<'_,'_>) -> Result<(), ValueError> {
        out.insert_key_float("din4108_document",1,&[Cell::Text(climate(self.climate_zone)),Cell::Text(&self.usage),Cell::Real(self.t_int_c),Cell::Real(self.rh_int),Cell::Integer(i64::from(self.has_mechanical_ventilation)),Cell::Real(self.airtightness_n50),Cell::Integer(i64::from(self.bb2_details_conform))],DOCUMENT_FLOATS)?;
        for(position,z)in self.zones.iter().enumerate(){let zone=out.insert_float("din4108_zone",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&z.id),Cell::Real(z.floor_area_m2),Cell::Text(&z.heaviness),Cell::Text(&z.night_ventilation)],ZONE_FLOATS)?;for(index,w)in z.windows.iter().enumerate(){out.insert_float("din4108_window",&[Cell::Integer(zone),ordinal(index)?,Cell::Text(&w.id),Cell::Text(&w.orientation),Cell::Real(w.inclination_deg),Cell::Real(w.area_m2),Cell::Real(w.g_value),Cell::Real(w.shading_fc)],WINDOW_FLOATS)?;}}
        for(position,e)in self.elements.iter().enumerate(){let element=out.insert_float("din4108_element",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&e.id),Cell::Text(&e.kind),Cell::Text(&e.zone_id),Cell::Real(e.orientation_deg),Cell::Real(e.inclination_deg),Cell::Text(&e.adjacent),Cell::Real(e.area_m2),Cell::Real(e.delta_u_g),Cell::Real(e.delta_u_f),Cell::Real(e.delta_u_r)],ELEMENT_FLOATS)?;for(index,l)in e.layers.iter().enumerate(){let layer=out.insert_float("din4108_layer",&[Cell::Integer(element),ordinal(index)?,Cell::Text(&l.id),Cell::Text(&l.material_id),Cell::Real(l.thickness_m),Cell::Real(l.lambda),Cell::Real(l.mu),Cell::Real(l.density),Cell::Text(&l.application_type),Cell::Text(&l.compressive_class),Cell::Text(&l.water_class),Cell::Text(&l.tensile_class),Cell::Text(&l.acoustic_class)],LAYER_FLOATS)?;for(ordinal_value,s)in l.segments.iter().enumerate(){out.insert_float("din4108_segment",&[Cell::Integer(layer),ordinal(ordinal_value)?,Cell::Text(&s.id),Cell::Text(&s.material_id),Cell::Real(s.fraction),Cell::Real(s.lambda),Cell::Real(s.mu),Cell::Real(s.density)],SEGMENT_FLOATS)?;}}}
        for(position,b)in self.thermal_bridges.iter().enumerate(){out.insert_float("din4108_thermal_bridge",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&b.id),Cell::Real(b.psi),Cell::Real(b.length_m),Cell::Text(&b.bb2_type)],BRIDGE_FLOATS)?;}
        Ok(())
    }
    fn admit_sqlite_values(&self, control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {semio_s_artifact_norm_contract::sqlite_native::schema(Din4108Snapshot::SQLITE_SCHEMA,7,25,control)?; let mut out = RowWriter::borrowed(control, phase)?; self.write_sqlite_rows(&mut out)?; out.finish_borrowed() }
}
impl ArtifactSqliteSnapshot for Din4108Snapshot{
 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let limits=control.limits();store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{admission::admit(record,native,limits)?;Self::__dsl_from_record_controlled(record,native)},control)}
    const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload, ValueError>{
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let add=|count:usize,size:usize|count.checked_add(size).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Native semantic row count overflow"));let mut rows=1usize;for size in[self.zones.len(),self.elements.len(),self.thermal_bridges.len()]{rows=add(rows,size)?}control.check_rows(rows)?;for(index,zone)in self.zones.iter().enumerate(){rows=add(rows,zone.windows.len())?;control.check_rows(rows)?;if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,index+1,self.zones.len())?}}for(index,element)in self.elements.iter().enumerate(){rows=add(rows,element.layers.len())?;control.check_rows(rows)?;for(index,layer)in element.layers.iter().enumerate(){rows=add(rows,layer.segments.len())?;control.check_rows(rows)?;if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,index+1,element.layers.len())?}}if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,index+1,self.elements.len())?}}
  self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }

 fn preflight_sqlite_snapshot_encoding(&self,_:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(), ValueError>{self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
        let mut bound=NativeEncodingBound::file_only(control)?;bound.add(16384)?;bound.repeated(self.usage.len(),24)?;
        for zone in &self.zones{bound.add(16384)?;for text in [&zone.id,&zone.heaviness,&zone.night_ventilation]{bound.repeated(text.len(),24)?;}for window in &zone.windows{bound.add(16384)?;for text in [&window.id,&window.orientation]{bound.repeated(text.len(),24)?;}}}
        for element in &self.elements{bound.add(16384)?;for text in [&element.id,&element.kind,&element.zone_id,&element.adjacent]{bound.repeated(text.len(),24)?;}for layer in &element.layers{bound.add(16384)?;for text in [&layer.id,&layer.material_id,&layer.application_type,&layer.compressive_class,&layer.water_class,&layer.tensile_class,&layer.acoustic_class]{bound.repeated(text.len(),24)?;}for segment in &layer.segments{bound.add(16384)?;for text in [&segment.id,&segment.material_id]{bound.repeated(text.len(),24)?;}}}}
        for bridge in &self.thermal_bridges{bound.add(16384)?;for text in [&bridge.id,&bridge.bb2_type]{bound.repeated(text.len(),24)?;}}bound.finish()
    }

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> { let mut out = RowWriter::new(Self::SQLITE_SCHEMA, control)?; self.write_sqlite_rows(&mut out)?; out.finish() }
    fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self, ValueError>{
        validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
        let documents=entities(database,"din4108_document",8,DOCUMENT_FLOATS,control)?;if documents.len()!=1||!documents.contains_key(&1){return Err(invalid("DIN4108 requires exactly document identity one"));}let d=documents[&1];
        let zone_rows=entities(database,"din4108_zone",7,ZONE_FLOATS,control)?;let window_rows=entities(database,"din4108_window",9,WINDOW_FLOATS,control)?;let element_rows=entities(database,"din4108_element",13,ELEMENT_FLOATS,control)?;let layer_rows=entities(database,"din4108_layer",14,LAYER_FLOATS,control)?;let segment_rows=entities(database,"din4108_segment",9,SEGMENT_FLOATS,control)?;let bridge_rows=entities(database,"din4108_thermal_bridge",7,BRIDGE_FLOATS,control)?;
        let zone_groups=groups(&zone_rows,&documents,control)?;let window_groups=groups(&window_rows,&zone_rows,control)?;let element_groups=groups(&element_rows,&documents,control)?;let layer_groups=groups(&layer_rows,&element_rows,control)?;let segment_groups=groups(&segment_rows,&layer_rows,control)?;let bridge_groups=groups(&bridge_rows,&documents,control)?;
        let mut zones=Vec::new();for z in siblings(&zone_groups,1){checkpoint(control,zones.len(),0)?;let mut windows=Vec::new();for w in siblings(&window_groups,z.rowid){checkpoint(control,windows.len(),0)?;windows.push(ZoneWindow{id:reconstruct_text(control,w.text(3)?)?,orientation:reconstruct_text(control,w.text(4)?)?,inclination_deg:w.real(5)?,area_m2:w.real(6)?,g_value:w.real(7)?,shading_fc:w.real(8)?});}zones.push(ThermalZone{id:reconstruct_text(control,z.text(3)?)?,floor_area_m2:z.real(4)?,heaviness:reconstruct_text(control,z.text(5)?)?,night_ventilation:reconstruct_text(control,z.text(6)?)?,windows});}
        let mut elements=Vec::new();for e in siblings(&element_groups,1){checkpoint(control,elements.len(),0)?;let mut layers=Vec::new();for l in siblings(&layer_groups,e.rowid){checkpoint(control,layers.len(),0)?;let mut segments=Vec::new();for s in siblings(&segment_groups,l.rowid){checkpoint(control,segments.len(),0)?;segments.push(LayerSegment{id:reconstruct_text(control,s.text(3)?)?,material_id:reconstruct_text(control,s.text(4)?)?,fraction:s.real(5)?,lambda:s.real(6)?,mu:s.real(7)?,density:s.real(8)?});}layers.push(LayerDocument{id:reconstruct_text(control,l.text(3)?)?,material_id:reconstruct_text(control,l.text(4)?)?,thickness_m:l.real(5)?,lambda:l.real(6)?,mu:l.real(7)?,density:l.real(8)?,application_type:reconstruct_text(control,l.text(9)?)?,compressive_class:reconstruct_text(control,l.text(10)?)?,water_class:reconstruct_text(control,l.text(11)?)?,tensile_class:reconstruct_text(control,l.text(12)?)?,acoustic_class:reconstruct_text(control,l.text(13)?)?,segments});}elements.push(EnvelopeElement{id:reconstruct_text(control,e.text(3)?)?,kind:reconstruct_text(control,e.text(4)?)?,zone_id:reconstruct_text(control,e.text(5)?)?,orientation_deg:e.real(6)?,inclination_deg:e.real(7)?,adjacent:reconstruct_text(control,e.text(8)?)?,area_m2:e.real(9)?,delta_u_g:e.real(10)?,delta_u_f:e.real(11)?,delta_u_r:e.real(12)?,layers});}
        let mut thermal_bridges=Vec::new();for b in siblings(&bridge_groups,1){checkpoint(control,thermal_bridges.len(),0)?;thermal_bridges.push(ThermalBridge{id:reconstruct_text(control,b.text(3)?)?,psi:b.real(4)?,length_m:b.real(5)?,bb2_type:reconstruct_text(control,b.text(6)?)?});}
        let snapshot=Self{climate_zone:read_climate(d.text(1)?)?,usage:reconstruct_text(control,d.text(2)?)?,t_int_c:d.real(3)?,rh_int:d.real(4)?,has_mechanical_ventilation:boolean(d,5)?,airtightness_n50:d.real(6)?,bb2_details_conform:boolean(d,7)?,zones,elements,thermal_bridges};control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(snapshot)
    }
}

pub fn sqlite_codec()->store::ArtifactSqliteSnapshotCodec{<Din4108Snapshot as store::ArtifactSqliteSnapshot>::sqlite_codec()}

#[path="🛂️admission/🦀️.rs"]
pub(in crate::standards::v1::subsets::any)mod admission;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

