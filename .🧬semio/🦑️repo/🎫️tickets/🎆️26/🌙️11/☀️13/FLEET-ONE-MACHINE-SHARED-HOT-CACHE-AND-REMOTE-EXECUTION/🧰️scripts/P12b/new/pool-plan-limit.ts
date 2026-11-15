/** 🪓️ Contract: a slice worker whose process tree passes the memory a plan may take on this machine is stopped from outside by the pool, because a worker busy in synchronous work cannot stop itself; the typed error names the plan stage the worker last reported and the size, the worker is terminated, a worker under the limit completes, a serving machine and an environment that says `off` have none, and the limit is the booking ceiling of the machine (40 % of 16 GB) unless one is given.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs */
import {expect,test} from "bun:test";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number};
const wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
const harness=(options:{hostMegabytes:number;observed:()=>number;planLimitMegabytes?:number}) => {
 const workers:{emit:(event:string,value:unknown)=>void}[]=[],state={terminated:0,released:0};
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,sampleMilliseconds:10,hostMegabytes:options.hostMegabytes,elasticity,unlearnedMegabytes:2048,
  ...options.planLimitMegabytes===undefined?{}:{planLimitMegabytes:options.planLimitMegabytes},
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>{state.released++;}}),
  observe:async()=>options.observed(),
  createWorker:()=>{const listeners:Record<string,(value:unknown)=>void>={};workers.push({emit:(event,value)=>listeners[event]?.(value)});return{pid:4242,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>{state.terminated++;return 0;}};}});
 const started=async()=>{for(let tries=0;workers.length===0&&tries<200;tries++)await wait(10);return workers[0]!;};
 return{pool,workers,state,started};
};

test("a worker whose tree passes the limit is stopped, the error names the last stage and the size, the worker is terminated and its reservation released",async()=>{
 let megabytes=1000;const{pool,state,started}=harness({hostMegabytes:16384,observed:()=>megabytes}),run=pool.run({root:"/tmp/plan-limit"}).then(()=>undefined,error=>error),worker=await started();
 worker.emit("message",{type:"stage",stage:"inference",managedMegabytes:10,residentMegabytes:3500});worker.emit("message",{type:"stage",stage:"tests",operation:"fields",managedMegabytes:20,residentMegabytes:3600});
 worker.emit("message",{type:"memory",heapMb:210.5,extraMb:5300.2,objectCount:4000000,objectTypes:[{type:"Object",count:3000000}]});
 await wait(60);megabytes=18511;
 const error:any=await run;await pool.close();
 expect(error?.code).toBe("plan-memory");expect(error.stage).toBe("tests");expect(error.operation).toBe("fields");expect(error.currentMegabytes).toBe(18511);expect(error.limitMegabytes).toBe(6553);
 expect(error.sample).toMatchObject({heapMb:210.5,extraMb:5300.2,objectCount:4000000,objectTypes:[{type:"Object",count:3000000}]});
 for(let tries=0;state.released<1&&tries<100;tries++)await wait(10);
 expect(state.terminated).toBeGreaterThanOrEqual(1);expect(state.released).toBe(1);expect(pool.stats().overBudget).toBe(1);
});

test("a stage the worker never reported is named by a question mark; a worker under the limit completes",async()=>{
 let megabytes=7000;const unnamed=harness({hostMegabytes:16384,observed:()=>megabytes}),run=unnamed.pool.run({root:"/tmp/plan-limit"}).then(()=>undefined,error=>error);await unnamed.started();megabytes=9000;
 const error:any=await run;await unnamed.pool.close();expect(error.stage).toBe("?");expect(error.operation).toBe("?");expect(error.limitMegabytes).toBe(6553);
 const fine=harness({hostMegabytes:16384,observed:()=>5000}),done=fine.pool.run({root:"/tmp/plan-limit"}),worker=await fine.started();await wait(60);worker.emit("message",{type:"done",graph:{ok:true}});
 expect(await done).toEqual({graph:{ok:true}});await fine.pool.close();
});

test("a serving machine has no limit, an explicit limit wins, and SEMIO_GRAPH_PLAN_LIMIT_MB=off switches the guard off",async()=>{
 const serving=harness({hostMegabytes:64000,observed:()=>50000}),ran=serving.pool.run({root:"/tmp/plan-limit"}),worker=await serving.started();await wait(80);worker.emit("message",{type:"done",graph:{}});
 expect(await ran).toEqual({graph:{}});await serving.pool.close();expect(serving.state.terminated).toBeGreaterThanOrEqual(0);expect(serving.pool.stats().overBudget).toBe(0);
 const explicit=harness({hostMegabytes:64000,observed:()=>3000,planLimitMegabytes:2500}),failed:any=await explicit.pool.run({root:"/tmp/plan-limit"}).then(()=>undefined,error=>error);await explicit.pool.close();
 expect(failed.code).toBe("plan-memory");expect(failed.limitMegabytes).toBe(2500);
 const before=process.env.SEMIO_GRAPH_PLAN_LIMIT_MB;process.env.SEMIO_GRAPH_PLAN_LIMIT_MB="off";
 try{
  const off=harness({hostMegabytes:16384,observed:()=>18000}),running=off.pool.run({root:"/tmp/plan-limit"}),first=await off.started();await wait(80);first.emit("message",{type:"done",graph:{}});
  expect(await running).toEqual({graph:{}});await off.pool.close();
 }finally{if(before===undefined)delete process.env.SEMIO_GRAPH_PLAN_LIMIT_MB;else process.env.SEMIO_GRAPH_PLAN_LIMIT_MB=before;}
});
