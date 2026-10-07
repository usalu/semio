fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::{artifact::RowIndex,transfer::reserve};
 use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;
 let document=single_float_row(database,"semio_animation_document")?;identity(document,2)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio animation document identifier"))}
 let timelines=RowIndex::new(database,"semio_animation_timeline",4,float_columns("semio_animation_timeline"),control,"invalid Semio animation timeline owner or identity")?;
 let channels=RowIndex::new(database,"semio_animation_channel",7,float_columns("semio_animation_channel"),control,"invalid Semio animation relationship owner or identity")?;
 let keyframes=RowIndex::new(database,"semio_animation_keyframe",5,float_columns("semio_animation_keyframe"),control,"invalid Semio animation relationship owner or identity")?;
 let mut weights=RowIndex::new(database,"semio_animation_weight",4,float_columns("semio_animation_weight"),control,"invalid Semio animation relationship owner or identity")?;
 let mut scalars=RowIndex::new(database,"semio_animation_scalar",2,float_columns("semio_animation_scalar"),control,"duplicate Semio animation variant detail")?;
 let mut vectors=RowIndex::new(database,"semio_animation_vector3",4,float_columns("semio_animation_vector3"),control,"duplicate Semio animation variant detail")?;
 let mut quaternions=RowIndex::new(database,"semio_animation_quaternion",5,float_columns("semio_animation_quaternion"),control,"duplicate Semio animation variant detail")?;
 let order=timelines.ordered(2,control,"Semio animation timeline ordinals must be contiguous")?;
 for(count,&index)in timelines.indices().iter().enumerate(){if timelines.row(index)?.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio animation timeline owner or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,timelines.len())?;}
 for(count,&index)in channels.indices().iter().enumerate(){if timelines.get(channels.row(index)?.integer(1)?,control)?.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio animation relationship owner or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,channels.len())?;}
 for(count,&index)in keyframes.indices().iter().enumerate(){if channels.get(keyframes.row(index)?.integer(1)?,control)?.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio animation relationship owner or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,keyframes.len())?;}
 for(count,&index)in weights.indices().iter().enumerate(){if keyframes.get(weights.row(index)?.integer(1)?,control)?.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio animation relationship owner or identity"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,weights.len())?;}
 let channel_order=channels.grouped_by(2,control,"Semio animation relationship ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 let keyframe_order=keyframes.grouped_by(2,control,"Semio animation relationship ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 let weight_order=weights.grouped_by(2,control,"Semio animation relationship ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 let mut snapshot=Owned::new(Self{schema:String::new(),timelines:Vec::new()});snapshot.get_mut().timelines=reserve(order.len(),control)?;let mut completed=0;
 for index in order{
  let row=timelines.row(index)?;let range=channels.range_by(&channel_order,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
  let mut timeline=Owned::new(AnimTimeline{name:None,channels:Vec::new()});timeline.get_mut().channels=reserve(range.len(),control)?;timeline.get_mut().name=row.optional_text(3)?.map(|word|reconstruct_text(control,word)).transpose()?;
  for position in range{
   let row=channels.row(channel_order[position])?;let custom=row.optional_text(5)?;let property=match row.text(4)?{"translation"=>AnimTargetProperty::Translation,"rotation"=>AnimTargetProperty::Rotation,"scale"=>AnimTargetProperty::Scale,"weights"=>AnimTargetProperty::Weights,"custom"=>{if custom.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"missing custom animation property name"))}AnimTargetProperty::Custom{name:String::new()}},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio animation target property"))};
   if row.text(4)?!="custom"&&custom.is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unexpected custom animation property name"))}
   let interpolation=match row.text(6)?{"linear"=>AnimInterpolation::Linear,"step"=>AnimInterpolation::Step,"cubic_spline"=>AnimInterpolation::CubicSpline,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio animation interpolation"))};
   let range=keyframes.range_by(&keyframe_order,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
   let mut channel=Owned::new(AnimChannel{target:AnimTarget{node:String::new(),property},interpolation,keyframes:Vec::new()});channel.get_mut().keyframes=reserve(range.len(),control)?;channel.get_mut().target.node=reconstruct_text(control,row.text(3)?)?;
   if let AnimTargetProperty::Custom{name}=&mut channel.get_mut().target.property{*name=reconstruct_text(control,custom.unwrap())?;}
   for position in range{
    let row=keyframes.row(keyframe_order[position])?;let mut keyframe=Owned::new(AnimKeyframe{t:row.real(3)?,value:AnimValue::Scalar{value:0.0}});
    match row.text(4)?{
     "scalar"=>{let detail=scalars.take(row.rowid,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing scalar animation value"))?;keyframe.get_mut().value=AnimValue::Scalar{value:detail.real(1)?};},
     "vector3"=>{let detail=vectors.take(row.rowid,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing vector animation value"))?;keyframe.get_mut().value=AnimValue::Vec3{value:SemioPoint3{x:detail.real(1)?,y:detail.real(2)?,z:detail.real(3)?}};},
     "quaternion"=>{let detail=quaternions.take(row.rowid,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing quaternion animation value"))?;keyframe.get_mut().value=AnimValue::Quat{value:SemioQuaternion{x:detail.real(1)?,y:detail.real(2)?,z:detail.real(3)?,w:detail.real(4)?}};},
     "weights"=>{
      keyframe.get_mut().value=AnimValue::Weights{values:Vec::new()};let range=weights.range_by(&weight_order,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
      if let AnimValue::Weights{values}=&mut keyframe.get_mut().value{*values=reserve(range.len(),control)?;for position in range{let detail=weights.take_index(weight_order[position],control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"multiply owned Semio animation weight"))?;values.push(detail.real(3)?);completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
     },
     _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio animation keyframe value kind")),
    }
    channel.get_mut().keyframes.push(keyframe.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
   }
   timeline.get_mut().channels.push(channel.take());control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
  }
  snapshot.get_mut().timelines.push(timeline.take());control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
 }
 if scalars.remaining()!=0||vectors.remaining()!=0||quaternions.remaining()!=0||weights.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"orphan or contradictory Semio animation variant detail"))}
 snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(snapshot.take())
}
