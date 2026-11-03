//! 🌍️ Borrowed geotechnical physical entity cells before typed ownership.
use crate::sqlite::snapshot::*;
use store::sqlite_snapshot::artifact::RowWriter;
use semio_framework_dsl_record::RecordValue as R;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native as n;
use semio_s_artifact_norm_contract::sqlite_native::Column::*;
fn count(root:&R)->Result<usize,ValueError>{let mut rows=1;for id in 7..=21{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}for(id,nested)in[(8,6),(9,12),(13,13),(16,7),(19,4),(19,5),(20,6)]{for owner in n::list(n::field(root,id)?)?{rows=n::add(rows,n::list(n::field(n::record(owner)?,nested)?)?.len())?}}Ok(rows)}
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key_float("en1997_document",1,&n::cells(root,[Text(0),Unsigned(1,u8::MAX as u64),Text(2),Text(3),Enumeration(4,&["En","De"]),Real(5),Real(6)])?,DOCUMENT)?;
 for(index,v)in n::list(n::field(root,7)?)?.iter().enumerate(){n::entity(out,"en1997_soil_layer",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10),Real(11),Real(12),Real(13)])?,LAYER)?;}
 for(index,v)in n::list(n::field(root,8)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1997_footing",1,index,&n::cells(v,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5)])?,FOOTING)?;
 for(index,a)in n::list(n::field(v,6)?)?.iter().enumerate(){n::entity(out,"en1997_foundation_load_case",parent,index,&n::cells(n::record(a)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7)])?,LOAD)?;}}
 for(index,v)in n::list(n::field(root,9)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1997_pile",1,index,&n::cells(v,[Text(0),Text(1),Real(2),Real(3),Unsigned(4,u32::MAX as u64),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10),Real(11),Real(12),Real(13),Real(14),Real(15),Real(16),Real(17),Real(18),Real(19),Real(20),Real(21),Real(22),Real(23),Real(24),Real(25),Enumeration(26,&["notGiven","c1215","c1620","c2025","c2530","c3037","c3545","c4050","c4555","c5060"])])?,PILE)?;
 for(index,a)in n::list(n::field(v,27)?)?.iter().enumerate(){n::entity(out,"en1997_pile_test_profile",parent,index,&n::cells(n::record(a)?,[Text(0),Real(1),Real(2)])?,TEST)?;}}
 for(index,v)in n::list(n::field(root,10)?)?.iter().enumerate(){n::entity(out,"en1997_retaining_wall",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Text(8),Text(9),Real(10),Real(11),Real(12),Real(13),Real(14),Real(15),Real(16),Real(17),Real(18),Real(19)])?,WALL)?;}
 for(index,v)in n::list(n::field(root,11)?)?.iter().enumerate(){n::entity(out,"en1997_slope",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Text(4),Real(5),Real(6),Real(7),Real(8)])?,SLOPE)?;}
 for(index,v)in n::list(n::field(root,12)?)?.iter().enumerate(){n::entity(out,"en1997_uplift_case",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4),Real(5)])?,UPLIFT)?;} for(index,v)in n::list(n::field(root,13)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1997_anchor",1,index,&n::cells(v,[Text(0),Text(1),Text(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10),Real(11),Real(12)])?,ANCHOR)?;
 for(index,a)in n::list(n::field(v,13)?)?.iter().enumerate(){n::entity(out,"en1997_anchor_test",parent,index,&n::cells(n::record(a)?,[Text(0),Real(1)])?,ANCHORTEST)?;}}
 for(index,v)in n::list(n::field(root,14)?)?.iter().enumerate(){n::entity(out,"en1997_embedded_wall",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10),Real(11)])?,EMBEDDEDWALL)?;}
 for(index,v)in n::list(n::field(root,15)?)?.iter().enumerate(){n::entity(out,"en1997_pile_group",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Unsigned(2,u32::MAX as u64),Unsigned(3,u32::MAX as u64),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10)])?,PILEGROUP)?;}
 for(index,v)in n::list(n::field(root,16)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1997_lab_test",1,index,&n::cells(v,[Text(0),Text(1),Text(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10),Real(11)])?,LABTEST)?;
 for(index,a)in n::list(n::field(v,12)?)?.iter().enumerate(){n::entity(out,"en1997_lab_point",parent,index,&n::cells(n::record(a)?,[Text(0),Real(1),Real(2)])?,LABPOINT)?;}}
 for(index,v)in n::list(n::field(root,17)?)?.iter().enumerate(){n::entity(out,"en1997_layer_water",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Real(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9)])?,LAYERWATER)?;}
 for(index,v)in n::list(n::field(root,18)?)?.iter().enumerate(){n::entity(out,"en1997_placement",1,index,&n::cells(n::record(v)?,[Text(0),Real(1),Real(2),Real(3),Real(4)])?,PLACEMENT)?;}
 for(index,c)in n::list(n::field(root,19)?)?.iter().enumerate(){let c=n::record(c)?;let parent=n::entity(out,"en1997_case",1,index,&n::cells(c,[Text(0),Text(1),Text(2),Text(3)])?,&[])?;
 for(index,r)in n::list(n::field(c,4)?)?.iter().enumerate(){n::entity(out,"en1997_case_number",parent,index,&n::cells(n::record(r)?,[Text(0),Real(1)])?,CASENUMBER)?;}
 for(index,r)in n::list(n::field(c,5)?)?.iter().enumerate(){n::entity(out,"en1997_case_text",parent,index,&n::cells(n::record(r)?,[Text(0),Text(1)])?,&[])?;}}
 for(index,v)in n::list(n::field(root,20)?)?.iter().enumerate(){let v=n::record(v)?;let parent=n::entity(out,"en1997_investigation",1,index,&n::cells(v,[Text(0),Real(1),Real(2),Real(3),Real(4),Text(5)])?,INVESTIGATION)?;
 for(index,a)in n::list(n::field(v,6)?)?.iter().enumerate(){n::entity(out,"en1997_investigation_reading",parent,index,&n::cells(n::record(a)?,[Text(0),Real(1),Real(2)])?,INVESTIGATIONREADING)?;}}
 for(index,v)in n::list(n::field(root,21)?)?.iter().enumerate(){n::entity(out,"en1997_characteristic_basis",1,index,&n::cells(n::record(v)?,[Text(0),Text(1),Text(2),Text(3)])?,&[])?;}
Ok(())
}
/// 🛂️ 20 authored physical tables and complete copied semantic controls.
pub fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{n::admit(root,native,limits,En1997Snapshot::SQLITE_SCHEMA,23,96,count(root)?,write)}
