//! 📏️ Borrowed EN1992 native roles admit every authored relational cell before typed construction.
use super::{invalid,annex,member_kind,support,exposure,ductility,fire_rating,tightness,En1992Snapshot};
use crate::{MemberKind,SupportCondition,ExposureClass,DuctilityClass,FireRating,TightnessClass,document::AnnexChoice};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_dsl_record::{DslField,RecordValue as R,FieldValue as F};
use semio_framework_value::{ValueError,ValueRefusalKind,NativeDecodeControl as N};
type Result<T>=std::result::Result<T,ValueError>;
#[derive(Clone,Copy)]enum Role{Text,Float,UInt,Bool,Annex,Member,Support,Exposure,Ductility,Fire,OptionalTightness}
use Role::*;
const WIDTHS:&[(&str,usize)]=&[("en1992_anchor",26),("en1992_anchor_action",31),("en1992_bar_layer",19),("en1992_concrete_grade",8),("en1992_document",10),("en1992_fire",7),("en1992_member",57),("en1992_member_action",31),("en1992_prestress",13),("en1992_prestress_steel",11),("en1992_punching",11),("en1992_reinforcement_grade",18),("en1992_stirrups",8)];
pub(super)fn extent(limits:SqliteDatabaseLimits)->Result<()>{if En1992Snapshot::SQLITE_SCHEMA.len()>limits.max_schema_bytes||limits.max_tables<WIDTHS.len()||WIDTHS.iter().any(|(_,width)|*width>limits.max_columns){return Err(ValueError::new(ValueRefusalKind::WorkLimit,"EN1992 authored schema extent exceeds caller limits"))}Ok(())}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"EN1992 semantic extent overflow"))}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,table:&str,bytes:usize,n:&mut N<'_>)->Result<()>{n.step()?;if !WIDTHS.iter().any(|(name,_)|*name==table){return Err(invalid("EN1992 census table is not authored"))}let rows=add(self.rows,1)?;let bytes=add(self.bytes,bytes)?;if rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"EN1992 semantic row limit exceeded"))}if bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"EN1992 semantic value limit exceeded"))}self.rows=rows;self.bytes=bytes;Ok(())}
}
fn shape(record:&R,count:u16)->Result<()>{if record.fields.keys().copied().eq(0..count){Ok(())}else{Err(invalid("EN1992 native record field map differs"))}}
fn field(record:&R,id:u16)->Result<&F>{record.fields.get(&id).ok_or_else(||invalid("EN1992 required native role is absent"))}
fn record(value:&F)->Result<&R>{match value{F::Record(value)=>Ok(value),_=>Err(invalid("EN1992 native role requires record"))}}
fn items(value:&F)->Result<&[F]>{match value{F::List(value)=>Ok(value),_=>Err(invalid("EN1992 native role requires record list"))}}
fn scalar<T:DslField>(value:&F,n:&mut N<'_>)->Result<T>{n.scoped_stage(|n|{n.begin_stage(0)?;T::from_value_controlled(value,n)})}
fn cost(value:&F,role:Role,n:&mut N<'_>)->Result<usize>{match role{
 Text=>{n.step()?;match value{F::Text(value)=>Ok(value.len()),_=>Err(invalid("EN1992 native role requires text"))}},
 Float=>{let value=scalar::<f64>(value,n)?;Ok(if value.is_nan(){11}else if value.is_infinite(){32}else{22})},
 UInt=>{let _=scalar::<u32>(value,n)?;Ok(8)},Bool=>{let _=scalar::<bool>(value,n)?;Ok(8)},
 Annex=>Ok(annex(scalar::<AnnexChoice>(value,n)?).len()),Member=>Ok(member_kind(scalar::<MemberKind>(value,n)?).len()),Support=>Ok(support(scalar::<SupportCondition>(value,n)?).len()),Exposure=>Ok(exposure(scalar::<ExposureClass>(value,n)?).len()),Ductility=>Ok(ductility(scalar::<DuctilityClass>(value,n)?).len()),Fire=>Ok(fire_rating(scalar::<FireRating>(value,n)?).len()),
 OptionalTightness=>if matches!(value,F::Absent){n.step()?;Ok(0)}else{Ok(tightness(scalar::<TightnessClass>(value,n)?).len())}
}}
fn fields(record:&R,roles:&[(u16,Role)],n:&mut N<'_>)->Result<usize>{let mut bytes=0;for(id,role)in roles{bytes=add(bytes,cost(field(record,*id)?,*role,n)?)?;}Ok(bytes)}
fn row(value:&F,count:u16,table:&str,base:usize,roles:&[(u16,Role)],c:&mut Census,n:&mut N<'_>)->Result<()>{let value=record(value)?;shape(value,count)?;c.row(table,add(base,fields(value,roles,n)?)?,n)}
fn collection(value:&F,count:u16,table:&str,roles:&[(u16,Role)],c:&mut Census,n:&mut N<'_>)->Result<()>{for value in items(value)?{row(value,count,table,24,roles,c,n)?;}Ok(())}
fn optional(value:&F,count:u16,table:&str,roles:&[(u16,Role)],c:&mut Census,n:&mut N<'_>)->Result<()>{if matches!(value,F::Absent){n.step()?;Ok(())}else{row(value,count,table,8,roles,c,n)}}
const ACTION:&[(u16,Role)]=&[(0,Text),(1,Text),(2,Text),(3,Text),(4,Float),(5,Float),(6,Float),(7,Float),(8,Float),(9,Float),(10,Float),(11,Float)];
pub(super)fn admit_record(source:&R,limits:SqliteDatabaseLimits,n:&mut N<'_>)->Result<()>{
 extent(limits)?;n.scoped_stage(|n|{
  n.begin_stage(0)?;shape(source,10)?;let mut c=Census{limits,rows:0,bytes:0};
  c.row("en1992_document",add(8,fields(source,&[(0,Annex),(1,Text),(2,Float),(3,Float),(4,Text)],n)?)?,n)?;
  collection(field(source,5)?,3,"en1992_concrete_grade",&[(0,Text),(1,Text),(2,Float)],&mut c,n)?;
  collection(field(source,6)?,7,"en1992_reinforcement_grade",&[(0,Text),(1,Text),(2,Float),(3,Float),(4,Ductility),(5,Float),(6,Float)],&mut c,n)?;
  collection(field(source,7)?,4,"en1992_prestress_steel",&[(0,Text),(1,Text),(2,Float),(3,Float)],&mut c,n)?;
  for value in items(field(source,8)?)?{
   let value=record(value)?;shape(value,32)?;
   c.row("en1992_member",add(24,fields(value,&[(0,Text),(1,Text),(2,Text),(3,Member),(4,Text),(5,Text),(6,Text),(7,Exposure),(8,Float),(9,Float),(10,Float),(11,Float),(12,Float),(13,Support),(14,Float),(21,Bool),(22,Float),(23,Bool),(24,OptionalTightness),(25,Float),(26,Float),(27,Float),(28,Float),(29,Float),(30,Float),(31,Float)],n)?)?,n)?;
   collection(field(value,15)?,8,"en1992_bar_layer",&[(0,Text),(1,Float),(2,UInt),(3,Text),(4,Float),(5,Float),(6,Text),(7,Float)],&mut c,n)?;
   optional(field(value,16)?,3,"en1992_stirrups",&[(0,Float),(1,Float),(2,UInt)],&mut c,n)?;
   optional(field(value,17)?,4,"en1992_punching",&[(0,Float),(1,Float),(2,Text),(3,Float)],&mut c,n)?;
   optional(field(value,18)?,4,"en1992_prestress",&[(0,Float),(1,Float),(2,Float),(3,Float)],&mut c,n)?;
   optional(field(value,19)?,4,"en1992_fire",&[(0,Fire),(1,Float),(2,Text),(3,Text)],&mut c,n)?;
   collection(field(value,20)?,12,"en1992_member_action",ACTION,&mut c,n)?;
  }
  for value in items(field(source,9)?)?{
   let value=record(value)?;shape(value,10)?;c.row("en1992_anchor",add(24,fields(value,&[(0,Text),(1,Float),(2,Bool),(3,Float),(4,Float),(5,Float),(6,Float),(7,Float),(8,Float)],n)?)?,n)?;
   collection(field(value,9)?,12,"en1992_anchor_action",ACTION,&mut c,n)?;
  }
  n.checkpoint()
 })
}
