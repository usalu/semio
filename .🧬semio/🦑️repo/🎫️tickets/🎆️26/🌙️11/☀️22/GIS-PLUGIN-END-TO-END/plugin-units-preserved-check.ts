import{readFileSync,writeFileSync}from"node:fs";import{resolve}from"node:path";import{createHash}from"node:crypto";
import{readModuleTreeSource}from"@semio-tech/framework-compiler-ts/syntax/module-tree";import Parser from"web-tree-sitter";
const root=resolve(import.meta.dir,"../../../../../../..");const file=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/📋️preserved.json");
const pres=JSON.parse(readFileSync(file,"utf8")),write=process.argv.includes("--write");
await Parser.init({wasmBinary:readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",root))});const p=new Parser();p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",root)));
const a=(s:string)=>(s.match(/\bassert(?:_eq|_ne)?!/g)??[]).length,seen=new Map<string,{path:string;sha:string;n:number}[]>();let total=0;
for(const path of new Set([pres.original,...pres.laws.map((r:any)=>r.source)])){const src=path===pres.original?readModuleTreeSource(resolve(root,path)):readFileSync(resolve(root,path),"utf8");total+=a(src);const t=p.parse(src);
for(const fn of t.rootNode.descendantsOfType("function_item")){let law=false;for(let at=fn.previousNamedSibling;at?.type==="attribute_item";at=at.previousNamedSibling)if(/\btest\b|::async_test\b/.test(at.text))law=true;if(!law)continue;const name=fn.childForFieldName("name")!.text;const r=seen.get(name)??[];r.push({path,sha:createHash("sha256").update(fn.text).digest("hex"),n:a(fn.text)});seen.set(name,r);}}
console.log("total assertions",total,"recorded",pres.originalAssertions+pres.addedAssertions);
const removals:Record<string,string>={ui_tree_stamping_caches_interaction_topology_from_a_domain_bound_tree:"stamp_and_cache_interaction_ui no longer returns the previous topology, so assert!(previous.is_empty()) has no subject"};
const names=new Set(pres.laws.map((r:any)=>r.name));
for(const r of pres.laws){const f=seen.get(r.name);if(!f){console.log("MISSING",r.name);continue}if(f.length>1)console.log("DUP",r.name,f.map(x=>x.path));const x=f[0];if(x.path!==r.source)console.log("MOVED",r.name,r.source,"->",x.path);if(write&&x.n<r.assertions){r.removedAssertions=r.assertions-x.n;r.removalReason=removals[r.name]??"UNEXPLAINED";r.assertions=x.n;r.sha256=x.sha;console.log("REMOVED",r.name,r.removedAssertions);continue}if(x.sha!==r.sha256||x.n!==r.assertions)console.log("CHANGED",r.name,"assert",r.assertions,"->",x.n);
if(write&&x.sha!==r.sha256){{r.sha256=x.sha;r.assertions=x.n;r.source=x.path}}}
for(const[n,f]of seen)if(!names.has(n)){console.log("UNRECORDED",n);if(write)pres.laws.push({name:n,source:f[0].path,sha256:f[0].sha,assertions:f[0].n})}
if(write)pres.addedAssertions=total-pres.originalAssertions;
if(write)writeFileSync(file,JSON.stringify(pres,null,2)+"\n");
