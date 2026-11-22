import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {createGraphFleetClientV1} from "../../🟦️.ts";
import {GRAPH_STATE_POLICY_V1 as policy} from "../../../../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

/** 🚦️ Contract: one slow answer never closes the fleet tier. A lookup whose timeout fired late (the event loop of the reading process was stalled, so the answer may have arrived unread) is asked once more with the confirm timeout before the breaker marker is written; a silent server (the timeout fired on time) and a refused connection still mark at once, so a dead fleet costs no more than before.
 * @see ../../📡️client/🟦️.ts */
const scratch:string[]=[];afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const silent=(signal:AbortSignal):Promise<Response>=>new Promise((_,reject)=>{signal.addEventListener("abort",()=>reject(signal.reason),{once:true});});
const blocked=(signal:AbortSignal):Promise<Response>=>{Bun.sleepSync(policy.timeouts.lookupMilliseconds+policy.breaker.lateMilliseconds+400);return silent(signal);};

test("a lookup whose timeout fired late is confirmed once before the breaker is marked, and equals an independent Python implementation",async()=>{
 const python=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});
 expect(python.status,python.stderr).toBe(0);
 const expected=JSON.parse(python.stdout) as {name:string;requests:number;result:"bytes"|"miss"|"down";marked:boolean}[];
 expect(expected.length).toBe(fixture.cases.length);
 expect(policy.breaker.confirmMilliseconds).toBeGreaterThanOrEqual(policy.timeouts.lookupMilliseconds);expect(policy.breaker.lateMilliseconds).toBeGreaterThan(0);
 for(const[index,row]of fixture.cases.entries()){
  const directory=mkdtempSync(join(tmpdir(),"confirm-")),queue=[...row.outcomes];scratch.push(directory);
  let requests=0;
  const client=createGraphFleetClientV1({endpoint:"http://graph.invalid",instance:"test",token:undefined,upload:false,stateDirectory:directory},{fetch:((_url:string,init:{signal:AbortSignal})=>{
   requests++;
   const outcome=queue.shift()!;
   if(outcome==="timeout")return silent(init.signal);
   if(outcome==="stalled")return blocked(init.signal);
   if(outcome==="network")return Promise.reject(new TypeError("fetch failed"));
   return Promise.resolve(outcome==="miss"?new Response(null,{status:404}):new Response(new Uint8Array([1,2,3]),{status:200}));
  }) as unknown as typeof fetch});
  const answer=await client.getResult("0".repeat(64)),result=answer==="miss"?"miss":typeof answer==="string"?"down":"bytes";
  expect({name:row.name,requests,result,marked:existsSync(join(directory,"down"))},row.name).toEqual(expected[index]!);
  expect(client.usable(),row.name).toBe(!expected[index]!.marked);
 }
},60000);
