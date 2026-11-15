import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{graphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{claimWantedV1,fleetAdmittedMegabytesV1,publishWantedV1,readServedV1,readWantedV1,wantedPolicyV1,wantedTextV1}from"../../🟦️.ts";
import{fleetReadV1}from"../../📖️read/🟦️.ts";
import{estimateOwnerCostV1,mergeOwnerCostTablesV1,ownerCostPolicyV1,publishOwnerCostsV1,readOwnerCostsV1,recordOwnerCostV1,emptyOwnerCostTableV1}from"../../💰️cost/🟦️.ts";
import{runWantedServerV1,wantedServerPolicyV1,type WantedInferV1}from"../../🛠️server/🟦️.ts";

/** 🌍️ Contract: the request of a Linux worker is read, claimed and answered by a macOS server (a slot is per requester, a machine serves its platform class), a request of a platform class nobody serves is counted and left alone, a worker that can give 0 MB per project at a small booking still infers the unknown projects, and the costs the server measured reach a worker of another platform. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`wanted-two-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const KEY="a".repeat(64),roomy=async()=>({freeMegabytes:20000,pressure:"normal" as const}),ok=(generation:number):WantedInferV1=>({ok:true,status:0,seconds:12,generation,detail:""});

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const client=(label:string)=>{const root=temp(label);return graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});};
 return{server,client};
}
const base=(serving:ReturnType<Awaited<ReturnType<typeof world>>["client"]>,lines:string[],requesters:string[],reads:string[])=>({client:serving,owner:"g1-server",platform:"darwin-arm64",steerDirectory:temp("steer"),language:"en" as const,requesters:async()=>requesters,headroom:roomy,infer:async(reason:string)=>{reads.push(reason);return ok(9);},quietMilliseconds:async()=>0,say:(line:string)=>lines.push(line),signal:new AbortController().signal,sleep:async()=>{},rounds:1});

test("the request of the Linux worker is read without its platform, claimed by the macOS server, served and answered; the answer is read back by the worker",async()=>{
 const {server,client}=await world();
 try{
  const linux=client("linux"),mac=client("mac"),lines:string[]=[],reads:string[]=[];
  expect(await publishWantedV1({client:linux,requester:"g1-m04",platform:"linux-x64",graphKey:KEY,projects:["p2","p1","p3"]})).toBe(true);
  const seen=await readWantedV1({client:mac,requester:"g1-m04"});
  expect(seen?.platform).toBe("linux-x64");expect(seen?.projects).toEqual(["p1","p2","p3"]);
  await runWantedServerV1(base(mac,lines,["g1-m04"],reads));
  expect(reads).toEqual(["request"]);
  const answer=await readServedV1({client:linux,requester:"g1-m04"});
  expect(answer?.projects).toEqual(["p1","p2","p3"]);expect(answer?.platform).toBe("linux-x64");expect(answer?.server).toBe("g1-server");
  expect(lines).toEqual([wantedTextV1("serving","en",{count:3,requester:"g1-m04"}),wantedTextV1("served","en",{count:3,requester:"g1-m04",seconds:12})]);
 }finally{await server.stop(true);}
},60000);

test("a request of a platform class this machine does not serve is counted, said once, left unclaimed and shown in the heartbeat",async()=>{
 const {server,client}=await world();
 try{
  const windows=client("windows"),linux=client("linux"),mac=client("mac"),other=client("other"),lines:string[]=[],reads:string[]=[];
  await publishWantedV1({client:windows,requester:"g1-w1",platform:"win32-x64",graphKey:KEY,projects:["w1","w2"]});
  await publishWantedV1({client:linux,requester:"g1-m04",platform:"linux-x64",graphKey:KEY,projects:["l1"]});
  const policy={...wantedServerPolicyV1,aliveRounds:1};
  await runWantedServerV1({...base(mac,lines,["g1-w1","g1-m04"],reads),server:policy,rounds:2});
  expect(reads).toEqual(["request"]);
  expect(lines.filter(line=>line.includes("g1-w1")&&line.includes("win32-x64"))).toHaveLength(1);
  expect(lines.some(line=>line.startsWith("graph-server: alive, 2 machines seen, 1 requests held, 1 of other platform classes left to others"))).toBe(true);
  expect((await claimWantedV1({client:other,owner:"g1-w9",platform:"win32-x64",requesters:["g1-w1","g1-m04"],known:()=>false})).map(request=>request.requester)).toEqual(["g1-w1"]);
  expect((await readServedV1({client:windows,requester:"g1-w1"}))?.projects??[]).toEqual([]);
 }finally{await server.stop(true);}
},60000);

test("a Linux worker names only the measured heavy projects even when its early booking is small, and with its real capacity it infers a wave of unknown projects itself",async()=>{
 const small=fleetAdmittedMegabytesV1({hostMegabytes:15848,availableMegabytes:9987});
 expect(small).toBe(7924);
 expect(fleetAdmittedMegabytesV1({hostMegabytes:15848,availableMegabytes:3000})).toBe(3000-wantedPolicyV1.availableFloorMegabytes);
 expect(fleetAdmittedMegabytesV1({hostMegabytes:15848})).toBe(7924);expect(fleetAdmittedMegabytesV1({hostMegabytes:15848,availableMegabytes:100})).toBe(0);
 const names=[...Array.from({length:517},(_,at)=>`@semio-tech/p${at}`),"@semio-tech/s-dev-composition","workspace"],lines:string[]=[];
 const result=await fleetReadV1({stale:names,admittedMegabytes:2048,client:undefined,requester:"g1-m04",platform:"linux-x64",graphKey:KEY,language:"en",estimate:name=>estimateOwnerCostV1(undefined,name,name==="workspace"?"root:language:cargo":"project"),say:line=>lines.push(line)}).catch(error=>error);
 expect(String((result as Error).message??result)).toContain("graph.wanted-expired");
 const text=lines.join("\n");expect(text).toContain("@semio-tech/s-dev-composition");expect(text).toContain("workspace");expect(text).not.toContain("@semio-tech/p0 ");
 expect(text).toContain("linux-x64");expect(text).toContain("posix-shared");
 const local=await fleetReadV1({stale:names.slice(0,517),admittedMegabytes:small,client:undefined,requester:"g1-m04",platform:"linux-x64",graphKey:KEY,language:"en",estimate:name=>estimateOwnerCostV1(undefined,name,"project"),say:line=>lines.push(line)}).catch(error=>error);
 expect(String((local as Error).message??local)).toContain("graph.wanted-expired");
 expect(lines.some(line=>line.includes("it can infer 192 of the others (48 per wave)")||line.includes("this machine can infer 192 (48 per wave)"))).toBe(true);
},60000);

test("the costs the macOS server measured reach a worker of another platform and decide only by name",async()=>{
 const {server,client}=await world();
 try{
  const mac=client("mac"),linux=client("linux"),table=recordOwnerCostV1(emptyOwnerCostTableV1(),{name:"@semio-tech/os-kernel",kind:"project",megabytes:3900,seconds:70,rows:423,stopped:false,atMs:5});
  expect(await publishOwnerCostsV1({client:mac,server:"g1-server",table})).toBe(true);
  const read=await readOwnerCostsV1({client:linux});expect(read).toEqual(table);
  expect(estimateOwnerCostV1(mergeOwnerCostTablesV1([read]),"@semio-tech/os-kernel")).toEqual({megabytes:3900,source:"recorded"});
  expect(ownerCostPolicyV1.heavySources).toEqual(["recorded","seed"]);
 }finally{await server.stop(true);}
},60000);

test("the whole path: the Linux worker (16 GB, 300 stale projects, 192 inferable) asks, the macOS server answers, the worker continues with what it can infer itself left over",async()=>{
 const {server,client}=await world();
 try{
  const linux=client("linux"),mac=client("mac"),lines:string[]=[],serverLines:string[]=[],reads:string[]=[];let served=false;
  const stale=Array.from({length:300},(_,at)=>`@semio-tech/p${String(at).padStart(3,"0")}`);
  const result=await fleetReadV1({stale,admittedMegabytes:7924,client:linux,requester:"g1-m04",platform:"linux-x64",graphKey:KEY,language:"en",estimate:name=>estimateOwnerCostV1(undefined,name,"project"),say:line=>lines.push(line),
   async sleep(){if(served)return;served=true;await runWantedServerV1(base(mac,serverLines,["g1-m04"],reads));}});
  expect(result.outcome).toBe("served");expect(result.arrived).toBeGreaterThanOrEqual(300-192);expect(result.missing.length).toBeLessThanOrEqual(192);
  expect((await readServedV1({client:linux,requester:"g1-m04"}))?.projects).toHaveLength(300);
  expect(reads).toEqual(["request"]);
  expect(lines[0]).toBe(wantedTextV1("plan","en",{stale:300,taken:0,local:192,wave:48,wanted:300}));
  expect(serverLines[0]).toBe(wantedTextV1("serving","en",{count:300,requester:"g1-m04"}));
 }finally{await server.stop(true);}
},60000);
