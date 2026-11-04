import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {join,resolve,dirname} from "node:path";
import {parse} from "smol-toml";

const contract=await Bun.file(new URL("./1103-norm-workspace.json",import.meta.url)).json();
async function audit(workspace,parse){
 const root=parse(await readFile(join(workspace.workspace,"Cargo.toml"),"utf8")),names=new Map(),inherited=[],invalid=[];
 for(const member of root.workspace.members){
  const path=join(workspace.workspace,member),manifest=parse(await readFile(join(path,"Cargo.toml"),"utf8"));
  assert(!names.has(manifest.package.name),"Unique package identity required");
  names.set(manifest.package.name,member);
  assert.equal(resolve(path,manifest.package.workspace),resolve(workspace.workspace),"Exact declared owner required");
  for(const kind of["dependencies","dev-dependencies","build-dependencies"])for(const[name,dependency]of Object.entries(manifest[kind]??{}))if(dependency.workspace===true)inherited.push({name,importer:manifest.package.name});
 }
 const missing=inherited.filter(row=>!root.workspace.dependencies[row.name]);
 for(const[name,dependency]of Object.entries(root.workspace.dependencies))if(dependency.path){
  const target=parse(await readFile(join(workspace.workspace,dependency.path,"Cargo.toml"),"utf8"));
  if(target.package.name!==(dependency.package??name))invalid.push(name);
 }
 return {workspace:workspace.workspace,members:names.size,missing,invalid,registrations:workspace.registrations.map(row=>({name:row.name,path:root.workspace.dependencies[row.name]?.path??null,member:names.get(row.name)??null}))};
}
const native=await Promise.all(contract.workspaces.map(row=>audit(row,Bun.TOML.parse))),oracle=await Promise.all(contract.workspaces.map(row=>audit(row,parse)));
assert.deepEqual(native,oracle,"Bun and smol-toml must agree");
console.log(JSON.stringify(native.map(({workspace,members,missing,invalid})=>({workspace,members,missing,invalid})),null,2));
for(const[index,row]of native.entries()){
 assert.deepEqual(row.missing,[],"Every inherited dependency must be registered");
 assert.deepEqual(row.invalid,[],"Every path dependency must name its actual package");
 assert.deepEqual(row.registrations,contract.workspaces[index].registrations.map(row=>({...row,member:row.path})),"Canonical mutation members and dependency paths must agree");
}
for(const row of contract.staleDependencies){
 const text=await readFile(row.manifest,"utf8"),manifest=parse(text),source=await readFile(row.source,"utf8");
 assert.deepEqual(manifest,Bun.TOML.parse(text),"Both TOML readers must agree on mutation dependencies");
 assert.equal(manifest.dependencies[row.name],undefined,"Nonexistent unused split dependency must be absent");
 assert(!(manifest.features["test-support"]??[]).some(feature=>feature.startsWith(row.name+"/")),"No feature may forward to an absent split");
 assert.deepEqual([...source.matchAll(/pub mod ([a-z0-9_]+);/g)].map(row=>row[1]),row.modules,"Every implemented local mutation module must remain");
 for(const mount of source.matchAll(/#\[path = "([^"]+)"\]/g))await readFile(resolve(dirname(row.source),mount[1]));
 const declared=new Set(Object.keys(manifest.dependencies).map(name=>name.replaceAll("-","_")));
 for(const imported of source.matchAll(/pub use (semio_[a-z0-9_]+)::/g))assert(declared.has(imported[1]),"Public crate reexports require an actual declared dependency");
}
console.log(JSON.stringify({owners:native.length,members:native.reduce((sum,row)=>sum+row.members,0),mutationParents:contract.staleDependencies.length,status:"passed"}));
