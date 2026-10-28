/** ⏱️ The profile output mode. */
export type RepoProfileModeV1="off"|"table"|"json";
/** 🛑️ One finished phase row. */
export interface RepoProfileRowV1{readonly seq:number;readonly id:number;readonly parent:number;readonly depth:number;readonly phase:string;readonly start:number;readonly duration:number;readonly self:number;readonly detail?:string}
/** 🔚️ A finished profile: totals in milliseconds and the ordered rows. */
export interface RepoProfileSummaryV1{readonly version:1;readonly unit:"ms";readonly total:number;readonly covered:number;readonly uncovered:number;readonly rows:readonly RepoProfileRowV1[]}
export interface RepoProfileRecordV1{id:number;parent:number;depth:number;phase:string;start:number;end:number;detail?:string}
export interface RepoProfileOptionsV1{mode?:RepoProfileModeV1;now?:()=>number;pid?:number;write?:(text:string)=>unknown}
export interface RepoProfileV1{readonly enabled:boolean;readonly mode:RepoProfileModeV1;phase<T>(phase:string,body:()=>T,detail?:string):T;event(phase:string,durationMilliseconds:number,detail?:string):void;mark(phase:string,detail?:string):void;summary():RepoProfileSummaryV1;flush():void}
export const REPO_PROFILE_VARIABLE_V1:"SEMIO_REPO_PROFILE";
export const REPO_PROFILE_PHASE_PATTERN_V1:RegExp;
export const REPO_PROFILE_LEGEND_V1:Readonly<Record<"seq"|"depth"|"phase"|"start"|"duration"|"self"|"detail",Readonly<{en:string;de:string}>>>;
export function readRepoProfileModeV1(environment?:Readonly<Record<string,string|undefined>>):RepoProfileModeV1;
export function summarizeRepoProfileV1(records:readonly RepoProfileRecordV1[],totalMilliseconds:number):RepoProfileSummaryV1;
export function renderRepoProfileTableV1(summary:RepoProfileSummaryV1,pid:number):string;
export function createRepoProfileV1(options?:RepoProfileOptionsV1):RepoProfileV1;
export function repoProfileV1():RepoProfileV1;
export function resetRepoProfileV1():void;
export function profilePhaseV1<T>(phase:string,body:()=>T,detail?:string):T;
export function profileMarkV1(phase:string,detail?:string):void;
export function profileCurrentPhaseV1():string|undefined;
export function installRepoProfileSpawnCensusV1(profile:RepoProfileV1):void;
export function repoProfileSpawnLabelV1(command:unknown):string;
