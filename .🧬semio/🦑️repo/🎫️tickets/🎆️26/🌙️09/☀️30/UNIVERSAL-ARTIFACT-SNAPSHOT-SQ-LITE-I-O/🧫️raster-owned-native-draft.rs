//! 🖨️ Staged literal Raster entities; not mounted before the owning Native baseline.
use crate::*;
use super::RasterSnapshot;
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,validate_sqlite_database_schema,artifact::{Projection,Cell,FloatColumn,FloatRow,Reconstruction,insert_ieee754}};
use std::collections::BTreeMap;
const SCHEMA:&str=include_str!("🗄️.sql");
fn ordinal(value:usize)->Result<i64,String>{i64::try_from(value).map_err(|e|e.to_string())}
fn scalar64(p:&mut Projection<'_,'_>,value:f64)->Result<i64,String>{insert_ieee754(p,"raster_scalar64",&[Cell::Real(value)],&[FloatColumn::Binary64(1)])}
fn scalar32(p:&mut Projection<'_,'_>,value:f32)->Result<i64,String>{insert_ieee754(p,"raster_scalar32",&[Cell::Float32(value)],&[FloatColumn::Binary32(1)])}
fn optional_unsigned(value:Option<u32>)->Cell<'static>{value.map(|v|Cell::Integer(i64::from(v))).unwrap_or(Cell::Null)}
fn transform(p:&mut Projection<'_,'_>,table:&str,id:i64,t:&RasterTransform)->Result<(),String>{let mut cells=Vec::new();for value in [t.x,t.y,t.a,t.b,t.c,t.d]{cells.push(Cell::Integer(scalar64(p,value)?));}p.insert_key(table,id,&cells)}
fn intrinsic(p:&mut Projection<'_,'_>,root:&dsl::DslValue)->Result<i64,String>{
 enum Edge<'a>{Item(i64,usize),Member(i64,usize,&'a str)}
 let mut pending=vec![(root,None)];let mut root_id=0;
 while let Some((v,edge))=pending.pop(){p.checkpoint()?;let kind=match v{dsl::DslValue::Null=>"null",dsl::DslValue::Bool(_)=>"boolean",dsl::DslValue::Number(dsl::Number::Int(_))=>"signed",dsl::DslValue::Number(dsl::Number::UInt(_))=>"unsigned",dsl::DslValue::Number(dsl::Number::Float(_))=>"float",dsl::DslValue::String(_)=>"text",dsl::DslValue::Bytes(_)=>"bytes",dsl::DslValue::Array(_)=>"array",dsl::DslValue::Object(_)=>"object"};let id=p.insert("raster_value",&[Cell::Text(kind)])?;
 match edge{None=>root_id=id,Some(Edge::Item(parent,i))=>{p.insert("raster_item",&[Cell::Integer(parent),Cell::Integer(ordinal(i)?),Cell::Integer(id)])?;},Some(Edge::Member(parent,i,name))=>{p.insert("raster_member",&[Cell::Integer(parent),Cell::Integer(ordinal(i)?),Cell::Text(name),Cell::Integer(id)])?;}}
 match v{
 dsl::DslValue::Null=>{},dsl::DslValue::Bool(v)=>p.insert_key("raster_boolean",id,&[Cell::Integer(i64::from(*v))])?,
 dsl::DslValue::Number(dsl::Number::Int(v))=>p.insert_key("raster_signed",id,&[Cell::Integer(*v)])?,
 dsl::DslValue::Number(dsl::Number::UInt(v))=>p.insert_key("raster_unsigned",id,&[Cell::Integer((v>>32)as i64),Cell::Integer((v&0xffffffff)as i64)])?,
 dsl::DslValue::Number(dsl::Number::Float(v))=>{let scalar=scalar64(p,*v)?;p.insert_key("raster_float",id,&[Cell::Integer(scalar)])?;},
 dsl::DslValue::String(v)=>p.insert_key("raster_text",id,&[Cell::Text(v)])?,dsl::DslValue::Bytes(v)=>p.insert_key("raster_bytes",id,&[Cell::Blob(v)])?,
 dsl::DslValue::Array(v)=>{p.insert_key("raster_array",id,&[])?;p.check_rows(pending.len().checked_add(v.len()).ok_or("Raster frontier overflow")?)?;for(i,child)in v.iter().enumerate().rev(){pending.push((child,Some(Edge::Item(id,i))));}},
 dsl::DslValue::Object(v)=>{p.insert_key("raster_object",id,&[])?;p.check_rows(pending.len().checked_add(v.len()).ok_or("Raster frontier overflow")?)?;for(i,(name,child))in v.iter().enumerate().rev(){pending.push((child,Some(Edge::Member(id,i,name))));}}
 }
 }Ok(root_id)
}
fn project(s:&RasterSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
 let mut p=Projection::new(SCHEMA,control)?;p.insert_key("raster_document",1,&[Cell::Text(&s.schema),Cell::Text(&s.id),s.title.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?;
 p.check_rows(s.assets.len())?;for(i,(key,child))in s.assets.iter().enumerate(){let target=&child.target;let dialect=&target.dialect;p.insert("raster_asset",&[Cell::Integer(1),Cell::Integer(ordinal(i)?),Cell::Text(key),Cell::Text(&child.child_id),Cell::Text(&dialect.artifact_kind),Cell::Text(&dialect.standard),Cell::Text(&dialect.subset),Cell::Text(&target.artifact_id)])?;}
 p.check_rows(s.layers.len())?;let mut pending=Vec::new();for(i,node)in s.layers.iter().enumerate().rev(){pending.push((node,None,i));}
 while let Some((node,parent,i))=pending.pop(){p.checkpoint()?;let(kind,id,name,visible,locked,opacity,blend,t,mask)=match node{
 RasterLayerNode::Pixel{id,name,visible,locked,opacity,blend_mode,transform,mask,..}=>("pixel",id,name,visible,locked,opacity,blend_mode,transform,mask.as_ref()),
 RasterLayerNode::Group{id,name,visible,locked,opacity,blend_mode,transform,mask,..}=>("group",id,name,visible,locked,opacity,blend_mode,transform,mask.as_ref()),
 RasterLayerNode::Adjustment{id,name,visible,locked,opacity,blend_mode,transform,..}=>("adjustment",id,name,visible,locked,opacity,blend_mode,transform,None)};
 let opacity=scalar32(&mut p,*opacity)?;let key=p.insert("raster_layer",&[if parent.is_none(){Cell::Integer(1)}else{Cell::Null},parent.map(Cell::Integer).unwrap_or(Cell::Null),Cell::Integer(ordinal(i)?),Cell::Text(kind),Cell::Text(id),Cell::Text(name),Cell::Integer(i64::from(*visible)),Cell::Integer(i64::from(*locked)),Cell::Integer(opacity),Cell::Text(blend)])?;transform(&mut p,"raster_transform",key,t)?;
 if let Some(mask)=mask{p.insert_key("raster_mask",key,&[Cell::Integer(i64::from(mask.enabled)),Cell::Integer(i64::from(mask.linked)),Cell::Integer(i64::from(mask.invert)),optional_unsigned(mask.width),optional_unsigned(mask.height),mask.image_key.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?;transform(&mut p,"raster_mask_transform",key,&mask.transform)?;}
 match node{
 RasterLayerNode::Pixel{width,height,image_key,..}=>p.insert_key("raster_pixel",key,&[optional_unsigned(*width),optional_unsigned(*height),image_key.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?,
 RasterLayerNode::Group{children,..}=>{p.insert_key("raster_group",key,&[])?;p.check_rows(pending.len().checked_add(children.len()).ok_or("Raster frontier overflow")?)?;for(i,child)in children.iter().enumerate().rev(){pending.push((child,Some(key),i));}},
 RasterLayerNode::Adjustment{adjustment_kind,params,..}=>{p.insert_key("raster_adjustment",key,&[Cell::Text(adjustment_kind)])?;p.check_rows(params.len())?;for(i,(name,value))in params.iter().enumerate(){let root=intrinsic(&mut p,value)?;p.insert("raster_parameter",&[Cell::Integer(key),Cell::Integer(ordinal(i)?),Cell::Text(name),Cell::Integer(root)])?;}}
 }
 }p.finish()
}

const TABLES:[(&str,usize);23]=[("raster_document",4),("raster_scalar64",4),("raster_scalar32",4),("raster_asset",9),("raster_layer",11),("raster_transform",7),("raster_mask",7),("raster_mask_transform",7),("raster_pixel",4),("raster_group",1),("raster_adjustment",2),("raster_parameter",5),("raster_value",2),("raster_boolean",2),("raster_signed",2),("raster_unsigned",3),("raster_float",2),("raster_text",2),("raster_bytes",2),("raster_array",1),("raster_item",4),("raster_object",1),("raster_member",5)];
struct Reader<'a,'c,'p>{rows:BTreeMap<&'static str,BTreeMap<i64,&'a SqliteRow>>,groups:BTreeMap<(&'static str,usize),BTreeMap<i64,Vec<&'a SqliteRow>>>,control:&'c mut SqliteSnapshotControl<'p>}
impl<'a,'c,'p> Reader<'a,'c,'p>{
 fn new(database:&'a SqliteDatabase,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,String>{control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,SCHEMA,control.limits()).map_err(|e|e.to_string())?;let mut rows=BTreeMap::new();for(name,width)in TABLES{let mut values=BTreeMap::new();for(i,row)in database.table(name)?.rows.iter().enumerate(){if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid||values.insert(row.rowid,row).is_some(){return Err("Raster row identity or fields".into());}if i%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,i,0)?;}}rows.insert(name,values);}Ok(Self{rows,groups:BTreeMap::new(),control})}
 fn has(&self,name:&str,id:i64)->bool{self.rows.get(name).is_some_and(|v|v.contains_key(&id))}
 fn take(&mut self,name:&str,id:i64)->Result<&'a SqliteRow,String>{self.rows.get_mut(name).ok_or("Raster table")?.remove(&id).ok_or_else(||"Raster dangling or multiply owned entity".into())}
 fn null(row:&SqliteRow,index:usize)->Result<bool,String>{Ok(row.values.get(index).ok_or("Raster column")?==&SqliteValue::Null)}
 fn text(&mut self,row:&SqliteRow,index:usize)->Result<String,String>{Reconstruction::new(self.control)?.text(row.text(index)?)}
 fn optional_text(&mut self,row:&SqliteRow,index:usize)->Result<Option<String>,String>{if Self::null(row,index)?{Ok(None)}else{self.text(row,index).map(Some)}}
 fn boolean(row:&SqliteRow,index:usize)->Result<bool,String>{match row.integer(index)?{0=>Ok(false),1=>Ok(true),_=>Err("Raster boolean".into())}}
 fn unsigned(row:&SqliteRow,index:usize)->Result<u32,String>{u32::try_from(row.integer(index)?).map_err(|e|e.to_string())}
 fn optional_unsigned(row:&SqliteRow,index:usize)->Result<Option<u32>,String>{if Self::null(row,index)?{Ok(None)}else{Self::unsigned(row,index).map(Some)}}
 fn real(&mut self,row:&SqliteRow,index:usize)->Result<f64,String>{let v=self.take("raster_scalar64",row.integer(index)?)?;Reconstruction::new(self.control)?.scalar()?;FloatRow::new(v,&[FloatColumn::Binary64(1)])?.real(1)}
 fn opacity(&mut self,row:&SqliteRow)->Result<f32,String>{let v=self.take("raster_scalar32",row.integer(9)?)?;Reconstruction::new(self.control)?.scalar()?;FloatRow::new(v,&[FloatColumn::Binary32(1)])?.binary32(1)}
 fn transform(&mut self,name:&str,id:i64)->Result<RasterTransform,String>{let v=self.take(name,id)?;Ok(RasterTransform{x:self.real(v,1)?,y:self.real(v,2)?,a:self.real(v,3)?,b:self.real(v,4)?,c:self.real(v,5)?,d:self.real(v,6)?})}
 fn list(&mut self,name:&'static str,parent:i64,index:usize,order:usize)->Result<Vec<&'a SqliteRow>,String>{let key=(name,index);if !self.groups.contains_key(&key){let source=self.rows.get(name).ok_or("Raster table")?;self.control.check_rows(source.len())?;let mut groups=BTreeMap::<i64,Vec<&SqliteRow>>::new();for(i,row)in source.values().enumerate(){if !Self::null(row,index)?{groups.entry(row.integer(index)?).or_default().push(*row);}if i%256==0{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,i,source.len())?;}}self.groups.insert(key,groups);}let source=self.groups.get_mut(&key).unwrap().remove(&parent).unwrap_or_default();let mut ordered=BTreeMap::new();for row in source{let ordinal=row.integer(order)?;if ordinal<0||ordered.insert(ordinal,row).is_some(){return Err("Raster duplicate or negative ordinal".into());}}let mut rows=Vec::new();for(i,(order,row))in ordered.into_iter().enumerate(){if order!=ordinal(i)?{return Err("Raster relationship order".into());}self.take(name,row.rowid)?;rows.push(row);if i%256==0{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,i,0)?;}}Ok(rows)}
 fn mask(&mut self,id:i64)->Result<Option<RasterLayerMask>,String>{if !self.has("raster_mask",id){return Ok(None)}let v=self.take("raster_mask",id)?;Ok(Some(RasterLayerMask{enabled:Self::boolean(v,1)?,linked:Self::boolean(v,2)?,invert:Self::boolean(v,3)?,width:Self::optional_unsigned(v,4)?,height:Self::optional_unsigned(v,5)?,image_key:self.optional_text(v,6)?,transform:self.transform("raster_mask_transform",id)?}))}
 fn intrinsic(&mut self,root:i64)->Result<dsl::DslValue,String>{
 enum Task<'a>{Node(i64),Array(i64,Vec<i64>),Object(i64,Vec<(&'a SqliteRow,i64)>)}
 let mut pending=vec![Task::Node(root)];let mut forest=Values(BTreeMap::new());
 while let Some(task)=pending.pop(){self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;match task{
 Task::Node(id)=>{let row=self.take("raster_value",id)?;let value=match row.text(1)?{
 "null"=>dsl::DslValue::Null,"boolean"=>dsl::DslValue::Bool(Self::boolean(self.take("raster_boolean",id)?,1)?),
 "signed"=>dsl::DslValue::Number(dsl::Number::Int(self.take("raster_signed",id)?.integer(1)?)),
 "unsigned"=>{let v=self.take("raster_unsigned",id)?;dsl::DslValue::Number(dsl::Number::UInt((u64::from(Self::unsigned(v,1)?)<<32)|u64::from(Self::unsigned(v,2)?)))},
 "float"=>{let v=self.take("raster_float",id)?;dsl::DslValue::Number(dsl::Number::Float(self.real(v,1)?))},
 "text"=>{let v=self.take("raster_text",id)?;dsl::DslValue::String(self.text(v,1)?)},
 "bytes"=>{let v=self.take("raster_bytes",id)?;dsl::DslValue::Bytes(Reconstruction::new(self.control)?.blob(v.blob(1)?)?)},
 "array"=>{self.take("raster_array",id)?;let rows=self.list("raster_item",id,1,2)?;self.control.check_rows(pending.len().checked_add(rows.len()).ok_or("Raster intrinsic frontier overflow")?)?;let ids=rows.iter().map(|v|v.integer(3)).collect::<Result<Vec<_>,_>>()?;pending.push(Task::Array(id,ids));for child in rows.iter().rev(){pending.push(Task::Node(child.integer(3)?));}continue;},
 "object"=>{self.take("raster_object",id)?;let rows=self.list("raster_member",id,1,2)?;self.control.check_rows(pending.len().checked_add(rows.len()).ok_or("Raster intrinsic frontier overflow")?)?;let mut items=Vec::new();for row in rows{items.push((row,row.integer(4)?));}for(_,child)in items.iter().rev(){pending.push(Task::Node(*child));}let index=pending.len()-items.len();pending.insert(index,Task::Object(id,items));continue;},_=>return Err("Raster intrinsic variant".into())};forest.0.insert(id,value);},
 Task::Array(id,children)=>{let mut values=Items(Vec::new());for child in children{values.0.push(forest.0.remove(&child).ok_or("Raster intrinsic item")?);}forest.0.insert(id,dsl::DslValue::Array(std::mem::take(&mut values.0)));},
 Task::Object(id,children)=>{let mut values=Members(Vec::new());for(row,child)in children{let name=self.text(row,3)?;values.0.push((name,forest.0.remove(&child).ok_or("Raster intrinsic member")?));}forest.0.insert(id,dsl::DslValue::Object(std::mem::take(&mut values.0)));}
 }}forest.0.remove(&root).ok_or_else(||"Raster intrinsic root".into())
 }
}
struct Values(BTreeMap<i64,dsl::DslValue>);
impl Drop for Values{fn drop(&mut self){while let Some((_,value))=self.0.pop_last(){<dsl::DslValue as dsl::FromValue>::retire_decoded(value);}}}
struct Members(Vec<(String,dsl::DslValue)>);
impl Drop for Members{fn drop(&mut self){while let Some((_,value))=self.0.pop(){<dsl::DslValue as dsl::FromValue>::retire_decoded(value);}}}

struct Items(Vec<dsl::DslValue>);
impl Drop for Items{fn drop(&mut self){while let Some(value)=self.0.pop(){<dsl::DslValue as dsl::FromValue>::retire_decoded(value);}}}
fn retire_snapshot(snapshot:RasterSnapshot){
 let mut cursor=store::ArtifactOwnedValueRetirementFactory::retire_owned(&crate::standards::v1::subsets::any::schema::mutations::binary::RasterSnapshotRetirementFactory,snapshot);
 loop{match cursor.close_step(256,65536).expect("Raster owned snapshot retirement"){
 store::SnapshotRetirementStep::Pending{..}=>{},
 store::SnapshotRetirementStep::Blocked=>panic!("Raster owned snapshot retirement blocked"),
 store::SnapshotRetirementStep::Complete=>break,
 }}assert!(cursor.terminal_is_empty());
}
fn retire_layer(root:RasterLayerNode){retire_snapshot(RasterSnapshot{schema:String::new(),id:String::new(),title:None,layers:vec![root],assets:RasterOwnedMap::new()});}
struct LayerOwned(Option<RasterLayerNode>);
impl Drop for LayerOwned{fn drop(&mut self){if let Some(node)=self.0.take(){retire_layer(node);}}}
struct Forest(BTreeMap<i64,RasterLayerNode>);
impl Drop for Forest{fn drop(&mut self){while let Some((_,node))=self.0.pop_last(){retire_layer(node);}}}
struct Parameters(RasterOwnedMap<dsl::DslValue>);
impl Drop for Parameters{fn drop(&mut self){while let Some((_,value))=self.0.take_last_entry(){<dsl::DslValue as dsl::FromValue>::retire_decoded(value);}self.0.retire();}}
struct SnapshotOwned(Option<RasterSnapshot>);
impl Drop for SnapshotOwned{fn drop(&mut self){if let Some(value)=self.0.take(){retire_snapshot(value);}}}
fn ordered_keys(a:&str,b:&str,control:&mut SqliteSnapshotControl<'_>)->Result<bool,String>{
 let left=a.as_bytes();let right=b.as_bytes();let limit=left.len().min(right.len());let mut position=0;
 control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,limit)?;
 while position<limit{let end=position.saturating_add(65536).min(limit);let order=left[position..end].cmp(&right[position..end]);if order!=std::cmp::Ordering::Equal{return Ok(order==std::cmp::Ordering::Less)}position=end;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,limit)?;}
 Ok(left.len()<right.len())
}
impl Reader<'_,'_,'_>{
 fn leaf(&mut self,row:&SqliteRow)->Result<RasterLayerNode,String>{
 if Self::null(row,1)?==Self::null(row,2)?{return Err("Raster ambiguous forest owner".into())}
 let key=row.rowid;let id=self.text(row,5)?;let name=self.text(row,6)?;let visible=Self::boolean(row,7)?;let locked=Self::boolean(row,8)?;let opacity=self.opacity(row)?;let blend_mode=self.text(row,10)?;let transform=self.transform("raster_transform",key)?;let mask=self.mask(key)?;
 Ok(match row.text(4)?{
 "pixel"=>{let v=self.take("raster_pixel",key)?;RasterLayerNode::Pixel{id,name,visible,locked,opacity,blend_mode,transform,mask,width:Self::optional_unsigned(v,1)?,height:Self::optional_unsigned(v,2)?,image_key:self.optional_text(v,3)?}},
 "group"=>{self.take("raster_group",key)?;RasterLayerNode::Group{id,name,visible,locked,opacity,blend_mode,transform,mask,children:Vec::new()}},
 "adjustment"=>{if mask.is_some(){return Err("Raster adjustment cannot own mask".into())}let v=self.take("raster_adjustment",key)?;let adjustment_kind=self.text(v,1)?;let rows=self.list("raster_parameter",key,1,2)?;if rows.len()>64{return Err("Raster map capacity".into())}let mut params=Parameters(RasterOwnedMap::new());let mut previous=None;for row in rows{let key=self.text(row,3)?;if let Some(previous)=previous{if !ordered_keys(previous,row.text(3)?,self.control)?{return Err("Raster parameter key order".into())}}previous=Some(row.text(3)?);let value=self.intrinsic(row.integer(4)?)?;if let Err(rejected)=params.0.insert(key,value){<dsl::DslValue as dsl::FromValue>::retire_decoded(rejected.value);return Err(rejected.reason.into())}}RasterLayerNode::Adjustment{id,name,visible,locked,opacity,blend_mode,transform,adjustment_kind,params:std::mem::take(&mut params.0)}},
 _=>return Err("Raster layer variant".into())})
 }
}
fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<RasterSnapshot,String>{
 let mut r=Reader::new(database,control)?;let document=r.take("raster_document",1)?;let mut owner=SnapshotOwned(Some(RasterSnapshot{schema:r.text(document,1)?,id:r.text(document,2)?,title:r.optional_text(document,3)?,layers:Vec::new(),assets:RasterOwnedMap::new()}));
 let assets=r.list("raster_asset",1,1,2)?;if assets.len()>64{return Err("Raster map capacity".into())}let mut previous=None;for row in assets{let key=r.text(row,3)?;if let Some(previous)=previous{if !ordered_keys(previous,row.text(3)?,r.control)?{return Err("Raster asset key order".into())}}previous=Some(row.text(3)?);let child=store::ArtifactChild::new(r.text(row,4)?,store::io_schema::ArtifactRef{dialect:store::io_schema::ArtifactDialect{artifact_kind:r.text(row,5)?,standard:r.text(row,6)?,subset:r.text(row,7)?},artifact_id:r.text(row,8)?});owner.0.as_mut().unwrap().assets.insert(key,child).map_err(|e|e.reason.to_owned())?;}
 enum Task<'a>{Node(&'a SqliteRow),Group(i64,Vec<i64>)}
 let roots=r.list("raster_layer",1,1,3)?;let root_ids=roots.iter().map(|v|v.rowid).collect::<Vec<_>>();let mut pending=Vec::new();for row in roots.into_iter().rev(){pending.push(Task::Node(row));}let mut forest=Forest(BTreeMap::new());
 while let Some(task)=pending.pop(){r.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;match task{
 Task::Node(row)=>{forest.0.insert(row.rowid,r.leaf(row)?);if row.text(4)?=="group"{let children=r.list("raster_layer",row.rowid,2,3)?;r.control.check_rows(pending.len().checked_add(children.len()).ok_or("Raster frontier overflow")?)?;let ids=children.iter().map(|v|v.rowid).collect();pending.push(Task::Group(row.rowid,ids));for row in children.into_iter().rev(){pending.push(Task::Node(row));}}},
 Task::Group(id,ids)=>{let mut node=LayerOwned(Some(forest.0.remove(&id).ok_or("Raster group owner")?));let Some(RasterLayerNode::Group{children,..})=node.0.as_mut()else{return Err("Raster group discriminant".into())};for id in ids{children.push(forest.0.remove(&id).ok_or("Raster child owner")?);}forest.0.insert(id,node.0.take().unwrap());}
 }}if r.rows.values().any(|v|!v.is_empty()){return Err("orphan Raster entities".into())}r.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;for id in root_ids{owner.0.as_mut().unwrap().layers.push(forest.0.remove(&id).ok_or("Raster root owner")?);}if !forest.0.is_empty(){return Err("unowned Raster layer".into())}Ok(owner.0.take().unwrap())
}
