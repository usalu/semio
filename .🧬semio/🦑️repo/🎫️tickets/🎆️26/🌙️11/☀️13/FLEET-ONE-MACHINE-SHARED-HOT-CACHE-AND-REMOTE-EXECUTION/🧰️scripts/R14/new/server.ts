import type{GraphFleetClientV1}from"../../🛰️exchange/🟦️.ts";
import{claimWantedV1,publishServedV1,wantedClaimKeyV1,wantedTextV1,type WantedHeadroomV1,type WantedPolicyV1,type WantedRequestV1,wantedPolicyV1}from"../🟦️.ts";
import{clearSteeringV1,readPendingV1,writePriorityV1}from"../../../🕸️graph/🕰️generations/🧭️steering/🟨️.mjs";
import{parseGraphWaveRowV1,type GraphWaveRowV1}from"../../../🕸️graph/📈️wave-row/🟨️.mjs";
import{estimateOwnerCostV1,estimateRemainingMillisecondsV1,orderByCostV1,type OwnerCostTableV1}from"../💰️cost/🟦️.ts";
import{publishOwnerCostsV1}from"../💰️cost/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{platformClassV1}from"../../🌍️platform-class/🟦️.ts";

/** 🛠️ The numbers of the serving machine: round length, how long the worktree must be quiet before a background refresh (with a maximum age that overrides it), the stall limit and the maximum of one graph read, the pauses of a background refresh, the free memory a refresh needs and the physical memory a machine must have to serve at all. See `../🧬️schema/🔣️.json`. */
export const wantedServerPolicyV1=Object.freeze({...schema.definitions.Server.const});
export type WantedServerPolicyV1=typeof wantedServerPolicyV1;
export type WantedInferV1=Readonly<{ok:boolean;status:number;seconds:number;generation:number|null;detail:string}>;

/** 🩺️ The verdict over one background graph read: it goes on while it makes progress and ends only when it does not. */
export type GraphReadWatchV1=Readonly<{feed(text:string):void;verdict():"go"|"stalled"|"too-long";generation():number|null;silentMilliseconds():number}>;

/** 🩺️ Watches the stderr of one graph read of the server: a wave is progress, told by a generation number higher than any seen (the line of a published generation, English or German, however the chunks are cut) or by a line that says the read pauses for others; the read is stopped when no progress was made for `inferStallMilliseconds` (published waves are durable, the next read starts from them, so a stopped read loses only the wave in flight) or when it has run `inferMaximumMilliseconds`. A cold refresh of an hour or more is never cut while its waves keep coming. */
export function createGraphReadWatchV1(policy:WantedServerPolicyV1=wantedServerPolicyV1,now:()=>number=Date.now,onWave?:(row:GraphWaveRowV1)=>void):GraphReadWatchV1{
 const started=now();let progressAt=started,newest=-1,pending="";
 return Object.freeze({
  feed(text:string):void{
   pending+=text;const lines=pending.split("\n");pending=lines.pop()!.slice(-4096);
   for(const line of lines){
    const wave=parseGraphWaveRowV1(line);
    if(wave){progressAt=now();if(wave.generation>newest)newest=wave.generation;try{onWave?.(wave);}catch{}continue;}
    if(/(?:paused after|pausiert nach) /iu.test(line)){progressAt=now();continue;}
    const found=/generation (\d+)/iu.exec(line);
    if(found&&Number(found[1])>newest){newest=Number(found[1]);progressAt=now();}
   }
  },
  verdict():"go"|"stalled"|"too-long"{const at=now();return at-started>=policy.inferMaximumMilliseconds?"too-long":at-progressAt>=policy.inferStallMilliseconds?"stalled":"go";},
  generation:()=>newest<0?null:newest,
  silentMilliseconds:()=>now()-progressAt,
 });
}

/** 🗨️ Words why the server stopped a graph read, in the language of the machine. */
export function graphReadEndTextV1(kind:"stalled"|"too-long",language:"en"|"de",policy:WantedServerPolicyV1=wantedServerPolicyV1):string{
 return wantedTextV1(kind==="stalled"?"stalled":"tooLong",language,{minutes:Math.round(policy.inferStallMilliseconds/60000),hours:Math.round(policy.inferMaximumMilliseconds/3600000)});
}

/** 🧘️ What the capacity gate shows a background refresh: the pressure level and the requests that wait. */
export type GraphYieldViewV1=Readonly<{pressure:string;waiters:readonly Readonly<{request:Readonly<{id:string;elastic?:boolean;quiet?:boolean}>}>[]}>;

const pause=(milliseconds:number,signal?:AbortSignal):Promise<void>=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal!.reason);},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

/** 🧘️ Lets a background refresh give way between two waves: while another command of this machine waits for memory (any request but small forecast grants, the workers of a build and its own) or the pressure is raised, the refresh pauses, tells so every `yieldNoticeMilliseconds` and looks again every `yieldPollMilliseconds`; after `yieldMaximumMilliseconds` it goes on with one wave, so a machine that is busy for good still advances. A gate that cannot be read never holds the refresh. Answers the milliseconds paused. */
export async function yieldToInteractiveV1(input:Readonly<{view():Promise<GraphYieldViewV1>;own:string|undefined;generation:number;language:"en"|"de";say(line:string):void;signal:AbortSignal|undefined;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;policy?:WantedServerPolicyV1}>):Promise<number>{
 const now=input.now??Date.now,policy=input.policy??wantedServerPolicyV1,sleep=input.sleep??pause,started=now();let noticed=-Infinity;
 for(;;){
  if(input.signal?.aborted)return now()-started;
  let waiting:number;
  try{
   const view=await input.view(),others=view.waiters.filter(row=>row.request.elastic!==true&&row.request.quiet!==true&&row.request.id!==input.own).length;
   waiting=others>0?others:view.pressure==="normal"?0:1;
  }catch{return now()-started;}
  if(waiting===0||now()-started>=policy.yieldMaximumMilliseconds)return now()-started;
  if(now()-noticed>=policy.yieldNoticeMilliseconds){noticed=now();input.say(wantedTextV1("paused",input.language,{generation:input.generation,waiters:waiting,seconds:Math.round((now()-started)/1000)}));}
  try{await sleep(policy.yieldPollMilliseconds,input.signal);}catch{return now()-started;}
 }
}

/** 🧘️ The yield of a background refresh bound to the capacity gate of this machine: called by the graph host after every published wave of a read the server started (`SEMIO_GRAPH_BACKGROUND=1`). */
export async function yieldBackgroundGraphV1(input:Readonly<{generation:number;signal:AbortSignal|undefined;language:"en"|"de";environment?:NodeJS.ProcessEnv}>):Promise<number>{
 const environment=input.environment??process.env,{readRemoteExecutionHostViewV1}=await import("../../../🌐️remote-execution/🌡️load/🟦️.ts");
 return yieldToInteractiveV1({view:async()=>{const view=await readRemoteExecutionHostViewV1(environment);return{pressure:view.host.pressure,waiters:view.waiters};},own:environment.SEMIO_HOST_CAPACITY_GRANT,generation:input.generation,language:input.language,say:line=>{process.stderr.write(line+"\n");},signal:input.signal});
}

/** 🔁️ The long-lived serving loop of a machine with headroom. It serves the requests of every machine of its platform class (a slot is per requester, the request names the platform) and counts the others. A machine that may not serve (`mayServe` answers false: it could never run the read) claims no request, so the request stays for a machine that can. It never blocks on a graph read: every round (seconds) it takes over the live requests of the other machines, tells the running read which projects to infer first (the priority file of `steerDirectory`), tells each requester which of its projects are fresh already (the read writes which asked-for projects are still dirty; every other asked-for project is fresh in this machine's tree) and keeps a sign of life on the answer, so a request is seen within one round, answered at once from what exists, and answered again as waves publish. Every answer carries the progress record of its request (how many projects, when the work last advanced: a read started or a wave published, and the time left estimated from what this read has finished or else from the measured cost table), so the requester waits for as long as it advances and not for a fixed time. A request is idempotent: the requester renews it (same creation time, later expiry, fewer projects) and the server keeps its claim, its running read and its position; a request asked again as a new one joins the held one (what is fresh stays fresh, nothing is inferred twice); the claims of the held requests are renewed so no other machine serves them meanwhile; the projects are ordered by how many held requests ask for them, so a project two workers wait for is inferred once and first. When no read runs and a request is held it starts one with the priority (headroom and pressure permitting; a failing read answers nobody and is held off for the quiet time); a read that ends well answers every held request completely with its generation. When no request is held it refreshes the fleet generation in the background after the worktree went quiet for `refreshQuietMilliseconds` (or the generation is older than `refreshIntervalMilliseconds`: edits arrive constantly, so a trailing debounce with a maximum age), only while its headroom allows it and no other machine holds the refresh slot of the interval (`mayRefresh`). The loop never throws except on cancellation. */
export async function runWantedServerV1(input:Readonly<{client:GraphFleetClientV1;owner:string;platform:string;language:"en"|"de";steerDirectory:string;requesters():Promise<readonly string[]>;headroom():Promise<WantedHeadroomV1>;mayServe?():Promise<boolean>;infer(reason:"request"|"refresh",signal:AbortSignal|undefined):Promise<WantedInferV1>;quietMilliseconds():Promise<number>;mayRefresh?():Promise<boolean>;costs?():OwnerCostTableV1|undefined;say(line:string):void;signal:AbortSignal;sleep(milliseconds:number,signal:AbortSignal):Promise<void>;now?:()=>number;rounds?:number;policy?:WantedPolicyV1;server?:WantedServerPolicyV1}>):Promise<void>{
 const now=input.now??Date.now,policy=input.policy??wantedPolicyV1,server=input.server??wantedServerPolicyV1,{language}=input;
 type Held={request:WantedRequestV1;list:readonly string[];told:boolean;toldAt:number;toldProgressAt:number;heldAt:number;claimedAt:number};
 type Running={reason:"request"|"refresh";startedAt:number;controller:AbortController;finished:boolean;done:Promise<void>};
 const held=new Map<string,Held>(),told=new Set<string>();
 let refreshedAt=-Infinity,progressAt=-Infinity,dirtyCount=-1,round=0,holdUntil=-Infinity,running:Running|undefined,priorityKey="",priorityAt=0,tick=0,seen=0,costsKey="",costsAt=-Infinity;
 const foreignNow=new Map<string,string>(),toldForeign=new Set<string>(),noteForeign=(request:WantedRequestV1)=>{foreignNow.set(request.requester,request.platform);const key=wantedClaimKeyV1(request);if(toldForeign.has(key))return;toldForeign.add(key);input.say(wantedTextV1("foreign",language,{count:request.projects.length,requester:request.requester,platform:request.platform,class:platformClassV1(request.platform),own:platformClassV1(input.platform)}));};
 const failed=(result:WantedInferV1)=>input.say(wantedTextV1("failed",language,{status:result.status,seconds:Math.round(result.seconds),detail:result.detail}));
 const remaining=(entry:Held,complete:boolean):number|null=>{
  if(complete)return 0;
  const done=new Set(entry.list),left=entry.request.projects.filter(name=>!done.has(name));
  return estimateRemainingMillisecondsV1({table:input.costs?.(),remaining:left,parallel:policy.waveProjects,...entry.list.length>0&&now()>entry.heldAt?{observed:{done:entry.list.length,milliseconds:now()-entry.heldAt}}:{}});
 };
 const answer=async(entry:Held,projects:readonly string[],generation:number|null)=>{
  entry.list=projects;entry.told=true;entry.toldAt=now();entry.toldProgressAt=progressAt;
  const complete=generation!==null;
  await publishServedV1({client:input.client,server:input.owner,request:entry.request,projects,generation,progress:{total:entry.request.projects.length,progressAtMs:Math.max(progressAt,entry.heldAt),remainingMilliseconds:remaining(entry,complete)},now}).catch(()=>false);
 };
 const start=(reason:"request"|"refresh")=>{
  clearSteeringV1(input.steerDirectory);priorityKey="";progressAt=now();dirtyCount=-1;
  const controller=new AbortController(),forward=()=>controller.abort(input.signal.reason);input.signal.addEventListener("abort",forward,{once:true});
  const state:Running={reason,startedAt:now(),controller,finished:false,done:Promise.resolve()};
  state.done=input.infer(reason,controller.signal).catch((error):WantedInferV1=>({ok:false,status:1,seconds:(now()-state.startedAt)/1000,generation:null,detail:String((error as Error)?.message??error).slice(0,200)})).then(async result=>{
   input.signal.removeEventListener("abort",forward);state.finished=true;
   if(input.signal.aborted)return;
   if(!result.ok){holdUntil=now()+server.refreshQuietMilliseconds;failed(result);if(reason==="refresh")refreshedAt=now();return;}
   refreshedAt=now();
   for(const entry of held.values()){await answer(entry,entry.request.projects,result.generation);told.add(wantedClaimKeyV1(entry.request));input.say(wantedTextV1("served",language,{count:entry.request.projects.length,requester:entry.request.requester,seconds:Math.round(result.seconds)}));}
   if(reason==="refresh")input.say(wantedTextV1("refreshed",language,{seconds:Math.round(result.seconds)}));
   held.clear();
  });
  running=state;
 };
 const steer=()=>{
  const table=input.costs?.(),asked=[...held.values()].sort((a,b)=>a.request.createdMs-b.request.createdMs),demand=new Map<string,number>();
  for(const entry of asked)for(const name of new Set(entry.request.projects))demand.set(name,(demand.get(name)??0)+1);
  const wanted=orderByCostV1(asked.flatMap(entry=>entry.request.projects),name=>estimateOwnerCostV1(table,name),new Set(asked.flatMap(entry=>entry.request.heavy??[])),demand),key=wanted.join("\n");
  if(key!==priorityKey){priorityKey=key;priorityAt=now();writePriorityV1(input.steerDirectory,wanted,priorityAt);}
 };
 const fresh=(request:WantedRequestV1):readonly string[]=>{
  const pending=readPendingV1(input.steerDirectory);
  if(pending===undefined||pending.priorityAtMs<priorityAt)return[];
  const dirty=new Set(pending.projects);
  return request.projects.filter(name=>!dirty.has(name));
 };
 const renewHeld=(request:WantedRequestV1)=>{
  const entry=held.get(request.requester);
  if(entry!==undefined&&wantedClaimKeyV1(entry.request)===wantedClaimKeyV1(request))entry.request=request;
 };
 const shareCosts=async()=>{
  const table=input.costs?.();if(table===undefined||Object.keys(table.owners).length===0)return;
  const key=JSON.stringify(table);if(key===costsKey||now()-costsAt<server.costsMilliseconds)return;
  costsKey=key;costsAt=now();await publishOwnerCostsV1({client:input.client,server:input.owner,table}).catch(()=>false);
 };
 while(!input.signal.aborted&&(input.rounds===undefined||round++<input.rounds)){
  try{
   await shareCosts();
   if(running?.finished===true){await running.done;running=undefined;}
   foreignNow.clear();
   if(now()>=holdUntil&&(input.mayServe===undefined||await input.mayServe())){
    const taken=await claimWantedV1({client:input.client,owner:input.owner,platform:input.platform,foreign:noteForeign,live:renewHeld,requesters:await input.requesters().catch(()=>[]).then(list=>{seen=list.length;return list;}),known:key=>told.has(key)||[...held.values()].some(entry=>wantedClaimKeyV1(entry.request)===key),signal:input.signal,now,policy});
    for(const request of taken){
     const before=held.get(request.requester);
     held.set(request.requester,{request,list:before?.list??[],told:before?.told??false,toldAt:before?.toldAt??-Infinity,toldProgressAt:before?.toldProgressAt??-Infinity,heldAt:before?.heldAt??now(),claimedAt:now()});
     if(before===undefined)input.say(wantedTextV1("serving",language,{count:request.projects.length,requester:request.requester}));
     else input.say(wantedTextV1("joined",language,{count:request.projects.length,arrived:before.list.length,requester:request.requester}));
    }
   }
   for(const [requester,entry] of held)if(entry.request.expiresMs<=now())held.delete(requester);
   for(const [requester,entry] of held){
    if(now()-entry.claimedAt<policy.renewMilliseconds)continue;
    entry.claimedAt=now();
    if(await input.client.renew(wantedClaimKeyV1(entry.request),input.owner,policy.ttlMilliseconds).catch(()=>"fault")==="lost"){held.delete(requester);input.say(wantedTextV1("lost",language,{requester}));}
   }
   if(held.size>0){
    steer();
    const pending=readPendingV1(input.steerDirectory);
    if(pending!==undefined&&pending.projects.length!==dirtyCount){dirtyCount=pending.projects.length;progressAt=now();}
    for(const entry of held.values()){
     const done=fresh(entry.request);
     if(!entry.told||done.length>entry.list.length){
      if(done.length>entry.list.length)input.say(wantedTextV1("answering",language,{arrived:done.length,count:entry.request.projects.length,requester:entry.request.requester}));
      await answer(entry,done,null);
     }else if(now()-entry.toldAt>=server.heartbeatMilliseconds||progressAt>entry.toldProgressAt)await answer(entry,entry.list,null);
    }
   }
   if(running===undefined&&!input.signal.aborted&&now()>=holdUntil){
    const room=await input.headroom();
    if(held.size>0&&room.pressure==="normal"&&room.freeMegabytes>=policy.serveHeadroomMegabytes)start("request");
    else if(held.size===0&&room.pressure==="normal"&&room.freeMegabytes>=server.refreshHeadroomMegabytes){
     const age=now()-refreshedAt,quiet=await input.quietMilliseconds();
     if(age>=server.refreshIntervalMilliseconds||(age>=server.refreshQuietMilliseconds&&quiet>=server.refreshQuietMilliseconds)){
      if(input.mayRefresh!==undefined&&!await input.mayRefresh())refreshedAt=now();
      else{
       input.say(wantedTextV1("refreshing",language,{reason:`${Math.round(quiet/1000)} s quiet, ${Number.isFinite(age)?Math.round(age/1000)+" s":"never"} since the last`}));
       start("refresh");
      }
     }
    }
   }
  }catch(error){if(input.signal.aborted)break;holdUntil=now()+server.refreshQuietMilliseconds;input.say(wantedTextV1("failed",language,{status:1,seconds:0,detail:String((error as Error)?.message??error).slice(0,200)}));}
  if(++tick%server.aliveRounds===0)input.say(wantedTextV1("alive",language,{requesters:seen,held:held.size,foreign:foreignNow.size,reading:running===undefined?"none":running.reason}));
  await input.sleep(server.roundMilliseconds,input.signal);
 }
 if(running!==undefined){if(input.signal.aborted)running.controller.abort(input.signal.reason);await running.done;}
}
