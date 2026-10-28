import type {ChildProcess} from "node:child_process";
export type GraphFarmRuntimeRowV1={readonly ramFactor:number;readonly heap?:{readonly variable:string;readonly prefix?:string;readonly unit:"bytes"|"megabytes"};readonly environment:Readonly<Record<string,string>>};
export type GraphFarmPolicyV1={readonly version:number;readonly languages:readonly string[];readonly languageEnvironment:readonly string[];readonly workersEnvironment:string;readonly planEnvironment:string;readonly ownWorkers:number;readonly minimumGrantedWorkers:number;readonly phasesEnvironment:string;readonly minimumProjects:number;readonly missBasisPoints:number;readonly projectsPerWorker:number;readonly cpuReserve:number;readonly maximumForcedWorkers:number;readonly maximumRestarts:number;readonly maximumAttempts:number;readonly loopsPerWorker:number;readonly recycleMemoryRatio:number;readonly recycleAfterProjects:number;readonly initTimeoutMilliseconds:number;readonly jobTimeoutMilliseconds:number;readonly shutdownGraceMilliseconds:number;readonly runtime:Readonly<Record<string,GraphFarmRuntimeRowV1>>};
export type GraphFarmEnvironmentV1=Readonly<Record<string,string|undefined>>;
export const GRAPH_FARM_POLICY_V1:GraphFarmPolicyV1;
export const GRAPH_FARM_MESSAGES_V1:Readonly<Record<string,Readonly<Record<string,string>>>>;
export function selectGraphFarmLanguagesV1(environment:GraphFarmEnvironmentV1):string[];
export function renderGraphFarmMessagesV1(key:string,fields:Readonly<Record<string,string|number>>,environment:GraphFarmEnvironmentV1):string[];
export function planGraphFarmV1(options:{projects:number;environment?:GraphFarmEnvironmentV1;cores:number;policy?:GraphFarmPolicyV1}):{workers:number;reason:string};
export function graphFarmRuntimeEnvironmentV1(runtime:string,workerMegabytes:number,policy?:GraphFarmPolicyV1):Record<string,string>;
export function awaitGrowingV1(promises:Promise<unknown>[]):Promise<void>;
export function graphFarmRuntimeV1():string;
export interface GraphFarmOutcomeV1{readonly reply?:unknown;readonly slot?:number;readonly generation?:number;readonly failure?:{readonly name:string;readonly message:string;readonly code:string|null};readonly lost?:string}
export interface GraphFarmStatsV1{readonly workers:number;readonly jobs:number;readonly replies:number;readonly failures:number;readonly lost:number;readonly recycled:number;readonly started:number;readonly peakRssMegabytes:number;readonly queuedMilliseconds:number;readonly workedMilliseconds:number;readonly bootMilliseconds:number;readonly restarts:number;readonly disabled?:string}
export interface GraphFarmV1{readonly width:number;readonly ready:Promise<boolean>;readonly idle:boolean;readonly usable:boolean;readonly pending:number;stats():GraphFarmStatsV1;run(payload:unknown,options?:{signal?:AbortSignal}):Promise<GraphFarmOutcomeV1>;close():Promise<void>}
export function createFarmV1(options:{entry:string;service:string;session:unknown;workers:number;megabytes:number;environment?:GraphFarmEnvironmentV1;signal?:AbortSignal;policy?:GraphFarmPolicyV1;accept?(identity:unknown):boolean;report?(event:string,fields:Readonly<Record<string,string|number>>):void;retired?(slot:number,generation:number):unknown;spawnProcess?:(command:string,args:readonly string[],options:Readonly<Record<string,unknown>>)=>ChildProcess;runtime?:string;command?:string;commandArguments?:readonly string[]}):GraphFarmV1;
export interface GraphFarmPoolV1{readonly source:"own"|"host";readonly workers:number;readonly megabytes:number}
export function ownGrantPoolV1(environment:GraphFarmEnvironmentV1,policy?:GraphFarmPolicyV1):GraphFarmPoolV1|undefined;
export function grantedPoolV1(granted:{readonly jobs:number;readonly megabytes:number}|undefined,policy?:GraphFarmPolicyV1):GraphFarmPoolV1|undefined;
export function choosePoolV1(pools:readonly {readonly ready:boolean;readonly usable:boolean;readonly idle:boolean;readonly pending:number;readonly width:number}[]):number;
export type GraphFarmRecycleReasonV1="memory"|"projects";
export function recycleReasonV1(worker:{readonly rssMegabytes:number;readonly jobs:number;readonly megabytes:number},policy?:GraphFarmPolicyV1):GraphFarmRecycleReasonV1|undefined;
