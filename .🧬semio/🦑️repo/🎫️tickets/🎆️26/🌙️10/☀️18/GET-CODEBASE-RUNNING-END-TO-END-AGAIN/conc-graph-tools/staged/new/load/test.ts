/** 🔌️ Contract: the repository protocol of the graph generation. A shared reader serves a verified or current generation without ever entering the builder custody; only a stale generation under a fresh policy enters it, the facts resource is held only for the pointer swap, closures read only their own projects, texts come in English and German, and partial inference results are shaped into exactly the facts and edges each owner consumed. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {GENERATION_MESSAGES_V1,extractGraphPolicyV1,loadGenerationGraphV1,renderGenerationMessageV1,selectGenerationLanguageV1,shapeComputeResultV1} from "../🟨️.mjs";
import {canonicalJsonV1} from "../../🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../🧪️tests/🏃️world/🟦️.ts";

const python=Bun.which("python3"),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"..","..","🧪️tests","🐍️oracle.py");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-load-"));
const clean=(dir:string)=>rmSync(dir,{recursive:true,force:true});
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
