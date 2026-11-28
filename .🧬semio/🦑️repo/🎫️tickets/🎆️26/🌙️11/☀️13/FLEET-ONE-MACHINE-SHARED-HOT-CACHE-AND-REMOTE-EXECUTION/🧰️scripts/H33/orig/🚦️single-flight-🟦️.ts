import {randomBytes} from "node:crypto";
import {hostname} from "node:os";
import {resolveRemoteCacheContractV1} from "../../🟦️.ts";
import {remoteCacheFetchV1,resolveRemoteCacheV1} from "../../🛰️fleet/🟦️.ts";
import {runtimeServeV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌍️serve/🟦️.ts";
import {SLOT_HEADER_V1} from "../../../🪢️slots/🟦️.ts";
import {registerNxSlotsV1,type NxTaskRowV1} from "../../🪢️nx-slots/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🚦️ The authored single-flight policy: lease and heartbeat, wait budget, timeouts, circuit breaker and the environment names.
 * @see ./🧬️schema/🔣️.json */
export const NX_FLIGHT_POLICY_V1=schema.definitions.Policy.const;
const policy=NX_FLIGHT_POLICY_V1,HASH=/^[a-z0-9_-]{1,128}$/u,SLOT=/^[0-9a-f]{64}$/u,INSTANCE=/^[a-z0-9][a-z0-9._-]{0,63}$/u;

/** ⏱️ The tunable durations in milliseconds and the breaker threshold of one front. */
export type NxFlightTimingV1=Readonly<{claimTtlMilliseconds:number;expectedMilliseconds:number;heartbeatMilliseconds:number;awaitSliceMilliseconds:number;waitBudgetMilliseconds:number;maximumHoldMilliseconds:number;claimTimeoutMilliseconds:number;upstreamTimeoutMilliseconds:number;requestTimeoutMilliseconds:number;probeTimeoutMilliseconds:number;failureThreshold:number;breakerMilliseconds:number}>;
/** 📊️ What one front did.
 * @see ./🧬️schema/🔣️.json */
export type NxFlightCountersV1=Readonly<{hits:number;misses:number;claimed:number;waited:number;served:number;taken:number;duplicated:number;unclaimed:number;puts:number;released:number;guarded:number;dropped:number}>;
/** 📣️ A noteworthy step of a front; the caller turns it into a bilingual line with `nxFlightTextV1`. */
export type NxFlightEventV1=Readonly<{kind:"waiting"|"served"|"duplicated"|"unavailable";key?:string;holder?:string;seconds?:number;endpoint?:string}>;
/** ⚙️ How to start a front: the upstream cache origin, its instance, the real token and optional overrides. */
export type NxFlightOptionsV1=Readonly<{upstream:string;instance:string;token?:string|undefined;owner?:string;timing?:Partial<NxFlightTimingV1>;fetch?:typeof fetch;report?:(event:NxFlightEventV1)=>void;port?:number}>;
/** 🔌️ One running front: the origin Nx talks to, the per-run token it must present and the lifecycle. */
export interface NxFlightV1{readonly origin:string;readonly url:string;readonly token:string;readonly owner:string;counters():NxFlightCountersV1;registerSlots(rows:readonly Pick<NxTaskRowV1,"hash"|"project"|"target"|"configuration">[]):Promise<Readonly<{claimed:number;missing:number}>|undefined>;environment():Record<string,string>;stop():Promise<void>}

/** 🗣️ Fills one text of the catalogue in one language. */
export function nxFlightTextV1(language:"en"|"de",event:NxFlightEventV1):string{
 const messages=schema.definitions.Messages.const as Record<string,Record<"en"|"de",string>>;
 return messages[event.kind]![language].replace(/\{(\w+)\}/gu,(_,name:string)=>String((event as Record<string,unknown>)[name]??""));
}

/** ⏲️ Reads the wait budget a person may set in whole seconds (0 never waits) and keeps every other duration of the policy. */
export function nxFlightTimingV1(environment:Readonly<Record<string,string|undefined>>={}):NxFlightTimingV1{
 const {version:_version,environment:_names,finishPath:_path,...timing}=policy,asked=environment[policy.environment.wait],seconds=asked===undefined||asked===""?undefined:Number(asked);
 return {...timing,...(seconds!==undefined&&Number.isFinite(seconds)&&seconds>=0?{waitBudgetMilliseconds:Math.floor(seconds*1000)}:{})};
}

const empty=():Response=>new Response("Not found\n",{status:404,headers:{"content-type":"text/plain"}});
const json=(status:number,body:unknown)=>Response.json(body,{status});
const ownerName=():string=>`${hostname().replace(/[^A-Za-z0-9._-]/gu,"-").slice(0,60)}:${process.pid}:${randomBytes(4).toString("hex")}`;
const pause=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));

type Outcome={state:"granted"}|{state:"held";by:string}|{state:"stored"}|{state:"unavailable"};

/** 🚦️ Starts the loopback front of the Nx remote cache. Nx reads `url` and `token` as its self-hosted cache; a GET that misses upstream claims the hash (`POST /claim/nx/<hash>`), a held hash waits for the holder's upload in long-poll slices until the wait budget is spent, a lapsed lease is taken over, and a front whose claims fail (unreachable, refused, full) lets the miss through. A PUT streams through (with the task's slot header when the executor announced it) and releases the claim; `POST /flight/finish` from the executor releases it at the end of a task, whatever its outcome, so a failed task never blocks the fleet. Claims are heartbeated until released, until the maximum hold or until the front stops. */
export async function startNxFlightV1(options:NxFlightOptionsV1):Promise<NxFlightV1>{
 if(!INSTANCE.test(options.instance))throw Error("Valid cache instance required / Gültige Cache-Instanz erforderlich");
 const timing={...nxFlightTimingV1(),...options.timing},upstream=options.upstream.replace(/\/+$/u,""),base=`${upstream}/${options.instance}`,transport=options.fetch??fetch,owner=options.owner??ownerName(),local=randomBytes(24).toString("hex"),report=options.report??(()=>{});
 const owned=new Map<string,{timer:ReturnType<typeof setInterval>;since:number}>(),slots=new Map<string,string>(),blocked=new Set<string>(),count={hits:0,misses:0,claimed:0,waited:0,served:0,taken:0,duplicated:0,unclaimed:0,puts:0,released:0,guarded:0,dropped:0};
 let failures=0,openUntil=0,tripped=false,recovering:Promise<boolean>|undefined,refused=false,announced=false,stopping=false;
 const authorization=(token:string|undefined):Record<string,string>=>token===undefined||token===""?{}:{authorization:`Bearer ${token}`};
 const note=()=>{if(!announced){announced=true;report({kind:"unavailable",endpoint:upstream});}};
 const trip=()=>{openUntil=Date.now()+timing.breakerMilliseconds;tripped=true;failures=0;note();};
 const failed=()=>{if(++failures>=timing.failureThreshold)trip();};
 const recover=():Promise<boolean>=>recovering??=(async()=>{
  try{const answer=await transport(`${upstream}/healthz`,{signal:AbortSignal.timeout(timing.probeTimeoutMilliseconds)});await answer.body?.cancel();if(answer.status===200){tripped=false;openUntil=0;failures=0;announced=false;return true;}}catch{}
  openUntil=Date.now()+timing.breakerMilliseconds;return false;
 })().finally(()=>{recovering=undefined;});
 const available=async():Promise<boolean>=>{
  if(stopping||Date.now()<openUntil)return false;
  return tripped?recover():true;
 };
 const guarded=async(url:string,init:RequestInit,milliseconds:number,signal?:AbortSignal):Promise<Response>=>{
  const first=new AbortController(),timer=setTimeout(()=>first.abort(),milliseconds),overall=AbortSignal.timeout(timing.upstreamTimeoutMilliseconds);
  try{return await transport(url,{...init,signal:AbortSignal.any([first.signal,overall,...(signal?[signal]:[])])});}
  finally{clearTimeout(timer);}
 };
 const settled=(response:Response):Response=>{if(response.status>=500&&response.status!==507)trip();return response;};
 const call=async(method:string,path:string,body:unknown,milliseconds:number,signal?:AbortSignal):Promise<Response|undefined>=>{
  try{
   const deadline=AbortSignal.timeout(milliseconds);
   return await transport(`${base}/claim/nx/${path}`,{method,headers:{...authorization(options.token),...(body===undefined?{}:{"content-type":"application/json"})},...(body===undefined?{}:{body:JSON.stringify(body)}),signal:signal?AbortSignal.any([signal,deadline]):deadline});
  }catch{return undefined;}
 };
 const usable=()=>!refused&&!stopping&&Date.now()>=openUntil&&!tripped;
 const answer=async(response:Response|undefined):Promise<Record<string,unknown>|undefined>=>{
  if(!response)return undefined;
  try{return await response.json() as Record<string,unknown>;}catch{return undefined;}
 };
 const claim=async(key:string):Promise<Outcome>=>{
  if(refused||!await available())return {state:"unavailable"};
  const response=await call("POST",key,{owner,ttlMilliseconds:timing.claimTtlMilliseconds,expectedMilliseconds:timing.expectedMilliseconds},timing.claimTimeoutMilliseconds);
  if(!response){failed();return {state:"unavailable"};}
  if([401,403,404,405].includes(response.status)){await response.body?.cancel();refused=true;note();return {state:"unavailable"};}
  const body=await answer(response);
  if(response.status===200&&body?.state==="granted"){failures=0;return {state:"granted"};}
  if(response.status===200&&body?.state==="stored"){failures=0;return {state:"stored"};}
  if(response.status===409&&body?.state==="held"){failures=0;return {state:"held",by:String(body.by??"")};}
  if(response.status===503&&body?.state==="full")return {state:"unavailable"};
  failed();return {state:"unavailable"};
 };
 const release=async(key:string):Promise<void>=>{
  const row=owned.get(key);if(!row)return;
  clearInterval(row.timer);owned.delete(key);count.released++;
  const response=await call("POST",`${key}/release`,{owner},timing.claimTimeoutMilliseconds);await response?.body?.cancel();
 };
 const hold=(key:string)=>{
  const since=Date.now(),timer=setInterval(()=>{
   if(Date.now()-since>=timing.maximumHoldMilliseconds){clearInterval(timer);owned.delete(key);return;}
   void call("POST",`${key}/renew`,{owner,ttlMilliseconds:timing.claimTtlMilliseconds},timing.claimTimeoutMilliseconds).then(response=>response?.body?.cancel());
  },timing.heartbeatMilliseconds);
  (timer as {unref?:()=>void}).unref?.();
  const previous=owned.get(key);if(previous)clearInterval(previous.timer);
  owned.set(key,{timer,since});
 };
 const forwardGet=async(key:string,signal?:AbortSignal):Promise<Response>=>{
  if(!await available())throw Error("Fleet cache down");
  try{return settled(await guarded(`${base}/v1/cache/${key}`,{headers:authorization(options.token)},timing.requestTimeoutMilliseconds,signal));}
  catch(error){if(!signal?.aborted&&!stopping)trip();throw error;}
 };
 const wait=async(key:string,signal:AbortSignal):Promise<Response|undefined>=>{
  const started=Date.now();let holder="",told=false;
  for(;;){
   if(signal.aborted||stopping)return undefined;
   const outcome=await claim(key);
   if(outcome.state==="unavailable"){count.unclaimed++;return undefined;}
   if(outcome.state==="granted"){count.claimed++;if(told)count.taken++;hold(key);return undefined;}
   if(outcome.state==="stored"){
    const again=await forwardGet(key,signal).catch(()=>undefined);
    if(again?.status===200){count.served++;report({kind:"served",key,holder});return again;}
    await again?.body?.cancel();return undefined;
   }
   holder=outcome.by;
   const elapsed=Date.now()-started;
   if(elapsed>=timing.waitBudgetMilliseconds){count.duplicated++;report({kind:"duplicated",key,holder,seconds:Math.round(elapsed/1000)});return undefined;}
   if(!told){told=true;count.waited++;report({kind:"waiting",key,holder});}
   const slice=Math.min(timing.awaitSliceMilliseconds,timing.waitBudgetMilliseconds-elapsed),polled=await call("GET",`${key}/await?timeoutMilliseconds=${Math.max(0,Math.floor(slice))}`,undefined,slice+timing.claimTimeoutMilliseconds,signal),state=(await answer(polled))?.state;
   if(state==="stored"){
    const again=await forwardGet(key,signal).catch(()=>undefined);
    if(again?.status===200){count.served++;report({kind:"served",key,holder});return again;}
    await again?.body?.cancel();
   }else if(!polled){failed();if(!usable()){count.unclaimed++;return undefined;}await pause(Math.min(1000,timing.claimTimeoutMilliseconds));}
  }
 };
 const get=async(key:string,request:Request):Promise<Response>=>{
  let first:Response;
  try{first=await forwardGet(key,request.signal);}catch{count.misses++;return empty();}
  if(first.status===200){count.hits++;await release(key);return first;}
  await first.body?.cancel();count.misses++;
  if(first.status!==404){count.dropped++;return empty();}
  const served=await wait(key,request.signal);
  return served??empty();
 };
 const put=async(key:string,request:Request):Promise<Response>=>{
  if(blocked.delete(key)){await request.body?.cancel();count.guarded++;await release(key);return new Response("Stored",{status:200,headers:{"content-type":"text/plain"}});}
  const headers:Record<string,string>={...authorization(options.token)},slot=slots.get(key);
  for(const name of["content-type","content-length"]){const value=request.headers.get(name);if(value!==null)headers[name]=value;}
  if(slot!==undefined)headers[SLOT_HEADER_V1]=slot;
  let response:Response;
  const bytes=new Uint8Array(await request.arrayBuffer());
  try{
   if(!await available())throw Error("Fleet cache down");
   response=settled(await guarded(`${base}/v1/cache/${key}`,{method:"PUT",headers,body:bytes} as RequestInit,timing.requestTimeoutMilliseconds+Math.ceil(bytes.length/2097152*1000)));
  }
  catch(error){if(!stopping&&(error as Error).message!=="Fleet cache down")trip();count.dropped++;await release(key);return new Response("Stored",{status:200,headers:{"content-type":"text/plain"}});}
  if(response.status>=500){await response.body?.cancel();count.dropped++;await release(key);return new Response("Stored",{status:200,headers:{"content-type":"text/plain"}});}
  count.puts++;slots.delete(key);await release(key);
  return new Response(response.body,{status:response.status,headers:{"content-type":response.headers.get("content-type")??"text/plain"}});
 };
 const blob=async(digest:string,request:Request):Promise<Response>=>{
  if(!["GET","HEAD","PUT"].includes(request.method))return new Response("Method not allowed\n",{status:405,headers:{allow:"GET, HEAD, PUT"}});
  const headers:Record<string,string>={...authorization(options.token)};
  for(const name of["content-type","content-length"]){const value=request.headers.get(name);if(value!==null)headers[name]=value;}
  try{
   if(!await available())throw Error("Fleet cache down");
   const init={method:request.method,headers,...(request.method==="PUT"?{body:request.body,duplex:"half"}:{})} as RequestInit,response=request.method==="PUT"?await transport(`${base}/cas/${digest}`,{...init,signal:AbortSignal.timeout(timing.upstreamTimeoutMilliseconds*5)}):await guarded(`${base}/cas/${digest}`,init,timing.requestTimeoutMilliseconds);
   if(response.status>=500&&response.status!==507)trip();
   if(response.status>=500){await response.body?.cancel();count.dropped++;return request.method==="PUT"?new Response("Stored",{status:200,headers:{"content-type":"text/plain"}}):empty();}
   return new Response(request.method==="HEAD"?null:response.body,{status:response.status,headers:{"content-type":response.headers.get("content-type")??"application/octet-stream",...(response.headers.get("content-length")?{"content-length":response.headers.get("content-length")!}:{})}});
  }catch(error){if(!stopping&&(error as Error).message!=="Fleet cache down")trip();count.dropped++;return request.method==="PUT"?new Response("Stored",{status:200,headers:{"content-type":"text/plain"}}):empty();}
 };
 const finish=async(request:Request):Promise<Response>=>{
  let body:Record<string,unknown>;
  try{body=await request.json() as Record<string,unknown>;}catch{return json(400,{error:"Body is not JSON"});}
  if(typeof body!=="object"||body===null||typeof body.hash!=="string"||!HASH.test(body.hash)||(body.outcome!=="succeeded"&&body.outcome!=="failed"))return json(400,{error:"Finish request invalid"});
  if(typeof body.slot==="string"&&SLOT.test(body.slot)&&body.outcome==="succeeded")slots.set(body.hash,body.slot);
  if(body.guarded===true)blocked.add(body.hash);
  const had=owned.has(body.hash);await release(body.hash);
  return json(200,{state:had?"released":"absent"});
 };
 const fetchHandler=async(request:Request):Promise<Response>=>{
  const url=new URL(request.url),given=request.headers.get("authorization");
  if(given!==`Bearer ${local}`)return new Response("Unauthorized\n",{status:401,headers:{"content-type":"text/plain"}});
  if(url.pathname===policy.finishPath){return request.method==="POST"?finish(request):new Response("Method not allowed\n",{status:405});}
  const parts=url.pathname.split("/").slice(1);
  if(parts.length===3&&parts[0]===options.instance&&parts[1]==="cas"&&SLOT.test(parts[2]!))return blob(parts[2]!,request);
  if(parts.length!==4||parts[0]!==options.instance||parts[1]!=="v1"||parts[2]!=="cache"||!HASH.test(parts[3]!))return empty();
  const key=parts[3]!;
  if(request.method==="GET")return get(key,request);
  if(request.method==="PUT")return put(key,request);
  return new Response("Method not allowed\n",{status:405,headers:{allow:"GET, PUT"}});
 };
 const server=await runtimeServeV1({hostname:"127.0.0.1",port:options.port??0,idleTimeoutSeconds:0,maximumRequestBodyBytes:2**30,fetch:fetchHandler});
 const origin=`http://127.0.0.1:${server.port}`;
 let closing:Promise<void>|undefined;
 return Object.freeze<NxFlightV1>({
  origin,url:`${origin}/${options.instance}`,token:local,owner,
  counters:()=>({...count}),
  registerSlots:rows=>registerNxSlotsV1({endpoint:upstream,instance:options.instance,token:options.token,rows,fetch:transport}),
  environment:()=>({NX_SELF_HOSTED_REMOTE_CACHE_SERVER:`${origin}/${options.instance}`,NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN:local,[policy.environment.url]:origin}),
  stop:()=>closing??=(async()=>{stopping=true;const keys=[...owned.keys()];await Promise.all(keys.map(key=>release(key)));await server.stop(true);})(),
 });
}

/** 🧭️ Routes the Nx cache environment that `nxRemoteCacheEnvironmentV1` resolved through a front: the real server and token move behind the loopback, Nx gets the front's address and per-run token, and the executors get the front's origin. Leaves the environment unchanged (`undefined`) when the switch is off, Nx is not routed to a remote cache or the address is not a cache origin with an instance. */
export async function routeNxThroughFlightV1(environment:Readonly<Record<string,string|undefined>>,options:Readonly<{report?:(event:NxFlightEventV1)=>void;fetch?:typeof fetch;timing?:Partial<NxFlightTimingV1>}>={}):Promise<NxFlightV1|undefined>{
 if(environment[policy.environment.switch]==="off")return undefined;
 const server=environment.NX_SELF_HOSTED_REMOTE_CACHE_SERVER,token=environment.NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN;
 if(server===undefined||server==="")return undefined;
 let parsed:URL;try{parsed=new URL(server);}catch{return undefined;}
 const segments=parsed.pathname.split("/").filter(Boolean);
 if(segments.length!==1||parsed.search!==""||parsed.hash!==""||!INSTANCE.test(segments[0]!)||parsed.username!==""||parsed.password!=="")return undefined;
 const contract=resolveRemoteCacheV1(environment)?.contract??resolveRemoteCacheContractV1(environment);
 return startNxFlightV1({upstream:parsed.origin,instance:segments[0]!,token,timing:{...nxFlightTimingV1(environment),...(contract===undefined?{}:{requestTimeoutMilliseconds:contract.timeoutSeconds*1000}),...options.timing},...(options.report?{report:options.report}:{}),fetch:options.fetch??remoteCacheFetchV1(environment)});
}

/** 🏁 Tells the front of this Nx run that the task `hash` ended: releases its claim at once and hands over its slot so the upload carries it. Called by the executor of the task; it needs no failure handling because it never throws, never waits longer than two seconds and does nothing without a front. */
export async function announceNxTaskFinishV1(environment:Readonly<Record<string,string|undefined>>,finish:Readonly<{hash:string|undefined;project:string;target:string;configuration?:string|undefined;slot?:string|undefined;guarded?:boolean;succeeded:boolean}>):Promise<void>{
 const origin=environment[policy.environment.url],token=environment.NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN;
 if(origin===undefined||origin===""||token===undefined||finish.hash===undefined||!HASH.test(finish.hash))return;
 try{
  const response=await fetch(`${origin}${policy.finishPath}`,{method:"POST",headers:{authorization:`Bearer ${token}`,"content-type":"application/json"},body:JSON.stringify({hash:finish.hash,project:finish.project,target:finish.target,...(finish.configuration?{configuration:finish.configuration}:{}),...(finish.slot?{slot:finish.slot}:{}),...(finish.guarded?{guarded:true}:{}),outcome:finish.succeeded?"succeeded":"failed"}),signal:AbortSignal.timeout(2000)});
  await response.body?.cancel();
 }catch{}
}
