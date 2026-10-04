//! 🧮️ Unmounted replacement facets for shared provider concrete allocation frontiers.

fn copy_text(text:&str,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<String,ValueError>{
 super::transfer::copy_text(text,phase,control)
}
fn copy_blob(blob:&[u8],control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<Vec<u8>,ValueError>{
 control.allocation_stage(phase,|remaining,progress|{
  let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|progress(event.completed,event.total);
  let mut native=semio_framework_value::NativeEncodeControl::new(remaining,&mut callback);let result=native.copy_bytes(blob);(result,native.owned_bytes())
 })?
}

impl<'c,'p> Projection<'c,'p>{
 /// 🏛️ Constructs each authored table through the existing controlled schema authority.
 pub fn new(sql:&str,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{
  let database=super::transfer::schema_controlled(sql,SqliteSnapshotPhase::ProjectSnapshot,control)?;
  Ok(Self{database,control,rows:0,bytes:0})
 }
 /// 🔗️ Admits actual cell and replacement row backing before committing a domain entity.
 pub fn insert_key(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),ValueError>{
  self.database.table(table)?;let rows=self.rows.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"artifact row count overflow"))?;self.control.check_rows(rows)?;
  let mut bytes=self.bytes.checked_add(8).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"artifact value byte count overflow"))?;
  for cell in cells{bytes=bytes.checked_add(cell.bytes()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"artifact value byte count overflow"))?;self.control.check_value_bytes(bytes)?;if match cell{Cell::Real(value)=>value.is_nan(),Cell::Float32(value)=>value.is_nan(),_=>false}{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"NaN requires an authored IEEE scalar companion"));}}
  self.control.check_value_bytes(bytes)?;self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,rows,0)?;
  let count=cells.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"artifact cell backing overflow"))?;
  let mut values=super::transfer::reserve(count,self.control)?;values.push(SqliteValue::Integer(key));for cell in cells{values.push(cell.owned(self.control)?);}
  let table=self.database.table_mut(table)?;super::transfer::grow(&mut table.rows,SqliteSnapshotPhase::ProjectSnapshot,self.control)?;table.rows.push(SqliteRow{rowid:key,values});
  self.rows=rows;self.bytes=bytes;Ok(())
 }
}

fn float_cells<'a>(cells:&[Cell<'a>],columns:&[FloatColumn],control:&mut SqliteSnapshotControl<'_>)->Result<Vec<Cell<'a>>,ValueError>{
 let phase=SqliteSnapshotPhase::ProjectSnapshot;control.checkpoint(phase,0,columns.len())?;
 let count=cells.len().checked_add(columns.len().checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"IEEE cell backing overflow"))?).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"IEEE cell backing overflow"))?;
 let mut indices=super::transfer::reserve(columns.len(),control)?;
 for column in columns{let index=column.index().checked_sub(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"IEEE column cannot be a row identity"))?;if index>=cells.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"missing authored IEEE scalar field"));}indices.push(index);}
 super::transfer::heap_sort(&mut indices,phase,control,|a,b,_|Ok(a.cmp(b)))?;
 if indices.windows(2).any(|pair|pair[0]==pair[1]){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate authored IEEE scalar column"));}
 let mut result=super::transfer::reserve(count,control)?;result.extend_from_slice(cells);
 for(count,column)in columns.iter().enumerate(){let index=column.index()-1;let cell=cells[index];
  let(value,bits)=match(column,cell){(_,Cell::Null)=>{result.extend([Cell::Null,Cell::Null]);continue;},(FloatColumn::Binary64(_),Cell::Real(value))=>(value,value.to_bits()as i64),(FloatColumn::Binary32(_),Cell::Float32(value))=>(f64::from(value),i64::from(value.to_bits())),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"native IEEE width differs from authored scalar field"))};
  result[index]=if value.is_nan(){Cell::Null}else{Cell::Real(value)};result.extend([Cell::Integer(bits),Cell::Text(numeric_class(value))]);if(count+1)%256==0{control.checkpoint(phase,count+1,columns.len())?;}
 }Ok(result)
}
/// 🧮️ Pays the actual expanded scalar cells before committing authored IEEE companions.
pub fn insert_ieee754(projection:&mut Projection<'_,'_>,table:&str,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<i64,ValueError>{
 float_cell_limit(projection,cells.len(),columns.len())?;let cells=float_cells(cells,columns,projection.control)?;projection.insert(table,&cells)
}
/// 🔗️ Pays expanded companion cells before committing an explicitly keyed entity.
pub fn insert_key_ieee754(projection:&mut Projection<'_,'_>,table:&str,key:i64,cells:&[Cell<'_>],columns:&[FloatColumn])->Result<(),ValueError>{
 float_cell_limit(projection,cells.len(),columns.len())?;let cells=float_cells(cells,columns,projection.control)?;projection.insert_key(table,key,&cells)
}

/// 🔢️ Admits a concrete borrowed-row Vec and orders it through existing in-place sorting.
pub fn ordered_row_refs<'a>(table:&'a super::SqliteTable,ordinal:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
 let phase=SqliteSnapshotPhase::ReconstructSnapshot;let total=table.rows.len();control.check_rows(total)?;control.checkpoint(phase,0,total)?;let mut rows=super::transfer::reserve(total,control)?;
 for(count,row)in table.rows.iter().enumerate(){let index=usize::try_from(row.integer(ordinal)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;if index>=total{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"relationship ordinals must be contiguous and unique"));}rows.push(row);if(count+1)%256==0{control.checkpoint(phase,count+1,total)?;}}
 super::transfer::heap_sort(&mut rows,phase,control,|a,b,_|Ok(a.integer(ordinal)?.cmp(&b.integer(ordinal)?)))?;
 for(expected,row)in rows.iter().enumerate(){if row.integer(ordinal)?!=expected as i64{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"relationship ordinals must be contiguous and unique"));}if(expected+1)%256==0{control.checkpoint(phase,expected+1,total)?;}}
 control.checkpoint(phase,total,total)?;Ok(rows)
}
