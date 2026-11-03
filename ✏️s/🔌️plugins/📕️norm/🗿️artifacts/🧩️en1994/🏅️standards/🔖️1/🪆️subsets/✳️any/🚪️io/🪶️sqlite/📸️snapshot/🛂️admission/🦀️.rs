//! 🧩️ Borrowed composite member and nested material cells before typed ownership.
use crate::sqlite::snapshot::*;
use semio_framework_dsl_record::RecordValue as R;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native as n;
use semio_s_artifact_norm_contract::sqlite_native::Column::*;
fn count(root:&R)->Result<usize,ValueError>{let mut rows=1;for(id,actions,nested)in[(3,27,3),(4,20,1),(5,9,2)]{for owner in n::list(n::field(root,id)?)?{rows=n::add(rows,nested)?;rows=n::add(rows,n::list(n::field(n::record(owner)?,actions)?)?.len())?}}rows=n::add(rows,n::list(n::field(root,6)?)?.len())?;for c in n::list(n::field(root,12)?)?{rows=n::add(rows,1)?;rows=n::add(rows,n::list(n::field(n::record(c)?,4)?)?.len())?;rows=n::add(rows,n::list(n::field(n::record(c)?,5)?)?.len())?}Ok(rows)}
fn sheeting(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,r:&R)->Result<(),ValueError>{n::keyed(out,table,parent,&n::cells(r,[Text(0),Real(1),Real(2),Real(3),Boolean(4)])?,SHEETING)}
fn beam_actions(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,r:&R,id:u16)->Result<(),ValueError>{for(index,a)in n::list(n::field(r,id)?)?.iter().enumerate(){n::entity(out,table,parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Text(2),Text(3),Real(4),Real(5),Real(6),Real(7),Real(8)])?,BEAM_ACTION)?;}Ok(())}
fn slab_actions(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,r:&R,id:u16)->Result<(),ValueError>{for(index,a)in n::list(n::field(r,id)?)?.iter().enumerate(){n::entity(out,table,parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Text(2),Text(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9)])?,SLAB_ACTION)?;}Ok(())}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key_float("en1994_document",1,&n::cells(root,[Enumeration(0,&["En","De"]),Text(1),Real(2),Text(7),Real(8),Text(9),Enumeration(10,&["RC1","RC2","RC3"])])?,DOCUMENT)?;
 for(index,b)in n::list(n::field(root,3)?)?.iter().enumerate(){let b=n::record(b)?;let parent=n::entity(out,"en1994_beam",1,index,&n::cells(b,[Text(0),Real(1),Real(2),Text(3),Text(4),Real(6),Real(7),Real(8),Real(10),Real(11),Real(12),Real(13),Real(14),Real(15),Real(16),Real(17),Unsigned(18,u32::MAX as u64),Real(19),Real(20),Real(21),Real(22),Real(23),Real(24),Real(25),Text(26)])?,BEAM)?;
 n::keyed(out,"en1994_beam_steel",parent,&n::cells(n::record(n::field(b,5)?)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8)])?,STEEL)?;
 sheeting(out,"en1994_beam_sheeting",parent,n::record(n::field(b,9)?)?)?;
 beam_actions(out,"en1994_beam_action",parent,b,27)?;}
 for(index,c)in n::list(n::field(root,4)?)?.iter().enumerate(){let c=n::record(c)?;let parent=n::entity(out,"en1994_column",1,index,&n::cells(c,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10),Real(11),Text(12),Real(13),Real(14),Real(15),Real(16),Real(17),Real(18),Text(19)])?,COLUMN)?;
 for(index,a)in n::list(n::field(c,20)?)?.iter().enumerate(){n::entity(out,"en1994_column_action",parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Text(2),Text(3),Real(4),Real(5)])?,COLUMN_ACTION)?;}}
 for(index,s)in n::list(n::field(root,5)?)?.iter().enumerate(){let s=n::record(s)?;let parent=n::entity(out,"en1994_slab",1,index,&n::cells(s,[Text(0),Real(1),Text(2),Real(4),Real(5),Real(6),Real(7),Real(8)])?,SLAB)?;
 sheeting(out,"en1994_slab_sheeting",parent,n::record(n::field(s,3)?)?)?;slab_actions(out,"en1994_slab_action",parent,s,9)?;}
 for(index,p)in n::list(n::field(root,6)?)?.iter().enumerate(){n::entity(out,"en1994_placement",1,index,&n::cells(n::record(p)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,PLACEMENT)?;} for(index,p)in n::list(n::field(root,11)?)?.iter().enumerate(){n::entity(out,"en1994_stud",1,index,&n::cells(n::record(p)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5)])?,STUD)?;}
 for(index,c)in n::list(n::field(root,12)?)?.iter().enumerate(){let c=n::record(c)?;let parent=n::entity(out,"en1994_case",1,index,&n::cells(c,[Text(0),Text(1),Text(2),Text(3)])?,&[])?;for(i,r)in n::list(n::field(c,4)?)?.iter().enumerate(){n::entity(out,"en1994_case_number",parent,i,&n::cells(n::record(r)?,[Text(0),Real(1)])?,CASE_NUMBER)?;}for(i,r)in n::list(n::field(c,5)?)?.iter().enumerate(){n::entity(out,"en1994_case_text",parent,i,&n::cells(n::record(r)?,[Text(0),Text(1)])?,&[])?;}}
 Ok(())
}
/// 🛂️ Fifteen authored SQL tables and complete copied semantic controls.
pub fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{n::admit(root,native,limits,En1994Snapshot::SQLITE_SCHEMA,15,39,count(root)?,write)}
