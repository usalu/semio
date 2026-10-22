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
const workerEntry=fileURLToPath(new URL("../👷️worker/🟨️.mjs",import.meta.url)),fakeService=new URL("./🧰️fake/🟨️.mjs",import.meta.url).href;

/** 🐍️ Runs the independent Python model and returns its JSON answer. */
function oracle(args:string[]):any{
 const done=Bun.spawnSync([python!,"-I",oraclePath,...args]);
 if(done.exitCode!==0)throw Error("Oracle failed: "+new TextDecoder().decode(done.stderr));
 return JSON.parse(new TextDecoder().decode(done.stdout));
}

/** 🧪️ Opens a farm of real worker processes over the deterministic service. */
function openFarm(workers:number,extra:Record<string,unknown>={}){
 return farm.createFarmV1({entry:workerEntry,service:fakeService,session:{identity:"law"},workers,accept:identity=>identity==="law",...extra} as never);
}

test("the fixture is closed, the policy and the texts equal their declaration, every text has both languages and equal holders",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
 expect(fixture.policy).toEqual(moduleSchema.$defs.Policy.const);
 expect(fixture.messages).toEqual(moduleSchema.$defs.Messages.const);
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
   const outcomes=await Promise.all(jobs.map(job=>pool.infer(job)));
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
  const outcomes=await Promise.all([{n:1},{n:2,crash:true},{n:3,fail:true},{n:4},{n:5}].map(job=>pool.infer(job)));
  expect(outcomes[1]).toBeUndefined();
  expect((outcomes[2] as {failure:{code:string;message:string}}).failure).toMatchObject({code:"LAW",message:"failed 3"});
  for(const index of [0,3,4])expect((outcomes[index] as {reply:{value:{n:number}}}).reply.value.n).toBe(index+1);
  expect(pool.stats().lost).toBeGreaterThanOrEqual(1);
 }finally{await pool.close();}
 const refused=openFarm(2,{accept:()=>false});
 try{
  expect(await refused.infer({n:1})).toBeUndefined();expect(await refused.infer({n:2})).toBeUndefined();
  expect(refused.stats().disabled).toBe("implementation differs");
 }finally{await refused.close();}
},60000);

test("a job given as a function is evaluated when it is sent, with the slot and the generation of the process that gets it",async()=>{
 const pool=openFarm(1),seen:{slot:number;generation:number}[]=[];
 try{
  const first=await pool.infer(where=>{seen.push(where);return{n:1};});
  expect((first as {reply:{value:{n:number}}}).reply.value.n).toBe(1);
  expect(await pool.infer(where=>{seen.push(where);return{n:2,crash:true};})).toBeUndefined();
  const third=await pool.infer(where=>{seen.push(where);return{n:3};});
  expect((third as {reply:{value:{n:number}}}).reply.value.n).toBe(3);
  expect(seen).toEqual([{slot:0,generation:1},{slot:0,generation:1},{slot:0,generation:2}]);
 }finally{await pool.close();}
},60000);

test("a worker past its memory budget is replaced after its job and the answers stay right",async()=>{
 const pool=openFarm(1,{workerMegabytes:1});
 try{
  const first=await pool.infer({n:1}),second=await pool.infer({n:2});
  expect((first as {reply:{pid:number}}).reply.pid).not.toBe((second as {reply:{pid:number}}).reply.pid);
  expect(pool.stats().recycled).toBeGreaterThanOrEqual(2);
  expect((second as {reply:{value:{n:number}}}).reply.value.n).toBe(2);
 }finally{await pool.close();}
},60000);

test("cancelling stops the workers and rejects what waits, closing leaves no worker behind",async()=>{
 const controller=new AbortController(),pool=openFarm(1,{signal:controller.signal});
 const running=pool.infer({n:1,delay:30000}),waiting=pool.infer({n:2,delay:30000});
 await new Promise(done=>setTimeout(done,1500));
 const settled=Promise.all([running,waiting]);
 controller.abort(Error("law cancel"));
 expect(await settled).toEqual([undefined,undefined]);
 expect(await pool.infer({n:3})).toBeUndefined();
 const per=openFarm(1),own=new AbortController(),blocked=per.infer({n:1,delay:30000}),queued=per.infer({n:2},{signal:own.signal});
 own.abort(Error("law job cancel"));
 await expect(queued).rejects.toThrow("law job cancel");
 await per.close();
 expect(await blocked).toBeUndefined();
},60000);

test("the worker protocol messages are closed",()=>{
 const ajv=new Ajv({strict:false}),definitions=moduleSchema.$defs as Record<string,object>;
 const valid=(name:string,value:unknown)=>ajv.validate({$ref:"#/$defs/"+name,$defs:definitions} as never,value);
 expect(valid("Init",{type:"init",service:"file:///a.mjs",sessionFile:"/tmp/session.bin"})).toBe(true);
 expect(valid("Init",{type:"init",service:"http://a.mjs",sessionFile:"/tmp/session.bin"})).toBe(false);
 expect(valid("Init",{type:"init",service:"file:///a.mjs",session:{}})).toBe(false);
 expect(valid("Run",{type:"run",id:3,job:{n:1}})).toBe(true);
 expect(valid("Run",{type:"run",id:-1,job:{}})).toBe(false);
 expect(valid("Failure",{type:"failure",id:1,name:"Error",message:"x",code:null})).toBe(true);
 expect(valid("Failure",{type:"failure",id:1,name:"Error",message:"x",extra:1})).toBe(false);
 expect(valid("Result",{type:"result",id:1,rssMegabytes:12.5,value:{}})).toBe(true);
 expect(valid("Shutdown",{type:"shutdown",extra:true})).toBe(false);
},60000);
