//! 🪨️ Handwritten masonry wall, opening, load-case and concentrated-load snapshot relationships.
use crate::standards::v1::subsets::any::schema::snapshot::En1996Snapshot;
use crate::{MasonryClass, MasonryWall, MortarClass, MortarType, UnitGroup, UnitMaterial, WallLoadCase, WallType, ExposureClass, WallOpening, ConcentratedLoad, document::{AnnexChoice, DesignSituation}};
use std::collections::BTreeMap;
use store::{ArtifactSqliteSnapshot, sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, validate_sqlite_database_schema, artifact::{Cell, FloatColumn, FloatRow, NativeEncodingBound, RowWriter, reconstruct_text}}};
use semio_framework_value::{ValueError, ValueRefusalKind};
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }


const WALL_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(11),FloatColumn::Binary64(12),FloatColumn::Binary64(13),FloatColumn::Binary64(16),FloatColumn::Binary64(17),FloatColumn::Binary64(18),FloatColumn::Binary64(19),FloatColumn::Binary64(22),FloatColumn::Binary64(23),FloatColumn::Binary64(25),FloatColumn::Binary64(26),FloatColumn::Binary64(27),FloatColumn::Binary64(30),FloatColumn::Binary64(31),FloatColumn::Binary64(32)];
const OPENING_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)];
const LOAD_CASE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),FloatColumn::Binary64(12),FloatColumn::Binary64(13)];
const CONCENTRATED_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)];
type Entities<'a> = BTreeMap<i64, FloatRow<'a>>;

macro_rules! choice {
    ($print:ident,$parse:ident,$kind:ty,{$($variant:path => $text:literal),+}) => {
        fn $print(value:$kind)-> &'static str {match value {$($variant => $text),+}}
        fn $parse(text:&str)->Result<$kind, ValueError>{match text {$($text=>Ok($variant)),+, _=>Err(invalid(concat!("EN1996 unsupported ",stringify!($kind))))}}
    };
}
choice!(annex,read_annex,AnnexChoice,{AnnexChoice::En=>"En",AnnexChoice::De=>"De"});
choice!(masonry_class,read_masonry_class,MasonryClass,{MasonryClass::Class1=>"Class1",MasonryClass::Class2=>"Class2",MasonryClass::Class3=>"Class3",MasonryClass::Class4=>"Class4",MasonryClass::Class5=>"Class5"});
choice!(design_situation,read_design_situation,DesignSituation,{DesignSituation::Persistent=>"Persistent",DesignSituation::Transient=>"Transient",DesignSituation::Accidental=>"Accidental",DesignSituation::Seismic=>"Seismic"});
choice!(wall_type,read_wall_type,WallType,{WallType::LoadBearing=>"LoadBearing",WallType::Shear=>"Shear",WallType::NonLoadBearing=>"NonLoadBearing"});
choice!(unit_group,read_unit_group,UnitGroup,{UnitGroup::Group1=>"Group1",UnitGroup::Group2=>"Group2",UnitGroup::Group3=>"Group3",UnitGroup::Group4=>"Group4"});
choice!(unit_material,read_unit_material,UnitMaterial,{UnitMaterial::Clay=>"Clay",UnitMaterial::CalciumSilicate=>"CalciumSilicate",UnitMaterial::Aerated=>"Aerated",UnitMaterial::Concrete=>"Concrete"});
choice!(mortar_type,read_mortar_type,MortarType,{MortarType::GeneralPurpose=>"GeneralPurpose",MortarType::ThinLayer=>"ThinLayer",MortarType::Lightweight=>"Lightweight"});
choice!(mortar_class,read_mortar_class,MortarClass,{MortarClass::M1=>"M1",MortarClass::M2_5=>"M2_5",MortarClass::M5=>"M5",MortarClass::M10=>"M10",MortarClass::M15=>"M15",MortarClass::M20=>"M20"});
choice!(exposure,read_exposure,ExposureClass,{ExposureClass::Mx1=>"Mx1",ExposureClass::Mx2=>"Mx2",ExposureClass::Mx3=>"Mx3",ExposureClass::Mx4=>"Mx4",ExposureClass::Mx5=>"Mx5"});

fn checkpoint(control:&mut SqliteSnapshotControl<'_>,position:usize,total:usize)->Result<(), ValueError>{if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,total)?;}Ok(())}
fn entities<'a>(database:&'a SqliteDatabase,table:&str,columns:usize,floats:&'static[FloatColumn],control:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>, ValueError>{
    let rows=&database.table(table)?.rows;let mut result=Entities::new();
    for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;let row=FloatRow::new(row,floats)?;if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid||result.insert(row.rowid,row).is_some(){return Err(invalid(format!("{table} requires exact fields and unique positive aliased identities")));}}
    Ok(result)
}
fn ordered<'a>(rows:impl Iterator<Item=FloatRow<'a>>,parent:i64,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<FloatRow<'a>>, ValueError>{
    let mut result=BTreeMap::new();for(position,row)in rows.enumerate(){checkpoint(control,position,0)?;let ordinal=usize::try_from(row.integer(2)?).map_err(|error| invalid(error.to_string()))?;if row.integer(1)?!=parent||result.insert(ordinal,row).is_some(){return Err(invalid("EN1996 entity has a foreign parent or duplicate ordinal"));}}
    for(position,ordinal)in result.keys().copied().enumerate(){checkpoint(control,position,result.len())?;if position!=ordinal{return Err(invalid("EN1996 ordinals must be dense"));}}Ok(result.into_values().collect())
}
fn groups<'a>(rows:Entities<'a>,parents:&Entities<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<BTreeMap<i64,Vec<FloatRow<'a>>>, ValueError>{
    let mut result=BTreeMap::<i64,Vec<FloatRow<'a>>>::new();for(position,row)in rows.into_values().enumerate(){checkpoint(control,position,0)?;let parent=row.integer(1)?;if !parents.contains_key(&parent){return Err(invalid("EN1996 child has an unknown owning entity"));}result.entry(parent).or_default().push(row);}Ok(result)
}
fn ordinal(value:usize)->Result<Cell<'static>, ValueError>{Ok(Cell::Integer(i64::try_from(value).map_err(|error| invalid(error.to_string()))?))}
fn flag(value:bool)->Cell<'static>{Cell::Integer(i64::from(value))}
fn read_flag(row:FloatRow<'_>,column:usize)->Result<bool, ValueError>{match row.integer(column)?{0=>Ok(false),1=>Ok(true),_=>Err(invalid("EN1996 boolean requires exactly zero or one"))}}


impl En1996Snapshot {
    fn write_sqlite_rows(&self, out: &mut RowWriter<'_,'_>) -> Result<(), ValueError> {
        
        out.insert_key("en1996_document",1,&[Cell::Text(annex(self.annex)),Cell::Text(masonry_class(self.masonry_class)),Cell::Text(design_situation(self.design_situation)),Cell::Integer(i64::from(self.storeys))])?;
        for(position,wall)in self.walls.iter().enumerate(){
            let id=out.insert_float("en1996_wall",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&wall.id),Cell::Text(&wall.label_en),Cell::Text(&wall.label_de),Cell::Text(wall_type(wall.wall_type)),Cell::Real(wall.thickness_m),Cell::Real(wall.height_m),Cell::Real(wall.length_m),Cell::Integer(i64::from(wall.support_sides)),Cell::Real(wall.slab_bearing_depth_m),Cell::Real(wall.eccentricity_top_m),Cell::Real(wall.eccentricity_bottom_m),Cell::Text(unit_group(wall.unit_group)),Cell::Text(unit_material(wall.unit_material)),Cell::Real(wall.f_b_pa),Cell::Real(wall.unit_length_m),Cell::Real(wall.unit_width_m),Cell::Real(wall.unit_height_m),Cell::Text(mortar_type(wall.mortar_type)),Cell::Text(mortar_class(wall.mortar_class)),Cell::Real(wall.mortar_strength_pa),Cell::Real(wall.bed_joint_thickness_m),flag(wall.reinforced),Cell::Real(wall.as_vertical_m2),Cell::Real(wall.as_horizontal_m2),Cell::Real(wall.f_yd_pa),Cell::Integer(i64::from(wall.fire_rei_min)),Cell::Text(exposure(wall.exposure)),Cell::Real(wall.mu),Cell::Real(wall.density_kg_m3),Cell::Real(wall.phi_infinity),flag(wall.is_basement)],WALL_FLOATS)?;
            for(position,opening)in wall.openings.iter().enumerate(){out.insert_float("en1996_opening",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&opening.id),Cell::Real(opening.width_m),Cell::Real(opening.height_m),Cell::Real(opening.sill_height_m)],OPENING_FLOATS)?;}
            for(position,case)in wall.load_cases.iter().enumerate(){
                let case_id=out.insert_float("en1996_load_case",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&case.id),Cell::Text(&case.design_situation),Cell::Text(&case.imposed_category),Cell::Real(case.g_k_slab_n),Cell::Real(case.q_k_imposed_pa),Cell::Real(case.tributary_area_m2),Cell::Real(case.slab_span_m),Cell::Real(case.q_k_snow_pa),Cell::Real(case.q_p_wind_pa),Cell::Real(case.c_pe),Cell::Real(case.h_k_earth_n)],LOAD_CASE_FLOATS)?;
                for(position,load)in case.concentrated.iter().enumerate(){out.insert_float("en1996_concentrated_load",&[Cell::Integer(case_id),ordinal(position)?,Cell::Text(&load.id),Cell::Real(load.force_n),Cell::Real(load.bearing_area_m2),Cell::Real(load.bearing_length_m)],CONCENTRATED_FLOATS)?;}
            }
        }
        Ok(())
    }
    fn admit_sqlite_values(&self, control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {semio_s_artifact_norm_contract::sqlite_native::schema(En1996Snapshot::SQLITE_SCHEMA,5,70,control)?; let mut out = RowWriter::borrowed(control, phase)?; self.write_sqlite_rows(&mut out)?; out.finish_borrowed() }
}
impl ArtifactSqliteSnapshot for En1996Snapshot {
 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{let limits=control.limits();store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {admission::admit(record,native,limits)?;Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)}
    const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload, ValueError>{
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let add=|count:usize,size:usize|count.checked_add(size).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Native semantic row count overflow"));let mut rows=add(1,self.walls.len())?;control.check_rows(rows)?;for(index,wall)in self.walls.iter().enumerate(){rows=add(add(rows,wall.openings.len())?,wall.load_cases.len())?;control.check_rows(rows)?;for(index,case)in wall.load_cases.iter().enumerate(){rows=add(rows,case.concentrated.len())?;control.check_rows(rows)?;if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,index+1,wall.load_cases.len())?}}if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,index+1,self.walls.len())?}}
  self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control,native_owner)
 }

 fn preflight_sqlite_snapshot_encoding(&self,_:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(), ValueError>{self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
        let mut bound=NativeEncodingBound::file_only(control)?;bound.add(16384)?;
        for wall in &self.walls{bound.add(32768)?;for text in [&wall.id,&wall.label_en,&wall.label_de]{bound.repeated(text.len(),24)?;}for opening in &wall.openings{bound.add(4096)?;bound.repeated(opening.id.len(),24)?;}for case in &wall.load_cases{bound.add(12288)?;for text in [&case.id,&case.design_situation,&case.imposed_category]{bound.repeated(text.len(),24)?;}for load in &case.concentrated{bound.add(4096)?;bound.repeated(load.id.len(),24)?;}}}
        bound.finish()
    }

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> { let mut out = RowWriter::new(Self::SQLITE_SCHEMA, control)?; self.write_sqlite_rows(&mut out)?; out.finish() }
    fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self, ValueError>{
        validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
        let documents=entities(database,"en1996_document",5,&[],control)?;if documents.len()!=1||!documents.contains_key(&1){return Err(invalid("EN1996 requires exactly document identity one"));}let document=documents[&1];
        let wall_rows=entities(database,"en1996_wall",34,WALL_FLOATS,control)?;let load_rows=entities(database,"en1996_load_case",14,LOAD_CASE_FLOATS,control)?;
        let mut openings=groups(entities(database,"en1996_opening",7,OPENING_FLOATS,control)?,&wall_rows,control)?;let mut loads=groups(entities(database,"en1996_concentrated_load",7,CONCENTRATED_FLOATS,control)?,&load_rows,control)?;let mut cases=groups(load_rows,&wall_rows,control)?;
        let mut walls=Vec::new();for row in ordered(wall_rows.into_values(),1,control)?{
            checkpoint(control,walls.len(),0)?;let mut wall_openings=Vec::new();for opening in ordered(openings.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{checkpoint(control,wall_openings.len(),0)?;wall_openings.push(WallOpening{id:reconstruct_text(control,opening.text(3)?)?,width_m:opening.real(4)?,height_m:opening.real(5)?,sill_height_m:opening.real(6)?});}
            let mut load_cases=Vec::new();for case in ordered(cases.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{
                checkpoint(control,load_cases.len(),0)?;let mut concentrated=Vec::new();for load in ordered(loads.remove(&case.rowid).unwrap_or_default().into_iter(),case.rowid,control)?{checkpoint(control,concentrated.len(),0)?;concentrated.push(ConcentratedLoad{id:reconstruct_text(control,load.text(3)?)?,force_n:load.real(4)?,bearing_area_m2:load.real(5)?,bearing_length_m:load.real(6)?});}
                load_cases.push(WallLoadCase{id:reconstruct_text(control,case.text(3)?)?,design_situation:reconstruct_text(control,case.text(4)?)?,imposed_category:reconstruct_text(control,case.text(5)?)?,g_k_slab_n:case.real(6)?,q_k_imposed_pa:case.real(7)?,tributary_area_m2:case.real(8)?,slab_span_m:case.real(9)?,q_k_snow_pa:case.real(10)?,q_p_wind_pa:case.real(11)?,c_pe:case.real(12)?,h_k_earth_n:case.real(13)?,concentrated});
            }
            walls.push(MasonryWall{id:reconstruct_text(control,row.text(3)?)?,label_en:reconstruct_text(control,row.text(4)?)?,label_de:reconstruct_text(control,row.text(5)?)?,wall_type:read_wall_type(row.text(6)?)?,thickness_m:row.real(7)?,height_m:row.real(8)?,length_m:row.real(9)?,support_sides:u8::try_from(row.integer(10)?).map_err(|error| invalid(error.to_string()))?,openings:wall_openings,slab_bearing_depth_m:row.real(11)?,eccentricity_top_m:row.real(12)?,eccentricity_bottom_m:row.real(13)?,unit_group:read_unit_group(row.text(14)?)?,unit_material:read_unit_material(row.text(15)?)?,f_b_pa:row.real(16)?,unit_length_m:row.real(17)?,unit_width_m:row.real(18)?,unit_height_m:row.real(19)?,mortar_type:read_mortar_type(row.text(20)?)?,mortar_class:read_mortar_class(row.text(21)?)?,mortar_strength_pa:row.real(22)?,bed_joint_thickness_m:row.real(23)?,reinforced:read_flag(row,24)?,as_vertical_m2:row.real(25)?,as_horizontal_m2:row.real(26)?,f_yd_pa:row.real(27)?,fire_rei_min:u32::try_from(row.integer(28)?).map_err(|error| invalid(error.to_string()))?,exposure:read_exposure(row.text(29)?)?,mu:row.real(30)?,density_kg_m3:row.real(31)?,phi_infinity:row.real(32)?,is_basement:read_flag(row,33)?,load_cases});
        }
        let snapshot=Self{annex:read_annex(document.text(1)?)?,masonry_class:read_masonry_class(document.text(2)?)?,design_situation:read_design_situation(document.text(3)?)?,storeys:u32::try_from(document.integer(4)?).map_err(|error| invalid(error.to_string()))?,walls};control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(snapshot)
    }
}

pub fn sqlite_codec()->store::ArtifactSqliteSnapshotCodec{<En1996Snapshot as store::ArtifactSqliteSnapshot>::sqlite_codec()}

#[path="🛂️admission/🦀️.rs"]
pub(in crate::standards::v1::subsets::any)mod admission;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
