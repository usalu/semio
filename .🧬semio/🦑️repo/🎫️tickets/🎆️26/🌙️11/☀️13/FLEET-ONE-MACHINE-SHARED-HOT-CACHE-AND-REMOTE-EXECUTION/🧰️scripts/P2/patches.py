import sys,os
sys.path.insert(0,os.path.dirname(__file__))
from pathutil import resolve
B=resolve("🧰framework/🛍products/🦑repo/🔨modules/📚library/🏗build")
G=resolve(B,"🕸graph/🕰generations")
GR=resolve(B,"🕸graph")
CUST=resolve(G,"🔐custody")
LOAD=resolve(G,"🔌load")
HOST=resolve(GR,"🕰host/🟦.ts")
PREF=resolve(GR,"🤝prefetch/🟦.ts")
PATCHES={}
NEW={}
MIRROR_ONLY={}

PATCHES[CUST+"/🧬schema/🔣.json".replace("🧬schema","🧬️schema") if False else resolve(CUST,"🧬schema/🔣.json")]=[
 ('"widenRounds": 8\n','"widenRounds": 8,\n        "attachPollMilliseconds": 5000,\n        "attachRecheckMilliseconds": 120000\n'),
 ('"blockedAll": "graph: {peers} refresh(es) go first, one of them rebuilds the whole workspace ({seconds} s)"\n',
  '"blockedAll": "graph: {peers} refresh(es) go first, one of them rebuilds the whole workspace ({seconds} s)",\n          "attached": "graph: attached to the graph build of another command on this machine (generation {generation}, {seconds} s), no second build is started",\n          "attachedWave": "graph: the build this command is attached to published generation {generation} ({waves} wave(s), {seconds} s)",\n          "attachedServed": "graph: generation {generation}, published by the build this command was attached to, serves it ({seconds} s)"\n'),
 ('"blockedAll": "Graph: {peers} Erneuerung(en) sind zuerst dran, eine davon baut den ganzen Arbeitsbereich neu ({seconds} s)"\n',
  '"blockedAll": "Graph: {peers} Erneuerung(en) sind zuerst dran, eine davon baut den ganzen Arbeitsbereich neu ({seconds} s)",\n          "attached": "Graph: an den Graph-Aufbau eines anderen Befehls auf diesem Rechner angehängt (Generation {generation}, {seconds} s), es wird kein zweiter Aufbau gestartet",\n          "attachedWave": "Graph: der Aufbau, an den dieser Befehl angehängt ist, hat Generation {generation} veröffentlicht ({waves} Welle(n), {seconds} s)",\n          "attachedServed": "Graph: Generation {generation}, vom Aufbau veröffentlicht, an den dieser Befehl angehängt war, bedient ihn ({seconds} s)"\n'),
 ('Waiting is event driven (the leases of the blocking claims) with a poll of `pollMilliseconds`; a waiter tells why at most every `noticeMilliseconds`, naming up to `sampleOwners` shared owners.",',
  'Waiting is event driven (the leases of the blocking claims) with a poll of `pollMilliseconds`; a waiter tells why at most every `noticeMilliseconds`, naming up to `sampleOwners` shared owners. A reader whose stale projects a live claim already refreshes does not prepare them beside the builder: it ATTACHES (`attachToBuilderV1`), looks every `attachPollMilliseconds` at the custody and the generation pointer, tells each wave the builder publishes and, at most every `attachRecheckMilliseconds`, tries to be served from the generation published so far; it ends when no live claim holds the custody against it any more.",'),
]

PATCHES[resolve(CUST,"🟨.d.mts")]=[
 ('export type CustodyMessageKeyV1="blocked"|"blockedAll";','export type CustodyMessageKeyV1="blocked"|"blockedAll"|"attached"|"attachedWave"|"attachedServed";'),
 ('/** 🏗️ Creates the slice custody of the generation documents under `root`. */',
  '/** 🧲️ What an attached reader sees of the custody: whether a live claim holds it against the reader and the newest generation. */\nexport interface CustodyLookV1{readonly held:boolean;readonly generation:number|undefined}\n/** 🧲️ Attaches a reader to the live builder of its stale projects instead of preparing them beside it; answers what `recheck` served, or undefined when nothing holds the custody against the reader (any more). */\nexport function attachToBuilderV1<T>(options:{look:()=>Promise<CustodyLookV1>;recheck:()=>Promise<T|undefined>;say?:(key:CustodyMessageKeyV1,fields:Record<string,string|number>)=>void;signal?:AbortSignal;now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;policy?:Pick<CustodyPolicyV1,"attachPollMilliseconds"|"attachRecheckMilliseconds"|"noticeMilliseconds">}):Promise<T|undefined>;\n/** 🏗️ Creates the slice custody of the generation documents under `root`. */'),
]

PATCHES[resolve(CUST,"🟨.d.mts")].append(('readonly sampleOwners:number;readonly widenRounds:number}','readonly sampleOwners:number;readonly widenRounds:number;readonly attachPollMilliseconds:number;readonly attachRecheckMilliseconds:number}'))

ATTACH_MJS='''return Object.freeze({hold,idle,list:live,resource});
}

const pause=(milliseconds,signal)=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason??fail("Aborted","ABORT_ERR"));return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal.reason??fail("Aborted","ABORT_ERR"));},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

/**
 * 🧲️ Attaches a reader to the live builder of its stale projects: the second reader of a machine neither builds nor prepares the projects beside the builder, it looks every `attachPollMilliseconds` (`look()` answers whether a live claim holds the custody against the reader and the newest generation), tells at once and then every notice interval that it is attached, tells each wave the builder publishes and at most every `attachRecheckMilliseconds` lets `recheck()` serve it from the generation published so far (any answer but undefined ends the attachment and is returned). It answers undefined as soon as nothing holds the custody against the reader any more, and when the custody cannot be looked at, so the caller builds or prepares as before: an attachment never fails a command, a cancellation is rethrown.
 */
export async function attachToBuilderV1({look,recheck,say=()=>{},signal,now=()=>Date.now(),sleep=pause,policy=P}){
 const started=now();let previous,waves=0,noticed=-Infinity,checked=started;
 for(;;){
  aborted(signal);
  let seen;
  try{seen=await look();}catch(error){if(signal?.aborted)throw error;return undefined;}
  if(!seen.held)return undefined;
  const seconds=Math.round((now()-started)/1000);
  if(seen.generation!==undefined&&previous!==undefined&&seen.generation>previous){
   waves+=seen.generation-previous;noticed=now();say("attachedWave",{generation:seen.generation,waves,seconds});
   if(now()-checked>=policy.attachRecheckMilliseconds){
    checked=now();let served;
    try{served=await recheck();}catch(error){if(signal?.aborted)throw error;}
    if(served!==undefined){say("attachedServed",{generation:seen.generation,seconds});return served;}
   }
  }else if(now()-noticed>=policy.noticeMilliseconds){noticed=now();say("attached",{generation:seen.generation??-1,seconds});}
  if(seen.generation!==undefined)previous=seen.generation;
  await sleep(policy.attachPollMilliseconds,signal);
 }
}'''
PATCHES[resolve(CUST,"🟨.mjs")]=[('return Object.freeze({hold,idle,list:live,resource});\n}',ATTACH_MJS)]

PATCHES[resolve(LOAD,"🟨.mjs")]=[
 ('if(host.fleetRead&&request)await host.fleetRead(request);\n',
  'if(host.fleetRead&&request)await host.fleetRead(request);\n if(host.attach&&request){const attached=await host.attach(request,async()=>{const again=await host.withShared((store,control)=>attempt(false,store,control));return again.needsBuild?undefined:again;});if(attached){await host.release?.();await publishRemoteGenerationV1(host,true);return{graph:attached.graph,stats:attached.stats};}}\n'),
]
PATCHES[resolve(LOAD,"🟨.d.mts")]=[
 (' prefetch?(request:{generation:number;dirty:readonly string[];roots:Readonly<Record<string,string>>;structural:boolean;implementation:string}):Promise<void>;\n',
  ' prefetch?(request:{generation:number;dirty:readonly string[];roots:Readonly<Record<string,string>>;structural:boolean;implementation:string}):Promise<void>;\n /** 🧲️ Attaches the command to the live builder of its stale projects on this machine (see `attachToBuilderV1`): `recheck` tries to serve the command from the generation published so far; answers what it served, or undefined when no live builder holds the custody against the command; optional. */\n attach?(request:{generation:number;dirty:readonly string[];roots:Readonly<Record<string,string>>;structural:boolean;implementation:string},recheck:()=>Promise<{graph:unknown;stats:GenerationLoadStatsV1}|undefined>):Promise<{graph:unknown;stats:GenerationLoadStatsV1}|undefined>;\n'),
 (' published?(generation:number):void;',' published?(generation:number):void|Promise<void>;'),
]

PATCHES[PREF]=[
 ('/** 🧵️ The owners of the previous generation that keep their fragment',
  '''/** 🧭️ What an attached reader looks at: whether a live claim of the builder custody (held or queued) conflicts with the owners of its request, and the generation the pointer names now; a failure answers by throwing and the attachment ends in the local read. */
export async function builderLookV1(input:{root:string;invocation:ScriptInvocation;phaseControl:ResourceLeaseControl;request:GenerationPrefetchRequestV1}):Promise<{held:boolean;generation:number|undefined}> {
  const{root,invocation,phaseControl,request}=input;
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  return withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,generationScopeV1,async(store,control)=>{
    const idle=await generationCustodyV1(root,store,control).idle({owners:request.dirty,structural:request.structural,signal:invocation.control.signal}),bytes=await generationFilesV1(store).read(join(store.root,"generations",GENERATION_POLICY_V1.pointer)) as Uint8Array|undefined;
    return{held:!idle,generation:bytes?readGenerationPointerV1(bytes).generation:undefined};
  },{optionalCaches:false,shared:true});
}


/** 🧵️ The owners of the previous generation that keep their fragment'''),
]
PATCHES[HOST]=[
 ('import {type SliceClaimV1} from "../🕰️generations/🔐️custody/🟨️.mjs";','import {attachToBuilderV1,renderCustodyMessageV1,type SliceClaimV1} from "../🕰️generations/🔐️custody/🟨️.mjs";'),
 ('builderIdleV1,previousEnvironmentOwnersV1,','builderIdleV1,builderLookV1,previousEnvironmentOwnersV1,'),
 ('    builderIdle:(request:GenerationPrefetchRequestV1)=>builderIdleV1({root,invocation,phaseControl,request}),\n',
  '    builderIdle:(request:GenerationPrefetchRequestV1)=>builderIdleV1({root,invocation,phaseControl,request}),\n    attach:<T>(request:GenerationPrefetchRequestV1,recheck:()=>Promise<T|undefined>)=>attachToBuilderV1<T>({look:()=>builderLookV1({root,invocation,phaseControl,request}),recheck,say:(key,fields)=>{process.stderr.write(renderCustodyMessageV1(key,fields,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language as "en"|"de")+"\\n");},signal:invocation.control.signal}),\n'),
]

ATTACH_TESTS='''
 await a.release();const again:SliceClaimV1=await custody.hold({owners:["a"]});await again.release();
});

const attachPolicy={...CUSTODY_POLICY_V1,attachPollMilliseconds:20,attachRecheckMilliseconds:150,noticeMilliseconds:60};
const lookOf=(custody:ReturnType<typeof world>["custody"],pointer:()=>number,owners:string[])=>async()=>({held:!await custody.idle({owners}),generation:pointer()});

test("a second reader on one state root attaches to the live builder: it never holds the custody beside it, tells the waves in English and German, is served from a wave through the recheck, and ends when the builder lets go",async()=>{
 const{custody,say,said}=world(),builder=await custody.hold({owners:["p1","p2","p3"]});
 let generation=0,rechecks=0,serve=false;
 const attached=attachToBuilderV1({look:lookOf(custody,()=>generation,["p2","p4"]),recheck:async()=>{rechecks++;return serve?{graph:"g"}:undefined;},say,policy:attachPolicy});
 await sleep(60);expect(said.map(row=>row.key)).toEqual(["attached"]);
 expect((await custody.list()).map(row=>[row.token,row.state])).toEqual([[builder.token,"held"]]);
 generation=1;await sleep(60);expect(said.map(row=>row.key)).toEqual(["attached","attachedWave"]);expect(rechecks,"a wave inside the interval is only told").toBe(0);
 await sleep(120);generation=3;await sleep(60);expect(rechecks).toBe(1);expect(said.filter(row=>row.key==="attachedWave").at(-1)!.text).toContain("3 wave(s)");
 expect(said.every(row=>row.text.startsWith("graph: "))).toBe(true);
 expect(renderCustodyMessageV1("attached",{generation:2,seconds:5},"de")).toContain("angehängt");expect(renderCustodyMessageV1("attachedWave",{generation:2,waves:1,seconds:5},"de")).toContain("Welle(n)");
 serve=true;await sleep(170);generation=4;expect(await attached).toEqual({graph:"g"});expect(rechecks).toBe(2);expect(said.at(-1)!.key).toBe("attachedServed");
 expect((await custody.list()).length).toBe(1);await builder.release();
 const left=await attachToBuilderV1({look:lookOf(custody,()=>generation,["p2"]),recheck:async()=>({graph:"never"}),policy:attachPolicy});
 expect(left,"nothing holds the custody against the reader any more").toBeUndefined();
 const next=await custody.hold({owners:["p2"]});expect((await custody.list()).length).toBe(1);await next.release();
});

test("an attached reader ends without a result when the builder releases, when the custody cannot be looked at, and rethrows a cancellation",async()=>{
 const{custody}=world(),builder=await custody.hold({owners:["p1"]}),started=Date.now();
 const waiting=attachToBuilderV1({look:lookOf(custody,()=>1,["p1"]),recheck:async()=>undefined,policy:attachPolicy});
 await sleep(50);await builder.release();expect(await waiting).toBeUndefined();expect(Date.now()-started).toBeLessThan(2000);
 expect(await attachToBuilderV1({look:async()=>{throw new Error("store gone");},recheck:async()=>({graph:"x"}),policy:attachPolicy})).toBeUndefined();
 const held=await custody.hold({owners:["p1"]}),controller=new AbortController();
 const stopped=attachToBuilderV1({look:lookOf(custody,()=>1,["p1"]),recheck:async()=>undefined,signal:controller.signal,policy:attachPolicy});
 await sleep(40);controller.abort(new Error("cancelled"));await expect(stopped).rejects.toThrow("cancelled");await held.release();
});

test("the recheck is asked at most once per interval and a failing recheck never ends the attachment",async()=>{
 const{custody}=world(),builder=await custody.hold({owners:["p1"]});let generation=0,asked=0;
 const attached=attachToBuilderV1({look:lookOf(custody,()=>generation,["p1"]),recheck:async()=>{asked++;throw new Error("verification failed");},policy:{...attachPolicy,attachRecheckMilliseconds:100}});
 for(let wave=1;wave<=8;wave++){generation=wave;await sleep(30);}
 expect(asked).toBeGreaterThanOrEqual(1);expect(asked).toBeLessThanOrEqual(3);
 await builder.release();expect(await attached).toBeUndefined();
});
'''
PATCHES[resolve(CUST,"🧪tests/🟦.ts")]=[
 ('import {CUSTODY_MESSAGES_V1,CUSTODY_POLICY_V1,admissionOfV1,','import {CUSTODY_MESSAGES_V1,CUSTODY_POLICY_V1,admissionOfV1,attachToBuilderV1,'),
 ('\n await a.release();const again:SliceClaimV1=await custody.hold({owners:["a"]});await again.release();\n});\n',ATTACH_TESTS),
]
def live_here(rel): return os.path.join(os.environ.get("P2_LIVE","/Users/Shared/semio"),os.path.dirname(rel))
def pin_here(rel): return [('here=dirname(fileURLToPath(import.meta.url))','here=%r'%live_here(rel))]
MIRROR_ONLY[resolve(CUST,"🧪tests/🟦.ts")]=pin_here(resolve(CUST,"🧪tests/🟦.ts"))

IDLE=resolve(LOAD,"🧪tests/🪑idle-builder/🟦.ts")
LOAD_TESTS='''expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);expect(asked).toEqual([]);expect(requests).toEqual([]);
 }finally{clean(dir);}
},60000);

async function twoReaders(dir:string,reader:{recheckMilliseconds:number;expandDelayMs:number}){
 await seed(dir,ringFiles(6));
 (await loadGenerationGraphV1(createMockHost(dir).host)).graph;
 const files=ringFiles(6);
 await applyOps(dir,[1,2,3,4].map(index=>({op:"write" as const,path:`pkg/p${index}/src/index.ts`,content:files[`pkg/p${index}/src/index.ts`]!.replace(/=\\d+;\\n$/u,"=77;\\n")})));
 const custody={tail:Promise.resolve()},order:string[]=[],lines:string[]=[];let holding=0,peak=0;
 const a=createMockHost(dir,{custody,ownerDelayMs:100}),b=createMockHost(dir,{custody});
 const track=(name:string,mock:ReturnType<typeof createMockHost>)=>{
  const host=mock.host as Record<string,any>,withBuilder=host.withBuilder.bind(host),expand=host.expand.bind(host);
  host.withBuilder=(body:(...input:unknown[])=>Promise<unknown>,request:unknown)=>withBuilder(async(...input:unknown[])=>{holding++;peak=Math.max(peak,holding);order.push(name+":held");try{return await body(...input);}finally{holding--;order.push(name+":done");}},request);
  if(name==="A")host.expand=async(projects:Record<string,unknown>)=>{await new Promise(resolve=>setTimeout(resolve,reader.expandDelayMs));return expand(projects);};
 };
 track("A",a);track("B",b);
 const pointer=()=>readGenerationPointerV1(readFileSync(join(dir,".gen","current.json"))).generation;
 const bHost=b.host as Record<string,any>;
 bHost.attach=(request:Request,recheck:()=>Promise<unknown>)=>attachToBuilderV1({look:async()=>({held:holding>0,generation:pointer()}),recheck,say:(key,fields)=>lines.push(key+":"+renderCustodyMessageV1(key,fields,"en")),policy:{...CUSTODY_POLICY_V1,attachPollMilliseconds:15,attachRecheckMilliseconds:reader.recheckMilliseconds,noticeMilliseconds:50}});
 const building=loadGenerationGraphV1(a.host);
 for(let waited=0;holding===0&&waited<400;waited++)await new Promise(resolve=>setTimeout(resolve,10));
 expect(holding,"the first reader holds the custody").toBe(1);
 const serving=await loadGenerationGraphV1(b.host);
 return{a,b,order,lines,peak:()=>peak,first:(await building).graph,second:serving.graph};
}

test("two readers on one state root: the second attaches to the builder, infers nothing itself, builds after it only to find the work done, and both graphs equal the Python model",async()=>{
 const dir=scratch(),previous=process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;process.env.SEMIO_SLICE_WAVE_BASIS_POINTS="200";
 try{
  const world=await twoReaders(dir,{recheckMilliseconds:3_600_000,expandDelayMs:0});
  expect(world.peak(),"never two builders at once").toBe(1);
  expect(world.order).toEqual(["A:held","A:done","B:held","B:done"]);
  expect(world.a.log.flatMap(row=>row.full).sort()).toEqual(["p1","p2","p3","p4"]);
  expect(world.b.log.flatMap(row=>row.full),"the attached reader infers nothing").toEqual([]);
  expect(world.b.events).not.toContain("prefetch");
  const keys=world.lines.map(line=>line.split(":")[0]);expect(keys[0]).toBe("attached");expect(keys).toContain("attachedWave");expect(keys).not.toContain("attachedServed");
  expect(world.lines.filter(line=>line.startsWith("attachedWave")).length).toBeGreaterThanOrEqual(2);
  expect(shape(world.first)).toEqual(pythonGraph(dir));expect(shape(world.second)).toEqual(pythonGraph(dir));
 }finally{if(previous===undefined)delete process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;else process.env.SEMIO_SLICE_WAVE_BASIS_POINTS=previous;clean(dir);}
},120000);

test("an attached reader takes the records as the waves publish: once the wave that covers it is published it is served from it before the builder has let go, and never builds",async()=>{
 const dir=scratch(),previous=process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;process.env.SEMIO_SLICE_WAVE_BASIS_POINTS="200";
 try{
  const world=await twoReaders(dir,{recheckMilliseconds:40,expandDelayMs:1500});
  expect(world.lines.map(line=>line.split(":")[0])).toContain("attachedServed");
  expect(world.order,"the second reader was served while the first still held the custody").toEqual(["A:held","A:done"]);
  expect(world.b.log.flatMap(row=>row.full)).toEqual([]);
  expect(shape(world.second)).toEqual(pythonGraph(dir));
 }finally{if(previous===undefined)delete process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;else process.env.SEMIO_SLICE_WAVE_BASIS_POINTS=previous;clean(dir);}
},120000);
'''
PATCHES[IDLE]=[
 ('import {loadGenerationGraphV1} from "../../🟨️.mjs";','import {loadGenerationGraphV1} from "../../🟨️.mjs";\nimport {readGenerationPointerV1} from "../../../🟨️.mjs";\nimport {CUSTODY_POLICY_V1,attachToBuilderV1,renderCustodyMessageV1} from "../../../🔐️custody/🟨️.mjs";'),
 ('import {mkdtempSync,realpathSync,rmSync} from "node:fs";','import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";'),
 ('expect(mock.events).toEqual(["shared","builder:wait","builder:held","builder:released"]);expect(asked).toEqual([]);expect(requests).toEqual([]);\n }finally{clean(dir);}\n},60000);\n',LOAD_TESTS),
 ('a command whose builder is busy prepares the stale owners first and carries the shared command closures through that preparation, and a host that cannot tell keeps the prefetch.','a command whose builder is busy and that cannot attach prepares the stale owners first and carries the shared command closures through that preparation, a host that cannot tell keeps the prefetch, and a command that can attach (a live builder of the same machine holds the custody against it) does not prepare anything beside that builder: it tells the waves, takes the records as they publish and builds only to find the work done.'),
]
MIRROR_ONLY[IDLE]=[('here=dirname(fileURLToPath(import.meta.url))','here=%r'%os.path.dirname(os.path.join("/Users/Shared/semio",IDLE)))]

HC=resolve("🧰framework/🔨modules/🏃process/🏗build/🌡host-capacity")
GATE=resolve(HC,"🎫gate/🟦.ts")
PATCHES[GATE]=[
 ('  const at=now(),active=activeAt(at);heartbeat(phase);transitions.push({phase:next,atMilliseconds:active});phase=next;\n',
  '  const at=now(),active=activeAt(at),left=phase;heartbeat(phase);transitions.push({phase:next,atMilliseconds:active});phase=next;\n'),
 ('  publish({...state,phase:next,...resized===undefined?{}:{reservedMegabytes:resized.reservedMegabytes,rampMilliseconds:resized.rampMilliseconds,resumedAtMs:at}});\n',
  '  const lasting=state.quiet!==true&&next==="rebuild"?{rampMilliseconds:Math.max(resized?.rampMilliseconds??state.rampMilliseconds??policy.memory.rampMilliseconds,policy.wait.staleMilliseconds),resumedAtMs:at}:state.quiet!==true&&left==="rebuild"&&resized===undefined?{rampMilliseconds:policy.memory.rampMilliseconds,resumedAtMs:at}:{};\n  publish({...state,phase:next,...resized===undefined?{}:{reservedMegabytes:resized.reservedMegabytes,rampMilliseconds:resized.rampMilliseconds,resumedAtMs:at},...lasting});\n'),
 ('A run that marks its phases (`markHostCapacityPhaseV1`) is cut into phase segments by the readings it was observed at, and its reservation follows the `resize` port at each boundary;',
  'A run that marks its phases (`markHostCapacityPhaseV1`) is cut into phase segments by the readings it was observed at, and its reservation follows the `resize` port at each boundary; a grant in its `rebuild` phase counts in full and lends the unused part of it to the workers that name it for as long as a grant may live (`wait.staleMilliseconds`, not only for the learned time to peak: a graph refresh of many waves runs for hours), so the farm workers of a build always fit inside its grant and a second rebuild of that size is not admitted beside it unless it fits physically;'),
]
PHG=resolve(HC,"🧪tests/🪜phases/🚪gate/🟦.ts")
PATCHES[PHG]=[
 (' expect(seen.rebuild).toMatchObject({reservedMegabytes:7000,phase:"rebuild",rampMilliseconds:90000});expect(seen.rebuild!.resumedAtMs).toBeGreaterThan(0);\n',
  ' expect(seen.rebuild).toMatchObject({reservedMegabytes:7000,phase:"rebuild",rampMilliseconds:policy.wait.staleMilliseconds});expect(seen.rebuild!.resumedAtMs).toBeGreaterThan(0);expect(seen.afterRebuild).toMatchObject({phase:"plan",rampMilliseconds:policy.memory.rampMilliseconds});\n'),
 ('return phase==="rebuild"?{reservedMegabytes:7000,rampMilliseconds:90000}:','return phase==="rebuild"?{reservedMegabytes:7000,rampMilliseconds:30000}:'),
 ('  expect(markHostCapacityPhaseV1(environment,"plan")).toBe(true);await sleep(60);level=800;await sleep(120);\n  expect(markHostCapacityPhaseV1(environment,"execution"))','  expect(markHostCapacityPhaseV1(environment,"plan")).toBe(true);await sleep(60);level=800;await sleep(120);seen.afterRebuild=grantRow(directory,handle.id);\n  expect(markHostCapacityPhaseV1(environment,"execution"))'),
]
MIRROR_ONLY[PHG]=[('new URL("../../../🟦️.ts",import.meta.url)','new URL("file://"+%r)'%os.path.join("/Users/Shared/semio",os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(PHG)))),"🟦️.ts")) ]
LENDT=resolve(HC,"🧪tests/🤲lend/🟦.ts")
LEND_TEST='''
test("Law: a graph rebuild keeps its reservation for the whole rebuild: forty minutes in, its farm workers are still lent from the unused part, and a second rebuild of the same size waits instead of being admitted beside it",()=>{
 const nowMs=1800000000000,host={version:1,atMs:nowMs,platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:27000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:2,openFiles:8000,openFilesMaximum:245760,attachedImages:2} as HostSampleV1;
 const holder=(rampMilliseconds:number)=>({id:"showholder1",operation:"show",jobs:1,cpu:2,reservedMegabytes:14889,admittedAtMs:nowMs-2_400_000,resumedAtMs:nowMs-2_400_000,observedMegabytes:4000,pid:4242,bootAtMs:1700000000000,label:"projects",rampMilliseconds}) as HostGrantV1;
 const farm={request:{id:"farmreq0001",operation:"exec",priority:"batch",minimumJobs:1,preferredJobs:6,fixedMegabytes:0,perJobMegabytes:1536,cpuPerJob:1,elastic:true,label:"graph-farm",holder:"showholder1"},arrivedAtMs:nowMs-1000,pid:4243} as HostWaiterV1;
 const second={request:{id:"showsecond1",operation:"show",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:14889,perJobMegabytes:0,cpuPerJob:2,label:"projects"},arrivedAtMs:nowMs-2000,pid:4244} as HostWaiterV1;
 const verdicts=(rampMilliseconds:number)=>Object.fromEntries(evaluateHostCapacityV1(policy,host,[holder(rampMilliseconds)],[second,farm],nowMs).decisions.map(row=>[row.id,`${row.verdict}:${row.reason}`]));
 expect(verdicts(21_600_000),"protected for the whole rebuild").toEqual({farmreq0001:"admit:lent",showsecond1:"wait:memory"});
 const lapsed=verdicts(1_800_000);
 expect(lapsed.showsecond1,"counted only by what it holds once its learned time to peak is over, a second rebuild of 14889 MB lands beside the first on a host with 28672 MB above the floor").toBe("admit:fits");
 expect(lapsed.farmreq0001,"and the workers of the first are not lent anything").not.toBe("admit:lent");
});
'''
PATCHES[LENDT]=[
 ('\ntest("The vocabulary agrees: the holder is part of the request and the grant, lent is a reason with an English and a German text",()=>{',LEND_TEST+'\ntest("The vocabulary agrees: the holder is part of the request and the grant, lent is a reason with an English and a German text",()=>{'),
]

WANTED=resolve(B,"🪪graph-state/🙋wanted")
SERVER=resolve(WANTED,"🛠server/🟦.ts")
SERVER_SCHEMA=resolve(WANTED,"🧬schema/🔣.json")
SERVER_TEST=resolve(WANTED,"🧪tests/🛠server/🟦.ts")
SIDECAR=resolve(B,"🌐remote-execution/🛰sidecar/🕸graph/🟦.ts")
SIDECAR_TEST=resolve(B,"🌐remote-execution/🛰sidecar/🧪tests/🕸graph/🟦.ts")

PATCHES[SERVER_SCHEMA]=[
 ('        "inferTimeoutMilliseconds": 2700000,\n','        "inferStallMilliseconds": 1500000,\n        "inferMaximumMilliseconds": 43200000,\n        "watchMilliseconds": 5000,\n        "yieldPollMilliseconds": 5000,\n        "yieldNoticeMilliseconds": 60000,\n        "yieldMaximumMilliseconds": 1200000,\n'),
 ('          "failed": "graph-server: the graph read ended with status {status} after {seconds} s: {detail}"\n',
  '          "failed": "graph-server: the graph read ended with status {status} after {seconds} s: {detail}",\n          "stalled": "no wave was published for {minutes} min; the read is stopped and the next one starts from the waves published so far",\n          "tooLong": "the read ran longer than the maximum of {hours} h and is stopped; the next one starts from the waves published so far",\n          "paused": "graph: paused after generation {generation}: {waiters} command(s) wait for memory on this machine ({seconds} s)"\n'),
 ('          "failed": "Graph-Server: das Lesen des Graphen endete nach {seconds} s mit Status {status}: {detail}"\n',
  '          "failed": "Graph-Server: das Lesen des Graphen endete nach {seconds} s mit Status {status}: {detail}",\n          "stalled": "seit {minutes} min wurde keine Welle veröffentlicht; das Lesen wird beendet und das nächste beginnt bei den bisher veröffentlichten Wellen",\n          "tooLong": "das Lesen dauerte länger als das Maximum von {hours} h und wird beendet; das nächste beginnt bei den bisher veröffentlichten Wellen",\n          "paused": "Graph: pausiert nach Generation {generation}: {waiters} Befehl(e) warten auf Speicher auf diesem Rechner ({seconds} s)"\n'),
]

SERVER_CODE='''

/** 🩺️ The verdict over one background graph read: it goes on while it makes progress and ends only when it does not. */
export type GraphReadWatchV1=Readonly<{feed(text:string):void;verdict():"go"|"stalled"|"too-long";generation():number|null;silentMilliseconds():number}>;

/** 🩺️ Watches the stderr of one graph read of the server: a wave is progress, told by a generation number higher than any seen (the line of a published generation, English or German, however the chunks are cut) or by a line that says the read pauses for others; the read is stopped when no progress was made for `inferStallMilliseconds` (published waves are durable, the next read starts from them, so a stopped read loses only the wave in flight) or when it has run `inferMaximumMilliseconds`. A cold refresh of an hour or more is never cut while its waves keep coming. */
export function createGraphReadWatchV1(policy:WantedServerPolicyV1=wantedServerPolicyV1,now:()=>number=Date.now):GraphReadWatchV1{
 const started=now();let progressAt=started,newest=-1,pending="";
 return Object.freeze({
  feed(text:string):void{
   pending+=text;const lines=pending.split("\\n");pending=lines.pop()!.slice(-4096);
   for(const line of lines){
    if(/(?:paused after|pausiert nach) /iu.test(line)){progressAt=now();continue;}
    const found=/generation (\\d+)/iu.exec(line);
    if(found&&Number(found[1])>newest){newest=Number(found[1]);progressAt=now();}
   }
  },
  verdict():"go"|"stalled"|"too-long"{const at=now();return at-started>=policy.inferMaximumMilliseconds?"too-long":at-progressAt>=policy.inferStallMilliseconds?"stalled":"go";},
  generation:()=>newest<0?null:newest,
  silentMilliseconds:()=>now()-progressAt,
 });
}

/** 🗨️ Words why the server stopped a graph read, in the language of the machine. */
export function graphReadEndTextV1(kind:"stalled"|"too-long",language:"en"|"de",policy:WantedServerPolicyV1=wantedServerPolicyV1):string{
 return wantedTextV1(kind==="stalled"?"stalled":"tooLong",language,{minutes:Math.round(policy.inferStallMilliseconds/60000),hours:Math.round(policy.inferMaximumMilliseconds/3600000)});
}

/** 🧘️ What the capacity gate shows a background refresh: the pressure level and the requests that wait. */
export type GraphYieldViewV1=Readonly<{pressure:string;waiters:readonly Readonly<{request:Readonly<{id:string;elastic?:boolean;quiet?:boolean}>}>[]}>;

const pause=(milliseconds:number,signal?:AbortSignal):Promise<void>=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal!.reason);},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

/** 🧘️ Lets a background refresh give way between two waves: while another command of this machine waits for memory (any request but small forecast grants, the workers of a build and its own) or the pressure is raised, the refresh pauses, tells so every `yieldNoticeMilliseconds` and looks again every `yieldPollMilliseconds`; after `yieldMaximumMilliseconds` it goes on with one wave, so a machine that is busy for good still advances. A gate that cannot be read never holds the refresh. Answers the milliseconds paused. */
export async function yieldToInteractiveV1(input:Readonly<{view():Promise<GraphYieldViewV1>;own:string|undefined;generation:number;language:"en"|"de";say(line:string):void;signal:AbortSignal|undefined;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;policy?:WantedServerPolicyV1}>):Promise<number>{
 const now=input.now??Date.now,policy=input.policy??wantedServerPolicyV1,sleep=input.sleep??pause,started=now();let noticed=-Infinity;
 for(;;){
  if(input.signal?.aborted)return now()-started;
  let waiting:number;
  try{
   const view=await input.view(),others=view.waiters.filter(row=>row.request.elastic!==true&&row.request.quiet!==true&&row.request.id!==input.own).length;
   waiting=others>0?others:view.pressure==="normal"?0:1;
  }catch{return now()-started;}
  if(waiting===0||now()-started>=policy.yieldMaximumMilliseconds)return now()-started;
  if(now()-noticed>=policy.yieldNoticeMilliseconds){noticed=now();input.say(wantedTextV1("paused",input.language,{generation:input.generation,waiters:waiting,seconds:Math.round((now()-started)/1000)}));}
  try{await sleep(policy.yieldPollMilliseconds,input.signal);}catch{return now()-started;}
 }
}

/** 🧘️ The yield of a background refresh bound to the capacity gate of this machine: called by the graph host after every published wave of a read the server started (`SEMIO_GRAPH_BACKGROUND=1`). */
export async function yieldBackgroundGraphV1(input:Readonly<{generation:number;signal:AbortSignal|undefined;language:"en"|"de";environment?:NodeJS.ProcessEnv}>):Promise<number>{
 const environment=input.environment??process.env,{readRemoteExecutionHostViewV1}=await import("../../../🌐️remote-execution/🌡️load/🟦️.ts");
 return yieldToInteractiveV1({view:async()=>{const view=await readRemoteExecutionHostViewV1(environment);return{pressure:view.host.pressure,waiters:view.waiters};},own:environment.SEMIO_HOST_CAPACITY_GRANT,generation:input.generation,language:input.language,say:line=>{process.stderr.write(line+"\\n");},signal:input.signal});
}
'''
PATCHES[SERVER]=[
 ('export type WantedInferV1=Readonly<{ok:boolean;status:number;seconds:number;generation:number|null;detail:string}>;\n','export type WantedInferV1=Readonly<{ok:boolean;status:number;seconds:number;generation:number|null;detail:string}>;'+SERVER_CODE),
 ('the bound of one graph read, the free memory','the stall limit and the maximum of one graph read, the pauses of a background refresh, the free memory'),
]

INFER_OLD='''  const started=Date.now(),child=Bun.spawn([graphServerCommandV1(environment),"show","projects"],{cwd:context.root,stdin:"ignore",stdout:"ignore",stderr:"pipe",env:{...environment,SEMIO_GRAPH_FLEET_READ:"off"}}),stop=()=>{try{child.kill("SIGINT");}catch{}setTimeout(()=>{try{child.kill("SIGKILL");}catch{}},10_000).unref();};
  const timer=setTimeout(stop,policy.inferTimeoutMilliseconds);signal?.addEventListener("abort",stop,{once:true});
  try{
   const [text,status]=await Promise.all([new Response(child.stderr).text(),child.exited]),generations=[...text.matchAll(/generation (\\d+)/gu)];
   return{ok:status===0,status,seconds:(Date.now()-started)/1000,generation:generations.length?Number(generations.at(-1)![1]):null,detail:status===0?reason:text.trim().split("\\n").at(-1)?.slice(0,200)??""};
  }finally{clearTimeout(timer);signal?.removeEventListener("abort",stop);}
 };
'''
INFER_NEW='''  return inferGraphReadV1({command:graphServerCommandV1(environment),cwd:context.root,environment,reason,signal,language});
 };
'''
INFER_FN='''/** 🔭️ One graph read of the server: `repo show projects` as a background refresh (it gives way between waves, `SEMIO_GRAPH_BACKGROUND=1`, and neither asks the fleet nor waits for it), watched by its progress and not by a fixed budget: it runs for as long as it publishes waves, and is stopped with SIGINT (SIGKILL ten seconds later) only after `inferStallMilliseconds` without one or after `inferMaximumMilliseconds`; what is published stays, so the next read starts from it. The stderr is read as it arrives, the last line of a failed read and the generation of the newest wave are the answer. */
export async function inferGraphReadV1(input:Readonly<{command:string;cwd:string;environment:Readonly<Record<string,string|undefined>>;reason:"request"|"refresh";signal:AbortSignal|undefined;language:"en"|"de";policy?:WantedServerPolicyV1;now?:()=>number}>):Promise<WantedInferV1>{
 const policy=input.policy??wantedServerPolicyV1,now=input.now??Date.now,started=now(),watch=createGraphReadWatchV1(policy,now);
 const child=Bun.spawn([input.command,"show","projects"],{cwd:input.cwd,stdin:"ignore",stdout:"ignore",stderr:"pipe",env:{...input.environment,SEMIO_GRAPH_FLEET_READ:"off",SEMIO_GRAPH_BACKGROUND:"1"} as Record<string,string>});
 const stop=()=>{try{child.kill("SIGINT");}catch{}setTimeout(()=>{try{child.kill("SIGKILL");}catch{}},10_000).unref();};
 let ended:"stalled"|"too-long"|undefined,tail="";
 const timer=setInterval(()=>{const verdict=watch.verdict();if(verdict!=="go"&&ended===undefined){ended=verdict;stop();}},policy.watchMilliseconds);
 input.signal?.addEventListener("abort",stop,{once:true});
 try{
  const decoder=new TextDecoder(),pump=(async()=>{for await(const chunk of child.stderr){const text=decoder.decode(chunk,{stream:true});watch.feed(text);tail=(tail+text).slice(-4096);}})();
  const[,status]=await Promise.all([pump,child.exited]);
  return{ok:status===0&&ended===undefined,status,seconds:(now()-started)/1000,generation:watch.generation(),detail:ended!==undefined?graphReadEndTextV1(ended,input.language,policy):status===0?input.reason:tail.trim().split("\\n").at(-1)?.slice(0,200)??""};
 }finally{clearInterval(timer);input.signal?.removeEventListener("abort",stop);}
}


/** 🕸️ The graph server of a machine:'''
PATCHES[SIDECAR]=[
 (INFER_OLD,INFER_NEW),
 ('/** 🕸️ The graph server of a machine:',INFER_FN),
 ('import {runWantedServerV1,wantedServerPolicyV1,type WantedInferV1} from','import {createGraphReadWatchV1,graphReadEndTextV1,runWantedServerV1,wantedServerPolicyV1,type WantedInferV1,type WantedServerPolicyV1} from'),
]

PATCHES[HOST].append(('    published:(generation:number)=>{seams.published(generation);},\n',
 '    published:async(generation:number)=>{seams.published(generation);if(process.env.SEMIO_GRAPH_BACKGROUND==="1")await(await import("../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts")).yieldBackgroundGraphV1({generation,signal:invocation.control.signal,language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language as "en"|"de"});},\n'))

SERVER_TESTS='''
test("Law: a read is watched by its progress, not by a budget: waves, in English or German and however the chunks are cut, keep it alive for hours; silence for the stall limit or the absolute maximum stops it",()=>{
 const policy={...wantedServerPolicyV1,inferStallMilliseconds:1000,inferMaximumMilliseconds:10000};let time=0;
 const watch=createGraphReadWatchV1(policy,()=>time);
 expect(watch.verdict()).toBe("go");expect(watch.generation()).toBeNull();
 watch.feed("graph: gener");time=500;watch.feed("ation 5 published, 48 projects refreshed in 80 s (0 kept their edges)\\n");expect(watch.generation()).toBe(5);
 time=1400;expect(watch.verdict(),"900 ms since the wave").toBe("go");
 watch.feed("graph: waiting for the graph build of a peer (3 s in the queue, generation 5 was the newest when this command queued)\\n");time=1600;expect(watch.verdict(),"the same generation again is no progress").toBe("stalled");
 watch.feed("Graph: Generation 6 veröffentlicht, 48 Projekte in 90 s erneuert (0 behielten ihre Kanten)\\n");expect(watch.verdict()).toBe("go");expect(watch.generation()).toBe(6);
 time=2500;watch.feed("graph: paused after generation 6: 1 command(s) wait for memory on this machine (60 s)\\n");time=3400;expect(watch.verdict(),"a pause for others is a sign of life").toBe("go");
 time=4500;expect(watch.verdict()).toBe("stalled");expect(watch.silentMilliseconds()).toBe(2000);
 for(let generation=7;time<9500;generation++){time+=900;watch.feed(`graph: generation ${generation} published\\n`);expect(watch.verdict()).toBe("go");}
 time=10000;watch.feed("graph: generation 99 published\\n");expect(watch.verdict(),"the maximum ends even a read that makes progress").toBe("too-long");
 expect(graphReadEndTextV1("stalled","en")).toContain("no wave was published for 25 min");expect(graphReadEndTextV1("stalled","de")).toContain("seit 25 min");
 expect(graphReadEndTextV1("too-long","en")).toContain("12 h");expect(graphReadEndTextV1("too-long","de")).toContain("12 h");
});

test("Law: the policy no longer holds a fixed budget for a read: 25 minutes without a wave, 12 hours at most",()=>{
 expect("inferTimeoutMilliseconds" in wantedServerPolicyV1).toBe(false);
 expect(wantedServerPolicyV1.inferStallMilliseconds).toBe(1_500_000);expect(wantedServerPolicyV1.inferMaximumMilliseconds).toBe(43_200_000);
 expect(wantedServerPolicyV1.inferStallMilliseconds,"a cold first wave (about 5 minutes here) fits several times").toBeGreaterThan(4*300_000);
});

test("A background refresh gives way between waves: it pauses while another command waits for memory or the pressure is raised, tells so in English and German, ignores small forecasts, workers and itself, advances one wave after the maximum and is never held by a gate it cannot read",async()=>{
 const policy={...wantedServerPolicyV1,yieldPollMilliseconds:10,yieldNoticeMilliseconds:25,yieldMaximumMilliseconds:200};let time=0;
 const row=(id:string,over:object={})=>({request:{id,...over}});
 const sleep=async(milliseconds:number)=>{time+=milliseconds;};
 const run=async(views:GraphYieldViewV1[],language:"en"|"de"="en",own:string|undefined="mine")=>{const lines:string[]=[];let at=0;const paused=await yieldToInteractiveV1({view:async()=>views[Math.min(at++,views.length-1)]!,own,generation:80,language,say:line=>lines.push(line),signal:undefined,sleep,now:()=>time,policy});return{paused,lines,looks:at};};
 time=0;expect((await run([{pressure:"normal",waiters:[]}])).paused,"nothing waits").toBe(0);
 time=0;expect((await run([{pressure:"normal",waiters:[row("mine",{elastic:false}),row("w1",{elastic:true}),row("q1",{quiet:true})]}])).paused,"itself, workers and small forecasts do not count").toBe(0);
 time=0;const busy=await run([{pressure:"normal",waiters:[row("other")]},{pressure:"normal",waiters:[row("other")]},{pressure:"normal",waiters:[row("other")]},{pressure:"normal",waiters:[]}]);
 expect(busy.paused).toBe(30);expect(busy.lines).toEqual([wantedTextV1("paused","en",{generation:80,waiters:1,seconds:0})]);expect(busy.lines[0]).toStartWith("graph: paused after generation 80: 1 command(s)");
 time=0;const german=await run([{pressure:"warn",waiters:[]},{pressure:"normal",waiters:[]}],"de");expect(german.paused).toBe(10);expect(german.lines[0]).toStartWith("Graph: pausiert nach Generation 80");
 time=0;expect((await run([{pressure:"normal",waiters:[row("other")]}])).paused,"after the maximum one wave goes on").toBeGreaterThanOrEqual(200);
 time=0;const unreadable=await yieldToInteractiveV1({view:async()=>{throw new Error("no gate");},own:undefined,generation:1,language:"en",say:()=>{throw new Error("silent");},signal:undefined,sleep,now:()=>time,policy});expect(unreadable).toBe(0);
 const controller=new AbortController();controller.abort();expect(await yieldToInteractiveV1({view:async()=>({pressure:"normal",waiters:[row("other")]}),own:undefined,generation:1,language:"en",say:()=>{},signal:controller.signal,sleep,now:()=>time,policy})).toBe(0);
});
'''
PATCHES[SERVER_TEST]=[
 ('import{runWantedServerV1,wantedServerPolicyV1,type WantedInferV1}from"../../🛠️server/🟦️.ts";','import{createGraphReadWatchV1,graphReadEndTextV1,runWantedServerV1,wantedServerPolicyV1,yieldToInteractiveV1,type GraphYieldViewV1,type WantedInferV1}from"../../🛠️server/🟦️.ts";'),
]
SIDECAR_TESTS='''
const fakeRepo=(script:string)=>{const dir=mkdtempSync(join(tmpdir(),"graph-infer-")),path=join(dir,"repo");writeFileSync(path,"#!/bin/sh\\n"+script+"\\n");chmodSync(path,0o755);return{dir,path};};
const readOf=(command:string,over:object={})=>inferGraphReadV1({command,cwd:tmpdir(),environment:{},reason:"refresh",signal:undefined,language:"en",policy:{...wantedServerPolicyV1,inferStallMilliseconds:700,watchMilliseconds:40},...over});

test("a graph read of the server is not cut while it publishes waves, although it runs three times as long as the stall limit",async()=>{
 const{dir,path}=fakeRepo('i=1\\nwhile [ $i -le 9 ]; do echo "graph: generation $i published, 48 projects refreshed in 80 s (0 kept their edges)" >&2; i=$((i+1)); sleep 0.25; done\\nexit 0');
 try{
  const started=Date.now(),result=await readOf(path);
  expect(Date.now()-started).toBeGreaterThan(2000);
  expect(result).toMatchObject({ok:true,status:0,generation:9,detail:"refresh"});
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);

test("a graph read that publishes nothing for the stall limit is stopped and says so; the generation of its last wave is kept in the answer",async()=>{
 const{dir,path}=fakeRepo('echo "graph: generation 4 published, 48 projects refreshed in 80 s (0 kept their edges)" >&2\\nexec sleep 60');
 try{
  const started=Date.now(),result=await readOf(path);
  expect(Date.now()-started).toBeLessThan(15000);
  expect(result.ok).toBe(false);expect(result.generation).toBe(4);expect(result.detail).toContain("no wave was published for");
  const german=await readOf(path,{language:"de"});expect(german.detail).toContain("seit ");
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("a graph read that fails reports the last line of its output, and one cancelled from outside is stopped at once; the read is started as a background refresh without a fleet request",async()=>{
 const{dir,path}=fakeRepo('echo "SEMIO_GRAPH_BACKGROUND=$SEMIO_GRAPH_BACKGROUND SEMIO_GRAPH_FLEET_READ=$SEMIO_GRAPH_FLEET_READ" >&2\\necho "boom: the store is full" >&2\\nexit 3');
 try{
  const failed=await readOf(path);expect(failed).toMatchObject({ok:false,status:3,generation:null,detail:"boom: the store is full"});
  const quiet=fakeRepo('echo started >&2\\nexec sleep 60'),controller=new AbortController();
  try{const reading=readOf(quiet.path,{signal:controller.signal,policy:{...wantedServerPolicyV1,inferStallMilliseconds:600000,watchMilliseconds:40}});await new Promise(resolve=>setTimeout(resolve,300));controller.abort();const result=await reading;expect(result.ok).toBe(false);}finally{rmSync(quiet.dir,{recursive:true,force:true});}
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);
'''
PATCHES[SIDECAR_TEST]=[
 ('import{expect,test}from"bun:test";\nimport{join}from"node:path";','import{expect,test}from"bun:test";\nimport{chmodSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";\nimport{tmpdir}from"node:os";\nimport{join}from"node:path";\nimport{wantedServerPolicyV1}from"../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";'),
 ('import{graphServerCommandV1,graphServerHeadroomV1,superviseGraphServerV1}from"../../🕸️graph/🟦️.ts";','import{graphServerCommandV1,graphServerHeadroomV1,inferGraphReadV1,superviseGraphServerV1}from"../../🕸️graph/🟦️.ts";'),
]

PATCHES[SERVER_TEST].append(('  expect(slots).toBe(1);expect(reads).toEqual([]);\n }finally{await server.stop(true);}\n},60000);\n','  expect(slots).toBe(1);expect(reads).toEqual([]);\n }finally{await server.stop(true);}\n},60000);\n'+SERVER_TESTS))
PATCHES[SIDECAR_TEST].append((' expect(graphServerHeadroomV1({totalMegabytes:32768,availableMegabytes:9992.4,pressure:"normal"},24000)).toEqual({freeMegabytes:9992,pressure:"normal"});\n});\n',' expect(graphServerHeadroomV1({totalMegabytes:32768,availableMegabytes:9992.4,pressure:"normal"},24000)).toEqual({freeMegabytes:9992,pressure:"normal"});\n});\n'+SIDECAR_TESTS))

FARM=resolve(B,"🕸graph/📥inference/📦project-nodes/🏊project-farm")
FARM_MJS=resolve(FARM,"🟨.mjs")
FARM_SCHEMA=resolve(FARM,"🏊pool/🧬schema/🔣.json")
FARM_DTS=resolve(FARM,"🏊pool/🟨.d.mts")
FARM_TEST=resolve(FARM,"🧪tests/🟦.ts")
PATCHES[FARM_SCHEMA]=[
 ('        "minimumProjects": 24,\n','        "minimumProjects": 24,\n        "parentWhileStarting": 2,\n        "startWaitMilliseconds": 20000,\n'),
 ('          "recycled": "graph: worker {worker} restarted after {megabytes} MB",\n','          "holding": "graph: the worker processes are starting; this process infers {parent} projects meanwhile and holds the others for the workers for at most {seconds} s",\n          "recycled": "graph: worker {worker} restarted after {megabytes} MB",\n'),
 ('          "recycled": "Graph: Worker-Prozess {worker} nach {megabytes} MB neu gestartet",\n','          "holding": "Graph: die Worker-Prozesse starten; dieser Prozess leitet unterdessen {parent} Projekte ab und hält die übrigen höchstens {seconds} s für die Worker zurück",\n          "recycled": "Graph: Worker-Prozess {worker} nach {megabytes} MB neu gestartet",\n'),
]
PATCHES[FARM_DTS]=[('readonly minimumProjects:number;','readonly minimumProjects:number;readonly parentWhileStarting:number;readonly startWaitMilliseconds:number;')]
PATCHES[FARM_MJS]=[
 ('  const listeners = [], pools = [], counts = { adopted: 0,','  const listeners = [], pools = [], readyWaiters = new Set(), counts = { parentStarts: 0, adopted: 0,'),
 ('  let state = "idle", started, closing = false, capacity, waiting, opening = Promise.resolve(), image, refusal, ownedRefresh;',
  '  let state = "idle", started, closing = false, capacity, waiting, opening = Promise.resolve(), image, refusal, ownedRefresh, gaveUp = false, holding = false;\n  const wake = () => { for (const resolve of [...readyWaiters]) resolve(); };\n  const readyWithin = milliseconds => new Promise(resolve => { const done = () => { clearTimeout(timer); readyWaiters.delete(done); resolve(); }, timer = setTimeout(done, milliseconds); timer.unref?.(); readyWaiters.add(done); });'),
 ('  const stop = (key, fields) => { state = "off"; say(key, fields); };','  const stop = (key, fields) => { state = "off"; say(key, fields); wake(); };'),
 ('pool.farm.ready.then(usable => { if (usable && state !== "off" && !closing) { pool.ready = true; for (const listener of listeners) listener(loops()); } });',
  'pool.farm.ready.then(usable => { if (usable && state !== "off" && !closing) { pool.ready = true; for (const listener of listeners) listener(loops()); } wake(); });'),
 ('      await refreshOwned();\n      const at = choosePoolV1(',
  '      await refreshOwned();\n      if (!gaveUp && !pools.some(pool => pool.ready) && (waiting !== undefined || pools.length > 0)) {\n        if (counts.parentStarts < policy.parentWhileStarting) { counts.parentStarts++; return false; }\n        if (!holding) { holding = true; say("holding", { parent: policy.parentWhileStarting, seconds: Math.round(policy.startWaitMilliseconds / 1000) }); }\n        await readyWithin(policy.startWaitMilliseconds);\n        if (!pools.some(pool => pool.ready)) gaveUp = true;\n        if (state === "off" || closing) return false;\n      }\n      const at = choosePoolV1('),
 ('    async close() {\n      closing = true;','    async close() {\n      closing = true; wake();'),
 (' * The farm then starts the worker the capacity grant of this process already pays for',' * While the workers start the parent does not infer beside them: the first `parentWhileStarting` projects are inferred in this process (the run goes on at once), the others wait for the pool at most `startWaitMilliseconds` once (a pool that is ready, the end of the farm or the wait ends it; after one wait that found no pool nothing waits again), so a wave of projects is adopted from the workers and the parent stays an adopter of finished leaves instead of holding the heap of eight inferences at once.\n * The farm then starts the worker the capacity grant of this process already pays for'),
]
FARM_TESTS='''
test("while the workers start, the parent infers only the first projects itself and holds the others for the pool; they are adopted from the workers once it is ready, and a pool that never comes releases them after one bounded wait",async()=>{
 const root=scratch(),children:ScriptedChild[]=[],log:string[]=[],lines:string[]=[];let open!:()=>void;const gate=new Promise<void>(done=>{open=done;});
 const id="farm_hold_00001",capacity=join(root,"capacity"),plan=JSON.stringify({version:1,id,jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536,floorMegabytes:1981});
 mkdirSync(join(capacity,"grants"),{recursive:true});
 writeFileSync(join(capacity,"grants",id+".json"),JSON.stringify({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:7168,admittedAtMs:Date.now(),observedMegabytes:null,pid:process.pid,bootAtMs:Math.round((Date.now()-uptime()*1000)/1000)*1000}));
 const farm=openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_DIRECTORY:capacity,SEMIO_HOST_CAPACITY_PLAN:plan},processed:()=>400,cores:10,write:line=>lines.push(line),engage:true,
  spawnProcess:(()=>{const child=new GatedChild("held",log,gate);children.push(child);return child;}) as never,request:(()=>{let cancel!:(value:undefined)=>void;return{granted:new Promise<undefined>(done=>{cancel=done;}),release:()=>cancel(undefined)};}) as never})!;
 const job=(name:string)=>({name,root:name,json:{},native:false,pipeline:false,nativeRoot:name,selectedInputs:[],head:[],cacheKey:"0".repeat(24)});
 expect(GRAPH_FARM_POLICY_V1.parentWhileStarting).toBe(2);
 expect(lines.filter(line=>line.includes("this process infers 2 projects meanwhile and holds the others for the workers for at most 20 s")).length,"told once, and only once something is held").toBe(0);
 expect(await farm.prepare(job("one"))).toBe(false);expect(await farm.prepare(job("two"))).toBe(false);
 const held=Array.from({length:6},(_,index)=>{let settled=false;const promise=farm.prepare(job("held"+index)).then(value=>{settled=true;return value;});return{promise,settled:()=>settled};});
 await Bun.sleep(300);
 expect(held.every(row=>!row.settled()),"six projects wait for the pool while the first two were inferred by the parent").toBe(true);
 expect(lines.filter(line=>line.includes("holds the others for the workers for at most 20 s")).length).toBe(1);
 expect(lines.find(line=>line.includes("holds the others"))).toContain("this process infers 2 projects meanwhile");
 const ready=new Promise<number>(done=>farm.onReady(done));open();await ready;
 const outcomes=await Promise.all(held.map(row=>row.promise));
 expect(outcomes.length).toBe(6);
 expect(log.filter(entry=>entry==="held:run").length,"the pool took the held projects").toBe(6);
 await farm.close();
},60000);

test("a pool that never comes releases the held projects after one wait of the declared length, and later projects do not wait again",async()=>{
 const root=scratch(),lines:string[]=[];
 const id="farm_hold_00002",capacity=join(root,"capacity"),plan=JSON.stringify({version:1,id,jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536,floorMegabytes:1981});
 mkdirSync(join(capacity,"grants"),{recursive:true});
 writeFileSync(join(capacity,"grants",id+".json"),JSON.stringify({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:7168,admittedAtMs:Date.now(),observedMegabytes:null,pid:process.pid,bootAtMs:Math.round((Date.now()-uptime()*1000)/1000)*1000}));
 let grant!:(value:undefined)=>void;
 const farm=openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_DIRECTORY:capacity,SEMIO_HOST_CAPACITY_PLAN:plan},processed:()=>400,cores:10,write:line=>lines.push(line),engage:true,
  spawnProcess:(()=>new ScriptedChild("never",[],{ready:false})) as never,request:(()=>({granted:new Promise<undefined>(done=>{grant=done;}),release:()=>undefined})) as never})!;
 const job=(name:string)=>({name,root:name,json:{},native:false,pipeline:false,nativeRoot:name,selectedInputs:[],head:[],cacheKey:"0".repeat(24)});
 await farm.prepare(job("a"));await farm.prepare(job("b"));
 const started=Date.now();
 expect(await farm.prepare(job("c"))).toBe(false);
 expect(Date.now()-started).toBeGreaterThanOrEqual(GRAPH_FARM_POLICY_V1.startWaitMilliseconds-500);
 const again=Date.now();expect(await farm.prepare(job("d"))).toBe(false);expect(Date.now()-again,"after one wait nothing waits again").toBeLessThan(500);
 grant(undefined);await farm.close();
},90000);
'''

FARM_TESTS=FARM_TESTS.replace('spawnProcess:(()=>new ScriptedChild("never",[],{ready:false})) as never','spawnProcess:(()=>new SilentChild("never",[])) as never')
PATCHES[FARM_TEST]=[
 ('test("a worker that boots above its budget but does not grow is kept','class SilentChild extends ScriptedChild{send(){return true;}}\nclass GatedChild extends ScriptedChild{constructor(tag:string,log:string[],readonly gate:Promise<void>){super(tag,log);}send(message:{type:string;id?:number}){if(message.type==="init"){void this.gate.then(()=>super.send(message));return true;}return super.send(message);}}\n'+FARM_TESTS+'\ntest("a worker that boots above its budget but does not grow is kept'),
]
MIRROR_ONLY[FARM_TEST]=pin_here(FARM_TEST)

CUTOFF_TEST='''
test("a refresh that is cut off keeps every published wave: the next read starts from the newest generation and infers only what no published wave holds",async()=>{
 const dir=scratch(),previous=process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;process.env.SEMIO_SLICE_WAVE_BASIS_POINTS="200";
 try{
  await seed(dir,ringFiles(6));
  (await loadGenerationGraphV1(createMockHost(dir).host)).graph;
  const files=ringFiles(6);
  await applyOps(dir,[1,2,3,4].map(index=>({op:"write" as const,path:`pkg/p${index}/src/index.ts`,content:files[`pkg/p${index}/src/index.ts`]!.replace(/=\\d+;\\n$/u,"=78;\\n")})));
  const controller=new AbortController(),cut=createMockHost(dir,{ownerDelayMs:120,signal:controller.signal}),pointer=()=>readGenerationPointerV1(readFileSync(join(dir,".gen","current.json"))).generation;
  const base=pointer(),running=loadGenerationGraphV1(cut.host).catch(error=>error);
  for(let waited=0;pointer()<base+2&&waited<600;waited++)await new Promise(resolve=>setTimeout(resolve,10));
  controller.abort(new Error("killed by the server"));
  expect(String(await running)).toContain("killed by the server");
  const kept=pointer();expect(kept,"two waves were published before the kill").toBeGreaterThanOrEqual(base+2);
  const again=createMockHost(dir),graph=(await loadGenerationGraphV1(again.host)).graph;
  const published=cut.log.flatMap(row=>row.full).slice(0,kept-base);
  expect(again.log.flatMap(row=>row.full).sort(),"only the owners no published wave holds are inferred again").toEqual(["p1","p2","p3","p4"].filter(name=>!published.includes(name)));
  expect(again.log.flatMap(row=>row.full).length).toBeLessThanOrEqual(4-(kept-base));
  expect(shape(graph)).toEqual(pythonGraph(dir));
 }finally{if(previous===undefined)delete process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;else process.env.SEMIO_SLICE_WAVE_BASIS_POINTS=previous;clean(dir);}
},120000);
'''
PATCHES[IDLE].append((' expect(shape(world.second)).toEqual(pythonGraph(dir));\n }finally{if(previous===undefined)delete process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;else process.env.SEMIO_SLICE_WAVE_BASIS_POINTS=previous;clean(dir);}\n},120000);\n'," expect(shape(world.second)).toEqual(pythonGraph(dir));\n }finally{if(previous===undefined)delete process.env.SEMIO_SLICE_WAVE_BASIS_POINTS;else process.env.SEMIO_SLICE_WAVE_BASIS_POINTS=previous;clean(dir);}\n},120000);\n"+CUTOFF_TEST))
