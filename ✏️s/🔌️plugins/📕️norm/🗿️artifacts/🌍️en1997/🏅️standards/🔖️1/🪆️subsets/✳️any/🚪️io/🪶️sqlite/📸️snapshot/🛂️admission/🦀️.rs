//! 🌍️ Borrowed geotechnical physical entity cells before typed ownership.
use super::*;
use store::sqlite_snapshot::artifact::RowWriter;
use semio_framework_dsl_record::RecordValue as R;
use semio_framework_value::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native::{self as n,Column::*};
fn count(root:&R)->Result<usize,ValueError>{let mut rows=1;for id in 7..=12{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}for(id,nested)in[(8,6),(9,12)]{for owner in n::list(n::field(root,id)?)?{rows=n::add(rows,n::list(n::field(n::record(owner)?,nested)?)?.len())?}}Ok(rows)}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key_float("en1997_document",1,&n::cells(root,[Text(0),Unsigned(1,u8::MAX as u64),Text(2),Text(3),Enumeration(4,&["En","De"]),Real(5),Real(6)])?,DOCUMENT)?;
 for(index,v)in n::list(n::field(root,7)?)?.iter().enumerate(){n::entity(out,"en1997_soil_layer",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10),Real(11),Real(12)])?,LAYER)?;}
 for(index,v)in n::list(n::field(root,8)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1997_footing",1,index,&n::cells(v,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5)])?,FOOTING)?;
 for(index,a)in n::list(n::field(v,6)?)?.iter().enumerate(){n::entity(out,"en1997_foundation_load_case",parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7)])?,LOAD)?;}}
 for(index,v)in n::list(n::field(root,9)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1997_pile",1,index,&n::cells(v,[Text(0),Text(1),Real(2),Real(3),Unsigned(4,u32::MAX as u64),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10),Real(11)])?,PILE)?;
 for(index,a)in n::list(n::field(v,12)?)?.iter().enumerate(){n::entity(out,"en1997_pile_test_profile",parent,index,&n::cells(n::record(a)?,[Text(0),Real(1),Real(2)])?,TEST)?;}}
 for(index,v)in n::list(n::field(root,10)?)?.iter().enumerate(){n::entity(out,"en1997_retaining_wall",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Text(8),Text(9),Real(10),Real(11),Real(12),Real(13),Real(14)])?,WALL)?;}
 for(index,v)in n::list(n::field(root,11)?)?.iter().enumerate(){n::entity(out,"en1997_slope",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Text(4)])?,SLOPE)?;}
 for(index,v)in n::list(n::field(root,12)?)?.iter().enumerate(){n::entity(out,"en1997_uplift_case",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5)])?,UPLIFT)?;}Ok(())
}
/// 🛂️ Nine authored physical tables and complete copied semantic controls.
pub(in super::super)fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{n::admit(root,native,limits,En1997Snapshot::SQLITE_SCHEMA,9,42,count(root)?,write)}
