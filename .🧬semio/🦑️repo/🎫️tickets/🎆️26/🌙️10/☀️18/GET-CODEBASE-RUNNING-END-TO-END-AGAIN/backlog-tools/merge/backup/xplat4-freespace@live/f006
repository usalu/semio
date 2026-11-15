import {spawn} from "node:child_process";
import {createRequire} from "node:module";
import {appendFileSync,closeSync,openSync} from "node:fs";
import {statfsSync,statSync} from "node:fs";
import {mkdir,stat,writeFile} from "node:fs/promises";
import {setPriority} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {repoCacheDirectory} from "../../../⚡️caching/🟦️.ts";
import {repositoryCacheBudgetV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import {cacheLedgerV1,cacheTextV1,type CacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {nxToolingStoreBoundPolicyV1} from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🟦️.ts";

/** 🚦️ What a writer that just looked at the graph image's free space should do about optional caches. */
export type GraphCollectorDecisionV1="launch"|"enough"|"throttled"|"failed";
/** ⏱️ Between two background launches nobody starts a third: the collector is single-flight, the stamp only stops a storm of writers from spawning processes. */
export const GRAPH_COLLECTOR_INTERVAL_MS_V1=300000;
/** ⏲️ The products collector measures trees, so a persistent shortage that only claimed slots can relieve waits longer between two launches. */
export const PRODUCTS_COLLECTOR_INTERVAL_MS_V1=900000;
const environmentKeys=Object.freeze(["PATH","HOME","USERPROFILE","TMPDIR","TEMP","TMP","SystemRoot","ComSpec","LANG","LC_ALL","LC_MESSAGES","SEMIO_LANGUAGE",...(createRequire(import.meta.url)("../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🟨️.cjs").selectedManagerV1("javascript").hostEnvironment as readonly string[])]);

/** 🧮️ Pure rule: enough space needs nothing, anything below the soft threshold (also below the hard floor, where the writer's own exclusive try cannot succeed while readers hold the image) is the background collector's job at most once per interval. */
export function graphCollectorDecisionV1(input:Readonly<{availableBytes:number;softBytes:number;nowMs:number;lastLaunchMs:number|null;minimumIntervalMs:number}>):GraphCollectorDecisionV1{
 if(input.availableBytes>=input.softBytes)return "enough";
 if(input.lastLaunchMs!==null&&input.nowMs-input.lastLaunchMs<input.minimumIntervalMs)return "throttled";
 return "launch";
}

/** 🌱️ The clean environment of the detached collector: the original invocation envelope of the writer must never leak into a new top-level command. */
export function graphCollectorEnvironmentV1(environment:Readonly<Record<string,string|undefined>>):Readonly<Record<string,string>>{
 const kept:Record<string,string>={SEMIO_CACHE_BACKGROUND:"1"};
 for(const key of environmentKeys){const value=environment[key];if(value!==undefined)kept[key]=value;}
 return kept;
}

/** 🔌️ The host side of a launch: clock, stamp file, process start (its output goes to a log file, a failure to start is appended to it), the status file and the report line of a failed launch; tests replace them. */
export type GraphCollectorPortsV1=Readonly<{now():number;readStamp(path:string):Promise<number|null>;writeStamp(path:string):Promise<void>;start(command:string,args:readonly string[],options:Readonly<{cwd:string;env:Readonly<Record<string,string>>;log:string}>):void;writeStatus(path:string,text:string):Promise<void>;report(line:string):void;environment:Readonly<Record<string,string|undefined>>;execPath:string;script:string}>;

const defaults=():GraphCollectorPortsV1=>({
 now:()=>Date.now(),
 readStamp:async path=>stat(path).then(row=>row.mtimeMs,()=>null),
 writeStamp:async path=>{await mkdir(dirname(path),{recursive:true});await writeFile(path,"");},
 start:(command,args,options)=>{
  const fd=openSync(options.log,"w");
  try{const child=spawn(command,[...args],{cwd:options.cwd,env:{...options.env},detached:true,stdio:["ignore",fd,fd],windowsHide:true});child.on("error",error=>{appendFileSync(options.log,"launch failed: "+error.message+"\n");});child.unref();if(child.pid!==undefined)try{setPriority(child.pid,10);}catch(error){appendFileSync(options.log,"priority not lowered: "+String((error as Error).message)+"\n");}}
  finally{closeSync(fd);}
 },
 writeStatus:async(path,text)=>{await mkdir(dirname(path),{recursive:true});await writeFile(path,text);},
 report:line=>{console.error(line);},
 environment:process.env,
 execPath:process.execPath,
 script:fileURLToPath(new URL("../../📜️script.ts",import.meta.url))
});

/** 🎯️ One background collector: the ledger area whose `reclaim` rule decides, the arguments of its `repo cache gc` command, the base name of its stamp, log and status files and the text of a failed launch. */
export type CollectorTargetV1=Readonly<{area:string;arguments:readonly string[];name:string;failed:string;intervalMs:number}>;
const graphTarget:CollectorTargetV1=Object.freeze({area:"graph-inference",arguments:["--area=graph-inference"],name:"graph-collector",failed:"gc.graph.launch.failed",intervalMs:GRAPH_COLLECTOR_INTERVAL_MS_V1}),productsTarget:CollectorTargetV1=Object.freeze({area:"bazel-workspaces",arguments:["--area=bazel-workspaces,bazel-outputs"],name:"products-collector",failed:"gc.products.launch.failed",intervalMs:PRODUCTS_COLLECTOR_INTERVAL_MS_V1});
const nxToolingTarget:CollectorTargetV1=Object.freeze({area:"nx-tooling",arguments:["--area=nx-tooling,nx-tmp"],name:"nx-tooling-collector",failed:"nxTooling.launch.failed",intervalMs:nxToolingStoreBoundPolicyV1().intervalMs});

async function launchCollectorV1(workspace:string,availableBytes:number,target:CollectorTargetV1,overrides:Partial<GraphCollectorPortsV1>,ledger:CacheLedgerV1):Promise<GraphCollectorDecisionV1>{
 const ports={...defaults(),...overrides},area=ledger.areas.find(row=>row.id===target.area),soft=area?.reclaim?.softAvailableBytes;
 if(soft===undefined)return "enough";
 const stamp=repoCacheDirectory(workspace,"build",target.name+".stamp"),log=repoCacheDirectory(workspace,"build",target.name+".log"),status=repoCacheDirectory(workspace,"build",target.name+".status.json"),now=ports.now(),decision=graphCollectorDecisionV1({availableBytes,softBytes:soft,nowMs:now,lastLaunchMs:await ports.readStamp(stamp),minimumIntervalMs:target.intervalMs});
 if(decision!=="launch")return decision;
 try{await ports.writeStamp(stamp);ports.start(ports.execPath,[ports.script,"repo","cache","gc",...target.arguments],{cwd:workspace,env:graphCollectorEnvironmentV1(ports.environment),log});}
 catch(error){
  const message=String((error as Error).message);
  try{await ports.writeStatus(status,JSON.stringify({version:1,atMs:now,state:"failed",message})+"\n");}catch(statusError){ports.report("status: "+String((statusError as Error).message));}
  ports.report(["en","de"].map(language=>cacheTextV1(language as "en"|"de",target.failed,{message})).join("\n"));
  return "failed";
 }
 return decision;
}

/** 🚀️ Starts the graph optional-cache collector as a detached, low-priority `repo cache gc --area=graph-inference` (output in `graph-collector.log` of the build area) when the image has less free space than the ledger's soft threshold but more than the hard floor; never waits for it and never fails the writer: a collector that cannot start is reported on the writer's stderr in both languages, recorded in `graph-collector.status.json` and returned as `failed`. Returns the decision taken. */
export async function launchGraphOptionalCollectorV1(workspace:string,availableBytes:number,overrides:Partial<GraphCollectorPortsV1>={},ledger:CacheLedgerV1=cacheLedgerV1()):Promise<GraphCollectorDecisionV1>{return launchCollectorV1(workspace,availableBytes,graphTarget,overrides,ledger);}

/** 🔰️ Starts the products collector as a detached, low-priority `repo cache gc --area=bazel-workspaces,bazel-outputs` (output in `products-collector.log` of the build area) below the soft threshold the ledger declares for the Bazel workspaces; single-flight (the stamp allows one launch per interval and the collector itself takes the session collector lease without waiting), never waits for it and never fails the writer. */
export async function launchProductsCollectorV1(workspace:string,availableBytes:number,overrides:Partial<GraphCollectorPortsV1>={},ledger:CacheLedgerV1=cacheLedgerV1()):Promise<GraphCollectorDecisionV1>{return launchCollectorV1(workspace,availableBytes,productsTarget,overrides,ledger);}

/** 🧶️ Starts the Nx toolkit store collector as a detached, low-priority `repo cache gc --area=nx-tooling,nx-tmp` (output in `nx-tooling-collector.log` of the build area) below the soft free-space threshold the ledger declares for the tools image; single-flight (one launch per interval of the bound policy, and the collector itself takes a lease without waiting), never waits for it and never fails the writer. Returns the decision taken. */
export async function launchNxToolingCollectorV1(workspace:string,availableBytes:number,overrides:Partial<GraphCollectorPortsV1>={},ledger:CacheLedgerV1=cacheLedgerV1()):Promise<GraphCollectorDecisionV1>{return launchCollectorV1(workspace,availableBytes,nxToolingTarget,overrides,ledger);}

/** 🧰️ What any Nx run does about a pressed tools image: it reads the free space of the mounted image (`undefined` means not mounted, which counts as free) and launches the single-flight background collector below the soft threshold. Never waits, never throws. */
export async function launchNxToolingCollectorIfPressedV1(workspace:string,overrides:Partial<GraphCollectorPortsV1>={},measure:(workspace:string)=>number|undefined=mountedToolsFreeBytesV1):Promise<GraphCollectorDecisionV1|"unmounted">{
 try{
  const available=measure(workspace);
  return available===undefined?"unmounted":await launchNxToolingCollectorV1(workspace,available,overrides);
 }catch{return "unmounted";}
}

/** 🕸️ What any graph loader does about a pressed image before it loads the graph: it reads the free space of the mounted graph image through `measure` (the default statfs of the mount; `undefined` means the image is not mounted, which counts as free) and launches the single-flight background collector below the soft threshold. Never waits, never throws. */
export async function launchGraphCollectorIfPressedV1(workspace:string,overrides:Partial<GraphCollectorPortsV1>={},measure:(workspace:string)=>number|undefined=mountedGraphFreeBytesV1):Promise<GraphCollectorDecisionV1|"unmounted">{
 try{
  const available=measure(workspace);
  return available===undefined?"unmounted":await launchGraphOptionalCollectorV1(workspace,available,overrides);
 }catch{return "unmounted";}
}

/** 🛍️ What any loader does about a pressed products image: it reads the free space of the mounted image (`undefined` means not mounted, which counts as free) and launches the single-flight background collector below the soft threshold. Never waits, never throws. */
export async function launchProductsCollectorIfPressedV1(workspace:string,overrides:Partial<GraphCollectorPortsV1>={},measure:(workspace:string)=>number|undefined=mountedProductsFreeBytesV1):Promise<GraphCollectorDecisionV1|"unmounted">{
 try{
  const available=measure(workspace);
  return available===undefined?"unmounted":await launchProductsCollectorV1(workspace,available,overrides);
 }catch{return "unmounted";}
}

/** 📏️ The free bytes of the mounted image of a partition of a workspace, undefined while the image is not mounted. */
function mountedPartitionFreeBytesV1(workspace:string,partition:"graph"|"products"|"tools"):number|undefined{
 const directory=repositoryCacheBudgetV1(workspace).partitions[partition].directory,mount=join(directory,"m");
 if(statSync(mount).dev===statSync(directory).dev)return undefined;
 const space=statfsSync(mount);return space.bavail*space.bsize;
}
/** 🆓️ The free bytes of the mounted graph image of a workspace, undefined while the image is not mounted. */
export const mountedGraphFreeBytesV1=(workspace:string):number|undefined=>mountedPartitionFreeBytesV1(workspace,"graph");
/** 💿️ The free bytes of the mounted products image of a workspace, undefined while the image is not mounted. */
export const mountedProductsFreeBytesV1=(workspace:string):number|undefined=>mountedPartitionFreeBytesV1(workspace,"products");
/** 🧰️ The free bytes of the mounted tools image of a workspace, undefined while the image is not mounted. */
export const mountedToolsFreeBytesV1=(workspace:string):number|undefined=>mountedPartitionFreeBytesV1(workspace,"tools");
