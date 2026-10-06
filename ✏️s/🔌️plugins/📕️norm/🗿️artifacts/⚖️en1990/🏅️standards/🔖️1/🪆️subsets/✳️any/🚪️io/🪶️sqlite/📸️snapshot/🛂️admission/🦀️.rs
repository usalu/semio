//! ⚖️ Borrowed design actions and genuine nullable owner references before typed ownership.
use super::*;
use semio_framework_dsl_record::RecordValue as R;
use semio_framework_value::NativeDecodeControl;
use semio_s_artifact_norm_contract::sqlite_native::{self as n,Column::*};
fn actions_count(root:&R)->Result<usize,ValueError>{let mut count=0;for id in 13..=16{count=n::add(count,n::list(n::field(root,id)?)?.len())?}Ok(count)}
fn count(root:&R)->Result<usize,ValueError>{let mut rows=n::add(1,actions_count(root)?.checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"norm action row extent overflow"))?)?;for id in 17..=19{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}Ok(rows)}
fn push<'a>(frontier:&mut Vec<(&'a str,i64)>,text:&'a str,id:i64)->Result<(),ValueError>{if frontier.len()==frontier.capacity(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"norm borrowed reference frontier exceeded admitted backing"))}frontier.push((text,id));Ok(())}
fn write<'a>(root:&'a R,out:&mut RowWriter<'_,'_>,mut actions:Vec<(&'a str,i64)>,mut members:Vec<(&'a str,i64)>)->Result<(),ValueError>{
 out.insert_key_float("en1990_document",1,&n::cells(root,[Enumeration(0,&["En","De"]),Text(1),Text(2),Real(3),Unsigned(4,u8::MAX as u64),Unsigned(5,u8::MAX as u64),Unsigned(6,u8::MAX as u64),Real(7),Real(8),Text(9),Text(10),Real(11),Real(12)])?,DOCUMENT_FLOATS)?;
 for(root_id,kind)in[(13,"permanent"),(14,"variable"),(15,"accidental"),(16,"seismic")]{for(index,a)in n::list(n::field(root,root_id)?)?.iter().enumerate(){let a=n::record(a)?;let text=n::text(n::field(a,0)?)?;let id=out.insert("en1990_action",&[Cell::Integer(1),Cell::Text(kind),n::ordinal(index)?,Cell::Text(text)])?;push(&mut actions,text,id)?;
 match root_id{13=>out.insert_key_float("en1990_permanent_action",id,&n::cells(a,[Text(1),Real(2)])?,PERMANENT_FLOATS)?,14=>out.insert_key_float("en1990_variable_action",id,&n::cells(a,[Text(1),Real(2)])?,VARIABLE_FLOATS)?,15=>out.insert_key_float("en1990_accidental_action",id,&n::cells(a,[Real(1)])?,ACCIDENTAL_FLOATS)?,16=>out.insert_key_float("en1990_seismic_action",id,&n::cells(a,[Real(1),Enumeration(2,&["I","II","III","IV"])])?,SEISMIC_FLOATS)?,_=>unreachable!()}}}
 sort_references(&mut actions,out)?;
 for(index,m)in n::list(n::field(root,17)?)?.iter().enumerate(){let m=n::record(m)?;let id=n::entity(out,"en1990_member",1,index,&n::cells(m,[Text(0),Text(1),Text(2),Real(3),Real(4),Real(5),Real(6),Real(7),Real(8),Real(9),Real(10),Real(11),Real(12)])?,MEMBER_FLOATS)?;push(&mut members,n::text(n::field(m,0)?)?,id)?;}
 sort_references(&mut members,out)?;
 for(index,b)in n::list(n::field(root,18)?)?.iter().enumerate(){let b=n::record(b)?;let reference=n::text(n::field(b,1)?)?;let member=resolve_index(&members,reference,out)?;out.insert_float("en1990_bridge_sls",&[Cell::Integer(1),n::ordinal(index)?,Cell::Text(n::text(n::field(b,0)?)?),Cell::Text(reference),member,Cell::Real(n::real(n::field(b,2)?)?),Cell::Real(n::real(n::field(b,3)?)?),Cell::Real(n::real(n::field(b,4)?)?),Cell::Real(n::real(n::field(b,5)?)?),Cell::Real(n::real(n::field(b,6)?)?),Cell::Real(n::real(n::field(b,7)?)?)],BRIDGE_FLOATS)?;}
 for(index,e)in n::list(n::field(root,19)?)?.iter().enumerate(){let e=n::record(e)?;let member_text=n::text(n::field(e,0)?)?;let action_text=n::text(n::field(e,1)?)?;let member=resolve_index(&members,member_text,out)?;let action=resolve_index(&actions,action_text,out)?;out.insert_float("en1990_effect",&[Cell::Integer(1),n::ordinal(index)?,Cell::Text(member_text),Cell::Text(action_text),member,action,Cell::Real(n::real(n::field(e,2)?)?)],EFFECT_FLOATS)?;}Ok(())
}
/// 🛂️ Nine SQL tables, exact nullable references and live cumulative paid frontiers.
pub(in super::super)fn admit(root:&R,native:&mut NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{
 n::check_schema(En1990Snapshot::SQLITE_SCHEMA,9,36,limits)?;let rows=count(root)?;if rows>limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"norm semantic rows exceed caller limit"))}
 let actions=native.allocate_vec::<(&str,i64)>(actions_count(root)?)?;let members=native.allocate_vec::<(&str,i64)>(n::list(n::field(root,17)?)?.len())?;
 n::admit(root,native,limits,En1990Snapshot::SQLITE_SCHEMA,9,36,rows,|root,out|write(root,out,actions,members))
}
