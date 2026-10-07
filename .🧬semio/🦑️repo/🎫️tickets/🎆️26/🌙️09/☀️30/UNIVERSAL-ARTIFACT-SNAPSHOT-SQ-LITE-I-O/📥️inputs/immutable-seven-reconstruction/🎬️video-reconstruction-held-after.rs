fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::{artifact::RowIndex,transfer::reserve};
 use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;
 let document=database.table("semio_video_document")?.single_row()?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio video document identifier"))}
 let streams=RowIndex::new(database,"semio_video_stream",9,&[],control,"invalid Semio video stream ownership or identity")?;
 let samples=RowIndex::new(database,"semio_video_sample",6,&[],control,"invalid Semio video sample ownership or identity")?;
 let order=streams.ordered(2,control,"Semio video stream ordinals must be contiguous")?;
 for(count,&index)in streams.indices().iter().enumerate(){if streams.row(index)?.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio video stream ownership or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,streams.len())?;}
 for(count,&index)in samples.indices().iter().enumerate(){let row=samples.row(index)?;if streams.get(row.integer(1)?,control)?.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio video sample ownership or identity"))}row.integer(2)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,samples.len())?;}
 let grouped=samples.grouped_by(2,control,"Semio video sample ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 let mut snapshot=Owned::new(Self{schema:String::new(),streams:Vec::new()});snapshot.get_mut().streams=reserve(order.len(),control)?;let mut completed=0;
 for index in order{
  let row=streams.row(index)?;let kind=match row.text(3)?{"video"=>SemioVideoStreamKind::Video,"audio"=>SemioVideoStreamKind::Audio,"subtitle"=>SemioVideoStreamKind::Subtitle,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio video stream kind"))};
  let width=u32::try_from(row.integer(5)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let height=u32::try_from(row.integer(6)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let rate=SemioRational{num:row.integer(7)?,den:row.integer(8)?};
  let range=samples.range_by(&grouped,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
  let mut stream=Owned::new(SemioVideoStream{kind,codec:String::new(),width,height,rate,samples:Vec::new()});stream.get_mut().samples=reserve(range.len(),control)?;stream.get_mut().codec=reconstruct_text(control,row.text(4)?)?;
  for position in range{
   let row=samples.row(grouped[position])?;let word=row.text(3)?;
   if word.len()>20||word.is_empty()||!word.bytes().all(|byte|byte.is_ascii_digit())||(word.len()>1&&word.starts_with('0')){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"noncanonical Semio video timestamp"))}
   let pts=word.parse::<u64>().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;
   let key=match row.integer(4)?{0=>false,1=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio video key-frame flag"))};
   let mut sample=Owned::new(SemioVideoSample{pts,key,data:Vec::new()});sample.get_mut().data=reconstruct_blob(control,row.blob(5)?)?;stream.get_mut().samples.push(sample.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
  }
  snapshot.get_mut().streams.push(stream.take());
 }
 snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(snapshot.take())
}
