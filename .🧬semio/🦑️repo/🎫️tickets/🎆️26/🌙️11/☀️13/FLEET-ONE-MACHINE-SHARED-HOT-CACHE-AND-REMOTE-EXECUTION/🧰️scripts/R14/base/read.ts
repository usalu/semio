import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";
import type{GraphFleetClientV1}from"../../🛰️exchange/🟦️.ts";
import{awaitWantedV1,planStaleReadV1,publishWantedV1,readServedV1,wantedPolicyV1,wantedTextV1,type WantedPlanV1,type WantedPolicyV1,type WantedProgressV1}from"../🟦️.ts";
import{classifyOwnersV1,describeHeavyV1,ownerCostPolicyV1,ownerCostTextV1,ownerGrantV1,type HeavyOwnerV1,type OwnerEstimateV1}from"../💰️cost/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{platformClassV1}from"../../🌍️platform-class/🟦️.ts";

/** 🚦️ The refusal of a read that waited its bound for the fleet and still lacks more projects than it can infer: the class `unserved`, exit status 75 (temporary failure, try again). */
export const wantedRefusalV1=Object.freeze({...schema.definitions.Refusal.const});

export type FleetReadV1=Readonly<{outcome:"local"|"served"|"refused";plan:WantedPlanV1;arrived:number;missing:readonly string[];waitedMilliseconds:number;heavy?:readonly HeavyOwnerV1[]}>;

/** 📖️ The fleet-first decision of a graph read that found `stale` projects (names): decide by the admitted memory, then take what the fleet already served, publish the rest as a request, wait with progress, a bound and cancellation, and answer which projects remain for this machine. Refuses with {@link wantedRefusalV1} when more remain than it can infer. The decision is made by MEMORY, owner by owner: with `estimate` (the cost of one project in managed megabytes) every project whose cost does not fit what this machine can give one project is heavy, is never inferred here and is asked from the fleet and waited for however few of them remain, so "2 projects left" never turns into a local inference of the workspace root; the light ones fill the grant by count as before. The grant is judged by `grantMegabytes` (the share of the machine, which a busy moment does not lower while a wave still fits) and the waves by `admittedMegabytes`. A machine admitted at least `serveHeadroomMegabytes` is a serving machine and always infers itself. The reader reserves nothing for inference before this has answered; the caller restores the newest fleet generation afterwards, which carries every served project. */
export async function fleetReadV1(input:Readonly<{stale:readonly string[];admittedMegabytes:number;grantMegabytes?:number;client:GraphFleetClientV1|undefined;requester:string;platform:string;graphKey:string;language:"en"|"de";estimate?:(name:string)=>OwnerEstimateV1;say(line:string):void;signal?:AbortSignal;sleep?(milliseconds:number):Promise<void>;now?:()=>number;policy?:WantedPolicyV1}>):Promise<FleetReadV1>{
 const policy=input.policy??wantedPolicyV1,now=input.now??Date.now,started=now(),{language}=input;
 if(input.admittedMegabytes>=policy.serveHeadroomMegabytes)return{outcome:"local",plan:planStaleReadV1({stale:input.stale.length,taken:0,admittedMegabytes:input.admittedMegabytes},{...policy,maximumLocalWaves:Number.MAX_SAFE_INTEGER}),arrived:0,missing:[],waitedMilliseconds:0};
 const usable=input.client!==undefined&&input.client.usable(),served=usable?await readServedV1({client:input.client!,requester:input.requester}).catch(()=>undefined):undefined,have=new Set(served!==undefined&&started-served.atMs<policy.ttlMilliseconds?served.projects:[]);
 const grant=ownerGrantV1(input.grantMegabytes??input.admittedMegabytes,policy),classes=input.estimate===undefined?{heavy:[] as HeavyOwnerV1[],light:input.stale}:classifyOwnersV1(input.stale,input.estimate,grant),heavyNames=new Map(classes.heavy.map(row=>[row.name,row])),heavyRest=classes.heavy.filter(row=>!have.has(row.name));
 const taken=input.stale.filter(name=>have.has(name)),plan=planStaleReadV1({stale:input.stale.length,taken:taken.length,admittedMegabytes:input.admittedMegabytes,heavy:heavyRest.length},policy);
 if(plan.mode==="local")return{outcome:"local",plan,arrived:taken.length,missing:[],waitedMilliseconds:0};
 const rest=input.stale.filter(name=>!have.has(name)),perProject=Math.round(grant.managedMegabytes*ownerCostPolicyV1.footprintFactor);
 input.say(heavyRest.length>0?ownerCostTextV1("plan",language,{stale:input.stale.length,taken:taken.length,heavy:heavyRest.length,named:describeHeavyV1(heavyRest,language),grant:perProject,local:plan.localProjects,wave:plan.waveProjects,wanted:plan.wantedProjects}):wantedTextV1("plan",language,{stale:input.stale.length,taken:taken.length,local:plan.localProjects,wave:plan.waveProjects,wanted:plan.wantedProjects}));
 const refuse=(arrived:number,missing:readonly string[]):never=>{
  const heavyMissing=missing.map(name=>heavyNames.get(name)).filter((row):row is HeavyOwnerV1=>row!==undefined).sort((a,b)=>b.megabytes-a.megabytes),named=describeHeavyV1(heavyMissing,language),fields={wanted:rest.length,arrived,seconds:Math.round((now()-started)/1000),missing:missing.length,local:plan.localProjects,heavy:heavyMissing.length>0?ownerCostTextV1("among",language,{named,grant:perProject}):""};
  input.say(wantedTextV1("class",language,{platform:input.platform,class:platformClassV1(input.platform)}));
  input.say(heavyMissing.length>0?ownerCostTextV1("refused",language,{...fields,named,grant:perProject}):wantedTextV1("refused",language,fields));
  throw refuseBuildV1("unserved","graph.wanted-expired",fields);
 };
 if(!usable||!await publishWantedV1({client:input.client!,requester:input.requester,platform:input.platform,graphKey:input.graphKey,projects:rest,heavy:heavyRest.map(row=>row.name),now,policy}).catch(()=>false)){
  input.say(wantedTextV1("unreachable",language,{wanted:rest.length,local:plan.localProjects}));
  return rest.length<=plan.localProjects&&heavyRest.length===0?{outcome:"served",plan,arrived:0,missing:rest,waitedMilliseconds:0}:refuse(0,rest);
 }
 let answer=new Set<string>(),fetchedAt=-Infinity,shown=-1,shownAt=-Infinity,heard:number|undefined;
 const refresh=async()=>{if(now()-fetchedAt<policy.pollMilliseconds/2)return;fetchedAt=now();const row=await readServedV1({client:input.client!,requester:input.requester}).catch(()=>undefined);if(row!==undefined){answer=new Set(row.projects);heard=row.atMs;}};
 const missing=await awaitWantedV1({projects:rest,take:async name=>{await refresh();return answer.has(name);},alive:()=>heard,...(input.signal===undefined?{}:{signal:input.signal}),...(input.sleep===undefined?{}:{sleep:input.sleep}),now,policy,settled:left=>left.length<=plan.localProjects&&!left.some(name=>heavyNames.has(name)),progress(progress:WantedProgressV1){if(progress.arrived!==shown||now()-shownAt>=10_000){shown=progress.arrived;shownAt=now();input.say(wantedTextV1("waiting",language,{arrived:progress.arrived,wanted:progress.wanted}));}}});
 const arrived=rest.length-missing.length;
 if(missing.length>plan.localProjects||missing.some(name=>heavyNames.has(name)))refuse(arrived,missing);
 return{outcome:"served",plan,arrived:arrived+taken.length,missing,waitedMilliseconds:now()-started,...heavyRest.length>0?{heavy:heavyRest}:{}};
}
