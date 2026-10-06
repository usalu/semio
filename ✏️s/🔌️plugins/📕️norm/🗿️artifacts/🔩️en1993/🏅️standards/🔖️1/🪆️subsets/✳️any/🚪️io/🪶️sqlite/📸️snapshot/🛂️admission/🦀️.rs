//! 🔩️ Borrowed steel material, member, joint and all force cells before typed ownership.
use super::*;
use semio_framework_dsl_record::RecordValue as R;
use semio_framework_value::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native::{self as n,Column::*};
fn count(root:&R)->Result<usize,ValueError>{let mut rows=1;for id in 1..=16{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}for(id,nested)in[(6,18),(7,4),(9,7),(10,6),(12,3),(14,4),(15,6),(16,7)]{for owner in n::list(n::field(root,id)?)?{rows=n::add(rows,n::list(n::field(n::record(owner)?,nested)?)?.len())?}}Ok(rows)}
fn forces(out:&mut RowWriter<'_,'_>,table:&str,parent:i64,r:&R,id:u16)->Result<(),ValueError>{for(index,a)in n::list(n::field(r,id)?)?.iter().enumerate(){n::entity(out,table,parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Real(2)])?,FORCE)?;}Ok(())}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key("en1993_document",1,&n::cells(root,[Enumeration(0,&["En","De"])])?)?;
 for(index,v)in n::list(n::field(root,1)?)?.iter().enumerate(){n::entity(out,"en1993_material",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Text(6),Text(7)])?,MATERIAL)?;}
 for(index,v)in n::list(n::field(root,2)?)?.iter().enumerate(){n::entity(out,"en1993_section",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Text(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10),Real(11),Real(12),Real(13),Real(14),Real(15),Real(16),Real(17),Real(18),Real(19)])?,SECTION)?;}
 for(index,v)in n::list(n::field(root,3)?)?.iter().enumerate(){n::entity(out,"en1993_member",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Text(2),Text(3),Text(4),Real(5),Real(6),Real(7),Real(8),Real(9),Text(10),Real(11),Text(12),Real(13),Text(14)])?,MEMBER)?;}
 for(index,v)in n::list(n::field(root,4)?)?.iter().enumerate(){n::entity(out,"en1993_load_case",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Text(2),Text(3)])?,&[])?;}
 for(index,v)in n::list(n::field(root,5)?)?.iter().enumerate(){let v=n::record(v)?;let c=n::cells(v,[Text(0),Text(1),Text(2)])?;let a=n::cells(n::record(n::field(v,3)?)?,[Real(0),Real(1),Real(2),Real(3),Real(4),Real(5)])?;n::entity(out,"en1993_member_action",1,index,&[c[0],c[1],c[2],a[0],a[1],a[2],a[3],a[4],a[5]],DESIGN)?;}
 for(index,v)in n::list(n::field(root,6)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1993_joint",1,index,&n::cells(v,[Text(0),Text(1),Text(2),Text(3),Real(4),Unsigned(5,u32::MAX as u64),Unsigned(6,u32::MAX as u64),Real(7),Real(8),Real(9),Real(10),Unsigned(11,u32::MAX as u64),Real(12),Real(13),Real(14),Real(15),Real(16),Text(17),Text(19),Real(20),Real(21),Real(22),Unsigned(23,u32::MAX as u64)])?,JOINT)?;
 for(index,a)in n::list(n::field(v,18)?)?.iter().enumerate(){n::entity(out,"en1993_joint_action",parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Real(2),Real(3)])?,JOINT_ACTION)?;}}
 for(index,v)in n::list(n::field(root,7)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1993_fatigue_detail",1,index,&n::cells(v,[Text(0),Text(1),Unsigned(2,u8::MAX as u64),Text(3)])?,&[])?;
 for(index,a)in n::list(n::field(v,4)?)?.iter().enumerate(){n::entity(out,"en1993_fatigue_band",parent,index,&n::cells(n::record(a)?,[Text(0),Real(1),Real(2)])?,BAND)?;}}
 for(index,v)in n::list(n::field(root,8)?)?.iter().enumerate(){n::entity(out,"en1993_fire_exposure",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Text(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9)])?,FIRE)?;}
 for(index,v)in n::list(n::field(root,9)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1993_cold_formed_member",1,index,&n::cells(v,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5),Real(6)])?,COLD)?;forces(out,"en1993_cold_formed_action",parent,v,7)?;}
 for(index,v)in n::list(n::field(root,10)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1993_plated_panel",1,index,&n::cells(v,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5)])?,PLATE)?;forces(out,"en1993_plated_action",parent,v,6)?;}
 for(index,v)in n::list(n::field(root,11)?)?.iter().enumerate(){n::entity(out,"en1993_silo_shell",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5),Real(6)])?,SILO)?;}
 for(index,v)in n::list(n::field(root,12)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1993_tension_component",1,index,&n::cells(v,[Text(0),Real(1),Real(2)])?,TENSION)?;forces(out,"en1993_tension_action",parent,v,3)?;}
 for(index,v)in n::list(n::field(root,13)?)?.iter().enumerate(){n::entity(out,"en1993_bridge_fatigue",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Unsigned(5,u8::MAX as u64),Text(6)])?,BRIDGE)?;}
 for(index,v)in n::list(n::field(root,14)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1993_tower_leg",1,index,&n::cells(v,[Text(0),Text(1),Real(2),Real(3)])?,TOWER)?;forces(out,"en1993_tower_action",parent,v,4)?;}
 for(index,v)in n::list(n::field(root,15)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1993_pile",1,index,&n::cells(v,[Text(0),Text(1),Text(2),Real(3),Real(4),Real(5)])?,PILE)?;forces(out,"en1993_pile_action",parent,v,6)?;}
 for(index,v)in n::list(n::field(root,16)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1993_crane_runway",1,index,&n::cells(v,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6)])?,CRANE)?;forces(out,"en1993_crane_action",parent,v,7)?;}Ok(())
}
/// 🛂️ Twenty-five authored tables and complete copied semantic controls.
pub(in super::super)fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{n::admit(root,native,limits,En1993Snapshot::SQLITE_SCHEMA,25,57,count(root)?,write)}
