/** 🗣️ Writes one farm message to stderr in the languages of the environment. */
export function sayGraphFarmV1(key:string,fields:Readonly<Record<string,unknown>>,environment?:NodeJS.ProcessEnv):void;
/** 🏊️ The handle of the worker pool of one graph phase. */
export interface GraphPoolV1{readonly plan:Readonly<{workers:number;reason:string;workerMegabytes:number}>;open():Promise<Readonly<{refused:string}>|Readonly<{refused?:undefined;farm:import("../🟨️.mjs").GraphFarmV1;workers:number;release():void}>>;release():void|Promise<void>}
/** 🏊️ Plans the worker pool of one graph phase and leases the resident pool of the workspace when the host grants the processes. */
export function startGraphPoolV1(options:Readonly<{workspaceRoot:string;projects:number;services?:Readonly<Record<string,string>>;prestart?:boolean;environment?:NodeJS.ProcessEnv;signal?:AbortSignal;command?:string;commandArguments?:readonly string[];spawnProcess?:unknown;say?:(key:string,fields:Readonly<Record<string,unknown>>,environment?:NodeJS.ProcessEnv)=>void}>):GraphPoolV1;
