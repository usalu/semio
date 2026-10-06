//! 🗂️ Paid borrowed row indexes preserve literal ownership and dense authored relationship order.
use super::*;
use store::sqlite_snapshot::SqliteTable;
use semio_framework_value::NativeDecodeControl;
use std::cmp::Ordering;
#[derive(Clone,Copy,PartialEq,Eq)]
enum Key{Body,List,Header}
struct Entry<'d>{row:&'d SqliteRow,used:bool}
struct Index<'d>{table:&'d SqliteTable,width:Option<usize>,key:Key,rows:Vec<Entry<'d>>}
pub(super)struct Reader<'d,'n,'p>{pub(super)database:&'d SqliteDatabase,pub(super)native:&'n mut NativeDecodeControl<'p>,indexes:Vec<Index<'d>>,used:usize}
fn compare_text(left:&str,right:&str,native:&mut NativeDecodeControl<'_>)->Result<Ordering,ValueError>{
 native.scoped_stage(|native|{native.begin_stage(0)?;let left=left.as_bytes();let right=right.as_bytes();let mut offset=0;
 while offset<left.len().min(right.len()){native.checkpoint()?;let end=offset.saturating_add(65_536).min(left.len().min(right.len()));let order=left[offset..end].cmp(&right[offset..end]);native.advance(end-offset)?;if order!=Ordering::Equal{return Ok(order)}offset=end;}
 Ok(left.len().cmp(&right.len()))})
}
fn sort<T>(values:&mut[T],native:&mut NativeDecodeControl<'_>,mut compare:impl FnMut(&T,&T,&mut NativeDecodeControl<'_>)->Result<Ordering,ValueError>)->Result<(),ValueError>{
 native.scoped_stage(|native|{native.begin_stage(0)?;
  for start in(0..values.len()/2).rev(){sift(values,start,values.len(),native,&mut compare)?;}
  for end in(1..values.len()).rev(){values.swap(0,end);native.step()?;sift(values,0,end,native,&mut compare)?;}
  native.checkpoint()
 })
}
fn sift<T>(values:&mut[T],mut root:usize,end:usize,native:&mut NativeDecodeControl<'_>,compare:&mut impl FnMut(&T,&T,&mut NativeDecodeControl<'_>)->Result<Ordering,ValueError>)->Result<(),ValueError>{
 loop{let Some(mut child)=root.checked_mul(2).and_then(|value|value.checked_add(1)).filter(|child|*child<end)else{return Ok(())};native.step()?;
  if child+1<end&&compare(&values[child],&values[child+1],native)?==Ordering::Less{child+=1;}
  if compare(&values[root],&values[child],native)?!=Ordering::Less{return Ok(())}
  values.swap(root,child);root=child;
 }
}
fn row_order(key:Key,left:&SqliteRow,right:&SqliteRow,native:&mut NativeDecodeControl<'_>)->Result<Ordering,ValueError>{
 native.step()?;match key{Key::Body=>Ok(left.rowid.cmp(&right.rowid)),Key::List=>Ok(left.integer(1)?.cmp(&right.integer(1)?).then(left.integer(2)?.cmp(&right.integer(2)?))),Key::Header=>Ok(compare_text(left.text(2)?,right.text(2)?,native)?.then(left.integer(3)?.cmp(&right.integer(3)?)))}
}
impl<'d,'n,'p>Reader<'d,'n,'p>{
 pub(super)fn new(database:&'d SqliteDatabase,native:&'n mut NativeDecodeControl<'p>)->Result<Self,ValueError>{
  let mut indexes=native.allocate_vec(database.tables.len())?;
  native.scoped_stage(|native|{native.begin_stage(database.tables.len())?;for table in &database.tables{native.step()?;indexes.push(Index{table,width:None,key:Key::Body,rows:Vec::new()});}Ok::<_,ValueError>(())})?;
  sort(&mut indexes,native,|left,right,native|compare_text(&left.table.name,&right.table.name,native))?;
  if indexes.windows(2).any(|pair|pair[0].table.name==pair[1].table.name){return Err(invalid("Architect table identity is repeated"))}
  Ok(Self{database,native,indexes,used:0})
 }
 fn locate(&mut self,table:&str)->Result<usize,ValueError>{
  self.native.scoped_stage(|native|{native.begin_stage(0)?;let mut first=0;let mut end=self.indexes.len();while first<end{native.step()?;let middle=first+(end-first)/2;match compare_text(&self.indexes[middle].table.name,table,native)?{Ordering::Less=>first=middle+1,Ordering::Greater=>end=middle,Ordering::Equal=>return Ok(middle)}}Err(invalid("Architect authored table is absent"))})
 }
 fn index(&mut self,table:&'static str,width:usize,key:Key)->Result<usize,ValueError>{
  let at=self.locate(table)?;
  if let Some(existing)=self.indexes[at].width{if existing!=width||self.indexes[at].key!=key{return Err(invalid("Architect table has conflicting typed roles"))}return Ok(at)}
  let source=&self.indexes[at].table.rows;let mut rows=self.native.allocate_vec(source.len())?;
  self.native.scoped_stage(|native|{native.begin_stage(source.len())?;for row in source{native.step()?;if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid{return Err(invalid("Architect row shape or identity differs"))}rows.push(Entry{row,used:false});}Ok::<_,ValueError>(())})?;
  sort(&mut rows,self.native,|left,right,native|{native.step()?;Ok(left.row.rowid.cmp(&right.row.rowid))})?;
  if rows.windows(2).any(|pair|pair[0].row.rowid==pair[1].row.rowid){return Err(invalid("Architect row identity is repeated"))}
  if key!=Key::Body{
   sort(&mut rows,self.native,|left,right,native|row_order(key,left.row,right.row,native))?;
   self.native.scoped_stage(|native|{native.begin_stage(rows.len())?;let mut previous:Option<&SqliteRow>=None;for entry in &rows{native.step()?;let row=entry.row;let column=if key==Key::List{2}else{3};let ordinal=usize::try_from(row.integer(column)?).map_err(|error|invalid(error.to_string()))?;
    if key==Key::Header&&row.integer(1)?!=1{return Err(invalid("Architect header document owner differs"))}
    let same=match previous{None=>false,Some(previous)=>if key==Key::List{previous.integer(1)?==row.integer(1)?}else{compare_text(previous.text(2)?,row.text(2)?,native)?==Ordering::Equal}};
    let expected=if same{usize::try_from(previous.unwrap().integer(column)?).ok().and_then(|value|value.checked_add(1)).ok_or_else(||invalid("Architect ordinal extent overflow"))?}else{0};
    if ordinal!=expected{return Err(invalid("Architect authored ordinals are not dense"))}previous=Some(row);
   }Ok::<_,ValueError>(())})?;
  }
  self.indexes[at].rows=rows;self.indexes[at].width=Some(width);self.indexes[at].key=key;Ok(at)
 }
 fn mark(&mut self,at:usize,position:usize)->Result<&'d SqliteRow,ValueError>{
  let entry=&mut self.indexes[at].rows[position];if entry.used{return Err(invalid("Architect entity is multiply owned"))}
  self.native.checkpoint()?;entry.used=true;self.used=self.used.checked_add(1).ok_or_else(||invalid("Architect consumed row count overflow"))?;Ok(entry.row)
 }
 pub(super)fn use_row(&mut self,table:&'static str,row:&SqliteRow)->Result<(),ValueError>{
  let at=self.index(table,row.values.len(),Key::Body)?;let position=self.indexes[at].rows.binary_search_by_key(&row.rowid,|entry|entry.row.rowid).map_err(|_|invalid("Architect consumed body is absent"))?;if !std::ptr::eq(self.indexes[at].rows[position].row,row){return Err(invalid("Architect consumed body has another owner"))}self.mark(at,position).map(|_|())
 }
 pub(super)fn singleton(&mut self,table:&'static str,width:usize)->Result<&'d SqliteRow,ValueError>{
  let at=self.index(table,width,Key::Body)?;if self.indexes[at].rows.len()!=1||self.indexes[at].rows[0].row.rowid!=1{return Err(invalid("Architect singleton shape differs"))}self.mark(at,0)
 }
 pub(super)fn optional_body(&mut self,table:&'static str,parent:i64,width:usize)->Result<Option<&'d SqliteRow>,ValueError>{
  let at=self.index(table,width,Key::Body)?;match self.indexes[at].rows.binary_search_by_key(&parent,|entry|entry.row.rowid){Ok(position)=>self.mark(at,position).map(Some),Err(_)=>Ok(None)}
 }
 pub(super)fn body(&mut self,table:&'static str,parent:i64,width:usize)->Result<&'d SqliteRow,ValueError>{self.optional_body(table,parent,width)?.ok_or_else(||invalid("Architect typed body is absent"))}
 pub(super)fn rows(&mut self,table:&'static str,parent:i64,width:usize)->Result<Vec<&'d SqliteRow>,ValueError>{
  let at=self.index(table,width,Key::List)?;let rows=&self.indexes[at].rows;let first=rows.partition_point(|entry|entry.row.integer(1).is_ok_and(|value|value<parent));let end=first+rows[first..].partition_point(|entry|entry.row.integer(1).is_ok_and(|value|value==parent));let mut result=self.native.allocate_vec(end-first)?;
  self.native.scoped_stage(|native|{native.begin_stage(end-first)?;for _ in first..end{native.step()?;}Ok::<_,ValueError>(())})?;
  for position in first..end{result.push(self.mark(at,position)?);}Ok(result)
 }
 pub(super)fn list<T:SqlField>(&mut self,table:&'static str,parent:i64)->Result<Vec<T>,ValueError>{
  let rows=self.rows(table,parent,T::WIDTH+3)?;let mut values=self.native.allocate_vec(rows.len())?;self.native.scoped_stage(|native|{native.begin_stage(rows.len())?;for row in rows{native.step()?;let mut index=3;values.push(T::read(row,&mut index,native)?);}native.checkpoint()})?;Ok(values)
 }
 pub(super)fn register(&mut self,name:&'static str)->Result<Vec<&'d SqliteRow>,ValueError>{
  let at=self.index("architect_entity_header",17,Key::Header)?;let rows=&self.indexes[at].rows;
  let(first,end)=self.native.scoped_stage(|native|{native.begin_stage(0)?;let mut first=0;let mut end=rows.len();while first<end{native.step()?;let middle=first+(end-first)/2;if compare_text(rows[middle].row.text(2)?,name,native)?==Ordering::Less{first=middle+1}else{end=middle}}let start=first;let mut end=rows.len();while first<end{native.step()?;let middle=first+(end-first)/2;if compare_text(rows[middle].row.text(2)?,name,native)?==Ordering::Greater{end=middle}else{first=middle+1}}Ok::<_,ValueError>((start,first))})?;
  let mut result=self.native.allocate_vec(end-first)?;for position in first..end{result.push(self.mark(at,position)?);}Ok(result)
 }
 pub(super)fn header(&mut self,row:&SqliteRow)->Result<EntityHeader,ValueError>{
  let mut i=4;let n=&mut*self.native;let id=SqlField::read(row,&mut i,n)?;let name=SqlField::read(row,&mut i,n)?;let description=SqlField::read(row,&mut i,n)?;let status=SqlField::read(row,&mut i,n)?;let priority=SqlField::read(row,&mut i,n)?;let owner_id=SqlField::read(row,&mut i,n)?;let authority_id=SqlField::read(row,&mut i,n)?;let timestamps=read_timestamp(row,&mut i,n)?;
  Ok(EntityHeader{id,name,description,status,priority,timestamps,ownership:Ownership{owner_id,authority_id,consultant_ids:self.list("architect_entity_consultant",row.rowid)?,participant_ids:self.list("architect_entity_participant",row.rowid)?},tags:self.list("architect_entity_tag",row.rowid)?,notes:self.list("architect_entity_note",row.rowid)?})
 }
 pub(super)fn finish(self)->Result<(),ValueError>{
  let mut total=0usize;self.native.scoped_stage(|native|{native.begin_stage(self.database.tables.len())?;for table in &self.database.tables{native.step()?;total=total.checked_add(table.rows.len()).ok_or_else(||invalid("Architect row extent overflow"))?;}native.checkpoint()})?;
  if total!=self.used{return Err(invalid("Architect contains unowned entities"))}Ok(())
 }
}
