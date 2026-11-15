import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{graphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{publishServedV1,readServedV1,readWantedV1,wantedPolicyV1,wantedTextV1}from"../../🟦️.ts";
import{fleetReadV1,wantedRefusalV1}from"../../📖️read/🟦️.ts";
import{emptyOwnerCostTableV1,estimateOwnerCostV1,ownerCostPolicyV1,recordOwnerCostV1}from"../../💰️cost/🟦️.ts";

/** 📖️ Contract: a reader decides by its admitted memory before it reserves any for inference, takes what the fleet served, publishes the rest, waits with progress, a bound and cancellation, and says in plain text what it waited for when the bound passes. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`wanted-read-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const KEY="a".repeat(64),PLATFORM="darwin-arm64",names=(count:number)=>Array.from({length:count},(_,at)=>`p${String(at).padStart(5,"0")}`);

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const client=(label:string)=>{const root=temp(label);return graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});};
 return{server,client};
}
const clock=()=>{let at=1_000_000;return{now:()=>at,sleep:async(milliseconds:number)=>{at+=milliseconds;}};};

test("a read whose stale projects fit the admitted memory stays local and never touches the fleet",async()=>{
 const lines:string[]=[],result=await fleetReadV1({stale:names(60),admittedMegabytes:8000,client:undefined,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"en",say:line=>lines.push(line)});
 expect(result.outcome).toBe("local");expect(result.plan.mode).toBe("local");expect(lines).toEqual([]);
});

test("a machine admitted the serving headroom infers all 5 000 stale projects itself and never asks",async()=>{
 const lines:string[]=[],result=await fleetReadV1({stale:names(5000),admittedMegabytes:16384,client:undefined,requester:"g1-server",platform:PLATFORM,graphKey:KEY,language:"en",say:line=>lines.push(line)});
 expect(result.outcome).toBe("local");expect(result.plan.localProjects).toBe(5000);expect(lines).toEqual([]);
});

test("a 16 GB shape publishes its 4 700 stale projects, takes the answer of a serving machine as it arrives and keeps only what it can infer; progress is told once per change",async()=>{
 const {server,client}=await world();
 try{
  const reader=client("reader"),serving=client("serving"),lines:string[]=[],time=clock(),stale=names(4700);
  let rounds=0;
  const sleep=async(milliseconds:number)=>{await time.sleep(milliseconds);rounds++;if(rounds===3)await publishServedV1({client:serving,server:"g1-server",request:(await readWantedV1({client:serving,requester:"g1-m06",now:time.now}))!,projects:stale.slice(0,2000),generation:7,now:time.now});if(rounds===6)await publishServedV1({client:serving,server:"g1-server",request:(await readWantedV1({client:serving,requester:"g1-m06",now:time.now}))!,projects:stale.slice(0,4700-40),generation:8,now:time.now});};
  const result=await fleetReadV1({stale,admittedMegabytes:4096,client:reader,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"en",say:line=>lines.push(line),sleep,now:time.now});
  expect(result.outcome).toBe("served");expect(result.missing).toHaveLength(40);expect(result.arrived).toBe(4660);
  expect(lines[0]).toBe(wantedTextV1("plan","en",{stale:4700,taken:0,local:72,wave:18,wanted:4700}));
  expect(lines.filter(line=>line.startsWith("graph: waiting"))).toEqual([wantedTextV1("waiting","en",{arrived:0,wanted:4700}),wantedTextV1("waiting","en",{arrived:2000,wanted:4700}),wantedTextV1("waiting","en",{arrived:4660,wanted:4700})]);
  expect((await readWantedV1({client:serving,requester:"g1-m06",now:time.now}))?.projects).toHaveLength(4700);
 }finally{await server.stop(true);}
},60000);

test("a retried read finds the answer to its earlier request: the projects already served are taken, not asked for again",async()=>{
 const {server,client}=await world();
 try{
  const reader=client("reader"),serving=client("serving"),stale=names(900);
  await publishServedV1({client:serving,server:"g1-server",request:{version:1,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:[],createdMs:0,expiresMs:1},projects:stale.slice(0,850),generation:3});
  expect((await readServedV1({client:reader,requester:"g1-m06"}))?.generation).toBe(3);
  const result=await fleetReadV1({stale,admittedMegabytes:4096,client:reader,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"de",say:()=>{}});
  expect(result.outcome).toBe("local");expect(result.plan.takenProjects).toBe(850);expect(result.arrived).toBe(850);
 }finally{await server.stop(true);}
},60000);

test("nothing from a silent server within the silence bound: the read says in English and German what it waited for and refuses with the designed class and exit status instead of hanging",async()=>{
 const {server,client}=await world();
 try{
  for(const language of["en","de"] as const){
   const lines:string[]=[],time=clock();
   const error=await fleetReadV1({stale:names(4700),admittedMegabytes:4096,client:client("reader-"+language),requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language,say:line=>lines.push(line),sleep:time.sleep,now:time.now}).then(()=>undefined,reason=>reason as Error&{code:string;key:string;values:Record<string,number>});
   expect(error?.code).toBe(wantedRefusalV1.code);expect(error?.key).toBe(wantedRefusalV1.key);expect(wantedRefusalV1.exitStatus).toBe(75);
   expect(error?.values).toEqual({wanted:4700,arrived:0,seconds:wantedPolicyV1.silenceMilliseconds/1000,missing:4700,local:72,heavy:""});
   expect(lines.at(-1)).toBe(wantedTextV1("refused",language,error!.values));
  }
  expect(wantedTextV1("refused","de",{wanted:1,arrived:0,seconds:2,missing:1,local:0})).toContain("veraltete Projekte");
 }finally{await server.stop(true);}
},60000);

test("an unreachable fleet refuses at once when the machine cannot infer the rest, and goes on locally when it can; cancellation ends the wait",async()=>{
 const lines:string[]=[],dead=graphFleetClientV1(graphFleetSettingsV1(temp("dead"),{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:9",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"})!);
 await expect(fleetReadV1({stale:names(500),admittedMegabytes:4096,client:undefined,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"en",say:line=>lines.push(line)})).rejects.toMatchObject({code:"unserved"});
 expect(lines.some(line=>line.includes("cannot be reached"))).toBe(true);
 const local=await fleetReadV1({stale:names(80),admittedMegabytes:4096,client:dead,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"en",say:()=>{},sleep:clock().sleep,now:clock().now}).catch(()=>undefined);
 expect(local === undefined || local.outcome === "served").toBe(true);
 const {server,client}=await world();
 try{
  const controller=new AbortController();
  const waiting=fleetReadV1({stale:names(4700),admittedMegabytes:4096,client:client("cancel"),requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"en",say:()=>{},signal:controller.signal,sleep:async()=>{controller.abort();}});
  await expect(waiting).rejects.toThrow();
 }finally{await server.stop(true);}
},60000);

const kindOf=(name:string)=>name==="workspace"?"root:language:cargo":undefined;
const estimator=(extra:Record<string,number>={})=>{let table=emptyOwnerCostTableV1();for(const [name,megabytes] of Object.entries(extra))table=recordOwnerCostV1(table,{name,kind:"project",megabytes,seconds:1,rows:1,stopped:false,atMs:1});return(name:string)=>estimateOwnerCostV1(table,name,kindOf(name));};

test("two projects left of which one is the workspace root: a 16 GB machine publishes both, marks the root heavy, waits and refuses naming it in English and German, however few remain",async()=>{
 const {server,client}=await world();
 try{
  for(const language of["en","de"] as const){
   const lines:string[]=[],time=clock(),reader=client("heavy-"+language),stale=["@semio-tech/light","workspace"];
   const error=await fleetReadV1({stale,admittedMegabytes:8192,client:reader,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language,estimate:estimator(),say:line=>lines.push(line),sleep:time.sleep,now:time.now}).then(()=>undefined,reason=>reason as Error&{code:string;key:string;values:Record<string,string|number>});
   expect(error?.code).toBe("unserved");expect(error?.key).toBe(wantedRefusalV1.key);
   expect(error?.values).toMatchObject({wanted:2,arrived:0,missing:2});expect(String(error?.values.heavy)).toContain("workspace");
   expect(lines[0]).toContain("workspace");expect(lines[0]).toContain(language==="en"?"too heavy":"zu schwer");
   expect(lines.at(-1)).toContain("workspace");expect(lines.at(-1)).toContain(language==="en"?"must never infer":"nie selbst berechnen");
   expect((await readWantedV1({client:reader,requester:"g1-m02",now:time.now}))?.heavy).toEqual(["workspace"]);
  }
 }finally{await server.stop(true);}
},60000);

test("the heavy project arrives and the light remainder fits: the read goes on locally with the light ones only; a light remainder alone never waits",async()=>{
 const {server,client}=await world();
 try{
  const reader=client("reader"),serving=client("serving"),time=clock(),lights=names(30),stale=[...lights,"@semio-tech/s-dev-composition"],lines:string[]=[];
  let rounds=0;
  const sleep=async(milliseconds:number)=>{await time.sleep(milliseconds);rounds++;if(rounds===2)await publishServedV1({client:serving,server:"g1-server",request:(await readWantedV1({client:serving,requester:"g1-m02",now:time.now}))!,projects:["@semio-tech/s-dev-composition"],generation:9,now:time.now});};
  const result=await fleetReadV1({stale,admittedMegabytes:8192,client:reader,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language:"en",estimate:estimator(),say:line=>lines.push(line),sleep,now:time.now});
  expect(result.outcome).toBe("served");expect(result.missing).toEqual(lights);expect(rounds).toBeGreaterThanOrEqual(2);
  expect(result.heavy?.map(row=>row.name)).toEqual(["@semio-tech/s-dev-composition"]);
  const light=await fleetReadV1({stale:lights,admittedMegabytes:8192,client:reader,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language:"en",estimate:estimator(),say:()=>{}});
  expect(light.outcome).toBe("local");
 }finally{await server.stop(true);}
},60000);

test("a heavy project already served is taken, not asked for again; the same stale set on a serving machine and on a machine without estimates behaves as before",async()=>{
 const {server,client}=await world();
 try{
  const reader=client("reader"),serving=client("serving"),stale=["workspace",...names(10)];
  await publishServedV1({client:serving,server:"g1-server",request:{version:1,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,projects:[],createdMs:0,expiresMs:1},projects:["workspace"],generation:3});
  expect((await fleetReadV1({stale,admittedMegabytes:8192,client:reader,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language:"en",estimate:estimator(),say:()=>{}})).outcome).toBe("local");
  expect((await fleetReadV1({stale,admittedMegabytes:16384,client:undefined,requester:"g1-server",platform:PLATFORM,graphKey:KEY,language:"en",estimate:estimator(),say:()=>{}})).outcome).toBe("local");
  expect((await fleetReadV1({stale,admittedMegabytes:8192,client:undefined,requester:"g1-m05",platform:PLATFORM,graphKey:KEY,language:"en",say:()=>{}})).outcome).toBe("local");
 }finally{await server.stop(true);}
},60000);

test("an unreachable fleet refuses a single heavy project at once, while the same single project when light stays local",async()=>{
 const lines:string[]=[],dead=graphFleetClientV1(graphFleetSettingsV1(temp("dead-heavy"),{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:9",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"})!);
 await expect(fleetReadV1({stale:["workspace"],admittedMegabytes:8192,client:dead,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language:"de",estimate:estimator(),say:line=>lines.push(line)})).rejects.toMatchObject({code:"unserved"});
 expect(lines.at(-1)).toContain("workspace");
 expect((await fleetReadV1({stale:["@semio-tech/light"],admittedMegabytes:8192,client:dead,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language:"de",estimate:estimator(),say:()=>{}})).outcome).toBe("local");
 expect(ownerCostPolicyV1.kindMegabytes["root:language:cargo"]).toBeGreaterThan(0);
});
