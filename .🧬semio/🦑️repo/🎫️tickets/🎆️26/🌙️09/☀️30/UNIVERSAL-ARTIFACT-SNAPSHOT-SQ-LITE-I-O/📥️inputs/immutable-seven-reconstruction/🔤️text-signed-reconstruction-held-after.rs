fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::{artifact::{RowIdentity,RowIndex},transfer::reserve};
 use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;
 let document=database.table("semio_text_document")?.single_row()?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio text document requires identifier 1"))}
 let runs=RowIndex::new_with_identity(database,"semio_text_run",5,&[],RowIdentity::Signed,control,"Semio text run document or identity is invalid")?;
 let marks=RowIndex::new_with_identity(database,"semio_text_mark",5,&[],RowIdentity::Signed,control,"Semio text mark run, identity or ordinal is invalid")?;
 let order=runs.ordered(2,control,"Semio text run ordinals must be contiguous and zero-based")?;
 for(count,&index)in runs.indices().iter().enumerate(){let row=runs.row(index)?;if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio text run document or identity is invalid"))}row.text(3)?;row.text(4)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,runs.len())?;}
 for(count,&index)in marks.indices().iter().enumerate(){let row=marks.row(index)?;if runs.get(row.integer(1)?,control)?.is_none()||row.integer(2)?<0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio text mark run, identity or ordinal is invalid"))}row.text(3)?;row.text(4)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,marks.len())?;}
 let grouped=marks.grouped_by(2,control,"Semio text mark ordinals must be contiguous and zero-based",|row|Ok((0,Some(row.integer(1)?))))?;
 let mut snapshot=Owned::new(Self{schema:String::new(),runs:Vec::new()});snapshot.get_mut().runs=reserve(order.len(),control)?;
 let mut completed=0;
 for index in order{
  let row=runs.row(index)?;let range=marks.range_by(&grouped,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
  let mut run=Owned::new(SemioTextRun{language:String::new(),content:String::new(),marks:Vec::new()});
  run.get_mut().marks=reserve(range.len(),control)?;run.get_mut().language=reconstruct_text(control,row.text(3)?)?;run.get_mut().content=reconstruct_text(control,row.text(4)?)?;
  for position in range{
   let row=marks.row(grouped[position])?;let kind=match row.text(3)?{"bold"=>SemioTextMarkKind::Bold,"italic"=>SemioTextMarkKind::Italic,"code"=>SemioTextMarkKind::Code,"link"=>SemioTextMarkKind::Link,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio text mark kind is invalid"))};
   let mut mark=Owned::new(SemioTextMark{kind,href:String::new()});mark.get_mut().href=reconstruct_text(control,row.text(4)?)?;run.get_mut().marks.push(mark.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
  }
  snapshot.get_mut().runs.push(run.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
 }
 snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(snapshot.take())
}
