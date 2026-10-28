import {lstat,readdir,realpath} from "node:fs/promises";
import {basename,dirname,join} from "node:path";
import {cacheTextV1,formatCacheBytesV1,type CacheLanguageV1} from "../../../../../💾️storage/🧾️ledger/🟦️.ts";
import {nxToolingStoreBoundPolicyV1,planNxToolingStoreBoundV1,type NxToolingStoreBoundPlanV1,type NxToolingStoreBoundPolicyV1,type NxToolingStoreImageV1} from "../🟦️.ts";

/** 🔭️ What the census found: no toolkit directory at all, no readable current activation, or the current identity with every generation and its birth time. */
export type NxToolingCensusV1=Readonly<{kind:"absent"}>|Readonly<{kind:"noCurrent"}>|Readonly<{kind:"ok";current:string;images:readonly Readonly<{identity:string;createdAt:number}>[]}>;
/** 🧾️ What an effect did: generations removed, generations a live reader kept, generations kept for any other reason. */
export type NxToolingRetireOutcomeV1=Readonly<{retired:readonly string[];held:readonly string[];kept:readonly string[]}>;
/** 🔌️ The host side of the bound: cancellation, the clock, progress, the census, the lease probe, the pressure verdict, the free space of the image and (absent in a dry run) the effect; tests replace them. */
export type NxToolingStoreBoundPortsV1=Readonly<{
 signal:AbortSignal;
 now():number;
 progress(stage:string):Promise<void>;
 census():Promise<NxToolingCensusV1>;
 held(identity:string):Promise<boolean>;
 pressure():Promise<boolean>;
 freeBytes():Promise<number|null>;
 idle():Promise<boolean>;
 sleep(milliseconds:number):Promise<void>;
 retire?(identities:readonly string[],limit:number,progress:(identity:string,index:number,total:number)=>Promise<void>):Promise<NxToolingRetireOutcomeV1|"busy">;
}>;
/** 📤️ The verdict of one run. */
export type NxToolingStoreBoundReportV1=Readonly<{dryRun:boolean;state:"absent"|"noCurrent"|"within"|"planned"|"busy"|"done";total:number;pressure:boolean;plan:NxToolingStoreBoundPlanV1|null;outcome:NxToolingRetireOutcomeV1|null;freeBeforeBytes:number|null;freeAfterBytes:number|null}>;

const hex=/^[a-f0-9]{64}$/u;

/** 🧮️ Lists the toolkit generations of a tools image with their birth time (modification time where the volume keeps none) and the identity the workspace activation points to; a bounded, cancellable read that follows no link inside the store. */
export async function censusNxToolingStoreV1(workspace:string,storeRoot:string,ports:Pick<NxToolingStoreBoundPortsV1,"signal"|"progress">,declared:NxToolingStoreBoundPolicyV1=nxToolingStoreBoundPolicyV1()):Promise<NxToolingCensusV1>{
 const base=join(storeRoot,"nx");let names:string[];
 try{names=await readdir(base);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return{kind:"absent"};throw error;}
 let current:string;
 try{current=await realpath(join(workspace,".nx/installation"));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return{kind:"noCurrent"};throw error;}
 const identity=basename(current),physical=await realpath(base);
 if(dirname(current)!==physical||!hex.test(identity))return{kind:"noCurrent"};
 const images:{identity:string;createdAt:number}[]=[];let seen=0;
 for(const name of names.filter(row=>hex.test(row)).sort()){
  ports.signal.throwIfAborted();
  if(++seen>declared.maximumCensus)throw Error(cacheTextV1("en","nxTooling.census",{count:declared.maximumCensus})+"\n"+cacheTextV1("de","nxTooling.census",{count:declared.maximumCensus}));
  if(seen%32===0)await ports.progress("nx-tooling:census:"+seen);
  const facts=await lstat(join(base,name)).catch(error=>{if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;});
  if(!facts?.isDirectory())continue;
  images.push({identity:name,createdAt:Math.max(0,facts.birthtimeMs>0?facts.birthtimeMs:facts.mtimeMs)});
 }
 return images.some(row=>row.identity===identity)?{kind:"ok",current:identity,images}:{kind:"noCurrent"};
}

/** 🏃️ Runs the bound once: census, probe only the generations the count rule would drop for a live reader, plan, and (unless only planning) wait a bounded time for the tools image to be idle, polling without queueing, then hand the planned identities to the effect. A reader that attaches in between is the effect's concern: it re-checks the reader lease of every identity under the whole-image exclusive scope. */
export async function runNxToolingStoreBoundV1(dryRun:boolean,ports:NxToolingStoreBoundPortsV1,declared:NxToolingStoreBoundPolicyV1=nxToolingStoreBoundPolicyV1()):Promise<NxToolingStoreBoundReportV1>{
 const empty=(state:"absent"|"noCurrent")=>({dryRun,state,total:0,pressure:false,plan:null,outcome:null,freeBeforeBytes:null,freeAfterBytes:null}) as NxToolingStoreBoundReportV1;
 await ports.progress("nx-tooling:census");const census=await ports.census();ports.signal.throwIfAborted();
 if(census.kind!=="ok")return empty(census.kind);
 const pressure=await ports.pressure(),freeBeforeBytes=await ports.freeBytes(),nowMs=ports.now(),first=planNxToolingStoreBoundV1({nowMs,current:census.current,pressure,images:census.images.map(row=>({...row,held:false}))},declared);
 const held=new Set<string>();let probed=0;
 for(const identity of first.retire){ports.signal.throwIfAborted();if(await ports.held(identity))held.add(identity);if(++probed%8===0)await ports.progress("nx-tooling:probe:"+probed+"/"+first.retire.length);}
 const images:NxToolingStoreImageV1[]=census.images.map(row=>({...row,held:held.has(row.identity)})),plan=planNxToolingStoreBoundV1({nowMs,current:census.current,pressure,images},declared);
 if(!plan.retire.length)return{dryRun,state:"within",total:images.length,pressure,plan,outcome:null,freeBeforeBytes,freeAfterBytes:freeBeforeBytes};
 if(dryRun||!ports.retire)return{dryRun,state:"planned",total:images.length,pressure,plan,outcome:null,freeBeforeBytes,freeAfterBytes:null};
 let waitedMs=0;
 while(!await ports.idle()){
  if(waitedMs>=declared.idleWaitMs)return{dryRun,state:"busy",total:images.length,pressure,plan,outcome:null,freeBeforeBytes,freeAfterBytes:freeBeforeBytes};
  await ports.progress("nx-tooling:wait:"+Math.round(waitedMs/1000)+"s");await ports.sleep(declared.idlePollMs);waitedMs+=declared.idlePollMs;ports.signal.throwIfAborted();
 }
 const outcome=await ports.retire(plan.retire,plan.limit,(identity,index,total)=>ports.progress("nx-tooling:retire:"+(index+1)+"/"+total+":"+identity.slice(0,12)));
 ports.signal.throwIfAborted();
 if(outcome==="busy")return{dryRun,state:"busy",total:images.length,pressure,plan,outcome:null,freeBeforeBytes,freeAfterBytes:freeBeforeBytes};
 return{dryRun,state:"done",total:images.length,pressure,plan,outcome,freeBeforeBytes,freeAfterBytes:await ports.freeBytes()};
}

/** 🗣️ The report in one language: the same lines in English and German, the bytes in the shared byte format. */
export function formatNxToolingStoreBoundV1(language:CacheLanguageV1,report:NxToolingStoreBoundReportV1):readonly string[]{
 const text=(key:string,variables:Readonly<Record<string,string|number>>={})=>cacheTextV1(language,key,variables),free=(bytes:number|null)=>bytes===null?"?":formatCacheBytesV1(bytes);
 if(report.state==="absent")return [text("nxTooling.absent")];
 if(report.state==="noCurrent")return [text("nxTooling.noCurrent")];
 const plan=report.plan!,count=(reason:string)=>plan.retained.filter(row=>row.reason===reason).length,lines=[text(report.pressure?"nxTooling.keepPressure":"nxTooling.keep",{total:report.total,keep:plan.keepNewest,held:count("held"),young:count("young")})];
 if(report.state==="within")return [...lines,text("nxTooling.within",{free:free(report.freeBeforeBytes)})];
 if(report.state==="planned")return [...lines,text("nxTooling.plan",{count:plan.retire.length,limit:plan.limit,oldest:plan.retire[0]!.slice(0,12),free:free(report.freeBeforeBytes)})];
 if(report.state==="busy")return [...lines,text("nxTooling.busy",{count:plan.retire.length})];
 const outcome=report.outcome!;
 return [...lines,text("nxTooling.done",{retired:outcome.retired.length,planned:plan.retire.length,limit:plan.limit,held:outcome.held.length,kept:outcome.kept.length,remaining:Math.max(0,plan.retire.length-outcome.retired.length-outcome.held.length-outcome.kept.length),before:free(report.freeBeforeBytes),after:free(report.freeAfterBytes)})];
}
