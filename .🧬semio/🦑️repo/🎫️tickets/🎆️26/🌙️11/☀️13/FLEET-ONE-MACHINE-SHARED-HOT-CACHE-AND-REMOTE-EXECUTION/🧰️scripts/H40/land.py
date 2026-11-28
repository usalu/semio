#!/usr/bin/env python3
"""H40 landing: anchored, atomic edits. `land.py check` verifies every anchor against the live tree and writes nothing;
`land.py apply` writes each file once (temp + rename) after re-reading it. Every anchor must match exactly the stated count."""
import os,re,sys,tempfile

L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
OFF=B+"🌐️remote-execution/🚀️offload/"
RC=B+"🌐️remote-cache/"
LAUNCH=L+"🏃️process/🦀️cargo/🚪️launcher/"
POOL=L+"⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/"

edits=[]
def edit(path,old,new,count=1,regex=False):edits.append((path,old,new,count,regex))

SCHEMA=OFF+"🧬️schema/🔣️.json"
edit(SCHEMA,'"floorMilliseconds": 60000,','"floorMilliseconds": 30000,')
edit(SCHEMA,'"unknownMilliseconds": 240000,','"unknownMilliseconds": 90000,')
edit(SCHEMA,'      "reattachMilliseconds": 10000\n','      "reattachMilliseconds": 10000,\n      "requeueMilliseconds": 20000\n')
edit(SCHEMA,r'("silenceMilliseconds",\s*"reattachMilliseconds")(\s*\])',r'\1,\n            "requeueMilliseconds"\2',1,True)
edit(SCHEMA,'            "reattachMilliseconds": {\n              "type": "integer",\n              "minimum": 1\n            }\n','            "reattachMilliseconds": {\n              "type": "integer",\n              "minimum": 1\n            },\n            "requeueMilliseconds": {\n              "type": "integer",\n              "minimum": 1\n            }\n')
edit(SCHEMA,'and how long the stream of an operation may stay silent before the client re-attaches to it.",','and how long the stream of an operation may stay silent before the client re-attaches to it, and how long an operation that went back to the queue after its worker dropped it (lost, left, refused) may wait for another machine before the unit compiles here (requeueMilliseconds, counted within the deadline). The floor is the shortest wait for a unit that was measured, the unknown value the wait for one that was never measured (a compile starved of memory or cores is not recorded), so a four-second unit never waits minutes for a machine that does not answer.",')

CLIENT=OFF+"🔌️client/🟦️.ts"
edit(CLIENT,'deadlineMilliseconds?:number;silenceMilliseconds?:number;','deadlineMilliseconds?:number;requeueMilliseconds?:number;silenceMilliseconds?:number;')
edit(CLIENT,'let phase="connect" as "connect"|"queue"|"execute",','let phase="connect" as "connect"|"queue"|"execute"|"requeue",')
edit(CLIENT,'execute:input.executeMilliseconds??offloadExecuteBudgetMillisecondsV1()},submitted','execute:input.executeMilliseconds??offloadExecuteBudgetMillisecondsV1(),requeue:input.requeueMilliseconds??deadlines.requeueMilliseconds},submitted')
edit(CLIENT,'if(next==="execute"){deadlineAt??=input.deadlineMilliseconds===undefined?undefined:Date.now()+input.deadlineMilliseconds;if(deadlineAt!==undefined)wait=Math.min(wait,deadlineAt-Date.now());}','if(next==="execute")deadlineAt??=input.deadlineMilliseconds===undefined?undefined:Date.now()+input.deadlineMilliseconds;\n  if((next==="execute"||next==="requeue")&&deadlineAt!==undefined)wait=Math.min(wait,deadlineAt-Date.now());')
edit(CLIENT,'if(stage==="CACHE_CHECK"||stage==="QUEUED"){if(phase==="connect")arm("queue");}','if(stage==="CACHE_CHECK"||stage==="QUEUED"){if(phase==="connect")arm("queue");else if(phase==="execute")arm("requeue");}')
edit(CLIENT,'if(expired)return declined(phase==="execute"?"infrastructure":"queue-timeout",phase==="execute"&&deadlineAt!==undefined&&Date.now()>=deadlineAt-50?`deadline ${input.deadlineMilliseconds} ms`:phase);','if(expired){const running=phase==="execute"||phase==="requeue";return declined(running?"infrastructure":"queue-timeout",running&&deadlineAt!==undefined&&Date.now()>=deadlineAt-50?`deadline ${input.deadlineMilliseconds} ms`:phase);}')
edit(CLIENT,'the silence of the server: the server repeats','the requeue of the operation (its worker dropped it and no other machine took it within `requeueMilliseconds`, see the policy: the unit compiles here at once) and the silence of the server: the server repeats')

OFFLOAD=OFF+"🟦️.ts"
edit(OFFLOAD,'/^silent\\b/u.test(outcome.detail)?"silent":undefined;','/^silent\\b/u.test(outcome.detail)?"silent":outcome.detail==="requeue"?"requeue":undefined;')

TIMEOUTS=OFF+"🧪️tests/⏱️timeouts/"
edit(TIMEOUTS+"🔣️.json",'"floorMilliseconds": 60000,','"floorMilliseconds": 30000,')
edit(TIMEOUTS+"🔣️.json",'"unknownMilliseconds": 240000,','"unknownMilliseconds": 90000,')
edit(TIMEOUTS+"🔣️.json",'  "reattachMilliseconds": 10000\n','  "reattachMilliseconds": 10000,\n  "requeueMilliseconds": 20000\n')
edit(TIMEOUTS+"🔣️.json",'"expected": 240000','"expected": 90000')
edit(TIMEOUTS+"🔣️.json",'"expected": 60000','"expected": 30000',6)
TIMEOUT_TEST='''
test("an operation that goes back to the queue after its worker dropped it and finds no other machine ends the wait within the requeue window, long before the deadline of the unit",async()=>{
 const {server,endpoint}=await served({[`${NS}.Execution/Execute`]:async function*(){yield operation("EXECUTING");await Bun.sleep(60);yield operation("QUEUED");for(;;){yield operation("QUEUED");await Bun.sleep(40);}}});
 try{
  const started=Date.now(),outcome=await executeOffloadV1(input(endpoint,{deadlineMilliseconds:600000,requeueMilliseconds:300,silenceMilliseconds:5000,executeMilliseconds:600000}));
  expect(outcome).toMatchObject({kind:"declined",reason:"infrastructure",detail:"requeue"});expect(Date.now()-started).toBeLessThan(4000);
 }finally{await server.stop();}
});

test("an operation that another machine takes after its worker dropped it keeps its deadline and finishes: the requeue window only bounds the wait for a machine",async()=>{
 const {server,endpoint}=await served({[`${NS}.Execution/Execute`]:async function*(){yield operation("EXECUTING");await Bun.sleep(50);yield operation("QUEUED");await Bun.sleep(100);yield operation("EXECUTING");await Bun.sleep(50);yield operation("COMPLETED",{status:{code:0,message:""},actionResult:encodeProtobufV1(REAPI_V1.actionResult,{exit_code:0}),cached:false,worker:"w2"});}});
 try{
  const outcome=await executeOffloadV1(input(endpoint,{deadlineMilliseconds:600000,requeueMilliseconds:2000,silenceMilliseconds:5000,executeMilliseconds:600000}));
  expect(outcome).toMatchObject({kind:"done",origin:"w2"});
 }finally{await server.stop();}
});

test("the deadline of a four-second unit is the floor of thirty seconds and the wait for a unit without a record is ninety seconds, never the old four minutes",()=>{
 expect(offloadDeadlineMillisecondsV1(4000,false)).toBe(30000);expect(offloadDeadlineMillisecondsV1(undefined,false)).toBe(90000);
 expect(OFFLOAD_POLICY_V1.deadline.requeueMilliseconds).toBeLessThan(OFFLOAD_POLICY_V1.deadline.unknownMilliseconds);
});
'''
edit(TIMEOUTS+"🟦️.ts",'\nconst removeBlobV1=(directory:string,hash:string):number=>','\n'+TIMEOUT_TEST.strip("\n")+'\n\nconst removeBlobV1=(directory:string,hash:string):number=>')

SERVER=RC+"🖥️server/🟦️.ts"
edit(SERVER,'''export async function remoteCacheBlobBoundV1(host:Pick<CasStoreV1,"root"|"policy">,sharePercent:number|undefined,measure:(root:string)=>ReturnType<typeof measureCasPartitionV1>=measureCasPartitionV1):Promise<number|undefined>{
 if(sharePercent===undefined)return undefined;
 const partition=measure(host.root),hint=await readCasUsageHintV1(host.root).catch(()=>undefined),cap=partition===undefined?host.policy.capBytes:casEffectiveCapBytesV1(host.policy,partition,hint?.bytes);
 return Math.floor(cap*sharePercent/100);
}''','''export async function remoteCacheBlobBoundV1(host:Pick<CasStoreV1,"root"|"policy">,sharePercent:number|undefined,measure:(root:string)=>ReturnType<typeof measureCasPartitionV1>=measureCasPartitionV1,room?:RemoteCacheBlobRoomV1):Promise<number|undefined>{
 if(sharePercent===undefined)return undefined;
 const partition=measure(host.root),hint=await readCasUsageHintV1(host.root).catch(()=>undefined),cap=partition===undefined?host.policy.capBytes:casEffectiveCapBytesV1(host.policy,partition,hint?.bytes),share=Math.floor(cap*sharePercent/100);
 if(room===undefined)return share;
 const census=await readCasCensusV1(host.root).catch(()=>undefined);
 if(census===undefined||Date.now()-census.atMs>room.censusMaximumSeconds*1000)return share;
 const fixed=census.classes.pinned.bytes+census.classes.held.bytes+census.classes.leased.bytes+census.classes.linked.bytes;
 return Math.min(share,Math.max(Math.floor(cap*room.floorPercent/100),cap-fixed-Math.floor(cap*room.headroomPercent/100)));
}''')
edit(SERVER,'export async function remoteCacheBlobBoundV1(','''/** 🪟️ What the blob bound leaves to the rest of the host store (`blobRoom` of the server policy): `headroomPercent` of the cap stays free for uploads in flight and the objects the sweep has not taken yet, `floorPercent` of the cap is the least the bound ever allows (the cache keeps working when the unit caches fill the cap), and a census of the store older than `censusMaximumSeconds` is not believed. */
export type RemoteCacheBlobRoomV1=Readonly<{headroomPercent:number;floorPercent:number;censusMaximumSeconds:number}>;

/** 📐️ The bytes the blobs of the entries may use (each once): `sharePercent` of the cap in force of the host store, and never more than the cap minus what the store holds for others (the objects the unit caches hold, pinned and leased ones: nothing the remote cache evicts frees them) minus the headroom, never less than the floor; the cap in force already follows the free space of the partition, so near the hard reserve the bound shrinks with it and the store keeps room for the next unit of the fleet instead of refusing it with a 507 (the share alone left 56 GiB of held objects and a 34 GiB bound in a 84 GiB cap, so every upload was refused until a collection by chance).
 * @see ../📐️blob-bound/🧪️tests/🟦️.ts */
export async function remoteCacheBlobBoundV1(''')
edit(SERVER,'blobSharePercent?:number;graph?:','blobSharePercent?:number;blobRoom?:RemoteCacheBlobRoomV1;graph?:')
edit(SERVER,'blobBoundBytes:()=>remoteCacheBlobBoundV1(options.hostStore!,options.blobSharePercent)','blobBoundBytes:()=>remoteCacheBlobBoundV1(options.hostStore!,options.blobSharePercent,undefined,options.blobRoom)')

EXEC=B+"🏃️execution/🧬️schema/🌐️remote-cache.json"
edit(EXEC,'"blobSharePercent", "graph",','"blobSharePercent", "blobRoom", "graph",')
edit(EXEC,'        "graph": {\n          "description": "The protected class of the graph exchange','''        "blobRoom": { "description": "What the blob bound leaves to the rest of the host store, so the store keeps accepting the next unit of the fleet instead of refusing it with a 507 (the share alone ignored the objects the unit caches hold, which nothing the cache evicts frees). headroomPercent: the share of the cap in force kept free for uploads in flight and for objects the sweep has not taken yet. floorPercent: the least the bound ever is, in percent of the cap (the cache keeps working when the unit caches fill the cap). censusMaximumSeconds: a census of the store older than this is not believed (the share alone applies). The bound is min(share, max(floor, cap - held - headroom)). / Was die Blob-Grenze dem Rest des Host-Speichers lässt, damit der Speicher die nächste Einheit der Flotte annimmt, statt sie mit 507 abzuweisen (der Anteil allein ignorierte die Objekte, die die Einheiten-Caches halten und die der Cache nicht freigeben kann). headroomPercent: Anteil der wirksamen Grenze, der für laufende Uploads und noch nicht gesammelte Objekte frei bleibt. floorPercent: das Mindeste der Grenze in Prozent (der Cache arbeitet weiter, wenn die Einheiten-Caches die Grenze füllen). censusMaximumSeconds: eine ältere Zählung des Speichers gilt nicht (es gilt der Anteil allein).", "type": "object", "additionalProperties": false, "required": ["headroomPercent", "floorPercent", "censusMaximumSeconds"], "properties": { "headroomPercent": { "type": "integer", "minimum": 0, "maximum": 50 }, "floorPercent": { "type": "integer", "minimum": 1, "maximum": 50 }, "censusMaximumSeconds": { "type": "integer", "minimum": 1 } } },
        "graph": {
          "description": "The protected class of the graph exchange''')
edit(B+"🔣️policy.json",'      "blobSharePercent": 50,\n','      "blobSharePercent": 50,\n      "blobRoom": { "headroomPercent": 8, "floorPercent": 5, "censusMaximumSeconds": 21600 },\n')
for path in [B+"🌐️remote-execution/🛰️sidecar/🗄️cache/🟦️.ts",RC+"🏠️hosted/🟦️.ts",RC+"⌨️command/🟦️.ts"]:
    edit(path,'blobSharePercent:policy.blobSharePercent,graph:policy.graph,','blobSharePercent:policy.blobSharePercent,blobRoom:policy.blobRoom,graph:policy.graph,')

STORE=RC+"💾️store/🟦️.ts"
edit(STORE,'"unavailable";};\n','"unavailable";};\n\n/** 🚨️ The shortest time between two collections that a capacity refusal of the host store asks for: the refusal says the store is at its cap in force, the next upload of the same unit comes seconds later (`retry-after`), so the room is made now and not when the admitted bytes next reach their trigger (five percent of the bound, at most every thirty seconds). */\nconst CAPACITY_RELIEF_MILLISECONDS_V1=5000;\n')
edit(STORE,'catch(error){if(this.options.signal?.aborted)throw error;return hostPutFailureV1(error);}','catch(error){if(this.options.signal?.aborted)throw error;if(error instanceof CasCapRefusalV1)this.relieve();return hostPutFailureV1(error);}',2)
edit(STORE,' private evictedBlobBytes=0;\n',' private evictedBlobBytes=0;\n private relievedAt=0;\n')
edit(STORE,' private async blobSize(hash:string):Promise<number>{','''
 /** 🚨️ Makes room after the host store refused a write at its cap in force: one collection (the blob bound included) at most once per `CAPACITY_RELIEF_MILLISECONDS_V1`, in the background; the refused write is answered 507 as before and its retry meets the room. */
 private relieve():void{
  const now=Date.now();if(now-this.relievedAt<CAPACITY_RELIEF_MILLISECONDS_V1)return;
  this.relievedAt=now;void this.collect().catch(()=>undefined);
 }
 private async blobSize(hash:string):Promise<number>{''')

BT=RC+"📐️blob-bound/🧪️tests/🟦️.ts"
edit(BT,'import {casUsageHintPathV1,recordCasMeasurementV1} from "../../../🗄️cas/🧹️collector/🟦️.ts";','import {casUsageHintPathV1,recordCasMeasurementV1,writeCasCensusV1,type CasCensusV1} from "../../../🗄️cas/🧹️collector/🟦️.ts";\nimport {CasCapRefusalV1} from "../../../🗄️cas/🚦️guard/🟦️.ts";')
BLOB_TESTS='''
const censusOf=(held:number,atMs:number):CasCensusV1=>({version:1,atMs,objects:1,bytes:held,classes:{pinned:{objects:0,bytes:0},held:{objects:1,bytes:held},leased:{objects:0,bytes:0},linked:{objects:0,bytes:0},expired:{objects:0,bytes:0},young:{objects:0,bytes:0},unreferenced:{objects:0,bytes:0}},holders:[]});

test("the server bound leaves room for what the unit caches hold: the share while they hold little, less as they hold more, never below the floor, and the share alone for a census that is missing or too old",async()=>{
 const root=directory("room"),host={root,policy:{...CAS_POLICY_V1,capBytes:100*2**30}},room={headroomPercent:10,floorPercent:5,censusMaximumSeconds:3600},none=()=>undefined,GiB=2**30;
 mkdirSync(join(root,"v1"),{recursive:true});
 expect(await remoteCacheBlobBoundV1(host,50,none,room),"no census: the share alone").toBe(50*GiB);
 await writeCasCensusV1(root,censusOf(20*GiB,Date.now()));
 expect(await remoteCacheBlobBoundV1(host,50,none,room),"little held: the share").toBe(50*GiB);
 await writeCasCensusV1(root,censusOf(60*GiB,Date.now()));
 expect(await remoteCacheBlobBoundV1(host,50,none,room),"cap - held - headroom").toBe(30*GiB);
 await writeCasCensusV1(root,censusOf(99*GiB,Date.now()));
 expect(await remoteCacheBlobBoundV1(host,50,none,room),"the floor").toBe(5*GiB);
 await writeCasCensusV1(root,censusOf(60*GiB,Date.now()-7200000));
 expect(await remoteCacheBlobBoundV1(host,50,none,room),"a census older than the maximum is not believed").toBe(50*GiB);
 await writeCasCensusV1(root,censusOf(60*GiB,Date.now()));
 expect(await remoteCacheBlobBoundV1(host,50,none),"without room the share alone").toBe(50*GiB);
 const python3=python;
 if(python3){
  const model=Bun.spawnSync([python3,"-I","-c","import json,sys\\nr=json.load(sys.stdin)\\nprint(min(r['share'],max(r['cap']*r['floor']//100,r['cap']-r['held']-r['cap']*r['headroom']//100)))"],{stdin:new TextEncoder().encode(JSON.stringify({share:50*GiB,cap:100*GiB,held:60*GiB,floor:5,headroom:10})),stdout:"pipe"});
  expect(Number(model.stdout.toString().trim())).toBe(30*GiB);
 }
});

test("a write the host store refuses at its cap makes room at once: the collection that the admitted-bytes trigger would only start later runs now, so the retry of the upload meets the room",async()=>{
 const tree=directory("relief-tree"),folder=directory("relief-store"),cas=directory("relief-cas"),inner=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0}}),refuse={value:false},bound={value:undefined as number|undefined};
 const host=new Proxy(inner,{get:(target,name)=>{const value=Reflect.get(target,name);if(name==="putChecked"&&refuse.value)return async()=>{throw new CasCapRefusalV1(10,5);};return typeof value==="function"?value.bind(target):value;}});
 writeFileSync(join(tree,"a.rs"),"a v1");
 const store=await RemoteCacheStoreV1.open({directory:folder,host,maximumBytes:2**30,maximumEntryBytes:2**22,maximumAgeMilliseconds:0,graceMilliseconds:60,blobBoundBytes:()=>bound.value,staleness:{mode:"enforce",tree:await openTreeDigestsV1(tree),examples:5,recheckMilliseconds:0,manifestNames:["unit.json"]}});
 const put=async(key:string,size:number)=>{const bytes=bytesOf(key,size);return store.put("main","nx",key,body(bytes),{mode:"keep",declaredLength:bytes.length});};
 expect(await put("old1",1000)).toBe("created");expect(await put("old2",1000)).toBe("created");
 await sleep(150);bound.value=1200;refuse.value=true;
 expect(await put("new",1000),"the refused write is answered as before").toBe("capacity");
 let live=true;for(let waited=0;live&&waited<3000;waited+=50){await sleep(50);live=(await store.read("main","nx","old1"))!==undefined;}
 expect(live,"the collection that follows the refusal took the oldest entry over the bound").toBe(false);
 await store.close();
});
'''
edit(BT,'\nconst bytesOf=(name:string,size:number)=>','\n'+BLOB_TESTS.strip("\n")+'\n\nconst bytesOf=(name:string,size:number)=>')

LS=LAUNCH+"🧬️schema/🔣️.json"
edit(LS,r'("required": \[\s*"store",\s*"configuration",\s*"parallelism")(\s*\],\s*"properties": \{\s*"store": \{\s*"const": "--store")',r'\1,\n            "freshLane"\2',1,True)
edit(LS,'            "parallelism": {\n              "const": "--parallelism"\n            }\n','            "parallelism": {\n              "const": "--parallelism"\n            },\n            "freshLane": {\n              "const": "--fresh-lane"\n            }\n')
edit(LS,'          "parallelism": "--parallelism"\n','          "parallelism": "--parallelism",\n          "freshLane": "--fresh-lane"\n')
edit(LS,r'(\n        "parallelism": \{\s*"anyOf": \[\s*\{\s*"type": "integer",\s*"minimum": 1,\s*"maximum": 1024\s*\},\s*\{\s*"const": "planned"\s*\}\s*\]\s*\},)(\s*"kind": \{)',r'\1\n        "freshLane": {\n          "const": true\n        },\2',None,True)

LT=LAUNCH+"🟦️.ts"
edit(LT,'import{withLaneRetentionV1}from"../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🟦️.ts";','import{retainRemovedLaneV1,withLaneRetentionV1}from"../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/📤️retention/🟦️.ts";')
edit(LT,'import {crateKeyV1,laneFlavourV1,measureLaneBytesV1} from','import {crateKeyV1,laneFlavourV1,measureLaneBytesV1,removeLaneV1} from')
edit(LT,'kind:"compile"|"resolve"|"lock";cargo:readonly string[]}>;','kind:"compile"|"resolve"|"lock";freshLane?:true;cargo:readonly string[]}>;')
edit(LT,'/** 🧭️ Parses `--store=shared|isolated --configuration=prototype|release [--parallelism=<cores>] -- <subcommand> <args…>` without consuming any Cargo argument.','/** 🧭️ Parses `--store=shared|isolated --configuration=prototype|release [--parallelism=<cores>] [--fresh-lane] -- <subcommand> <args…>` without consuming any Cargo argument. `--fresh-lane` (shared store only) runs the call in a lane that is born for it and removed after it: a cold Cargo target with nothing carried over, so a probe measures cold, local-hit and remote-hit honestly; an isolated store is fresh by itself.')
edit(LT,'const head=argv.slice(0,delimiter),cargo=argv.slice(delimiter+1),selected=new Map<string,string>(),names=Object.values(policy.options);','const all=argv.slice(0,delimiter),flagged=all.filter(argument=>argument===policy.options.freshLane).length,head=all.filter(argument=>argument!==policy.options.freshLane),cargo=argv.slice(delimiter+1),selected=new Map<string,string>(),names=Object.values(policy.options).filter(name=>name!==policy.options.freshLane);\n if(flagged>1)throw Error(`Repeated repository Cargo launch option: ${policy.options.freshLane} / Wiederholte Startoption des Repository-Cargo: ${policy.options.freshLane}`);')
edit(LT,' if(parallelism!==undefined&&!/^[1-9][0-9]{0,3}$/u.test(parallelism))',' if(flagged===1&&store!=="shared")throw Error(`${policy.options.freshLane} needs ${policy.options.store}=shared (an isolated store is fresh by itself) / ${policy.options.freshLane} braucht ${policy.options.store}=shared (ein isolierter Speicher ist von sich aus frisch)`);\n if(parallelism!==undefined&&!/^[1-9][0-9]{0,3}$/u.test(parallelism))')
edit(LT,'kind,cargo:Object.freeze([...cargo])}))','kind,...flagged===1?{freshLane:true}:{},cargo:Object.freeze([...cargo])}))')
edit(LT,'classes?:readonly string[];environment:Readonly<Record<string,string|undefined>>;control:ResourceLeaseControl}>;\n','classes?:readonly string[];fresh?:boolean;environment:Readonly<Record<string,string|undefined>>;control:ResourceLeaseControl}>;\n')
edit(LT,'const warm=planPoolAcquireV1(readPoolV1(options.cacheRoot,Date.now()),{crateKey:crateKeyV1(options.crates)},poolBoundsV1(quotaBytes)).kind==="warm";','const warm=options.fresh!==true&&planPoolAcquireV1(readPoolV1(options.cacheRoot,Date.now()),{crateKey:crateKeyV1(options.crates)},poolBoundsV1(quotaBytes)).kind==="warm";')
edit(LT,' try{\n  const lane=await acquirePooledLaneV1(options.cacheRoot,',' let freshId:string|undefined;\n try{\n  const lane=await acquirePooledLaneV1(options.cacheRoot,')
edit(LT,'owner:{session:agentSessionAddressV1(session),agent:session.agent.id},...flavour===undefined?{}:{flavour}}','owner:{session:agentSessionAddressV1(session),agent:session.agent.id},...flavour===undefined?{}:{flavour},...options.fresh===true?{fresh:true}:{}}')
edit(LT,'report:(kind,parameters)=>{process.stderr.write(laneMessageV1(kind,parameters,language)+"\\n");}});\n  return await withLaneRetentionV1(','report:(kind,parameters)=>{process.stderr.write(laneMessageV1(kind,parameters,language)+"\\n");}});\n  if(options.fresh===true)freshId=lane.paths.id;\n  return await withLaneRetentionV1(')
edit(LT,' }finally{await headroom.release();}\n}\n',' }finally{await headroom.release();if(freshId!==undefined)removeLaneV1(options.cacheRoot,freshId,"lane",Date.now(),paths=>{retainRemovedLaneV1(paths);});}\n}\n')
edit(LT,'classes?:readonly string[];capacityBytes:number;emergencyBytes:number;environment:Readonly<Record<string,string|undefined>>;temporary:string;control:ResourceLeaseControl}>,body:','classes?:readonly string[];fresh?:boolean;capacityBytes:number;emergencyBytes:number;environment:Readonly<Record<string,string|undefined>>;temporary:string;control:ResourceLeaseControl}>,body:')
edit(LT,'emergencyBytes:space.maximumWorkspaceBytes,environment,temporary:store.environment.TMPDIR!,control},run)','emergencyBytes:space.maximumWorkspaceBytes,...request.freshLane===true?{fresh:true}:{},environment,temporary:store.environment.TMPDIR!,control},run)')

PT=POOL+"🟦️.ts"
edit(PT,'owner?:LaneOwnerV1;flavour?:string}>,options:','owner?:LaneOwnerV1;flavour?:string;fresh?:boolean}>,options:')
edit(PT,'choice=planPoolAcquireV1(lanes,{crateKey:key},bounds);','choice:PoolChoiceV1=request.fresh===true?{kind:"new"}:planPoolAcquireV1(lanes,{crateKey:key},bounds);')
PTEST=POOL+"🧪️tests/🔐️pool-claims/🟦️.ts"
POOL_TEST='''
test("a fresh acquisition is a lane of its own even while a free lane of the same crate set is warm: the cold Cargo target a probe asks for",async()=>{
 const root=scratch();
 try{
  const first=await acquirePooledLaneV1(root,{crates:["a"],quotaBytes:big});const id=first.paths.id;first.release();
  const fresh=await acquirePooledLaneV1(root,{crates:["a"],quotaBytes:big,fresh:true});
  expect(fresh.choice).toBe("new");expect(fresh.paths.id).not.toBe(id);fresh.release();
  const warm=await acquirePooledLaneV1(root,{crates:["a"],quotaBytes:big});
  expect(warm.choice,"an ordinary acquisition still takes a warm lane").toBe("warm");warm.release();
 }finally{rmSync(root,{recursive:true,force:true});}
});
'''
edit(PTEST,'\ntest("the reserved default lane is no part of the pool','\n'+POOL_TEST.strip("\n")+'\n\ntest("the reserved default lane is no part of the pool')

LTEST=LAUNCH+"🧪️tests/"
edit(LTEST+"🟦️.ts",'.option("--parallelism <cores>").argument("[cargo...]")','.option("--parallelism <cores>").option("--fresh-lane").argument("[cargo...]")')
edit(LTEST+"🟦️.ts",'expect(selected.parallelism===undefined?"planned":Number(selected.parallelism),row.name).toBe(request.parallelism);','expect(selected.parallelism===undefined?"planned":Number(selected.parallelism),row.name).toBe(request.parallelism);expect(Boolean((independent.opts() as {freshLane?:boolean}).freshLane),row.name).toBe(request.freshLane===true);')
edit(LTEST+"🔣️.json",'{\n  "version": 1,\n  "arguments": [\n','''{
  "version": 1,
  "arguments": [
    {
      "name": "fresh-lane-shared",
      "argv": ["--fresh-lane", "--store=shared", "--configuration=prototype", "--", "check", "-p", "semio-framework-diagnostic"],
      "environment": {},
      "request": { "version": 1, "store": "shared", "configuration": "prototype", "parallelism": "planned", "kind": "compile", "freshLane": true, "cargo": ["check", "-p", "semio-framework-diagnostic"] }
    },
    {
      "name": "fresh-lane-isolated-refused",
      "argv": ["--store=isolated", "--configuration=prototype", "--fresh-lane", "--", "check"],
      "environment": {},
      "request": null
    },
    {
      "name": "fresh-lane-repeated-refused",
      "argv": ["--fresh-lane", "--store=shared", "--configuration=prototype", "--fresh-lane", "--", "check"],
      "environment": {},
      "request": null
    },
''')

CAT=B+"⚙️operations/🔣️catalog.json"
edit(CAT,'''      "description": { "en": "Positive number of cores for Cargo.", "de": "Positive Anzahl von Kernen für Cargo." }
    },
''','''      "description": { "en": "Positive number of cores for Cargo.", "de": "Positive Anzahl von Kernen für Cargo." }
    },
    {
      "name": "--fresh-lane",
      "value": "none",
      "operations": ["cargo"],
      "description": { "en": "cargo: run in a fresh, empty lane that exists for this call only (a cold Cargo target, nothing carried over from earlier calls) and is removed afterwards; units still come from the local unit cache and the fleet cache, so a probe tells cold, local hit and remote hit apart. Only with --store=shared (an isolated store is fresh by itself).", "de": "cargo: läuft in einer frischen, leeren Lane, die nur für diesen Aufruf besteht (kaltes Cargo-Target, nichts aus früheren Aufrufen) und danach entfernt wird; Einheiten kommen weiterhin aus dem lokalen Einheiten-Cache und dem Flotten-Cache, so dass eine Messung kalt, lokalen und entfernten Treffer trennt. Nur mit --store=shared (ein isolierter Speicher ist von sich aus frisch)." }
    },
''')
edit(CAT,'"flags": ["--store", "--configuration", "--parallelism"],','"flags": ["--store", "--configuration", "--parallelism", "--fresh-lane"],')
edit(CAT,'[--parallelism=<cores>] -- <cargo subcommand> <args>"','[--parallelism=<cores>] [--fresh-lane] -- <cargo subcommand> <args>"')
edit(CAT,'''          "description": { "en": "Type-check one crate in the shared store.", "de": "Prüft eine Crate im gemeinsamen Speicher." }
        }
''','''          "description": { "en": "Type-check one crate in the shared store.", "de": "Prüft eine Crate im gemeinsamen Speicher." }
        },
        {
          "command": "repo cargo --store=shared --configuration=prototype --fresh-lane -- check -p <crate> --lib",
          "description": { "en": "Type-check one crate in a cold lane that exists for this call only (probes: cold, local hit, remote hit).", "de": "Prüft eine Crate in einer kalten Lane, die nur für diesen Aufruf besteht (Messungen: kalt, lokaler Treffer, entfernter Treffer)." }
        }
''')

def apply(write):
    byfile={}
    for e in edits:byfile.setdefault(e[0],[]).append(e)
    bad=0
    for path,rows in byfile.items():
        text=open(path,encoding="utf8").read()
        for _,old,new,count,regex in rows:
            if regex:
                found=len(re.findall(old,text,re.S))
                want=count if count is not None else found
                ok=found==want and found>=1
                if ok:text=re.sub(old,new,text,flags=re.S)
            else:
                found=text.count(old);ok=found==count
                if ok:text=text.replace(old,new)
            if not ok:bad+=1;print("ANCHOR",found,"!=",count,path.split("/")[-2]+"/"+path.split("/")[-1],repr(old[:70]))
        if write and bad==0:
            fd,temporary=tempfile.mkstemp(dir=os.path.dirname(path),prefix=".h40-");os.close(fd)
            open(temporary,"w",encoding="utf8").write(text);os.replace(temporary,path)
    return bad

if __name__=="__main__":
    mode=sys.argv[1] if len(sys.argv)>1 else "check"
    bad=apply(False)
    if bad:sys.exit(1)
    if mode=="apply":
        if apply(True):sys.exit(1)
        print("applied",len(edits),"edits in",len({e[0] for e in edits}),"files")
    else:print("anchors ok:",len(edits),"edits in",len({e[0] for e in edits}),"files")
