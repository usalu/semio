//! 🏭️ Borrowed VDI catalogue, custom tagged values and physical cells before typed ownership.
use super::*;
use semio_framework_dsl_record::{RecordValue as R,FieldValue as F};
use semio_framework_value::{DslValue as V,NativeDecodeControl};
use semio_s_artifact_norm_contract::sqlite_native::{self as n,Column::*};
type Object<'a>=&'a[(String,V)];
fn attributes(configuration:&R)->Result<Object<'_>,ValueError>{match n::field(configuration,1)?{F::Value(V::Object(fields))=>Ok(fields),_=>Err(invalid("VDI native attributes require their authored value object"))}}
fn vf<'a>(fields:Object<'a>,key:&str)->Result<&'a V,ValueError>{crate::sheet_attribute_field(fields,key)}
fn vt<'a>(fields:Object<'a>,key:&str)->Result<Cell<'a>,ValueError>{vf(fields,key)?.as_str().map(Cell::Text).ok_or_else(||invalid("VDI attribute text required"))}
fn vr(fields:Object<'_>,key:&str)->Result<Cell<'static>,ValueError>{crate::sheet_attribute_real(fields,key).map(Cell::Real)}
fn vu(fields:Object<'_>,key:&str)->Result<Cell<'static>,ValueError>{crate::sheet_attribute_u16(fields,key).map(|value|Cell::Integer(i64::from(value)))}
fn vo<'a>(fields:Object<'a>,key:&str)->Result<Cell<'a>,ValueError>{match fields.iter().find(|(k,_)|k==key).map(|(_,v)|v){None|Some(V::Null)=>Ok(Cell::Null),Some(V::String(value))=>Ok(Cell::Text(value)),_=>Err(invalid("VDI optional attribute text required"))}}
fn kind(fields:Object<'_>)->Result<&str,ValueError>{vf(fields,"kind")?.as_str().ok_or_else(||invalid("VDI attribute kind required"))}
fn entries(fields:Object<'_>)->Result<&[V],ValueError>{match fields.iter().find(|(k,_)|k=="entries").map(|(_,v)|v){None=>Ok(&[]),Some(V::Array(values))=>Ok(values),_=>Err(invalid("VDI generic attribute list required"))}}
fn extension(value:&F)->Result<&[(String,F)],ValueError>{n::map(n::field(n::record(value)?,0)?)}
fn count(root:&R,native:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{
 let catalog=n::record(n::field(root,0)?)?;let file=n::record(n::field(catalog,0)?)?;let mut rows=n::add(n::add(5,extension(n::field(catalog,2)?)?.len())?,extension(n::field(file,6)?)?.len())?;
 for p in n::list(n::field(catalog,1)?)?{let p=n::record(p)?;rows=n::add(rows,3)?;for id in[2,6,7]{rows=n::add(rows,n::list(n::field(p,id)?)?.len())?}rows=n::add(rows,extension(n::field(p,8)?)?.len())?;
 for r in n::list(n::field(p,4)?)?{let r=n::record(r)?;rows=n::add(rows,n::add(n::add(1,n::list(n::field(r,1)?)?.len())?,extension(n::field(r,2)?)?.len())?)?;}
 let c=n::record(n::field(p,5)?)?;rows=n::add(rows,n::list(n::field(c,3)?)?.len())?;let fields=attributes(c)?;let allowed:&[&str]=match kind(fields)?{"valveHeating"=>&["kind","dn","kvsM3S","pressureClass","connectionType","authorityMin","authorityMax"],"radiator"=>&["kind","standardOutputW","heatExponentN","lengthM","heightM","depthM","connectionType"],"pumpHeating"=>&["kind","dnSuction","dnDischarge","nominalFlowM3S","nominalHeadM","motorPowerW","hydraulicEfficiency","qhCurveRef"],"heatGenerator"=>&["kind","nominalHeatOutputW","fuelType","flowTempMaxC","returnTempMinC"],"generic"=>&["kind","entries"],_=>return Err(invalid("unknown VDI sheet attribute kind"))};let F::Value(value)=n::field(c,1)?else{unreachable!()};crate::sheet_attribute_object(value,allowed,native)?;
 if kind(fields)?=="generic"{let entries=entries(fields)?;rows=n::add(rows,entries.len())?;for entry in entries{crate::sheet_attribute_object(entry,&["key","value","unit"],native)?;}}
 }
 rows=n::add(rows,n::map(n::field(root,1)?)?.len())?;for entry in n::list(n::field(n::record(n::field(root,4)?)?,0)?)?{let e=n::record(entry)?;rows=n::add(rows,n::add(1,n::list(n::field(e,2)?)?.len())?)?;}
 for(_,g)in n::map(n::field(root,5)?)?{let g=n::record(g)?;rows=n::add(rows,n::add(n::add(1,n::list(n::field(g,2)?)?.len())?,n::map(n::field(g,3)?)?.len())?)?;}
 for(_,c)in n::map(n::field(root,6)?)?{rows=n::add(rows,n::add(3,n::list(n::field(n::record(c)?,3)?)?.len())?)?;}Ok(rows)
}
fn extensions(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,value:&F)->Result<(),ValueError>{for(key,value)in extension(value)?{out.insert(table,&[Cell::Integer(parent),Cell::Text(key),Cell::Text(n::text(value)?)])?;}Ok(())}
fn strings(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,value:&F)->Result<(),ValueError>{for(index,value)in n::list(value)?.iter().enumerate(){out.insert(table,&[Cell::Integer(parent),n::ordinal(index)?,Cell::Text(n::text(value)?)])?;}Ok(())}
fn limit(value:&F)->Result<Cell<'static>,ValueError>{match value{F::UInt(value)if usize::try_from(*value).is_ok()=>Ok(Cell::Integer(*value as i64)),_=>Err(invalid("VDI security limit exceeds native usize"))}}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let catalog=n::record(n::field(root,0)?)?;let file=n::record(n::field(catalog,0)?)?;let date=n::record(n::field(root,2)?)?;out.insert_key("vdi3805_document",1,&[Cell::Integer(n::unsigned(n::field(date,0)?,u16::MAX as u64)?),Cell::Integer(n::unsigned(n::field(date,1)?,u8::MAX as u64)?),Cell::Integer(n::boolean(n::field(root,3)?)?)])?;
 out.insert_key("vdi3805_catalogue",1,&[Cell::Integer(1)])?;let cells=n::cells(file,[Text(0),Text(1),Text(3),Text(4),Unsigned(5,u32::MAX as u64)])?;out.insert_key("vdi3805_file",1,&[Cell::Integer(1),cells[0],cells[1],cells[2],cells[3],cells[4]])?;
 let b=n::record(n::field(file,2)?)?;let cells=n::cells(b,[Text(0),Text(1),Unsigned(2,u32::MAX as u64)])?;out.insert_key("vdi3805_building_system",1,&[Cell::Integer(1),cells[0],cells[1],cells[2]])?;
 let l=n::record(n::field(root,7)?)?;out.insert_key("vdi3805_limits",1,&[Cell::Integer(1),limit(n::field(l,0)?)?,limit(n::field(l,1)?)?,limit(n::field(l,2)?)?,limit(n::field(l,3)?)?])?;
 extensions(out,"vdi3805_catalogue_extension",1,n::field(catalog,2)?)?;extensions(out,"vdi3805_file_extension",1,n::field(file,6)?)?;
 for(index,p)in n::list(n::field(catalog,1)?)?.iter().enumerate(){let p=n::record(p)?;let identity=n::cells(n::record(n::field(p,1)?)?,[Text(0),Text(1),Text(2)])?;let parent=out.insert("vdi3805_product",&[Cell::Integer(1),n::ordinal(index)?,Cell::Text(n::text(n::field(p,0)?)?),identity[0],identity[1],identity[2],Cell::Integer(n::unsigned(n::field(p,3)?,u16::MAX as u64)?)])?;
 for(index,t)in n::list(n::field(p,2)?)?.iter().enumerate(){n::entity(out,"vdi3805_title",parent,index,&n::cells(n::record(t)?,[Text(0),Text(1)])?,&[])?;}
 for(index,r)in n::list(n::field(p,4)?)?.iter().enumerate(){let r=n::record(r)?;let id=n::entity(out,"vdi3805_record",parent,index,&n::cells(r,[Text(0)])?,&[])?;strings(out,"vdi3805_record_field",id,n::field(r,1)?)?;extensions(out,"vdi3805_record_extension",id,n::field(r,2)?)?;}
 let c=n::record(n::field(p,5)?)?;let fields=attributes(c)?;let tag=kind(fields)?;out.insert_key("vdi3805_configuration",parent,&[Cell::Integer(parent),Cell::Text(n::text(n::field(c,0)?)?),Cell::Text(tag),n::optional(n::field(c,2)?)?])?;strings(out,"vdi3805_function_ref",parent,n::field(c,3)?)?;
 match tag{
 "valveHeating"=>out.insert_key_float("vdi3805_valve",parent,&[Cell::Integer(parent),vu(fields,"dn")?,vr(fields,"kvsM3S")?,vt(fields,"pressureClass")?,vt(fields,"connectionType")?,vr(fields,"authorityMin")?,vr(fields,"authorityMax")?],VALVE)?,
 "radiator"=>out.insert_key_float("vdi3805_radiator",parent,&[Cell::Integer(parent),vr(fields,"standardOutputW")?,vr(fields,"heatExponentN")?,vr(fields,"lengthM")?,vr(fields,"heightM")?,vr(fields,"depthM")?,vt(fields,"connectionType")?],RADIATOR)?,
 "pumpHeating"=>out.insert_key_float("vdi3805_pump",parent,&[Cell::Integer(parent),vu(fields,"dnSuction")?,vu(fields,"dnDischarge")?,vr(fields,"nominalFlowM3S")?,vr(fields,"nominalHeadM")?,vr(fields,"motorPowerW")?,vr(fields,"hydraulicEfficiency")?,vo(fields,"qhCurveRef")?],PUMP)?,
 "heatGenerator"=>out.insert_key_float("vdi3805_heat_generator",parent,&[Cell::Integer(parent),vr(fields,"nominalHeatOutputW")?,vt(fields,"fuelType")?,vr(fields,"flowTempMaxC")?,vr(fields,"returnTempMinC")?],HEAT)?,
 "generic"=>{out.insert_key("vdi3805_generic",parent,&[Cell::Integer(parent)])?;for(index,x)in entries(fields)?.iter().enumerate(){let V::Object(fields)=x else{return Err(invalid("VDI generic entry object required"))};out.insert("vdi3805_generic_entry",&[Cell::Integer(parent),n::ordinal(index)?,vt(fields,"key")?,vt(fields,"value")?,vo(fields,"unit")?])?;}},
 _=>return Err(invalid("unknown VDI sheet attribute kind"))}
 for(index,a)in n::list(n::field(p,6)?)?.iter().enumerate(){n::entity(out,"vdi3805_accessory",parent,index,&n::cells(n::record(a)?,[Text(0),Boolean(1),Unsigned(2,u32::MAX as u64)])?,&[])?;}
 for(index,a)in n::list(n::field(p,7)?)?.iter().enumerate(){n::entity(out,"vdi3805_component",parent,index,&n::cells(n::record(a)?,[Text(0),Unsigned(1,u32::MAX as u64)])?,&[])?;}extensions(out,"vdi3805_product_extension",parent,n::field(p,8)?)?;
 }
 for(key,value)in n::map(n::field(root,1)?)?{out.insert("vdi3805_edition",&[Cell::Integer(1),Cell::Text(key),Cell::Text(n::enumeration(value,&["legacy","current"])?)])?;}
 for(index,x)in n::list(n::field(n::record(n::field(root,4)?)?,0)?)?.iter().enumerate(){let x=n::record(x)?;let dn=match n::field(x,3)?{F::Absent=>Cell::Null,value=>Cell::Integer(n::unsigned(value,u16::MAX as u64)?)};let parent=out.insert("vdi3805_index_entry",&[Cell::Integer(1),n::ordinal(index)?,Cell::Text(n::text(n::field(x,0)?)?),Cell::Integer(n::unsigned(n::field(x,1)?,u16::MAX as u64)?),dn])?;strings(out,"vdi3805_index_tag",parent,n::field(x,2)?)?;}
 for(key,g)in n::map(n::field(root,5)?)?{let g=n::record(g)?;let b=n::cells(n::record(n::field(g,1)?)?,[Real(0),Real(1),Real(2),Real(3),Real(4),Real(5)])?;let parent=out.insert_float("vdi3805_geometry",&[Cell::Integer(1),Cell::Text(key),Cell::Text(n::text(n::field(g,0)?)?),b[0],b[1],b[2],b[3],b[4],b[5]],GEOMETRY)?;
 for(index,c)in n::list(n::field(g,2)?)?.iter().enumerate(){let c=n::record(c)?;let p=n::tuple(n::field(c,2)?,3)?;let d=n::tuple(n::field(c,3)?,3)?;let diameter=match n::field(c,4)?{F::Absent=>None,value=>Some(n::real(value)?)};out.insert_float("vdi3805_connection",&[Cell::Integer(parent),n::ordinal(index)?,Cell::Text(n::text(n::field(c,0)?)?),Cell::Text(n::text(n::field(c,1)?)?),Cell::Real(n::real(&p[0])?),Cell::Real(n::real(&p[1])?),Cell::Real(n::real(&p[2])?),Cell::Real(n::real(&d[0])?),Cell::Real(n::real(&d[1])?),Cell::Real(n::real(&d[2])?),Cell::Integer(i64::from(diameter.is_some())),Cell::Real(diameter.unwrap_or(0.0))],CONNECTION)?;}
 for(key,value)in n::map(n::field(g,3)?)?{out.insert_float("vdi3805_geometry_parameter",&[Cell::Integer(parent),Cell::Text(key),Cell::Real(n::real(value)?)],PARAMETER)?;}}
 for(key,c)in n::map(n::field(root,6)?)?{let c=n::record(c)?;let parent=out.insert("vdi3805_curve",&[Cell::Integer(1),Cell::Text(key),Cell::Text(n::text(n::field(c,0)?)?)])?;
 for(axis,id)in[(0,1),(1,2)]{let u=n::record(n::field(c,id)?)?;out.insert_float("vdi3805_curve_unit",&[Cell::Integer(parent),Cell::Integer(axis),Cell::Text(n::text(n::field(u,0)?)?),Cell::Text(n::enumeration(n::field(u,1)?,&["dimensionless","length","area","volume","mass","time","temperature","force","pressure","stress","moment","energy","power","thermalConductivity","thermalResistance","heatTransferCoefficient","airPermeability","ventilationRate","acceleration"])?),Cell::Integer(n::boolean(n::field(u,2)?)?),Cell::Real(n::real(n::field(u,3)?)?)],UNIT)?;}
 for(index,p)in n::list(n::field(c,3)?)?.iter().enumerate(){n::entity(out,"vdi3805_curve_point",parent,index,&n::cells(n::record(p)?,[Real(0),Real(1)])?,POINT)?;}}Ok(())
}
/// 🛂️ Thirty-two physical tables with all authored tagged branches and copied controls.
pub(in super::super)fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{native.scoped_stage(|native|{native.begin_stage(0)?;let rows=count(root,native)?;n::admit(root,native,limits,Vdi3805Snapshot::SQLITE_SCHEMA,32,27,rows,write)})}
