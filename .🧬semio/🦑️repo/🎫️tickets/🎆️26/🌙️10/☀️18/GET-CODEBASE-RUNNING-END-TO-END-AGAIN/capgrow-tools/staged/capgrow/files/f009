import {availableParallelism}from"node:os";
import {validateJsonSchemaSubset}from"../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {currentHostProbeV1,parseDarwinCountersV1,readHostCountersV1,type HostProbeV1}from"../../🌡️host-capacity/📡️sampling/🟦️.ts";
import {readLiveHostCapacityPlanV1}from"../../🌡️host-capacity/🎫️gate/🟦️.ts";
import type{CompilerCapacityRequestV1}from"../🎫️lease/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const MEGABYTE=1048576,policy=Object.freeze({...schema.definitions.Policy.const});
/** 🧮️ Why a compile was planned below the share of the host: the tightest limit that applied. */
export type CompileUnitsReasonV1="share"|"needed"|"granted"|"memory"|"pressure-warn"|"pressure-critical";
/** 🌡️ The live host facts one plan reads; `null` marks a fact the platform does not offer (no limit from it): the jobs the enclosing machine admission holds now and the memory its jobs may hold (`grantedMegabytes`), and the measured footprint of one compile unit (`unitMegabytes`, `compileUnitMegabytesV1`). */
export type CompileUnitsHostV1=Readonly<{cores:number;availableMegabytes:number|null;pressure:"normal"|"warn"|"critical"|null;grantedJobs:number|null;grantedMegabytes:number|null;unitMegabytes:number}>;
/** 🎯️ What the work can use at most (`null`: any number of cores). */
export type CompileUnitsIntentV1=Readonly<{needed:number|null}>;
/** ⚖️ The planned units: ask for up to `preferred` cores, start as soon as `minimum` are free; `budgetMegabytes` is the memory the compile may hold under its machine admission (null without one), which bounds the memory-admitting compiler broker. */
export type CompileUnitsDecisionV1=Readonly<{preferred:number;minimum:number;reason:CompileUnitsReasonV1;budgetMegabytes:number|null}>;

/** 🛂️ Requires one closed definition of the compile units schema. */
function requireDefinition<T>(definition:"Host"|"Intent"|"Decision",value:unknown):T{
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value);
 if(errors.length)throw Error(fill(schema.definitions.Messages.const.en.errors.definitionRequired,{detail:errors[0]!}));
 return value as T;
}
function fill(text:string,values:Readonly<Record<string,string|number>>):string{return text.replace(/\{([a-z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);}

/** 🧩️ The share of the cores one compile may use and the number it needs free to start, from the core count alone (the plan before any live limit). */
export function compileUnitsShareV1(cores:number):Readonly<{preferred:number;minimum:number}>{
 if(!Number.isSafeInteger(cores)||cores<1)throw Error(fill(schema.definitions.Messages.const.en.errors.definitionRequired,{detail:"cores"}));
 const preferred=Math.max(1,Math.floor(cores*policy.shareBasisPoints/10000));
 return Object.freeze({preferred,minimum:Math.max(1,Math.ceil(preferred*policy.minimumShareBasisPoints/10000))});
}

/** 🧱️ A short counted slot for a tool whose own worker pool cannot be bounded by the environment (Bun's bundler ignores `GOMAXPROCS`): the authored fixed number of units of its class (`fixedClasses`), never more than half the host, queued like every other compile, so it no longer reserves the whole host. */
export function fixedCompilerCapacityRequestV1(name:keyof typeof policy.fixedClasses,cores:number=availableParallelism()):CompilerCapacityRequestV1{
 const row=policy.fixedClasses[name];if(row===undefined)throw Error(fill(schema.definitions.Messages.const.en.errors.definitionRequired,{detail:String(name)}));
 const maximum=Math.min(row.maximumUnits,compileUnitsShareV1(cores).preferred);
 return Object.freeze({kind:"available" as const,maximumParallelism:maximum,minimumParallelism:Math.min(row.minimumUnits,maximum)});
}

/** 🎟️ The counted capacity request of a tool whose worker pool follows `GOMAXPROCS`/`SEMIO_COMPILER_PARALLELISM` (bundlers, Go, Vite, uv, wasm tooling): the share of the cores, started once half of it is free, so such a tool runs beside others instead of reserving the whole host. */
export function countedCompilerCapacityRequestV1(cores:number=availableParallelism()):CompilerCapacityRequestV1{
 const share=compileUnitsShareV1(cores);
 return Object.freeze({kind:"available" as const,maximumParallelism:share.preferred,minimumParallelism:share.minimum});
}

/** 📏️ The measured footprint of one compile unit: the `unitQuantileBasisPoints` quantile (nearest rank) of the newest peak of every unit the compiler broker recorded, never below `minimumUnitMegabytes`; the authored `megabytesPerUnit` until `minimumUnitSamples` units were measured.
 * @see ./🧪️tests/🐍️oracle/🐍️.py */
export function compileUnitMegabytesV1(history:ReadonlyMap<string,readonly Readonly<{peakRssBytes:number}>[]>=new Map()):number{
 const peaks=[...history.values()].filter(rows=>rows.length>0).map(rows=>rows[rows.length-1]!.peakRssBytes).sort((left,right)=>left-right);
 if(peaks.length<policy.minimumUnitSamples)return policy.megabytesPerUnit;
 return Math.max(policy.minimumUnitMegabytes,Math.ceil(peaks[Math.max(0,Math.ceil(peaks.length*policy.unitQuantileBasisPoints/10000)-1)]!/MEGABYTE));
}

/** 🔨️ Plans counted compile capacity from the live host: one compile may use `shareBasisPoints` of the cores (so at least two run side by side), never more than its work can use, than the machine admission carries (its granted jobs, or more when the memory its jobs may hold carries more compile units at the measured footprint of one: a compile is one action that runs several units), than the free memory carries (`unitMegabytes` each above the reserve), one core under critical pressure and half under raised pressure; it starts once `minimumShareBasisPoints` of that are free. Under a machine admission the compile may hold the memory of its grant, and never less than its planned units need at the measured footprint (`budgetMegabytes`).
 * @see ./🧬️schema/🔣️.json
 * @see ./🧪️tests/🐍️oracle/🐍️.py */
export function planCompileUnitsV1(hostValue:CompileUnitsHostV1,intentValue:CompileUnitsIntentV1={needed:null}):CompileUnitsDecisionV1{
 const host=requireDefinition<CompileUnitsHostV1>("Host",hostValue),intent=requireDefinition<CompileUnitsIntentV1>("Intent",intentValue);
 let units=compileUnitsShareV1(host.cores).preferred,reason:CompileUnitsReasonV1="share";
 const limit=(value:number,why:CompileUnitsReasonV1)=>{if(value<units){units=Math.max(1,value);reason=why;}};
 if(intent.needed!==null)limit(intent.needed,"needed");
 if(host.grantedJobs!==null)limit(Math.max(host.grantedJobs,host.grantedMegabytes===null?0:Math.floor(host.grantedMegabytes/host.unitMegabytes)),"granted");
 if(host.availableMegabytes!==null)limit(Math.floor(Math.max(0,host.availableMegabytes-policy.reserveMegabytes)/host.unitMegabytes),"memory");
 if(host.pressure==="critical")limit(1,"pressure-critical");
 else if(host.pressure==="warn")limit(Math.ceil(units/policy.pressureWarnDivisor),"pressure-warn");
 return Object.freeze(requireDefinition<CompileUnitsDecisionV1>("Decision",{preferred:units,minimum:Math.max(1,Math.ceil(units*policy.minimumShareBasisPoints/10000)),reason,budgetMegabytes:host.grantedMegabytes===null?null:Math.max(1,host.grantedMegabytes,units*host.unitMegabytes)}));
}

/** 📬️ Turns a plan into the lease request of the counted capacity: bounded cores, started at `minimum`. */
export function compileUnitsRequestV1(decision:CompileUnitsDecisionV1):CompilerCapacityRequestV1{
 const plan=requireDefinition<CompileUnitsDecisionV1>("Decision",decision);
 return Object.freeze({kind:"available" as const,maximumParallelism:plan.preferred,minimumParallelism:plan.minimum});
}

/** 🔌️ Reads the live host facts: cores, free memory and memory pressure of the platform probe, the jobs and job memory the enclosing machine admission holds now (its grant row, so a run that grew after this process started is seen grown) and the unit footprint measured in the compiler broker's `history`. A counter that cannot be read stays `null`, no fact, no limit. */
export async function readCompileUnitsHostV1(options:Readonly<{environment?:Readonly<Record<string,string|undefined>>;signal?:AbortSignal;probe?:HostProbeV1;cores?:number;history?:ReadonlyMap<string,readonly Readonly<{peakRssBytes:number}>[]>}>={}):Promise<CompileUnitsHostV1>{
 const environment=options.environment??process.env,plan=readLiveHostCapacityPlanV1(environment);let counters:Awaited<ReturnType<typeof readHostCountersV1>>|undefined;
 try{
  const probe=options.probe??currentHostProbeV1(options.signal);
  counters=probe.platform==="darwin"?parseDarwinCountersV1(...await Promise.all([probe.run("vm_stat",[],probe.signal),probe.run("sysctl",["-n","vm.swapusage"],probe.signal),probe.run("sysctl",["-n","kern.memorystatus_vm_pressure_level"],probe.signal)])):await readHostCountersV1(probe);
 }catch(error){if(options.signal?.aborted)throw options.signal.reason;}
 const available=counters?.availableBytes===null||counters?.availableBytes===undefined?null:Math.floor(Math.min(counters.availableBytes,counters.constrainedBytes??Infinity)/MEGABYTE);
 return Object.freeze({cores:options.cores??availableParallelism(),availableMegabytes:available,pressure:counters?.pressureLevel??null,grantedJobs:plan?.jobs??null,grantedMegabytes:plan?.memoryMegabytes??null,unitMegabytes:compileUnitMegabytesV1(options.history)});
}

/** 🚦️ Plans and requests in one step from the live host: the request the compile hands to the compiler capacity scope and the plan it logs. */
export async function compileUnitsForHostV1(options:Readonly<{environment?:Readonly<Record<string,string|undefined>>;signal?:AbortSignal;needed?:number;probe?:HostProbeV1;cores?:number;history?:ReadonlyMap<string,readonly Readonly<{peakRssBytes:number}>[]>}>={}):Promise<Readonly<{host:CompileUnitsHostV1;decision:CompileUnitsDecisionV1;request:CompilerCapacityRequestV1}>>{
 const host=await readCompileUnitsHostV1(options),decision=planCompileUnitsV1(host,{needed:options.needed??null});
 return Object.freeze({host,decision,request:compileUnitsRequestV1(decision)});
}

/** 🖨️ Describes a plan, a wait or a grant in the given language (English first, German second); unknown placeholders stay visible. */
export function describeCompileUnitsV1(language:"en"|"de",name:"planned"|"waiting"|"granted",values:Readonly<Record<string,string|number>>):string{
 const catalogue=schema.definitions.Messages.const[language],reason=values.reason;
 return fill(catalogue[name],{...values,...typeof reason==="string"&&reason in catalogue.reasons?{reason:catalogue.reasons[reason as keyof typeof catalogue.reasons]}:{}});
}

/** 🔖️ Refuses a grant above what was planned: the cargo jobs flag, the jobserver tokens and the memory share all follow the granted units, so a larger number would overbook the host. */
export function requireGrantedCompileUnitsV1(decision:CompileUnitsDecisionV1,granted:number):number{
 if(!Number.isSafeInteger(granted)||granted<decision.minimum||granted>decision.preferred)throw Error(schema.definitions.Messages.const.en.errors.grantedMoreThanPlanned);
 return granted;
}
