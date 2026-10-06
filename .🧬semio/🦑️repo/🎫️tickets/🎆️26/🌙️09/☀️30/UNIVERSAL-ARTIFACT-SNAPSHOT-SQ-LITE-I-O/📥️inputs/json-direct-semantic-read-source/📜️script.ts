import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {dirname,join} from "node:path";
const root="/Users/ueli/Documents/semio",input=dirname(import.meta.path),owner="🧰️framework/🔨️modules/🎒️pack/🔤️json";
type Pair={path:string,before:string,after:string};
function read(path:string){return readFileSync(join(root,path),"utf8")}
function replace(text:string,before:string,after:string){if(text.split(before).length!==2)throw Error("exact source anchor absent or duplicate: "+before.slice(0,90));return text.replace(before,after)}
function save(name:string,pairs:Pair[],mount:boolean){writeFileSync(join(input,name),JSON.stringify(pairs,null,2)+"\n");if(mount){for(const pair of pairs){if(read(pair.path)!==pair.before)throw Error("guard changed: "+pair.path)}for(const pair of pairs){mkdirSync(dirname(join(root,pair.path)),{recursive:true});writeFileSync(join(root,pair.path),pair.after)}}console.log(JSON.stringify({paths:pairs.length,mounted:mount}))}
const command=process.argv[2];
if(command==="demand"){
 const path=owner+"/🧪️tests/🔬️unit/🦀️.rs",before=read(path);save("demand-guarded-pairs.json",[{path,before,after:before+"\n"+readFileSync(join(input,"law.rs"),"utf8")}],true);
}else if(command==="stage-provider"){
 const path=owner+"/🦀️.rs",before=read(path);let after=before;
 after=replace(after,"pub use borrowed_read_source::{JsonReadSource,JsonBorrowedParseCursor};","pub use borrowed_read_source::{JsonReadSource,JsonBorrowedParseCursor,JsonBorrowedDslCursor,JsonParsedValue};");
 const frameStart=after.indexOf("struct JsonFrame {");const frameEnd=after.indexOf("struct JsonStringScan",frameStart);if(frameStart<0||frameEnd<0)throw Error("frame region absent");
 let frame=after.slice(frameStart,frameEnd).replace("struct JsonFrame {","struct JsonFrame<V:JsonParsedValue> {").replaceAll("JsonCandidates<Value>","JsonCandidates<V>").replaceAll("JsonCandidates<(String,Value)>","JsonCandidates<(String,V)>").replaceAll("Vec<Value>","Vec<V>").replaceAll("Vec<(String,Value)>","Vec<(String,V)>");
 after=after.slice(0,frameStart)+frame+after.slice(frameEnd);
 const start=after.indexOf("pub struct JsonParseCursor {"),end=after.indexOf("impl semio_framework_value::retirement::RetireOwned for Number",start);if(start<0||end<0)throw Error("grammar region absent");
 let grammar=after.slice(start,end);
 grammar=replace(grammar,"pub struct JsonParseCursor {","pub type JsonParseCursor=JsonGrammarCursor<Value>;\n\n/// 🌳️ One retained grammar moves admitted semantic cells directly into its declared first-party output.\npub struct JsonGrammarCursor<V:JsonParsedValue> {");
 grammar=grammar.replaceAll("Vec<JsonFrame>","Vec<JsonFrame<V>>").replaceAll("Option<Value>","Option<V>").replaceAll("JsonCandidates<Value>","JsonCandidates<V>").replace("impl JsonParseCursor {","impl<V:JsonParsedValue> JsonGrammarCursor<V> {");
 grammar=grammar.replaceAll("Value::String","V::json_string").replaceAll("Value::Number","V::json_number").replaceAll("Value::Bool","V::json_bool").replaceAll("Value::Null","V::json_null()");
 grammar=replace(grammar,"Value::Object(Object(frame.members))","V::json_object(frame.members)");
 grammar=replace(grammar,"Value::Array(frame.array)","V::json_array(frame.array)");
 after=after.slice(0,start)+grammar+after.slice(end);
 after=replace(after,"impl semio_framework_value::retirement::RetireOwned for JsonFrame {","impl<V:JsonParsedValue> semio_framework_value::retirement::RetireOwned for JsonFrame<V> {");
 after=replace(after,"impl semio_framework_value::retirement::RetireOwned for JsonParseCursor {","impl<V:JsonParsedValue> semio_framework_value::retirement::RetireOwned for JsonGrammarCursor<V> {");
 const sourcePath=owner+"/📥️decode/🫳️borrowed/🦀️.rs",sourceBefore=read(sourcePath);let source=sourceBefore;
 source=replace(source,"pub struct JsonBorrowedParseCursor<'source,S:JsonReadSource+?Sized>{source:&'source S,parser:super::JsonParseCursor}","pub type JsonBorrowedParseCursor<'source,S> = JsonBorrowedCursor<'source,S,super::Value>;\npub type JsonBorrowedDslCursor<'source,S> = JsonBorrowedCursor<'source,S,semio_framework_value::DslValue>;\npub struct JsonBorrowedCursor<'source,S:JsonReadSource+?Sized,V:JsonParsedValue>{source:&'source S,parser:super::JsonGrammarCursor<V>}");
 source=replace(source,"impl<'source,S:JsonReadSource+?Sized> JsonBorrowedParseCursor<'source,S>{","impl<'source,S:JsonReadSource+?Sized,V:JsonParsedValue> JsonBorrowedCursor<'source,S,V>{");
 source=replace(source,"parser:super::JsonParseCursor::new(policy)","parser:super::JsonGrammarCursor::new(policy)");
 source=replace(source,"Result<Option<super::Value>,JsonError>","Result<Option<V>,JsonError>");
 source+="\n"+readFileSync(join(input,"semantic.rs"),"utf8");
 save("provider-held-pairs.json",[{path,before,after},{path:sourcePath,before:sourceBefore,after:source}],false);
}else if(command==="provider"){
 save("provider-guarded-pairs.json",JSON.parse(readFileSync(join(input,"provider-held-pairs.json"),"utf8")),true);
}else throw Error("unknown command");
