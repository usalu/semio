import{readFileSync,writeFileSync,mkdirSync}from"node:fs";import{dirname,join}from"node:path";
const root="/Users/ueli/Documents/semio",input=dirname(import.meta.path),owner="🧰️framework/🔨️modules/🎒️pack/🔤️json";
type Pair={path:string,before:string,after:string};const read=(path:string)=>{try{return readFileSync(join(root,path),"utf8")}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return"";throw error}};
const replace=(text:string,before:string,after:string)=>{if(text.split(before).length!==2)throw Error("exact anchor absent or duplicate: "+before.slice(0,80));return text.replace(before,after)};
function save(name:string,pairs:Pair[],mount:boolean){writeFileSync(join(input,name),JSON.stringify(pairs,null,2)+"\n");if(mount){for(const pair of pairs)if(read(pair.path)!==pair.before)throw Error("guard changed:"+pair.path);for(const pair of pairs){mkdirSync(dirname(join(root,pair.path)),{recursive:true});writeFileSync(join(root,pair.path),pair.after)}}console.log(JSON.stringify({paths:pairs.length,mounted:mount}))}
const command=process.argv[2];
if(command==="demand"){
 const path=owner+"/🧪️tests/🔬️unit/🦀️.rs",before=read(path),fixturePath=owner+"/🧫️fixtures/🫳️read-limits.json";
 save("demand-guarded-pairs.json",[{path,before,after:before+"\n"+readFileSync(join(input,"law.rs"),"utf8")},{path:fixturePath,before:read(fixturePath),after:readFileSync(join(input,"fixture.json"),"utf8")}],true);
}else if(command==="stage-provider"){
 const path=owner+"/🦀️.rs",before=read(path);let after=before;
 after=replace(after,"JsonBorrowedDslCursor,JsonParsedValue};","JsonBorrowedDslCursor,JsonParsedValue,JsonReadLimits};");
 after=replace(after,"object: bool, state: u8, values: JsonCandidates<V>","object: bool, state: u8, item_count:u64, values: JsonCandidates<V>");
 after=replace(after,"position: usize, validated_position:usize, policy: JsonMemberPolicy","position: usize, validated_position:usize, limits:JsonReadLimits, policy: JsonMemberPolicy");
 after=replace(after,"Self { position:0, validated_position:0, policy, frames:Vec::new()","Self { position:0, validated_position:0, limits:JsonReadLimits{maximum_bytes:u64::MAX,maximum_allocation_bytes:usize::MAX,maximum_depth:MAX_DEPTH as usize,maximum_items:u64::MAX}, policy, frames:Vec::new()");
 after=replace(after,"        for _ in 0..maximum_units{\n            control.checkpoint()?;","        control.scoped_maximum(self.limits.maximum_allocation_bytes,|control|{\n        for _ in 0..maximum_units{\n            control.checkpoint()?;");
 after=replace(after,"        Ok(None)\n    }\n    fn advance<S:JsonReadSource+?Sized>","        Ok(None)\n        })\n    }\n    fn advance<S:JsonReadSource+?Sized>");
 after=replace(after,"        if self.frames.len() as u32>MAX_DEPTH {return Err(JsonError::MaxDepthExceeded(MAX_DEPTH));}","        if self.frames.len()>self.limits.maximum_depth.min(MAX_DEPTH as usize){return Err(JsonError::MaxDepthExceeded(self.limits.maximum_depth.min(MAX_DEPTH as usize)as u32));}\n        let declared_item=self.frames.len().checked_sub(1).filter(|index|matches!(self.frames[*index].state,0|2));\n        if let Some(index)=declared_item{if self.frames[index].item_count>=self.limits.maximum_items{return Err(ValueError::new(ValueRefusalKind::WorkLimit,\"JSON declared collection extent exceeds caller limit\").into());}}");
 after=replace(after,"        Ok(())\n    }\n    fn close_frame(&mut self)","        if let Some(index)=declared_item{self.frames[index].item_count+=1;}\n        Ok(())\n    }\n    fn close_frame(&mut self)");
 after=replace(after,"if self.frames.capacity()==0 {self.frames=control.allocate_vec(MAX_DEPTH as usize+2)?;return Ok(());}","if self.frames.capacity()==0 {self.frames=control.allocate_vec(self.limits.maximum_depth.min(MAX_DEPTH as usize)+2)?;return Ok(());}");
 after=replace(after,"self.frames.push(JsonFrame {object:byte==b'{',state:0,values:Default::default()","self.frames.push(JsonFrame {object:byte==b'{',state:0,item_count:0,values:Default::default()");
 const sourcePath=owner+"/📥️decode/🫳️borrowed/🦀️.rs",sourceBefore=read(sourcePath);let source=sourceBefore;
 source=replace(source,"    pub fn source(&self)->&'source S{self.source}",readFileSync(join(input,"constructor.rs"),"utf8")+"    pub fn source(&self)->&'source S{self.source}");
 source+="\n"+readFileSync(join(input,"limits.rs"),"utf8");
 save("provider-held-pairs.json",[{path,before,after},{path:sourcePath,before:sourceBefore,after:source}],false);
}else if(command==="provider"){save("provider-guarded-pairs.json",JSON.parse(readFileSync(join(input,"provider-held-pairs.json"),"utf8")),true);}
else throw Error("unknown command");
