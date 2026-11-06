//! 🧮️ Exact authored FEM cells shared by owned SQL rows and borrowed Native admission.
use semio_framework_io_sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,FloatColumn,Projection,insert_ieee754,insert_key_ieee754}};
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
fn refused(kind:ValueRefusalKind,message:&str)->ValueError{ValueError::new(kind,message)}
/// 🧱️ FEM table traversal writes through either cold or original paid row sinks.
pub trait RowSink{
 fn work(&mut self)->Result<(),ValueError>;
 fn insert(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>;
 fn insert_float(&mut self,table:&str,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>;
 fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<(),ValueError>;
 fn checkpoint_total(&mut self,total:usize)->Result<(),ValueError>;
}
/// 🧮️ Fixed table census reuses the authored traversal without allocating a database.
pub struct TableCensus<'a>{names:&'a [&'a str],counts:&'a mut [usize],action:&'a mut dyn FnMut()->Result<(),ValueError>}
impl<'a> TableCensus<'a>{
 /// 🏁️ Binds exact original table slots and a caller-owned action port.
 pub fn new(names:&'a [&'a str],counts:&'a mut [usize],action:&'a mut dyn FnMut()->Result<(),ValueError>)->Result<Self,ValueError>{if names.len()!=counts.len(){return Err(refused(ValueRefusalKind::InvariantViolated,"FEM table census requires empty declared slots"))}for count in counts.iter(){action()?;if *count!=0{return Err(refused(ValueRefusalKind::InvariantViolated,"FEM table census requires empty declared slots"))}}Ok(Self{names,counts,action})}
 fn row(&mut self,table:&str)->Result<i64,ValueError>{
  let mut selected=None;for(index,name)in self.names.iter().enumerate(){(self.action)()?;if *name==table{selected=Some(index);break}}
  let index=selected.ok_or_else(||refused(ValueRefusalKind::InvalidValue,"FEM row names an undeclared table"))?;
  (self.action)()?;let next=self.counts[index].checked_add(1).ok_or_else(||refused(ValueRefusalKind::WorkLimit,"FEM table row count overflow"))?;let key=i64::try_from(next).map_err(|_|refused(ValueRefusalKind::WorkLimit,"FEM table row identity overflow"))?;self.counts[index]=next;Ok(key)
 }
}
impl RowSink for TableCensus<'_>{
 fn work(&mut self)->Result<(),ValueError>{(self.action)()}
 fn insert(&mut self,table:&str,_:&[Cell<'_>])->Result<i64,ValueError>{self.row(table)}
 fn insert_float(&mut self,table:&str,_:&[Cell<'_>],_:&[FloatColumn])->Result<i64,ValueError>{self.row(table)}
 fn insert_key_float(&mut self,table:&str,_:i64,_:&[Cell<'_>],_:&[FloatColumn])->Result<(),ValueError>{self.row(table).map(|_|())}
 fn checkpoint_total(&mut self,_:usize)->Result<(),ValueError>{(self.action)()}
}
enum Rows<'c,'p,'a>{Owned(Projection<'c,'p>),Forecast{control:&'c mut SqliteSnapshotControl<'p>,count:usize,bytes:usize,phase:SqliteSnapshotPhase,action:Option<&'a mut dyn FnMut()->Result<(),ValueError>>}}
pub struct RowWriter<'c,'p,'a>{rows:Rows<'c,'p,'a>}
impl<'c,'p,'a> RowWriter<'c,'p,'a>{
 pub fn new(sql:&str,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{Ok(Self{rows:Rows::Owned(Projection::new(sql,control)?)})}
 pub fn forecast(control:&'c mut SqliteSnapshotControl<'p>,phase:SqliteSnapshotPhase)->Result<Self,ValueError>{control.checkpoint(phase,0,0)?;Ok(Self{rows:Rows::Forecast{control,count:0,bytes:0,phase,action:None}})}
 /// 🎟️ Borrows the caller action port for each selected forecast row and field.
 pub fn forecast_controlled(control:&'c mut SqliteSnapshotControl<'p>,phase:SqliteSnapshotPhase,action:&'a mut dyn FnMut()->Result<(),ValueError>)->Result<Self,ValueError>{control.checkpoint(phase,0,0)?;Ok(Self{rows:Rows::Forecast{control,count:0,bytes:0,phase,action:Some(action)}})}
 /// 🧮️ Selects one authored traversal action when the original caller supplied a port.
 pub fn work(&mut self)->Result<(),ValueError>{match &mut self.rows{Rows::Owned(_)=>Ok(()),Rows::Forecast{action,..}=>turn(action)}}
 pub fn insert(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>{match &mut self.rows{Rows::Owned(inner)=>inner.insert(table,cells),Rows::Forecast{control,count,bytes,phase,action}=>admit_row(control,count,bytes,*phase,action,cells,&[])}}
 pub fn insert_float(&mut self,table:&str,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{match &mut self.rows{Rows::Owned(inner)=>insert_ieee754(inner,table,cells,columns),Rows::Forecast{control,count,bytes,phase,action}=>admit_row(control,count,bytes,*phase,action,cells,columns)}}
 pub fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<(),ValueError>{match &mut self.rows{Rows::Owned(inner)=>insert_key_ieee754(inner,table,key,cells,columns),Rows::Forecast{control,count,bytes,phase,action}=>admit_row(control,count,bytes,*phase,action,cells,columns).map(|_|())}}
 pub fn checkpoint_total(&mut self,total:usize)->Result<(),ValueError>{match &mut self.rows{Rows::Owned(inner)=>inner.checkpoint_total(total),Rows::Forecast{control,count,phase,..}=>control.checkpoint(*phase,*count,total)}}
 pub fn finish(self)->Result<SqliteDatabase,ValueError>{match self.rows{Rows::Owned(inner)=>inner.finish(),Rows::Forecast{..}=>Err(refused(ValueRefusalKind::InvalidValue,"FEM borrowed admission owns no database"))}}
 pub fn finish_forecast(self,total:usize)->Result<(),ValueError>{match self.rows{Rows::Forecast{control,count,phase,..}=>{if count!=total{return Err(refused(ValueRefusalKind::InvalidValue,"FEM borrowed row census differs"));}control.checkpoint(phase,count,total)},Rows::Owned(_)=>Err(refused(ValueRefusalKind::InvalidValue,"FEM owned rows are not a borrowed admission"))}}
}
impl RowSink for RowWriter<'_,'_,'_>{
 fn work(&mut self)->Result<(),ValueError>{Self::work(self)}
 fn insert(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,ValueError>{Self::insert(self,table,cells)}
 fn insert_float(&mut self,table:&str,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{Self::insert_float(self,table,cells,columns)}
 fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<(),ValueError>{Self::insert_key_float(self,table,key,cells,columns)}
 fn checkpoint_total(&mut self,total:usize)->Result<(),ValueError>{Self::checkpoint_total(self,total)}
}
fn turn(action:&mut Option<&mut dyn FnMut()->Result<(),ValueError>>)->Result<(),ValueError>{match action{Some(action)=>action(),None=>Ok(())}}
fn admit_row(control:&mut SqliteSnapshotControl<'_>,count:&mut usize,bytes:&mut usize,phase:SqliteSnapshotPhase,action:&mut Option<&mut dyn FnMut()->Result<(),ValueError>>,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{
 turn(action)?;let mut row_bytes=8usize;
 for cell in cells{turn(action)?;let size=match cell{Cell::Null=>0,Cell::Integer(_)|Cell::Real(_)|Cell::Float32(_)=>8,Cell::Text(value)=>value.len(),Cell::PagedText(value)=>value.text_bytes(),Cell::Blob(value)=>value.len()};row_bytes=row_bytes.checked_add(size).ok_or_else(||refused(ValueRefusalKind::OwnershipLimit,"FEM semantic cell byte count overflow"))?;}
 for(n,column)in columns.iter().enumerate(){turn(action)?;let index=match column{FloatColumn::Binary64(index)|FloatColumn::Binary32(index)=>*index};for previous in &columns[..n]{turn(action)?;if match previous{FloatColumn::Binary64(other)|FloatColumn::Binary32(other)=>*other==index}{return Err(refused(ValueRefusalKind::InvalidValue,"FEM duplicate authored IEEE column"));}}let cell=cells.get(index.checked_sub(1).ok_or_else(||refused(ValueRefusalKind::InvalidValue,"FEM IEEE column is a row identity"))?).ok_or_else(||refused(ValueRefusalKind::InvalidValue,"FEM IEEE field is missing"))?;
  let value=match(column,cell){(_,Cell::Null)=>continue,(FloatColumn::Binary64(_),Cell::Real(value))=>*value,(FloatColumn::Binary32(_),Cell::Float32(value))=>f64::from(*value),_=>return Err(refused(ValueRefusalKind::InvalidValue,"FEM native IEEE width differs"))};
  let class=if value.is_nan(){row_bytes-=8;"nan"}else if value==f64::INFINITY{"positiveInfinity"}else if value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"};row_bytes=row_bytes.checked_add(8+class.len()).ok_or_else(||refused(ValueRefusalKind::OwnershipLimit,"FEM IEEE semantic byte count overflow"))?;
 }
 let next_bytes=bytes.checked_add(row_bytes).ok_or_else(||refused(ValueRefusalKind::OwnershipLimit,"FEM semantic byte count overflow"))?;control.check_value_bytes(next_bytes)?;
 let next=count.checked_add(1).ok_or_else(||refused(ValueRefusalKind::WorkLimit,"FEM entity row count overflow"))?;control.check_rows(next)?;if next%256==0{control.checkpoint(phase,next,0)?;}let key=i64::try_from(next).map_err(|_|refused(ValueRefusalKind::WorkLimit,"FEM entity identity exceeds i64"))?;*count=next;*bytes=next_bytes;Ok(key)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
