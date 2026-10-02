//! ◻️ Authored slot-graph ownership and genuine cumulative native construction.
use super::*;
use dsl::{DslValue,FieldValue,RecordValue,TextError,NativeDecodeControl,NativeEncodeControl};
use semio_framework_value::{FromValue,ToValue};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding};
fn error(s:impl Into<String>)->TextError{dsl::__rt::field_error(s.into())}
fn add(rows:&mut usize,count:usize,maximum:usize)->Result<(),String>{*rows=rows.checked_add(count).filter(|n|*n<=maximum).ok_or("wfc2d native rows exceed caller limit")?;Ok(())}
fn field<'a>(v:&'a DslValue,key:&str,c:&mut NativeDecodeControl<'_>)->Result<Option<&'a DslValue>,String>{
 let DslValue::Object(items)=v else{return Err("wfc2d native media object differs".into())};let mut found=None;
 for(name,value)in items{c.step()?;if name==key{if found.is_some(){return Err("wfc2d native media has duplicate fields".into())}found=Some(value)}}Ok(found)
}
fn array(v:Option<&DslValue>)->Result<&[DslValue],String>{match v{Some(DslValue::Array(items))=>Ok(items),None=>Ok(&[]),_=>Err("wfc2d native media collection differs".into())}}
fn table(record:&RecordValue,id:u16)->Result<&[FieldValue],String>{match record.get(id){Some(FieldValue::List(items))=>Ok(items),None|Some(FieldValue::Absent)=>Ok(&[]),_=>Err("wfc2d native table shape differs".into())}}
fn admit_record(record:&RecordValue,c:&mut NativeDecodeControl<'_>,maximum:usize)->Result<(),String>{
 c.scoped_stage(|c|{c.begin_stage(0)?;let mut rows=1usize;add(&mut rows,0,maximum)?;for id in[2,3,4,5]{add(&mut rows,table(record,id)?.len(),maximum)?;}
 for tile in table(record,4)?{c.step()?;let FieldValue::Record(tile)=tile else{return Err("wfc2d native tile row differs".into())};
  let media=match tile.get(3){None|Some(FieldValue::Absent)|Some(FieldValue::Value(DslValue::Null))=>continue,Some(FieldValue::Value(v))=>v,_=>return Err("wfc2d native media field differs".into())};
  if matches!(media,DslValue::String(name)if name=="Empty"){continue}let DslValue::Object(items)=media else{return Err("wfc2d native external media tag differs".into())};let[(kind,value)]=items.as_slice()else{return Err("wfc2d native external media arity differs".into())};add(&mut rows,1,maximum)?;
  match kind.as_str(){"Bitmap"=>add(&mut rows,array(field(value,"palette",c)?)?.len(),maximum)?,"Vector"=>{let paths=array(field(value,"paths",c)?)?;add(&mut rows,paths.len(),maximum)?;for path in paths{add(&mut rows,array(field(path,"segments",c)?)?.len(),maximum)?;for key in["fill","stroke"]{if field(path,key,c)?.is_some_and(|v|!matches!(v,DslValue::Null)){add(&mut rows,1,maximum)?;}}}},"Image"=>{},_=>return Err("wfc2d native media kind differs".into())}
 }c.checkpoint()})
}
fn construct(record:&RecordValue,c:&mut NativeDecodeControl<'_>,maximum:usize)->Result<Wfc2dSnapshot,TextError>{
 admit_record(record,c,maximum).map_err(error)?;let parsed=Wfc2dSnapshotDsl::__dsl_from_record_controlled(record,c)?;
 let slots=c.scoped_stage(|c|->Result<Vec<Wfc2dSlot>,String>{c.begin_stage(parsed.slots.len())?;let mut values=c.allocate_vec::<Wfc2dSlot>(parsed.slots.len())?;for slot in parsed.slots{values.push(Wfc2dSlot{id:slot.id,x:slot.x,y:slot.y,width:slot.width,height:slot.height,pinned_tile_id:slot.pinned_tile_id});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 let edges=c.scoped_stage(|c|->Result<Vec<Wfc2dSlotEdge>,String>{c.begin_stage(parsed.edges.len())?;let mut values=c.allocate_vec::<Wfc2dSlotEdge>(parsed.edges.len())?;for edge in parsed.edges{values.push(Wfc2dSlotEdge{id:edge.id,from_slot_id:edge.from_slot_id,to_slot_id:edge.to_slot_id,relation:edge.relation});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 let tiles=c.scoped_stage(|c|->Result<Vec<Wfc2dTile>,String>{c.begin_stage(parsed.tiles.len())?;let mut values=c.allocate_vec::<Wfc2dTile>(parsed.tiles.len())?;for tile in parsed.tiles{let media=if matches!(tile.media,DslValue::Null){Wfc2dTileMedia::Empty}else{Wfc2dTileMedia::from_value_controlled(&tile.media,c).map_err(|e|e.to_string())?};values.push(Wfc2dTile{id:tile.id,label:tile.label,weight:tile.weight,media});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 let rules=c.scoped_stage(|c|->Result<Vec<Wfc2dRule>,String>{c.begin_stage(parsed.rules.len())?;let mut values=c.allocate_vec::<Wfc2dRule>(parsed.rules.len())?;for rule in parsed.rules{values.push(Wfc2dRule{id:rule.id,tile_a_id:rule.tile_a_id,tile_b_id:rule.tile_b_id,relation:rule.relation,allowed:rule.allowed});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 Ok(Wfc2dSnapshot{schema:parsed.schema,seed:parsed.seed,slots,edges,tiles,rules})
}
pub(crate)fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Wfc2dSnapshot,String>{let maximum=c.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,<Wfc2dSnapshot as store::ArtifactDsl>::envelope_id(),Wfc2dSnapshotDsl::__dsl_spec_producer(),|record,native|construct(record,native,maximum),c)}
fn rows(s:&Wfc2dSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<(),String>{
 let maximum=c.limits().max_rows;let mut rows=1usize;add(&mut rows,0,maximum)?;for count in[s.slots.len(),s.edges.len(),s.tiles.len(),s.rules.len()]{add(&mut rows,count,maximum)?;}
 c.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let mut visited=0usize;for tile in &s.tiles{match &tile.media{Wfc2dTileMedia::Empty=>{},Wfc2dTileMedia::Bitmap{palette,..}=>{add(&mut rows,1,maximum)?;add(&mut rows,palette.len(),maximum)?;},Wfc2dTileMedia::Vector{paths}=>{add(&mut rows,1,maximum)?;add(&mut rows,paths.len(),maximum)?;for path in paths{add(&mut rows,path.segments.len(),maximum)?;add(&mut rows,usize::from(path.fill.is_some())+usize::from(path.stroke.is_some()),maximum)?;visited+=1;if visited%256==0{c.checkpoint(SqliteSnapshotPhase::EncodeNative,visited,0)?;}}},Wfc2dTileMedia::Image{..}=>add(&mut rows,1,maximum)?}visited+=1;if visited%256==0{c.checkpoint(SqliteSnapshotPhase::EncodeNative,visited,0)?;}}c.checkpoint(SqliteSnapshotPhase::EncodeNative,0,rows)
}
fn optional_text(v:&Option<String>,c:&mut NativeEncodeControl<'_>)->Result<Option<String>,String>{v.as_deref().map(|s|c.copy_text(s)).transpose()}
fn project(s:&Wfc2dSnapshot,c:&mut NativeEncodeControl<'_>)->Result<RecordValue,TextError>{
 let schema=c.copy_text(&s.schema).map_err(error)?;
 let slots=c.scoped_stage(|c|->Result<Vec<Wfc2dSlotDsl>,String>{c.begin_stage(s.slots.len())?;let mut values=c.allocate_vec::<Wfc2dSlotDsl>(s.slots.len())?;for slot in &s.slots{values.push(Wfc2dSlotDsl{id:c.copy_text(&slot.id)?,x:slot.x,y:slot.y,width:slot.width,height:slot.height,pinned_tile_id:optional_text(&slot.pinned_tile_id,c)?});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 let edges=c.scoped_stage(|c|->Result<Vec<Wfc2dSlotEdgeDsl>,String>{c.begin_stage(s.edges.len())?;let mut values=c.allocate_vec::<Wfc2dSlotEdgeDsl>(s.edges.len())?;for edge in &s.edges{values.push(Wfc2dSlotEdgeDsl{id:c.copy_text(&edge.id)?,from_slot_id:c.copy_text(&edge.from_slot_id)?,to_slot_id:c.copy_text(&edge.to_slot_id)?,relation:c.copy_text(&edge.relation)?});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 let tiles=c.scoped_stage(|c|->Result<Vec<Wfc2dTileDsl>,String>{c.begin_stage(s.tiles.len())?;let mut values=c.allocate_vec::<Wfc2dTileDsl>(s.tiles.len())?;for tile in &s.tiles{let id=c.copy_text(&tile.id)?;let label=optional_text(&tile.label,c)?;let media=tile.media.to_value_controlled(c).map_err(|e|e.to_string())?;values.push(Wfc2dTileDsl{id,label,weight:tile.weight,media});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 let rules=c.scoped_stage(|c|->Result<Vec<Wfc2dRuleDsl>,String>{c.begin_stage(s.rules.len())?;let mut values=c.allocate_vec::<Wfc2dRuleDsl>(s.rules.len())?;for rule in &s.rules{values.push(Wfc2dRuleDsl{id:c.copy_text(&rule.id)?,tile_a_id:c.copy_text(&rule.tile_a_id)?,tile_b_id:c.copy_text(&rule.tile_b_id)?,relation:optional_text(&rule.relation,c)?,allowed:rule.allowed});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 Wfc2dSnapshotDsl{schema,seed:s.seed,slots,edges,tiles,rules}.__dsl_to_record_controlled(c)
}
pub(crate)fn encode_sqlite_snapshot_native(s:&Wfc2dSnapshot,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{rows(s,c)?;store::encode_sqlite_snapshot_record_native(encoding,<Wfc2dSnapshot as store::ArtifactDsl>::envelope_id(),Wfc2dSnapshotDsl::__dsl_spec_producer(),|native|project(s,native),c)}
