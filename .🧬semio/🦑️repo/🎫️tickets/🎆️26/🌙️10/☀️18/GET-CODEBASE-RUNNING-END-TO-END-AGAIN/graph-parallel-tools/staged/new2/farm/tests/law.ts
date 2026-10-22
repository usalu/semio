import{test,expect}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{fileURLToPath}from"node:url";
import{join}from"node:path";
import Ajv from"ajv";
import{testPythonV1}from"../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import fixtureSchema from"./🧬️schema/🔣️.json";
import fixture from"./🔣️.json";
import moduleSchema from"../🧬️schema/🔣️.json";
import*as farm from"../🟨️.mjs";

const oraclePath=fileURLToPath(new URL("./🐍️oracle.py",import.meta.url)),python=testPythonV1();
const workerEntry=fileURLToPath(new URL("../👷️worker/🟨️.mjs",import.meta.url)),fakeService=new URL("./🧰️fake/🟨️.mjs",import.meta.url).href,otherService=new URL("./🧰️other/🟨️.mjs",import.meta.url).href;

/** 🐍️ Runs the independent Python model and returns its JSON answer. */
function oracle(args:string[]):any{
 const done=Bun.spawnSync([python!,"-I",oraclePath,...args]);
 if(done.exitCode!==0)throw Error("Oracle failed: "+new TextDecoder().decode(done.stderr));
 return JSON.parse(new TextDecoder().decode(done.stdout));
}

/** 🧪️ Opens a farm of real worker processes over the deterministic service. */
function openFarm(workers:number,extra:Record<string,unknown>={}){
 const pool=farm.createFarmV1({entry:workerEntry,services:{law:fakeService},workers,accept:(_kind:string,identity:unknown)=>identity==="law",...extra} as never);
 pool.open("law",{identity:"law"});
 return pool;
}

test("the fixture is closed, the policy and the texts equal their declaration, every text has both languages and equal holders",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
 expect(fixture.policy).toEqual(moduleSchema.$defs.Policy.const);
 expect(fixture.messages).toEqual(moduleSchema.$defs.Messages.const);
 expect(fixture.reasons).toEqual(moduleSchema.$defs.Reasons2.const);
 expect(JSON.parse(JSON.stringify(farm.GRAPH_FARM_REASONS_V1))).toEqual(moduleSchema.$defs.Reasons2.const);
 expect(Object.keys(moduleSchema.$defs.Reasons2.const.de)).toEqual(Object.keys(moduleSchema.$defs.Reasons2.const.en));
 expect(JSON.parse(JSON.stringify(farm.GRAPH_FARM_POLICY_V1))).toEqual(moduleSchema.$defs.Policy.const);
 expect(JSON.parse(JSON.stringify(farm.GRAPH_FARM_MESSAGES_V1))).toEqual(moduleSchema.$defs.Messages.const);
 const holders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]!).sort();
 const texts=moduleSchema.$defs.Messages.const;
 expect(Object.keys(texts.de)).toEqual(Object.keys(texts.en));
 for(const key of Object.keys(texts.en) as (keyof typeof texts.en)[])expect(holders(texts.de[key]),key).toEqual(holders(texts.en[key]));
},60000);

test("sizing, languages, texts and heap environments equal the recorded answers and the independent model",()=>{
 const directory=mkdtempSync(join(tmpdir(),"farm-law-"));
 try{
  const path=join(directory,"fixture.json");writeFileSync(path,JSON.stringify(fixture));
  const model=oracle(["expect",path]);
  expect(model).toEqual(fixture.expect);
  fixture.plan.forEach((row,index)=>{
   const input={...row.input,host:{...row.input.host}} as never;
   if("error" in fixture.expect.plan[index]!)expect(()=>farm.planGraphFarmV1(input),row.name).toThrow();
   else expect(farm.planGraphFarmV1(input),row.name).toEqual(fixture.expect.plan[index] as never);
  });
  fixture.languages.forEach((row,index)=>expect(farm.selectGraphFarmLanguagesV1(row.environment),row.name).toEqual(fixture.expect.languages[index] as never));
  fixture.messageCases.forEach((row,index)=>expect(farm.renderGraphFarmMessagesV1(row.key as never,row.fields as never,row.environment),row.name).toEqual(fixture.expect.messages[index] as never));
  fixture.runtime.forEach((row,index)=>expect(farm.graphFarmRuntimeEnvironmentV1(row.runtime,row.megabytes),row.name).toEqual(fixture.expect.runtime[index] as never));
 }finally{rmSync(directory,{recursive:true,force:true});}
},60000);

test("the intern tables send each distinct item once, equal the independent model and rebuild the items on the other side",()=>{
 fixture.tables.forEach((row,index)=>{
  const expected=fixture.expect.tables[index] as {ids:number[];fresh:string[]}[],encoder=farm.createInternEncoderV1<string>(),decoder=farm.createInternDecoderV1<string>();
  row.batches.forEach((batch,at)=>{
   const sent=encoder.encode(batch);
   expect([...sent.ids],row.name).toEqual(expected[at]!.ids);
   expect(sent.fresh.map(([,item])=>item),row.name).toEqual(expected[at]!.fresh);
   decoder.learn(sent.fresh);
   expect(decoder.decode(sent.ids),row.name).toEqual(batch);
  });
 });
 const same={id:1},byObject=farm.createInternEncoderV1<{id:number}>().encode([same,same,{id:1}]);
 expect([...byObject.ids]).toEqual([1,1,2]);expect(byObject.fresh.length).toBe(2);
 const first=farm.createInternEncoderV1<{key:string}>({keyOf:item=>item.key}),a={key:"k"},b={key:"k"},sent=first.encode([a,b,a]);
 expect([...sent.ids]).toEqual([1,1,1]);expect(sent.fresh.length).toBe(1);
 const canonical=new Map<string,{key:string}>(),decoder=farm.createInternDecoderV1<{key:string}>({canonical:item=>canonical.get(item.key)??(canonical.set(item.key,item),item)}),other=farm.createInternDecoderV1<{key:string}>({canonical:item=>canonical.get(item.key)??(canonical.set(item.key,item),item)});
 decoder.learn(sent.fresh);other.learn(farm.createInternEncoderV1<{key:string}>().encode([{key:"k"}]).fresh);
 expect(decoder.decode([1])[0]).toBe(other.decode([1])[0]);
 expect(()=>decoder.decode([2])).toThrow();
 expect(()=>decoder.learn([[0,{key:"z"}]])).toThrow();
 expect(()=>decoder.learn([[farm.GRAPH_FARM_POLICY_V1.maximumTableEntries+1,{key:"z"}]])).toThrow();
},60000);

test("whatever the completion order and the number of workers, every job gets exactly the answer a serial run gives it",async()=>{
 for(const workers of [1,2,4]){
  const pool=openFarm(workers);
  try{
   const jobs=Array.from({length:48},(_,n)=>({n,delay:(n*7919)%23}));
   const outcomes=await Promise.all(jobs.map(job=>pool.infer("law",job)));
   outcomes.forEach((outcome,index)=>{
    expect(outcome&&"reply"in outcome,`job ${index} of ${workers}`).toBe(true);
    const reply=(outcome as {reply:Record<string,any>}).reply;
    expect(reply.value).toEqual({n:index,square:index*index});expect([...reply.ids]).toEqual([index,index+1]);
   });
   const stats=pool.stats();expect(stats.jobs).toBe(48);expect(stats.replies).toBe(48);expect(stats.fallbacks).toBe(0);
   expect(new Set(outcomes.map(outcome=>(outcome as {reply:{pid:number}}).reply.pid)).size).toBeLessThanOrEqual(workers);
  }finally{await pool.close();}
 }
},60000);

test("a crashed worker, a failing job and a refused identity never fail the caller: it is told to infer the job itself",async()=>{
 const pool=openFarm(2);
 try{
  const outcomes=await Promise.all([{n:1},{n:2,crash:true},{n:3,fail:true},{n:4},{n:5}].map(job=>pool.infer("law",job)));
  expect(outcomes[1]).toEqual({fallback:expect.any(String)});
  expect((outcomes[2] as {failure:{code:string;message:string}}).failure).toMatchObject({code:"LAW",message:"failed 3"});
  for(const index of [0,3,4])expect((outcomes[index] as {reply:{value:{n:number}}}).reply.value.n).toBe(index+1);
  expect(pool.stats().lost).toBeGreaterThanOrEqual(1);
 }finally{await pool.close();}
 const refused=openFarm(2,{accept:()=>false});
 try{
  expect(await refused.infer("law",{n:1})).toEqual({fallback:expect.stringContaining("implementation differs")});expect(await refused.infer("law",{n:2})).toEqual({fallback:expect.stringContaining("disabled")});
  expect(refused.stats().disabled).toBe("implementation differs");
 }finally{await refused.close();}
},60000);

test("a job given as a function is evaluated when it is sent, with the slot and the generation of the process that gets it",async()=>{
 const pool=openFarm(1),seen:{slot:number;generation:number}[]=[];
 try{
  const first=await pool.infer("law",where=>{seen.push(where);return{n:1};});
  expect((first as {reply:{value:{n:number}}}).reply.value.n).toBe(1);
  expect(await pool.infer("law",where=>{seen.push(where);return{n:2,crash:true};})).toEqual({fallback:"exited"});
  const third=await pool.infer("law",where=>{seen.push(where);return{n:3};});
  expect((third as {reply:{value:{n:number}}}).reply.value.n).toBe(3);
  expect(seen).toEqual([{slot:0,generation:1},{slot:0,generation:1},{slot:0,generation:2}]);
 }finally{await pool.close();}
},60000);

test("a worker past its memory budget is replaced after its job and the answers stay right",async()=>{
 const pool=openFarm(1,{workerMegabytes:1});
 try{
  const first=await pool.infer("law",{n:1}),second=await pool.infer("law",{n:2});
  expect((first as {reply:{pid:number}}).reply.pid).not.toBe((second as {reply:{pid:number}}).reply.pid);
  expect(pool.stats().recycled).toBeGreaterThanOrEqual(2);
  expect((second as {reply:{value:{n:number}}}).reply.value.n).toBe(2);
 }finally{await pool.close();}
},60000);

test("cancelling stops the workers and rejects what waits, closing leaves no worker behind",async()=>{
 const controller=new AbortController(),pool=openFarm(1,{signal:controller.signal});
 const running=pool.infer("law",{n:1,delay:30000}),waiting=pool.infer("law",{n:2,delay:30000});
 await new Promise(done=>setTimeout(done,1500));
 const settled=Promise.all([running,waiting]);
 controller.abort(Error("law cancel"));
 expect(await settled).toEqual([{fallback:"closed"},{fallback:"closed"}]);
 expect(await pool.infer("law",{n:3})).toEqual({fallback:"closed"});
 const per=openFarm(1),own=new AbortController(),blocked=per.infer("law",{n:1,delay:30000}),queued=per.infer("law",{n:2},{signal:own.signal});
 own.abort(Error("law job cancel"));
 await expect(queued).rejects.toThrow("law job cancel");
 await per.close();
 expect(await blocked).toEqual({fallback:"closed"});
},60000);

test("the worker protocol messages are closed",()=>{
 const ajv=new Ajv({strict:false}),definitions=moduleSchema.$defs as Record<string,object>;
 const valid=(name:string,value:unknown)=>ajv.validate({$ref:"#/$defs/"+name,$defs:definitions} as never,value);
 expect(valid("Load",{type:"load",services:{law:"file:///a.mjs"}})).toBe(true);
 expect(valid("Load",{type:"load",services:{law:"http://a.mjs"}})).toBe(false);
 expect(valid("Loaded",{type:"loaded",pid:7})).toBe(true);
 expect(valid("Open",{type:"open",kind:"law",service:"file:///a.mjs",token:1,sessionFile:"/tmp/session.bin"})).toBe(true);
 expect(valid("Open",{type:"open",kind:"law",service:"file:///a.mjs",token:0,sessionFile:"/tmp/session.bin"})).toBe(false);
 expect(valid("Open",{type:"open",kind:"law",service:"file:///a.mjs",token:1,session:{}})).toBe(false);
 expect(valid("Opened",{type:"opened",kind:"law",token:2,identity:"x"})).toBe(true);
 expect(valid("Run",{type:"run",id:3,kind:"law",job:{n:1}})).toBe(true);
 expect(valid("Run",{type:"run",id:3,job:{n:1}})).toBe(false);
 expect(valid("Run",{type:"run",id:-1,kind:"law",job:{}})).toBe(false);
 expect(valid("Failure",{type:"failure",id:1,name:"Error",message:"x",code:null})).toBe(true);
 expect(valid("Failure",{type:"failure",id:1,name:"Error",message:"x",extra:1})).toBe(false);
 expect(valid("Result",{type:"result",id:1,rssMegabytes:12.5,value:{}})).toBe(true);
 expect(valid("Shutdown",{type:"shutdown",extra:true})).toBe(false);
},60000);

test("two service kinds share the same worker processes and a kind is served only after its session is open",async()=>{
 const pool=openFarm(1,{services:{law:fakeService}});
 try{
  const waiting=pool.infer("other",{n:4}),first=await pool.infer("law",{n:1});
  expect((first as {reply:{value:{n:number}}}).reply.value.n).toBe(1);
  pool.open("other",{identity:"other"},{service:otherService,accept:(_kind:string,identity:unknown)=>identity==="other"});
  const second=await waiting,third=await pool.infer("law",{n:2});
  expect((second as {reply:{kind:string;twice:number;pid:number}}).reply).toMatchObject({kind:"other",twice:8});
  expect((second as {reply:{pid:number}}).reply.pid).toBe((first as {reply:{pid:number}}).reply.pid);
  expect((third as {reply:{pid:number}}).reply.pid).toBe((first as {reply:{pid:number}}).reply.pid);
  expect(pool.stats().started).toBe(1);expect(pool.stats().opens).toBe(2);
  expect(()=>pool.open("unknown",{})).toThrow("Declared graph farm service required");
 }finally{await pool.close();}
},60000);

test("a prestarted pool has booted every worker before any job is sent, and the first job then waits for nothing but the session",async()=>{
 const pool=farm.createFarmV1({entry:workerEntry,services:{law:fakeService},workers:3,accept:()=>true} as never);
 try{
  pool.prestart();
  expect(pool.stats().started).toBe(3);
  await new Promise(done=>setTimeout(done,2500));
  expect(pool.stats().loadedAfterMilliseconds).toBeGreaterThan(0);expect(pool.stats().firstJobAfterMilliseconds).toBeUndefined();
  pool.open("law",{identity:"law"});
  const outcomes=await Promise.all(Array.from({length:6},(_,n)=>pool.infer("law",{n})));
  outcomes.forEach((outcome,n)=>expect((outcome as {reply:{value:{n:number}}}).reply.value.n).toBe(n));
  expect(pool.stats().started).toBe(3);expect(pool.stats().firstJobAfterMilliseconds).toBeGreaterThan(pool.stats().loadedAfterMilliseconds!-1);
 }finally{await pool.close();}
},60000);

test("a resident pool serves a later build with the processes it already booted, and a new session replaces the old one in every worker",async()=>{
 const policy={...farm.GRAPH_FARM_POLICY_V1,residentIdleMilliseconds:60000};
 const first=farm.acquireGraphFarmV1({key:"law-resident",entry:workerEntry,services:{law:fakeService},workers:2,accept:()=>true,policy} as never);
 let again:ReturnType<typeof farm.acquireGraphFarmV1>|undefined;
 try{
  first.farm.open("law",{identity:"law"});
  const before=await Promise.all([0,1,2,3].map(n=>first.farm.infer("law",{n})));
  const pids=new Set(before.map(outcome=>(outcome as {reply:{pid:number}}).reply.pid));
  first.release();
  again=farm.acquireGraphFarmV1({key:"law-resident",entry:workerEntry,services:{law:fakeService},workers:2,accept:()=>true,policy} as never);
  expect(again.farm).toBe(first.farm);
  again.farm.open("law",{identity:"law"});
  const after=await Promise.all([0,1,2,3].map(n=>again!.farm.infer("law",{n})));
  for(const outcome of after){expect(pids.has((outcome as {reply:{pid:number}}).reply.pid)).toBe(true);expect((outcome as {reply:{opens:number}}).reply.opens).toBe(2);}
  expect(again.farm.stats().started).toBe(2);
 }finally{again?.release();await farm.closeGraphFarmsV1();}
 const replaced=farm.acquireGraphFarmV1({key:"law-resident",entry:workerEntry,services:{law:fakeService},workers:1,accept:()=>true,policy} as never);
 try{expect(replaced.farm).not.toBe(first.farm);expect(first.farm.closed).toBe(true);}finally{replaced.release();await farm.closeGraphFarmsV1();}
},60000);

test("the last release keeps the processes for the idle time of the policy and then closes the pool",async()=>{
 const policy={...farm.GRAPH_FARM_POLICY_V1,residentIdleMilliseconds:300};
 const lease=farm.acquireGraphFarmV1({key:"law-idle",entry:workerEntry,services:{law:fakeService},workers:1,accept:()=>true,policy} as never);
 lease.farm.open("law",{identity:"law"});
 await lease.farm.infer("law",{n:1});
 lease.release();lease.release();
 expect(lease.farm.closed).toBe(false);
 await new Promise(done=>setTimeout(done,1200));
 expect(lease.farm.closed).toBe(true);
 const next=farm.acquireGraphFarmV1({key:"law-idle",entry:workerEntry,services:{law:fakeService},workers:1,accept:()=>true,policy} as never);
 try{expect(next.farm).not.toBe(lease.farm);}finally{next.release();await farm.closeGraphFarmsV1();}
},60000);

test("a pool limited to fewer workers than it has never uses the others",async()=>{
 const pool=openFarm(4);
 try{
  pool.limit(2);
  const outcomes=await Promise.all(Array.from({length:12},(_,n)=>pool.infer("law",{n,delay:20})));
  expect(new Set(outcomes.map(outcome=>(outcome as {reply:{pid:number}}).reply.pid)).size).toBeLessThanOrEqual(2);
  expect(pool.stats().started).toBeLessThanOrEqual(2);
  expect(()=>pool.limit(0)).toThrow("Positive worker limit required");
 }finally{await pool.close();}
},60000);
