import Ajv from"ajv";
import{afterAll,expect,test}from"bun:test";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{graphFleetClientV1}from"../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../🧶️nx/🟦️.ts";
import{awaitWantedV1,inferableProjectsV1,planStaleReadV1,publishWantedV1,readWantedV1,claimWantedV1,publishServedV1,readServedV1,wantedClaimKeyV1,wantedEstimateTextV1,wantedPolicyV1,wantedTextV1}from"../🟦️.ts";

/** 🙋️ Contract: a machine that cannot afford the inference of the stale projects asks the fleet instead. The decision follows the admitted memory (measured: one wave of 48 projects holds 7.1 GB), the request is one replaceable entry per requester in the fleet cache, a machine with headroom serves it once under a claim, and the requester waits a bounded time, takes records as they arrive and stops on cancellation. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`wanted-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const KEY="a".repeat(64);

test("the memory of the grant decides the wave: nothing below the useful minimum, a wave of 48 from 7200 MB, linear in between (the measured footprints of U)",()=>{
 const p=wantedPolicyV1;
 expect(inferableProjectsV1(1500)).toEqual({waveProjects:0,projects:0});
 expect(inferableProjectsV1(4096)).toEqual({waveProjects:18,projects:72});
 expect(inferableProjectsV1(6553)).toEqual({waveProjects:41,projects:164});
 expect(inferableProjectsV1(p.waveMegabytes)).toEqual({waveProjects:48,projects:192});
 expect(inferableProjectsV1(22380)).toEqual({waveProjects:48,projects:192});
 expect(inferableProjectsV1(Number.NaN)).toEqual({waveProjects:0,projects:0});
 expect(inferableProjectsV1(p.baseMegabytes+7*104)).toEqual({waveProjects:0,projects:0});
});

test("a 16 GB machine with 4.1 to 6.5 GB asks the fleet for 4 700 stale projects; a read with 150 stale projects stays local; projects the fleet already had are not asked for again",()=>{
 expect(planStaleReadV1({stale:4700,taken:0,admittedMegabytes:4096})).toEqual({mode:"fleet",waveProjects:18,localProjects:72,takenProjects:0,wantedProjects:4700});
 expect(planStaleReadV1({stale:4700,taken:4650,admittedMegabytes:4096})).toEqual({mode:"local",waveProjects:18,localProjects:50,takenProjects:4650,wantedProjects:0});
 expect(planStaleReadV1({stale:150,taken:0,admittedMegabytes:6553}).mode).toBe("local");
 expect(planStaleReadV1({stale:150,taken:0,admittedMegabytes:1500}).mode).toBe("fleet");
 expect(planStaleReadV1({stale:0,taken:0,admittedMegabytes:1500})).toEqual({mode:"local",waveProjects:0,localProjects:0,takenProjects:0,wantedProjects:0});
 expect(planStaleReadV1({stale:20000,taken:0,admittedMegabytes:32768}).wantedProjects).toBe(wantedPolicyV1.maximumProjects);
});

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const client=(label:string)=>{const root=temp(label);return graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});};
 return{server,client};
}

test("a request is read back by its requester alone whatever the platform of the reader, a newer one replaces it, an expired or foreign one is not served",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),reader=client("reader");
  expect(await readWantedV1({client:reader,requester:"g1-m03"})).toBeUndefined();
  expect(await publishWantedV1({client:requester,requester:"g1-m03",platform:"darwin-arm64",graphKey:KEY,projects:["b","a"]})).toBe(true);
  const first=await readWantedV1({client:reader,requester:"g1-m03"});
  expect(first?.projects).toEqual(["a","b"]);expect(first?.graphKey).toBe(KEY);
  expect(await readWantedV1({client:reader,requester:"g1-m04"})).toBeUndefined();
  expect((await readWantedV1({client:reader,requester:"g1-m03"}))?.platform).toBe("darwin-arm64");
  expect(await readWantedV1({client:reader,requester:"g1-m03",now:()=>Date.now()+wantedPolicyV1.ttlMilliseconds+1})).toBeUndefined();
  await publishWantedV1({client:requester,requester:"g1-m03",platform:"darwin-arm64",graphKey:"b".repeat(64),projects:["c"],now:()=>Date.now()+5});
  const second=await readWantedV1({client:reader,requester:"g1-m03"});
  expect(second?.projects).toEqual(["c"]);expect(second?.graphKey).toBe("b".repeat(64));
 }finally{await server.stop(true);}
},60000);

test("a machine takes over a live request once; the requester itself and a request of another graph state are left alone; a held claim keeps a second server away",async()=>{
 const {server,client}=await world();
 try{
  const requester=client("requester"),a=client("a"),b=client("b"),owned=new Set<string>();
  await publishWantedV1({client:requester,requester:"g1-m03",platform:"darwin-arm64",graphKey:KEY,projects:["p1","p2","p3"]});
  const base={platform:"darwin-arm64",requesters:["g1-m03","g1-server"],graphKey:async()=>KEY,known:(key:string)=>owned.has(key)};
  expect(await claimWantedV1({...base,client:requester,owner:"g1-m03"})).toEqual([]);
  expect(await claimWantedV1({...base,client:a,owner:"g1-server",graphKey:async()=>"c".repeat(64)})).toEqual([]);
  const first=await claimWantedV1({...base,client:a,owner:"g1-server"});
  expect(first.map(request=>[request.requester,request.projects])).toEqual([["g1-m03",["p1","p2","p3"]]]);
  for(const request of first)owned.add(wantedClaimKeyV1(request));
  expect(await claimWantedV1({...base,client:a,owner:"g1-server"})).toEqual([]);
  expect(await claimWantedV1({...base,client:b,owner:"g1-m06",known:()=>false})).toEqual([]);
 }finally{await server.stop(true);}
},60000);

test("the requester takes records as they arrive, reports progress, gives up after the silence of the server and stops on cancellation",async()=>{
 let clock=0;const arrivals=new Map<string,number>([["a",0],["b",4000],["c",1e12]]),seen:number[]=[];
 const take=async(name:string)=>clock>=arrivals.get(name)!;
 const missing=await awaitWantedV1({projects:["a","b","c"],take,now:()=>clock,sleep:async ms=>{clock+=ms;},progress:progress=>seen.push(progress.arrived)});
 expect(missing).toEqual(["c"]);
 expect(seen[0]).toBe(1);expect(seen.at(-1)).toBe(2);expect(seen).toEqual([...seen].sort());
 expect(clock).toBeGreaterThanOrEqual(4000+wantedPolicyV1.silenceMilliseconds);expect(clock).toBeLessThan(4000+wantedPolicyV1.silenceMilliseconds+wantedPolicyV1.pollMilliseconds+1);
 const controller=new AbortController();let rounds=0;
 await expect(awaitWantedV1({projects:["z"],take:async()=>false,signal:controller.signal,now:()=>0,sleep:async()=>{if(++rounds===2)controller.abort();}})).rejects.toBeDefined();
 expect(rounds).toBe(2);
 expect(await awaitWantedV1({projects:[],take,now:()=>0})).toEqual([]);
 clock=0;expect(await awaitWantedV1({projects:["x"],take:async()=>{throw Error("down");},now:()=>clock,sleep:async ms=>{clock+=ms;}})).toEqual(["x"]);
});

test("the wait follows progress: it outlives the first-arrival bound while records keep arriving, ends on silence of the server, on no arrival although the server is alive, and at the absolute maximum",async()=>{
 const p=wantedPolicyV1,names=Array.from({length:1000},(_,index)=>"p"+index);
 const run=async(arrivalAt:(index:number)=>number,alive?:(clock:number)=>number|undefined)=>{
  let clock=0;const when=names.map((_,index)=>arrivalAt(index));
  const missing=await awaitWantedV1({projects:names,take:async name=>clock>=when[Number(name.slice(1))]!,...(alive===undefined?{}:{alive:()=>alive(clock)}),now:()=>clock,sleep:async ms=>{clock+=ms;}});
  return{missing:missing.length,clock};
 };
 const steady=await run(index=>index*5000);
 expect(steady.missing).toBe(0);expect(steady.clock).toBeGreaterThan(p.waitMilliseconds);
 const silent=await run(index=>index<10?index*1000:Infinity);
 expect(silent.missing).toBe(990);expect(silent.clock).toBeGreaterThanOrEqual(9000+p.silenceMilliseconds);expect(silent.clock).toBeLessThan(9000+p.silenceMilliseconds+p.pollMilliseconds+1);
 const alive=await run(index=>index<10?index*1000:Infinity,clock=>clock);
 expect(alive.missing).toBe(990);expect(alive.clock).toBeGreaterThanOrEqual(9000+p.waitMilliseconds);expect(alive.clock).toBeLessThan(9000+p.waitMilliseconds+p.pollMilliseconds+1);
 const slow=await run(index=>index*200_000,clock=>clock);
 expect(slow.missing).toBeGreaterThan(0);expect(slow.clock).toBeGreaterThanOrEqual(p.waitMaximumMilliseconds);expect(slow.clock).toBeLessThan(p.waitMaximumMilliseconds+p.pollMilliseconds+1);
});

test("Law: the wait is bound by the absence of progress, not by wall time: a server whose progress advances every 60 s keeps the reader waiting 40 minutes (far beyond the bound without progress), the same server standing still ends it after the bound with the cause `stalled`, a silent one with `silent`, a server that reports progress for ever ends at the absolute maximum with `limit`",async()=>{
 const p=wantedPolicyV1,names=Array.from({length:50},(_,index)=>"p"+index),FORTY=40*60_000;
 const run=async(input:Readonly<{progress:(clock:number)=>number|undefined;alive:(clock:number)=>number|undefined;arrived:(clock:number)=>number}>)=>{
  let clock=0;const renewed:{clock:number;missing:number}[]=[],ends:string[]=[];
  const missing=await awaitWantedV1({projects:names,take:async name=>Number(name.slice(1))<input.arrived(clock),alive:()=>input.alive(clock),progressSign:()=>input.progress(clock),renew:async left=>{renewed.push({clock,missing:left.length});},end:cause=>ends.push(cause),now:()=>clock,sleep:async ms=>{clock+=ms;}});
  return{missing:missing.length,clock,renewed,ends};
 };
 const moving=await run({progress:clock=>Math.floor(clock/60_000),alive:clock=>clock,arrived:clock=>clock>=FORTY?50:0});
 expect(moving.missing).toBe(0);expect(moving.ends).toEqual([]);expect(moving.clock).toBeGreaterThanOrEqual(FORTY);expect(moving.clock).toBeGreaterThan(p.waitMilliseconds);
 expect(moving.renewed.length).toBeGreaterThanOrEqual(Math.floor(FORTY/p.renewMilliseconds)-1);expect(moving.renewed.every(row=>row.missing===50)).toBe(true);
 const standing=await run({progress:clock=>Math.floor(Math.min(clock,600_000)/60_000),alive:clock=>clock,arrived:()=>0});
 expect(standing.ends).toEqual(["stalled"]);expect(standing.missing).toBe(50);expect(standing.clock).toBeGreaterThanOrEqual(600_000+p.waitMilliseconds);expect(standing.clock).toBeLessThan(600_000+p.waitMilliseconds+p.pollMilliseconds+1);
 const silent=await run({progress:clock=>Math.floor(clock/60_000),alive:()=>0,arrived:()=>0});
 expect(silent.ends).toEqual(["silent"]);expect(silent.clock).toBeGreaterThanOrEqual(p.silenceMilliseconds);expect(silent.clock).toBeLessThan(p.silenceMilliseconds+p.pollMilliseconds+1);
 const endless=await run({progress:clock=>Math.floor(clock/60_000),alive:clock=>clock,arrived:()=>0});
 expect(endless.ends).toEqual(["limit"]);expect(endless.clock).toBeGreaterThanOrEqual(p.waitMaximumMilliseconds);expect(endless.clock).toBeLessThan(p.waitMaximumMilliseconds+p.pollMilliseconds+1);
},60000);

test("a request is idempotent: publishing it again with the creation time of the first renews it (later expiry, fewer projects) and keeps the claim key, a new creation time is new work; the estimate reads in English and German",async()=>{
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 try{
  const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"},root=temp("client"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})}),p=wantedPolicyV1;
  await publishWantedV1({client,requester:"g1-m03",platform:"darwin-arm64",graphKey:KEY,projects:["a","b","c"],createdMs:1000,now:()=>1000});
  const first=(await readWantedV1({client,requester:"g1-m03",now:()=>1000}))!;
  await publishWantedV1({client,requester:"g1-m03",platform:"darwin-arm64",graphKey:KEY,projects:["c"],createdMs:1000,now:()=>1000+30*60_000});
  const renewed=(await readWantedV1({client,requester:"g1-m03",now:()=>1000+30*60_000}))!;
  expect(renewed.projects).toEqual(["c"]);expect(renewed.expiresMs).toBe(1000+30*60_000+p.ttlMilliseconds);expect(wantedClaimKeyV1(renewed)).toBe(wantedClaimKeyV1(first));
  await publishWantedV1({client,requester:"g1-m03",platform:"darwin-arm64",graphKey:KEY,projects:["c"],now:()=>1000+31*60_000});
  expect(wantedClaimKeyV1((await readWantedV1({client,requester:"g1-m03",now:()=>1000+31*60_000}))!)).not.toBe(wantedClaimKeyV1(first));
 }finally{await server.stop(true);}
 expect(wantedEstimateTextV1(null,"en")).toBe("unknown");expect(wantedEstimateTextV1(null,"de")).toBe("unbekannt");
 expect(wantedEstimateTextV1(0,"en")).toBe("under a minute");expect(wantedEstimateTextV1(90_000,"de")).toBe("etwa 2 min");expect(wantedEstimateTextV1(3_900_000,"en")).toBe("about 1 h 5 min");
},60000);

test("the answer with its progress record validates against the schema with Ajv as written and as read back, a record without progress stays valid, a damaged progress is refused, and every text exists in both languages with the same placeholders",async()=>{
 const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/Served",definitions:schema.definitions});
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 try{
  const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"},root=temp("client"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})}),request={version:1 as const,requester:"g1-m03",platform:"darwin-arm64",graphKey:KEY,projects:["a","b"],createdMs:1,expiresMs:2};
  await publishServedV1({client,server:"g1-server",request,projects:["a"],generation:null,progress:{total:2,progressAtMs:5000,remainingMilliseconds:null},now:()=>6000});
  const row=(await readServedV1({client,requester:"g1-m03"}))!;
  expect(row).toMatchObject({total:2,progressAtMs:5000,remainingMilliseconds:null,atMs:6000});expect(validate(row)).toBe(true);
  await publishServedV1({client,server:"g1-server",request,projects:["a"],generation:3,now:()=>7000});
  const bare=(await readServedV1({client,requester:"g1-m03"}))!;expect(bare.progressAtMs).toBeUndefined();expect(validate(bare)).toBe(true);
  expect(validate({...row,progressAtMs:-1})).toBe(false);expect(validate({...row,remainingMilliseconds:1.5})).toBe(false);expect(validate({...row,extra:1})).toBe(false);
 }finally{await server.stop(true);}
 const holes=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort();
 for(const key of Object.keys(schema.definitions.Texts.const.en) as (keyof typeof schema.definitions.Texts.const.en)[])expect(holes(schema.definitions.Texts.const.de[key])).toEqual(holes(schema.definitions.Texts.const.en[key]));
 expect(wantedTextV1("waitingServer","de",{arrived:1,wanted:2,age:3,estimate:"etwa 4 min"})).toBe("Graph: warte auf die Flotte: 1 von 2 Projekten eingetroffen; der Server kam zuletzt vor 3 s voran, geschätzte Restzeit: etwa 4 min");
},60000);
