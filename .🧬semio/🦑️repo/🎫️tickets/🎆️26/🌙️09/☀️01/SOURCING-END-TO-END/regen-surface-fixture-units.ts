import{readFileSync,writeFileSync}from"node:fs";
import{resolve}from"node:path";
import{createHash}from"node:crypto";
import Parser from"web-tree-sitter";
const root="/Users/ueli/Documents/semio";
const manifestPath=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/📋️surface-fixture.json");
const sha=(value:string|Buffer)=>createHash("sha256").update(value).digest("hex");
const assertions=(text:string)=>(text.match(/\bassert(?:_eq|_ne)?!/g)??[]).length;
const isLaw=(fn:Parser.SyntaxNode)=>{for(let attr=fn.previousNamedSibling;attr?.type==="attribute_item";attr=attr.previousNamedSibling)if(/\btest\b|::async_test\b/.test(attr.text))return true;return false;};
await Parser.init({wasmBinary:readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",root))});
const parser=new Parser();parser.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",root)));
const manifest=JSON.parse(readFileSync(manifestPath,"utf8"));
const changes:string[]=[];
for(const file of manifest.files){
 const target=file.functions[0]?.target??file.path;
 const bytes=readFileSync(resolve(root,target));
 const next={sha256:sha(bytes),bytes:bytes.length,assertions:assertions(bytes.toString("utf8"))};
 for(const key of Object.keys(next)as(keyof typeof next)[])if(file[key]!==next[key]){changes.push(`${file.path} ${key}`);file[key]=next[key];}
 const tree=parser.parse(readFileSync(resolve(root,target),"utf8"));
 const actual=tree.rootNode.descendantsOfType("function_item").map(fn=>({name:fn.childForFieldName("name")?.text??"",sha256:sha(fn.text),assertions:assertions(fn.text),law:isLaw(fn)}));
 tree.delete();
 const previous=file.functions as any[],used=new Map<string,number>(),next_:any[]=[];
 for(const row of actual){
  const old=previous.filter(fn=>fn.name===row.name),index=used.get(row.name)??0;used.set(row.name,index+1);
  const entry=old[index]??{name:row.name,sha256:"",assertions:0,law:row.law,target};
  if(!old[index])changes.push("ADD "+row.name);else if(entry.sha256!==row.sha256||entry.assertions!==row.assertions)changes.push("UPDATE "+row.name);
  entry.sha256=row.sha256;entry.assertions=row.assertions;next_.push(entry);
 }
 for(const fn of previous)if(!actual.some(row=>row.name===fn.name))changes.push("REMOVE "+fn.name);
 if(target===file.path)file.functions=next_;else for(const row of next_){const kept=previous.find(fn=>fn.name===row.name);if(kept){kept.sha256=row.sha256;kept.assertions=row.assertions}}
}
{const tree=parser.parse(readFileSync(resolve(root,manifest.aggregate.path),"utf8"));const fn=tree.rootNode.descendantsOfType("function_item").find(n=>n.childForFieldName("name")?.text===manifest.aggregate.lawName);if(!fn)throw Error("aggregate law missing");const next=sha(fn.text);if(next!==manifest.aggregate.sha256){changes.push("UPDATE aggregate "+manifest.aggregate.lawName);manifest.aggregate.sha256=next}tree.delete();}
{const tree=parser.parse(readFileSync(resolve(root,manifest.macro.target),"utf8"));const macro=tree.rootNode.descendantsOfType("macro_definition").find(n=>n.childForFieldName("name")?.text==="bounded_first_step_tool_proofs");if(!macro)throw Error("macro missing");const next=sha(macro.text.replaceAll("$crate::tool_contract::ArtifactBoundedFirstStepProof","$crate::ArtifactBoundedFirstStepProof"));if(next!==manifest.macro.sha256){changes.push("UPDATE macro");manifest.macro.sha256=next}tree.delete();}
writeFileSync(manifestPath,JSON.stringify(manifest,null,2)+"\n");
console.log(changes.join("\n"));
