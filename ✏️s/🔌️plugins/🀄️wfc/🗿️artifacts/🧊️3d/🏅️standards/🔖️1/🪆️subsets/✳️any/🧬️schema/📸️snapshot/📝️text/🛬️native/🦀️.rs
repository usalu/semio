//! 🧊️ Authored graph-native frontier and cumulative literal ownership.
use super::*;
use dsl::{DslValue,FieldValue,RecordValue,TextError,NativeDecodeControl,NativeEncodeControl};
use semio_framework_value::{FromValue,ToValue};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding};
fn error(s:impl Into<String>)->TextError{dsl::__rt::field_error(s.into())}
fn add(rows:&mut usize,count:usize,maximum:usize)->Result<(),String>{*rows=rows.checked_add(count).filter(|n|*n<=maximum).ok_or("wfc3d native rows exceed caller limit")?;Ok(())}
fn field<'a>(v:&'a DslValue,key:&str,c:&mut NativeDecodeControl<'_>)->Result<Option<&'a DslValue>,String>{
 let DslValue::Object(items)=v else{return Err("wfc3d native media object differs".into())};let mut found=None;
 for(name,value)in items{c.step()?;if name==key{if found.is_some(){return Err("wfc3d native media has duplicate fields".into())}found=Some(value)}}Ok(found)
}
fn array(v:Option<&DslValue>)->Result<&[DslValue],String>{match v{Some(DslValue::Array(items))=>Ok(items),None=>Ok(&[]),_=>Err("wfc3d native media collection differs".into())}}
fn table(record:&RecordValue,id:u16)->Result<&[FieldValue],String>{match record.get(id){Some(FieldValue::List(items))=>Ok(items),None|Some(FieldValue::Absent)=>Ok(&[]),_=>Err("wfc3d native table shape differs".into())}}
fn admit_record(record:&RecordValue,c:&mut NativeDecodeControl<'_>,maximum:usize)->Result<(),String>{
 c.scoped_stage(|c|{c.begin_stage(0)?;let mut rows=1usize;add(&mut rows,0,maximum)?;for id in[2,3,4,5]{add(&mut rows,table(record,id)?.len(),maximum)?;}
 for tile in table(record,4)?{c.step()?;let FieldValue::Record(tile)=tile else{return Err("wfc3d native tile row differs".into())};add(&mut rows,1,maximum)?;
  let media=match tile.get(3){None|Some(FieldValue::Absent)|Some(FieldValue::Value(DslValue::Null))=>continue,Some(FieldValue::Value(v))=>v,_=>return Err("wfc3d native media field differs".into())};
  match field(media,"kind",c)?{Some(DslValue::String(kind))if kind=="mesh"=>{add(&mut rows,array(field(media,"positions",c)?)?.len(),maximum)?;add(&mut rows,array(field(media,"indices",c)?)?.len(),maximum)?;if field(media,"color",c)?.is_some_and(|v|!matches!(v,DslValue::Null)){add(&mut rows,1,maximum)?;}},Some(DslValue::String(kind))if kind=="meshChild"=>{},_=>return Err("wfc3d native media kind differs".into())}
 }c.checkpoint()})
}
fn construct(record:&RecordValue,c:&mut NativeDecodeControl<'_>,maximum:usize)->Result<Wfc3dSnapshot,TextError>{
 admit_record(record,c,maximum).map_err(error)?;let parsed=Wfc3dSnapshotDsl::__dsl_from_record_controlled(record,c)?;
 let slots=c.scoped_stage(|c|->Result<Vec<Slot3d>,String>{c.begin_stage(parsed.slots.len())?;let mut values=c.allocate_vec::<Slot3d>(parsed.slots.len())?;for slot in parsed.slots{values.push(Slot3d{id:slot.id,x:slot.x,y:slot.y,z:slot.z,width:slot.width,height:slot.height,depth:slot.depth,pinned_tile_id:slot.pinned_tile_id});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 let edges=c.scoped_stage(|c|->Result<Vec<SlotEdge>,String>{c.begin_stage(parsed.edges.len())?;let mut values=c.allocate_vec::<SlotEdge>(parsed.edges.len())?;for edge in parsed.edges{values.push(SlotEdge{id:edge.id,from_slot_id:edge.from_slot_id,to_slot_id:edge.to_slot_id,relation:edge.relation});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 let tiles=c.scoped_stage(|c|->Result<Vec<Tile>,String>{c.begin_stage(parsed.tiles.len())?;let mut values=c.allocate_vec::<Tile>(parsed.tiles.len())?;for tile in parsed.tiles{let media=if matches!(tile.media,DslValue::Null){TileMedia3d::default()}else{TileMedia3d::from_value_controlled(&tile.media,c).map_err(|e|e.to_string())?};values.push(Tile{id:tile.id,label:tile.label,weight:tile.weight,media});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 let rules=c.scoped_stage(|c|->Result<Vec<GraphRule>,String>{c.begin_stage(parsed.rules.len())?;let mut values=c.allocate_vec::<GraphRule>(parsed.rules.len())?;for rule in parsed.rules{values.push(GraphRule{id:rule.id,tile_a_id:rule.tile_a_id,tile_b_id:rule.tile_b_id,relation:rule.relation,allowed:rule.allowed});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 Ok(Wfc3dSnapshot{schema:parsed.schema,seed:parsed.seed,slots,edges,tiles,rules})
}
pub(crate)fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Wfc3dSnapshot,String>{let maximum=c.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,<Wfc3dSnapshot as store::ArtifactDsl>::envelope_id(),Wfc3dSnapshotDsl::__dsl_spec_producer(),|record,native|construct(record,native,maximum),c)}
fn rows(s:&Wfc3dSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<(),String>{
 let maximum=c.limits().max_rows;let mut rows=1usize;add(&mut rows,0,maximum)?;for count in[s.slots.len(),s.edges.len(),s.tiles.len(),s.rules.len()]{add(&mut rows,count,maximum)?;}
 c.checkpoint(SqliteSnapshotPhase::EncodeNative,0,s.tiles.len())?;for(i,tile)in s.tiles.iter().enumerate(){add(&mut rows,1,maximum)?;if let TileMedia3d::Mesh{positions,indices,color}=&tile.media{add(&mut rows,positions.len(),maximum)?;add(&mut rows,indices.len(),maximum)?;add(&mut rows,usize::from(color.is_some()),maximum)?;}if i%256==0{c.checkpoint(SqliteSnapshotPhase::EncodeNative,i,s.tiles.len())?;}}c.checkpoint(SqliteSnapshotPhase::EncodeNative,s.tiles.len(),s.tiles.len())
}
fn optional_text(v:&Option<String>,c:&mut NativeEncodeControl<'_>)->Result<Option<String>,String>{v.as_deref().map(|s|c.copy_text(s)).transpose()}
fn project(s:&Wfc3dSnapshot,c:&mut NativeEncodeControl<'_>)->Result<RecordValue,TextError>{
 let schema=c.copy_text(&s.schema).map_err(error)?;
 let slots=c.scoped_stage(|c|->Result<Vec<Slot3dDsl>,String>{c.begin_stage(s.slots.len())?;let mut values=c.allocate_vec::<Slot3dDsl>(s.slots.len())?;for slot in &s.slots{values.push(Slot3dDsl{id:c.copy_text(&slot.id)?,x:slot.x,y:slot.y,z:slot.z,width:slot.width,height:slot.height,depth:slot.depth,pinned_tile_id:optional_text(&slot.pinned_tile_id,c)?});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 let edges=c.scoped_stage(|c|->Result<Vec<SlotEdgeDsl>,String>{c.begin_stage(s.edges.len())?;let mut values=c.allocate_vec::<SlotEdgeDsl>(s.edges.len())?;for edge in &s.edges{values.push(SlotEdgeDsl{id:c.copy_text(&edge.id)?,from_slot_id:c.copy_text(&edge.from_slot_id)?,to_slot_id:c.copy_text(&edge.to_slot_id)?,relation:c.copy_text(&edge.relation)?});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 let tiles=c.scoped_stage(|c|->Result<Vec<TileDsl>,String>{c.begin_stage(s.tiles.len())?;let mut values=c.allocate_vec::<TileDsl>(s.tiles.len())?;for tile in &s.tiles{let id=c.copy_text(&tile.id)?;let label=optional_text(&tile.label,c)?;let media=tile.media.to_value_controlled(c).map_err(|e|e.to_string())?;values.push(TileDsl{id,label,weight:tile.weight,media});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 let rules=c.scoped_stage(|c|->Result<Vec<GraphRuleDsl>,String>{c.begin_stage(s.rules.len())?;let mut values=c.allocate_vec::<GraphRuleDsl>(s.rules.len())?;for rule in &s.rules{values.push(GraphRuleDsl{id:c.copy_text(&rule.id)?,tile_a_id:c.copy_text(&rule.tile_a_id)?,tile_b_id:c.copy_text(&rule.tile_b_id)?,relation:optional_text(&rule.relation,c)?,allowed:rule.allowed});c.step()?;}c.checkpoint()?;Ok(values)}).map_err(error)?;
 Wfc3dSnapshotDsl{schema,seed:s.seed,slots,edges,tiles,rules}.__dsl_to_record_controlled(c)
}
pub(crate)fn encode_sqlite_snapshot_native(s:&Wfc3dSnapshot,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{rows(s,c)?;store::encode_sqlite_snapshot_record_native(encoding,<Wfc3dSnapshot as store::ArtifactDsl>::envelope_id(),Wfc3dSnapshotDsl::__dsl_spec_producer(),|native|project(s,native),c)}
