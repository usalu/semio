//! 🌬️ Complete DIN16798 physical entities borrowed before native typed construction.
use super::*;
use semio_framework_dsl_record::{RecordValue as R};
use semio_framework_value::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native::{self as n,Column::*};
fn write(root:&R,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let d=n::cells(root,[Enumeration(0,&["En","De"]),Real(1),Real(2),Real(5),Real(6),Real(7),Real(8),Real(9)])?;
 out.insert_key_float("din16798_document",1,&d,DOCUMENT_FLOATS)?;
 for(index,value)in n::list(n::field(root,3)?)?.iter().enumerate(){let r=n::record(value)?;let c=n::cells(r,[Text(0),Text(1),Text(2),Real(3),Unsigned(4,u32::MAX as u64),Text(5),Text(6),Text(7),Real(8),Real(9),Real(10),Real(11),Real(12),Real(13),Real(14),Real(15),Real(16),Real(17),Real(18),Text(19),Text(20)])?;
 out.insert_float("din16798_zone",&[Cell::Integer(1),n::ordinal(index)?,c[0],c[1],c[2],c[3],c[4],c[5],c[6],c[7],c[8],c[9],c[10],c[11],c[12],c[13],c[14],c[15],c[16],c[17],c[18],c[19],c[20]],ZONE_FLOATS)?;
 }
 for(index,value)in n::list(n::field(root,4)?)?.iter().enumerate(){let r=n::record(value)?;let c=n::cells(r,[Text(0),Text(1),Text(2),Real(3),Unsigned(4,u8::MAX as u64),Real(5),Text(6),Text(7),Unsigned(8,u32::MAX as u64),Real(9),Real(10),Real(11),Real(12),Text(13),Real(14),Real(15),Real(16)])?;
 out.insert_float("din16798_vent_system",&[Cell::Integer(1),n::ordinal(index)?,c[0],c[1],c[2],c[3],c[4],c[5],c[6],c[7],c[8],c[9],c[10],c[11],c[12],c[13],c[14],c[15],c[16]],SYSTEM_FLOATS)?;
 }Ok(())
}
/// 🛂️ Actual closed three-table semantic census under unchanged copied caller limits.
pub(in super::super)fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{let rows=n::add(n::add(1,n::list(n::field(root,3)?)?.len())?,n::list(n::field(root,4)?)?.len())?;n::admit(root,native,limits,Din16798Snapshot::SQLITE_SCHEMA,3,48,rows,write)}
