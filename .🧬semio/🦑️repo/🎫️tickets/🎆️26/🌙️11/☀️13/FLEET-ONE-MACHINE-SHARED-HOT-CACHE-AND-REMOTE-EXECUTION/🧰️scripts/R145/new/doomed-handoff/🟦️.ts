/** 🚫️ Contract: a hand-off of too heavy owners to the fleet is a second load of the whole read, so it is started only when the fleet can answer: a refusal that says no machine delivered a record (`producers` 0) ends the read at once with the typed refusal and starts no second load; a refusal without that number, or with machines that delivered, still hands off.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs */
import{expect,test}from"bun:test";
import{createSlicePoolV1}from"../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import{HOST_POLICY_V1}from"../../📨️protocol/🎚️policy/🟨️.mjs";

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number},guard=HOST_POLICY_V1.slicePlanGuard as {reserveMegabytes:number;ceilingBasisPoints:number;settleWaitMilliseconds:number},MiB=1048576;
const wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
type Fake={emit:(event:string,value:unknown)=>void};
const harness=()=>{
 const workers:Fake[]=[],decisions:{key:string}[]=[];
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,sampleMilliseconds:10,heartbeatMilliseconds:30,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
  planGuard:{...guard,settleWaitMilliseconds:20},hostPressure:()=>"normal",report:()=>undefined,
  pressure:()=>({sample:()=>({tripped:false,kind:"memory",value:0,limit:20,seconds:0}),stop:()=>undefined}),
  headroom:()=>({limitBytes:Infinity,currentBytes:100*MiB,availableBytes:8000*MiB}),
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
  observe:async()=>1000,
  createWorker:()=>{const listeners:Record<string,(value:unknown)=>void>={},fake:Fake={emit:(event,value)=>listeners[event]?.(value)};workers.push(fake);
   return{pid:4242,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},send:()=>true,terminate:async()=>0};}});
 const start=(name:string)=>pool.run({root:"/tmp/"+name},{onProgress:(_ms:number,detail:any)=>{if(detail?.decision?.key==="handoff")decisions.push(detail.decision);}});
 const next=async(count:number)=>{for(let tries=0;workers.length<count&&tries<300;tries++)await wait(10);return workers[count-1];};
 return{pool,workers,decisions,start,next};
};
const heavy=(values:Record<string,unknown>)=>({type:"error",message:"graph.owner-too-heavy",stack:"",refusal:{code:"unserved",key:"build.graph.owner-too-heavy",values:{named:"s-dev-composition (858 MB)",limit:3000,...values}}});

test("no machine of the fleet delivered: the refusal ends the read at once, one load, no hand-off",async()=>{
 const{pool,workers,decisions,start,next}=harness(),run=start("doomed").then(()=>undefined,reason=>reason);
 (await next(1))!.emit("message",heavy({producers:0}));
 const error:any=await run;await wait(100);await pool.close();
 expect(error?.key).toBe("build.graph.owner-too-heavy");expect(workers).toHaveLength(1);expect(decisions).toEqual([]);
},15000);

test("machines delivered records, or the refusal does not say: the load is handed off again as before",async()=>{
 const{pool,workers,decisions,start,next}=harness(),run=start("alive");
 (await next(1))!.emit("message",heavy({producers:2}));
 (await next(2))!.emit("message",heavy({}));
 (await next(3))!.emit("message",{type:"done",graph:{ok:1}});
 expect(await run).toEqual({graph:{ok:1}});await pool.close();
 expect(workers).toHaveLength(3);expect(decisions.map(row=>row.key)).toEqual(["handoff","handoff"]);
},15000);
