import {readFileSync,writeFileSync,renameSync,mkdirSync,existsSync} from 'node:fs';
import {dirname,join} from 'node:path';
const root='🧰️framework/🔨️modules/🧵️job/⚠️failure/',dir=root+'🫴️projection/🔤️json/',pair='🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/📸️native-snapshot/🫴️receiving/📜️pair/🫴️failure/',generated=join(dirname(import.meta.path),'🗑️generated');
function edit(file:string,f:(s:string)=>string){const old=readFileSync(file,'utf8'),next=f(old),tmp=join(generated,'json-projection-'+Date.now()+'.rs');if(next===old)throw Error('unchanged '+file);writeFileSync(tmp,next);if(readFileSync(file,'utf8')!==old)throw Error('concurrent '+file);renameSync(tmp,file)}
function rep(s:string,a:string,b:string){if(s.split(a).length!==2)throw Error('anchor '+a.slice(0,100));return s.replace(a,b)}
function create(file:string,s:string){if(existsSync(file))throw Error('exists '+file);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,s)}
const oldPair=readFileSync(pair+'🦀️.rs','utf8'),kind=oldPair.split('\n').find(s=>s.startsWith('fn json_kind'))!,child=oldPair.split('\n').find(s=>s.startsWith(' Self::Json(v)=>'))!;
const rows=JSON.parse(readFileSync(pair+'🧪️fixtures/🔣️.json','utf8')).json;
create(dir+'🧪️fixtures/🔣️.json',JSON.stringify({version:1,variants:rows,name:'duplicate 源',offset:73},null,2)+'\n');
create(dir+'🧬️schema/🔣️.json',JSON.stringify({$schema:'https://json-schema.org/draft/2020-12/schema',type:'object',required:['version','variants','name','offset'],additionalProperties:false,properties:{version:{const:1},variants:{type:'array',minItems:12,maxItems:12,items:{type:'array',items:{type:'string'},minItems:2}},name:{type:'string'},offset:{type:'integer',minimum:0}}},null,2)+'\n');
create(dir+'🟦️.ts',`import{test,expect}from'bun:test';import{Database}from'bun:sqlite';import corpus from'./🧪️fixtures/🔣️.json';test('complete JSON original fields agree with independent ordered SQL paths',()=>{const db=new Database(':memory:');try{db.run('CREATE TABLE fields(kind TEXT, ordinal INTEGER, field TEXT, PRIMARY KEY(kind,ordinal))');for(const[kind,...fields]of corpus.variants)for(const[index,field]of fields.entries())db.run('INSERT INTO fields VALUES(?1,?2,?3)',[kind,index,field]);expect(corpus.variants.length).toBe(12);for(const[kind,...fields]of corpus.variants){expect(db.query('SELECT field FROM fields WHERE kind=?1 ORDER BY ordinal').all(kind).map((r:any)=>r.field)).toEqual(fields);expect(new Set(fields).size).toBe(fields.length)}const original={kind:'DuplicateMember',name:corpus.name,offset:corpus.offset};expect(JSON.parse(JSON.stringify(original))).toEqual(original)}finally{db.close()}});
`);
create(dir+'🦀️.rs',`//! 🔤️ Complete borrowed JSON syntax failures preserve original names, offsets and nested native causes.
use super::{OriginalDomainFailure,OriginalDomainProjection,OriginalDomainView as V,ValueError};
use semio_framework_pack_json::JsonError;
use semio_framework_value_retained_clone::{Number,ValueRefusalKind,RetainedCloneProgress};
#[derive(Clone,Copy)]enum Node<'a>{Json(&'a JsonError),Value(&'a ValueError),Text(&'a str),UInt(u64)}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original JSON failure projection ordinal changed")}
${kind}
impl<'a> Node<'a>{
 fn keys(self)->Result<&'static[&'static str],ValueError>{let Self::Json(j)=self else{return Err(invalid())};Ok(match j{JsonError::Native(_)=>&["kind","original"],JsonError::UnexpectedEof|JsonError::InvalidUtf8=>&["kind"],JsonError::UnexpectedByte{..}=>&["kind","found","offset"],JsonError::ControlCharacterInString{..}=>&["kind","byte","offset"],JsonError::DuplicateMember{..}=>&["kind","name","offset"],JsonError::MaxDepthExceeded(_)=>&["kind","depth"],_=>&["kind","offset"]})}
 fn child(self,index:usize)->Result<Self,ValueError>{Ok(match self{
${child}
 _=>return Err(invalid())})}
 fn view(self)->Result<V<'a>,ValueError>{Ok(match self{Self::Json(_)=>V::IntrinsicObject(self.keys()?.len()),Self::Value(v)=>return super::project_original_value_error(v,&[]),Self::Text(v)=>V::IntrinsicText(v),Self::UInt(v)=>V::IntrinsicNumber(Number::UInt(v))})}
}
fn locate<'a>(mut node:Node<'a>,path:&[usize])->Result<(Node<'a>,usize),ValueError>{for(position,&index)in path.iter().enumerate(){if matches!(node,Node::Value(_)){return Ok((node,position))}node=node.child(index)?}Ok((node,path.len()))}
/// 🧾️ Lends every original JSON syntax field without rendering or copying its owned name.
pub fn project_original_json_failure<'a>(error:&'a JsonError,path:&[usize])->Result<V<'a>,ValueError>{let(node,position)=locate(Node::Json(error),path)?;match node{Node::Value(v)=>super::project_original_value_error(v,&path[position..]),node=>node.view()}}
/// 🔑️ Lends each complete original syntax field key at its selected original path.
pub fn key_original_json_failure<'a>(error:&'a JsonError,path:&[usize],index:usize)->Result<&'a str,ValueError>{let(node,position)=locate(Node::Json(error),path)?;match node{Node::Value(v)=>super::key_original_value_error(v,&path[position..],index),node=>node.keys()?.get(index).copied().ok_or_else(invalid)}}
/// 🫴️ Owns the unchanged syntax failure through the common Source-qualified domain receiving port.
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
pub struct OriginalJsonFailure{pub original:JsonError}
impl From<JsonError> for OriginalJsonFailure{fn from(original:JsonError)->Self{Self{original}}}
impl OriginalDomainFailure for OriginalJsonFailure{
 fn domain_code(&self)->&str{json_kind(&self.original)}
 fn domain_message(&self)->&str{match &self.original{JsonError::Native(v)=>&v.message,_=>json_kind(&self.original)}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{match &self.original{JsonError::Native(v)=>v.retained_progress(),_=>Default::default()}}
}
impl OriginalDomainProjection for OriginalJsonFailure{
 fn projection_identity(&self)->usize{&self.original as*const JsonError as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{project_original_json_failure(&self.original,path)}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{key_original_json_failure(&self.original,path,index)}
}
`);
edit(root+'🦀️.rs',s=>s+'\n#[path="🫴️projection/🔤️json/🦀️.rs"]\nmod json_failure_projection;\npub use json_failure_projection::{OriginalJsonFailure,project_original_json_failure,key_original_json_failure};\n');
edit(root+'📦️packages/🦀️rust/Cargo.toml',s=>rep(s,'[dependencies]\n','[dependencies]\nsemio-framework-pack-json = { path = "../../../../🎒️pack/🔤️json/📦️packages/🦀️rust" }\n'));
edit(pair+'🦀️.rs',s=>{
 s=rep(s,'key_original_retirement_failure};','key_original_retirement_failure,project_original_json_failure,key_original_json_failure};');
 s=rep(s,kind+'\n','');
 const jsonKeys='Self::Json(j)=>match j{JsonError::Native(_)=>&["kind","original"],JsonError::UnexpectedEof|JsonError::InvalidUtf8=>&["kind"],JsonError::UnexpectedByte{..}=>&["kind","found","offset"],JsonError::ControlCharacterInString{..}=>&["kind","byte","offset"],JsonError::DuplicateMember{..}=>&["kind","name","offset"],JsonError::MaxDepthExceeded(_)=>&["kind","depth"],_=>&["kind","offset"]},';
 s=rep(s,jsonKeys,'');s=rep(s,child+'\n','');
 s=rep(s,'Self::Value(v)=>return project_original_value_error(v,&[]),','Self::Json(v)=>return project_original_json_failure(v,&[]),Self::Value(v)=>return project_original_value_error(v,&[]),');
 s=rep(s,'matches!(node,Node::Value(_)|','matches!(node,Node::Json(_)|Node::Value(_)|');
 s=rep(s,'match node{Node::Value(v)=>project_original_value_error','match node{Node::Json(v)=>project_original_json_failure(v,&path[position..]),Node::Value(v)=>project_original_value_error');
 s=rep(s,'match node{Node::Value(v)=>key_original_value_error','match node{Node::Json(v)=>key_original_json_failure(v,&path[position..],index),Node::Value(v)=>key_original_value_error');
 return s;
});
console.log('complete JSON domain projection mounted; pair delegates canonical borrowed projection');
