import {requireHostCapacityV1,type HostCapacityPolicyV1,type HostRequestV1} from "../📏️decision/🟦️.ts";
import {estimateHostRatioV1,hostSampleWeightsV1,recordHostOutcomeV1,type HostEstimateStoreV1} from "../📐️estimates/🟦️.ts";

const UNIT=1_000_000;

/** 🌦️ What the door expects of a run before it starts: `quiet` when the class of the run has mostly executed nothing so far (a fully cached or covered plan), the small reservation such a run has needed (`megabytes`) and the time it needs to reach it, the effective quiet runs behind the forecast and the share of quiet runs among the recent ones, in basis points; `followUpMilliseconds` (only when the follow-up rule alone made the forecast quiet) is the time since the earlier run of the class finished. */
export type HostForecastV1=Readonly<{quiet:boolean;megabytes:number;rampMilliseconds:number;samples:number;shareBasisPoints:number;followUpMilliseconds?:number}>;
/** 🏁️ What a finished run of a forecast-aware operation reports: whether any work had to run (the grant was upgraded, or it was the full grant that the executor confirmed), and the peak its process tree held. */
export type HostForecastRunV1=Readonly<{executed:boolean;peakMegabytes:number;activeMilliseconds:number;peakAtMilliseconds:number;endedAtMs:number}>;

/** 🏷️ The two classes below a class: the runs that executed nothing and the runs that had to execute work; the forecast reads their decayed weights, the quiet class also teaches the size of a quiet run. */
export function hostForecastClassesV1(leaf:string):Readonly<{quiet:string;busy:string}>{return Object.freeze({quiet:`${leaf}~quiet`,busy:`${leaf}~busy`});}

/** 🔮️ Forecasts whether the next run of a class executes nothing: it does when the earlier run of the class finished at most `quietFollowUpMilliseconds` ago (a run that ended well left its results cached, so the run that follows it is a replay unless the world changed in between: the follow-up rule, which also starts a class that has no quiet history yet; a wrong forecast costs nothing because the upgrade protects, a missed one costs the whole queue wait) or when enough decayed quiet runs exist (`quietMinimumSamples`), they are at least `quietShareBasisPoints` of the quiet and busy runs together, the newest of all runs was quiet (one busy run flips the forecast at once, so the run after an edit asks for the full grant) and the learned quiet size stays within `quietMaximumMegabytes`. The size is the quiet class's own estimate (decayed quantile of the peaks plus margin, trust-blended with the reference) against `quietReferenceMegabytes`; a forecast never changes what a run may use, only what it must reserve before it knows. */
export function forecastHostQuietV1(policy:HostCapacityPolicyV1,store:HostEstimateStoreV1,leaf:string,nowMs:number):HostForecastV1{
 const rules=policy.estimates,classes=hostForecastClassesV1(leaf),quiet=store.classes[classes.quiet]?.samples??[],busy=store.classes[classes.busy]?.samples??[];
 const quietWeight=hostSampleWeightsV1(policy,quiet,nowMs).reduce((sum,row)=>sum+row,0),busyWeight=hostSampleWeightsV1(policy,busy,nowMs).reduce((sum,row)=>sum+row,0),total=quietWeight+busyWeight;
 const share=total===0?0:Math.floor(quietWeight*10000/total+0.5),newest=(rows:typeof quiet)=>rows.reduce((latest,row)=>Math.max(latest,row.atMs),-1);
 const estimate=estimateHostRatioV1(policy,store,[classes.quiet],nowMs),megabytes=Math.floor(estimate.ratioBasisPoints*rules.quietReferenceMegabytes/10000+0.5);
 const history=quietWeight>=rules.quietMinimumSamples*UNIT&&share>=rules.quietShareBasisPoints&&newest(quiet)>newest(busy),since=nowMs-Math.max(newest(quiet),newest(busy)),followUp=!history&&rules.quietFollowUpMilliseconds>0&&Math.max(newest(quiet),newest(busy))>=0&&since>=0&&since<=rules.quietFollowUpMilliseconds,quiet_=(history||followUp)&&megabytes<=rules.quietMaximumMegabytes;
 return Object.freeze({quiet:quiet_,megabytes,rampMilliseconds:estimate.rampMilliseconds,samples:estimate.samples,shareBasisPoints:share,...quiet_&&followUp?{followUpMilliseconds:since}:{}});
}

/** ✍️ Records one finished run under the class it belongs to: the quiet class when it executed nothing, the busy class otherwise, both as a peak against `quietReferenceMegabytes` so the quiet size is an absolute number; a pure function of its arguments. */
export function recordHostForecastV1(policy:HostCapacityPolicyV1,store:HostEstimateStoreV1,leaf:string,run:HostForecastRunV1):HostEstimateStoreV1{
 const classes=hostForecastClassesV1(leaf);
 return recordHostOutcomeV1(policy,store,[run.executed?classes.busy:classes.quiet],{operation:"run",jobs:1,peakMegabytes:run.peakMegabytes,defaultMegabytes:policy.estimates.quietReferenceMegabytes,activeMilliseconds:run.activeMilliseconds,peakAtMilliseconds:run.peakAtMilliseconds,endedAtMs:run.endedAtMs});
}

/** 🪶️ The small request of a forecast-quiet run: the learned quiet size as one fixed block of one job under the id (and label) of the full request, marked `quiet` so it passes the queue barrier, with the time its class needs to reach that size and no ramp slot (it is bounded by `memory.maximumQuiet` instead). */
export function quietHostRequestV1(full:HostRequestV1,forecast:HostForecastV1):HostRequestV1{
 return requireHostCapacityV1<HostRequestV1>("Request",Object.freeze({id:full.id,operation:full.operation,...full.label===undefined?{}:{label:full.label},priority:full.priority,minimumJobs:1,preferredJobs:1,fixedMegabytes:forecast.megabytes,perJobMegabytes:0,cpuPerJob:1,rampMilliseconds:forecast.rampMilliseconds,rampSlot:false,quiet:true}));
}
