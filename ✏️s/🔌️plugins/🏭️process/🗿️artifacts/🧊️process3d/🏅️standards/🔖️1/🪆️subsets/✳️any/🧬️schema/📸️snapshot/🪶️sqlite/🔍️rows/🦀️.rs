//! 🔍️ concrete paid Process3d identity, ownership and relationship indexes.
use super::{SQL,TABLES};
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteTable,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind,transfer::reserve};
use std::cmp::Ordering;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn tick(work:&mut usize,control:&mut SqliteSnapshotControl<'_>)->Result<()>{*work=work.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Process3d indexed scan overflow"))?;if *work%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,*work,0)?;}Ok(())}
fn sort<T>(values:&mut[T],control:&mut SqliteSnapshotControl<'_>,mut compare:impl FnMut(&T,&T)->Result<Ordering>)->Result<()>{
 fn sift<T>(values:&mut[T],mut root:usize,end:usize,work:&mut usize,control:&mut SqliteSnapshotControl<'_>,compare:&mut impl FnMut(&T,&T)->Result<Ordering>)->Result<()>{
  loop{let Some(left)=root.checked_mul(2).and_then(|n|n.checked_add(1)).filter(|n|*n<end)else{return Ok(())};let right=left+1;let next=if right<end&&compare(&values[left],&values[right])?.is_lt(){right}else{left};tick(work,control)?;if !compare(&values[root],&values[next])?.is_lt(){return Ok(())}values.swap(root,next);root=next;}
 }
 let mut work=0;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,values.len())?;for root in(0..values.len()/2).rev(){sift(values,root,values.len(),&mut work,control,&mut compare)?;}for end in(1..values.len()).rev(){values.swap(0,end);sift(values,0,end,&mut work,control,&mut compare)?;}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,values.len(),values.len())
}
struct Table<'a>{value:&'a SqliteTable,identities:Vec<usize>,relationships:Vec<usize>,used:Vec<u8>}
/// 🗂️ Borrows exact authored cells while paying every ownership and ordering index.
pub(super) struct Rows<'a>{tables:Vec<Table<'a>>,work:usize,used:usize,total:usize}
impl<'a> Rows<'a>{
 pub(super) fn new(database:&'a SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self>{
  store::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,SQL,SqliteSnapshotPhase::ReconstructSnapshot,control)?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let mut tables=reserve(TABLES.len(),control)?;let mut work=0;let mut total=0usize;
  for(number,(name,width))in TABLES.into_iter().enumerate(){let value=database.table(name)?;let mut identities=reserve(value.rows.len(),control)?;for(index,row)in value.rows.iter().enumerate(){tick(&mut work,control)?;if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid{return Err(invalid("Process3d row identity or width"))}identities.push(index);}sort(&mut identities,control,|a,b|Ok(value.rows[*a].rowid.cmp(&value.rows[*b].rowid)))?;for pair in identities.windows(2){tick(&mut work,control)?;if value.rows[pair[0]].rowid==value.rows[pair[1]].rowid{return Err(invalid("Process3d duplicate row identity"))}}
   let mut relationships=if number==0||number==4{Vec::new()}else{reserve(value.rows.len(),control)?};if number!=0&&number!=4{for index in 0..value.rows.len(){tick(&mut work,control)?;relationships.push(index);}sort(&mut relationships,control,|a,b|Ok(value.rows[*a].integer(1)?.cmp(&value.rows[*b].integer(1)?).then_with(||value.rows[*a].rowid.cmp(&value.rows[*b].rowid))))?;}
   let count=value.rows.len().checked_add(7).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Process3d ownership bitmap overflow"))?/8;let mut used=reserve(count,control)?;let mut initialized=0;while initialized<count{initialized=initialized.saturating_add(65536).min(count);used.resize(initialized,0);control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,initialized,count)?;}total=total.checked_add(value.rows.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Process3d row count overflow"))?;tables.push(Table{value,identities,relationships,used});
  }Ok(Self{tables,work,used:0,total})
 }
 pub(super) fn row(&mut self,table:usize,id:i64,control:&mut SqliteSnapshotControl<'_>)->Result<&'a SqliteRow>{
  let table=self.tables.get_mut(table).ok_or_else(||invalid("Process3d table identity"))?;let mut start=0;let mut end=table.identities.len();while start<end{tick(&mut self.work,control)?;let middle=start+(end-start)/2;match table.value.rows[table.identities[middle]].rowid.cmp(&id){Ordering::Less=>start=middle+1,Ordering::Greater=>end=middle,Ordering::Equal=>{let index=table.identities[middle];let mask=1u8<<(index%8);if table.used[index/8]&mask!=0{return Err(invalid("Process3d entity has multiple owners"))}table.used[index/8]|=mask;self.used+=1;return Ok(&table.value.rows[index]);}}}Err(invalid("Process3d dangling relationship"))
 }
 pub(super) fn children(&mut self,table:usize,parent:i64,ordered:bool,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>>{
  let indexed=self.tables.get(table).ok_or_else(||invalid("Process3d table identity"))?;let value=indexed.value;let relationships=&indexed.relationships;let mut first=0;let mut end=relationships.len();while first<end{tick(&mut self.work,control)?;let middle=first+(end-first)/2;if value.rows[relationships[middle]].integer(1)?<parent{first=middle+1}else{end=middle}}let start=first;end=relationships.len();while first<end{tick(&mut self.work,control)?;let middle=first+(end-first)/2;if value.rows[relationships[middle]].integer(1)?<=parent{first=middle+1}else{end=middle}}let count=first-start;if !ordered&&count>1{return Err(invalid("Process3d relationship cardinality"))}let mut children=reserve(count,control)?;
  for index in &relationships[start..first]{tick(&mut self.work,control)?;let row=&value.rows[*index];if ordered&&row.integer(2)?<0{return Err(invalid("Process3d negative relationship ordinal"))}children.push(row);}
  if ordered{sort(&mut children,control,|a,b|Ok(a.integer(2)?.cmp(&b.integer(2)?)))?;}for(index,row)in children.iter().enumerate(){tick(&mut self.work,control)?;if ordered&&row.integer(2)?!=i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Process3d relationship ordinal overflow"))?{return Err(invalid("Process3d noncontiguous or repeated ordinal"))}self.row(table,row.rowid,control)?;}Ok(children)
 }
 pub(super) fn one(&mut self,table:usize,parent:i64,control:&mut SqliteSnapshotControl<'_>)->Result<&'a SqliteRow>{let children=self.children(table,parent,false,control)?;if children.len()!=1{return Err(invalid("Process3d missing required relationship"))}Ok(children[0])}
 pub(super) fn finish(&self)->Result<()>{if self.total!=self.used{return Err(invalid("Process3d unowned entity or mismatched variant"))}Ok(())}
}
