import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
const root=process.cwd(), ticket=dirname(import.meta.dir), require=createRequire(import.meta.url);
const api=require("nx/src/plugins/js/lock-file/bun-parser"), lock=readFileSync(join(root,"bun.lock"),"utf8"), digest=createHash("sha256").update(lock).digest("hex");
const start=performance.now(), nodes=api.getBunTextLockfileNodes(lock,digest), graph=JSON.parse(readFileSync(join(root,".nx/workspace-data/project-graph.json"),"utf8"));
const context={workspaceRoot:root,externalNodes:nodes,projects:Object.fromEntries(Object.entries(graph.nodes).map(([id,node]:[string,any])=>[id,node.data])),fileMap:{projectFileMap:{},nonProjectFiles:[]}};
const dependencies=api.getBunTextLockfileDependencies(lock,digest,context);
const packages=["sharp","pdfjs-dist","@napi-rs/canvas","d3-force"], observed=packages.map(name=>{
 const node=nodes["npm:"+name];assert.ok(node,`Missing external node ${name}`);
 const version=JSON.parse(readFileSync(join(root,"node_modules",name,"package.json"),"utf8")).version;assert.equal(node.data.version,version,name);
 const seen=new Set<string>(); const visit=(id:string):void=>{if(seen.has(id))return;seen.add(id);for(const edge of dependencies.filter((edge:any)=>edge.source===id))visit(edge.target);};visit(node.name);
 return {name,version,node:node.name,direct:dependencies.filter((edge:any)=>edge.source===node.name).length,closure:seen.size};
});
const result={milliseconds:performance.now()-start,nodes:Object.keys(nodes).length,dependencies:dependencies.length,observed};
writeFileSync(join(ticket,"🗑️generated/external-dependency-discovery.json"),JSON.stringify(result,null,2));console.log("[DEBUG] Installed Nx Bun lockfile parser",JSON.stringify(result));

const { HashPlanner, transferProjectGraph }=require("nx/src/native"), { transformProjectGraphForRust }=require("nx/src/native/transform-objects");
const plannerGraph={nodes:{probe:{name:"probe",type:"lib",data:{root:"probe",targets:{build:{executor:"nx:run-commands",inputs:[{externalDependencies:packages}],options:{command:"echo probe"}}}}}},externalNodes:nodes,dependencies:{probe:[],...Object.fromEntries(Object.keys(nodes).map(id=>[id,dependencies.filter((edge:any)=>edge.source===id)]))}};
const planner=new HashPlanner({},transferProjectGraph(transformProjectGraphForRust(plannerGraph)));
const plan=planner.getPlans(["probe:build"],{roots:["probe:build"],tasks:{"probe:build":{id:"probe:build",target:{project:"probe",target:"build"},projectRoot:"probe",overrides:{},outputs:[],cache:true,parallelism:true}},dependencies:{"probe:build":[]},continuousDependencies:{}});
console.log("[DEBUG] Actual Nx native scoped hash plan",JSON.stringify(plan));
const importers=["🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts","🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📜️script.ts"];
for(const importer of importers){const moduleRequire=createRequire(join(root,importer));console.log("[DEBUG] PDF resolution",importer,moduleRequire.resolve("pdfjs-dist/package.json"),moduleRequire("pdfjs-dist/package.json").version);}

const {cacheInternals}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"));
const parsed=require("nx/src/utils/json").parseJson(lock), patches=Object.fromEntries(Object.values(parsed.patchedDependencies??{}).map((path:any)=>[path,readFileSync(join(root,path))]));
const ownStart=performance.now(), model=cacheInternals.bunLockGraph(parsed,patches);
const ownObserved=[...packages,"@semio-tech/print/pdfjs-dist"].map(key=>{
 const visited=new Set<string>();const visit=(id:string):void=>{if(visited.has(id))return;visited.add(id);for(const edge of model.dependencies.filter((edge:any)=>edge.source===id))visit(edge.target);};visit("npm:"+key);
 return {key,version:model.externalNodes["npm:"+key].data.version,closure:visited.size};
});
console.log("[DEBUG] Location-sensitive graph",JSON.stringify({milliseconds:performance.now()-ownStart,nodes:Object.keys(model.externalNodes).length,dependencies:model.dependencies.length,observed:ownObserved}));
