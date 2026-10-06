//! 🏋️ Borrowed environmental actions and impact entities before typed ownership.
use super::*;
use semio_framework_dsl_record::RecordValue as R;
use semio_framework_value::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native::{self as n,Column::*};
fn count(root:&R)->Result<usize,ValueError>{let mut rows=1;for id in 65..=69{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}for case in n::list(n::field(root,69)?)?{let case=n::record(case)?;for id in 1..=2{rows=n::add(rows,n::list(n::field(case,id)?)?.len())?}}Ok(rows)}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key_float("en1991_document",1,&n::cells(root,[
 Enumeration(0,&["En","De"]),Text(1),Real(2),Real(3),Boolean(4),Unsigned(5,u8::MAX as u64),Real(6),Unsigned(7,u8::MAX as u64),Unsigned(8,u8::MAX as u64),Real(9),Real(10),Boolean(11),Real(12),Real(13),Real(14),Real(15),Real(16),Real(17),Real(18),Real(19),Text(20),Unsigned(21,u8::MAX as u64),Real(22),Unsigned(23,u8::MAX as u64),
 Enumeration(24,&["none","nominal","parametric"]),Enumeration(25,&["standard","external","hydrocarbon","parametric"]),Real(26),Real(27),Real(28),Real(29),Real(30),Real(31),Real(32),Text(33),Real(34),Real(35),Text(36),Real(37),Enumeration(38,&["building","bridge"]),Unsigned(39,u8::MAX as u64),Real(40),Real(41),Real(42),Real(43),Real(44),Real(45),Real(46),Real(47),Text(48),Boolean(49),Text(50),Text(51),Real(52),Real(53),Real(54),Boolean(55),Text(56),Real(57),Real(58),Real(59),Real(60),Real(61),Real(62),Real(63),Real(64)])?,DOCUMENT_FLOATS)?;
 for(index,v)in n::list(n::field(root,65)?)?.iter().enumerate(){n::entity(out,"en1991_floor",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5)])?,FLOOR_FLOATS)?;}
 for(index,v)in n::list(n::field(root,66)?)?.iter().enumerate(){n::entity(out,"en1991_self_weight",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3)])?,SELF_WEIGHT_FLOATS)?;}
 for(index,v)in n::list(n::field(root,67)?)?.iter().enumerate(){n::entity(out,"en1991_roof",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Boolean(5),Real(6),Real(7),Boolean(8),Real(9)])?,ROOF_FLOATS)?;}
 for(index,v)in n::list(n::field(root,68)?)?.iter().enumerate(){n::entity(out,"en1991_wind_face",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9)])?,WIND_FLOATS)?;}
 for(index,v)in n::list(n::field(root,69)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1991_accidental_case",1,index,&n::cells(v,[Text(0)])?,&[])?;
 for(index,impact)in n::list(n::field(v,1)?)?.iter().enumerate(){n::entity(out,"en1991_impact",parent,index,&n::cells(n::record(impact)?,[Real(0),Real(1),Real(2)])?,ACCIDENTAL_FLOATS)?;}
 for(index,explosion)in n::list(n::field(v,2)?)?.iter().enumerate(){n::entity(out,"en1991_explosion",parent,index,&n::cells(n::record(explosion)?,[Real(0),Real(1),Real(2)])?,ACCIDENTAL_FLOATS)?;}}Ok(())
}
/// 🛂️ Eight authored SQL tables including the full 152-column document extent.
pub(in super::super)fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{n::admit(root,native,limits,En1991Snapshot::SQLITE_SCHEMA,8,152,count(root)?,write)}
