import {compileJsonSchemaSubset,type CompiledJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const validators=new Map<string,CompiledJsonSchemaSubset>();

/** 🛡️ Validates a value against one definition of the trust schema. */
export function requireTrustShapeV1(definition:"Trust"|"Policy"|"Fixture",value:unknown,message:string):void{
 let validator=validators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);validators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🔏️ The tier a reader gives the manifest: `unusable` (no fresh health of a living daemon with a reachable store), `verify` (check every entry against the tree before believing it) or `trusted` (the manifest reflects the tree for the scope now). */
export type TrustTierV1="unusable"|"verify"|"trusted";

/** 🔏️ The statement of the daemon about its manifest, as written into the health file: for `scope`, the start of the oldest sweep that found the manifest equal to the tree, and when the last event happened that can make it wrong (and its kind). */
export type TrustV1=Readonly<{scope:"shared-source";verifiedMs:number|null;lostMs:number|null;lostKind:string|null}>;

/** 🕰️ The times of the rule in milliseconds: health is fresh for 30 s, a verification is believed for 10 min, a change younger than 30 s that a sweep finds is no drift (the faster tiers had no time yet). */
export const trustPolicyV1=Object.freeze({version:1,healthMaximumAgeMilliseconds:30_000,verifiedMaximumAgeMilliseconds:600_000,driftGraceMilliseconds:30_000} as const);

const ticketPrefix=".🧬semio/🦑️repo/🎫️tickets/",neutralHeads=new Set(["attest","skipped","watch","seal","entry-boot","skew","scan","compaction","unsupported"]),originPattern=/^[a-z0-9][a-z0-9-]{0,39}$/u;

/** 🎯️ Whether a path belongs to the scope `shared-source`: everything but the git directory, the machine cache, the ticket tree and AppleDouble litter. */
export function sourcePathV1(path:string):boolean{
 const name=path.split(":")[0]!,base=name.slice(name.lastIndexOf("/")+1);
 return !(name===".git"||name.startsWith(".git/")||name.startsWith(".cache/")||name.startsWith(ticketPrefix)||base.startsWith("._"));
}

/** 🏷️ Whether a failure key (`kind:rest`, as the daemon notes them) can make the manifest wrong for the scope (`manifest`) or cannot (`neutral`): git objects and packs, paths outside the scope, attestation, skipped names, clock skew, peer journal read errors and failed scans cannot; a failed apply, an unstable read, a refusal or a lost blob for a source path, journal, state and unknown kinds can. Unknown means `manifest`. */
export function classifyFailureV1(key:string):"manifest"|"neutral"{
 const cut=key.indexOf(":"),head=cut<0?key:key.slice(0,cut),rest=cut<0?"":key.slice(cut+1);
 if(head.startsWith("git-broken-")||neutralHeads.has(head))return "neutral";
 if(head==="blocked"){
  if(rest.startsWith("journal:"))return "manifest";
  if(rest.startsWith("objects:"))return "neutral";
  return sourcePathV1(rest)?"manifest":"neutral";
 }
 if(head==="blocked-long"||head==="blob-wanted"||head==="entry-guard"||head==="refused"||head==="unstable")return sourcePathV1(rest)?"manifest":"neutral";
 if(head==="error"){
  if(rest.startsWith("git:"))return "neutral";
  if(rest.startsWith("clash:"))return "manifest";
  const first=rest.split(":")[0]!;
  if(!first.includes("/")&&originPattern.test(first))return "neutral";
  return sourcePathV1(rest)?"manifest":"neutral";
 }
 return "manifest";
}

/** 🩺️ The tier for a health file: `unusable` unless it is fresh, its store is `ok` and its process lives; then `verify` unless its trust statement holds a verification that started after the last loss and is not older than ten minutes; else `trusted`. The lifetime failure counters of the health file play no part. */
export function decideTrustV1(health:Readonly<{wallMs:number;pid:number;store:string;trust?:TrustV1}>,now:number,alive:(pid:number)=>boolean,policy:Readonly<{healthMaximumAgeMilliseconds:number;verifiedMaximumAgeMilliseconds:number}>=trustPolicyV1):TrustTierV1{
 if(!(Math.abs(now-health.wallMs)<=policy.healthMaximumAgeMilliseconds&&health.store==="ok"&&alive(health.pid)))return "unusable";
 const trust=health.trust;
 if(trust===undefined||trust.verifiedMs===null)return "verify";
 if(trust.lostMs!==null&&trust.verifiedMs<=trust.lostMs)return "verify";
 if(now-trust.verifiedMs>policy.verifiedMaximumAgeMilliseconds)return "verify";
 return "trusted";
}

/** 🔏️ The sweeps that can verify: the file sweep (every file of the manifest against the tree), the directory sweep (every known directory against its listing) and the complete tree walk (both at once). */
export type TrustPartV1="file"|"directory"|"walk";

/** 🔏️ The tracker of the daemon. */
export type TrustTrackerV1={failure(key:string):boolean;lost(kind:string):void;verified(part:TrustPartV1,startedMs:number):void;record():TrustV1};

/** 🏭️ Creates the tracker: `failure` classifies a noted failure and loses trust when it can make the manifest wrong, `lost` loses it for a drift the sweeps found, `verified` records a sweep that began at `startedMs` and found the manifest equal to the tree; `record` is the statement for the health file. Nothing is verified until a file sweep and a directory sweep (or a walk) completed. */
export function createTrustTrackerV1(now:()=>number):TrustTrackerV1{
 let lostMs:number|null=null,lostKind:string|null=null;
 const clean:Record<TrustPartV1,number|null>={file:null,directory:null,walk:null};
 const kindOf=(key:string):string=>{const cut=key.indexOf(":"),head=(cut<0?key:key.slice(0,cut)).toLowerCase().replace(/[^a-z0-9-]/gu,"-").replace(/^[^a-z]+/u,"");return (head===""?"failure":head).slice(0,40);};
 const lose=(kind:string):void=>{lostMs=now();lostKind=kind;};
 return {
  failure(key){if(classifyFailureV1(key)!=="manifest")return false;lose(kindOf(key));return true;},
  lost:kind=>{lose(kind);},
  verified(part,startedMs){if(clean[part]===null||startedMs>clean[part]!)clean[part]=startedMs;},
  record(){
   const both=clean.file===null||clean.directory===null?null:Math.min(clean.file,clean.directory),verifiedMs=both===null?clean.walk:clean.walk===null?both:Math.max(both,clean.walk);
   return {scope:"shared-source",verifiedMs,lostMs,lostKind};
  }
 };
}
