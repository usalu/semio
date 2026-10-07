fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::{artifact::RowIndex,transfer::reserve};
 use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;
 let document=database.table("semio_audio_document")?.single_row()?;identity(document,4)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio audio document identifier"))}
 let sample_rate=u32::try_from(document.integer(2)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;
 let format=match document.text(3)?{"pcm8"=>SemioAudioFormat::Pcm8,"pcm16"=>SemioAudioFormat::Pcm16,"pcm24"=>SemioAudioFormat::Pcm24,"pcm32"=>SemioAudioFormat::Pcm32,"f32"=>SemioAudioFormat::Float32,"f64"=>SemioAudioFormat::Float64,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio audio sample format"))};
 let channels=RowIndex::new(database,"semio_audio_channel",3,&[],control,"invalid Semio audio channel owner or identifier")?;
 let samples=RowIndex::new(database,"semio_audio_sample",5,&[],control,"invalid Semio sample channel or identifier")?;
 let tags=RowIndex::new(database,"semio_audio_tag",5,&[],control,"invalid Semio audio tag owner or identifier")?;
 let order=channels.ordered(2,control,"Semio audio channel ordinals must be contiguous")?;let tag_order=tags.ordered(2,control,"Semio audio tag ordinals must be contiguous")?;
 for(count,&index)in channels.indices().iter().enumerate(){if channels.row(index)?.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio audio channel owner or identifier"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,channels.len())?;}
 for(count,&index)in samples.indices().iter().enumerate(){let row=samples.row(index)?;if channels.get(row.integer(1)?,control)?.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio sample channel or identifier"))}row.integer(2)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,samples.len())?;}
 let grouped=samples.grouped_by(2,control,"Semio audio sample ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 let mut snapshot=Owned::new(Self{schema:String::new(),sample_rate,format,channels:Vec::new(),tags:Vec::new()});snapshot.get_mut().channels=reserve(order.len(),control)?;snapshot.get_mut().tags=reserve(tag_order.len(),control)?;
 let mut completed=0;
 for index in order{
  let row=channels.row(index)?;let range=samples.range_by(&grouped,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
  let mut channel=Owned::new(SemioAudioChannel{samples:Vec::new()});channel.get_mut().samples=reserve(range.len(),control)?;
  for position in range{
   let row=samples.row(grouped[position])?;let bits=u32::try_from(row.integer(4)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let value=f32::from_bits(bits);
   if value.is_nan(){if row.values[3]!=SqliteValue::Null{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"NaN Semio sample must have NULL numeric value"))}}else if row.real(3)?!=f64::from(value){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio sample value disagrees with IEEE754 bits"))}
   channel.get_mut().samples.push(value);completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
  }
  snapshot.get_mut().channels.push(channel.take());
 }
 for index in tag_order{
  let row=tags.row(index)?;if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio audio tag owner or identifier"))}
  let mut tag=Owned::new(SemioAudioTag{key:String::new(),value:String::new()});tag.get_mut().key=reconstruct_text(control,row.text(3)?)?;tag.get_mut().value=reconstruct_text(control,row.text(4)?)?;snapshot.get_mut().tags.push(tag.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
 }
 snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(snapshot.take())
}
