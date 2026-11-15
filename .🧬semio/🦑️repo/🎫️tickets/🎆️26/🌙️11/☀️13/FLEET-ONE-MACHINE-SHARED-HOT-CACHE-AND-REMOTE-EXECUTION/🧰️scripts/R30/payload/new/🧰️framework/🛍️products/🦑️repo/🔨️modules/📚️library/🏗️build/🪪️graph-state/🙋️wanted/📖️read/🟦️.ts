import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";
import type{GraphFleetClientV1}from"../../🛰️exchange/🟦️.ts";
import{awaitWantedV1,planStaleReadV1,publishWantedV1,readServedV1,wantedEstimateTextV1,wantedPolicyV1,wantedTextV1,type WantedEndV1,type WantedPlanV1,type WantedPolicyV1,type WantedProgressV1,type WantedServedV1}from"../🟦️.ts";
import{classifyOwnersV1,describeHeavyV1,grantOfFootprintV1,ownerCostPolicyV1,ownerCostTextV1,ownerGrantV1,planSoloV1,type HeavyOwnerV1,type OwnerEstimateV1,type OwnerGrantV1}from"../💰️cost/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{platformClassV1}from"../../🌍️platform-class/🟦️.ts";

/** 🚦️ The refusal of a read that waited its bound for the fleet and still lacks more projects than it can infer: the class `unserved`, exit status 75 (temporary failure, try again). */
export const wantedRefusalV1=Object.freeze({...schema.definitions.Refusal.const});

export type FleetReadV1=Readonly<{outcome:"local"|"served"|"refused";plan:WantedPlanV1;arrived:number;missing:readonly string[];waitedMilliseconds:number;inferred?:number;solo?:SoloWaveV1}>;
/** 🏋️ The heavy owners among the remaining projects that this machine infers itself, one at a time and alone after the light ones, each with at most `grant` (see `soloGrantV1`). */
export type SoloWaveV1=Readonly<{owners:readonly HeavyOwnerV1[];grant:OwnerGrantV1}>;

/** 📖️ The fleet-first decision of a graph read that found `stale` projects (names): decide by the admitted memory, then take what the fleet already served, publish the rest as a request, wait with progress, a bound and cancellation, and answer which projects remain for this machine (`missing`). Refuses with {@link wantedRefusalV1} only when the machine cannot run even the smallest wave: `plan.localProjects` (the waves one wait allows) decides only how many light projects it still waits the fleet out for, never how many it infers, so the light projects the fleet did not deliver are inferred here, wave after wave, however many they are, and one machine stays usable whenever the fleet is down or busy. The decision is made by MEMORY, owner by owner: with `estimate` (the cost of one project in managed megabytes) every project whose cost does not fit what this machine can give one project is heavy: it is never inferred beside others, is asked from the fleet first and waited for however few of them remain, so "2 projects left" never turns into a parallel local inference of the workspace root; the light ones fill the grant by count as before. A machine must be able to read alone (local-first): when the fleet does not serve (unreachable, or the wait ends) the heavy ones that remain are inferred here one at a time, alone, after the light ones, each with `soloGrant` (the memory one owner can be given while nothing else of the read runs, `soloGrantV1`; none when omitted); `solo` of the result names them and the grant, and the caller must run exactly these sequentially with that grant. A learned cost never refuses an owner (`planSoloV1`): the heavy ones that remain are attempted alone under `soloGrant`, where the memory governor stops one only when it really grows past it (typed `graph.owner-too-heavy`, naming the owner and the measured numbers in both languages); only a machine whose `soloGrant` is nothing refuses them here. The grant is judged by `grantMegabytes` (the share of the machine, which a busy moment does not lower while a wave still fits) and the waves by `admittedMegabytes`. A machine admitted at least `serveHeadroomMegabytes` is a serving machine and always infers itself. While the request is outstanding and the machine can run a wave, `infer` (the governed inference the caller runs after the wait: it never throws except on cancellation) runs ONE continuous inference of all the light projects the fleet has not delivered here at the same time (one plan stage, one worker farm for the whole wait), lightest first (the server works the heaviest first, so the two meet in the middle instead of overlapping); it consults `skip` before each project (what the fleet delivered meanwhile is dropped, a wait that ended by its bounds drops the rest) and tells `done` after each; the request is renewed without the inferred ones every `narrowMilliseconds` (the same `createdMs`, so the server keeps its claim and position and drops them), and the read ends when every light project is served or inferred (`inferred` counts the second kind, `arrived` the first); a wait that ends by its bounds leaves the remainder to the caller. The reader reserves nothing for inference before this has answered, and before it starts inferring; the caller restores the newest fleet generation afterwards, which carries every served project. */
export async function fleetReadV1(input:Readonly<{stale:readonly string[];admittedMegabytes:number;grantMegabytes?:number;soloGrant?:OwnerGrantV1;client:GraphFleetClientV1|undefined;requester:string;platform:string;graphKey:string;language:"en"|"de";estimate?:(name:string)=>OwnerEstimateV1;readings?:Readonly<Record<string,string|number>>;infer?(run:Readonly<{names:readonly string[];skip(name:string):boolean;done(name:string):void}>):Promise<unknown>;say(line:string):void;signal?:AbortSignal;sleep?(milliseconds:number):Promise<void>;now?:()=>number;policy?:WantedPolicyV1}>):Promise<FleetReadV1>{
 const policy=input.policy??wantedPolicyV1,now=input.now??Date.now,started=now(),{language}=input;
 if(input.admittedMegabytes>=policy.serveHeadroomMegabytes)return{outcome:"local",plan:planStaleReadV1({stale:input.stale.length,taken:0,admittedMegabytes:input.admittedMegabytes},{...policy,maximumLocalWaves:Number.MAX_SAFE_INTEGER}),arrived:0,missing:[],waitedMilliseconds:0};
 const usable=input.client!==undefined&&input.client.usable(),served=usable?await readServedV1({client:input.client!,requester:input.requester}).catch(()=>undefined):undefined,have=new Set(served!==undefined&&started-served.atMs<policy.ttlMilliseconds?served.projects:[]);
 const grant=ownerGrantV1(input.grantMegabytes??input.admittedMegabytes,policy),classes=input.estimate===undefined?{heavy:[] as HeavyOwnerV1[],light:input.stale}:classifyOwnersV1(input.stale,input.estimate,grant),heavyNames=new Map(classes.heavy.map(row=>[row.name,row])),heavyRest=classes.heavy.filter(row=>!have.has(row.name));
 const taken=input.stale.filter(name=>have.has(name)),plan=planStaleReadV1({stale:input.stale.length,taken:taken.length,admittedMegabytes:input.admittedMegabytes,heavy:heavyRest.length},policy);
 if(plan.mode==="local")return{outcome:"local",plan,arrived:taken.length,missing:[],waitedMilliseconds:0};
 const rest=input.stale.filter(name=>!have.has(name)),perProject=Math.round(grant.managedMegabytes*ownerCostPolicyV1.footprintFactor);
 if(input.readings!==undefined)input.say(wantedTextV1("readings",language,input.readings));
 input.say(heavyRest.length>0?ownerCostTextV1("plan",language,{stale:input.stale.length,taken:taken.length,heavy:heavyRest.length,named:describeHeavyV1(heavyRest,language),grant:perProject,local:plan.localProjects,wave:plan.waveProjects,wanted:plan.wantedProjects}):wantedTextV1("plan",language,{stale:input.stale.length,taken:taken.length,local:plan.localProjects,wave:plan.waveProjects,wanted:plan.wantedProjects}));
 const soloGrant=input.soloGrant??grantOfFootprintV1(0),soloLimit=Math.round(soloGrant.managedMegabytes*ownerCostPolicyV1.footprintFactor);
 const solo=(missing:readonly string[]):SoloWaveV1|undefined=>{
  const{fit,over}=planSoloV1(heavyRest.filter(row=>missing.includes(row.name)),soloGrant);
  if(over.length>0){const named=describeHeavyV1(over,language);input.say(ownerCostTextV1("exceeds",language,{named,grant:soloLimit}));throw refuseBuildV1("unserved","graph.owner-too-heavy",{named,limit:soloLimit});}
  if(fit.length===0)return undefined;
  input.say(ownerCostTextV1("solo",language,{count:fit.length,named:describeHeavyV1(fit,language),grant:soloLimit}));
  return{owners:fit,grant:soloGrant};
 };
 const light=(missing:readonly string[])=>missing.filter(name=>!heavyNames.has(name)).length;
 const refuse=(arrived:number,missing:readonly string[],cause:WantedEndV1|"unreachable"):never=>{
  const fields={wanted:rest.length,arrived,seconds:Math.round((now()-started)/1000),missing:missing.length,local:plan.localProjects,heavy:""};
  input.say(wantedTextV1("class",language,{platform:input.platform,class:platformClassV1(input.platform)}));
  if(cause==="silent")input.say(wantedTextV1("causeSilent",language,{minutes:Math.round(policy.silenceMilliseconds/60000)}));
  else if(cause==="stalled")input.say(wantedTextV1("causeStalled",language,{minutes:Math.round(policy.waitMilliseconds/60000)}));
  else if(cause==="limit")input.say(wantedTextV1("causeLimit",language,{hours:Math.round(policy.waitMaximumMilliseconds/3600000)}));
  input.say(wantedTextV1("refused",language,fields));
  throw refuseBuildV1("unserved","graph.wanted-expired",fields);
 };
 const created=now(),cost=(name:string)=>input.estimate?.(name).megabytes??0,order=rest.filter(name=>!heavyNames.has(name)).sort((a,b)=>cost(a)-cost(b)||(a<b?1:a>b?-1:0)),cooperate=input.infer!==undefined&&plan.waveProjects>0&&order.length>0;
 if(!usable||!await publishWantedV1({client:input.client!,requester:input.requester,platform:input.platform,graphKey:input.graphKey,projects:rest,heavy:heavyRest.map(row=>row.name),createdMs:created,now,policy}).catch(()=>false)){
  input.say(wantedTextV1("unreachable",language,{wanted:rest.length,local:plan.localProjects}));
  if(plan.waveProjects===0)return refuse(0,rest,"unreachable");
  const alone=solo(rest);
  return{outcome:"served",plan,arrived:0,missing:rest,waitedMilliseconds:0,...alone===undefined?{}:{solo:alone}};
 }
 const done=new Set<string>();let answer=new Set<string>(),leftHere=new Set<string>(),looping=cooperate,ended=false,fetchedAt=-Infinity,shown=-1,shownAt=-Infinity,heard:number|undefined,heardRow:WantedServedV1|undefined,heardLocalAt=started,endCause:WantedEndV1="stalled";
 const refresh=async()=>{if(now()-fetchedAt<policy.pollMilliseconds/2)return;fetchedAt=now();const row=await readServedV1({client:input.client!,requester:input.requester}).catch(()=>undefined);if(row!==undefined){answer=new Set(row.projects);for(const name of row.left??[])leftHere.add(name);if(row.atMs!==heard)heardLocalAt=now();heard=row.atMs;heardRow=row;}};
 const fleetArrived=()=>rest.reduce((sum,name)=>sum+(answer.has(name)&&!done.has(name)?1:0),0);
 const narrow=async()=>{const left=rest.filter(name=>!answer.has(name)&&!done.has(name)&&!leftHere.has(name));if(left.length>0)await publishWantedV1({client:input.client!,requester:input.requester,platform:input.platform,graphKey:input.graphKey,projects:left,heavy:heavyRest.map(row=>row.name).filter(name=>left.includes(name)),createdMs:created,now,policy}).catch(()=>false);};
 const cut=new Set<string>(),skipped=new Set<string>();let narrowedAt=now();
 const work=cooperate?(async()=>{
  input.say(wantedTextV1("inferring",language,{count:order.length}));
  input.signal?.throwIfAborted();await refresh();
  const names=order.filter(name=>!answer.has(name));
  if(names.length===0)return;
  await input.infer!({names,skip:name=>{if(answer.has(name)){skipped.add(name);return true;}if(ended){cut.add(name);return true;}return false;},done:name=>{done.add(name);if(now()-narrowedAt>=policy.narrowMilliseconds){narrowedAt=now();void narrow();}}});
  for(const name of names)if(!cut.has(name)&&!skipped.has(name))done.add(name);
  await narrow();
 })().catch(error=>{if(input.signal?.aborted)return;input.say(wantedTextV1("inferFailed",language,{detail:String((error as Error)?.message??error).replace(/\s+/gu," ").slice(0,160)}));}).finally(()=>{looping=false;}):undefined;
 const missing=await awaitWantedV1({projects:rest,take:async name=>{await refresh();return answer.has(name)||leftHere.has(name)||done.has(name);},alive:()=>heard,progressSign:()=>heardRow?.progressAtMs,renew:async left=>{await publishWantedV1({client:input.client!,requester:input.requester,platform:input.platform,graphKey:input.graphKey,projects:left,heavy:heavyRest.map(row=>row.name).filter(name=>left.includes(name)),createdMs:created,now,policy});},end:cause=>{endCause=cause;},...(input.signal===undefined?{}:{signal:input.signal}),...(input.sleep===undefined?{}:{sleep:input.sleep}),now,policy,settled:left=>!looping&&light(left)<=plan.localProjects&&!left.some(name=>heavyNames.has(name)),progress(progress:WantedProgressV1){const here=done.size,fleet=cooperate?fleetArrived():progress.arrived,mark=progress.arrived;if(mark!==shown||now()-shownAt>=10_000){shown=mark;shownAt=now();const age=Math.max(0,Math.round(((heardRow?.atMs??0)-(heardRow?.progressAtMs??0)+now()-heardLocalAt)/1000));input.say(cooperate?(heardRow?.progressAtMs===undefined?wantedTextV1("cooperating",language,{arrived:fleet,wanted:progress.wanted,inferred:here,left:Math.max(0,progress.wanted-fleet-here)}):wantedTextV1("cooperatingServer",language,{arrived:fleet,wanted:progress.wanted,inferred:here,left:Math.max(0,progress.wanted-fleet-here),age,estimate:wantedEstimateTextV1(heardRow.remainingMilliseconds,language)})):heardRow?.progressAtMs===undefined?wantedTextV1("waiting",language,{arrived:progress.arrived,wanted:progress.wanted}):wantedTextV1("waitingServer",language,{arrived:progress.arrived,wanted:progress.wanted,age:Math.max(0,Math.round((heardRow.atMs-heardRow.progressAtMs+now()-heardLocalAt)/1000)),estimate:wantedEstimateTextV1(heardRow.remainingMilliseconds,language)}));}}}).finally(()=>{ended=true;});
 await work;
 if(input.signal?.aborted)input.signal.throwIfAborted();
 const open=missing.filter(name=>!done.has(name)),kept=[...open,...rest.filter(name=>leftHere.has(name)&&!answer.has(name)&&!done.has(name)&&!open.includes(name))],arrived=rest.filter(name=>answer.has(name)&&!done.has(name)).length;
 if(leftHere.size>0)input.say(wantedTextV1("leftHere",language,{count:leftHere.size,named:[...leftHere].sort().join(", ")}));
 if(plan.waveProjects===0)refuse(arrived,kept,endCause);
 const alone=solo(kept);
 return{outcome:"served",plan,arrived:arrived+taken.length,...cooperate?{inferred:done.size}:{},missing:kept,waitedMilliseconds:now()-started,...alone===undefined?{}:{solo:alone}};
}
