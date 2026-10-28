/** 🔌️ Contract: the repository protocol of the graph generation. A shared reader serves a verified or current generation without ever entering the builder custody; only a stale generation under a fresh policy enters it, the facts resource is held only for the pointer swap, closures read only their own projects, texts come in English and German, and partial inference results are shaped into exactly the facts and edges each owner consumed. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {existsSync,mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {minimatch} from "minimatch";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {GENERATION_MESSAGES_V1,extractGraphPolicyV1,implementationDigestV1,loadGenerationGraphV1,renderGenerationMessageV1,scopeImplementationRowsV1,sliceClosureV1,sliceViewV1,expandInChunksV1,shapeComputeResultV1,survivingStubsV1} from "../🟨️.mjs";
import {GENERATION_POLICY_V1,canonicalJsonV1} from "../../🟨️.mjs";
import {encodeGenerationPinV1,generationPinOpenV1,readGenerationPinV1} from "../../🧷️pin/🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../🧪️tests/🏃️world/🟦️.ts";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"..","..","🧪️tests","🐍️.py");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-load-"));
const clean=(dir:string)=>rmSync(dir,{recursive:true,force:true});
const sleep=(ms:number)=>new Promise<void>(resolve=>setTimeout(resolve,ms));
const pythonGraph=(dir:string,names?:string[])=>{const graph=JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());if(!names)return graph;return{...graph,projects:Object.fromEntries(Object.entries(graph.projects).filter(([name])=>names.includes(name))),dependencies:graph.dependencies.filter((edge:any)=>names.includes(edge.source)&&names.includes(edge.target))};};
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));
const shape=(graph:any)=>normalized({projects:graph.projects,dependencies:graph.dependencies,externalNodes:graph.externalNodes,unavailable:graph.unavailable??[]});

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("the bilingual texts are the declared ones, equal in both languages by placeholder, and rendering fills them",()=>{
 expect(JSON.parse(JSON.stringify(GENERATION_MESSAGES_V1))).toEqual(schema.definitions.Messages.const);
 expect(renderGenerationMessageV1("published",{generation:7,computed:2,seconds:3},"en")).toBe("graph: generation 7 published, 2 projects refreshed in 3 s");
 expect(renderGenerationMessageV1("published",{generation:7,computed:2,seconds:3},"de")).toBe("Graph: Generation 7 veröffentlicht, 2 Projekte in 3 s erneuert");
 expect(()=>renderGenerationMessageV1("other" as never,{},"en")).toThrow();
});


for(const row of fixture.policy)test("policy flag "+JSON.stringify([row.args,row.env]),()=>{
 if((row.expected as {error?:boolean}).error)expect(()=>extractGraphPolicyV1(row.args,row.env)).toThrow();
 else expect(extractGraphPolicyV1(row.args,row.env)).toEqual(row.expected as never);
});

test("shaping routes facts, edges and unclaimed facts exactly like the independent model",()=>{
 const{workspaceRoot,part,stubs,expected}=fixture.shape;
 expect(shapeComputeResultV1({workspaceRoot,part:part as never,stubs:stubs as never})).toEqual(expected as never);
 const owners=shapeComputeResultV1({workspaceRoot,part:part as never,stubs:stubs as never}).owners;
 expect(owners.find(owner=>owner.name==="root")!.rows!.length).toBe(1);
 expect(owners.find(owner=>owner.name==="b")).toMatchObject({partial:"edges"});
});

test("a shared reader never enters the builder; a stale generation does and holds the facts resource only for the swap",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir);
  const cold=await loadGenerationGraphV1(mock.host) as any;
  expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);expect(shape(cold)).toEqual(pythonGraph(dir));expect(mock.said.at(-2)!.text).toContain("published");expect(mock.said.at(-1)!.text).toContain("fact index of generation 0");
  mock.events.length=0;mock.holdsLog.length=0;const warm=await loadGenerationGraphV1(mock.host) as any;
  expect(mock.events).toEqual(["shared"]);expect(mock.holdsLog).toEqual([]);expect(shape(warm)).toEqual(shape(cold));
  await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:'import "@syn/p4";\nexport const v=2;\n'}]);
  mock.events.length=0;mock.holdsLog.length=0;mock.log.length=0;const edited=await loadGenerationGraphV1(mock.host) as any;
  expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);expect(mock.log).toEqual([{full:["p2"],edges:[],cold:false}]);
  expect(mock.holdsLog.map(row=>row.resource)).toEqual(["repository-graph-facts:mock"]);expect(mock.holdsLog[0]!.heldMs).toBeLessThan(100);
  expect(shape(edited)).toEqual(pythonGraph(dir));
 }finally{clean(dir);}
},60000);

test("a current policy serves the newest generation without verification or a builder; without one it builds",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(5));const mock=createMockHost(dir),empty=await loadGenerationGraphV1(mock.host,{policy:"current"}) as any;
  expect(mock.events).toContain("builder:held");expect(shape(empty)).toEqual(pythonGraph(dir));
  await applyOps(dir,[{op:"write",path:"pkg/p1/src/index.ts",content:"export const stale=1;\n"}]);
  mock.events.length=0;mock.said.length=0;const served=await loadGenerationGraphV1(mock.host,{policy:"current"}) as any;
  expect(mock.events).toEqual(["shared"]);expect(mock.said.map(row=>row.text)).toEqual(["graph: serving generation 0 without verification (graph policy current)"]);
  expect(served.projects.p1.sources.length).toBe(1);expect(served.projects.p1.sources[0].path).toBe("pkg/p1/src/index.ts");
  expect(shape(served)).not.toEqual(pythonGraph(dir));
  mock.events.length=0;const fresh=await loadGenerationGraphV1(mock.host) as any;expect(mock.events).toContain("builder:held");expect(shape(fresh)).toEqual(pythonGraph(dir));
  const german=createMockHost(dir,{language:"de"});await loadGenerationGraphV1(german.host,{policy:"current"});
  expect(german.said.map(row=>row.text)).toEqual(["Graph: Generation 1 wird ohne Prüfung geliefert (Graph-Richtlinie current)"]);
 }finally{clean(dir);}
},60000);

test("a closure reads and expands only its own projects and never enters the builder while its facts hold",async()=>{
 const dir=scratch();
 try{
  const files:Record<string,string>={};
  for(const[name,imports]of[["a",["@syn/b"]],["b",["@syn/c"]],["c",[]],["d",["@syn/e"]],["e",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";
  await seed(dir,files);const mock=createMockHost(dir);await loadGenerationGraphV1(mock.host);
  await applyOps(dir,[{op:"write",path:"pkg/e/src/index.ts",content:"export const v=2;\n"}]);
  mock.events.length=0;mock.expanded.length=0;const graph=await loadGenerationGraphV1(mock.host,{closure:["a"]}) as any;
  expect(mock.events).toEqual(["shared"]);expect(mock.expanded).toEqual([["a","b","c"]]);expect(shape(graph)).toEqual(pythonGraph(dir,["a","b","c"]));
  expect(mock.said.at(-1)!.text).toBe("graph: 3 projects of the closure verified, no other project read");
  await applyOps(dir,[{op:"write",path:"pkg/c/src/index.ts",content:"export const v=3;\n"}]);
  mock.events.length=0;mock.log.length=0;const rebuilt=await loadGenerationGraphV1(mock.host,{closure:["a"]}) as any;
  expect(mock.events).toContain("builder:held");expect(mock.log[0]!.full).toEqual(["c"]);expect(shape(rebuilt)).toEqual(pythonGraph(dir,["a","b","c"]));
 }finally{clean(dir);}
},60000);

test("readers are served while a builder holds its custody; a dirty reader queues behind it and then adopts its generation",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(5));const first=createMockHost(dir);await loadGenerationGraphV1(first.host);
  await applyOps(dir,[{op:"write",path:"pkg/p0/src/index.ts",content:"export const v=9;\n"}]);
  let open!:()=>void;const gate=new Promise<void>(resolve=>{open=resolve;}),slow=createMockHost(dir,{gate:()=>gate});
  const builder=loadGenerationGraphV1(slow.host) as Promise<any>;
  while(!slow.events.includes("builder:held"))await new Promise(resolve=>setTimeout(resolve,5));
  const peer=createMockHost(dir),race=await Promise.race([loadGenerationGraphV1(peer.host,{policy:"current"}),new Promise(resolve=>setTimeout(()=>resolve("timeout"),400))]);
  expect(race).not.toBe("timeout");
  const waiting=loadGenerationGraphV1(peer.host,{policy:"fresh"}) as Promise<any>,state={done:false};void waiting.then(()=>{state.done=true;});
  await new Promise(resolve=>setTimeout(resolve,150));
  open();const a=await builder,b=await waiting;
  expect(shape(a)).toEqual(pythonGraph(dir));expect(shape(b)).toEqual(shape(a));
 }finally{clean(dir);}
},60000);

test("progress counters are the shared graph phases and a cancelled load leaves the pointer alone",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(4));const mock=createMockHost(dir);await loadGenerationGraphV1(mock.host);
  expect(mock.published.every(resource=>/^build:graph-phase:(facts|assembly):\d+:\d+$/u.test(resource))).toBe(true);
  expect(mock.published.some(resource=>resource.startsWith("build:graph-phase:assembly:"))).toBe(true);
  await applyOps(dir,[{op:"write",path:"pkg/p1/src/index.ts",content:"export const v=5;\n"}]);
  const controller=new AbortController(),cancelled=createMockHost(dir,{signal:controller.signal});controller.abort(new Error("cancelled"));
  await expect(loadGenerationGraphV1(cancelled.host)).rejects.toThrow("cancelled");
  const after=createMockHost(dir),graph=await loadGenerationGraphV1(after.host) as any;expect(shape(graph)).toEqual(pythonGraph(dir));expect(after.log.length).toBe(1);
 }finally{clean(dir);}
},60000);

for(const[index,row]of fixture.implementation.entries())test("implementation facts are scoped and stamped like the independent model "+index,()=>{
 const scoped=scopeImplementationRowsV1(row.rows as never),expected=row.expected as {all:unknown[];environment:unknown[];digest:string};
 expect({all:scoped.all,environment:scoped.environment}).toEqual({all:expected.all,environment:expected.environment} as never);
 expect(implementationDigestV1(row.rows as never)).toBe(expected.digest);
});

test("shaping adds the implementation facts of the consumers: every project the data files, the environment also the stamps of the inferrers that shape edges",()=>{
 const{workspaceRoot,part,stubs,implementation,expected}=fixture.shapeImplementation;
 expect(shapeComputeResultV1({workspaceRoot,part:part as never,stubs:stubs as never,implementation:implementation as never})).toEqual(expected as never);
});

test("the stale generation text is declared in both languages",()=>{
 expect(renderGenerationMessageV1("stale",{generation:2},"en")).toBe("graph: serving generation 2 built by another implementation (graph policy current)");
 expect(renderGenerationMessageV1("stale",{generation:2},"de")).toBe("Graph: Generation 2 stammt von einer anderen Implementierung und wird geliefert (Graph-Richtlinie current)");
});

const stampTable=(nodes="n/1",core="c/1",edges="e/1")=>({"inferrer:nodes":{stamp:"semio.graph-inferrer.nodes/"+nodes.split("/")[1],scope:"project" as const},"inferrer:core":{stamp:"semio.graph-inferrer.core/"+core.split("/")[1],scope:"all" as const},"inferrer:edges":{stamp:"semio.graph-inferrer.edges/"+edges.split("/")[1],scope:"environment" as const}});
const stampOf=(dir:string):string=>{const pointer=JSON.parse(readFileSync(join(dir,".gen","current.json"),"utf8"));return JSON.parse(readFileSync(join(dir,".gen","m",pointer.manifest+".json"),"utf8")).implementation;};
const brokenRing=(count:number,broken:number)=>{const files=ringFiles(count);for(let i=count-broken;i<count;i++)files[`pkg/p${i}/project.json`]=JSON.stringify({name:`p${i}`,broken:true});return files;};
const names=(count:number)=>Array.from({length:count},(_,i)=>"p"+i).sort();

test("a generation names the stamps of the inferrers, never the code: tool source edits refresh nothing, one project edit one project, a bumped stamp exactly its consumers, and each reason names its input in English and German",async()=>{
 const dir=scratch(),every=names(12),valid=names(10),files={...brokenRing(12,2),"tool/graph.mjs":"1","data/policy.json":"{}"};
 try{
  await seed(dir,files);const options={implementationFiles:["data/policy.json"],stamps:stampTable()};
  const loader=createMockHost(dir,options);await loadGenerationGraphV1(loader.host);
  const stampA=implementationDigestV1(await loader.host.implementationRows());expect(stampOf(dir)).toBe(stampA);expect(loader.log).toEqual([{full:every,edges:[],cold:true}]);expect(loader.said.map(row=>row.key)).toContain("cold");
  await applyOps(dir,[{op:"write",path:"tool/graph.mjs",content:"2"}]);
  const tool=createMockHost(dir,options);await loadGenerationGraphV1(tool.host);
  expect(tool.events).toEqual(["shared"]);expect(tool.log).toEqual([]);expect(stampOf(dir)).toBe(stampA);
  await applyOps(dir,[{op:"write",path:"pkg/p3/project.json",content:JSON.stringify({name:"p3",package:"@syn/p3",uses:[],edit:1})}]);
  const manifest=createMockHost(dir,options);await loadGenerationGraphV1(manifest.host);
  expect(manifest.log).toEqual([{full:["p3"],edges:[],cold:false}]);
  const reason=manifest.said.find(row=>row.key==="reason");expect(reason!.text).toMatch(/^graph: p3 is stale: .*pkg\/p3\/project\.json/u);
  const german=createMockHost(dir,{...options,language:"de"});await applyOps(dir,[{op:"write",path:"pkg/p3/project.json",content:JSON.stringify({name:"p3",package:"@syn/p3",uses:[],edit:2})}]);await loadGenerationGraphV1(german.host);
  expect(german.said.find(row=>row.key==="reason")!.text).toMatch(/^Graph: p3 ist veraltet: .*pkg\/p3\/project\.json/u);
  const nodes=createMockHost(dir,{...options,stamps:stampTable("n/2")});await loadGenerationGraphV1(nodes.host);
  expect(nodes.log).toEqual([{full:every,edges:[],cold:false}]);expect(nodes.said.find(row=>row.key==="reason")!.text).toContain("the inference protocol of nodes changed from semio.graph-inferrer.nodes/1 to semio.graph-inferrer.nodes/2");
  const stampB=implementationDigestV1(await nodes.host.implementationRows());expect(stampB).not.toBe(stampA);expect(stampOf(dir)).toBe(stampB);
  const edges=createMockHost(dir,{...options,stamps:stampTable("n/2","c/1","e/2")});await loadGenerationGraphV1(edges.host);
  expect(edges.log).toEqual([{full:[],edges:[],cold:false}]);expect(edges.said.find(row=>row.key==="reason")!.text).toContain("@env is stale");expect(edges.said.some(row=>row.text.includes("edges changed from"))).toBe(true);
  const core=createMockHost(dir,{...options,stamps:stampTable("n/2","c/2","e/2")});await loadGenerationGraphV1(core.host);
  expect(core.log).toEqual([{full:every,edges:[],cold:false}]);
  await applyOps(dir,[{op:"write",path:"data/policy.json",content:'{"v":2}'}]);
  const data=createMockHost(dir,{...options,stamps:stampTable("n/2","c/2","e/2")});await loadGenerationGraphV1(data.host);
  expect(data.log).toEqual([{full:every,edges:[],cold:false}]);
  loader.events.length=0;loader.said.length=0;const stale=await loadGenerationGraphV1(loader.host,{policy:"current"}) as any;
  expect(loader.events).toEqual(["shared"]);expect(loader.said.map(row=>row.text)).toEqual(["graph: serving generation "+(JSON.parse(readFileSync(join(dir,".gen","current.json"),"utf8")).generation)+" built by another implementation (graph policy current)"]);expect(Object.keys(stale.projects).length).toBe(10);
  loader.events.length=0;await loadGenerationGraphV1(loader.host);expect(loader.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);expect(valid.length).toBe(10);
 }finally{clean(dir);}
},120000);

test("a tool source edited every few hundred milliseconds never restarts a 300 project build and never refreshes a project afterwards",async()=>{
 const count=300,broken=30,files={...brokenRing(count,broken),"tool/graph.mjs":"0"};
 const quiet=scratch(),busy=scratch(),options={stamps:stampTable(),ownerDelayMs:8};
 try{
  await seed(quiet,files);const timed=createMockHost(quiet,options),started=Date.now();await loadGenerationGraphV1(timed.host);const quietMs=Date.now()-started;
  await seed(busy,files);const loaded=createMockHost(busy,options),loadedStamp=implementationDigestV1(await loaded.host.implementationRows());
  let editing=true,edits=0;const editor=(async()=>{while(editing){await sleep(250);edits++;await writeFile(join(busy,"tool/graph.mjs"),String(edits));}})();
  const begin=Date.now(),graph=await loadGenerationGraphV1(loaded.host) as any,busyMs=Date.now()-begin;editing=false;await editor;
  expect(edits).toBeGreaterThanOrEqual(3);expect(loaded.log.length).toBe(1);expect(loaded.said.at(-2)!.text).toContain("published");expect(loaded.said.at(-1)!.text).toContain("fact index of generation");expect(busyMs).toBeLessThanOrEqual(1.2*quietMs+500);
  expect(stampOf(busy)).toBe(loadedStamp);expect(Object.keys(graph.projects).length).toBe(count-broken);
  const next=createMockHost(busy,options);await loadGenerationGraphV1(next.host);
  expect(next.log).toEqual([]);expect(next.events).toEqual(["shared"]);expect(stampOf(busy)).toBe(implementationDigestV1(await next.host.implementationRows()));
 }finally{clean(quiet);clean(busy);}
},240000);


const sliceTree=()=>{
 const files:Record<string,string>={};
 for(const[name,imports]of[["a",["@syn/b"]],["b",["@syn/c"]],["c",[]],["d",["@syn/e"]],["e",[]],["f",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";
 return files;
};


for(const[index,row]of(fixture as any).closures.entries())test("selected closure is the independent model "+index,async()=>{
 const owners=Object.fromEntries(Object.entries(row.owners).map(([name,value]:[string,any])=>[name,value.root])),fragments=row.owners as Record<string,any>;
 const run=()=>sliceClosureV1({owners,fragment:async(name:string)=>({edges:fragments[name].edges.map((target:string)=>({source:name,target})),node:{targets:fragments[name].targets,namedInputs:fragments[name].namedInputs}}),selection:row.selection as never,roots:row.roots,external:row.external??[],providers:row.providers??[]});
 if(row.expected.error)await expect(run()).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});else expect([...await run()].sort()).toEqual(row.expected.admitted);
});


for(const[index,row]of(fixture as any).closures.entries())test("selected closure over the slice view of the fragments is the independent model "+index,async()=>{
 const owners=Object.fromEntries(Object.entries(row.owners).map(([name,value]:[string,any])=>[name,value.root])),fragments=row.owners as Record<string,any>;
 const run=()=>sliceClosureV1({owners,fragment:async(name:string)=>sliceViewV1({edges:fragments[name].edges.map((target:string)=>({source:name,target,type:"static"})),node:{targets:Object.fromEntries(Object.entries(fragments[name].targets??{}).map(([target,operation]:[string,any])=>[target,{...operation,executor:"repo:noop",options:{big:"x".repeat(64)},outputs:["o"]}])),namedInputs:fragments[name].namedInputs}}),selection:row.selection as never,roots:row.roots,external:row.external??[],providers:row.providers??[]});
 if(row.expected.error)await expect(run()).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});else expect([...await run()].sort()).toEqual(row.expected.admitted);
});

test("the slice view keeps only what the closure reads",()=>{
 const view=sliceViewV1({edges:[{target:"b",source:"a",sourceFile:"x"} as never],node:{targets:{build:{inputs:["{workspaceRoot}/x",{projects:["b"],input:"nativeSources"},{env:"E"}],dependsOn:["^build"],executor:"repo:script",options:{command:"x"}} as never}}});
 expect(view).toEqual({edges:[{target:"b"}],node:{targets:{build:{inputs:[{input:"nativeSources",projects:["b"]}],dependsOn:["^build"]}}}});
 const grouped=sliceViewV1({edges:[],node:{targets:{test:{inputs:["default","^shared","!{projectRoot}/dist/**",{env:"E"}]}},namedInputs:{default:["{projectRoot}/**/*",{input:"cargoToolchain",projects:["proctor"],extra:1} as never],shared:[{input:"own",dependencies:true}]}}});
 expect(grouped).toEqual({edges:[],node:{targets:{test:{inputs:["default","^shared"],dependsOn:undefined}},namedInputs:{default:[{input:"cargoToolchain",projects:["proctor"]}],shared:[{input:"own",dependencies:true}]}}});
});

test("an orchestrating build pins a generation that covers the nested run it launches: the outer slice admits the declared children, the nested slice is inside it",async()=>{
 const fragments:Record<string,{root:string;edges:string[];targets:Record<string,unknown>}>={
  dash:{root:"pkg/dash",edges:["lib"],targets:{build:{executor:"repo:script",metadata:{semio:{execution:{version:1,kind:"orchestration",children:[{project:"dash",target:"compile"}]},cache:{class:"driver-owned"}}}},compile:{inputs:["default",{input:"scriptProvider:leases:native",projects:["providers"]}]}}},
  providers:{root:"lib/providers",edges:["process"],targets:{}},process:{root:"process",edges:["framework"],targets:{}},framework:{root:"framework",edges:["value"],targets:{}},value:{root:"value",edges:[],targets:{}},lib:{root:"lib",edges:[],targets:{}},course:{root:"course",edges:["framework"],targets:{build:{}}},
 };
 const owners=Object.fromEntries(Object.entries(fragments).map(([name,row])=>[name,row.root])),fragment=async(name:string)=>sliceViewV1({edges:fragments[name]!.edges.map(target=>({target})),node:{targets:fragments[name]!.targets as never}});
 const closure=(target:string)=>sliceClosureV1({owners,fragment,selection:{kind:"tasks",tasks:[{project:"dash",target}]}});
 const outer=await closure("build"),nested=await closure("compile");
 expect([...outer].sort()).toEqual(["dash","framework","lib","process","providers","value"]);
 expect([...nested].every(name=>outer.has(name))).toBe(true);
 const names=Object.keys(owners).sort(),pin=readGenerationPinV1(encodeGenerationPinV1({owner:1,generation:7,manifest:"a".repeat(64),owners:names,covered:outer,atMs:Date.now()}));
 expect(generationPinOpenV1({pin,owners:names,needed:nested})).toEqual([]);
 expect((await fragment("dash")).node.targets.build).toEqual({inputs:[],dependsOn:undefined,metadata:{semio:{execution:{kind:"orchestration",children:[{project:"dash",target:"compile"}]}}}});
 fragments.dash!.targets.build={metadata:{semio:{execution:{version:1,kind:"orchestration",children:[{project:"ghost",target:"compile"}]}}}};
 await expect(closure("build")).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});
});

test("the slice scenario corpus agrees with the python oracle",()=>{
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});

test("a selected scope is the closure of its projects read from the generation: it builds the generation when none is usable, current serves it, fresh refreshes only the dirty closure, the admission names the selection",async()=>{
 const dir=scratch(),selection={kind:"tasks",tasks:[{project:"a",target:"build"}]},options={selection,slice:{selection,roots:["a"]}};
 try{
  await seed(dir,sliceTree());const mock=createMockHost(dir);
  const cold=await loadGenerationGraphV1(mock.host,options) as any;expect(mock.events).toContain("builder:held");expect(mock.log.length).toBe(1);expect(Object.keys(cold.projects).sort()).toEqual(["a","b","c"]);
  mock.events.length=0;mock.expanded.length=0;const graph=await loadGenerationGraphV1(mock.host,{...options,policy:"current"}) as any;
  expect(mock.events).toEqual(["shared"]);expect(Object.keys(graph.projects).sort()).toEqual(["a","b","c"]);expect(graph.admission.selection).toEqual(selection);expect(graph.admission.registry).toBe(mock.host.registryDigest);expect(graph.admission.closure).toMatch(/^[a-f0-9]{64}$/u);expect(mock.expanded).toEqual([["a","b","c"]]);
  await applyOps(dir,[{op:"write",path:"pkg/f/src/index.ts",content:"export const other=1;\n"},{op:"write",path:"pkg/c/src/index.ts",content:"export const v=2;\n"}]);
  mock.events.length=0;mock.log.length=0;const served=await loadGenerationGraphV1(mock.host,{...options,policy:"current"}) as any;
  expect(mock.events).toEqual(["shared"]);expect(served.projects.c.sources.length).toBe(1);
  const fresh=await loadGenerationGraphV1(mock.host,options) as any;
  expect(mock.log).toEqual([{full:["c"],edges:[],cold:false}]);expect(Object.keys(fresh.projects).sort()).toEqual(["a","b","c"]);
  const unknown={kind:"tasks",tasks:[{project:"zz",target:"build"}]};
  await expect(loadGenerationGraphV1(mock.host,{selection:unknown,slice:{selection:unknown,roots:["zz"]}})).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});
 }finally{clean(dir);}
},60000);

test("a lean load names projects, targets and unavailable projects from the manifest alone and expands nothing",async()=>{
 const dir=scratch();
 try{
  const files=ringFiles(12);for(const index of[10,11])files[`pkg/p${index}/project.json`]=JSON.stringify({name:`p${index}`,broken:true});
  await seed(dir,files);const mock=createMockHost(dir);await loadGenerationGraphV1(mock.host);
  mock.events.length=0;mock.expanded.length=0;const lean=await loadGenerationGraphV1(mock.host,{policy:"current",lean:true}) as any;
  expect(mock.events).toEqual(["shared"]);expect(mock.expanded).toEqual([]);expect(lean.lean).toBe(true);expect(Object.keys(lean.projects).sort()).toEqual(Array.from({length:10},(_,i)=>"p"+i).sort());
  expect(lean.unavailable.map((row:any)=>row.project).sort()).toEqual(["p10","p11"]);
 }finally{clean(dir);}
},60000);

test("compact inputs are expanded within the capacity of one capture: text bytes, not UTF-16 units, bound every call and an oversized project is refused",async()=>{
 const reuse=await import("../../../../👁️observation/📥️reuse/🟨️.mjs"),policy=(await import("../../../../👁️observation/📥️reuse/🧬️schema/🔣️.json")).default.definitions.Policy.const;
 const limit=policy.maximumTextBytes,text=(bytes:number)=>"é".repeat(Math.floor(bytes/2));
 const wide=text(limit+2);expect(wide.length).toBeLessThan(limit);expect(Buffer.byteLength(wide)).toBeGreaterThan(limit);
 expect(()=>reuse.captureInferenceInventoryReferencesV1({p:wide})).toThrow("text capacity exceeded");
 const projects:Record<string,unknown>={};for(let index=0;index<5;index++)projects["p"+index]={sources:text(Math.floor(limit/3))};
 const calls:number[]=[],expand=async(chunk:Record<string,unknown>)=>{reuse.captureInferenceInventoryReferencesV1(chunk);calls.push(Object.keys(chunk).length);return chunk;};
 await expect(expand(projects)).rejects.toThrow("text capacity exceeded");calls.length=0;
 const bounded=await expandInChunksV1(projects,expand,{maximumProjects:32,maximumBytes:Math.floor(limit/4)});
 expect(Object.keys(bounded).sort()).toEqual(Object.keys(projects).sort());expect(calls.every(count=>count===1)).toBe(true);
 calls.length=0;const generous=await expandInChunksV1(projects,expand,{maximumProjects:32,maximumBytes:limit});expect(Object.keys(generous).length).toBe(5);expect(calls.reduce((sum,count)=>sum+count,0)).toBe(5);expect(Math.max(...calls)).toBeLessThan(5);
 await expect(expandInChunksV1({big:{sources:wide}},expand,{maximumProjects:32,maximumBytes:limit})).rejects.toThrow("text capacity exceeded");
},120000);

test("a removed owner is no stub of the next inference, so no edge refresh can ask for its published fragment",()=>{
 const stubs={a:{root:"pkg/a"},b:{root:"pkg/b"},c:{root:"pkg/c"}};
 expect(Object.keys(survivingStubsV1(stubs,new Set(["b"])))).toEqual(["a","c"]);
 expect(survivingStubsV1(stubs,[])).toEqual(stubs);
 expect(survivingStubsV1(stubs,["a","b","c","unknown"])).toEqual({});
 const python3=Bun.spawnSync([python!,"-I","-c","import json,sys;s=json.load(sys.stdin);print(json.dumps({k:v for k,v in s['stubs'].items() if k not in s['removed']}))"],{stdin:new TextEncoder().encode(JSON.stringify({stubs,removed:["b"]}))});
 expect(JSON.parse(python3.stdout.toString())).toEqual(survivingStubsV1(stubs,["b"]));
});

test("the stale projects of a pending refresh are prepared once, after the shared attempt and before the builder, for the owners the plan named",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const requests:{generation:number;dirty:readonly string[];roots:Readonly<Record<string,string>>;implementation:string}[]=[];
  const mock=createMockHost(dir,{prefetch:async request=>{requests.push(request);}});
  await loadGenerationGraphV1(mock.host);
  expect(requests).toEqual([]);expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);
  mock.events.length=0;await loadGenerationGraphV1(mock.host);expect(requests).toEqual([]);expect(mock.events).toEqual(["shared"]);
  await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:'import "@syn/p4";\nexport const v=2;\n'}]);
  mock.events.length=0;await loadGenerationGraphV1(mock.host);
  expect(mock.events).toEqual(["shared","prefetch","builder:wait","builder:held","builder:released"]);
  expect(requests).toHaveLength(1);expect(requests[0]!.dirty).toEqual(["p2"]);expect(requests[0]!.generation).toBe(0);expect(Object.keys(requests[0]!.roots)).toEqual(["p2"]);expect(requests[0]!.roots.p2).toBe("pkg/p2");expect(requests[0]!.implementation).toMatch(/^[a-f0-9]{64}$/u);
 }finally{clean(dir);}
},60000);

test("a refresh restricted to a closure prepares only the stale owners of that closure",async()=>{
 const dir=scratch();
 try{
  const files:Record<string,string>={};
  for(const[name,imports]of[["a",["@syn/b"]],["b",["@syn/c"]],["c",[]],["d",["@syn/e"]],["e",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";
  await seed(dir,files);const requests:string[][]=[];const mock=createMockHost(dir,{prefetch:async request=>{requests.push([...request.dirty]);}});
  await loadGenerationGraphV1(mock.host);
  await applyOps(dir,[{op:"write",path:"pkg/c/src/index.ts",content:"export const v=3;\n"},{op:"write",path:"pkg/e/src/index.ts",content:"export const v=4;\n"}]);
  await loadGenerationGraphV1(mock.host,{closure:["a"]});
  expect(requests).toEqual([["c"]]);
 }finally{clean(dir);}
},60000);

test("a prefetch that settles late delays the builder, and one that rejects fails the load before any builder is entered",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));const mock=createMockHost(dir);await loadGenerationGraphV1(mock.host);
  await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:'import "@syn/p4";\nexport const v=5;\n'}]);
  let release!:()=>void;const late=new Promise<void>(resolve=>{release=resolve;});
  const slow=createMockHost(dir,{prefetch:()=>late});const loading=loadGenerationGraphV1(slow.host);
  await sleep(150);expect(slow.events).toEqual(["shared","prefetch"]);release();await loading;expect(slow.events).toEqual(["shared","prefetch","builder:wait","builder:held","builder:released"]);
  await applyOps(dir,[{op:"write",path:"pkg/p3/src/index.ts",content:'import "@syn/p5";\nexport const v=6;\n'}]);
  const failing=createMockHost(dir,{prefetch:async()=>{throw new Error("cancelled");}});
  await expect(loadGenerationGraphV1(failing.host)).rejects.toThrow("cancelled");expect(failing.events).toEqual(["shared","prefetch"]);
 }finally{clean(dir);}
},60000);


const {selectProjectNamesV1}=await import("../🟨️.mjs") as unknown as {selectProjectNamesV1:(names:readonly string[],patterns:unknown)=>string[]};

const nxSelectionV1=(names:string[],patterns:string[])=>{
 const excluded=patterns.filter(pattern=>pattern.startsWith("!")).map(pattern=>pattern.slice(1)),included=patterns.filter(pattern=>!pattern.startsWith("!")),hit=(pattern:string,name:string)=>pattern==="*"||minimatch(name,pattern);
 return names.filter(name=>!excluded.some(pattern=>hit(pattern,name))&&(included.length===0?excluded.length>0:included.some(pattern=>hit(pattern,name)))).sort();
};

test("project name patterns select exactly what minimatch selects under the Nx rules: `*` is every project, globs never cross a scope separator, `!` removes, only-`!` starts from every project, an empty list selects none",()=>{
 const names=["workspace","@semio-tech/repo-cli","@semio-tech/repo-test-domain","@semio-tech/repo-library","@semio-tech/framework","@semio-tech/framework-os","plain","@semio-tech/deep/x"];
 for(const patterns of [["*"],["*","!workspace"],["*","!workspace","!@semio-tech/repo-test-domain"],["@semio-tech/repo-*"],["@semio-tech/repo-*","!workspace"],["@semio-tech/repo-*","!@semio-tech/repo-test-*"],["@semio-tech/*"],["@semio-tech/**"],["@semio-tech/framework?os"],["@semio-tech/framework{,-os}"],["@semio-tech/{repo-cli,framework}"],["!workspace"],["!@semio-tech/*"],[],["plain"],["plain","!plain"],["@semio-tech/none-*"],["workspace","@semio-tech/repo-cli"]])
  expect(selectProjectNamesV1(names,patterns).sort(),JSON.stringify(patterns)).toEqual(nxSelectionV1(names,patterns));
 expect(selectProjectNamesV1(names,"@semio-tech/repo-*").sort()).toEqual(nxSelectionV1(names,["@semio-tech/repo-*"]));
 expect(selectProjectNamesV1(names,["ghost"])).toEqual(["ghost"]);
 expect(selectProjectNamesV1(names,["ghost","!ghost"])).toEqual([]);
 expect(()=>selectProjectNamesV1(names,[null as never])).toThrow("Project patterns must be strings");
});

const checkout=(()=>{let at=here;while(at!==dirname(at)&&!existsSync(join(at,"rust-toolchain.toml")))at=dirname(at);return at;})(),rootProject=join(checkout,"\u{1F4CB}\u{FE0F}project.json");
for(const aggregate of ["lint","lint-repo","build","test"])test.skipIf(!existsSync(rootProject))(`the workspace aggregate ${aggregate} of this repository admits exactly the projects its own dependsOn patterns select`,async()=>{
 const targets=JSON.parse(readFileSync(rootProject,"utf8")).targets as Record<string,{dependsOn?:unknown[]}>,own=(targets[aggregate]?.dependsOn??[]).filter((entry:any):entry is {target:string;projects:string[]}=>typeof entry==="object"&&Array.isArray((entry as any).projects));
 expect(own.length,"the aggregate selects projects by pattern").toBeGreaterThan(0);
 const names=["workspace","@semio-tech/repo-cli","@semio-tech/repo-library","@semio-tech/repo-test-domain","@semio-tech/framework","@semio-tech/course"],owners=Object.fromEntries(names.map((name,index)=>[name,"pkg/"+index])),fragments:Record<string,any>=Object.fromEntries(names.map(name=>[name,{edges:[],targets:{lint:{},build:{},test:{}}}]));
 fragments.workspace.targets[aggregate]={dependsOn:targets[aggregate]!.dependsOn};
 const admitted=await sliceClosureV1({owners,fragment:async(name:string)=>({edges:[],node:{targets:fragments[name].targets}}),selection:{kind:"tasks",tasks:[{project:"workspace",target:aggregate}]}});
 const named=(targets[aggregate]!.dependsOn??[]).filter((entry):entry is string=>typeof entry==="string"&&entry.includes(":")&&!entry.startsWith("^")).map(entry=>entry.slice(0,entry.lastIndexOf(":"))),expected=new Set(["workspace",...named,...own.flatMap(entry=>nxSelectionV1(names,entry.projects))]);
 expect([...admitted].sort()).toEqual([...expected].sort());
 expect(admitted.size,"the aggregate reaches projects beyond itself").toBeGreaterThan(1);
});
