import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json" with { type: "json" };
import{type ResourceLeaseControl}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {isAbsolute,join} from "node:path";
import {requireBazelSlotV1} from "../🧵️sessions/🟦️.ts";
import {bazelServerDirectoryV1} from "../🧵️sessions/📖️status/🟦️.ts";
import {bazelOutputRootV1} from "../📐️path-plan/🟦️.ts";
import budget from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {cacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {resolveRemoteCacheContractV1} from "../../../🌐️remote-cache/🟦️.ts";
import {bazelRemoteCacheV1,type BazelCredentialHelperV1} from "../🌐️remote-cache/🟦️.ts";

const hostPaths=schema.definitions.HostPaths.const;
/** 🔤️ Refuses the host paths that a platform's Bazel server cannot open byte-exactly, with the empirical reason authored in the schema. */
export function requireBazelHostPathsV1(platform:string,layout:Readonly<Record<string,unknown>>):void{
 const rule=(hostPaths.platforms as Readonly<Record<string,{pattern:string;guarded?:readonly string[]}>>)[platform];if(!rule)return;
 const pattern=new RegExp(rule.pattern);for(const key of rule.guarded??hostPaths.guarded){const value=layout[key];if(typeof value!=="string"||!pattern.test(value))throw Error(`ASCII Bazel host path required on ${platform}: ${key}`);}
}
/** 🛤️ Names every native Bazel cache writer inside its authored finite partition. */
export function bazelCacheLayoutV1(roots:Readonly<{tools:string;dependencies:string;products:string}>){
 if(Object.values(roots).some(root=>!isAbsolute(root)))throw Error("Absolute Bazel storage roots required");
 const layout={version:1 as const,executable:join(roots.tools,"bazel/bin"),install:join(roots.tools,"bazel/install"),modules:join(roots.dependencies,"bazel/modules"),workspace:join(roots.products,"bazel/workspaces"),outputs:join(roots.products,"bazel/outputs"),actions:join(roots.products,"bazel/actions"),pool:join(roots.products,"bazel/pool"),server:bazelServerDirectoryV1(roots.products),temporary:join(roots.products,"tmp")};if(Object.values(layout).some(value=>typeof value==="string"&&value.length>256))throw Error("Bazel storage path exceeds 256 characters");if(validateJsonSchemaSubset(schema.definitions.StorageLayout,layout,schema).length)throw Error("Closed Bazel cache layout required");requireBazelHostPathsV1(process.platform,layout);return Object.freeze(layout);
}
/** 🎟️ Retains authenticated executable, dependency and output backing until all original children settle. */
export interface BazelStorageV1{readonly tools:RepositoryCacheStoreV1;readonly dependencies:RepositoryCacheStoreV1;readonly products:RepositoryCacheStoreV1;readonly paths:ReturnType<typeof bazelCacheLayoutV1>}
/** 🔐️ Refuses fabricated store contexts before the first filesystem observation or writer. */
export async function requireBazelStorageV1(root:string,storage:BazelStorageV1):Promise<void>{for(const id of ["tools","dependencies","products"] as const){const store=storage[id];requireNativeStorageReservationV1(store.storage,store.control);await store.recheck();if(store.root!==repositoryCachePartitionRootV1(root,id))throw Error("Original declared Bazel partition required");}const roots={tools:storage.tools.root,dependencies:storage.dependencies.root,products:storage.products.root},layout=bazelCacheLayoutV1(roots);if(JSON.stringify(layout)!==JSON.stringify(storage.paths))throw Error("Original Bazel partition layout required");for(const[key,path]of Object.entries(layout))if(key!=="version")requireNativeStoragePathV1(key==="executable"||key==="install"?roots.tools:key==="modules"?roots.dependencies:roots.products,path as string);}
/** 🏪️ Keeps all finite stores alive through the complete native Bazel invocation. */
export function withBazelStorageV1<T>(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(storage:BazelStorageV1,actualPhase:ResourceLeaseControl)=>Promise<T>):Promise<T>{return withRepositoryCacheStoreV1(root,"tools",invocation,phaseControl,(tools,toolPhase)=>withRepositoryCacheStoreV1(root,"dependencies",invocation,toolPhase,(dependencies,dependencyPhase)=>withRepositoryCacheStoreV1(root,"products",invocation,dependencyPhase,(products,actualPhase)=>body(Object.freeze({tools,dependencies,products,paths:bazelCacheLayoutV1({tools:tools.root,dependencies:dependencies.root,products:products.root})}),actualPhase))));}
/** 🧳️ The two stores `tools setup bazel` writes: the executable and the module catalog never touch the products partition. */
export type BazelSetupStorageV1=Pick<BazelStorageV1,"tools"|"dependencies"|"paths">;
/** 🛂️ Refuses fabricated setup contexts; the products root is a declared path here and is never opened. */
export async function requireBazelSetupStorageV1(root:string,storage:BazelSetupStorageV1):Promise<void>{for(const id of ["tools","dependencies"] as const){const store=storage[id];requireNativeStorageReservationV1(store.storage,store.control);await store.recheck();if(store.root!==repositoryCachePartitionRootV1(root,id))throw Error("Original declared Bazel partition required");}const roots={tools:storage.tools.root,dependencies:storage.dependencies.root,products:repositoryCachePartitionRootV1(root,"products")},layout=bazelCacheLayoutV1(roots);if(JSON.stringify(layout)!==JSON.stringify(storage.paths))throw Error("Original Bazel partition layout required");for(const key of ["executable","install","modules"] as const)requireNativeStoragePathV1(key==="modules"?roots.dependencies:roots.tools,layout[key]);}
/** 🏬️ Keeps only the tools and dependencies stores alive while Bazel and its fixed module catalog are provisioned. */
export function withBazelSetupStorageV1<T>(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(storage:BazelSetupStorageV1,actualPhase:ResourceLeaseControl)=>Promise<T>):Promise<T>{return withRepositoryCacheStoreV1(root,"tools",invocation,phaseControl,(tools,toolPhase)=>withRepositoryCacheStoreV1(root,"dependencies",invocation,toolPhase,(dependencies,actualPhase)=>body(Object.freeze({tools,dependencies,paths:bazelCacheLayoutV1({tools:tools.root,dependencies:dependencies.root,products:repositoryCachePartitionRootV1(root,"products")})}),actualPhase)));}

const cachePolicy=schema.definitions.CachePolicy.const;
/** 📏️ Bounds Bazel's disk cache to its authored share of the finite products partition, aligned down to whole MiB. */
export function bazelDiskCacheMaximumBytesV1(productsMaximumLogicalBytes:number=budget.partitions.products.maximumLogicalBytes):number{
 if(!Number.isSafeInteger(productsMaximumLogicalBytes)||productsMaximumLogicalBytes<1048576)throw Error("Finite products budget required");
 return Math.floor(productsMaximumLogicalBytes*cachePolicy.diskCache.productsShareBasisPoints/10000/1048576)*1048576;
}
/** 🌡️ The free space below which the session collector frees packages and output bases of the products image, and the free space it frees up to: the `reclaim` rule the cache ledger declares for the workspaces area. */
export function bazelWorkspaceReclaimV1():Readonly<{softAvailableBytes:number;targetAvailableBytes:number}>{
 const rule=cacheLedgerV1().areas.find(area=>area.id==="bazel-workspaces")?.reclaim;if(!rule)throw refuseBuildV1("missing","bazel.workspaces-reclaim-rule-required");return rule;
}
/** 📋️ Says who collects the Bazel disk cache and how far: Bazel's own garbage collector inside every persistent server keeps it within its authored share of the products partition and age, so the cache collector only reports the bound. */
export function bazelDiskCacheReportV1(language:"en"|"de"):string{
 const megabytes=Math.floor(bazelDiskCacheMaximumBytesV1()/1048576),days=cachePolicy.diskCache.garbageCollectionMaximumAgeDays;
 return language==="de"?`[bazel] der Disk-Cache wird von Bazels eigenem Garbage-Collector in jedem Server gehalten: höchstens ${megabytes} MB und ${days} Tage`:`[bazel] the disk cache is kept by Bazel's own garbage collector inside every server: at most ${megabytes} MB and ${days} days`;
}
/** 🖥️ Names the output base of one session slot: a short ASCII path (slot ids are `s0`, `s1`, ...) whose server serves every command of the slot's one holder, because Bazel restarts a server bound to another workspace directory and serializes commands on one output base. */
export function bazelServerBaseV1(layout:ReturnType<typeof bazelCacheLayoutV1>,slot:string):string{
 const base=join(layout.server,requireBazelSlotV1(slot));if(base.length>256)throw Error("Bazel storage path exceeds 256 characters");return base;
}
/** 🌐️ Derives the closed startup and command flags for Bazel's disk, repository-content and optional remote caches; the remote part is the Bazel rendering of the neutral remote cache contract (`🌐️remote-cache`) and stays off until an endpoint is configured. The token travels only in `environment`, which the caller hands to the Bazel client and never persists. */
export function bazelCacheFlagsV1(layout:ReturnType<typeof bazelCacheLayoutV1>,slot:string,options:Readonly<{environment?:Readonly<Record<string,string|undefined>>;skipCache?:boolean;skipRemoteCache?:boolean;checkout?:string}>={}):Readonly<{startup:readonly string[];command:readonly string[];remote:boolean;environment:Readonly<Record<string,string>>;helper?:BazelCredentialHelperV1}>{
 const input=options.environment??{},contract=resolveRemoteCacheContractV1(input),command=[`--disk_cache=${layout.actions}`,`--experimental_disk_cache_gc_max_size=${bazelDiskCacheMaximumBytesV1()}`];
 if(options.skipCache)command.push("--noremote_accept_cached");
 const rendering=contract!==undefined&&!options.skipRemoteCache?bazelRemoteCacheV1(contract,input,{helperDirectory:layout.temporary}):undefined;
 if(rendering)command.push(...rendering.flags);
 const flags={startup:Object.freeze((()=>{const short=options.checkout===undefined?undefined:bazelOutputRootV1(process.platform,options.checkout);return short?[`--output_user_root=${short.root}`,`--output_base=${short.base(requireBazelSlotV1(slot))}`]:[`--output_base=${bazelServerBaseV1(layout,slot)}`];})()),command:Object.freeze(command),remote:rendering!==undefined,environment:rendering?.environment??Object.freeze({}),...(rendering?.helper?{helper:rendering.helper}:{})};
 if(validateJsonSchemaSubset(schema.definitions.CacheFlags,flags,schema).length)throw Error("Closed Bazel cache flags required");return Object.freeze(flags);
}
