import schema from "./@@SCHEMA@@/@@JSON@@" with {type:"json"};
import registry from "./@@REGISTRY@@/@@JSON@@" with {type:"json"};
import {validateJsonSchemaSubset} from "@@VALID@@";
import type {ScriptInvocation} from "@@PROC@@/@@ROUTING@@/@@TS@@";
import type {ResourceLeaseControl} from "@@PROC@@/@@LEASES@@/@@TS@@";
import type {BuildGraphV1} from "@@UP@@@@GRAPH@@/@@TS@@";
import type {BuildCacheToolV1,CacheProbeToolV1} from "@@UP@@@@OPERATIONS@@/@@TS@@";
import type {RemoteCacheToolV1} from "@@UP@@@@REMOTECACHE@@/@@COMMAND@@/@@TS@@";

/** 🆔️ The identity of a build backend: any id the registry declares, never a closed list in code. */
export type BuildBackendIdV1=string;
/** 🏷️ How a backend or operation is provided: by the tool itself, by an adapter around it, by neutral code, or not at all. */
export type BackendSupportV1="native"|"adapter"|"neutral"|"missing";
/** 🧩️ A behaviour every backend is judged on by the conformance suite. */
export type BackendCapabilityV1=typeof schema.definitions.Capability.enum[number];
/** 🌍️ A text in both supported languages. */
export type BackendTextV1=Readonly<{en:string;de:string}>;
/** 🗒️ One backend as the registry declares it. */
export type BackendRowV1=Readonly<{id:BuildBackendIdV1;title:BackendTextV1;kind:"external"|"in-house";default?:boolean;module:string;driver:string;dependencyModel:"all"|"consumers";processEnvironment?:Readonly<Record<string,string>>;nativeSnapshot?:Readonly<{variable:string;environment:Readonly<Record<string,string>>}>;nativeCaller?:Readonly<{namespace:string;schema:string}>;cacheProof?:Readonly<{logMarker?:string;activity?:boolean;spawnMarker?:Readonly<{executed:string;completed:string}>}>;operations:Readonly<Record<string,BackendSupportV1>>;capabilities:Readonly<Partial<Record<BackendCapabilityV1,Readonly<{support:BackendSupportV1;note?:BackendTextV1}>>>>}>;

/** 📜️ Reads the authored registry once and refuses a row set that breaks its schema, repeats an id or names a missing default. */
export function readBackendRegistryV1(value:unknown=registry):readonly BackendRowV1[]{
 const issues=validateJsonSchemaSubset(schema,value,schema);
 if(issues.length)throw Error(`Closed build backend registry required: ${issues.slice(0,3).join("; ")}`);
 const rows=(value as {backends:BackendRowV1[]}).backends,ids=rows.map(row=>row.id);
 if(new Set(ids).size!==ids.length)throw Error("Unique build backend ids required");
 if(rows.filter(row=>row.default===true).length!==1)throw Error("Exactly one default build backend required");
 return Object.freeze(rows.map(row=>Object.freeze(row)));
}

const rows=readBackendRegistryV1();

/** 📇️ The registered backends in registry order. */
export function buildBackendRowsV1():readonly BackendRowV1[]{return rows;}
/** 🔢️ The registered backend ids in registry order. */
export function buildBackendIdsV1():readonly BuildBackendIdV1[]{return rows.map(row=>row.id);}
/** ⭐️ The backend that runs when nothing selects one: the registry row marked as default. */
export function buildBackendDefaultIdV1():BuildBackendIdV1{return rows.find(row=>row.default===true)!.id;}
/** 🧾️ The ids as a help-text alternation such as `a|b|c`. */
export function buildBackendAlternationV1():string{return buildBackendIdsV1().join("|");}
/** 🏷️ The first launch argument of every backend that nests its own tool; neutral code admits these next to `repo`. */
export function buildBackendNativeNamespacesV1():string[]{return rows.flatMap(row=>row.nativeCaller?[row.nativeCaller.namespace]:[]);}
/** 🧭️ Where a registered backend's conformance driver module lives; the row path is relative to this module. */
export function buildBackendDriverUrlV1(id:string):URL{return new URL(buildBackendRowV1(id).driver,import.meta.url);}
/** 🔎️ Finds one registered backend or refuses an unknown name with the list of valid ones. */
export function buildBackendRowV1(id:string):BackendRowV1{
 const row=rows.find(candidate=>candidate.id===id);
 if(!row)throw Error(`Unknown build tool: ${id} (${buildBackendAlternationV1()})\nUnbekanntes Build-Werkzeug: ${id} (${buildBackendAlternationV1()})`);
 return row;
}
/** ✔️ Admits a backend id and returns it. */
export function requireBuildBackendIdV1(id:string):BuildBackendIdV1{return buildBackendRowV1(id).id;}
/** 🛡️ The operations a backend does not provide: what the registry marks `missing`. */
export function backendMissingOperationsV1(id:string):string[]{return Object.entries(buildBackendRowV1(id).operations).filter(([,support])=>support==="missing").map(([name])=>name).sort();}
/** 🪪️ The declared native-snapshot protocol, owned by whichever backend declares one. */
export function buildNativeSnapshotProtocolV1():Readonly<{variable:string;environment:Readonly<Record<string,string>>}>|undefined{return rows.find(row=>row.nativeSnapshot)?.nativeSnapshot;}

/** 🎛️ What a backend operation receives: the workspace, the issued invocation and phase, the resolved arguments and the selected backend. */
export type BackendContextV1=Readonly<{root:string;invocation:ScriptInvocation;phase:ResourceLeaseControl;args:readonly string[];tool:BuildBackendIdV1;workers?:"on"|"off"}>;
/** 🪢️ A context that also carries the pinned build graph and the continuous task a watch re-runs. */
export type BackendExecutionContextV1=BackendContextV1&Readonly<{graph:BuildGraphV1;watch?:string}>;

/** 🔌️ Everything neutral code needs from a build backend; neutral code reaches a backend only through the registry and this interface. */
export interface BuildBackendV1{
 readonly id:BuildBackendIdV1;
 setup(context:BackendContextV1):Promise<void>;
 reset(context:BackendContextV1):Promise<void>;
 prepareStorage(context:BackendContextV1,snapshotBytes:number):Promise<void>;
 cache():Promise<BuildCacheToolV1>;
 remoteCache():Promise<RemoteCacheToolV1>;
 execute(context:BackendExecutionContextV1):Promise<void>;
 affectedProjects?(context:BackendExecutionContextV1,files:readonly string[]):Promise<readonly string[]>;
 bootstrapArguments?(args:readonly string[]):readonly string[]|undefined;
 cacheProbe?(context:Readonly<{root:string;invocation:ScriptInvocation}>):Promise<CacheProbeToolV1>;
 collectCache?(context:Readonly<{root:string;invocation:ScriptInvocation;phase:ResourceLeaseControl;dryRun:boolean;signal:AbortSignal}>):Promise<string>;
}

const loaded=new Map<string,Promise<BuildBackendV1>>();

/** 🚚️ Loads the adapter module a registry row names and checks that it exports a backend of the same id. */
export async function loadBuildBackendV1(id:string):Promise<BuildBackendV1>{
 const row=buildBackendRowV1(id);
 let pending=loaded.get(row.id);
 if(!pending){
  pending=import(new URL(row.module,import.meta.url).href).then((module:{backend?:BuildBackendV1})=>{
   if(!module.backend||module.backend.id!==row.id)throw Error(`Build backend module ${row.module} must export backend with id ${row.id}`);
   return module.backend;
  });
  loaded.set(row.id,pending);
 }
 return pending;
}

/** 🧭️ The launch a backend's own tool needs for a command, asked of the backends the command concerns: the selected one, or the named one for `tools setup`; the namespace is the first argument the backend's nested launches carry. `undefined` when none launches anything. */
export async function buildBackendBootstrapV1(args:readonly string[],tool:string):Promise<Readonly<{namespace:string;arguments:readonly string[]}>|undefined>{
 const target=args[0]==="tools"?args[2]??tool:tool,ids=target==="all"?buildBackendIdsV1():[requireBuildBackendIdV1(target)];
 for(const id of ids){
  const launch=(await loadBuildBackendV1(id)).bootstrapArguments?.(args),namespace=buildBackendRowV1(id).nativeCaller?.namespace;
  if(launch){if(!namespace)throw Error(`Build backend ${id} launches its tool and must declare nativeCaller`);return {namespace,arguments:launch};}
 }
 return undefined;
}
