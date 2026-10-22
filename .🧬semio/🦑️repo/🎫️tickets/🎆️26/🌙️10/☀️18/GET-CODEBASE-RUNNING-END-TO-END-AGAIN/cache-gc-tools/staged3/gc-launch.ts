import {spawn} from "node:child_process";
import {appendFileSync,closeSync,openSync} from "node:fs";
import {mkdir,stat,writeFile} from "node:fs/promises";
import {setPriority} from "node:os";
import {dirname} from "node:path";
import {fileURLToPath} from "node:url";
import quota from "../../📥️inputs/💾️storage/🧬️schema/🔣️.json" with {type:"json"};
import {repoCacheDirectory} from "../../../⚡️caching/🟦️.ts";
import {cacheLedgerV1,cacheTextV1,type CacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";

/** 🚦️ What a writer that just looked at the graph image's free space should do about optional caches. */
export type GraphCollectorDecisionV1="launch"|"enough"|"throttled"|"inline-floor"|"failed";
/** ⏱️ Between two background launches nobody starts a third: the collector is single-flight, the stamp only stops a storm of writers from spawning processes. */
export const GRAPH_COLLECTOR_INTERVAL_MS_V1=300000;
const environmentKeys=Object.freeze(["PATH","HOME","USERPROFILE","TMPDIR","TEMP","TMP","SystemRoot","ComSpec","LANG","LC_ALL","LC_MESSAGES","SEMIO_LANGUAGE","BUN_INSTALL"]);

/** 🧮️ Pure rule: enough space needs nothing, space below the hard floor is the writer's own last resort, everything between is the background collector's job at most once per interval. */
export function graphCollectorDecisionV1(input:Readonly<{availableBytes:number;softBytes:number;floorBytes:number;nowMs:number;lastLaunchMs:number|null;minimumIntervalMs:number}>):GraphCollectorDecisionV1{
 if(input.availableBytes>=input.softBytes)return "enough";
 if(input.availableBytes<input.floorBytes)return "inline-floor";
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

/** 🚀️ Starts the graph optional-cache collector as a detached, low-priority `repo cache gc --area=graph-inference` (output in `graph-collector.log` of the build area) when the image has less free space than the ledger's soft threshold but more than the hard floor; never waits for it and never fails the writer: a collector that cannot start is reported on the writer's stderr in both languages, recorded in `graph-collector.status.json` and returned as `failed`. Returns the decision taken. */
export async function launchGraphOptionalCollectorV1(workspace:string,availableBytes:number,overrides:Partial<GraphCollectorPortsV1>={},ledger:CacheLedgerV1=cacheLedgerV1()):Promise<GraphCollectorDecisionV1>{
 const ports={...defaults(),...overrides},area=ledger.areas.find(row=>row.id==="graph-inference"),soft=area?.reclaim?.softAvailableBytes;
 if(soft===undefined)return "enough";
 const stamp=repoCacheDirectory(workspace,"build","graph-collector.stamp"),log=repoCacheDirectory(workspace,"build","graph-collector.log"),status=repoCacheDirectory(workspace,"build","graph-collector.status.json"),now=ports.now(),decision=graphCollectorDecisionV1({availableBytes,softBytes:soft,floorBytes:quota.definitions.OptionalQuota.policy.minimumAvailableBytes,nowMs:now,lastLaunchMs:await ports.readStamp(stamp),minimumIntervalMs:GRAPH_COLLECTOR_INTERVAL_MS_V1});
 if(decision!=="launch")return decision;
 try{await ports.writeStamp(stamp);ports.start(ports.execPath,[ports.script,"repo","cache","gc","--area=graph-inference"],{cwd:workspace,env:graphCollectorEnvironmentV1(ports.environment),log});}
 catch(error){
  const message=String((error as Error).message);
  try{await ports.writeStatus(status,JSON.stringify({version:1,atMs:now,state:"failed",message})+"\n");}catch(statusError){ports.report("status: "+String((statusError as Error).message));}
  ports.report(["en","de"].map(language=>cacheTextV1(language as "en"|"de","gc.graph.launch.failed",{message})).join("\n"));
  return "failed";
 }
 return decision;
}
