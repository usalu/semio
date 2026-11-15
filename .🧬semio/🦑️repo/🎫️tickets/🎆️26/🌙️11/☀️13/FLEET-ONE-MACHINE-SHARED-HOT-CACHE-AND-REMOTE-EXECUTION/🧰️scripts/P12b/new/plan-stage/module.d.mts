/** 🪜️ The declared limits and stages of a graph plan. */
export interface PlanStagePolicyV1{readonly version:1;readonly hostShare:number;readonly minimumMegabytes:number;readonly maximumMegabytes:number;readonly serveHostMegabytes:number;readonly limitEnvironment:string;readonly off:string;readonly owner:string;readonly stages:readonly string[];readonly stageMegabytes:Readonly<Record<string,number>>;readonly sampleMilliseconds:number;readonly recentSamples:number}
export declare const PLAN_STAGE_POLICY_V1:PlanStagePolicyV1;
export type PlanStageMessageKeyV1="started"|"finished"|"stopped"|"detail";
export declare const PLAN_STAGE_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<PlanStageMessageKeyV1,string>>>>;
/** 📡️ The message a slice worker sends to its service when a plan stage starts. */
export interface PlanStageEventV1{readonly type:"stage";readonly stage:string;readonly operation:string;readonly managedMegabytes:number;readonly residentMegabytes:number}
export function onPlanStageV1(listener:(event:PlanStageEventV1)=>void):()=>boolean;
export function renderPlanStageMessageV1(key:PlanStageMessageKeyV1,fields:Readonly<Record<string,string|number>>,environment:Readonly<Record<string,string|undefined>>):string[];
export function planStageLimitBytesV1(environment?:Readonly<Record<string,string|undefined>>,hostMegabytes?:number,policy?:PlanStagePolicyV1):number;
export interface PlanStageGuardV1{mark(stage:string):void;operation(name:string):void;run<T>(work:()=>Promise<T>|T):Promise<T>}
export function createPlanStageGuardV1(options?:{limitBytes?:number;ports?:{sample:()=>number;collect:()=>void};environment?:Readonly<Record<string,string|undefined>>;report?:(line:string)=>void;now?:()=>number;resident?:()=>number;usage?:()=>Readonly<Record<string,number>>;heap?:()=>Readonly<Record<string,any>>|undefined;resources?:()=>readonly string[]|undefined;stageMegabytes?:Readonly<Record<string,number>>;refuse?:(detail:{stage:string;operation:string;megabytes:number;limit:number})=>Error}):PlanStageGuardV1;
