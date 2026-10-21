/** 🔌️ Contract: the repository protocol of the graph generation. A shared reader serves a verified or current generation without ever entering the builder custody; only a stale generation under a fresh policy enters it, the facts resource is held only for the pointer swap, closures read only their own projects, texts come in English and German, and partial inference results are shaped into exactly the facts and edges each owner consumed. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {GENERATION_MESSAGES_V1,extractGraphPolicyV1,implementationDigestV1,loadGenerationGraphV1,renderGenerationMessageV1,scopeImplementationRowsV1,selectGenerationLanguageV1,shapeComputeResultV1} from "../🟨️.mjs";
import {GENERATION_POLICY_V1,canonicalJsonV1} from "../../🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../🧪️tests/🏃️world/🟦️.ts";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"..","..","🧪️tests","🐍️oracle.py");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-load-"));
const clean=(dir:string)=>rmSync(dir,{recursive:true,force:true});
const sleep=(ms:number)=>new Promise<void>(resolve=>setTimeout(resolve,ms));
const pythonGraph=(dir:string,names?:string[])=>{const graph=JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());if(!names)return graph;return{...graph,projects:Object.fromEntries(Object.entries(graph.projects).filter(([name])=>names.includes(name))),dependencies:graph.dependencies.filter((edge:any)=>names.includes(edge.source)&&names.includes(edge.target))};};
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));
const shape=(graph:any)=>normalized({projects:graph.projects,dependencies:graph.dependencies,externalNodes:graph.externalNodes,unavailable:graph.unavailable??[]});

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

test("the bilingual texts are the declared ones, equal in both languages by placeholder, and rendering fills them",()=>{
 expect(JSON.parse(JSON.stringify(GENERATION_MESSAGES_V1))).toEqual(schema.definitions.Messages.const);
 expect(renderGenerationMessageV1("published",{generation:7,computed:2,seconds:3},{})).toBe("graph: generation 7 published, 2 projects refreshed in 3 s");
 expect(renderGenerationMessageV1("published",{generation:7,computed:2,seconds:3},{LANG:"de_DE.UTF-8"})).toBe("Graph: Generation 7 veröffentlicht, 2 Projekte in 3 s erneuert");
 expect(()=>renderGenerationMessageV1("other" as never,{},{})).toThrow();
});

for(const row of fixture.language)test("language "+JSON.stringify(row.env),()=>{expect(selectGenerationLanguageV1(row.env)).toBe(row.expected as "en"|"de");});

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
  expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);expect(shape(cold)).toEqual(pythonGraph(dir));expect(mock.said.at(-1)!.text).toContain("published");
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
  const german=createMockHost(dir,{environment:{LANG:"de_DE.UTF-8"}});await loadGenerationGraphV1(german.host,{policy:"current"});
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
  expect(mock.events).toContain("builder:held");expect(mock.log[0]!.full).toEqual(["c","e"]);expect(shape(rebuilt)).toEqual(pythonGraph(dir,["a","b","c"]));
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
 const scoped=scopeImplementationRowsV1(row.rows as never),expected=row.expected as {all:unknown[];projects:unknown[];digest:string};
 expect({all:scoped.all,projects:scoped.projects}).toEqual({all:expected.all,projects:expected.projects} as never);
 expect(implementationDigestV1(row.rows as never)).toBe(expected.digest);
});

test("shaping adds the implementation facts of the consumers: valid projects the shared and their own, unavailable ones and the environment only the shared",()=>{
 const{workspaceRoot,part,stubs,implementation,expected}=fixture.shapeImplementation;
 expect(shapeComputeResultV1({workspaceRoot,part:part as never,stubs:stubs as never,implementation:implementation as never})).toEqual(expected as never);
});

test("the stale generation text is declared in both languages",()=>{
 expect(renderGenerationMessageV1("stale",{generation:2},{})).toBe("graph: serving generation 2 built by another implementation (graph policy current)");
 expect(renderGenerationMessageV1("stale",{generation:2},{LANG:"de_DE.UTF-8"})).toBe("Graph: Generation 2 stammt von einer anderen Implementierung und wird geliefert (Graph-Richtlinie current)");
});

const implementationPathsV1=()=>{const scopes=GENERATION_POLICY_V1.implementation;return{inert:"impl/"+scopes.none[0]+"ledger.mjs",nodes:"impl/"+scopes.projects[0]+"nodes.mjs",core:"impl/core/graph.mjs"};};
const stampOf=(dir:string):string=>{const pointer=JSON.parse(readFileSync(join(dir,".gen","current.json"),"utf8"));return JSON.parse(readFileSync(join(dir,".gen","m",pointer.manifest+".json"),"utf8")).implementation;};
const brokenRing=(count:number,broken:number)=>{const files=ringFiles(count);for(let i=count-broken;i<count;i++)files[`pkg/p${i}/project.json`]=JSON.stringify({name:`p${i}`,broken:true});return files;};
const names=(count:number)=>Array.from({length:count},(_,i)=>"p"+i).sort();

test("a build names the implementation modules it loaded: an inert module refreshes nothing, a node module the valid projects, a shared module every project, and a reader of another implementation is served or verified without a rebuild",async()=>{
 const dir=scratch(),paths=implementationPathsV1(),files={...brokenRing(12,2),[paths.inert]:"1",[paths.nodes]:"1",[paths.core]:"1"},valid=names(10),every=names(12);
 try{
  await seed(dir,files);const options={implementationFiles:Object.values(paths)};
  const loader=createMockHost(dir,options);await loadGenerationGraphV1(loader.host);
  const stampA=implementationDigestV1(await loader.host.implementationRows());expect(stampOf(dir)).toBe(stampA);expect(loader.log).toEqual([{full:every,edges:[],cold:true}]);
  await applyOps(dir,[{op:"write",path:paths.inert,content:"2"}]);
  const inert=createMockHost(dir,options);await loadGenerationGraphV1(inert.host);
  expect(inert.events).toEqual(["shared"]);expect(inert.log).toEqual([]);expect(stampOf(dir)).toBe(stampA);
  await applyOps(dir,[{op:"write",path:paths.nodes,content:"2"}]);
  const nodes=createMockHost(dir,options),graph=await loadGenerationGraphV1(nodes.host) as any;
  expect(nodes.log).toEqual([{full:valid,edges:[],cold:false}]);expect(Object.keys(graph.projects).sort()).toEqual(valid);expect(graph.unavailable.map((row:any)=>row.project).sort()).toEqual(["p10","p11"]);
  const stampB=implementationDigestV1(await nodes.host.implementationRows());expect(stampB).not.toBe(stampA);expect(stampOf(dir)).toBe(stampB);
  await applyOps(dir,[{op:"write",path:paths.core,content:"2"}]);
  const core=createMockHost(dir,options);await loadGenerationGraphV1(core.host);
  expect(core.log).toEqual([{full:every,edges:[],cold:false}]);const stampC=implementationDigestV1(await core.host.implementationRows());expect(stampOf(dir)).toBe(stampC);
  loader.events.length=0;loader.said.length=0;const stale=await loadGenerationGraphV1(loader.host,{policy:"current"}) as any;
  expect(loader.events).toEqual(["shared"]);expect(loader.said.map(row=>row.text)).toEqual(["graph: serving generation 2 built by another implementation (graph policy current)"]);expect(Object.keys(stale.projects).length).toBe(10);
  loader.events.length=0;await loadGenerationGraphV1(loader.host);expect(loader.events).toEqual(["shared"]);expect(loader.log.length).toBe(1);expect(stampOf(dir)).toBe(stampC);
 }finally{clean(dir);}
},120000);

test("an implementation module edited every few hundred milliseconds never restarts a 300 project build: it publishes the generation of the implementation it loaded and the next fresh load refreshes exactly the consumers of the edited module",async()=>{
 const count=300,broken=30,paths=implementationPathsV1(),files={...brokenRing(count,broken),[paths.inert]:"0",[paths.nodes]:"0",[paths.core]:"0"},valid=names(count-broken);
 const quiet=scratch(),busy=scratch(),options={implementationFiles:Object.values(paths),ownerDelayMs:8};
 try{
  await seed(quiet,files);const timed=createMockHost(quiet,options),started=Date.now();await loadGenerationGraphV1(timed.host);const quietMs=Date.now()-started;
  await seed(busy,files);const loaded=createMockHost(busy,options),loadedStamp=implementationDigestV1(await loaded.host.implementationRows());
  let editing=true,edits=0;const editor=(async()=>{while(editing){await sleep(250);edits++;await writeFile(join(busy,paths.nodes),String(edits));}})();
  const begin=Date.now(),graph=await loadGenerationGraphV1(loaded.host) as any,busyMs=Date.now()-begin;editing=false;await editor;
  expect(edits).toBeGreaterThanOrEqual(3);expect(loaded.log.length).toBe(1);expect(loaded.said.at(-1)!.text).toContain("published");expect(busyMs).toBeLessThanOrEqual(1.2*quietMs+500);
  expect(stampOf(busy)).toBe(loadedStamp);expect(Object.keys(graph.projects).length).toBe(count-broken);
  const next=createMockHost(busy,options);await loadGenerationGraphV1(next.host);
  expect(next.log).toEqual([{full:valid,edges:[],cold:false}]);expect(stampOf(busy)).toBe(implementationDigestV1(await next.host.implementationRows()));
  const again=createMockHost(busy,options);await loadGenerationGraphV1(again.host);expect(again.log).toEqual([]);expect(again.events).toEqual(["shared"]);
 }finally{clean(quiet);clean(busy);}
},240000);
