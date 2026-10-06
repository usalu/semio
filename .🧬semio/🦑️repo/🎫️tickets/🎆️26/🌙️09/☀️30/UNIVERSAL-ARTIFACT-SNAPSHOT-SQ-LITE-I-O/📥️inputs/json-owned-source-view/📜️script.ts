import{readFileSync,writeFileSync,mkdirSync}from"node:fs";import{dirname,join}from"node:path";
const root="/Users/ueli/Documents/semio",input=dirname(import.meta.path),owner="🧰️framework/🔨️modules/🎒️pack/🔤️json";
type Pair={path:string,before:string,after:string};const read=(path:string)=>readFileSync(join(root,path),"utf8");
const replace=(text:string,before:string,after:string)=>{if(text.split(before).length!==2)throw Error("exact original source cursor anchor: "+before.slice(0,100));return text.replace(before,after)};
function save(name:string,pairs:Pair[],mount:boolean){writeFileSync(join(input,name),JSON.stringify(pairs,null,2)+"\n");if(mount){for(const pair of pairs)if(read(pair.path)!==pair.before)throw Error("fresh guard "+pair.path);for(const pair of pairs){mkdirSync(dirname(join(root,pair.path)),{recursive:true});writeFileSync(join(root,pair.path),pair.after)}}console.log("[DEBUG] "+JSON.stringify({paths:pairs.length,mounted:mount}))}
const command=process.argv[2];
if(command==="demand"){
 const path=owner+"/🧪️tests/🔬️unit/🦀️.rs",before=read(path);if(before.includes("retained_json_cursor_owns_exact_borrowed_source_view"))throw Error("demand already mounted");save("demand-guarded-pairs.json",[{path,before,after:before+"\n"+readFileSync(join(input,"law.rs"),"utf8")}],true);
}else if(command==="stage-provider"){
 const path=owner+"/📥️decode/🫳️borrowed/🦀️.rs",before=read(path);let after=before;
 after=replace(after,"pub struct JsonBorrowedCursor<'source,S:JsonReadSource+?Sized,V:JsonParsedValue>{source:&'source S,parser:super::JsonGrammarCursor<V>}","pub type JsonBorrowedCursor<'source,S,V>=JsonSourceCursor<&'source S,V>;\n/// 🪟️ Retains an immutable first-party source view by value alongside its original grammar owner.\npub struct JsonSourceCursor<S:JsonReadSource,V:JsonParsedValue>{source:S,parser:super::JsonGrammarCursor<V>}");
 after=replace(after,"impl<'source,S:JsonReadSource+?Sized,V:JsonParsedValue> JsonBorrowedCursor<'source,S,V>{","impl<S:JsonReadSource,V:JsonParsedValue> JsonSourceCursor<S,V>{");
 after=replace(after,"pub fn new(source:&'source S,policy:","pub fn new(source:S,policy:");
 after=replace(after,"pub fn new_with_limits(source:&'source S,policy:","pub fn new_with_limits(source:S,policy:");
 after=replace(after,"pub fn source(&self)->&'source S{self.source}","pub fn source_ref(&self)->&S{&self.source}");
 after=replace(after,"self.parser.step_source(self.source,maximum_units,control)","self.parser.step_source(&self.source,maximum_units,control)");
 after=replace(after,"/// 📏️ One immutable finite source", "impl<'source,S:JsonReadSource+?Sized,V:JsonParsedValue> JsonSourceCursor<&'source S,V>{pub fn source(&self)->&'source S{self.source}}\n\n/// 📏️ One immutable finite source");
 after=replace(after,"impl JsonReadSource for str {","impl<S:JsonReadSource+?Sized> JsonReadSource for &S{fn byte_len(&self)->usize{(**self).byte_len()}fn byte_at(&self,index:usize)->Option<u8>{(**self).byte_at(index)}}\nimpl JsonReadSource for str {");
 const rootPath=owner+"/🦀️.rs",rootBefore=read(rootPath),rootAfter=replace(rootBefore,"JsonParsedValue,JsonReadLimits};","JsonParsedValue,JsonReadLimits,JsonSourceCursor};");
 save("provider-held-pairs.json",[{path,before,after},{path:rootPath,before:rootBefore,after:rootAfter}],false);
}else if(command==="copy-authority"){
 const path=owner+"/📥️decode/🫳️borrowed/🦀️.rs",before=read(path);let after=replace(before,"pub struct JsonSourceCursor<S:JsonReadSource,V:JsonParsedValue>","pub struct JsonSourceCursor<S:JsonReadSource+Copy,V:JsonParsedValue>");after=replace(after,"impl<S:JsonReadSource,V:JsonParsedValue> JsonSourceCursor<S,V>","impl<S:JsonReadSource+Copy,V:JsonParsedValue> JsonSourceCursor<S,V>");
 const unit=owner+"/🧪️tests/🔬️unit/🦀️.rs",unitBefore=read(unit),unitAfter=replace(unitBefore,"    struct OriginalView<'source>","    #[derive(Clone,Copy)]\n    struct OriginalView<'source>");save("copy-authority-guarded-pairs.json",[{path,before,after},{path:unit,before:unitBefore,after:unitAfter}],true);
}else if(command==="neutral-readback"){
 const [pair]=JSON.parse(readFileSync(join(input,"demand-guarded-pairs.json"),"utf8"))as Pair[];
 const before=pair.after,after=read(pair.path);const law=readFileSync(join(input,"law.rs"),"utf8");if(!after.endsWith(law))throw Error("exact mounted neutral source law");save("neutral-demand-guarded-readback.json",[{path:pair.path,before,after}],false);
}else if(command==="provider"){save("provider-guarded-pairs.json",JSON.parse(readFileSync(join(input,"provider-held-pairs.json"),"utf8")),true)}else throw Error("exact demand, stage-provider or provider required");
