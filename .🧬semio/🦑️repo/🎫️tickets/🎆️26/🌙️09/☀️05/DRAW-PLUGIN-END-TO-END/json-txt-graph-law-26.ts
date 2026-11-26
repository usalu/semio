/** 🧬️ Checks that the JSON converter imports the public TXT schema without pulling the TXT app facade. */
import assert from "node:assert/strict";
import {dirname,join,resolve} from "node:path";
import {parse} from "smol-toml";
import fixture from "./json-txt-graph-law-26.json";
const parent=resolve("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"),root=join(parent,"🧾️json");
const all=new Set([fixture.converterOwner,...fixture.unusedOwners]);
const read=async(path:string)=>parse(await Bun.file(path).text()) as any;
const direct=(manifest:any)=>[fixture.facade,fixture.schemaOwner].filter(name=>Object.hasOwn(manifest.dependencies??{},name));
const workspace=await read(join(root,"Cargo.toml"));
const target=workspace.workspace.dependencies[fixture.schemaOwner];
assert.equal(workspace.workspace.dependencies[fixture.facade],undefined);
assert(target?.path,"A real path is required for the TXT schema dependency");
const ownerPath=resolve(root,target.path),owner=await read(join(ownerPath,"Cargo.toml"));
assert.equal(owner.package.name,fixture.schemaOwner);
assert.match(await Bun.file(join(ownerPath,owner.lib?.path??"🦀️.rs")).text(),new RegExp(`pub use schema::snapshot::${fixture.title}`));
const registry=await Bun.file("📋️build-owners.json").json();
assert.equal(resolve(registry.owners.find((row:any)=>row.name===fixture.schemaOwner).manifest),join(ownerPath,"Cargo.toml"));
const manifests=new Map<string,string>();
for(const path of new Bun.Glob("**/Cargo.toml").scanSync({cwd:root,type:"file"})){
 const manifestPath=join(root,path),manifest=await read(manifestPath),name=manifest.package?.name;
 if(!all.has(name))continue;
 manifests.set(name,manifestPath);
 assert.deepEqual(direct(manifest),name===fixture.converterOwner?[fixture.schemaOwner]:[],name);
}
assert.equal(manifests.size,all.size);
const combined=new Map(Object.entries(fixture.combinedLocks));
assert.equal(combined.size,24);
const converted:string[]=[];
for(const path of new Bun.Glob("**/*.rs").scanSync({cwd:root,type:"file"})){
 const text=await Bun.file(join(root,path)).text();
 assert(!text.includes(`${fixture.facade.replaceAll("-","_")}::`),path);
 if(text.includes(`${fixture.schemaOwner.replaceAll("-","_")}::${fixture.title}`))converted.push(path);
}
assert.equal(converted.length,2);
const mirror=join(dirname(manifests.get(fixture.converterOwner)!),"🪞️mirror","🦀️.rs");
const mounts=await Bun.file(mirror).text();
for(const path of converted)assert(mounts.includes(path.split("🧱️base/").at(-1)!),`Expected compiler mount for ${path}`);
const lockPaths=Bun.argv.slice(2);
if(lockPaths[0]?.endsWith("unit-graph-prototype-00.json")){
 const graph=await Bun.file(lockPaths.shift()!).json();
 const names=graph.units.map((unit:any)=>unit.pkg_id.split("#").at(-1).split("@")[0]);
 const reachable=(revised:boolean)=>{
  const seen=new Set<number>(),pending=[...graph.roots];
  while(pending.length){
   const index=pending.pop()!;
   if(seen.has(index))continue;
   seen.add(index);
   const edge=revised?combined.get(names[index]):undefined;
   for(const dep of graph.units[index].dependencies){
    if(edge&&names[dep.index]===edge.from){
     if(edge.to){
      const replacement=graph.units.findIndex((unit:any,at:number)=>names[at]===edge.to&&unit.platform===graph.units[dep.index].platform&&unit.mode===graph.units[dep.index].mode);
      assert(replacement>=0,edge.to);
      pending.push(replacement);
     }
    }else pending.push(dep.index);
   }
  }
  return seen;
 };
 const original=reachable(false),revised=reachable(true);
 const removed=[...original].filter(index=>!revised.has(index)).map(index=>names[index]).sort();
 assert(removed.includes(fixture.facade),JSON.stringify(removed));
 assert([...revised].some(index=>names[index]===fixture.schemaOwner));
 console.log("[DEBUG] previous Draw WASI graph with projected edges",JSON.stringify({before:original.size,after:revised.size,removed}));
}
for(const lockPath of lockPaths){
 const lock=await read(lockPath),rows=lock.package as any[];
 for(const row of rows){
  const edge=combined.get(row.name);
  if(!edge)continue;
  const dependencies=row.dependencies??[];
  assert(!dependencies.includes(edge.from),`${lockPath}: ${row.name}`);
  if(edge.to)assert(dependencies.includes(edge.to),`${lockPath}: ${row.name}`);
 }
}
console.log(`[DEBUG] two JSON converters, seven JSON package manifests, 24 combined edge rules, ${lockPaths.length} lock graphs`);
