/** ⏳️ Contract: an Nx run that waits behind the single-flight front for another machine's task is not killed by the 10 minute liveness rule of the Script process before the wait budget is spent, and a stopped Nx run is a classified refusal in both languages, never an internal error.
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {answerClaimRequestV1,createClaimTableV1} from "../../../../🎟️claims/🟦️.ts";
import {nxFlightStallEnvironmentV1,nxFlightTimingV1,startNxFlightV1} from "../../🟦️.ts";
import {scriptProcessStallDecisionV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🐕️watchdog/🟦️.ts";
import {isRepoErrorV1} from "../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {refuseProcessV1} from "../../../../../../../../../../🔨️modules/🏃️process/📖️texts/🟦️.ts";

const NAME="SEMIO_SCRIPT_STALL_MS",WAIT="SEMIO_NX_FLIGHT_WAIT_SECONDS";

test("the stall period outlasts the wait budget of the front",()=>{
 const lifted=nxFlightStallEnvironmentV1({});
 expect(Number(lifted[NAME])).toBeGreaterThan(nxFlightTimingV1({}).waitBudgetMilliseconds);
});

test("every wait budget is outlasted by the stall period, judged by the watchdog rule itself",()=>{
 for(const seconds of [0,60,600,601,1200,1800,7200]){
  const environment={[WAIT]:String(seconds)},budget=seconds*1000,lifted=nxFlightStallEnvironmentV1(environment),period=lifted[NAME]===undefined?600000:Number(lifted[NAME]);
  expect(scriptProcessStallDecisionV1({idleMilliseconds:budget,liveChildren:0,treeIdleMilliseconds:0,waiting:0},period)).toBe("healthy");
 }
});

test("a front that never waits and a period that is long enough change nothing",()=>{
 expect(nxFlightStallEnvironmentV1({[WAIT]:"0"})).toEqual({});
 expect(nxFlightStallEnvironmentV1({[NAME]:"7200000"})).toEqual({});
});

test("a stopped Nx run is a classified refusal that renders in both languages",()=>{
 const stalled=refuseProcessV1("stalled","nx.run-stalled",{status:124,command:"nx run-many --targets=build"}),failed=refuseProcessV1("failed","nx.run-failed",{status:1,command:"nx run-many --targets=build"});
 expect(isRepoErrorV1(stalled)&&stalled.code==="stalled").toBe(true);
 expect(isRepoErrorV1(failed)&&failed.code==="failed").toBe(true);
});

test("the process declares a bounded wait for exactly as long as the front awaits another machine",async()=>{
 const table=createClaimTableV1(),store=new Map<string,string>(),key="15698478355623763392";
 const handler=async(request:Request):Promise<Response>=>{
  const url=new URL(request.url),parts=url.pathname.split("/").slice(1);
  const claimed=await answerClaimRequestV1({request,url,table,defaultInstance:"main",authorize:()=>undefined,readOnly:false,has:async(_namespace,_kind,id)=>store.has(id),readBody:async()=>new Uint8Array(await request.arrayBuffer())});
  if(claimed)return claimed;
  if(parts[1]==="v1"&&parts[2]==="cache"){
   const id=parts[3]!;
   if(request.method==="GET"){const value=store.get(id);return value===undefined?new Response("Not found\n",{status:404}):new Response(value,{status:200});}
   if(request.method==="PUT"){store.set(id,await request.text());table.stored("main","nx",id);return new Response("Stored",{status:200});}
  }
  return new Response("Not found\n",{status:404});
 };
 const transport=((input:Request|string|URL,init?:RequestInit)=>handler(new Request(input as string,init))) as typeof fetch;
 let open=0,peak=0;
 const timing={claimTtlMilliseconds:30_000,heartbeatMilliseconds:30,awaitSliceMilliseconds:100,waitBudgetMilliseconds:20_000,claimTimeoutMilliseconds:1000,upstreamTimeoutMilliseconds:5000};
 const holder=await startNxFlightV1({upstream:"http://fleet.test",instance:"main",token:"t",owner:"a:1:aa",fetch:transport,timing});
 const waiter=await startNxFlightV1({upstream:"http://fleet.test",instance:"main",token:"t",owner:"b:1:bb",fetch:transport,timing,waiting:()=>{open++;peak=Math.max(peak,open);return()=>{open--;};}});
 try{
  const ask=(flight:typeof holder)=>fetch(`${flight.url}/v1/cache/${key}`,{headers:{authorization:`Bearer ${flight.token}`}});
  expect((await ask(holder)).status).toBe(404);
  const pending=ask(waiter);
  await new Promise(resolve=>setTimeout(resolve,400));
  expect(open).toBe(1);
  await fetch(`${holder.url}/v1/cache/${key}`,{method:"PUT",headers:{authorization:`Bearer ${holder.token}`,"content-type":"application/octet-stream"},body:"result"});
  const answer=await pending;
  expect(await answer.text()).toBe("result");
  expect(peak).toBe(1);
  expect(open).toBe(0);
 }finally{await holder.stop();await waiter.stop();}
},30_000);
