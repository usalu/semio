//! 🧩️ Borrowed composite member and nested material cells before typed ownership.
use super::*;
use semio_framework_dsl_record::RecordValue as R;
use semio_framework_value::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native::{self as n,Column::*};
fn count(root:&R)->Result<usize,ValueError>{let mut rows=1;for(id,actions,nested)in[(3,17,4),(4,13,1),(5,9,2)]{for owner in n::list(n::field(root,id)?)?{rows=n::add(rows,nested)?;rows=n::add(rows,n::list(n::field(n::record(owner)?,actions)?)?.len())?}}Ok(rows)}
fn sheeting(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,r:&R)->Result<(),ValueError>{n::keyed(out,table,parent,&n::cells(r,[Text(0),Real(1),Real(2),Real(3),Boolean(4)])?,SHEETING)}
fn actions(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,r:&R,id:u16)->Result<(),ValueError>{for(index,a)in n::list(n::field(r,id)?)?.iter().enumerate(){n::entity(out,table,parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Text(2),Text(3),Real(4),Real(5),Real(6),Real(7)])?,ACTION)?;}Ok(())}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key_float("en1994_document",1,&n::cells(root,[Enumeration(0,&["En","De"]),Text(1),Real(2),Text(6),Real(7),Text(8)])?,DOCUMENT)?;
 for(index,b)in n::list(n::field(root,3)?)?.iter().enumerate(){let b=n::record(b)?;let parent=n::entity(out,"en1994_beam",1,index,&n::cells(b,[Text(0),Real(1),Real(2),Text(3),Text(4),Real(6),Real(7),Real(8),Real(11),Real(12),Real(13),Real(14),Real(15),Real(16)])?,BEAM)?;
 n::keyed(out,"en1994_beam_steel",parent,&n::cells(n::record(n::field(b,5)?)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8)])?,STEEL)?;
 sheeting(out,"en1994_beam_sheeting",parent,n::record(n::field(b,9)?)?)?;
 n::keyed(out,"en1994_beam_studs",parent,&n::cells(n::record(n::field(b,10)?)?,[Real(0),Real(1),Real(2),Unsigned(3,u32::MAX as u64),Real(4),Unsigned(5,u32::MAX as u64)])?,STUDS)?;
 actions(out,"en1994_beam_action",parent,b,17)?;}
 for(index,c)in n::list(n::field(root,4)?)?.iter().enumerate(){let c=n::record(c)?;let parent=n::entity(out,"en1994_column",1,index,&n::cells(c,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10),Real(11),Text(12)])?,COLUMN)?;
 for(index,a)in n::list(n::field(c,13)?)?.iter().enumerate(){n::entity(out,"en1994_column_action",parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Text(2),Text(3),Real(4),Real(5)])?,COLUMN_ACTION)?;}}
 for(index,s)in n::list(n::field(root,5)?)?.iter().enumerate(){let s=n::record(s)?;let parent=n::entity(out,"en1994_slab",1,index,&n::cells(s,[Text(0),Real(1),Text(2),Real(4),Real(5),Real(6),Real(7),Real(8)])?,SLAB)?;
 sheeting(out,"en1994_slab_sheeting",parent,n::record(n::field(s,3)?)?)?;actions(out,"en1994_slab_action",parent,s,9)?;}Ok(())
}
/// 🛂️ Eleven authored SQL tables and complete copied semantic controls.
pub(in super::super)fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{n::admit(root,native,limits,En1994Snapshot::SQLITE_SCHEMA,11,39,count(root)?,write)}
