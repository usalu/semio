//! ⚡️ Borrowed building, climate and actual child-reference cells before typed ownership.
use super::*;
use semio_framework_dsl_record::{RecordValue as R,FieldValue as F};
use semio_framework_value::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native::{self as n,Column::*};
fn plant(root:&R)->Result<Option<&R>,ValueError>{let cooling=n::record(n::field(root,14)?)?;match n::field(cooling,0)?{F::Absent=>Ok(None),value=>n::record(value).map(Some)}}
fn count(root:&R)->Result<usize,ValueError>{let rows=n::add(20,n::list(n::field(root,9)?)?.len())?;n::add(n::add(rows,n::list(n::field(root,10)?)?.len())?,usize::from(plant(root)?.is_some()))}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key_float("din18599_document",1,&n::cells(root,[Enumeration(0,&["Residential","NonResidential"]),Enumeration(1,&["Detached","SemiDetached","EndTerrace","MidTerrace"]),Enumeration(2,&["Residential","Office","School"]),Enumeration(3,&["DetailedMonthly","Tabular"]),Real(4),Real(5),Real(6),Real(7),Enumeration(8,&["A","B","C","D"])])?,DOCUMENT_FLOATS)?;
 let climate=n::record(n::field(root,17)?)?;let theta=n::tuple(n::field(climate,0)?,12)?;let irradiance=n::tuple(n::field(climate,1)?,12)?;for(month,(theta,g))in(1i64..).zip(theta.iter().zip(irradiance)){out.insert_key_float("din18599_climate_month",month,&[Cell::Integer(1),Cell::Real(n::real(theta)?),Cell::Real(n::real(g)?)],CLIMATE_FLOATS)?;}
 let child=n::record(n::field(root,18)?)?;let target=n::record(n::field(child,1)?)?;out.insert_key("din18599_climate_child",1,&[Cell::Integer(1),Cell::Text(n::text(n::field(child,0)?)?),Cell::Text(n::text(n::field(target,0)?)?),Cell::Text(n::text(n::field(target,1)?)?),Cell::Text(n::text(n::field(target,2)?)?),Cell::Text(n::text(n::field(target,3)?)?)])?;
 for(index,z)in n::list(n::field(root,9)?)?.iter().enumerate(){n::entity(out,"din18599_zone",1,index,&n::cells(n::record(z)?,[Text(0),Text(1),Text(2),Enumeration(3,&["WFH","Office","School"]),Real(4),Real(5),Real(6),Real(7),Unsigned(8,u32::MAX as u64),Real(9),Real(10)])?,ZONE_FLOATS)?;}
 for(index,e)in n::list(n::field(root,10)?)?.iter().enumerate(){n::entity(out,"din18599_element",1,index,&n::cells(n::record(e)?,[Text(0),Text(1),Text(2),Enumeration(3,&["Wall","Roof","Floor","Door","Window"]),Text(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10),Enumeration(11,&["Outdoor","Ground","Unheated","Heated"])])?,ELEMENT_FLOATS)?;}
 n::keyed(out,"din18599_heating",1,&n::cells(n::record(n::field(root,11)?)?,[Real(0),Real(1),Real(2),Real(3),Text(4)])?,HEATING_FLOATS)?;
 n::keyed(out,"din18599_dhw",1,&n::cells(n::record(n::field(root,12)?)?,[Real(0),Real(1),Real(2),Text(3)])?,DHW_FLOATS)?;
 n::keyed(out,"din18599_ventilation",1,&n::cells(n::record(n::field(root,13)?)?,[Real(0),Real(1),Real(2)])?,VENTILATION_FLOATS)?;
 out.insert_key("din18599_cooling",1,&[Cell::Integer(1)])?;if let Some(plant)=plant(root)?{n::keyed(out,"din18599_cooling_plant",1,&n::cells(plant,[Real(0),Text(1)])?,PLANT_FLOATS)?;}
 n::keyed(out,"din18599_lighting",1,&n::cells(n::record(n::field(root,15)?)?,[Real(0)])?,LIGHTING_FLOATS)?;
 n::keyed(out,"din18599_renewables",1,&n::cells(n::record(n::field(root,16)?)?,[Real(0),Real(1),Real(2)])?,RENEWABLES_FLOATS)?;Ok(())
}
/// 🛂️ Twelve authored SQL tables and complete copied semantic controls.
pub(in super::super)fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{n::admit(root,native,limits,Din18599Snapshot::SQLITE_SCHEMA,12,27,count(root)?,write)}
