import Parser from "web-tree-sitter";
import {execFileSync} from "node:child_process";
import {readFileSync,writeFileSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
await Parser.init();
const parser=new Parser();parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",process.cwd())),"out/tree-sitter-rust.wasm")));
const ticket=resolve(import.meta.dir,".."),changed:string[]=[],semantic:string[]=[];
const files=execFileSync("rg",["--files","-g","*.rs","🧰️framework","✏️s"],{encoding:"utf8",maxBuffer:32*1024*1024}).trim().split("\n");
for(const file of files){
 if(file.includes("/🗿️artifact-reference/")||file.includes("/🔮️oracles/"))continue;
 let source=readFileSync(file,"utf8");if(!/\b(?:to_uri|parse_uri(?:_controlled)?|to_coordinate|parse_coordinate)\s*\(/.test(source)||!/\b(?:ArtifactRef|ArtifactDialect|DialectCoordinateText)\b/.test(source))continue;
 const pure=file.includes("/🧬️schema/")&&!file.includes("/🚪️io/")&&!file.includes("/🧪️tests/");if(pure){semantic.push(file);continue;}
 const tree=parser.parse(source)!;const edits:{start:number,text:string}[]=[];
 const walk=(node:any)=>{
  if(node.type==="function_item"){
   const body=node.childForFieldName("body");if(body){const native:string[]=[];if(/\b(?:to_uri|parse_uri(?:_controlled)?)\s*\(/.test(body.text))native.push("ArtifactReferenceText as _");if(/\b(?:to_coordinate|parse_coordinate)\s*\(/.test(body.text))native.push("DialectCoordinateText as _");if(native.length&&!body.text.includes("semio_framework_artifact_reference::io::text::artifact_reference"))edits.push({start:body.startIndex+1,text:"\nuse semio_framework_artifact_reference::io::text::artifact_reference::{"+native.join(",")+"};\n"});}return;
  }
  for(const child of node.namedChildren)walk(child);
 };walk(tree.rootNode);
 if(!edits.length)continue;
 for(const edit of edits.sort((a,b)=>b.start-a.start))source=source.slice(0,edit.start)+edit.text+source.slice(edit.start);
 if(parser.parse(source)!.rootNode.hasError()&&!tree.rootNode.hasError())throw Error("Native trait import caused parse error: "+file);
 writeFileSync(file,source);changed.push(file);
}
writeFileSync(join(ticket,"artifact-reference-native-trait-routing.md"),"# Explicit Native Reference Extensions\n\nPhysical text parsing and formatting are available only through explicit IO text extension imports. These imports are confined to native, host and testing functions. Pure mutation consumers require typed-target correction; no IO trait was added there.\n\n## Updated Native Consumers\n\n"+changed.sort().map(f=>"- \`"+f+"\`").join("\n")+"\n\n## Pure Consumers Requiring Correction\n\n"+semantic.sort().map(f=>"- \`"+f+"\`").join("\n")+"\n");
process.stdout.write(JSON.stringify({changed:changed.length,semantic:semantic.length})+"\n");
