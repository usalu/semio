//! 🧱️ Complete DIN4108 envelope, layer, segment and window cells before typed binding.
use super::*;
use semio_framework_dsl_record::RecordValue as R;
use semio_framework_value::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native::{self as n,Column::*};
fn count(root:&R)->Result<usize,ValueError>{let mut rows=1;for id in[7,8,9]{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}for z in n::list(n::field(root,7)?)?{rows=n::add(rows,n::list(n::field(n::record(z)?,4)?)?.len())?}for e in n::list(n::field(root,8)?)?{for l in n::list(n::field(n::record(e)?,10)?)?{rows=n::add(rows,1)?;rows=n::add(rows,n::list(n::field(n::record(l)?,11)?)?.len())?}}Ok(rows)}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key_float("din4108_document",1,&n::cells(root,[Enumeration(0,&["Zone1","Zone2","Zone3","Zone4"]),Text(1),Real(2),Real(3),Boolean(4),Real(5),Boolean(6)])?,DOCUMENT_FLOATS)?;
 for(index,z)in n::list(n::field(root,7)?)?.iter().enumerate(){let z=n::record(z)?;let parent=n::entity(out,"din4108_zone",1,index,&n::cells(z,[Text(0),Real(1),Text(2),Text(3)])?,ZONE_FLOATS)?;
 for(index,w)in n::list(n::field(z,4)?)?.iter().enumerate(){n::entity(out,"din4108_window",parent,index,&n::cells(n::record(w)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5)])?,WINDOW_FLOATS)?;}}
 for(index,e)in n::list(n::field(root,8)?)?.iter().enumerate(){let e=n::record(e)?;let parent=n::entity(out,"din4108_element",1,index,&n::cells(e,[Text(0),Text(1),Text(2),Real(3),Real(4),Text(5),Real(6),Real(7),Real(8),Real(9)])?,ELEMENT_FLOATS)?;
 for(index,l)in n::list(n::field(e,10)?)?.iter().enumerate(){let l=n::record(l)?;let parent=n::entity(out,"din4108_layer",parent,index,&n::cells(l,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Text(6),Text(7),Text(8),Text(9),Text(10)])?,LAYER_FLOATS)?;
 for(index,s)in n::list(n::field(l,11)?)?.iter().enumerate(){n::entity(out,"din4108_segment",parent,index,&n::cells(n::record(s)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5)])?,SEGMENT_FLOATS)?;}}}
 for(index,b)in n::list(n::field(root,9)?)?.iter().enumerate(){n::entity(out,"din4108_thermal_bridge",1,index,&n::cells(n::record(b)?,[Text(0),Real(1),Real(2),Text(3)])?,BRIDGE_FLOATS)?;}Ok(())
}
/// 🛂️ All seven concrete tables, IEEE physical columns and copied limits are admitted before ownership.
pub(in super::super)fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{n::admit(root,native,limits,Din4108Snapshot::SQLITE_SCHEMA,7,25,count(root)?,write)}
