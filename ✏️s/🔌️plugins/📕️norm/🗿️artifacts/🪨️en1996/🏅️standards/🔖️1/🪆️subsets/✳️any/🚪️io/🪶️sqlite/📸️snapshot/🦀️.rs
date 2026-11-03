//! 🪨️ Handwritten masonry wall, opening, load-case and concentrated-load snapshot relationships.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::En1996Snapshot;
use crate::{CaseNumber, CaseRow, CaseText, MasonryClass, MasonryWall, Placement, MortarClass, MortarType, UnitGroup, UnitMaterial, WallLoadCase, WallType, ExposureClass, WallOpening, ConcentratedLoad, WallPier, WallLintel, WallTie, WallPrestress, ConfinedElement, ShearFlange, WallBar, TieKind, BarKind, FrostClass, WallChase, ChaseKind, MovementJoint, BracingSystem, LoadCaseSituation, ImposedCategory};
use semio_s_plugin_norm_compliance::{AnnexChoice, DesignSituation, ReliabilityClass};
use std::collections::BTreeMap;
use store::{os_store::serialization::ArtifactSqliteSnapshotHost, sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase, validate_sqlite_database_schema, artifact::{Cell, FloatColumn, FloatRow, NativeEncodingBound, RowWriter, reconstruct_text}}};
use semio_framework_value::{ValueError, ValueRefusalKind};
fn imposed_category_text(a:ImposedCategory)->&'static str{match a{ImposedCategory::A=>"a",ImposedCategory::B=>"b",ImposedCategory::C=>"c",ImposedCategory::D=>"d",ImposedCategory::E=>"e",ImposedCategory::F=>"f",ImposedCategory::G=>"g",ImposedCategory::H=>"h",}}
fn read_imposed_category(a:&str)->Result<ImposedCategory, ValueError>{match a{"a"=>Ok(ImposedCategory::A),"b"=>Ok(ImposedCategory::B),"c"=>Ok(ImposedCategory::C),"d"=>Ok(ImposedCategory::D),"e"=>Ok(ImposedCategory::E),"f"=>Ok(ImposedCategory::F),"g"=>Ok(ImposedCategory::G),"h"=>Ok(ImposedCategory::H),_=>Err(invalid("EN1996 imposed category differs"))}}
fn load_case_situation_text(a:LoadCaseSituation)->&'static str{match a{LoadCaseSituation::Persistent=>"persistent",LoadCaseSituation::Transient=>"transient",LoadCaseSituation::Accidental=>"accidental",LoadCaseSituation::Seismic=>"seismic",}}
fn read_load_case_situation(a:&str)->Result<LoadCaseSituation, ValueError>{match a{"persistent"=>Ok(LoadCaseSituation::Persistent),"transient"=>Ok(LoadCaseSituation::Transient),"accidental"=>Ok(LoadCaseSituation::Accidental),"seismic"=>Ok(LoadCaseSituation::Seismic),_=>Err(invalid("EN1996 load case situation differs"))}}
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }

const WALL_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(11),FloatColumn::Binary64(12),FloatColumn::Binary64(13),FloatColumn::Binary64(16),FloatColumn::Binary64(17),FloatColumn::Binary64(18),FloatColumn::Binary64(19),FloatColumn::Binary64(22),FloatColumn::Binary64(23),FloatColumn::Binary64(25),FloatColumn::Binary64(26),FloatColumn::Binary64(27),FloatColumn::Binary64(30),FloatColumn::Binary64(31),FloatColumn::Binary64(32),FloatColumn::Binary64(35),FloatColumn::Binary64(36),FloatColumn::Binary64(37),FloatColumn::Binary64(38),FloatColumn::Binary64(39),FloatColumn::Binary64(40),FloatColumn::Binary64(41),FloatColumn::Binary64(42),FloatColumn::Binary64(43),FloatColumn::Binary64(44),FloatColumn::Binary64(46),FloatColumn::Binary64(47)];
const CASE_NUMBER_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4)];
const PLACEMENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];
const CHASE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10)];
const MOVEMENT_JOINT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5)];
const OPENING_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];
const PIER_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
const LINTEL_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];
const TIE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];
const PRESTRESS_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9)];
const CONFINED_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];
const FLANGE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6)];
const BAR_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7)];
const LOAD_CASE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11),FloatColumn::Binary64(12),FloatColumn::Binary64(13)];
const CONCENTRATED_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8)];
type Entities<'a> = BTreeMap<i64, FloatRow<'a>>;

macro_rules! choice {
    ($print:ident,$parse:ident,$kind:ty,{$($variant:path => $text:literal),+}) => {
        fn $print(value:$kind)-> &'static str {match value {$($variant => $text),+}}
        fn $parse(text:&str)->Result<$kind, ValueError>{match text {$($text=>Ok($variant)),+, _=>Err(invalid(concat!("EN1996 unsupported ",stringify!($kind))))}}
    };
}
choice!(annex,read_annex,AnnexChoice,{AnnexChoice::En=>"En",AnnexChoice::De=>"De"});
choice!(reliability_class,read_reliability_class,ReliabilityClass,{ReliabilityClass::RC1=>"RC1",ReliabilityClass::RC2=>"RC2",ReliabilityClass::RC3=>"RC3"});
choice!(masonry_class,read_masonry_class,MasonryClass,{MasonryClass::Class1=>"Class1",MasonryClass::Class2=>"Class2",MasonryClass::Class3=>"Class3",MasonryClass::Class4=>"Class4",MasonryClass::Class5=>"Class5"});
choice!(design_situation,read_design_situation,DesignSituation,{DesignSituation::Persistent=>"Persistent",DesignSituation::Transient=>"Transient",DesignSituation::Accidental=>"Accidental",DesignSituation::Seismic=>"Seismic"});
choice!(wall_type,read_wall_type,WallType,{WallType::LoadBearing=>"LoadBearing",WallType::Shear=>"Shear",WallType::NonLoadBearing=>"NonLoadBearing"});
choice!(unit_group,read_unit_group,UnitGroup,{UnitGroup::Group1=>"Group1",UnitGroup::Group2=>"Group2",UnitGroup::Group3=>"Group3",UnitGroup::Group4=>"Group4"});
choice!(unit_material,read_unit_material,UnitMaterial,{UnitMaterial::Clay=>"Clay",UnitMaterial::CalciumSilicate=>"CalciumSilicate",UnitMaterial::Aerated=>"Aerated",UnitMaterial::Concrete=>"Concrete"});
choice!(mortar_type,read_mortar_type,MortarType,{MortarType::GeneralPurpose=>"GeneralPurpose",MortarType::ThinLayer=>"ThinLayer",MortarType::Lightweight=>"Lightweight"});
choice!(mortar_class,read_mortar_class,MortarClass,{MortarClass::M1=>"M1",MortarClass::M2_5=>"M2_5",MortarClass::M5=>"M5",MortarClass::M10=>"M10",MortarClass::M15=>"M15",MortarClass::M20=>"M20"});
choice!(exposure,read_exposure,ExposureClass,{ExposureClass::Mx1=>"Mx1",ExposureClass::Mx2=>"Mx2",ExposureClass::Mx3=>"Mx3",ExposureClass::Mx4=>"Mx4",ExposureClass::Mx5=>"Mx5"});

choice!(tie_kind,read_tie_kind,TieKind,{TieKind::WallTie=>"WallTie",TieKind::Anchor=>"Anchor",TieKind::RingTie=>"RingTie"});
choice!(bar_kind,read_bar_kind,BarKind,{BarKind::Anchorage=>"Anchorage",BarKind::Shear=>"Shear",BarKind::DeepBeam=>"DeepBeam"});
choice!(frost_class,read_frost_class,FrostClass,{FrostClass::Exempt=>"Exempt",FrostClass::Moderate=>"Moderate",FrostClass::Severe=>"Severe"});
choice!(chase_kind,read_chase_kind,ChaseKind,{ChaseKind::Vertical=>"Vertical",ChaseKind::Horizontal=>"Horizontal",ChaseKind::Recess=>"Recess"});
choice!(bracing_system,read_bracing_system,BracingSystem,{BracingSystem::Unreinforced=>"Unreinforced",BracingSystem::Confined=>"Confined",BracingSystem::Reinforced=>"Reinforced"});

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
        
        out.insert_key("en1996_document",1,&[Cell::Text(annex(self.annex)),Cell::Text(masonry_class(self.masonry_class)),Cell::Text(design_situation(self.design_situation)),Cell::Integer(i64::from(self.storeys)),Cell::Text(reliability_class(self.reliability_class))])?;
        for(position,wall)in self.walls.iter().enumerate(){
            let id=out.insert_float("en1996_wall",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&wall.id),Cell::Text(&wall.label_en),Cell::Text(&wall.label_de),Cell::Text(wall_type(wall.wall_type)),Cell::Real(wall.thickness_m),Cell::Real(wall.height_m),Cell::Real(wall.length_m),Cell::Integer(i64::from(wall.support_sides)),Cell::Real(wall.slab_bearing_depth_m),Cell::Real(wall.eccentricity_top_m),Cell::Real(wall.eccentricity_bottom_m),Cell::Text(unit_group(wall.unit_group)),Cell::Text(unit_material(wall.unit_material)),Cell::Real(wall.f_b_pa),Cell::Real(wall.unit_length_m),Cell::Real(wall.unit_width_m),Cell::Real(wall.unit_height_m),Cell::Text(mortar_type(wall.mortar_type)),Cell::Text(mortar_class(wall.mortar_class)),Cell::Real(wall.mortar_strength_pa),Cell::Real(wall.bed_joint_thickness_m),flag(wall.reinforced),Cell::Real(wall.as_vertical_m2),Cell::Real(wall.as_horizontal_m2),Cell::Real(wall.f_yd_pa),Cell::Integer(i64::from(wall.fire_rei_min)),Cell::Text(exposure(wall.exposure)),Cell::Real(wall.mu),Cell::Real(wall.density_kg_m3),Cell::Real(wall.phi_infinity),flag(wall.is_basement),Cell::Text(frost_class(wall.frost_class)),Cell::Real(wall.cover_m),Cell::Real(wall.deflection_limit),Cell::Real(wall.soil_height_m),Cell::Real(wall.soil_density_kg_m3),Cell::Real(wall.isotherm_depth_m),Cell::Real(wall.plan_x_m),Cell::Real(wall.plan_y_m),Cell::Real(wall.plan_angle_deg),Cell::Real(wall.in_plane_shear_n),Cell::Real(wall.in_plane_moment_nm),Cell::Text(bracing_system(wall.bracing_system)),Cell::Real(wall.diaphragm_stiffness_n_per_m),Cell::Real(wall.web_spacing_m)],WALL_FLOATS)?;
            for(position,opening)in wall.openings.iter().enumerate(){out.insert_float("en1996_opening",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&opening.id),Cell::Real(opening.width_m),Cell::Real(opening.height_m),Cell::Real(opening.sill_height_m),Cell::Real(opening.position_m)],OPENING_FLOATS)?;}
            for(position,row)in wall.piers.iter().enumerate(){out.insert_float("en1996_pier",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&row.id),Cell::Real(row.position_m),Cell::Real(row.width_m)],PIER_FLOATS)?;}
            for(position,row)in wall.lintels.iter().enumerate(){out.insert_float("en1996_lintel",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&row.id),Cell::Real(row.position_m),Cell::Real(row.span_m),Cell::Real(row.depth_m),Cell::Real(row.bearing_length_m)],LINTEL_FLOATS)?;}
            for(position,row)in wall.ties.iter().enumerate(){out.insert_float("en1996_tie",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&row.id),Cell::Text(tie_kind(row.kind)),Cell::Real(row.position_m),Cell::Real(row.level_m),Cell::Real(row.capacity_n)],TIE_FLOATS)?;}
            for(position,row)in wall.prestress.iter().enumerate(){out.insert_float("en1996_prestress",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&row.id),Cell::Real(row.force_n),Cell::Real(row.area_m2),Cell::Real(row.f_pk_pa),Cell::Real(row.position_m),Cell::Real(row.eccentricity_m),Cell::Real(row.loss_ratio)],PRESTRESS_FLOATS)?;}
            for(position,row)in wall.confined.iter().enumerate(){out.insert_float("en1996_confined",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&row.id),Cell::Real(row.position_m),Cell::Real(row.width_m),Cell::Real(row.depth_m),Cell::Real(row.as_m2)],CONFINED_FLOATS)?;}
            for(position,row)in wall.flanges.iter().enumerate(){out.insert_float("en1996_flange",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&row.id),Cell::Real(row.length_m),Cell::Real(row.thickness_m),Cell::Real(row.position_m)],FLANGE_FLOATS)?;}
            for(position,row)in wall.bars.iter().enumerate(){out.insert_float("en1996_bar",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&row.id),Cell::Text(bar_kind(row.kind)),Cell::Real(row.diameter_m),Cell::Real(row.length_m),Cell::Real(row.spacing_m)],BAR_FLOATS)?;}
            for(position,case)in wall.load_cases.iter().enumerate(){
                let case_id=out.insert_float("en1996_load_case",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&case.id),Cell::Text(load_case_situation_text(case.design_situation)),Cell::Text(imposed_category_text(case.imposed_category)),Cell::Real(case.g_k_slab_n),Cell::Real(case.q_k_imposed_pa),Cell::Real(case.tributary_area_m2),Cell::Real(case.slab_span_m),Cell::Real(case.q_k_snow_pa),Cell::Real(case.q_p_wind_pa),Cell::Real(case.c_pe),Cell::Real(case.h_k_earth_n)],LOAD_CASE_FLOATS)?;
                for(position,load)in case.concentrated.iter().enumerate(){out.insert_float("en1996_concentrated_load",&[Cell::Integer(case_id),ordinal(position)?,Cell::Text(&load.id),Cell::Real(load.force_n),Cell::Real(load.bearing_area_m2),Cell::Real(load.bearing_length_m),Cell::Real(load.distance_a1_m),Cell::Real(load.position_m)],CONCENTRATED_FLOATS)?;}
            }
        }
        for(position,placement)in self.placements.iter().enumerate(){out.insert_float("en1996_placement",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&placement.id),Cell::Real(placement.x),Cell::Real(placement.y),Cell::Real(placement.z),Cell::Real(placement.yaw)],PLACEMENT_FLOATS)?;}
        for(position,row)in self.chases.iter().enumerate(){out.insert_float("en1996_chase",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&row.id),Cell::Text(&row.wall_id),Cell::Text(chase_kind(row.kind)),Cell::Real(row.x_m),Cell::Real(row.y_m),Cell::Real(row.width_m),Cell::Real(row.length_m),Cell::Real(row.depth_m)],CHASE_FLOATS)?;}
        for(position,row)in self.movement_joints.iter().enumerate(){out.insert_float("en1996_movement_joint",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&row.id),Cell::Text(&row.wall_id),Cell::Real(row.position_m)],MOVEMENT_JOINT_FLOATS)?;}
        for(position,case)in self.cases.iter().enumerate(){
            let id=out.insert_float("en1996_case",&[Cell::Integer(1),ordinal(position)?,Cell::Text(&case.id),Cell::Text(&case.label_en),Cell::Text(&case.label_de),Cell::Text(&case.entry_id)],&[])?;
            for(position,number)in case.numbers.iter().enumerate(){out.insert_float("en1996_case_number",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&number.id),Cell::Real(number.value)],CASE_NUMBER_FLOATS)?;}
            for(position,text)in case.texts.iter().enumerate(){out.insert_float("en1996_case_text",&[Cell::Integer(id),ordinal(position)?,Cell::Text(&text.id),Cell::Text(&text.value)],&[])?;}
        }
        Ok(())
    }
    fn admit_sqlite_values(&self, control: &mut SqliteSnapshotControl<'_>, phase: SqliteSnapshotPhase) -> Result<(), ValueError> {semio_s_artifact_norm_contract::sqlite_native::schema(En1996Snapshot::SQLITE_SCHEMA,18,105,control)?; let mut out = RowWriter::borrowed(control, phase)?; self.write_sqlite_rows(&mut out)?; out.finish_borrowed() }
}
impl ArtifactSqliteSnapshotHost for En1996Snapshot {
 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{let limits=control.limits();store::decode_sqlite_snapshot_record_native(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, ValueError> = (|| {admission::admit(record,native,limits)?;Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)}
    
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload, ValueError>{
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let add=|count:usize,size:usize|count.checked_add(size).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Native semantic row count overflow"));let mut rows=add(add(add(1,self.walls.len())?,self.placements.len())?,self.cases.len())?;rows=add(rows,self.chases.len())?;rows=add(rows,self.movement_joints.len())?;for case in &self.cases{rows=add(add(rows,case.numbers.len())?,case.texts.len())?}control.check_rows(rows)?;for(index,wall)in self.walls.iter().enumerate(){rows=add(add(rows,wall.openings.len())?,wall.load_cases.len())?;rows=add(rows,wall.piers.len())?;rows=add(rows,wall.lintels.len())?;rows=add(rows,wall.ties.len())?;rows=add(rows,wall.prestress.len())?;rows=add(rows,wall.confined.len())?;rows=add(rows,wall.flanges.len())?;rows=add(rows,wall.bars.len())?;control.check_rows(rows)?;for(index,case)in wall.load_cases.iter().enumerate(){rows=add(rows,case.concentrated.len())?;control.check_rows(rows)?;if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,index+1,wall.load_cases.len())?}}if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::EncodeNative,index+1,self.walls.len())?}}
  self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control,native_owner)
 }

 fn preflight_sqlite_snapshot_encoding(&self,_:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(), ValueError>{self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
        let mut bound=NativeEncodingBound::file_only(control)?;bound.add(16384)?;
        for wall in &self.walls{bound.add(32768)?;for text in [&wall.id,&wall.label_en,&wall.label_de]{bound.repeated(text.len(),24)?;}for opening in &wall.openings{bound.add(4096)?;bound.repeated(opening.id.len(),24)?;}for row in &wall.piers{bound.add(4096)?;bound.repeated(row.id.len(),24)?;}for row in &wall.lintels{bound.add(4096)?;bound.repeated(row.id.len(),24)?;}for row in &wall.ties{bound.add(4096)?;bound.repeated(row.id.len(),24)?;}for row in &wall.prestress{bound.add(4096)?;bound.repeated(row.id.len(),24)?;}for row in &wall.confined{bound.add(4096)?;bound.repeated(row.id.len(),24)?;}for row in &wall.flanges{bound.add(4096)?;bound.repeated(row.id.len(),24)?;}for row in &wall.bars{bound.add(4096)?;bound.repeated(row.id.len(),24)?;}for case in &wall.load_cases{bound.add(12288)?;for text in [case.id.as_str(),load_case_situation_text(case.design_situation),imposed_category_text(case.imposed_category)]{bound.repeated(text.len(),24)?;}for load in &case.concentrated{bound.add(4096)?;bound.repeated(load.id.len(),24)?;}}}
        for placement in &self.placements{bound.add(4096)?;bound.repeated(placement.id.len(),24)?;}
        for row in &self.chases{bound.add(4096)?;bound.repeated(row.id.len(),24)?;bound.repeated(row.wall_id.len(),24)?;}
        for row in &self.movement_joints{bound.add(4096)?;bound.repeated(row.id.len(),24)?;bound.repeated(row.wall_id.len(),24)?;}
        for case in &self.cases{bound.add(4096)?;for text in [&case.id,&case.label_en,&case.label_de,&case.entry_id]{bound.repeated(text.len(),24)?;}for number in &case.numbers{bound.add(2048)?;bound.repeated(number.id.len(),24)?;}for text in &case.texts{bound.add(2048)?;for value in [&text.id,&text.value]{bound.repeated(value.len(),24)?;}}}
        bound.finish()
    }

    
    
}
impl pack::snapshot::ArtifactSqliteSnapshot for En1996Snapshot {
const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> { let mut out = RowWriter::new(Self::SQLITE_SCHEMA, control)?; self.write_sqlite_rows(&mut out)?; out.finish() }
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self, ValueError>{
        validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
        let documents=entities(database,"en1996_document",6,&[],control)?;if documents.len()!=1||!documents.contains_key(&1){return Err(invalid("EN1996 requires exactly document identity one"));}let document=documents[&1];
        let wall_rows=entities(database,"en1996_wall",48,WALL_FLOATS,control)?;let placement_rows=entities(database,"en1996_placement",8,PLACEMENT_FLOATS,control)?;let movement_joints_rows=entities(database,"en1996_movement_joint",6,MOVEMENT_JOINT_FLOATS,control)?;let chases_rows=entities(database,"en1996_chase",11,CHASE_FLOATS,control)?;let case_rows=entities(database,"en1996_case",7,&[],control)?;let mut case_numbers=groups(entities(database,"en1996_case_number",5,CASE_NUMBER_FLOATS,control)?,&case_rows,control)?;let mut case_texts=groups(entities(database,"en1996_case_text",5,&[],control)?,&case_rows,control)?;let load_rows=entities(database,"en1996_load_case",14,LOAD_CASE_FLOATS,control)?;
        let mut openings=groups(entities(database,"en1996_opening",8,OPENING_FLOATS,control)?,&wall_rows,control)?;let mut loads=groups(entities(database,"en1996_concentrated_load",9,CONCENTRATED_FLOATS,control)?,&load_rows,control)?;let mut cases=groups(load_rows,&wall_rows,control)?;let mut g_piers=groups(entities(database,"en1996_pier",6,PIER_FLOATS,control)?,&wall_rows,control)?;let mut g_lintels=groups(entities(database,"en1996_lintel",8,LINTEL_FLOATS,control)?,&wall_rows,control)?;let mut g_ties=groups(entities(database,"en1996_tie",8,TIE_FLOATS,control)?,&wall_rows,control)?;let mut g_prestress=groups(entities(database,"en1996_prestress",10,PRESTRESS_FLOATS,control)?,&wall_rows,control)?;let mut g_confined=groups(entities(database,"en1996_confined",8,CONFINED_FLOATS,control)?,&wall_rows,control)?;let mut g_flanges=groups(entities(database,"en1996_flange",7,FLANGE_FLOATS,control)?,&wall_rows,control)?;let mut g_bars=groups(entities(database,"en1996_bar",8,BAR_FLOATS,control)?,&wall_rows,control)?;
        let mut walls=Vec::new();for row in ordered(wall_rows.into_values(),1,control)?{
            checkpoint(control,walls.len(),0)?;let mut wall_openings=Vec::new();for opening in ordered(openings.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{checkpoint(control,wall_openings.len(),0)?;wall_openings.push(WallOpening{id:reconstruct_text(control,opening.text(3)?)?,width_m:opening.real(4)?,height_m:opening.real(5)?,sill_height_m:opening.real(6)?,position_m:opening.real(7)?});}
            let mut v_piers=Vec::new();for r in ordered(g_piers.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{checkpoint(control,v_piers.len(),0)?;v_piers.push(WallPier{id:reconstruct_text(control,r.text(3)?)?,position_m:r.real(4)?,width_m:r.real(5)?});}
            let mut v_lintels=Vec::new();for r in ordered(g_lintels.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{checkpoint(control,v_lintels.len(),0)?;v_lintels.push(WallLintel{id:reconstruct_text(control,r.text(3)?)?,position_m:r.real(4)?,span_m:r.real(5)?,depth_m:r.real(6)?,bearing_length_m:r.real(7)?});}
            let mut v_ties=Vec::new();for r in ordered(g_ties.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{checkpoint(control,v_ties.len(),0)?;v_ties.push(WallTie{id:reconstruct_text(control,r.text(3)?)?,kind:read_tie_kind(r.text(4)?)?,position_m:r.real(5)?,level_m:r.real(6)?,capacity_n:r.real(7)?});}
            let mut v_prestress=Vec::new();for r in ordered(g_prestress.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{checkpoint(control,v_prestress.len(),0)?;v_prestress.push(WallPrestress{id:reconstruct_text(control,r.text(3)?)?,force_n:r.real(4)?,area_m2:r.real(5)?,f_pk_pa:r.real(6)?,position_m:r.real(7)?,eccentricity_m:r.real(8)?,loss_ratio:r.real(9)?});}
            let mut v_confined=Vec::new();for r in ordered(g_confined.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{checkpoint(control,v_confined.len(),0)?;v_confined.push(ConfinedElement{id:reconstruct_text(control,r.text(3)?)?,position_m:r.real(4)?,width_m:r.real(5)?,depth_m:r.real(6)?,as_m2:r.real(7)?});}
            let mut v_flanges=Vec::new();for r in ordered(g_flanges.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{checkpoint(control,v_flanges.len(),0)?;v_flanges.push(ShearFlange{id:reconstruct_text(control,r.text(3)?)?,length_m:r.real(4)?,thickness_m:r.real(5)?,position_m:r.real(6)?});}
            let mut v_bars=Vec::new();for r in ordered(g_bars.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{checkpoint(control,v_bars.len(),0)?;v_bars.push(WallBar{id:reconstruct_text(control,r.text(3)?)?,kind:read_bar_kind(r.text(4)?)?,diameter_m:r.real(5)?,length_m:r.real(6)?,spacing_m:r.real(7)?});}
            let mut load_cases=Vec::new();for case in ordered(cases.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{
                checkpoint(control,load_cases.len(),0)?;let mut concentrated=Vec::new();for load in ordered(loads.remove(&case.rowid).unwrap_or_default().into_iter(),case.rowid,control)?{checkpoint(control,concentrated.len(),0)?;concentrated.push(ConcentratedLoad{id:reconstruct_text(control,load.text(3)?)?,force_n:load.real(4)?,bearing_area_m2:load.real(5)?,bearing_length_m:load.real(6)?,distance_a1_m:load.real(7)?,position_m:load.real(8)?});}
                load_cases.push(WallLoadCase{id:reconstruct_text(control,case.text(3)?)?,design_situation:read_load_case_situation(case.text(4)?)?,imposed_category:read_imposed_category(case.text(5)?)?,g_k_slab_n:case.real(6)?,q_k_imposed_pa:case.real(7)?,tributary_area_m2:case.real(8)?,slab_span_m:case.real(9)?,q_k_snow_pa:case.real(10)?,q_p_wind_pa:case.real(11)?,c_pe:case.real(12)?,h_k_earth_n:case.real(13)?,concentrated});
            }
            walls.push(MasonryWall{id:reconstruct_text(control,row.text(3)?)?,label_en:reconstruct_text(control,row.text(4)?)?,label_de:reconstruct_text(control,row.text(5)?)?,wall_type:read_wall_type(row.text(6)?)?,thickness_m:row.real(7)?,height_m:row.real(8)?,length_m:row.real(9)?,support_sides:u8::try_from(row.integer(10)?).map_err(|error| invalid(error.to_string()))?,openings:wall_openings,slab_bearing_depth_m:row.real(11)?,eccentricity_top_m:row.real(12)?,eccentricity_bottom_m:row.real(13)?,unit_group:read_unit_group(row.text(14)?)?,unit_material:read_unit_material(row.text(15)?)?,f_b_pa:row.real(16)?,unit_length_m:row.real(17)?,unit_width_m:row.real(18)?,unit_height_m:row.real(19)?,mortar_type:read_mortar_type(row.text(20)?)?,mortar_class:read_mortar_class(row.text(21)?)?,mortar_strength_pa:row.real(22)?,bed_joint_thickness_m:row.real(23)?,reinforced:read_flag(row,24)?,as_vertical_m2:row.real(25)?,as_horizontal_m2:row.real(26)?,f_yd_pa:row.real(27)?,fire_rei_min:u32::try_from(row.integer(28)?).map_err(|error| invalid(error.to_string()))?,exposure:read_exposure(row.text(29)?)?,mu:row.real(30)?,density_kg_m3:row.real(31)?,phi_infinity:row.real(32)?,is_basement:read_flag(row,33)?,frost_class:read_frost_class(row.text(34)?)?,cover_m:row.real(35)?,deflection_limit:row.real(36)?,soil_height_m:row.real(37)?,soil_density_kg_m3:row.real(38)?,isotherm_depth_m:row.real(39)?,plan_x_m:row.real(40)?,plan_y_m:row.real(41)?,plan_angle_deg:row.real(42)?,in_plane_shear_n:row.real(43)?,in_plane_moment_nm:row.real(44)?,bracing_system:read_bracing_system(row.text(45)?)?,diaphragm_stiffness_n_per_m:row.real(46)?,piers:v_piers,lintels:v_lintels,ties:v_ties,prestress:v_prestress,confined:v_confined,flanges:v_flanges,bars:v_bars,load_cases});
        }
        let mut placements=Vec::new();for row in ordered(placement_rows.into_values(),1,control)?{checkpoint(control,placements.len(),0)?;placements.push(Placement{id:reconstruct_text(control,row.text(3)?)?,x:row.real(4)?,y:row.real(5)?,z:row.real(6)?,yaw:row.real(7)?});}
        let mut movement_joints=Vec::new();for row in ordered(movement_joints_rows.into_values(),1,control)?{checkpoint(control,movement_joints.len(),0)?;movement_joints.push(MovementJoint{id:reconstruct_text(control,row.text(3)?)?,wall_id:reconstruct_text(control,row.text(4)?)?,position_m:row.real(5)?});}
        let mut chases=Vec::new();for row in ordered(chases_rows.into_values(),1,control)?{checkpoint(control,chases.len(),0)?;chases.push(WallChase{id:reconstruct_text(control,row.text(3)?)?,wall_id:reconstruct_text(control,row.text(4)?)?,kind:read_chase_kind(row.text(5)?)?,x_m:row.real(6)?,y_m:row.real(7)?,width_m:row.real(8)?,length_m:row.real(9)?,depth_m:row.real(10)?});}
        let mut design_cases=Vec::new();for row in ordered(case_rows.into_values(),1,control)?{
            checkpoint(control,design_cases.len(),0)?;let mut numbers=Vec::new();for number in ordered(case_numbers.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{numbers.push(CaseNumber{id:reconstruct_text(control,number.text(3)?)?,value:number.real(4)?});}
            let mut texts=Vec::new();for text in ordered(case_texts.remove(&row.rowid).unwrap_or_default().into_iter(),row.rowid,control)?{texts.push(CaseText{id:reconstruct_text(control,text.text(3)?)?,value:reconstruct_text(control,text.text(4)?)?});}
            design_cases.push(CaseRow{id:reconstruct_text(control,row.text(3)?)?,label_en:reconstruct_text(control,row.text(4)?)?,label_de:reconstruct_text(control,row.text(5)?)?,entry_id:reconstruct_text(control,row.text(6)?)?,numbers,texts});
        }
        let snapshot=Self{annex:read_annex(document.text(1)?)?,masonry_class:read_masonry_class(document.text(2)?)?,design_situation:read_design_situation(document.text(3)?)?,storeys:u32::try_from(document.integer(4)?).map_err(|error| invalid(error.to_string()))?,reliability_class:read_reliability_class(document.text(5)?)?,walls,placements,movement_joints,chases,cases:design_cases};control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(snapshot)
    }
}

pub fn sqlite_codec()->store::os_store::serialization::ArtifactSqliteSnapshotCodec{<En1996Snapshot as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()}

#[path="🛂️admission/🦀️.rs"]
pub mod admission;

