//! 🪶️ Borrowed aluminium sections, actions and keyed connection details before typed ownership.
use super::*;
use semio_framework_dsl_record::RecordValue as R;
use semio_framework_value::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native::{self as n,Column::*};
fn count(root:&R)->Result<usize,ValueError>{let mut rows=1;for id in 1..=8{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}for(id,nested)in[(2,7),(3,11),(4,4),(7,6),(8,5)]{for item in n::list(n::field(root,id)?)?{rows=n::add(rows,n::list(n::field(n::record(item)?,nested)?)?.len())?;if id==4{rows=n::add(rows,2)?}}}Ok(rows)}
fn actions(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,r:&R,id:u16)->Result<(),ValueError>{for(index,a)in n::list(n::field(r,id)?)?.iter().enumerate(){n::entity(out,table,parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Text(2),Text(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10)])?,ACTION)?;}Ok(())}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key("en1999_document",1,&n::cells(root,[Enumeration(0,&["En","De"])])?)?;
 for(index,m)in n::list(n::field(root,1)?)?.iter().enumerate(){let c=n::cells(n::record(m)?,[Text(0),Text(1)])?;out.insert("en1999_material",&[Cell::Integer(1),n::ordinal(index)?,c[0],c[1]])?;}
 for(index,s)in n::list(n::field(root,2)?)?.iter().enumerate(){let s=n::record(s)?;let parent=n::entity(out,"en1999_section",1,index,&n::cells(s,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6)])?,SECTION)?;
 for(index,p)in n::list(n::field(s,7)?)?.iter().enumerate(){n::entity(out,"en1999_plate_element",parent,index,&n::cells(n::record(p)?,[Text(0),Real(1),Real(2),Boolean(3),Boolean(4),Real(5)])?,PLATE)?;}}
 for(index,m)in n::list(n::field(root,3)?)?.iter().enumerate(){let m=n::record(m)?;let parent=n::entity(out,"en1999_member",1,index,&n::cells(m,[Text(0),Text(1),Text(2),Real(3),Enumeration(4,&["simplySupported","continuous","cantilever"]),Real(5),Real(6),Real(7),Real(8),Real(9),Boolean(10)])?,MEMBER)?;actions(out,"en1999_member_action",parent,m,11)?;}
 for(index,c)in n::list(n::field(root,4)?)?.iter().enumerate(){let c=n::record(c)?;let cells=n::cells(c,[Text(0),Text(1),Text(2),Text(3)])?;let parent=out.insert("en1999_connection",&[Cell::Integer(1),n::ordinal(index)?,cells[0],cells[1],cells[2],cells[3]])?;actions(out,"en1999_connection_action",parent,c,4)?;
 out.insert_key_float("en1999_bolt_group",parent,&n::cells(n::record(n::field(c,5)?)?,[Text(0),Real(1),Unsigned(2,u32::MAX as u64),Unsigned(3,u32::MAX as u64),Real(4),Real(5),Real(6),Real(7)])?,BOLTS)?;
 out.insert_key_float("en1999_weld_group",parent,&n::cells(n::record(n::field(c,6)?)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,WELDS)?;}
 for(index,f)in n::list(n::field(root,5)?)?.iter().enumerate(){n::entity(out,"en1999_fire_scenario",1,index,&n::cells(n::record(f)?,[Text(0),Text(1),Real(2),Real(3)])?,FIRE)?;}
 for(index,f)in n::list(n::field(root,6)?)?.iter().enumerate(){n::entity(out,"en1999_fatigue_detail",1,index,&n::cells(n::record(f)?,[Text(0),Text(1),Text(2),Real(3),Real(4),Real(5),Real(6),Real(7)])?,FATIGUE)?;}
 for(index,s)in n::list(n::field(root,7)?)?.iter().enumerate(){let s=n::record(s)?;let parent=n::entity(out,"en1999_cold_formed_sheet",1,index,&n::cells(s,[Text(0),Text(1),Real(2),Real(3),Real(4),Boolean(5)])?,SHEET)?;actions(out,"en1999_sheet_action",parent,s,6)?;}
 for(index,s)in n::list(n::field(root,8)?)?.iter().enumerate(){let s=n::record(s)?;let parent=n::entity(out,"en1999_shell",1,index,&n::cells(s,[Text(0),Text(1),Real(2),Real(3),Real(4)])?,SHELL)?;actions(out,"en1999_shell_action",parent,s,5)?;}Ok(())
}
/// 🛂️ Sixteen authored tables and complete copied semantic controls.
pub(in super::super)fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{n::admit(root,native,limits,En1999Snapshot::SQLITE_SCHEMA,16,28,count(root)?,write)}
