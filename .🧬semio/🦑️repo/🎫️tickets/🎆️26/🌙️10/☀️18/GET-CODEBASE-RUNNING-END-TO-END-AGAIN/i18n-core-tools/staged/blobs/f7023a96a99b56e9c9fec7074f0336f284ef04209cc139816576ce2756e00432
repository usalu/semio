import {compileJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {generationCanonicalJsonV1,generationSha256V1} from "../🧾️generation/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import generationSchema from "../🧾️generation/🧬️schema/🔣️.json" with {type:"json"};
import {declareFormatStampV1} from "../../../../🏗️build/🧷️format-stamps/🟨️.mjs";

declareFormatStampV1("dependency.walk",generationSchema);

const protocol=schema.$defs.Protocol.const,texts=schema.$defs.Texts.const as Readonly<Record<string,Readonly<Record<string,string>>>>,schemaOf=(name:string)=>compileJsonSchemaSubset((schema.$defs as Record<string,unknown>)[name] as never,schema as never);
const pointerSchema=schemaOf("Pointer"),eventSchema=schemaOf("Event");

/** 🚦️ Names the closed neutral contract (the schema) every concurrent dependency phase, lease name, pointer, retention rule and text follows. */
export const concurrentProtocolV1=protocol;
/** 🔑️ Is the closed set of leases a dependency build coordinates with its readers. */
export type ConcurrentLeaseKindV1="build"|"bindings"|"writer";
/** 🧭️ Is the closed set of phases a dependency operation passes through. */
export type ConcurrentPhaseV1=keyof typeof protocol.phases;

/** 🏷️ Names one workspace lease; the bindings gate keeps the name the readers already hold shared. */
export function concurrentLeaseResourceV1(kind:ConcurrentLeaseKindV1,workspace:string):string{
 if(kind!=="build"&&kind!=="bindings"&&kind!=="writer")throw Error("Closed concurrent lease kind required");
 if(typeof workspace!=="string"||!workspace.length||workspace.includes("\0"))throw Error("Concrete workspace root required");
 return protocol.resources[kind]+workspace;
}
/** 📌️ Names the lease a reader holds shared on one generation and a collector tries exclusively before removing it. */
export function concurrentPinResourceV1(workspace:string,generation:string):string{
 const prefix=generation.slice(0,20);
 if(!/^[a-f0-9]{20}$/u.test(prefix)||typeof workspace!=="string"||!workspace.length)throw Error("Concrete generation prefix required");
 return protocol.resources.pin+workspace+":"+prefix;
}
/** 📌️ Reduces generations (complete, or already reduced to their physical directory name) to the sorted distinct names a holder pins. */
export function concurrentPinNamesV1(generations:readonly string[]):readonly string[]{
 const names=new Set<string>();
 for(const generation of generations){if(!/^[a-f0-9]{20}(?:[a-f0-9]{44})?$/u.test(generation))throw Error("Complete generation required");names.add(generation.slice(0,20));}
 return Object.freeze([...names].sort());
}

/** 📍️ Is one atomically replaced record of which immutable generation a scope currently publishes and what it retired when. */
export type ConcurrentPointerV1=Readonly<{version:1;purpose:"dependency-generation-pointer";scope:string;generation:string;epoch:number;publishedAtMs:number;bytes:number;retired:readonly Readonly<{generation:string;atMs:number}>[];sha256:string}>;
const sealPointer=(body:Omit<ConcurrentPointerV1,"sha256">):ConcurrentPointerV1=>Object.freeze({...body,sha256:generationSha256V1(generationCanonicalJsonV1(body))});
/** 📛️ Names the file of one scope's pointer without trusting the scope text as a path. */
export function concurrentPointerNameV1(scope:string):string{
 if(typeof scope!=="string"||!scope.length||scope.length>256)throw Error("Bounded scope required");
 return generationSha256V1(scope).slice(0,protocol.pointer.scopeKeyUnits)+protocol.pointer.suffix;
}
/** ➡️ Seals the pointer that follows `previous` when a scope switches to `generation` of `bytes` physical size (0 when unknown); republishing the same generation changes nothing and the clock never moves backwards. */
export function sealConcurrentPointerV1(previous:ConcurrentPointerV1|null,scope:string,generation:string,nowMs:number,bytes:number):ConcurrentPointerV1{
 if(!/^[a-f0-9]{64}$/u.test(generation)||!Number.isSafeInteger(nowMs)||nowMs<0||!Number.isSafeInteger(bytes)||bytes<0)throw Error("Complete generation, clock and size required");
 if(previous&&previous.scope!==scope)throw Error("Pointer of the same scope required");
 if(previous&&previous.generation===generation)return previous;
 const at=Math.max(nowMs,previous?.publishedAtMs??0);
 const retired=previous?[...previous.retired.filter(row=>row.generation!==generation&&row.generation!==previous.generation),{generation:previous.generation,atMs:at}]:[];
 const kept=retired.sort((left,right)=>left.atMs-right.atMs||Buffer.compare(Buffer.from(left.generation),Buffer.from(right.generation))).slice(-protocol.pointer.maximumRetired);
 return sealPointer({version:1,purpose:"dependency-generation-pointer",scope,generation,epoch:(previous?.epoch??0)+1,publishedAtMs:at,bytes,retired:Object.freeze(kept.map(row=>Object.freeze({...row})))});
}
/** 🔓️ Admits only a bounded, schema-valid, correctly sealed pointer. */
export function parseConcurrentPointerV1(bytes:Uint8Array):ConcurrentPointerV1{
 if(bytes.length>protocol.pointer.maximumBytes)throw Error("Bounded dependency pointer required");
 const value=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(bytes))as ConcurrentPointerV1&Record<string,unknown>,errors=pointerSchema.validate(value);
 if(errors.length)throw Error("Closed dependency pointer required",{cause:errors.slice(0,4)});
 const{sha256,...body}=value;
 if(generationSha256V1(generationCanonicalJsonV1(body))!==sha256)throw Error("Sealed dependency pointer required");
 return Object.freeze(value);
}
/** 💾️ Encodes one pointer as the exact bytes a publisher replaces atomically. */
export const encodeConcurrentPointerV1=(pointer:ConcurrentPointerV1):Buffer=>Buffer.from(JSON.stringify(pointer)+"\n");

/** 🗂️ Is one unreferenced physical generation directory with the time it last changed. */
export type ConcurrentRowV1=Readonly<{name:string;modifiedAtMs:number}>;
/** 🧹️ Is what a collector removes now and what it keeps with the reason. */
export type ConcurrentCollectionV1=Readonly<{collect:readonly string[];keep:readonly Readonly<{name:string;reason:"grace"|"rollback"}>[]}>;
/** ⏳️ Decides which unreferenced generations may be removed: a generation is kept for the grace period after it stopped being current (for the rollback period when it is the newest retired one of its scope), both shrink to the pressure grace while the image is nearly full or cannot hold the generation about to be built, and pinned generations are left to the caller's exclusive pin attempt. */
export function concurrentCollectionV1(input:Readonly<{rows:readonly ConcurrentRowV1[];pointers:readonly ConcurrentPointerV1[];nowMs:number;freeFraction:number;freeBytes?:number;neededBytes?:number}>):ConcurrentCollectionV1{
 const policy=protocol.retention,pressure=input.freeFraction<policy.pressureFreeFraction||(input.neededBytes!==undefined&&input.freeBytes!==undefined&&input.freeBytes<input.neededBytes),retiredAt=new Map<string,number>(),rollback=new Set<string>();
 for(const pointer of input.pointers){
  for(const row of pointer.retired)retiredAt.set(row.generation.slice(0,20),Math.max(retiredAt.get(row.generation.slice(0,20))??0,row.atMs));
  const newest=[...pointer.retired].sort((left,right)=>right.atMs-left.atMs)[0];
  if(newest)rollback.add(newest.generation.slice(0,20));
 }
 const rows=input.rows.map(row=>{const since=retiredAt.get(row.name)??row.modifiedAtMs,age=Math.max(0,input.nowMs-since),isRollback=rollback.has(row.name),needed=pressure?policy.pressureGraceMilliseconds:isRollback?Math.max(policy.graceMilliseconds,policy.rollbackMilliseconds):policy.graceMilliseconds;return{name:row.name,since,age,needed,isRollback};}).sort((left,right)=>left.since-right.since||Buffer.compare(Buffer.from(left.name),Buffer.from(right.name)));
 const due=rows.filter(row=>row.age>=row.needed),collect=due.slice(0,policy.maximumCollectedPerRun).map(row=>row.name),keep=rows.filter(row=>row.age<row.needed).map(row=>Object.freeze({name:row.name,reason:row.isRollback?"rollback" as const:"grace" as const}));
 return Object.freeze({collect:Object.freeze(collect),keep:Object.freeze(keep)});
}

/** ⏱️ Is what one waiting round does: whether the waiter has waited too long, asks whether the work became unnecessary and reports progress. */
export type ConcurrentWaitStepV1=Readonly<{expired:boolean;recheck:boolean;progress:boolean}>;
/** 🔁️ Schedules a politely waiting acquisition: it probes the lease every round, rechecks readiness every few seconds and reports progress on its own cadence. */
export function concurrentWaitStepV1(state:Readonly<{elapsedMs:number;sinceRecheckMs:number;sinceProgressMs:number;canRecheck:boolean;maximumMs:number}>):ConcurrentWaitStepV1{
 const wait=protocol.wait;
 return Object.freeze({expired:state.elapsedMs>=state.maximumMs,recheck:state.canRecheck&&state.sinceRecheckMs>=wait.recheckMilliseconds,progress:state.sinceProgressMs>=wait.progressMilliseconds});
}

/** 🗣️ Renders one closed message in an explicit language; there is no hidden default language. */
export function concurrentTextV1(language:string,name:string,values:Readonly<Record<string,string|number>>={}):string{
 const table=texts[language],template=table?.[name];
 if(!table||typeof template!=="string")throw Error("Declared language and message required");
 return Object.entries(values).reduce((text,[key,value])=>text.replaceAll("{"+key+"}",String(value)),template);
}
/** 📣️ Is one language-neutral concurrency event. */
export type ConcurrentEventV1=Readonly<{version:1;purpose:"dependency-concurrency-event";event:(typeof protocol.events)[number];resource?:ConcurrentLeaseKindV1;elapsedMs?:number;scope?:string;generation?:string;epoch?:number;count?:number;neededBytes?:number;freeBytes?:number;reason?:string}>;
/** 📣️ Admits only a schema-valid event. */
export function concurrentEventV1(body:Omit<ConcurrentEventV1,"version"|"purpose">):ConcurrentEventV1{
 const value={version:1 as const,purpose:"dependency-concurrency-event" as const,...body},errors=eventSchema.validate(value);
 if(errors.length)throw Error("Closed dependency concurrency event required",{cause:errors.slice(0,4)});
 return Object.freeze(value);
}
/** 🧮️ Is the admission verdict for building one more generation: how many bytes it needs and whether the free space holds them. */
export type ConcurrentAdmissionV1=Readonly<{neededBytes:number;requiredBytes:number;freeBytes:number;admitted:boolean}>;
/** 🛂️ Admits one more generation of `scope` only when the free bytes hold its expected size plus the declared margin; the expectation is the size recorded when the scope was last published, else the largest recorded size, else the declared default. */
export function concurrentAdmissionV1(input:Readonly<{scope:string;pointers:readonly ConcurrentPointerV1[];freeBytes:number}>):ConcurrentAdmissionV1{
 const policy=protocol.admission,own=input.pointers.find(pointer=>pointer.scope===input.scope)?.bytes??0,largest=input.pointers.reduce((size,pointer)=>Math.max(size,pointer.bytes),0),neededBytes=own>0?own:largest>0?largest:policy.defaultGenerationBytes,requiredBytes=Math.ceil(neededBytes*policy.marginPercent/100);
 if(!Number.isSafeInteger(input.freeBytes)||input.freeBytes<0)throw Error("Measured free bytes required");
 return Object.freeze({neededBytes,requiredBytes,freeBytes:input.freeBytes,admitted:input.freeBytes>=requiredBytes});
}

/** 📐️ Is the physical size the dependencies partition must declare so it holds the package cache, scratch space and the installed generation set plus one building set with headroom; a linking install backend shares data with the cache, a copying one does not. */
export function concurrentRequiredImageBytesV1(sharing:boolean):number{
 const budget=protocol.budget,generation=sharing?budget.physicalGenerationBytes:budget.copiedGenerationBytes;
 return Math.ceil((budget.packageCacheBytes+budget.tmpReserveBytes+2*budget.scopes*generation)*(100+budget.headroomPercent)/100);
}
/** 🧽️ Names the scratch directories no process has touched for the stale period (shortened under pressure), judged by name and modification time alone. */
export function concurrentStaleTemporariesV1(rows:readonly ConcurrentRowV1[],nowMs:number,pressure:boolean):readonly string[]{
 const policy=protocol.temporaries,pattern=new RegExp(policy.namePattern,"u"),limit=pressure?policy.pressureStaleMilliseconds:policy.staleMilliseconds;
 return Object.freeze(rows.filter(row=>pattern.test(row.name)&&nowMs-row.modifiedAtMs>=limit).map(row=>row.name).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))));
}
