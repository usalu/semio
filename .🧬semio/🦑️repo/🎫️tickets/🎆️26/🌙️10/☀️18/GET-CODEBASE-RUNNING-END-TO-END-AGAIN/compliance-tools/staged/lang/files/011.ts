import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {cacheTextV1} from "../../../../💾️storage/🧾️ledger/🟦️.ts";
import {planNxToolingRetentionV1} from "../🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import policy from "./🔣️policy.json" with {type:"json"};
import {refuseBootstrapV1} from "../../../📖️texts/🟦️.ts";

/** 🎚️ The closed policy of the proactive Nx toolkit store bound. */
export type NxToolingStoreBoundPolicyV1=Readonly<{version:1;keepNewest:number;keepNewestUnderPressure:number;minimumAgeMs:number;maximumPerRun:number;maximumCensus:number;idleWaitMs:number;idlePollMs:number;probePatienceMs:number;intervalMs:number}>;
/** 🗂️ One toolkit generation as the census saw it; `held` is true while a live reader keeps its owner lease. */
export type NxToolingStoreImageV1=Readonly<{identity:string;createdAt:number;held:boolean}>;
/** 📥️ Everything the pure bound needs: the clock, the current activation, whether the machine budget or the free space of the image presses, and the census. */
export type NxToolingStoreBoundInputV1=Readonly<{nowMs:number;current:string;pressure:boolean;images:readonly NxToolingStoreImageV1[]}>;
/** 🏷️ Why a generation stays. */
export type NxToolingStoreReasonV1="current"|"held"|"newest"|"young";
/** 📤️ The plan: how many newest generations were kept, why each other one stays, every candidate to retire oldest first, and how many of them one run may actually retire. */
export type NxToolingStoreBoundPlanV1=Readonly<{keepNewest:number;retained:readonly Readonly<{identity:string;reason:NxToolingStoreReasonV1}>[];retire:readonly string[];limit:number}>;

/** 📋️ Validates the bound policy: closed schema, and no larger bound under pressure than without. */
export function readNxToolingStoreBoundPolicyV1(value:unknown):NxToolingStoreBoundPolicyV1{
 const errors=validateJsonSchemaSubset(schema,value),row=value as NxToolingStoreBoundPolicyV1;
 if(errors.length||row.keepNewestUnderPressure>row.keepNewest){const reason=errors[0]??"pressure";throw Error(cacheTextV1("en","nxTooling.policy",{reason})+"\n"+cacheTextV1("de","nxTooling.policy",{reason}));}
 return Object.freeze({...row});
}

let memo:NxToolingStoreBoundPolicyV1|undefined;
/** 🧭️ The authored bound policy, validated once. */
export function nxToolingStoreBoundPolicyV1():NxToolingStoreBoundPolicyV1{return memo??=readNxToolingStoreBoundPolicyV1(policy);}

/** 🌿️ The environment variable that lengthens (or, with 0, removes) the bounded wait for an idle tools image on one machine. */
export const NX_TOOLING_IDLE_WAIT_ENVIRONMENT_V1="SEMIO_NX_TOOLING_IDLE_WAIT_MS";

/** 🔧️ The authored policy with the machine's override of the idle wait applied; an override that is not a whole number of milliseconds within the schema bound is refused in both languages. */
export function resolveNxToolingStoreBoundPolicyV1(environment:Readonly<Record<string,string|undefined>>,declared:NxToolingStoreBoundPolicyV1=nxToolingStoreBoundPolicyV1()):NxToolingStoreBoundPolicyV1{
 const value=environment[NX_TOOLING_IDLE_WAIT_ENVIRONMENT_V1];
 return value===undefined||value===""?declared:readNxToolingStoreBoundPolicyV1({...declared,idleWaitMs:/^(0|[1-9][0-9]{0,8})$/u.test(value)?Number(value):Number.NaN});
}

/** 🧮️ The pure bound: the count rule of `planNxToolingRetentionV1` (newest N plus the current activation) widened by every generation a live reader holds and every generation younger than the minimum age, every other generation listed oldest first together with the per-run limit of actual retirements (so a generation the retirement keeps never starves the younger ones behind it). Nothing here reads the disk. */
export function planNxToolingStoreBoundV1(input:NxToolingStoreBoundInputV1,declared:NxToolingStoreBoundPolicyV1=nxToolingStoreBoundPolicyV1()):NxToolingStoreBoundPlanV1{
 if(!Number.isFinite(input.nowMs)||typeof input.pressure!=="boolean"||input.images.some(image=>typeof image.held!=="boolean"))throw refuseBootstrapV1("invalid","tools.cohort-required");
 const keepNewest=input.pressure?declared.keepNewestUnderPressure:declared.keepNewest,droppable=new Set(planNxToolingRetentionV1(input.images.map(({identity,createdAt})=>({identity,createdAt})),input.current,keepNewest)),candidates:NxToolingStoreImageV1[]=[],retained:{identity:string;reason:NxToolingStoreReasonV1}[]=[];
 const newestFirst=[...input.images].sort((left,right)=>right.createdAt-left.createdAt||(left.identity<right.identity?-1:left.identity>right.identity?1:0));
 for(const image of newestFirst){
  const reason:NxToolingStoreReasonV1|undefined=image.identity===input.current?"current":image.held?"held":!droppable.has(image.identity)?"newest":input.nowMs-image.createdAt<declared.minimumAgeMs?"young":undefined;
  if(reason===undefined)candidates.push(image);else retained.push({identity:image.identity,reason});
 }
 const oldestFirst=[...candidates].sort((left,right)=>left.createdAt-right.createdAt||(left.identity<right.identity?-1:left.identity>right.identity?1:0)).map(image=>image.identity);
 return Object.freeze({keepNewest,retained:Object.freeze(retained),retire:Object.freeze(oldestFirst),limit:declared.maximumPerRun});
}
