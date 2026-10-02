//! 🗺️ Literal flat native records retain each intrinsic Value variant and durable child identity.
use super::*;
use dsl::DslField;
use dsl::schema::Number;
#[derive(dsl::DslRecord)]struct Child{child_id:String,artifact_id:String,artifact_kind:String,standard:String,subset:String}
#[derive(dsl::DslScalar)]enum Kind{Null,Boolean,Unsigned,Signed,Float,Text,Bytes,Array,Object}
struct Octets(Vec<u8>);
impl DslField for Octets{
 fn shape()->dsl::Shape{dsl::Shape::Bytes64}
 fn to_value(&self)->dsl::FieldValue{dsl::FieldValue::Bytes64(self.0.clone())}
 fn from_value(value:&dsl::FieldValue)->Result<Self,String>{match value{dsl::FieldValue::Bytes64(v)=>Ok(Self(v.clone())),_=>Err("GIS native octets differ".into())}}
 fn from_value_controlled(value:&dsl::FieldValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<Self,String>{match value{dsl::FieldValue::Bytes64(v)=>control.copy_bytes(v).map(Self),_=>Err("GIS native octets differ".into())}}
}
#[derive(dsl::DslRecord)]struct Member{name:String,value:u64}
#[derive(dsl::DslRecord)]struct Value{kind:Kind,boolean:Option<bool>,unsigned:Option<u64>,signed:Option<i64>,float:Option<f64>,text:Option<String>,bytes:Option<Octets>,items:Vec<u64>,members:Vec<Member>}
#[derive(dsl::DslRecord)]struct Feature{id:String,value:u64}
#[derive(dsl::DslRecord)]#[dsl(extension="gismap")]struct Map{positions:Vec<Feature>,routes:Vec<Feature>,regions:Vec<Feature>,drawing:Child,image:Option<Child>,value:Child,values:Vec<Value>}
fn child<S>(c:&store::ArtifactChild<S>)->Child{Child{child_id:c.child_id.clone(),artifact_id:c.target.artifact_id.clone(),artifact_kind:c.target.dialect.artifact_kind.clone(),standard:c.target.dialect.standard.clone(),subset:c.target.dialect.subset.clone()}}
fn owned_child<S>(c:Child)->store::ArtifactChild<S>{store::ArtifactChild::new(c.child_id,store::os_io::ArtifactRef{artifact_id:c.artifact_id,dialect:store::os_io::ArtifactDialect{artifact_kind:c.artifact_kind,standard:c.standard,subset:c.subset}})}
fn features<'a>(values:&'a[MapFeature],pending:&mut std::collections::VecDeque<&'a dsl::DslValue>,next:&mut u64)->Vec<Feature>{values.iter().map(|v|{let id=*next;*next+=1;pending.push_back(&v.data);Feature{id:v.id.clone(),value:id}}).collect()}
impl Map{
 fn from_snapshot(snapshot:&GisMapSnapshot)->Self{let mut pending=std::collections::VecDeque::new();let mut next=0;let positions=features(&snapshot.positions,&mut pending,&mut next);let routes=features(&snapshot.routes,&mut pending,&mut next);let regions=features(&snapshot.regions,&mut pending,&mut next);let mut values=Vec::new();while let Some(value)=pending.pop_front(){let mut node=Value{kind:Kind::Null,boolean:None,unsigned:None,signed:None,float:None,text:None,bytes:None,items:vec![],members:vec![]};match value{dsl::DslValue::Null=>{},dsl::DslValue::Bool(value)=>{node.kind=Kind::Boolean;node.boolean=Some(*value)},dsl::DslValue::Number(Number::UInt(value))=>{node.kind=Kind::Unsigned;node.unsigned=Some(*value)},dsl::DslValue::Number(Number::Int(value))=>{node.kind=Kind::Signed;node.signed=Some(*value)},dsl::DslValue::Number(Number::Float(value))=>{node.kind=Kind::Float;node.float=Some(*value)},dsl::DslValue::String(value)=>{node.kind=Kind::Text;node.text=Some(value.clone())},dsl::DslValue::Bytes(value)=>{node.kind=Kind::Bytes;node.bytes=Some(Octets(value.clone()))},dsl::DslValue::Array(items)=>{node.kind=Kind::Array;for value in items{node.items.push(next);next+=1;pending.push_back(value)}},dsl::DslValue::Object(members)=>{node.kind=Kind::Object;for(name,value)in members{node.members.push(Member{name:name.clone(),value:next});next+=1;pending.push_back(value)}}}values.push(node);}Self{positions,routes,regions,drawing:child(&snapshot.drawing),image:snapshot.image.as_ref().map(child),value:child(&snapshot.value),values}}
}
pub(super) fn retire_value(value:dsl::DslValue){let mut pending=vec![value];while let Some(value)=pending.pop(){match value{dsl::DslValue::Array(values)=>pending.extend(values),dsl::DslValue::Object(values)=>pending.extend(values.into_iter().map(|(_,value)|value)),_=>{}}}}
pub(super) fn retire(snapshot:GisMapSnapshot){for value in snapshot.positions.into_iter().chain(snapshot.routes).chain(snapshot.regions){retire_value(value.data)}}
struct Values(Vec<Option<dsl::DslValue>>);
impl Drop for Values{fn drop(&mut self){for value in self.0.iter_mut().filter_map(Option::take){retire_value(value)}}}
struct Items(Vec<dsl::DslValue>);
impl Drop for Items{fn drop(&mut self){for value in self.0.drain(..){retire_value(value)}}}
struct Members(Vec<(String,dsl::DslValue)>);
impl Drop for Members{fn drop(&mut self){for(_,value)in self.0.drain(..){retire_value(value)}}}
struct Features(Vec<MapFeature>);
impl Drop for Features{fn drop(&mut self){for feature in self.0.drain(..){retire_value(feature.data)}}}
fn take(values:&mut Values,key:u64,parent:Option<usize>)->Result<dsl::DslValue,String>{let key=usize::try_from(key).map_err(|_|"GIS native Value index exceeds native domain")?;if key>=values.0.len()||parent.is_some_and(|p|key<=p){return Err("GIS native Value topology differs".into())}values.0[key].take().ok_or_else(||"GIS native Value has repeated ownership".into())}
fn reconstruct(record:Map,control:&mut dsl::NativeDecodeControl<'_>)->Result<GisMapSnapshot,String>{let Map{positions,routes,regions,drawing,image,value,values}=record;let mut units=values.len();control.begin_stage(values.len())?;for node in&values{control.step()?;units=units.checked_add(node.items.len()).and_then(|v|v.checked_add(node.members.len())).ok_or("GIS native work count overflow")?;}control.begin_stage(units)?;let mut owned=Values(control.allocate_vec::<Option<dsl::DslValue>>(values.len())?);owned.0.resize_with(values.len(),||None);
 for(index,node)in values.into_iter().enumerate().rev(){control.step()?;let Value{kind,boolean,unsigned,signed,float,text,bytes,items,members}=node;let value=match(kind,boolean,unsigned,signed,float,text,bytes,items.is_empty(),members.is_empty()){
 (Kind::Null,None,None,None,None,None,None,true,true)=>dsl::DslValue::Null,
 (Kind::Boolean,Some(value),None,None,None,None,None,true,true)=>dsl::DslValue::Bool(value),
 (Kind::Unsigned,None,Some(value),None,None,None,None,true,true)=>dsl::DslValue::Number(Number::UInt(value)),
 (Kind::Signed,None,None,Some(value),None,None,None,true,true)=>dsl::DslValue::Number(Number::Int(value)),
 (Kind::Float,None,None,None,Some(value),None,None,true,true)=>dsl::DslValue::Number(Number::Float(value)),
 (Kind::Text,None,None,None,None,Some(value),None,true,true)=>dsl::DslValue::String(value),
 (Kind::Bytes,None,None,None,None,None,Some(value),true,true)=>dsl::DslValue::Bytes(value.0),
 (Kind::Array,None,None,None,None,None,None,_,true)=>{let mut result=Items(control.allocate_vec::<dsl::DslValue>(items.len())?);for key in items{control.step()?;result.0.push(take(&mut owned,key,Some(index))?);}dsl::DslValue::Array(std::mem::take(&mut result.0))},
 (Kind::Object,None,None,None,None,None,None,true,_)=>{let mut result=Members(control.allocate_vec::<(String,dsl::DslValue)>(members.len())?);for member in members{control.step()?;result.0.push((member.name,take(&mut owned,member.value,Some(index))?));}dsl::DslValue::Object(std::mem::take(&mut result.0))},_=>return Err("GIS native Value has unrelated variant fields".into())};owned.0[index]=Some(value);
 }
 let count=positions.len().checked_add(routes.len()).and_then(|v|v.checked_add(regions.len())).ok_or("GIS feature count overflow")?;control.begin_stage(count)?;
 let mut restore=|features:Vec<Feature>|->Result<Features,String>{let mut result=Features(control.allocate_vec::<MapFeature>(features.len())?);for feature in features{control.step()?;result.0.push(MapFeature{id:feature.id,data:take(&mut owned,feature.value,None)?});}Ok(result)};
 let mut positions=restore(positions)?;let mut routes=restore(routes)?;let mut regions=restore(regions)?;if owned.0.iter().any(Option::is_some){return Err("GIS native Value has unowned records".into())}control.checkpoint()?;Ok(GisMapSnapshot{positions:std::mem::take(&mut positions.0),routes:std::mem::take(&mut routes.0),regions:std::mem::take(&mut regions.0),drawing:owned_child(drawing),image:image.map(owned_child),value:owned_child(value)})
}
fn ordinary(record:Map)->Result<GisMapSnapshot,String>{let mut callback=|_:protocol::native_decoding::NativeDecodeProgress|true;let mut control=dsl::NativeDecodeControl::new(usize::MAX,&mut callback);reconstruct(record,&mut control)}
impl store::ArtifactDsl for GisMapSnapshot{
 const EXTENSION:&'static str="gismap";fn envelope_id()->&'static str{"gis.gismap"}
 fn print_dsl(&self)->String{let body=dsl::print(&Map::from_snapshot(self).__dsl_to_record(),&Map::__dsl_spec(),dsl::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("GIS map identity");store::semio_format::wrap_text(&envelope,&body)}
 fn parse_dsl(text:&str)->Result<Self,store::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|e|dsl::__rt::field_error(e.to_string()))?;if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(dsl::__rt::field_error("GIS map native Text identity differs"))}let record=dsl::parse_exact(body,&Map::__dsl_spec(),&dsl::ParseOptions{limits:dsl::Limits::default(),mode:dsl::SourceMode::Document})?;ordinary(Map::__dsl_from_record(&record)?).map_err(dsl::__rt::field_error)}
}
impl store::ArtifactPack for GisMapSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn record_spec()->Option<dsl::RecordSpec>{Some(Map::__dsl_spec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{let raw=store::pack_rt::encode_document(&Map::__dsl_spec(),&Map::from_snapshot(self).__dsl_to_record(),options)?;let envelope=store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1).map_err(|e|store::PackError::Schema(e.to_string()))?;Ok(store::semio_format::wrap_binary(&envelope,&raw))}
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{let(envelope,raw)=store::semio_format::unwrap_binary(bytes).map_err(|e|store::PackError::Schema(e.to_string()))?;if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(),store::semio_format::Component::Pack,1){return Err(store::PackError::Schema("GIS map native Pack identity differs".into()))}let(record,_)=store::pack_rt::decode_document(&raw,&Map::__dsl_spec(),options)?;ordinary(Map::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?).map_err(store::PackError::Schema)}
}

pub(super) fn reconstruct_record_controlled(source:&dsl::RecordValue,control:&mut dsl::NativeDecodeControl<'_>,maximum_rows:usize)->Result<GisMapSnapshot,String>{
 fn list(source:&dsl::RecordValue,id:u16)->Result<&[dsl::FieldValue],String>{match source.fields.get(&id){Some(dsl::FieldValue::List(values))=>Ok(values),_=>Err("GIS logical list differs".into())}}
 let values=list(source,6)?;let feature_count=list(source,0)?.len().checked_add(list(source,1)?.len()).and_then(|count|list(source,2).ok().and_then(|v|count.checked_add(v.len()))).ok_or("GIS feature count overflow or list differs")?;
 let image=match source.fields.get(&4){Some(dsl::FieldValue::Absent)|None=>0,Some(dsl::FieldValue::Record(_))=>1,_=>return Err("GIS optional image record differs".into())};
 let mut rows=values.len().checked_mul(2).and_then(|count|count.checked_add(feature_count)).and_then(|count|count.checked_add(3+image)).ok_or("GIS native row count overflow")?;if rows>maximum_rows{return Err("GIS native row limit exceeded".into())}
 control.scoped_stage(|control|->Result<(),String>{control.begin_stage(values.len())?;for value in values{control.step()?;let record=match value{dsl::FieldValue::Record(record)=>record,_=>return Err("GIS native Value record differs".into())};let octets=match record.fields.get(&6){Some(dsl::FieldValue::Absent)|None=>0,Some(dsl::FieldValue::Bytes64(bytes))=>bytes.len(),_=>return Err("GIS native octet field differs".into())};rows=rows.checked_add(octets).and_then(|count|list(record,7).ok().and_then(|v|count.checked_add(v.len()))).and_then(|count|list(record,8).ok().and_then(|v|count.checked_add(v.len()))).ok_or("GIS native relationship count overflow or list differs")?;if rows>maximum_rows{return Err("GIS native row limit exceeded".into())}}Ok(())})?;
 reconstruct(Map::__dsl_from_record_controlled(source,control).map_err(|e|e.message)?,control)
}
