import type{FootprintGateV1,FootprintPolicyV1}from"../🦶️footprint/🟨️.mjs";
export interface WaveAdmissionPolicyV1{readonly version:1;readonly holdPressure:"normal"|"warn"|"critical";readonly swapOutMegabytesPerSecond:number;readonly availableFloorMegabytes:number;readonly sampleTtlMilliseconds:number;readonly sampleStaleMilliseconds:number;readonly stepMilliseconds:number;readonly progressMilliseconds:number;readonly settleMinimumMilliseconds:number;readonly overGrantMaxWaitMilliseconds:number;readonly hostMaxWaitMilliseconds:number;readonly reducedWidth:number;readonly languages:readonly string[]}
export declare const WAVE_ADMISSION_POLICY_V1:WaveAdmissionPolicyV1;
export type WaveAdmissionMessageKeyV1="overGrant"|"hostBusy"|"reduced"|"resumed";
export declare const WAVE_ADMISSION_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<WaveAdmissionMessageKeyV1,string>>>>;
export function renderWaveAdmissionMessageV1(key:WaveAdmissionMessageKeyV1,fields:Readonly<Record<string,string|number>>,environment?:Readonly<Record<string,string|undefined>>):string[];
export interface HostPressureV1{readonly atMs?:number;readonly pressure:"normal"|"warn"|"critical";readonly availableMegabytes:number;readonly swapUsedMegabytes:number|null;readonly swapOutMegabytesPerSecond:number|null}
export type WaveAdmissionDecisionV1="admit"|"overGrant"|"hostBusy";
export function decideWaveAdmissionV1(input:{footprintBytes:number;grantBytes:number|undefined;host:HostPressureV1|undefined;footprintPolicy?:FootprintPolicyV1;policy?:WaveAdmissionPolicyV1}):WaveAdmissionDecisionV1;
export interface HostPressureReaderV1{current():HostPressureV1|undefined;refresh():Promise<HostPressureV1|undefined>}
export function createHostPressureReaderV1(options?:{read?:(previous:{atMs:number;swappedOutMegabytes:number|null}|undefined)=>Promise<{host:HostPressureV1;swappedOutMegabytes:number|null}>;now?:()=>number;ttlMilliseconds?:number;staleMilliseconds?:number}):HostPressureReaderV1;
export interface WaveAdmissionStatsV1{readonly holds:number;readonly waits:number;readonly waitedMilliseconds:number;readonly settles:number;readonly reduced:number}
export interface WaveAdmissionV1{holds():WaveAdmissionDecisionV1;narrow():boolean;between(signal?:AbortSignal):Promise<"admit"|"reduced">;stats():WaveAdmissionStatsV1}
export function createWaveAdmissionV1(options?:{gate?:()=>Pick<FootprintGateV1,"grantBytes"|"footprintBytes"|"settle">|undefined;host?:HostPressureReaderV1;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;now?:()=>number;report?:(line:string)=>void;environment?:Readonly<Record<string,string|undefined>>;policy?:WaveAdmissionPolicyV1}):WaveAdmissionV1;
export function waveAdmissionV1():WaveAdmissionV1;
