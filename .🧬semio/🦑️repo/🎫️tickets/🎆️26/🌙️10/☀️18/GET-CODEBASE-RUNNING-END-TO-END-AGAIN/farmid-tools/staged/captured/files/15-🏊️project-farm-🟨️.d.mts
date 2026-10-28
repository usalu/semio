import type{ChildProcess}from"node:child_process";
import type{GraphFarmEnvironmentV1}from"./🏊️pool/🟨️.mjs";
/** 🧬️ How the farm starts a worker from the snapshot its parent runs from: the loader command and arguments, the variables of a captured process, the digest of the snapshot and its text. */
export interface ProjectFarmLaunchV1{readonly command:string;readonly arguments:readonly string[];readonly environment:Readonly<Record<string,string>>;readonly implementation:string;text():string}
/** 📦️ One project the farm infers in a worker. */
export interface ProjectFarmJobV1{readonly name:string;readonly root:string;readonly json:unknown;readonly native:boolean;readonly pipeline:boolean;readonly nativeRoot:string;readonly selectedInputs:readonly string[];readonly head:readonly unknown[];readonly cacheKey:string}
/** 🎟️ A request for more workers in the machine-wide capacity queue. */
export interface ProjectFarmCapacityV1{readonly granted:Promise<{readonly jobs:number;readonly megabytes:number}|undefined>;release():void}
/** 🏊️ The open farm of one graph construction. */
export interface ProjectFarmV1{onReady(listener:(loops:number)=>void):void;prepare(job:ProjectFarmJobV1):Promise<boolean>;close():Promise<void>}
/** 🚜️ Plans the farm of one graph construction, or answers undefined when the work is small or the farm is off; `launch: null` starts workers from their files. */
export function openProjectFarmV1(options:{workspaceRoot:string;context:{readonly registry?:unknown};projects:number;commandInputs:unknown;lateClosures?:boolean;projectsByRoot:ReadonlyMap<string,unknown>;componentRoots:readonly string[];environment?:GraphFarmEnvironmentV1;signal?:AbortSignal;processed?:()=>number;launch?:ProjectFarmLaunchV1|null;command?:string;commandArguments?:readonly string[];spawnProcess?:(command:string,args:readonly string[],options:Readonly<Record<string,unknown>>)=>ChildProcess;request?:(input:{preferredJobs:number;environment:GraphFarmEnvironmentV1;signal?:AbortSignal;write:(line:string)=>void})=>ProjectFarmCapacityV1;cores?:number;write?:(line:string)=>void}):ProjectFarmV1|undefined;
