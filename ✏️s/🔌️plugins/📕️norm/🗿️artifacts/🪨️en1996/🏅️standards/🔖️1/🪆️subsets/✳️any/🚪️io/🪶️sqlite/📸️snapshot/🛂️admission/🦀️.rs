//! 🪨️ Borrowed masonry walls, openings and load cells before typed ownership.
use super::*;
use semio_framework_dsl_record::RecordValue as R;
use semio_framework_value::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native::{self as n,Column::*};
fn count(root:&R)->Result<usize,ValueError>{let walls=n::list(n::field(root,4)?)?;let mut rows=n::add(1,walls.len())?;for wall in walls{let wall=n::record(wall)?;rows=n::add(rows,n::list(n::field(wall,8)?)?.len())?;let cases=n::list(n::field(wall,32)?)?;rows=n::add(rows,cases.len())?;for case in cases{rows=n::add(rows,n::list(n::field(n::record(case)?,11)?)?.len())?}}Ok(rows)}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key("en1996_document",1,&n::cells(root,[Enumeration(0,&["En","De"]),Enumeration(1,&["Class1","Class2","Class3","Class4","Class5"]),Enumeration(2,&["Persistent","Transient","Accidental","Seismic"]),Unsigned(3,u32::MAX as u64)])?)?;
 for(index,wall)in n::list(n::field(root,4)?)?.iter().enumerate(){let wall=n::record(wall)?;let parent=n::entity(out,"en1996_wall",1,index,&n::cells(wall,[Text(0),Text(1),Text(2),Enumeration(3,&["LoadBearing","Shear","NonLoadBearing"]),Real(4),Real(5),Real(6),Unsigned(7,u8::MAX as u64),Real(9),Real(10),Real(11),Enumeration(12,&["Group1","Group2","Group3","Group4"]),Enumeration(13,&["Clay","CalciumSilicate","Aerated","Concrete"]),Real(14),Real(15),Real(16),Real(17),Enumeration(18,&["GeneralPurpose","ThinLayer","Lightweight"]),Enumeration(19,&["M1","M2_5","M5","M10","M15","M20"]),Real(20),Real(21),Boolean(22),Real(23),Real(24),Real(25),Unsigned(26,u32::MAX as u64),Enumeration(27,&["Mx1","Mx2","Mx3","Mx4","Mx5"]),Real(28),Real(29),Real(30),Boolean(31)])?,WALL_FLOATS)?;
 for(index,opening)in n::list(n::field(wall,8)?)?.iter().enumerate(){n::entity(out,"en1996_opening",parent,index,&n::cells(n::record(opening)?,[Text(0),Real(1),Real(2),Real(3)])?,OPENING_FLOATS)?;}
 for(index,case)in n::list(n::field(wall,32)?)?.iter().enumerate(){let case=n::record(case)?;let parent=n::entity(out,"en1996_load_case",parent,index,&n::cells(case,[Text(0),Text(1),Text(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10)])?,LOAD_CASE_FLOATS)?;
 for(index,load)in n::list(n::field(case,11)?)?.iter().enumerate(){n::entity(out,"en1996_concentrated_load",parent,index,&n::cells(n::record(load)?,[Text(0),Real(1),Real(2),Real(3)])?,CONCENTRATED_FLOATS)?;}}}Ok(())
}
/// 🛂️ Five authored SQL tables and complete copied semantic controls.
pub(in super::super)fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{n::admit(root,native,limits,En1996Snapshot::SQLITE_SCHEMA,5,70,count(root)?,write)}
