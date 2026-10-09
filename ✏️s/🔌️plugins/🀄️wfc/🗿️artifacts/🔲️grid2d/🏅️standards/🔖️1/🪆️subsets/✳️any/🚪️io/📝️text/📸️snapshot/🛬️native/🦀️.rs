//! 🔲️ Authored grid-native ownership admission and cumulative typed construction.
use crate::standards::v1::subsets::any::io::text::snapshot::*;
use semio_framework_value::DslValue;
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::RecordValue;
use semio_framework_diagnostic::TextError;
use semio_framework_value::NativeDecodeControl;
use semio_framework_value::NativeEncodeControl;
use semio_framework_value::{FromValue,ToValue};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding};
use semio_framework_value::ValueError;
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message)}
fn error(error:ValueError)->TextError{TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1))}
fn add(rows:&mut usize,count:usize,maximum:usize)->Result<(),ValueError>{*rows=rows.checked_add(count).filter(|n|*n<=maximum).ok_or_else(||invalid("grid2d native rows exceed caller limit"))?;Ok(())}
fn field<'a>(v:&'a DslValue,key:&str,c:&mut NativeDecodeControl<'_>)->Result<Option<&'a DslValue>,ValueError>{
 let semio_framework_value::DslValue::Object(items)=v else{return Err(invalid("grid2d native media object differs"))};let mut found=None;
 for(name,value)in items{c.step()?;if name==key{if found.is_some(){return Err(invalid("grid2d native media has duplicate fields"))}found=Some(value)}}Ok(found)
}
fn array(v:Option<&DslValue>)->Result<&[DslValue],ValueError>{match v{Some(semio_framework_value::DslValue::Array(items))=>Ok(items),None=>Ok(&[]),_=>Err(invalid("grid2d native media collection differs"))}}
fn table(record:&RecordValue,id:u16)->Result<&[FieldValue],ValueError>{match record.get(id){Some(semio_framework_dsl_record::FieldValue::List(items))=>Ok(items),None|Some(semio_framework_dsl_record::FieldValue::Absent)=>Ok(&[]),_=>Err(invalid("grid2d native table shape differs"))}}
fn admit_record(record:&RecordValue,c:&mut NativeDecodeControl<'_>,maximum:usize)->Result<(),ValueError>{
 c.scoped_stage(|c|{c.begin_stage(0)?;let mut rows=1usize;add(&mut rows,0,maximum)?;for id in[8,9,10,11]{add(&mut rows,table(record,id)?.len(),maximum)?;}
 for tile in table(record,8)?{c.step()?;let semio_framework_dsl_record::FieldValue::Record(tile)=tile else{return Err(invalid("grid2d native tile row differs"))};add(&mut rows,1,maximum)?;
  let media=match tile.get(3){None|Some(semio_framework_dsl_record::FieldValue::Absent)|Some(semio_framework_dsl_record::FieldValue::Value(semio_framework_value::DslValue::Null))=>continue,Some(semio_framework_dsl_record::FieldValue::Value(v))=>v,_=>return Err(invalid("grid2d native media field differs"))};
  match field(media,"kind",c)?{Some(semio_framework_value::DslValue::String(kind))if kind=="bitmap"=>add(&mut rows,array(field(media,"palette",c)?)?.len(),maximum)?,Some(semio_framework_value::DslValue::String(kind))if kind=="vector"=>{let paths=array(field(media,"paths",c)?)?;add(&mut rows,paths.len(),maximum)?;for path in paths{add(&mut rows,array(field(path,"segments",c)?)?.len(),maximum)?;for key in["fill","stroke"]{if field(path,key,c)?.is_some_and(|v|!matches!(v,DslValue::Null)){add(&mut rows,1,maximum)?;}}}},Some(semio_framework_value::DslValue::String(kind))if kind=="image"=>{},_=>return Err(invalid("grid2d native media kind differs"))}
 }c.checkpoint()})
}
pub(crate)fn construct(record:&RecordValue,c:&mut NativeDecodeControl<'_>,maximum:usize)->Result<Grid2dSnapshot,ValueError>{
 admit_record(record,c,maximum)?;let parsed=Grid2dSnapshotDsl::__dsl_from_record_controlled(record,c)?;
 let tiles=c.scoped_stage(|c|->Result<Vec<WfcTile2d>,ValueError>{c.begin_stage(parsed.tiles.len())?;let mut values=c.allocate_vec::<WfcTile2d>(parsed.tiles.len())?;
 for tile in parsed.tiles{let media=if matches!(tile.media,DslValue::Null){WfcTileMedia2d::Vector{paths:Vec::new()}}else{WfcTileMedia2d::from_value_controlled(&tile.media,c)?};values.push(WfcTile2d{id:tile.id,label:tile.label,weight:tile.weight,media});c.step()?;}c.checkpoint()?;Ok(values)})?;
 let rules=c.scoped_stage(|c|->Result<Vec<WfcAdjacencyRule2d>,ValueError>{c.begin_stage(parsed.rules.len())?;let mut values=c.allocate_vec::<WfcAdjacencyRule2d>(parsed.rules.len())?;for rule in parsed.rules{let direction=direction_from_token(&rule.direction).map_err(|e|ValueError::new(e.kind,e.message))?;values.push(WfcAdjacencyRule2d{id:rule.id,tile_a_id:rule.tile_a_id,tile_b_id:rule.tile_b_id,direction,allowed:rule.allowed});c.step()?;}c.checkpoint()?;Ok(values)})?;
 let pinned=c.scoped_stage(|c|->Result<Vec<WfcPinnedCell2d>,ValueError>{c.begin_stage(parsed.pinned.len())?;let mut values=c.allocate_vec::<WfcPinnedCell2d>(parsed.pinned.len())?;for cell in parsed.pinned{values.push(WfcPinnedCell2d{x:cell.x,y:cell.y,tile_id:cell.tile_id});c.step()?;}c.checkpoint()?;Ok(values)})?;
 let masked=c.scoped_stage(|c|->Result<Vec<WfcCell2d>,ValueError>{c.begin_stage(parsed.masked.len())?;let mut values=c.allocate_vec::<WfcCell2d>(parsed.masked.len())?;for cell in parsed.masked{values.push(WfcCell2d{x:cell.x,y:cell.y});c.step()?;}c.checkpoint()?;Ok(values)})?;
 Ok(Grid2dSnapshot{schema:parsed.schema,seed:parsed.seed,width:parsed.width,height:parsed.height,cell_width:parsed.cell_width,cell_height:parsed.cell_height,periodic_x:parsed.periodic_x,periodic_y:parsed.periodic_y,tiles,rules,pinned,masked})
}
pub(crate)fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Grid2dSnapshot,ValueError>{let maximum=c.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,<Grid2dSnapshot as store::ArtifactDsl>::envelope_id(),Grid2dSnapshotDsl::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {construct(record,native,maximum)})(); *snapshot_output = Some(constructed?); Ok(()) },c,native_control)}
fn rows(s:&Grid2dSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let maximum=c.limits().max_rows;let mut rows=1usize;add(&mut rows,0,maximum)?;for count in[s.tiles.len(),s.rules.len(),s.pinned.len(),s.masked.len()]{add(&mut rows,count,maximum)?;}
 c.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let mut visited=0usize;for tile in &s.tiles{add(&mut rows,1,maximum)?;match &tile.media{WfcTileMedia2d::Bitmap{palette,..}=>add(&mut rows,palette.len(),maximum)?,WfcTileMedia2d::Vector{paths}=>{add(&mut rows,paths.len(),maximum)?;for path in paths{add(&mut rows,path.segments.len(),maximum)?;add(&mut rows,usize::from(path.fill.is_some())+usize::from(path.stroke.is_some()),maximum)?;visited+=1;if visited%256==0{c.checkpoint(SqliteSnapshotPhase::EncodeNative,visited,0)?;}}},WfcTileMedia2d::Image{..}=>{}}visited+=1;if visited%256==0{c.checkpoint(SqliteSnapshotPhase::EncodeNative,visited,0)?;}}c.checkpoint(SqliteSnapshotPhase::EncodeNative,0,rows)
}
fn optional_text(v:&Option<String>,c:&mut NativeEncodeControl<'_>)->Result<Option<String>,ValueError>{v.as_deref().map(|s|c.copy_text(s)).transpose()}
pub(crate)fn project(s:&Grid2dSnapshot,c:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{
 let schema=c.copy_text(&s.schema)?;
 let tiles=c.scoped_stage(|c|->Result<Vec<WfcTile2dDsl>,ValueError>{c.begin_stage(s.tiles.len())?;let mut values=c.allocate_vec::<WfcTile2dDsl>(s.tiles.len())?;for tile in &s.tiles{let id=c.copy_text(&tile.id)?;let label=optional_text(&tile.label,c)?;let media=tile.media.to_value_controlled(c)?;values.push(WfcTile2dDsl{id,label,weight:tile.weight,media});c.step()?;}c.checkpoint()?;Ok(values)})?;
 let rules=c.scoped_stage(|c|->Result<Vec<WfcAdjacencyRule2dDsl>,ValueError>{c.begin_stage(s.rules.len())?;let mut values=c.allocate_vec::<WfcAdjacencyRule2dDsl>(s.rules.len())?;for rule in &s.rules{values.push(WfcAdjacencyRule2dDsl{id:c.copy_text(&rule.id)?,tile_a_id:c.copy_text(&rule.tile_a_id)?,tile_b_id:c.copy_text(&rule.tile_b_id)?,direction:c.copy_text(direction_token(rule.direction))?,allowed:rule.allowed});c.step()?;}c.checkpoint()?;Ok(values)})?;
 let pinned=c.scoped_stage(|c|->Result<Vec<WfcPinnedCell2dDsl>,ValueError>{c.begin_stage(s.pinned.len())?;let mut values=c.allocate_vec::<WfcPinnedCell2dDsl>(s.pinned.len())?;for cell in &s.pinned{values.push(WfcPinnedCell2dDsl{x:cell.x,y:cell.y,tile_id:c.copy_text(&cell.tile_id)?});c.step()?;}c.checkpoint()?;Ok(values)})?;
 let masked=c.scoped_stage(|c|->Result<Vec<WfcCell2dDsl>,ValueError>{c.begin_stage(s.masked.len())?;let mut values=c.allocate_vec::<WfcCell2dDsl>(s.masked.len())?;for cell in &s.masked{values.push(WfcCell2dDsl{x:cell.x,y:cell.y});c.step()?;}c.checkpoint()?;Ok(values)})?;
 Grid2dSnapshotDsl{schema,seed:s.seed,width:s.width,height:s.height,cell_width:s.cell_width,cell_height:s.cell_height,periodic_x:s.periodic_x,periodic_y:s.periodic_y,tiles,rules,pinned,masked}.__dsl_to_record_controlled(c)
}
pub(crate)fn encode_sqlite_snapshot_native(s:&Grid2dSnapshot,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{rows(s,c)?;store::encode_sqlite_snapshot_record_native(encoding,<Grid2dSnapshot as store::ArtifactDsl>::envelope_id(),Grid2dSnapshotDsl::__dsl_spec_producer(),|native|project(s,native),c,native_owner)}
