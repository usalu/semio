//! 🏗️ Borrowed typed row admission shared by the two finite-element domains.
use std::collections::BTreeMap;
#[path="🧮️projection/🦀️.rs"]
mod projection;
pub(crate) use projection::RowWriter;
use store::sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{Cell,FloatColumn,FloatRow,Reconstruction}};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn positioned(error:ValueError)->semio_framework_diagnostic::TextError{semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1))}
pub(crate) type Entities<'a>=BTreeMap<i64,FloatRow<'a>>;
pub(crate) fn ordinal(n:usize)->Result<Cell<'static>,ValueError>{Ok(Cell::Integer(i64::try_from(n).map_err(|e|invalid(e.to_string()))?))}
pub(crate) fn decimal(mut value:usize,buffer:&mut[u8;20])->&str{let mut at=buffer.len();loop{at-=1;buffer[at]=b'0'+(value%10)as u8;value/=10;if value==0{break;}}std::str::from_utf8(&buffer[at..]).expect("decimal digits")}
pub(crate) fn add_rows(total:&mut usize,count:usize,control:&SqliteSnapshotControl<'_>)->Result<(),ValueError>{*total=total.checked_add(count).ok_or_else(||invalid("FEM row count overflow"))?;control.check_rows(*total)}
pub(crate) fn checkpoint(c:&mut SqliteSnapshotControl<'_>,n:usize,total:usize)->Result<(),ValueError>{if n%256==0{c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,n,total)?;}Ok(())}
pub(crate) fn entities<'a>(d:&'a SqliteDatabase,name:&str,count:usize,floats:&'static[FloatColumn],c:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>,ValueError>{
 let rows=&d.table(name)?.rows;let mut out=Entities::new();for(n,row)in rows.iter().enumerate(){checkpoint(c,n,rows.len())?;let r=FloatRow::new(row,floats)?;if r.rowid<=0||r.integer(0)?!=r.rowid||r.values.len()!=count||out.insert(r.rowid,r).is_some(){return Err(invalid("FEM aliased identity or authored column shape differs"));}}c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,rows.len(),rows.len())?;Ok(out)
}
pub(crate) fn groups<'a>(rows:Entities<'a>,parents:&Entities<'_>,c:&mut SqliteSnapshotControl<'_>)->Result<BTreeMap<i64,Vec<FloatRow<'a>>>,ValueError>{let total=rows.len();let mut out=BTreeMap::<i64,Vec<FloatRow<'a>>>::new();for(n,row)in rows.into_values().enumerate(){checkpoint(c,n,total)?;let parent=row.integer(1)?;if !parents.contains_key(&parent){return Err(invalid("FEM owning foreign key is dangling"));}out.entry(parent).or_default().push(row);}Ok(out)}
pub(crate) fn ordered<'a>(rows:impl IntoIterator<Item=FloatRow<'a>>,parent:i64,c:&mut SqliteSnapshotControl<'_>)->Result<Vec<FloatRow<'a>>,ValueError>{let mut out=BTreeMap::new();for(n,row)in rows.into_iter().enumerate(){checkpoint(c,n,0)?;let ordinal=usize::try_from(row.integer(2)?).map_err(|e|invalid(e.to_string()))?;if row.integer(1)?!=parent||out.insert(ordinal,row).is_some(){return Err(invalid("FEM relationship parent or ordinal differs"));}}let total=out.len();let mut values=Vec::with_capacity(total);for(n,(ordinal,row))in out.into_iter().enumerate(){checkpoint(c,n,total)?;if n!=ordinal{return Err(invalid("FEM relationship ordinals are not contiguous"));}values.push(row);}c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,total,total)?;Ok(values)}
pub(crate) fn text(row:FloatRow<'_>,index:usize,c:&mut SqliteSnapshotControl<'_>)->Result<String,ValueError>{Reconstruction::new(c)?.text(row.text(index)?)}
pub(crate) fn real(row:FloatRow<'_>,index:usize,c:&mut SqliteSnapshotControl<'_>)->Result<f64,ValueError>{Reconstruction::new(c)?.scalar()?;row.real(index)}
pub(crate) fn count(row:FloatRow<'_>,index:usize,c:&mut SqliteSnapshotControl<'_>)->Result<usize,ValueError>{let word=row.text(index)?;if word.is_empty()||word.len()>20||(word.len()>1&&word.starts_with('0'))||!word.bytes().all(|b|b.is_ascii_digit()){return Err(invalid("FEM native count is not canonical unsigned decimal"));}Reconstruction::new(c)?.scalar()?;word.parse::<usize>().map_err(|e|invalid(e.to_string()))}
pub(crate) fn points(rows:Vec<FloatRow<'_>>,parent:i64,c:&mut SqliteSnapshotControl<'_>)->Result<Vec<[f64;2]>,ValueError>{let mut values=Vec::new();for row in ordered(rows,parent,c)?{values.push([real(row,3,c)?,real(row,4,c)?]);}Ok(values)}
pub(crate) fn variants<'a>(rows:Entities<'a>,parents:&Entities<'_>,kind:&str,index:usize,c:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>,ValueError>{for(n,row)in rows.values().enumerate(){checkpoint(c,n,rows.len())?;if parents.get(&row.rowid).ok_or_else(||invalid("FEM variant has no owning entity"))?.text(index)?!=kind{return Err(invalid("FEM variant disagrees with its owner"));}}Ok(rows)}
