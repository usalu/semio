/** 🔎 Lists preserved-law sha256 pins that no longer match the actual test function bodies. */
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import{createHash}from"node:crypto";
import Parser from"web-tree-sitter";
const root="/Users/ueli/Documents/semio";
const unitsDir=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units");
const preservation=JSON.parse(readFileSync(resolve(unitsDir,"📋️preserved.json"),"utf8"));
await Parser.init({wasmBinary:readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",root))});
const p=new Parser();p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",root)));
const actual=new Map<string,{sha256:string,assertions:number}>();
for(const path of new Set(preservation.laws.map((row:{source:string})=>row.source))){
 const tree=p.parse(readFileSync(resolve(root,path as string),"utf8"));
 for(const fn of tree.rootNode.descendantsOfType("function_item")){
  let isLaw=false;for(let attr=fn.previousNamedSibling;attr?.type==="attribute_item";attr=attr.previousNamedSibling)if(/\btest\b|::async_test\b/.test(attr.text))isLaw=true;
  if(!isLaw)continue;const name=fn.childForFieldName("name")?.text as string;
  actual.set(path+"|"+name,{sha256:createHash("sha256").update(fn.text).digest("hex"),assertions:(fn.text.match(/\bassert(?:_eq|_ne)?!/g)??[]).length});
 }
}
let bad=0;
for(const row of preservation.laws){
 const a=actual.get(row.source+"|"+row.name);
 if(!a){bad++;console.log("MISSING",row.name,row.source);continue;}
 if(a.sha256!==row.sha256||a.assertions!==row.assertions){bad++;console.log("DIFF",row.name,"\n  file:",row.source,"\n  pinned:",row.sha256,row.assertions,"\n  actual:",a.sha256,a.assertions);}
}
console.log("bad="+bad,"laws="+preservation.laws.length);
