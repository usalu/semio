import {validateJsonSchemaSubset}from"../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import type{NativeBuildPassV1}from"../📋️contract/🟦️.ts";

const policy=schema.definitions.Policy.const;
/** 🎫️ What a running pass holds here: the `local` slot (a jobserver token, at most one at a time) or a `light` one (the cargo process and the input shipping only, its units compile on workers). */
export type PassJobSlotV1="local"|"light";

/** 🛰️ Whether everything a pass still does after its inputs are shipped can run on a worker: its target is host independent, so its units, its final cdylib among them, are offloadable. Any other pass keeps the strictly sequential local behaviour.
 * @see ./🧬️schema/🔣️.json (`Policy.offloadableTargetPrefixes`) */
export const nativeBuildPassOffloadableV1=(pass:Pick<NativeBuildPassV1,"target">):boolean=>policy.offloadableTargetPrefixes.some(prefix=>pass.target.startsWith(prefix));

/** 🧵️ Runs the passes of one lane in order with up to `jobs` offloadable passes at once: the first running pass holds the local slot, the others a light one; a pass that is not offloadable, or `jobs` of one, waits for every running pass and runs alone, which is the sequential behaviour of a build without a fleet. The first failure stops launching, is handed to `fail` at once (the driver ends the sibling cargo children) and is thrown after every running pass settled.
 * @see ./🧪️tests/🟦️.ts */
export async function runPassesInJobSlotsV1<T extends Pick<NativeBuildPassV1,"target">>(passes:readonly T[],options:Readonly<{jobs:number;check:()=>void;run:(pass:T,slot:PassJobSlotV1)=>Promise<void>;fail?:(error:unknown)=>void;offloadable?:(pass:T)=>boolean}>):Promise<void>{
 const running=new Set<Promise<void>>(),offloadable=options.offloadable??nativeBuildPassOffloadableV1;let failure:{error:unknown}|undefined,localHeld=false;
 const launch=(pass:T,slot:PassJobSlotV1):Promise<void>=>{
  if(slot==="local")localHeld=true;
  const task:Promise<void>=(async()=>options.run(pass,slot))().catch(error=>{if(!failure){failure={error};options.fail?.(error);}}).finally(()=>{running.delete(task);if(slot==="local")localHeld=false;});
  running.add(task);return task;
 };
 try{for(const pass of passes){
  if(failure)break;
  options.check();
  if(options.jobs<=1||!offloadable(pass)){while(running.size)await Promise.race(running);if(failure)break;await launch(pass,"local");continue;}
  while(running.size>=options.jobs&&!failure)await Promise.race(running);
  if(failure)break;
  launch(pass,localHeld?"light":"local");
 }}catch(error){if(!failure){failure={error};options.fail?.(error);}}
 while(running.size)await Promise.allSettled([...running]);
 if(failure)throw failure.error;
}

/** 🧵️ Why the concurrent component jobs of a build are fewer than every worker slot that fits one wasm final: no worker can take one, the authored cap, the memory the light remote jobs may hold here or raised memory pressure. */
export type ComponentJobsReasonV1="jobs-local-only"|"jobs-all"|"jobs-cap"|"jobs-memory"|"jobs-pressure";
/** 🌐️ What one component jobs plan reads: the local grant (`local`), the worker slots whose machine has the free memory for one component final at its predicted peak (`fleetSlots`, `null` when no fleet answers) and the live memory facts of this machine. */
export type ComponentJobsHostV1=Readonly<{local:number;fleetSlots:number|null;availableMegabytes:number|null;pressure:"normal"|"warn"|"critical"|null;unitMegabytes:number}>;
/** 🎟️ The planned concurrent component jobs: `jobs` is `local` (jobs that hold a local slot) plus `remote` (jobs whose remaining work is offloadable and that hold only the cargo process and the input shipping here). */
export type ComponentJobsDecisionV1=Readonly<{jobs:number;local:number;remote:number;reason:ComponentJobsReasonV1}>;

const fill=(text:string,values:Readonly<Record<string,string|number>>):string=>text.replace(/\{([a-z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);
function requireDefinition<T>(definition:"ComponentJobsHost"|"ComponentJobsDecision",value:unknown):T{
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value);
 if(errors.length)throw Error(fill(schema.definitions.Messages.const.en.errors.definitionRequired,{detail:errors[0]!}));
 return value as T;
}

/** 🧵️ Sizes the concurrent component jobs of one build: the local grant plus the worker slots that can take a wasm final (`fleetSlots`, counted at the predicted peak of the final), bounded by the authored cap (`componentJobsMaximum`) and by the memory the local residue of an offloaded job (`componentJobResidueMegabytes`: the cargo process, its fingerprint checks and the input shipping) may hold beside the local compiles; none under critical and half under raised memory pressure. Without a fleet answer the plan is the local grant, so a build with the fleet down behaves as before.
 * @see ./🧬️schema/🔣️.json
 * @see ./🧪️tests/🧵️plan/🐍️oracle/🐍️.py */
export function planComponentJobsV1(hostValue:ComponentJobsHostV1):ComponentJobsDecisionV1{
 const host=requireDefinition<ComponentJobsHostV1>("ComponentJobsHost",hostValue);
 if(host.fleetSlots===null||host.fleetSlots===0)return Object.freeze(requireDefinition<ComponentJobsDecisionV1>("ComponentJobsDecision",{jobs:host.local,local:host.local,remote:0,reason:"jobs-local-only"}));
 let remote=host.fleetSlots,reason:ComponentJobsReasonV1="jobs-all";
 const limit=(value:number,why:ComponentJobsReasonV1)=>{if(value<remote){remote=Math.max(0,value);reason=why;}};
 limit(Math.max(0,policy.componentJobsMaximum-host.local),"jobs-cap");
 if(host.availableMegabytes!==null)limit(Math.floor(Math.max(0,host.availableMegabytes-policy.reserveMegabytes-host.local*host.unitMegabytes)/policy.componentJobResidueMegabytes),"jobs-memory");
 if(host.pressure==="critical")limit(0,"jobs-pressure");
 else if(host.pressure==="warn")limit(Math.floor(remote/policy.pressureWarnDivisor),"jobs-pressure");
 return Object.freeze(requireDefinition<ComponentJobsDecisionV1>("ComponentJobsDecision",{jobs:host.local+remote,local:host.local,remote,reason}));
}

/** 🖨️ Describes a plan in the given language (English first, German second). */
export function describeComponentJobsV1(language:"en"|"de",values:Readonly<Record<string,string|number>>):string{
 const catalogue=schema.definitions.Messages.const[language],reason=values.reason;
 return fill(catalogue.jobs,{...values,...typeof reason==="string"&&reason in catalogue.reasons?{reason:catalogue.reasons[reason as keyof typeof catalogue.reasons]}:{}});
}
