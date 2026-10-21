import {afterAll,afterEach,describe,expect,test} from "bun:test";
import {appendFileSync,chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync,lstatSync} from "node:fs";
import {connect,createServer} from "node:net";
import {spawn,spawnSync} from "node:child_process";
import {join} from "node:path";
import {createResidentServiceV1,ServiceRunningErrorV1} from "../service/index.mjs";
import {forwardCommandV1,launcherCapabilityV1} from "../client/index.mjs";
import {createFrameDecoderV1,encodeFrameV1} from "../protocol/frames.mjs";
import {POLICY_V1} from "../protocol/policy.mjs";
import {attenuateCapabilityV1,mintCapabilityV1} from "../protocol/capability.mjs";
import {endpointsV1,worktreeIdV1} from "../protocol/endpoint.mjs";
import {capabilityIdV1,verifyCapabilityV1,createRevocationRegistryV1} from "../protocol/capability.mjs";
import coreSchema from "../../core/service.schema.json" with {type:"json"};
import {createFilterV1} from "../state/index.mjs";
import {residentGraphSeamsV1} from "../graph-hooks/host/index.mjs";
import {taskClosureV1} from "../ports/graph.mjs";
import {WATCHER,cleanup,haveBinary,pick,rng,sleep,snapshot,tempRoot,waitFor,writeTree} from "./common.ts";

const native=haveBinary(WATCHER);
const cleanups:(()=>Promise<void>|void)[]=[];
afterAll(async()=>{for(const fn of cleanups.reverse())await fn();});

const owners=[
 {name:"a",root:"pkgs/a",deps:[],tasks:[{name:"build",command:["b","a"],outputs:[".out/a"]}]},
 {name:"b",root:"pkgs/b",deps:["a"],tasks:[{name:"build",command:["b","b"],dependsOn:["a:build"],outputs:[".out/b"]},{name:"test",command:["t","b"],dependsOn:["b:build"],outputs:[".out/bt"]}]},
 {name:"c",root:"pkgs/c",deps:["b"],tasks:[{name:"build",command:["b","c"],dependsOn:["b:build"],outputs:[".out/c"]}]},
 {name:"d",root:"pkgs/d",deps:[],tasks:[{name:"build",command:["b","d"],outputs:[".out/d"]}]},
 ...["e","f","g","h"].map(name=>({name,root:"pkgs/"+name,deps:[] as string[],tasks:[{name:"build",command:["b",name],outputs:[".out/"+name]}]})),
];
const graph={generation:3,owners,env:{paths:["package.json"],prefixes:[]}};
const closureOf=(name:string)=>{const seen=new Set([name]),order=[name];for(let i=0;i<order.length;i++)for(const dep of owners.find(o=>o.name===order[i])!.deps)if(!seen.has(dep)){seen.add(dep);order.push(dep);}return order;};

type Options={emit?:(id:string,out:(stream:"stdout"|"stderr",text:string)=>Promise<unknown>)=>Promise<void>;backend?:string;policy?:Record<string,unknown>;requireCapability?:boolean;execution?:(id:string,root:string,signal:AbortSignal)=>Promise<number>;implementationPaths?:string[];refusal?:(id:string)=>{code:string;fields?:Record<string,string>}|undefined;custody?:any;binary?:string|undefined;graphOwners?:any;root?:string};

async function startService(options:Options={}){
 const root=options.root??tempRoot("svc-");if(!options.root)cleanups.push(()=>cleanup(root));
 if(!options.root)writeTree(root,{"package.json":"{}","pkgs/a/src/x.ts":"a","pkgs/b/src/x.ts":"b","pkgs/c/src/x.ts":"c","pkgs/d/src/x.ts":"d","pkgs/e/src/x.ts":"e","pkgs/f/src/x.ts":"f","pkgs/g/src/x.ts":"g","pkgs/h/src/x.ts":"h"});
 const base=mkdtempSync("/tmp/sr-");cleanups.push(()=>cleanup(base));
 const runtime=endpointsV1(root,{XDG_RUNTIME_DIR:base},process.platform);
 const executed:string[]=[],aborted:string[]=[],clients:any[]=[];
 const filter=createFilterV1({rooted:[".git"],names:["node_modules"]});
 const execution={async run({id,signal,onSpawn,onOutput,owner,client}:any){
  executed.push(id);clients.push(client);
  const refusal=options.refusal?.(id);if(refusal)return{exit:70,refusal};
  onSpawn?.();
  const code=options.execution?await options.execution(id,root,signal):0;
  if(signal.aborted){aborted.push(id);throw signal.reason;}
  if(code===0){const out=id.endsWith(":test")?".out/"+owner+"t":".out/"+owner;mkdirSync(join(root,out),{recursive:true});writeFileSync(join(root,out,"o.txt"),"built "+id+"\n");}
  if(options.emit)await options.emit(id,onOutput);else await onOutput("stdout","ran "+id+"\n");
  return{exit:code,signal:null};
 }};
 const listing=async()=>{const s=snapshot(root,rel=>filter.excluded(rel)||rel.startsWith(".out/")||rel===".semio-fence"||rel.startsWith(".semio-fence/"));return{directories:[...s.directories].map(([p,[m,i]])=>[p,m,i]),members:[...s.files]};};
 const custodyEvents:string[]=[];
 let currentOwners=options.graphOwners??owners;
 const ports={graph:{load:async()=>({...graph,owners:currentOwners}),environmentDigest:async()=>"salt",listing},execution,custody:options.custody??{acquire:async()=>{custodyEvents.push("acquire");},release:async()=>{custodyEvents.push("release");}}};
 const service=createResidentServiceV1({root,runtime,ports,implementation:"1".repeat(64),implementationPaths:options.implementationPaths??[],backend:options.backend??(native?"auto":"none"),watcherBinary:options.binary===undefined&&native?WATCHER:options.binary,exclude:{rooted:[".git"],names:["node_modules"]},fenceDirectory:".semio-fence",policy:{...POLICY_V1,fenceTimeoutMilliseconds:800,...(options.policy??{})},requireCapability:options.requireCapability??true,log:()=>{}});
 const started=await service.start();
 cleanups.push(()=>service.stop("test"));
 const run=(command:string,args:string[],extra:Record<string,unknown>={})=>{
  const out:string[]=[],err:string[]=[];
  return forwardCommandV1({root,command,args,cwd:root,languages:["en"],environment:{XDG_RUNTIME_DIR:base},stdout:{write:(t:string)=>out.push(String(t))},stderr:{write:(t:string)=>err.push(String(t))},...extra}).then((r:any)=>({...r,out:out.join(""),err:err.join("")}));
 };
 return{root,runtime,service,started,executed,aborted,clients,run,base,custodyEvents,ports,setOwners:(value:any)=>{currentOwners=value;}};
}

/** 🔌 A raw protocol connection for abuse and capability tests. */
function raw(runtime:{endpoint:string}){
 const frames:any[]=[];const socket=connect(runtime.endpoint);let closed=false;
 const decoder=createFrameDecoderV1({maximumFrameBytes:1<<24,onFrame:f=>frames.push(f),onError:()=>{}});
 socket.on("data",c=>decoder.push(c));socket.on("end",()=>socket.destroy());socket.on("close",()=>{closed=true;});socket.on("error",()=>{});
 return{frames,socket,send:(f:unknown)=>socket.write(typeof f==="string"?f:encodeFrameV1(f)),closed:()=>closed,wait:(pred:(f:any)=>boolean,ms=3000)=>waitFor(()=>frames.some(pred),ms).then(ok=>ok?frames.find(pred):undefined)};
}
const hello=(languages:string[]=["en"],cap?:string)=>({t:"hello",v:1,client:{name:"t",pid:1,lane:"l"},languages,...(cap?{cap}:{})});
const keyOf=(runtime:{key:string})=>readFileSync(runtime.key);

describe("memo, planning and invalidation",()=>{
 test("a second run is answered from the memo without executing anything",async()=>{
  const s=await startService();
  const first=await s.run("run",["c:build"]);expect(first.served).toBe("executed");expect(first.exit).toBe(0);expect(s.executed.sort()).toEqual(["a:build","b:build","c:build"]);
  const second=await s.run("run",["c:build"]);expect(second.served).toBe("memo");expect(second.out).toContain("Up to date");expect(s.executed.length).toBe(3);
 });

 test("the memo replay speaks the first language asked for: German first is German, never English",async()=>{
  const s=await startService();await s.run("run",["c:build"]);
  const german=await s.run("run",["c:build"],{languages:["de","en"]});expect(german.served).toBe("memo");expect(german.out).toContain("Aktuell, seit");expect(german.out).not.toContain("Up to date");
 });

 test("an edit executes exactly the affected tasks, then everything is current again (100 random edits against the closure oracle)",async()=>{
  const s=await startService();await s.run("run",["c:build"]);await s.run("test",["b"]);
  const random=rng(5);
  for(let i=0;i<100;i++){
   const owner=pick(random,["a","b","c","d"]),file=s.root+"/pkgs/"+owner+"/src/x.ts";
   appendFileSync(file,"e"+i);s.executed.length=0;
   const result=await s.run("run",["c:build"]);
   const expected=["a","b","c"].includes(owner)?["a","b","c"].filter(o=>closureOf("c").includes(o)&&closureOf(o).includes(owner)).map(o=>o+":build").sort():[];
   expect([...s.executed].sort()).toEqual(expected);
   expect(result.served).toBe(expected.length?"executed":"memo");
   s.executed.length=0;expect((await s.run("run",["c:build"])).served).toBe("memo");expect(s.executed).toEqual([]);
  }
 },60000);

 test("read your writes: a run started right after an edit always sees it",async()=>{
  const s=await startService();await s.run("run",["a:build"]);
  for(let i=0;i<60;i++){appendFileSync(s.root+"/pkgs/a/src/x.ts","w"+i);s.executed.length=0;const r=await s.run("run",["a:build"]);expect(r.served).toBe("executed");expect(s.executed).toEqual(["a:build"]);}
 },60000);

 test("only tasks that exit 0 are memoised; a change during the run prevents memoisation",async()=>{
  let fail=true,edit=false;
  const s=await startService({execution:async(id,root)=>{if(id==="d:build"&&fail)return 3;if(id==="a:build"&&edit){appendFileSync(root+"/pkgs/a/src/x.ts","during");}return 0;}});
  expect((await s.run("run",["d:build"])).exit).toBe(3);fail=false;expect((await s.run("run",["d:build"])).served).toBe("executed");expect((await s.run("run",["d:build"])).served).toBe("memo");
  edit=true;expect((await s.run("run",["a:build"])).served).toBe("executed");edit=false;
  s.executed.length=0;expect((await s.run("run",["a:build"])).served).toBe("executed");expect(s.executed).toEqual(["a:build"]);expect((await s.run("run",["a:build"])).served).toBe("memo");
 });

 test("outputs: deleting or changing a produced file rebuilds, an unrelated output does not",async()=>{
  const s=await startService();await s.run("run",["b:build"]);
  mkdirSync(s.root+"/.out/other",{recursive:true});writeFileSync(s.root+"/.out/other/z","z");
  expect((await s.run("run",["b:build"])).served).toBe("memo");
  rmSync(s.root+"/.out/b",{recursive:true});s.executed.length=0;
  expect((await s.run("run",["b:build"])).served).toBe("executed");expect(s.executed).toEqual(["b:build"]);
  appendFileSync(s.root+"/.out/a/o.txt","tamper");s.executed.length=0;
  expect((await s.run("run",["b:build"])).served).toBe("executed");expect(s.executed).toEqual(["a:build"]);
 });

 test("environment inputs and the unknown task",async()=>{
  const s=await startService();await s.run("run",["c:build"]);s.executed.length=0;
  writeFileSync(s.root+"/package.json",'{"changed":true}');
  const r=await s.run("run",["c:build"]);expect(r.served).toBe("executed");expect([...s.executed].sort()).toEqual(["a:build","b:build","c:build"]);
  const unknown=await s.run("run",["zzz:build"]);expect(unknown.exit).toBe(1);expect(unknown.err).toContain("unknown task");
 });

 test("run-many, test, affected and dirty queries",async()=>{
  const s=await startService();
  expect((await s.run("run-many",["a","d","--targets=build"])).served).toBe("executed");
  expect((await s.run("test",["b"])).exit).toBe(0);
  const cursor=JSON.parse((await s.run("cursor",[])).out);
  appendFileSync(s.root+"/pkgs/a/src/x.ts","q");
  const dirty=JSON.parse((await s.run("dirty",["--since",cursor.instance+":"+cursor.seq])).out);
  expect(dirty.complete).toBe(true);expect(dirty.changes.map((c:any)=>c.path)).toContain("pkgs/a/src/x.ts");
  const affected=JSON.parse((await s.run("affected",["a","b","c"])).out).stale.sort();
  expect(affected).toEqual(["a:build","b:build","c:build"]);
  expect((await s.run("dirty",["--since","other:1"])).out).toContain('"complete":false');
 });
});

describe("frontier cursors for the generation host",()=>{
 test("adopted digests answer dirty --digest from the journal; unknown digests, foreign instances and bad cursors prove nothing; the real seams save the stat pass",async()=>{
  const s=await startService();
  const cursor=JSON.parse((await s.run("cursor",[])).out),digest="ab".repeat(32);
  expect((await s.run("dirty",["--digest",digest])).out).toContain('"reason":"digest"');
  expect((await s.run("adopt-frontier",[digest,cursor.instance+":"+cursor.seq])).out).toBe("adopted\n");
  expect(JSON.parse((await s.run("dirty",["--digest",digest])).out)).toMatchObject({complete:true,changes:[]});
  appendFileSync(s.root+"/pkgs/a/src/x.ts","e");
  expect(JSON.parse((await s.run("dirty",["--digest",digest])).out).changes.map((c:any)=>c.path)).toContain("pkgs/a/src/x.ts");
  const other="cd".repeat(32);
  expect((await s.run("adopt-frontier",[other,"someone-else:1"])).out).toBe("ignored\n");expect((await s.run("adopt-frontier",[other,cursor.instance+":999999"])).out).toBe("ignored\n");expect((await s.run("adopt-frontier",["nope",cursor.instance+":1"])).out).toBe("ignored\n");
  expect((await s.run("dirty",["--digest",other])).out).toContain('"reason":"digest"');
  const filter=createFilterV1({rooted:[".git"],names:["node_modules"]});
  const capture=()=>{const snap=snapshot(s.root,rel=>filter.excluded(rel)||rel.startsWith(".out/")||rel===".semio-fence"||rel.startsWith(".semio-fence/"));return{version:1,capturedAt:Date.now(),directories:[...snap.directories].map(([p,[m,i]])=>[p,m,i]).sort((a:any,b:any)=>a[0]<b[0]?-1:1),members:[...snap.files].sort((a,b)=>a[0]<b[0]?-1:1)};};
  const digestOf=(frontier:any)=>require("node:crypto").createHash("sha256").update(JSON.stringify(frontier.members)).digest("hex");
  let stat=0;
  const seams=residentGraphSeamsV1({root:s.root,environment:{XDG_RUNTIME_DIR:s.base},membership:async()=>{stat++;const frontier=capture();return{digest:digestOf(frontier),frontier,changes:[]};},holds:async()=>true});
  const first=await seams.membership(undefined);expect(stat).toBe(1);
  appendFileSync(s.root+"/pkgs/b/src/x.ts","edit");writeFileSync(s.root+"/pkgs/b/src/new.ts","n");rmSync(s.root+"/pkgs/c/src/x.ts");
  await sleep(50);
  const second=await seams.membership({manifest:{membership:first.digest},frontier:first.frontier});
  expect(stat).toBe(1);
  const fresh=capture();
  expect(second.frontier.members).toEqual(fresh.members);expect(second.frontier.directories).toEqual(fresh.directories);
  expect(second.changes.filter((c:any)=>!c.directory).map((c:any)=>c.kind+" "+c.path).sort()).toEqual(["added pkgs/b/src/new.ts","changed pkgs/b/src/x.ts".replace("changed","changed"),"removed pkgs/c/src/x.ts"].filter(x=>!x.startsWith("changed")).sort());
 },20000);
});

describe("graph changes",()=>{
 test("editing a project configuration re-reads the graph: new tasks appear, changed commands rebuild, untouched results stay valid",async()=>{
  const s=await startService();await s.run("run",["c:build"]);
  expect((await s.run("run",["i:build"])).exit).toBe(1);
  writeTree(s.root,{"pkgs/i/src/x.ts":"i"});
  const next=owners.map(o=>o.name==="c"?{...o,tasks:[{...o.tasks[0]!,command:["b","c","v2"]}]}:o).concat([{name:"i",root:"pkgs/i",deps:[],tasks:[{name:"build",command:["b","i"],outputs:[".out/i"]}]} as any]);
  s.setOwners(next);writeFileSync(s.root+"/pkgs/c/project.json",'{"v":2}');
  s.executed.length=0;
  expect((await s.run("run",["i:build"])).served).toBe("executed");expect(s.executed).toEqual(["i:build"]);
  s.executed.length=0;const again=await s.run("run",["c:build"]);expect(again.served).toBe("executed");expect(s.executed).toEqual(["c:build"]);
  expect((await s.run("run",["c:build"])).served).toBe("memo");
 },20000);
});

describe("parallel tasks",()=>{
 test("lines of tasks that run at the same time never interleave inside a line, a carriage return counts as a line end and the last partial line is flushed",async()=>{
  const s=await startService({emit:async(id,out)=>{
   if(id==="a:build"){await out("stdout","a-one ");await sleep(60);await out("stdout","a-two\n");await out("stderr","a-err-partial");await sleep(60);await out("stdout","a-tail");}
   else{await sleep(20);await out("stdout","d-whole\n");await out("stdout","d-progress 50%\r");await out("stdout","d-done\n");}
  }});
  const r=await s.run("run-many",["a","d","--targets=build"]);
  expect(r.exit).toBe(0);
  const lines=r.out.split(/(?<=[\n\r])/u).filter(Boolean).sort();
  expect(lines).toEqual(["a-one a-two\n","a-tail","d-done\n","d-progress 50%\r","d-whole\n"].sort());
  expect(r.err).toBe("a-err-partial");
 });
});

describe("large output",()=>{
 test("a 900 KB task output is chunked, replayed from the memo byte for byte, and survives the process client without truncation",async()=>{
  const big=("line äö😀 "+"x".repeat(90)+"\n").repeat(9000);
  const s=await startService({execution:async()=>0});
  s.ports.execution.run=async({id,onSpawn,onOutput}:any)=>{onSpawn?.();await onOutput("stdout",big);mkdirSync(join(s.root,".out/d"),{recursive:true});writeFileSync(join(s.root,".out/d/o.txt"),"x");return{exit:0};};
  const first=await s.run("run",["d:build"]);expect(first.out).toBe(big);
  const again=await s.run("run",["d:build"]);expect(again.served).toBe("memo");
  const done=await new Promise<{status:number|null;stdout:string}>(resolve=>{const c=spawn("bun",[join(import.meta.dir,"..","client","main.mjs"),"--root",s.root,"--","status"],{env:{...process.env,XDG_RUNTIME_DIR:s.base}});let stdout="";c.stdout.on("data",d=>stdout+=d);c.on("exit",status=>resolve({status,stdout}));});
  expect(done.status).toBe(0);expect(JSON.parse(done.stdout).state.files).toBeGreaterThan(0);
 },30000);
});

describe("environment binding",()=>{
 test("a request whose tool or mode differs from the service's is refused with fallback, a matching one is served",async()=>{
  const previous=process.env.SEMIO_BUILD_TOOL;delete process.env.SEMIO_BUILD_TOOL;
  try{
   const s=await startService();
   const other=await s.run("run",["d:build"],{env:{SEMIO_BUILD_TOOL:"bazel"}});
   expect(other.handled).toBe(false);expect(other.reason).toBe("environment");
   expect((await s.run("run",["d:build"],{env:{}})).handled).toBe(true);
  }finally{if(previous!==undefined)process.env.SEMIO_BUILD_TOOL=previous;}
 });
});

describe("capabilities, languages and protocol abuse",()=>{
 test("no, forged, expired and out-of-scope tokens are refused with both languages; narrower tokens work",async()=>{
  const s=await startService();const key=keyOf(s.runtime);
  const ask=async(cap:string|undefined,command="run",args=["d:build"])=>{const c=raw(s.runtime);c.send(hello(["de"],cap));c.send({t:"request",id:1,command,args,cwd:s.root});const f=await c.wait(x=>x.t==="refused"||x.t==="result");c.socket.destroy();return f;};
  expect((await ask(undefined))).toMatchObject({t:"refused",code:"capability",retry:"never"});
  const refusal=await ask("v1.eyJpZCI6IjAwMDAwMDAwMDAwMDAwMDAiLCJjYXZlYXRzIjpbXX0."+"A".repeat(43));expect(refusal.message.en.length).toBeGreaterThan(5);expect(refusal.message.de).not.toBe(refusal.message.en);
  expect(await ask(mintCapabilityV1(key,[{k:"notAfter",v:Date.now()-1}]))).toMatchObject({code:"capability"});
  expect(await ask(mintCapabilityV1(key,[{k:"scope",v:["status"]}]))).toMatchObject({code:"scope"});
  const full=mintCapabilityV1(key,[]);expect(await ask(full)).toMatchObject({t:"result",exit:0});
  const projectBound=attenuateCapabilityV1(full,{k:"projects",v:["d"]});
  expect(await ask(projectBound)).toMatchObject({t:"result",exit:0});expect(await ask(projectBound,"run",["a:build"])).toMatchObject({code:"scope"});
  expect(await ask(attenuateCapabilityV1(projectBound,{k:"scope",v:["status"]}))).toMatchObject({code:"scope"});
  expect(await ask(launcherCapabilityV1(key),"issue",[])).toMatchObject({code:"scope"});
  const issued=await ask(mintCapabilityV1(key,[{k:"scope",v:["admin","run"]}]),"issue",["--classes","run","--for","a","--ttl","60"]);
  expect(issued).toMatchObject({t:"result"});
 });

 test("an issued token carries its restrictions",async()=>{
  const s=await startService();const key=keyOf(s.runtime);
  const admin=mintCapabilityV1(key,[]);const out:string[]=[];
  const r=await forwardCommandV1({root:s.root,command:"issue",args:["--agent","a1","--classes","run","--for","a","--ttl","60"],cwd:s.root,languages:["en"],cap:admin,environment:{XDG_RUNTIME_DIR:s.base},stdout:{write:(t:string)=>out.push(t)},stderr:{write(){}}});
  expect(r.exit).toBe(0);const token=out.join("").trim();
  expect((await s.run("run",["a:build"],{cap:token,lane:"a1"})).exit).toBe(0);
  expect((await s.run("run",["d:build"],{cap:token,lane:"a1"})).refused).toBe("scope");
  expect((await s.run("status",[],{cap:token,lane:"a1"})).refused).toBe("scope");
 });

 test("unsupported languages are refused in both languages; requested languages are honoured",async()=>{
  const s=await startService();const key=keyOf(s.runtime),cap=mintCapabilityV1(key,[]);
  const bad=raw(s.runtime);bad.send({...hello(["en"],cap),languages:["fr"]});
  expect(await bad.wait(f=>f.t==="refused")).toMatchObject({code:"invalid"});
  const none=raw(s.runtime);none.send({...hello(["en"],cap),languages:["en"]});
  const de=raw(s.runtime);de.send(hello(["de"],cap));de.send({t:"request",id:1,command:"run",args:["a:build"],cwd:s.root});
  const done=await de.wait(f=>f.t==="result");expect(done).toBeDefined();
  const progress=de.frames.filter(f=>f.t==="progress");expect(progress.length).toBeGreaterThan(0);
  for(const p of progress)expect(Object.keys(p.message)).toEqual(["de"]);
 });

 test("protocol abuse: first frame, double hello, oversize, garbage, invalid UTF-8, id reuse, unknown command",async()=>{
  const s=await startService();const cap=mintCapabilityV1(keyOf(s.runtime),[]);
  const noHello=raw(s.runtime);noHello.send({t:"ping",n:1});expect(await noHello.wait(f=>f.t==="refused")).toMatchObject({code:"invalid"});
  const twice=raw(s.runtime);twice.send(hello(["en"],cap));twice.send(hello(["en"],cap));expect(await twice.wait(f=>f.t==="refused")).toMatchObject({code:"invalid"});
  const small=await startService({policy:{maximumFrameBytes:4096}});const bigCap=mintCapabilityV1(keyOf(small.runtime),[]);
  const big=raw(small.runtime);big.send(hello(["en"],bigCap));big.send('{"t":"request","id":1,"command":"run","args":["'+"x".repeat(20_000)+'"],"cwd":"/"}\n');
  expect(await big.wait(f=>f.t==="refused")).toMatchObject({code:"invalid"});expect(await waitFor(()=>big.closed(),2000)).toBe(true);
  const junk=raw(s.runtime);junk.send("not json at all\n");expect(await junk.wait(f=>f.t==="refused")).toMatchObject({code:"invalid"});
  const bytes=raw(s.runtime);bytes.send(Buffer.from([0x7b,0xff,0x7d,10]) as any);expect(await bytes.wait(f=>f.t==="refused")).toMatchObject({code:"invalid"});
  const reuse=raw(s.runtime);reuse.send(hello(["en"],cap));reuse.send({t:"request",id:7,command:"watch",args:["d:build"],cwd:s.root});reuse.send({t:"request",id:7,command:"ping",args:[],cwd:s.root});
  expect(await reuse.wait(f=>f.t==="refused")).toMatchObject({code:"invalid"});reuse.send({t:"cancel",id:7});expect(await reuse.wait(f=>f.t==="result"&&f.id===7)).toMatchObject({exit:130});
  const unknown=raw(s.runtime);unknown.send(hello(["en"],cap));unknown.send({t:"request",id:1,command:"frobnicate",args:[],cwd:s.root});
  expect(await unknown.wait(f=>f.t==="refused")).toMatchObject({code:"unknown-command",retry:"fallback"});
  const ping=raw(s.runtime);ping.send(hello(["en"],cap));ping.send({t:"ping",n:9});expect(await ping.wait(f=>f.t==="pong")).toMatchObject({n:9});
  expect((await s.run("ping",[])).out).toBe("pong\n");
 });
});

describe("agent sessions: issue, revoke, and refusals of the authority core",()=>{
 const slow=(ms:number)=>(id:string,_root:string,signal:AbortSignal)=>new Promise<number>(resolve=>{const timer=setTimeout(()=>resolve(0),ms);signal.addEventListener("abort",()=>{clearTimeout(timer);resolve(1);},{once:true});});
 const issue=async(s:any,...args:string[])=>{const out:string[]=[];const r=await forwardCommandV1({root:s.root,command:"issue",args,cwd:s.root,languages:["en"],cap:mintCapabilityV1(keyOf(s.runtime),[]),environment:{XDG_RUNTIME_DIR:s.base},stdout:{write:(t:string)=>out.push(t)},stderr:{write(){}}});expect(r.exit).toBe(0);return out.join("").trim();};
 const ask=async(s:any,cap:string,lane:string,command="run",args=["d:build"],wait=(f:any)=>f.t==="refused"||f.t==="result")=>{const c=raw(s.runtime);c.send({...hello(["en"],cap),client:{name:"t",pid:1,lane}});c.send({t:"request",id:1,command,args,cwd:s.root});const f=await c.wait(wait);c.socket.destroy();return f;};

 test("an issued token is bound to its worktree and agent; another lane is refused with scope",async()=>{
  const s=await startService(),token=await issue(s,"--agent","codex-1","--classes","run","--ttl","60");
  expect(verifyCapabilityV1(keyOf(s.runtime),token)!.filter((c:any)=>c.k==="worktree"||c.k==="agent")).toEqual([{k:"worktree",v:worktreeIdV1(s.root)},{k:"agent",v:"codex-1"}]);
  expect(await ask(s,token,"codex-1")).toMatchObject({t:"result",exit:0});
  expect(await ask(s,token,"someone-else","run",["a:build"])).toMatchObject({t:"refused",code:"scope"});
  const other=mintCapabilityV1(keyOf(s.runtime),[{k:"scope",v:["run"]},{k:"worktree",v:"0".repeat(12)}]);
  expect(await ask(s,other,"codex-1")).toMatchObject({t:"refused",code:"scope"});
 });

 test("issue defaults to everything but administration for an hour, and refuses a token without an agent or with a lifetime beyond the ceiling, in the language asked for",async()=>{
  const s=await startService(),admin=mintCapabilityV1(keyOf(s.runtime),[]);
  const call=async(args:string[],languages=["en"])=>{const out:string[]=[],err:string[]=[];const r=await forwardCommandV1({root:s.root,command:"issue",args,cwd:s.root,languages,cap:admin,environment:{XDG_RUNTIME_DIR:s.base},stdout:{write:(t:string)=>out.push(t)},stderr:{write:(t:string)=>err.push(t)}});return{exit:r.exit,out:out.join("").trim(),err:err.join("")};};
  const before=Date.now(),issued=await call(["--agent","codex-1"]);expect(issued.exit).toBe(0);
  const caveats=verifyCapabilityV1(keyOf(s.runtime),issued.out)!,expiry=(caveats.find((c:any)=>c.k==="notAfter") as any).v;
  expect(caveats.find((c:any)=>c.k==="scope")).toEqual({k:"scope",v:["status","query","run","watch"]});expect(expiry-before).toBeGreaterThanOrEqual(3_600_000-50);expect(expiry-before).toBeLessThan(3_600_000+5_000);
  expect((await call([])).exit).toBe(1);expect((await call(["--agent","a b"])).exit).toBe(1);expect((await call(["--agent","x"],["de"]))).toMatchObject({exit:0});
  const tooLong=await call(["--agent","x","--ttl","86401"]);expect(tooLong.exit).toBe(1);expect(tooLong.err).toContain("86400");
  expect((await call(["--agent","x","--ttl","86400"])).exit).toBe(0);expect((await call(["--agent","x","--ttl","0"])).exit).toBe(1);expect((await call(["--agent","x","--ttl","abc"])).exit).toBe(1);
  expect((await call([],["de"])).err).toContain("braucht --agent");
 });

 test("the client identity reaches the execution port: token, agent lane and the named projects",async()=>{
  const s=await startService(),token=await issue(s,"--agent","codex-1","--ttl","60");
  await ask(s,token,"codex-1","run",["b:build"]);
  expect(s.clients.length).toBe(2);
  for(const client of s.clients)expect(client).toEqual({token,agent:"codex-1",named:["b"]});
 });

 test("revoking a session cancels its running command and refuses its later requests; other sessions are unaffected; a restart keeps the revocation",async()=>{
  const s=await startService({execution:slow(30000)}),a=await issue(s,"--agent","a1","--ttl","60"),b=await issue(s,"--agent","b1","--ttl","60"),admin=mintCapabilityV1(keyOf(s.runtime),[]);
  const running=raw(s.runtime);running.send({...hello(["en"],a),client:{name:"t",pid:1,lane:"a1"}});running.send({t:"request",id:1,command:"run",args:["d:build"],cwd:s.root});
  await running.wait(f=>f.t==="progress"&&f.phase==="execute");
  const out:string[]=[];const revoke=await forwardCommandV1({root:s.root,command:"revoke",args:[a],cwd:s.root,languages:["en"],cap:admin,environment:{XDG_RUNTIME_DIR:s.base},stdout:{write:(t:string)=>out.push(t)},stderr:{write(){}}});
  expect(revoke.exit).toBe(0);expect(out.join("")).toContain("revoked session "+capabilityIdV1(a)+", 1 running");
  const refusal=await running.wait(f=>f.t==="refused");expect(refusal).toMatchObject({code:"capability"});expect(refusal.message.en).toContain("revoked");expect(refusal.message.de).toContain("widerrufen");
  expect(await ask(s,a,"a1")).toMatchObject({t:"refused",code:"capability"});
  expect((await waitFor(()=>s.aborted.includes("d:build"),2000))).toBe(true);
  expect(createRevocationRegistryV1(s.runtime.revoked).has(capabilityIdV1(a)!)).toBe(true);
  const audit=readFileSync(s.runtime.audit,"utf8").trim().split("\n").map(line=>JSON.parse(line));
  expect(audit).toEqual([{v:1,at:expect.any(Number),event:"revoke",session:capabilityIdV1(a),agent:"-",worktree:worktreeIdV1(s.root),operation:"revoke",args:[]}]);
  expect(JSON.stringify(audit)).not.toContain(a);
  expect(b).not.toBe(a);
 },20000);

 test("revoking an agent refuses every session of it, including self-minted launcher tokens",async()=>{
  const s=await startService(),admin=mintCapabilityV1(keyOf(s.runtime),[]);
  const launcher=launcherCapabilityV1(keyOf(s.runtime),Date.now(),{worktree:worktreeIdV1(s.root),agent:"codex-9"});
  expect(await ask(s,launcher,"codex-9")).toMatchObject({t:"result",exit:0});
  const out:string[]=[];await forwardCommandV1({root:s.root,command:"revoke",args:["--agent","codex-9"],cwd:s.root,languages:["en"],cap:admin,environment:{XDG_RUNTIME_DIR:s.base},stdout:{write:(t:string)=>out.push(t)},stderr:{write(){}}});
  expect(out.join("")).toContain("revoked agent codex-9");
  expect(await ask(s,launcher,"codex-9")).toMatchObject({t:"refused",code:"capability"});
  expect(await ask(s,launcher,"codex-8")).toMatchObject({t:"refused",code:"scope"});
 });

 test("the task closure of the named projects is what a client may have built on its behalf",()=>{
  const target=(dependsOn:string[])=>({execution:{kind:"command"},dependsOn});
  const graph={version:1,projects:{a:{name:"a",root:"pkgs/a",targets:{build:target([])}},b:{name:"b",root:"pkgs/b",targets:{build:target(["^build"])}},c:{name:"c",root:"pkgs/c",targets:{build:target(["^build"])}},d:{name:"d",root:"pkgs/d",targets:{build:target([])}}},dependencies:[{source:"b",target:"a"},{source:"c",target:"b"}]};
  const closure=(...named:string[])=>[...taskClosureV1(graph,named)].sort();
  expect([closure("c"),closure("b"),closure("a"),closure("d"),closure("a","d"),closure()]).toEqual([["a","b","c"],["a","b"],["a"],["d"],["a","d"],[]]);
 });

 test("a refusal of the authority core becomes a refusal of the request in both languages with the core's wire code; nothing falls back",async()=>{
  const wire=coreSchema.definitions.Policy.const.wire as Record<string,string>;
  const fields={caveat:"agent",reason:"x",task:"d:build",hours:"24"},fill=(template:string)=>template.replace(/\{(\w+)\}/gu,(_,key:string)=>(fields as Record<string,string>)[key]!);
  for(const code of Object.keys(wire)){
   const s=await startService({refusal:()=>({code,fields})}),token=await issue(s,"--agent","codex-1","--ttl","60");
   const frame=await ask(s,token,"codex-1"),messages=coreSchema.definitions.Policy.const.messages as Record<string,Record<string,string>>;
   expect([code,frame.t,frame.code,frame.retry]).toEqual([code,"refused",wire[code],code==="busy"?"later":"never"]);
   expect(frame.message).toEqual({en:fill(messages.en![code]!),de:fill(messages.de![code]!)});
  }
 },60000);
});

describe("cancellation, deadlines, governance",()=>{
 const slow=(ms:number)=>(id:string,_root:string,signal:AbortSignal)=>new Promise<number>(resolve=>{const timer=setTimeout(()=>resolve(0),ms);signal.addEventListener("abort",()=>{clearTimeout(timer);resolve(1);},{once:true});});

 test("cancel aborts the tool and answers 130; disconnect aborts too; deadline refuses",async()=>{
  const s=await startService({execution:slow(30000)});const cap=mintCapabilityV1(keyOf(s.runtime),[]);
  const c=raw(s.runtime);c.send(hello(["en"],cap));c.send({t:"request",id:1,command:"run",args:["d:build"],cwd:s.root});
  await c.wait(f=>f.t==="progress"&&f.phase==="execute");c.send({t:"cancel",id:1});
  expect(await c.wait(f=>f.t==="result")).toMatchObject({exit:130,signal:"SIGINT"});expect(s.aborted).toEqual(["d:build"]);
  const d=raw(s.runtime);d.send(hello(["en"],cap));d.send({t:"request",id:1,command:"run",args:["a:build"],cwd:s.root});
  await d.wait(f=>f.t==="progress"&&f.phase==="execute");d.socket.destroy();
  expect(await waitFor(()=>s.aborted.includes("a:build"),2000)).toBe(true);
  const e=raw(s.runtime);e.send(hello(["en"],cap));e.send({t:"request",id:1,command:"run",args:["b:build"],cwd:s.root,deadlineMs:150});
  expect(await e.wait(f=>f.t==="refused")).toMatchObject({code:"deadline"});
  expect(s.service.governor.inFlight).toBe(0);
 },20000);

 test("bounded concurrency, bounded queue with position reports, busy refusal, and per-lane fairness",async()=>{
  const gate:(()=>void)[]=[];
  const s=await startService({policy:{maximumInFlight:2,maximumQueued:3},execution:(id,_r,signal)=>new Promise<number>(resolve=>{gate.push(()=>resolve(0));signal.addEventListener("abort",()=>resolve(1),{once:true});})});
  const cap=mintCapabilityV1(keyOf(s.runtime),[]);
  const plan=[["A","e"],["A","f"],["A","g"],["A","a"],["B","h"],["A","d"],["B","b"]] as const;
  const clients=plan.map(([lane],i)=>{const c=raw(s.runtime);c.send({...hello(["en"],cap),client:{name:"t",pid:i,lane}});return c;});
  for(const[i,[,owner]]of plan.entries()){clients[i]!.send({t:"request",id:1,command:"run",args:[owner+":build"],cwd:s.root});await waitFor(()=>clients[i]!.frames.some(f=>f.t==="accepted"||f.t==="refused"),2000);await sleep(30);}
  await sleep(200);
  const busy=clients.filter(c=>c.frames.some(f=>f.t==="refused"&&f.code==="busy"));
  expect(busy.length).toBe(2);expect(busy[0]!.frames.find(f=>f.t==="refused").retry).toBe("later");
  expect(s.service.governor.inFlight).toBe(2);expect(s.service.governor.queued).toBe(3);
  const queued=clients.flatMap(c=>c.frames.filter(f=>f.t==="progress"&&f.phase==="queued"));expect(queued.length).toBeGreaterThan(0);
  expect(Math.max(...queued.map(f=>f.queue))).toBeLessThanOrEqual(3);
  expect(s.executed).toEqual(["e:build","f:build"]);
  for(let i=0;i<40&&clients.filter(c=>c.frames.some(f=>f.t==="result")).length<5;i++){gate.splice(0).forEach(g=>g());await sleep(60);}
  expect(clients.filter(c=>c.frames.some(f=>f.t==="result")).length).toBe(5);
  expect(s.executed.slice(2).sort()).toEqual(["a:build","g:build","h:build"]);
  expect(s.executed.indexOf("h:build")).toBeLessThan(s.executed.indexOf("a:build"));
 },20000);
});

describe("lifecycle",()=>{
 test("one instance per worktree; a stale socket from a crashed instance is replaced; an untrusted directory is refused",async()=>{
  const s=await startService();
  const second=createResidentServiceV1({root:s.root,runtime:s.runtime,ports:s.ports,implementation:"1".repeat(64),backend:"none",exclude:{rooted:[],names:[]},fenceDirectory:".semio-fence",policy:POLICY_V1});
  await expect(second.start()).rejects.toBeInstanceOf(ServiceRunningErrorV1);
  await s.service.stop("test");
  expect(existsSync(s.runtime.endpoint)).toBe(false);expect(existsSync(s.runtime.descriptor)).toBe(false);
  const dead=createServer();await new Promise<void>(r=>dead.listen(s.runtime.endpoint,()=>r()));
  (dead as any)._handle?.close?.();dead.close();
  writeFileSync(s.runtime.endpoint+".marker","x");
  const again=createResidentServiceV1({root:s.root,runtime:s.runtime,ports:s.ports,implementation:"1".repeat(64),backend:"none",exclude:{rooted:[],names:[]},fenceDirectory:".semio-fence",policy:POLICY_V1});
  await again.start();expect((await forwardCommandV1({root:s.root,command:"ping",args:[],cwd:s.root,languages:["en"],environment:{XDG_RUNTIME_DIR:s.base},stdout:{write(){}},stderr:{write(){}}})).handled).toBe(true);
  await again.stop("test");
  if(process.platform!=="win32"){
   chmodSync(s.runtime.directory,0o755);
   const bad=createResidentServiceV1({root:s.root,runtime:s.runtime,ports:s.ports,implementation:"1".repeat(64),backend:"none",exclude:{rooted:[],names:[]},fenceDirectory:".semio-fence",policy:POLICY_V1});
   await expect(bad.start()).rejects.toThrow(/private directory/u);
   expect((await forwardCommandV1({root:s.root,command:"ping",args:[],cwd:s.root,languages:["en"],environment:{XDG_RUNTIME_DIR:s.base},stdout:{write(){}},stderr:{write(){}}})).handled).toBe(false);
  }
 });

 test("idle shutdown, and the pin is released when idle and reacquired on demand",async()=>{
  const s=await startService({policy:{idleShutdownMilliseconds:700,pinReleaseMilliseconds:200,residentCheckMilliseconds:100}});
  await s.run("run",["d:build"]);expect(s.custodyEvents).toEqual(["acquire"]);
  expect(await waitFor(()=>s.custodyEvents.includes("release"),3000)).toBe(true);
  await s.run("run",["d:build"]);expect(s.custodyEvents.filter(e=>e==="acquire").length).toBe(2);
  const reason=await Promise.race([s.service.closed,sleep(8000).then(()=>"timeout")]);expect(reason).toBe("idle");
  expect(existsSync(s.runtime.endpoint)).toBe(false);
 },20000);

 test("a changed implementation file makes the service refuse with fallback, drain, and exit",async()=>{
  const root=tempRoot("svc-stale-");cleanups.push(()=>cleanup(root));
  writeTree(root,{"package.json":"{}","pkgs/a/src/x.ts":"a","pkgs/b/src/x.ts":"b","pkgs/c/src/x.ts":"c","pkgs/d/src/x.ts":"d","tools/service.mjs":"v1"});
  const s=await startService({root,implementationPaths:["tools/service.mjs"],backend:native?"auto":"none"});
  if(!native)return;
  expect((await s.run("ping",[])).handled).toBe(true);
  appendFileSync(root+"/tools/service.mjs","v2");
  const reason=await Promise.race([s.service.closed,sleep(5000).then(()=>"timeout")]);expect(reason).toBe("stale");
  expect((await s.run("ping",[])).handled).toBe(false);
 },20000);

 test("memory pressure drops the caches first",async()=>{
  const s=await startService({policy:{maximumResidentBytes:1,residentCheckMilliseconds:100}});
  await s.run("run",["a:build"]);
  const outcome=await Promise.race([waitFor(()=>s.service.state.memo.size()===0,3000).then(ok=>ok?"dropped":"kept"),s.service.closed]);
  expect(["dropped","memory"]).toContain(outcome);
 });
});

describe("degraded trust stays correct",()=>{
 for(const backend of["none","poll"])test(`backend ${backend}: silent edits are found by stat proof`,async()=>{
  const s=await startService({backend,binary:undefined});
  await s.run("run",["c:build"]);expect((await s.run("run",["c:build"])).served).toBe("memo");
  await sleep(30);writeFileSync(s.root+"/pkgs/b/src/x.ts","changed silently and longer");
  s.executed.length=0;const r=await s.run("run",["c:build"]);expect(r.served).toBe("executed");expect([...s.executed].sort()).toEqual(["b:build","c:build"]);
  rmSync(s.root+"/.out/c",{recursive:true});s.executed.length=0;await s.run("run",["c:build"]);expect(s.executed).toEqual(["c:build"]);
 },20000);

 test.skipIf(!native)("killing the watcher process mid-session never produces a wrong answer, and the service recovers",async()=>{
  const s=await startService();await s.run("run",["c:build"]);
  const pid=Number(spawnSync("pgrep",["-f","semio-repo-watch watch --root "+s.root]).stdout.toString().trim().split("\n")[0]);
  expect(pid).toBeGreaterThan(1);process.kill(pid,"SIGKILL");
  appendFileSync(s.root+"/pkgs/a/src/x.ts","while the watcher was dead");
  s.executed.length=0;const r=await s.run("run",["c:build"]);expect(r.served).toBe("executed");expect(s.executed.includes("a:build")).toBe(true);
  expect(await waitFor(()=>s.service.watcher().state().trust==="exact",8000)).toBe(true);
  expect((await s.run("run",["c:build"])).served).toBe("memo");
 },30000);
});

describe.skipIf(!native)("process level crash safety",()=>{
 test("kill -9 of the service process leaves no watcher behind, clients fall back at once, a new service takes over",async()=>{
  const root=tempRoot("svc-crash-");cleanups.push(()=>cleanup(root));
  writeTree(root,{"package.json":"{}","tsconfig.base.json":"{}","bun.lock":"l","pkgs/a/src/x.ts":"a"});
  writeFileSync(root+"/graph.json",JSON.stringify({generation:1,owners:[owners[0]],env:{paths:["package.json"],prefixes:[]}}));
  const base=mkdtempSync("/tmp/sr-");cleanups.push(()=>cleanup(base));
  const serve=join(import.meta.dir,"..","..","bench","serve.mjs");
  const env={...process.env,XDG_RUNTIME_DIR:base};
  const first=spawn("bun",[serve,root,"--bin",join(WATCHER,".."),"--js-scan"],{env,stdio:["ignore","pipe","inherit"]});
  const line=await new Promise<string>(resolve=>first.stdout.once("data",c=>resolve(String(c))));
  expect(JSON.parse(line.split("\n")[0]!).ready).toBe(true);
  const watcherPid=Number(spawnSync("pgrep",["-f","semio-repo-watch watch --root "+root]).stdout.toString().trim().split("\n")[0]);
  expect(watcherPid).toBeGreaterThan(1);
  first.kill("SIGKILL");
  expect(await waitFor(()=>{try{process.kill(watcherPid,0);return false;}catch{return true;}},4000)).toBe(true);
  const probe=await forwardCommandV1({root,command:"ping",args:[],cwd:root,languages:["en"],environment:env,stdout:{write(){}},stderr:{write(){}}});
  expect(probe.handled).toBe(false);
  const second=spawn("bun",[serve,root,"--bin",join(WATCHER,".."),"--js-scan"],{env,stdio:["ignore","pipe","inherit"]});
  const line2=await new Promise<string>(resolve=>second.stdout.once("data",c=>resolve(String(c))));
  expect(JSON.parse(line2.split("\n")[0]!).ready).toBe(true);
  second.kill("SIGTERM");await new Promise(r=>second.once("exit",r));
 },30000);
});
