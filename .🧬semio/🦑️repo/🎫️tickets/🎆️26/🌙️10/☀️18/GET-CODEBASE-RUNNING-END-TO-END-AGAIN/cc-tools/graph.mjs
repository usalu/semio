import {readFileSync,existsSync} from "node:fs";
export function loadGraph(file){
 const m=JSON.parse(readFileSync(file,"utf8"));
 const byId=new Map(m.packages.map(p=>[p.id,p]));
 const members=new Set(m.workspace_members);
 const deps=new Map();
 for(const n of m.resolve.nodes){
  const name=byId.get(n.id)?.name;
  deps.set(name,{normal:new Set(),dev:new Set()});
 }
 for(const n of m.resolve.nodes){
  const name=byId.get(n.id).name,d=deps.get(name);
  for(const dep of n.deps){const t=byId.get(dep.pkg)?.name;if(!t)continue;
   for(const k of dep.dep_kinds){if(k.kind===null||k.kind==="build")d.normal.add(t);else d.dev.add(t);}}
 }
 return {deps,memberNames:new Set([...members].map(id=>byId.get(id).name))};
}
export function closure(g,name,withDev){
 const seen=new Set();const stack=[name];let first=true;
 while(stack.length){const x=stack.pop();if(seen.has(x))continue;seen.add(x);const d=g.deps.get(x);if(!d)continue;
  for(const t of d.normal)stack.push(t);
  if(first&&withDev)for(const t of d.dev)stack.push(t);first=false;}
 return seen;
}
