/** 🎛️ Contract: the pool and the machine's own read are one budget. Memory the own work really holds is already out of the available memory; its untouched reservations are held back only up to a share (all of them once it waits); cores it books but does not use are measured idle and given to the pool; one child always runs above the floor; and a pool that waits says the number that blocks it. Oracle: the host of g1-server at 09:41 UTC 2026-11-17 (32 GB, 10 cores, a read reserving 14336 MB of which 7826 MB touched, nine farm workers booking 9 cores, load 43) and the independent Python implementation of the fixtures. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{startRemoteCacheServerV1}from"../../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{graphFleetClientV1}from"../../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../../../../🪪️graph-state/🧶️nx/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolIdV1,ownerPoolSizingV1,ownerPoolTargetV1,publishOwnerPoolV1,type OwnerPoolHostV1,type OwnerPoolManifestV1,type OwnerPoolUnitV1}from"../../🟦️.ts";
import{runOwnerPoolV1}from"../../🏃️runner/🟨️.mjs";
import{ownerPoolHostOfV1}from"../../../../../../🌐️remote-execution/🛰️sidecar/🏊️pool/🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`owner-pool-budget-${label}-`));scratch.push(path);return path;};
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const driverPath=join(import.meta.dir,"..","🧫️fixtures","🧮️driver.mjs");
const host=(over:Partial<OwnerPoolHostV1>&{foreground?:Partial<OwnerPoolHostV1["foreground"]>}={}):OwnerPoolHostV1=>({cores:8,totalMegabytes:16384,availableMegabytes:9000,pressure:"normal",load:null,swapOutMegabytesPerSecond:null,idleCores:null,...over,foreground:{cpu:0,unrealizedMegabytes:0,waiters:0,...over.foreground}});
const serverAt0941:OwnerPoolHostV1={cores:10,totalMegabytes:32768,availableMegabytes:15851,pressure:"normal",load:43.45,swapOutMegabytesPerSecond:0,idleCores:0,foreground:{cpu:11,unrealizedMegabytes:9966,waiters:0,oldestWaitMilliseconds:0}};

test("the server of 09:41: the read's untouched reservation no longer leaves the pool one child, and the share held back is stated",()=>{
 const sizing=ownerPoolSizingV1({host:serverAt0941,children:1,childMegabytes:1064});
 expect(sizing.children).toBe(3);
 expect(sizing.limit).toMatchObject({limit:"load",needMegabytes:1064,heldMegabytes:Math.floor(32768*P.ownHoldShareBasisPoints/10000),floorMegabytes:4096});
 const before=Math.floor((15851-4096-9966)/1064)+1;
 expect(before,"the old arithmetic: spare 1789 MB, one more child").toBe(2);
 expect(sizing.children).toBeGreaterThan(before);
});

test("a 16 GB worker whose read books every core: idle cores are the pool's, a waiting read keeps its reservation (all of it when its ask is unknown) and a share of the cores free",()=>{
 const reading=host({idleCores:3,load:6,foreground:{cpu:8,unrealizedMegabytes:5200}});
 const target=ownerPoolTargetV1({host:reading,children:2,childMegabytes:1100});
 expect(target.children).toBe(5);
 const waiting=ownerPoolSizingV1({host:host({idleCores:3,load:6,foreground:{cpu:6,unrealizedMegabytes:5200,waiters:2,oldestWaitMilliseconds:1000}}),children:2,childMegabytes:1100});
 expect(waiting.children).toBe(3);
 expect(waiting.limit?.heldMegabytes).toBe(5200);
 const measured=ownerPoolTargetV1({host:host({idleCores:0,load:6,foreground:{cpu:8,unrealizedMegabytes:5200}}),children:2,childMegabytes:1100});
 expect(measured.children,"no idle core: the pool keeps what it has, never fewer than the minimum share").toBe(Math.max(2,Math.ceil(8*P.minimumShareBasisPoints/10000)));
});

test("never both idle: with the read present the pool never gets fewer children than the read's own bookings leave idle, and without a read it gets the whole budget",()=>{
 for(const available of[3000,6000,9000,12000])for(const idle of[0,1,2,4,8])for(const children of[0,1,3])for(const unrealized of[0,2000,6000,12000]){
  const row=host({availableMegabytes:available,idleCores:idle,load:2,foreground:{cpu:8,unrealizedMegabytes:unrealized}}),alone=host({availableMegabytes:available,idleCores:idle,load:2});
  const withRead=ownerPoolTargetV1({host:row,children,childMegabytes:1000}).children,without=ownerPoolTargetV1({host:alone,children,childMegabytes:1000}).children;
  expect(withRead,JSON.stringify({available,idle,children,unrealized})).toBeLessThanOrEqual(without);
  const floor=Math.max(P.floorMinimumMegabytes,Math.min(P.floorMaximumMegabytes,Math.floor(16384*P.floorShareBasisPoints/10000)));
  if(available>=floor)expect(withRead).toBeGreaterThanOrEqual(1);
  const budget=Math.floor((available-floor-Math.min(unrealized,Math.floor(16384*P.ownHoldShareBasisPoints/10000)))/1000);
  expect(withRead,"memory and idle cores decide, the booked cores do not").toBe(Math.max(1,Math.min(8,Math.max(Math.ceil(8*P.minimumShareBasisPoints/10000),children+idle),children+budget)));
 }
 const none=ownerPoolSizingV1({host:host({availableMegabytes:11000,idleCores:8,load:0}),children:0,childMegabytes:1000});
 expect(none).toMatchObject({children:8,reason:null});
});

test("one child always runs above the floor, whatever the machine's own work reserves and however big the estimate",()=>{
 for(const unrealized of[0,5000,50000])for(const childMegabytes of[500,2000,9000]){
  const sizing=ownerPoolSizingV1({host:host({availableMegabytes:2048+1,foreground:{cpu:8,unrealizedMegabytes:unrealized}}),children:0,childMegabytes});
  expect(sizing.children).toBeGreaterThanOrEqual(1);expect(sizing.limit.freeMegabytes).toBeLessThanOrEqual(1);
 }
 expect(ownerPoolSizingV1({host:host({pressure:"critical"}),children:3,childMegabytes:800})).toMatchObject({children:0,reason:"pressure",limit:{limit:"pressure"}});
});

test("a read whose farm waits for memory the gate holds back on paper while 19 GB are free: the pool keeps what the waiting request asks for, not the whole reservation, and the idle cores beyond the waiter's share",()=>{
 const asked=ownerPoolSizingV1({host:{...serverAt0941,availableMegabytes:19038,load:30.41,idleCores:6,foreground:{cpu:9,unrealizedMegabytes:15939,waiters:1,waitingMegabytes:1536,oldestWaitMilliseconds:0}},children:1,childMegabytes:1064});
 expect(asked).toMatchObject({children:4,reason:"yielding",limit:{limit:"cores",heldMegabytes:Math.floor(32768*P.ownHoldShareBasisPoints/10000)+1536,freeMegabytes:5299}});
 const unknown=ownerPoolSizingV1({host:{...serverAt0941,availableMegabytes:19038,load:2,idleCores:6,foreground:{cpu:2,unrealizedMegabytes:15939,waiters:1,oldestWaitMilliseconds:0}},children:1,childMegabytes:1064});
 expect(unknown).toMatchObject({children:1,reason:"nothing-fits",limit:{limit:"memory",heldMegabytes:15939,freeMegabytes:0}});
});

test("the host view of the capacity gate maps to the pool's host: an idle farm worker of the read books a core and reserves memory, a pool child is never in it",()=>{
 const grant=(label:string,cpu:number,reservedMegabytes:number,observedMegabytes:number|null)=>({label,cpu,reservedMegabytes,observedMegabytes});
 const view={host:{cores:10,totalMegabytes:32768,availableMegabytes:15851,pressure:"normal",load:43.45,swapOutMegabytesPerSecond:0},grants:[...Array.from({length:9},()=>grant("graph-farm",1,1536,0)),grant("projects",2,14336,7826)],waiters:[]};
 const mapped=ownerPoolHostOfV1(view as never,1000);
 expect(mapped.foreground).toMatchObject({cpu:11,unrealizedMegabytes:9966,waiters:0,waitingMegabytes:0});
 const farm={request:{id:"w",operation:"graph-farm",label:"graph-farm",priority:"batch",minimumJobs:1,preferredJobs:2,fixedMegabytes:100,perJobMegabytes:1536,cpuPerJob:1},arrivedAtMs:0,pid:1},sized={request:{...farm.request,id:"x",label:"build",minimumMegabytes:900},arrivedAtMs:500,pid:2};
 const waiting=ownerPoolHostOfV1({...view,waiters:[farm,sized]} as never,5000);
 expect(waiting.foreground).toMatchObject({waiters:2,waitingMegabytes:1636+900,oldestWaitMilliseconds:4500});
});

const unitsOf=(count:number,burn=400):OwnerPoolUnitV1[]=>Array.from({length:count},(_all,index)=>({name:`owner-${String(index).padStart(4,"0")}`,kind:"k",expectedMilliseconds:burn,json:{name:`owner-${index}`}}));
const manifestOf=(units:readonly OwnerPoolUnitV1[],tag:string,config:object):OwnerPoolManifestV1=>{
 const partial={version:1 as const,identity:"identity-"+tag,globals:{projectAuthority:"a",registryDigest:"r",commandRevision:"c",late:false},publisher:"p:"+tag};
 return{head:{...partial,revision:"revision-a",id:ownerPoolIdV1(partial,units),createdMs:Date.now()},session:Buffer.from(JSON.stringify(config)),units};
};
async function world(){
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 return{server,environment:{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"}};
}
const watch=(patches:Record<string,unknown>[])=>({update(patch:Record<string,unknown>){patches.push({...patch});},flush(){},current:()=>({})});

test("a pool with claimable work that waits states the exact numbers in English and German: what the next child needs, what is free above the floor, what the machine's own work holds, who runs",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("root"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  expect(await publishOwnerPoolV1({client,owner:"o",manifest:manifestOf(unitsOf(6),"memory",{burnMilliseconds:10,revision:"revision-a"})})).toBe(true);
  const sizing=ownerPoolSizingV1({host:host({availableMegabytes:6000,idleCores:6,load:2,foreground:{cpu:0,unrealizedMegabytes:3000}}),children:2,childMegabytes:1237}),patches:Record<string,unknown>[]=[];
  expect(sizing).toMatchObject({children:3,reason:"nothing-fits",limit:{limit:"memory",needMegabytes:1237,freeMegabytes:258,heldMegabytes:2457,floorMegabytes:2048}});
  await runOwnerPoolV1({root,environment:{...process.env,...environment},signal:AbortSignal.timeout(20000),capacity:async()=>sizing,once:true,driver:driverPath,priority:0,targetPollMilliseconds:100,pollMilliseconds:300,status:watch(patches),revision:"revision-a"});
  const said=patches.filter(patch=>patch.reason==="nothing-fits"&&patch.reasonText!==null&&patch.reasonText!==undefined) as {reasonText:{en:string;de:string}}[];
  expect(said.length).toBeGreaterThan(0);
  const {en,de}=said[0]!.reasonText;
  for(const text of[en,de])for(const number of["1237","258","2048","2457"])expect(text,number).toContain(number);
  expect(en).toContain("needs");expect(de).toContain("braucht");expect(en).not.toBe(de);
 }finally{await server.stop();}
},60000);

test("fewer children than the target is not silent: the pool says how many run and when the next may start",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("root"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  expect(await publishOwnerPoolV1({client,owner:"o",manifest:manifestOf(unitsOf(30,3000),"starting",{burnMilliseconds:2500,revision:"revision-a"})})).toBe(true);
  const patches:Record<string,unknown>[]=[],controller=new AbortController(),running=runOwnerPoolV1({root,environment:{...process.env,...environment},signal:controller.signal,capacity:async()=>({children:3,reason:null}),driver:driverPath,priority:0,targetPollMilliseconds:100,pollMilliseconds:300,status:watch(patches),revision:"revision-a"});
  const began=Date.now();while(Date.now()-began<20000&&!patches.some(patch=>patch.reason==="starting"))await new Promise<void>(done=>setTimeout(done,50));
  controller.abort();await running;
  const said=patches.find(patch=>patch.reason==="starting") as {reasonText:{en:string;de:string};children:number;target:number}|undefined;
  expect(said).toBeDefined();expect(said!.target).toBe(3);
  expect(said!.reasonText.en).toMatch(/^starting: \d of 3 children run, the next one may start in \d+ s$/u);
  expect(said!.reasonText.de).toMatch(/^startet: \d von 3 Kindprozessen laufen, der nächste darf in \d+ s starten$/u);
 }finally{await server.stop();}
},60000);
