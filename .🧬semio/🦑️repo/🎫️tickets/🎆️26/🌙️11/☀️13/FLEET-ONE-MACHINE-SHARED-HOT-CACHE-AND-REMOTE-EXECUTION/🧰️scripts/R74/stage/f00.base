import {mkdirSync,readdirSync,readFileSync,renameSync,statSync,unlinkSync,writeFileSync} from "node:fs";
import {homedir,uptime} from "node:os";
import {isAbsolute,join} from "node:path";
import {requireMaterializedPathV1,type ResourceLeaseControl} from "../../../../🔒️leases/🎛️control/🟦️.ts";
import {agedHostSampleV1,evaluateHostCapacityV1,hostBootAnchorV1,hostCapacityErrorV1,isCurrentHostGrantV1,isHostCapacityV1,requireHostCapacityV1,withUnmanagedHostV1,type HostBootAnchorV1,type HostCapacityPolicyV1,type HostEvaluationV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../📏️decision/🟦️.ts";
import {currentHostProbeV1,readHostSampleV1,type HostSampleRecordV1} from "../../📡️sampling/🟦️.ts";
import {hostDeclarationKeyV1} from "../../🪞️profile/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import type {HostCapacityPortsV1} from "../📋️contract/🟦️.ts";

const protocol=schema.definitions.Protocol.const;

/** 🗂️ The machine-wide gate directory: one per user and shared by every checkout, so the whole machine is one admission domain. */
export function hostCapacityDirectoryV1(environment:Readonly<Record<string,string|undefined>>=process.env,platform:NodeJS.Platform=process.platform,home:string=homedir()):string{
 const override=environment[protocol.directoryEnvironment];
 if(override)return requireMaterializedPathV1(override);
 const base=platform==="darwin"?join(home,"Library","Caches"):platform==="win32"?environment.LOCALAPPDATA??join(home,"AppData","Local"):isAbsolute(environment.XDG_CACHE_HOME??"")?environment.XDG_CACHE_HOME!:join(home,".cache");
 return requireMaterializedPathV1(join(base,"semio","host-capacity"));
}


const bootAnchors=new Map<string,HostBootAnchorV1>();
const readBootAnchor=(value:unknown):HostBootAnchorV1|undefined=>{const row=value as Partial<HostBootAnchorV1>|undefined;return Number.isSafeInteger(row?.bootAtMs)&&Number.isSafeInteger(row?.uptimeMs)?{bootAtMs:row!.bootAtMs!,uptimeMs:row!.uptimeMs!}:undefined;};
/** ⚓️ The boot instant of the grants in `directory`: read from the anchor of the boot (see `hostBootAnchorV1`), written when this is the first process of the boot, and kept for the life of this process, so the wall clock is read once and never moves it. */
export function hostBootAtV1(directory:string,nowMs:number,uptimeMs:number=Math.floor(uptime()*1000)):number{
 let anchor=bootAnchors.get(directory);
 if(anchor===undefined){
  const path=join(directory,protocol.bootFile),stored=readBootAnchor(readJson(path));
  anchor=hostBootAnchorV1(stored,nowMs,uptimeMs);
  if(anchor!==stored)try{writeJson(path,anchor);}catch{}
  bootAnchors.set(directory,anchor);
 }
 return anchor.bootAtMs;
}

export const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};
/** 🕳️ Says whether an error only reports that a file or directory is not there. */
export const absentV1=(error:unknown):boolean=>["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException)?.code??"");
export const readJson=(path:string):unknown=>{try{return JSON.parse(readFileSync(path,"utf8"));}catch(error){if(absentV1(error)||error instanceof SyntaxError)return undefined;throw error;}};
export const remove=(path:string):void=>{try{unlinkSync(path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}};
export const modified=(path:string):number=>{try{return statSync(path).mtimeMs;}catch(error){if(absentV1(error))return 0;throw error;}};
export const names=(directory:string,child:string):string[]=>readdirSync(join(directory,child)).filter(file=>file.endsWith(".json"));

/** 📋️ What the gate currently knows. */
export type HostCapacityStatusV1=Readonly<{host:HostSampleV1;grants:readonly HostGrantV1[];waiters:readonly HostWaiterV1[];evaluation:HostEvaluationV1}>;

export function writeJson(path:string,value:unknown):void{
 const text=JSON.stringify(value);if(text.length>protocol.maximumFileBytes)throw hostCapacityErrorV1("recordTooLarge");
 const temporary=`${path}.${process.pid}.tmp`;writeFileSync(temporary,text);renameSync(temporary,path);
}

export function prepare(directory:string):string{
 const root=requireMaterializedPathV1(directory);for(const name of ["grants","queue"])mkdirSync(join(root,name),{recursive:true,mode:0o700});
 return root;
}

export function readGrants(directory:string,now:number,policy:HostCapacityPolicyV1):HostGrantV1[]{
 const rows:HostGrantV1[]=[],boot=hostBootAtV1(directory,now);
 for(const name of names(directory,"grants")){
  const path=join(directory,"grants",name),value=readJson(path);
  if(!isHostCapacityV1("Grant",value)){if(now-modified(path)>policy.wait.lockStaleMilliseconds)remove(path);continue;}
  const grant=value as HostGrantV1;
  if(!isCurrentHostGrantV1({grant,expectedId:name.slice(0,-5),pidAlive:alive(grant.pid),nowMs:now,bootAtMs:boot,modifiedAtMs:modified(path)},policy))remove(path);else rows.push(grant);
 }
 return rows;
}

export function readWaiters(directory:string,now:number,policy:HostCapacityPolicyV1):HostWaiterV1[]{
 const rows:HostWaiterV1[]=[],stale=policy.wait.pollMilliseconds*30;
 for(const name of names(directory,"queue")){
  const path=join(directory,"queue",name),value=readJson(path);
  if(!isHostCapacityV1("Waiter",value)){if(now-modified(path)>policy.wait.lockStaleMilliseconds)remove(path);continue;}
  const waiter=value as HostWaiterV1;
  if(!alive(waiter.pid)||now-modified(path)>stale)remove(path);else rows.push(waiter);
 }
 return rows;
}

export async function currentSample(directory:string,policy:HostCapacityPolicyV1,control:ResourceLeaseControl,ports:HostCapacityPortsV1,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<HostSampleV1>{
 const now=ports.now??Date.now,path=join(directory,"sample.json"),prior=readJson(path) as HostSampleRecordV1|undefined,cached=isHostCapacityV1("Host",prior?.host)?prior!.host:undefined,declared=hostDeclarationKeyV1(environment);
 if(cached&&(prior!.declared??"")===declared&&now()>=cached.atMs&&now()-cached.atMs<=policy.wait.sampleMaximumAgeMilliseconds)return cached;
 const known=cached&&(prior!.declared??"")===declared?cached:undefined,previous=cached?{atMs:cached.atMs,swappedOutMegabytes:prior!.swappedOutMegabytes}:undefined;
 const record=await(async()=>ports.sample?ports.sample(previous):readHostSampleV1(policy,currentHostProbeV1(control.signal),previous,now,environment))().catch((error:unknown)=>{if(known===undefined||control.signal.aborted)throw error;return undefined;});
 if(record===undefined)return agedHostSampleV1(known!,now());
 requireHostCapacityV1<HostSampleV1>("Host",record.host);
 try{writeJson(path,{...record,declared});}catch{}
 return record.host;
}

/** 🚥️ Reads the gate without joining its queue: the sample, the live grants, the waiters and what the next request would get. */
export async function readHostCapacityStatusV1(options:HostCapacityPortsV1&Readonly<{directory:string;policy:HostCapacityPolicyV1;control:ResourceLeaseControl;environment?:Readonly<Record<string,string|undefined>>}>):Promise<HostCapacityStatusV1>{
 const now=options.now??Date.now,directory=prepare(options.directory),host=await currentSample(directory,options.policy,options.control,options,options.environment),grants=readGrants(directory,now(),options.policy),waiters=readWaiters(directory,now(),options.policy);
 return Object.freeze({host,grants,waiters,evaluation:evaluateHostCapacityV1(options.policy,withUnmanagedHostV1(host,grants),grants,waiters,now())});
}
