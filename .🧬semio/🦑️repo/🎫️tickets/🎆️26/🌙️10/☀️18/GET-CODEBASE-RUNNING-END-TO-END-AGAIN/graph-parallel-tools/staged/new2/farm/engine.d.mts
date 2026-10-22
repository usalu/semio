import type {ChildProcess,SpawnOptions} from "node:child_process";

/** 🏊️ The declared limits of the graph inference farm. */
export interface GraphFarmPolicyV1{readonly version:1;readonly languages:readonly ("en"|"de")[];readonly languageEnvironment:readonly string[];readonly workersEnvironment:string;readonly workerMegabytesEnvironment:string;readonly minimumWorkerMegabytes:number;readonly planEnvironment:string;readonly phasesEnvironment:string;readonly maximumWorkers:number;readonly minimumProjects:number;readonly projectsPerWorker:number;readonly workerMegabytes:number;readonly parentReserveMegabytes:number;readonly hostShareBasisPoints:number;readonly cpuReserve:number;readonly maximumRestarts:number;readonly capacityWaitMilliseconds:number;readonly initTimeoutMilliseconds:number;readonly jobTimeoutMilliseconds:number;readonly shutdownGraceMilliseconds:number;readonly maximumTableEntries:number;readonly maximumRecordBytes:number;readonly residentIdleMilliseconds:number;readonly prestartColdBasisPoints:number;readonly grantBytes:number;readonly maximumFallbacks:number;readonly seedMegabytes:number;readonly recordTableMegabytes:number;readonly runtime:Readonly<Record<string,Readonly<{ramFactor:number;environment:Readonly<Record<string,string>>}>>>}
export declare const GRAPH_FARM_POLICY_V1:GraphFarmPolicyV1;
export type GraphFarmMessageKeyV1="starting"|"serial"|"fallback"|"recycled"|"finished"|"fallbackLimit"|"disabled";
export declare const GRAPH_FARM_REASONS_V1:Readonly<Record<"en"|"de",Readonly<Record<string,string>>>>;
export declare const GRAPH_FARM_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<GraphFarmMessageKeyV1,string>>>>;
/** 🌐️ The languages of the process; every supported language when none is named. */
export function selectGraphFarmLanguagesV1(environment:Readonly<Record<string,string|undefined>>):("en"|"de")[];
/** 🗣️ One line per language of the environment. */
export function renderGraphFarmMessagesV1(key:GraphFarmMessageKeyV1,fields:Readonly<Record<string,string|number>>,environment:Readonly<Record<string,string|undefined>>):string[];
export type GraphFarmReasonV1="disabled"|"forced"|"small"|"single"|"cpu"|"memory"|"work"|"limit";
export interface GraphFarmPlanV1{readonly workers:number;readonly reason:GraphFarmReasonV1;readonly workerMegabytes:number}
/** 📐️ Sizes the farm from the work, the processors and the memory the host and the parent leave it. */
export function planGraphFarmV1(input:Readonly<{projects:number;environment?:Readonly<Record<string,string|undefined>>;host:Readonly<{cores:number;totalMegabytes:number;availableMegabytes?:number}>;parentMegabytes?:number;policy?:GraphFarmPolicyV1}>):GraphFarmPlanV1;
/** 🧬️ The environment that bounds the heap of one worker process of a runtime. */
export function graphFarmRuntimeEnvironmentV1(runtime:string,workerMegabytes:number,policy?:GraphFarmPolicyV1):Record<string,string>;
export interface InternEncoderV1<T>{encode(items:readonly T[]):{ids:Uint32Array;fresh:[number,T][]};readonly size:number}
/** 📤️ Sends each distinct item once. */
export function createInternEncoderV1<T>(options?:{keyOf?:(item:T)=>string|undefined}):InternEncoderV1<T>;
export interface InternDecoderV1<T>{learn(fresh:readonly (readonly [number,T])[]):void;decode(ids:ArrayLike<number>):T[];readonly size:number}
/** 📥️ Receives what an encoder sent. */
export function createInternDecoderV1<T>(options?:{canonical?:(item:T)=>T;maximumEntries?:number}):InternDecoderV1<T>;
/** 🚦️ Names the runtime that runs this process. */
export function graphFarmRuntimeV1():"bun"|"node";
export interface FarmOptionsV1{entry:string;services?:Readonly<Record<string,string>>;workers:number;environment?:Readonly<Record<string,string|undefined>>;signal?:AbortSignal;policy?:GraphFarmPolicyV1;accept?:(kind:string,identity:unknown)=>boolean;report?:(event:"recycled"|"disabled",fields:Readonly<Record<string,string|number>>)=>void;spawnProcess?:(command:string,args:readonly string[],options:SpawnOptions)=>ChildProcess;runtime?:string;command?:string;commandArguments?:readonly string[];workerMegabytes?:number}
export interface FarmStatsV1{workers:number;jobs:number;replies:number;failures:number;lost:number;recycled:number;started:number;fallbacks:number;peakRssMegabytes:number;opens:number;restarts:number;loadedAfterMilliseconds?:number;firstJobAfterMilliseconds?:number;disabled?:string}
export type FarmOutcomeV1={reply:Record<string,unknown>&{rssMegabytes:number}}|{failure:{name:string;message:string;code:string|null}}|{fallback:string};
export interface FarmV1{readonly width:number;readonly closed:boolean;stats():FarmStatsV1;prestart():void;limit(count:number):void;open(kind:string,session:unknown,options?:{accept?:(kind:string,identity:unknown)=>boolean;service?:string}):boolean;infer(kind:string,job:unknown|((where:{slot:number;generation:number})=>unknown),options?:{signal?:AbortSignal}):Promise<FarmOutcomeV1>;close():Promise<void>}
export type GraphFarmV1=FarmV1;
/** 🏊️ A bounded pool of worker processes that answers every job with data or lets the caller infer it itself; it boots early, opens a session per service kind and serves several builds. */
export function createFarmV1(options:FarmOptionsV1):FarmV1;
/** 🏊️ Leases the resident pool of one key; the last release keeps its processes for the idle time of the policy. */
export function acquireGraphFarmV1(options:FarmOptionsV1&Readonly<{key:string}>):Readonly<{farm:FarmV1;release():void}>;
/** 🧹️ Closes every resident pool now. */
export function closeGraphFarmsV1():Promise<void>;
