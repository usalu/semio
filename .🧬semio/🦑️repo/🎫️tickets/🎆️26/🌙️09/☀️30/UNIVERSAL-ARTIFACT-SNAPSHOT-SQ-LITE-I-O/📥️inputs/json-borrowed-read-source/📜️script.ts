import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {dirname,join} from "node:path";
const root="/Users/ueli/Documents/semio";
const input=dirname(import.meta.path);
const owner="🧰️framework/🔨️modules/🎒️pack/🔤️json";
type Pair={path:string,before:string,after:string};
function read(path:string){try{return readFileSync(join(root,path),"utf8")}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return "";throw error}}
function replace(text:string,before:string,after:string){if(text.split(before).length!==2)throw Error("exact source anchor absent or duplicate: "+before.slice(0,90));return text.replace(before,after)}
function pairs(name:string,values:Pair[],mount:boolean){writeFileSync(join(input,name),JSON.stringify(values,null,2)+"\n");if(mount){for(const pair of values){if(read(pair.path)!==pair.before)throw Error("guard changed: "+pair.path)}for(const pair of values){mkdirSync(dirname(join(root,pair.path)),{recursive:true});writeFileSync(join(root,pair.path),pair.after)}}console.log(JSON.stringify({paths:values.length,mounted:mount}))}
const command=process.argv[2];
if(command==="demand"){
 const path=owner+"/🧪️tests/🔬️unit/🦀️.rs";const before=read(path);
 const fixturePath=owner+"/🧫️fixtures/🫳️read-source.json";
 pairs("demand-guarded-pairs.json",[{path,before,after:before+"\n"+readFileSync(join(input,"law.rs"),"utf8")},{path:fixturePath,before:read(fixturePath),after:readFileSync(join(input,"fixture.json"),"utf8")}],true);
}else if(command==="source-demand"){
 const path=owner+"/🧪️tests/🧱️ownership/🟦️.ts";const before=read(path);
 const schemaPath=owner+"/🧬️schema/🫳️read-source/🔣️.json";
 pairs("source-demand-guarded-pairs.json",[{path,before,after:before+"\n"+readFileSync(join(input,"source-law.ts"),"utf8")},{path:schemaPath,before:read(schemaPath),after:readFileSync(join(input,"schema.json"),"utf8")}],true);
}else if(command==="source-type-join"){
 const path=owner+"/🧪️tests/🧱️ownership/🟦️.ts";const before=read(path);
 pairs("source-type-join-guarded-pairs.json",[{path,before,after:replace(before,'const syntax = (node: Node, text: string): unknown =>','const syntax = (node: Node, text: string): Awaited<ReturnType<typeof decodeJsonSyntax>> =>')}],true);
}else if(command==="fixture-joins"){
 const path=owner+"/🧪️tests/🔬️unit/🦀️.rs";const before=read(path);let after=before;
 after=replace(after,'include_str!("../../../🧫️fixtures/🫳️read-source.json")','include_str!("../../🧫️fixtures/🫳️read-source.json")');
 after=replace(after,'&stringify(&parsed)','&to_json_string(&parsed)');
 after=replace(after,'error.into_value_error().kind()','error.into_value_error().kind');
 pairs("fixture-join-guarded-pairs.json",[{path,before,after}],true);
}else if(command==="bound-source-demand"){
 const path=owner+"/🧪️tests/🔬️unit/🦀️.rs";const before=read(path);const previous:Pair[]=JSON.parse(readFileSync(join(input,"fixture-join-guarded-pairs.json"),"utf8"));const old=previous[0].after;const start=old.indexOf("#[test]\nfn retained_json_borrowed_source_");if(start<0)throw Error("own source law absent");
 pairs("bound-source-demand-guarded-pairs.json",[{path,before,after:replace(before,old.slice(start),readFileSync(join(input,"law.rs"),"utf8").trimStart())}],true);
}else if(command==="stage-provider"){
 const path=owner+"/🦀️.rs";const before=read(path);let after=before;
 const producerPath=owner+"/📥️decode/🫳️borrowed/🦀️.rs";
 after=replace(after,"//#region 🔖️Lexer","#[path = \"📥️decode/🫳️borrowed/🦀️.rs\"]\nmod borrowed_read_source;\npub use borrowed_read_source::{JsonReadSource,JsonBorrowedParseCursor};\n\n//#region 🔖️Lexer");
 after=replace(after,"fn finish(&self,input:&str)->Result<Number,JsonError> {","fn finish<S:JsonReadSource+?Sized>(&self,input:&S)->Result<Number,JsonError> {");
 const start=after.indexOf("        let text=&input[self.start..self.position];");const end=after.indexOf("        if !value.is_finite()",start);if(start<0||end<0)throw Error("number source anchors absent");
 after=after.slice(0,start)+readFileSync(join(input,"number.rs"),"utf8")+after.slice(end);
 after=replace(after,"fn step(&mut self,input:&str)->Result<Option<Number>,JsonError> {\n        let byte=input.as_bytes().get(self.position).copied();","fn step<S:JsonReadSource+?Sized>(&mut self,input:&S)->Result<Option<Number>,JsonError> {\n        let byte=input.byte_at(self.position);");
 after=after.replace("i64::try_from(input.len()).unwrap_or(i64::MAX-400)","i64::try_from(input.byte_len()).unwrap_or(i64::MAX-400)");
 after=replace(after,"position: usize, policy: JsonMemberPolicy, frames: Vec<JsonFrame>","position: usize, validated_position:usize, policy: JsonMemberPolicy, frames: Vec<JsonFrame>");
 after=replace(after,"Self { position:0, policy, frames:Vec::new()","Self { position:0, validated_position:0, policy, frames:Vec::new()");
 const old="    pub fn step(&mut self, input:&str, maximum_units:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Option<Value>,JsonError> {\n        for _ in 0..maximum_units {\n            control.checkpoint()?;\n            if self.complete { return Ok(self.result.take()); }\n            self.advance(input,control)?;control.step()?;\n            if self.complete { return Ok(self.result.take()); }\n        }\n        Ok(None)\n    }\n    fn advance(&mut self,input:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),JsonError> {";
 const next="    pub fn step(&mut self,input:&str,maximum_units:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Option<Value>,JsonError>{\n        self.validated_position=input.len();\n        self.step_source(input,maximum_units,control)\n    }\n    fn step_source<S:JsonReadSource+?Sized>(&mut self,input:&S,maximum_units:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Option<Value>,JsonError>{\n        for _ in 0..maximum_units{\n            control.checkpoint()?;\n            if self.validated_position<input.byte_len(){\n                let character=borrowed_read_source::character(input,self.validated_position)?;\n                self.validated_position+=character.len_utf8();control.step()?;continue;\n            }\n            if self.complete{return Ok(self.result.take());}\n            self.advance(input,control)?;control.step()?;\n            if self.complete{return Ok(self.result.take());}\n        }\n        Ok(None)\n    }\n    fn advance<S:JsonReadSource+?Sized>(&mut self,input:&S,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),JsonError> {";
 after=replace(after,old,next);
 after=replace(after,"                    let mut lexer=Lexer {input,pos:if scan.writing {scan.position}else {self.position}};\n                    let character=json_character(&mut lexer)?;\n                    if scan.writing {scan.position=lexer.pos;}else {self.position=lexer.pos;}","                    let mut position=if scan.writing{scan.position}else{self.position};\n                    let character=borrowed_read_source::json_character(input,&mut position)?;\n                    if scan.writing{scan.position=position;}else{self.position=position;}");
 after=replace(after,"        let byte=input.as_bytes().get(self.position).copied();","        let byte=input.byte_at(self.position);");
 after=replace(after,"if !input[self.position..].starts_with(text)","if !borrowed_read_source::starts_with(input,self.position,text)");
 const characterStart=after.indexOf("fn json_character(lexer: &mut Lexer<'_>)");const characterEnd=after.indexOf("//#endregion 🔖️ToFromValueBridge",characterStart);if(characterStart<0||characterEnd<0)throw Error("character source anchors");
 after=after.slice(0,characterStart)+"fn json_character(lexer:&mut Lexer<'_>)->Result<Option<char>,JsonError>{borrowed_read_source::json_character(lexer.input,&mut lexer.pos)}\n"+after.slice(characterEnd);
 pairs("provider-held-pairs.json",[{path,before,after},{path:producerPath,before:read(producerPath),after:readFileSync(join(input,"producer.rs"),"utf8")}],false);
}else if(command==="provider"){
 const values:Pair[]=JSON.parse(readFileSync(join(input,"provider-held-pairs.json"),"utf8"));pairs("provider-guarded-pairs.json",values,true);
}else throw Error("unknown command");
