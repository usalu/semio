//! 🧮️ Exact authored FEM cells shared by owned SQL rows and borrowed Native admission.
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,FloatColumn,Projection,insert_ieee754,insert_key_ieee754}};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn refused(kind:ValueRefusalKind,message:&str)->ValueError{ValueError::new(kind,message)}
enum Rows<'c,'p>{Owned(Projection<'c,'p>),Forecast{control:&'c mut SqliteSnapshotControl<'p>,count:usize,bytes:usize,phase:SqliteSnapshotPhase}}
pub(crate) struct RowWriter<'c,'p>{rows:Rows<'c,'p>}
impl<'c,'p> RowWriter<'c,'p>{
 pub(crate) fn new(sql:&str,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{Ok(Self{rows:Rows::Owned(Projection::new(sql,control)?)})}
 pub(crate) fn forecast(control:&'c mut SqliteSnapshotControl<'p>,phase:SqliteSnapshotPhase)->Result<Self,ValueError>{control.checkpoint(phase,0,0)?;Ok(Self{rows:Rows::Forecast{control,count:0,bytes:0,phase}})}
 pub(crate) fn insert(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>{match &mut self.rows{Rows::Owned(inner)=>inner.insert(table,cells),Rows::Forecast{control,count,bytes,phase}=>admit_row(control,count,bytes,*phase,cells,&[])}}
 pub(crate) fn insert_float(&mut self,table:&str,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{match &mut self.rows{Rows::Owned(inner)=>insert_ieee754(inner,table,cells,columns),Rows::Forecast{control,count,bytes,phase}=>admit_row(control,count,bytes,*phase,cells,columns)}}
 pub(crate) fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<(),ValueError>{match &mut self.rows{Rows::Owned(inner)=>insert_key_ieee754(inner,table,key,cells,columns),Rows::Forecast{control,count,bytes,phase}=>admit_row(control,count,bytes,*phase,cells,columns).map(|_|())}}
 pub(crate) fn checkpoint_total(&mut self,total:usize)->Result<(),ValueError>{match &mut self.rows{Rows::Owned(inner)=>inner.checkpoint_total(total),Rows::Forecast{control,count,phase,..}=>control.checkpoint(*phase,*count,total)}}
 pub(crate) fn finish(self)->Result<SqliteDatabase,ValueError>{match self.rows{Rows::Owned(inner)=>inner.finish(),Rows::Forecast{..}=>Err(refused(ValueRefusalKind::InvalidValue,"FEM borrowed admission owns no database"))}}
 pub(crate) fn finish_forecast(self,total:usize)->Result<(),ValueError>{match self.rows{Rows::Forecast{control,count,phase,..}=>{if count!=total{return Err(refused(ValueRefusalKind::InvalidValue,"FEM borrowed row census differs"));}control.checkpoint(phase,count,total)},Rows::Owned(_)=>Err(refused(ValueRefusalKind::InvalidValue,"FEM owned rows are not a borrowed admission"))}}
}
fn admit_row(control:&mut SqliteSnapshotControl<'_>,count:&mut usize,bytes:&mut usize,phase:SqliteSnapshotPhase,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{
 let mut row_bytes=8usize;
 for cell in cells{let size=match cell{Cell::Null=>0,Cell::Integer(_)|Cell::Real(_)|Cell::Float32(_)=>8,Cell::Text(value)=>value.len(),Cell::Blob(value)=>value.len()};row_bytes=row_bytes.checked_add(size).ok_or_else(||refused(ValueRefusalKind::OwnershipLimit,"FEM semantic cell byte count overflow"))?;}
 for(n,column)in columns.iter().enumerate(){let index=match column{FloatColumn::Binary64(index)|FloatColumn::Binary32(index)=>*index};if columns[..n].iter().any(|previous|match previous{FloatColumn::Binary64(other)|FloatColumn::Binary32(other)=>*other==index}){return Err(refused(ValueRefusalKind::InvalidValue,"FEM duplicate authored IEEE column"));}let cell=cells.get(index.checked_sub(1).ok_or_else(||refused(ValueRefusalKind::InvalidValue,"FEM IEEE column is a row identity"))?).ok_or_else(||refused(ValueRefusalKind::InvalidValue,"FEM IEEE field is missing"))?;
  let value=match(column,cell){(_,Cell::Null)=>continue,(FloatColumn::Binary64(_),Cell::Real(value))=>*value,(FloatColumn::Binary32(_),Cell::Float32(value))=>f64::from(*value),_=>return Err(refused(ValueRefusalKind::InvalidValue,"FEM native IEEE width differs"))};
  let class=if value.is_nan(){row_bytes-=8;"nan"}else if value==f64::INFINITY{"positiveInfinity"}else if value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"};row_bytes=row_bytes.checked_add(8+class.len()).ok_or_else(||refused(ValueRefusalKind::OwnershipLimit,"FEM IEEE semantic byte count overflow"))?;
 }
 let next_bytes=bytes.checked_add(row_bytes).ok_or_else(||refused(ValueRefusalKind::OwnershipLimit,"FEM semantic byte count overflow"))?;control.check_value_bytes(next_bytes)?;
 let next=count.checked_add(1).ok_or_else(||refused(ValueRefusalKind::WorkLimit,"FEM entity row count overflow"))?;control.check_rows(next)?;if next%256==0{control.checkpoint(phase,next,0)?;}*count=next;*bytes=next_bytes;i64::try_from(next).map_err(|_|refused(ValueRefusalKind::WorkLimit,"FEM entity identity exceeds i64"))
}
