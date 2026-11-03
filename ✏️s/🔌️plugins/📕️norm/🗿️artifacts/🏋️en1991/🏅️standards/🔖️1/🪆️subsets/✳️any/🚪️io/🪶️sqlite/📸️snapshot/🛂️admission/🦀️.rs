//! 🏋️ Borrowed environmental actions and impact entities before typed ownership.
use crate::sqlite::snapshot::*;
use semio_framework_dsl_record::RecordValue as R;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native as n;
use semio_s_artifact_norm_contract::sqlite_native::Column::*;
fn count(root:&R)->Result<usize,ValueError>{let mut rows=1;for id in 78..=106{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}for case in n::list(n::field(root,93)?)?{let case=n::record(case)?;for id in 4..=5{rows=n::add(rows,n::list(n::field(case,id)?)?.len())?}}for case in n::list(n::field(root,82)?)?{let case=n::record(case)?;for id in 1..=2{rows=n::add(rows,n::list(n::field(case,id)?)?.len())?}}Ok(rows)}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key_float("en1991_document",1,&n::cells(root,[
 Enumeration(0,&["En","De"]),Text(1),Real(2),Real(3),Boolean(4),Unsigned(5,u8::MAX as u64),Real(6),Unsigned(7,u8::MAX as u64),Unsigned(8,u8::MAX as u64),Real(9),Real(10),Boolean(11),Real(12),Real(13),Real(14),Real(15),Real(16),Real(17),Real(18),Real(19),Text(20),Unsigned(21,u8::MAX as u64),Real(22),Unsigned(23,u8::MAX as u64),
 Enumeration(24,&["none","nominal","parametric"]),Enumeration(25,&["standard","external","hydrocarbon","parametric"]),Real(26),Real(27),Real(28),Real(29),Real(30),Real(31),Real(32),Text(33),Real(34),Real(35),Text(36),Real(37),Enumeration(38,&["building","bridge"]),Unsigned(39,u8::MAX as u64),Real(40),Real(41),Real(42),Real(43),Real(44),Real(45),Real(46),Real(47),Text(48),Boolean(49),Text(50),Text(51),Real(52),Real(53),Real(54),Boolean(55),Text(56),Real(57),Real(58),Real(59),Real(60),Real(61),Real(62),Real(63),Real(64),Real(65),Real(66),Real(67),Real(68),Real(69),Real(70),Real(71),Real(72),Real(73),Real(74),Real(75),Real(76),Enumeration(77,&["occupancyTable","userSpecified"])])?,DOCUMENT_FLOATS)?;
 for(index,v)in n::list(n::field(root,78)?)?.iter().enumerate(){n::entity(out,"en1991_floor",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5)])?,FLOOR_FLOATS)?;}
 for(index,v)in n::list(n::field(root,79)?)?.iter().enumerate(){n::entity(out,"en1991_self_weight",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3)])?,SELF_WEIGHT_FLOATS)?;}
 for(index,v)in n::list(n::field(root,80)?)?.iter().enumerate(){n::entity(out,"en1991_roof",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Boolean(5),Real(6),Real(7),Boolean(8),Real(9)])?,ROOF_FLOATS)?;}
 for(index,v)in n::list(n::field(root,81)?)?.iter().enumerate(){n::entity(out,"en1991_wind_face",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9)])?,WIND_FLOATS)?;}
 for(index,v)in n::list(n::field(root,82)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1991_accidental_case",1,index,&n::cells(v,[Text(0)])?,&[])?;
 for(index,impact)in n::list(n::field(v,1)?)?.iter().enumerate(){n::entity(out,"en1991_impact",parent,index,&n::cells(n::record(impact)?,[Real(0),Real(1),Real(2)])?,ACCIDENTAL_FLOATS)?;}
 for(index,explosion)in n::list(n::field(v,2)?)?.iter().enumerate(){n::entity(out,"en1991_explosion",parent,index,&n::cells(n::record(explosion)?,[Real(0),Real(1),Real(2)])?,ACCIDENTAL_FLOATS)?;}}
 for(index,v)in n::list(n::field(root,83)?)?.iter().enumerate(){n::entity(out,"en1991_bridge_lane",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Unsigned(3,u8::MAX as u64)])?,BRIDGE_LANE_FLOATS)?;}
 for(index,v)in n::list(n::field(root,84)?)?.iter().enumerate(){n::entity(out,"en1991_construction_stage",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4)])?,CONSTRUCTION_STAGE_FLOATS)?;}
 for(index,v)in n::list(n::field(root,85)?)?.iter().enumerate(){n::entity(out,"en1991_roof_surface",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Text(2),Real(3),Real(4)])?,ROOF_SURFACE_FLOATS)?;}
 for(index,v)in n::list(n::field(root,86)?)?.iter().enumerate(){n::entity(out,"en1991_placement",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,PLACEMENT_FLOATS)?;}
 for(index,v)in n::list(n::field(root,87)?)?.iter().enumerate(){n::entity(out,"en1991_special_vehicle",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3)])?,SPECIAL_VEHICLE_FLOATS)?;}
 for(index,v)in n::list(n::field(root,88)?)?.iter().enumerate(){n::entity(out,"en1991_wind_bridge",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Unsigned(3,u8::MAX as u64),Real(4),Unsigned(5,u8::MAX as u64),Real(6),Real(7),Real(8)])?,WIND_BRIDGE_FLOATS)?;}
 for(index,v)in n::list(n::field(root,89)?)?.iter().enumerate(){n::entity(out,"en1991_dynamic_element",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7)])?,DYNAMIC_ELEMENT_FLOATS)?;}
 for(index,v)in n::list(n::field(root,90)?)?.iter().enumerate(){n::entity(out,"en1991_impact_action",1,index,&n::cells(n::record(v)?,[Text(0),Unsigned(1,u8::MAX as u64),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8)])?,IMPACT_ACTION_FLOATS)?;}
 for(index,v)in n::list(n::field(root,91)?)?.iter().enumerate(){n::entity(out,"en1991_internal_explosion",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,INTERNAL_EXPLOSION_FLOATS)?;}
 for(index,v)in n::list(n::field(root,92)?)?.iter().enumerate(){n::entity(out,"en1991_thermal_section",1,index,&n::cells(n::record(v)?,[Text(0),Unsigned(1,u8::MAX as u64),Unsigned(2,u8::MAX as u64),Real(3),Real(4),Real(5),Real(6)])?,THERMAL_SECTION_FLOATS)?;}
 for(index,c)in n::list(n::field(root,93)?)?.iter().enumerate(){let c=n::record(c)?;let parent=n::entity(out,"en1991_case",1,index,&n::cells(c,[Text(0),Text(1),Text(2),Text(3)])?,&[])?;
 for(i,r)in n::list(n::field(c,4)?)?.iter().enumerate(){n::entity(out,"en1991_case_number",parent,i,&n::cells(n::record(r)?,[Text(0),Real(1)])?,CASE_NUMBER_FLOATS)?;}
 for(i,r)in n::list(n::field(c,5)?)?.iter().enumerate(){n::entity(out,"en1991_case_text",parent,i,&n::cells(n::record(r)?,[Text(0),Text(1)])?,&[])?;}}
 for(index,v)in n::list(n::field(root,94)?)?.iter().enumerate(){n::entity(out,"en1991_helicopter_landing",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5)])?,HELICOPTER_LANDING_FLOATS)?;}
 for(index,v)in n::list(n::field(root,95)?)?.iter().enumerate(){n::entity(out,"en1991_tie_run",1,index,&n::cells(n::record(v)?,[Text(0),Unsigned(1,u8::MAX as u64),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9)])?,TIE_RUN_FLOATS)?;}
 for(index,v)in n::list(n::field(root,96)?)?.iter().enumerate(){n::entity(out,"en1991_construction_load",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Unsigned(2,u8::MAX as u64),Real(3)])?,CONSTRUCTION_LOAD_FLOATS)?;}
 for(index,v)in n::list(n::field(root,97)?)?.iter().enumerate(){n::entity(out,"en1991_falling_load",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5),Real(6)])?,FALLING_LOAD_FLOATS)?;}
 for(index,v)in n::list(n::field(root,98)?)?.iter().enumerate(){n::entity(out,"en1991_execution_imperfection",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5),Real(6)])?,EXECUTION_IMPERFECTION_FLOATS)?;}
 for(index,v)in n::list(n::field(root,99)?)?.iter().enumerate(){n::entity(out,"en1991_derailment_case",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5),Real(6)])?,DERAILMENT_CASE_FLOATS)?;}
 for(index,v)in n::list(n::field(root,101)?)?.iter().enumerate(){n::entity(out,"en1991_crane_hoist",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Text(3)])?,CRANE_HOIST_FLOATS)?;}
 for(index,v)in n::list(n::field(root,102)?)?.iter().enumerate(){n::entity(out,"en1991_footbridge",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,FOOTBRIDGE_FLOATS)?;}
 for(index,v)in n::list(n::field(root,103)?)?.iter().enumerate(){n::entity(out,"en1991_rail_dynamic",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Text(2)])?,RAIL_DYNAMIC_FLOATS)?;}
 for(index,v)in n::list(n::field(root,104)?)?.iter().enumerate(){n::entity(out,"en1991_load_dispersal",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,LOAD_DISPERSAL_FLOATS)?;}
 for(index,v)in n::list(n::field(root,105)?)?.iter().enumerate(){n::entity(out,"en1991_bulk_solid",1,index,&n::cells(n::record(v)?,[Text(0),Real(1)])?,BULK_SOLID_FLOATS)?;}
 for(index,v)in n::list(n::field(root,106)?)?.iter().enumerate(){n::entity(out,"en1991_rotating_machine",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3)])?,ROTATING_MACHINE_FLOATS)?;}
for(index,v)in n::list(n::field(root,107)?)?.iter().enumerate(){n::entity(out,"en1991_fatigue_model",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Unsigned(2,u8::MAX as u64),Real(3),Real(4)])?,FATIGUE_MODEL_FLOATS)?;}
for(index,v)in n::list(n::field(root,108)?)?.iter().enumerate(){n::entity(out,"en1991_support_collision",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5)])?,SUPPORT_COLLISION_FLOATS)?;}
for(index,v)in n::list(n::field(root,109)?)?.iter().enumerate(){n::entity(out,"en1991_local_fire",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5)])?,LOCAL_FIRE_FLOATS)?;}
for(index,v)in n::list(n::field(root,110)?)?.iter().enumerate(){n::entity(out,"en1991_snow_guard",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3)])?,SNOW_GUARD_FLOATS)?;}
for(index,v)in n::list(n::field(root,111)?)?.iter().enumerate(){n::entity(out,"en1991_rail_derailment",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4)])?,RAIL_DERAILMENT_FLOATS)?;}
for(index,v)in n::list(n::field(root,112)?)?.iter().enumerate(){n::entity(out,"en1991_rail_load_group",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8)])?,RAIL_LOAD_GROUP_FLOATS)?;}
for(index,v)in n::list(n::field(root,113)?)?.iter().enumerate(){n::entity(out,"en1991_dust_explosion",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5),Real(6)])?,DUST_EXPLOSION_FLOATS)?;}
 Ok(())
}
/// 🛂️ 33 authored SQL tables including the full 192-column document extent.
pub fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{n::admit(root,native,limits,En1991Snapshot::SQLITE_SCHEMA,33,189,count(root)?,write)}
