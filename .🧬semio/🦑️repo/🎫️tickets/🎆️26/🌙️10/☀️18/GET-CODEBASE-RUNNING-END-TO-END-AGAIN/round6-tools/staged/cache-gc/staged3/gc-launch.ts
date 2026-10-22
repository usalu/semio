import {spawn} from "node:child_process";
import {mkdir,stat,writeFile} from "node:fs/promises";
import {setPriority} from "node:os";
import {dirname} from "node:path";
import {fileURLToPath} from "node:url";
import quota from "../../📥️inputs/💾️storage/🧬️schema/🔣️.json" with {type:"json"};
import {repoCacheDirectory} from "../../../⚡️caching/🟦️.ts";
import {cacheLedgerV1,type CacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";

/** 🚦️ What a writer that just looked at the graph image's free space should do about optional caches. */
export type GraphCollectorDecisionV1="launch"|"enough"|"throttled"|"inline-floor";
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

/** 🔌️ The host side of a launch: clock, stamp file and process start; tests replace them. */
export type GraphCollectorPortsV1=Readonly<{now():number;readStamp(path:string):Promise<number|null>;writeStamp(path:string):Promise<void>;start(command:string,args:readonly string[],options:Readonly<{cwd:string;env:Readonly<Record<string,string>>}>):void;environment:Readonly<Record<string,string|undefined>>;execPath:string;script:string}>;

const defaults=():GraphCollectorPortsV1=>({
 now:()=>Date.now(),
 readStamp:async path=>stat(path).then(row=>row.mtimeMs,()=>null),
 writeStamp:async path=>{await mkdir(dirname(path),{recursive:true});await writeFile(path,"");},
 start:(command,args,options)=>{const child=spawn(command,[...args],{cwd:options.cwd,env:{...options.env},detached:true,stdio:"ignore",windowsHide:true});child.on("error",()=>{});child.unref();if(child.pid!==undefined)try{setPriority(child.pid,10);}catch{}},
 environment:process.env,
 execPath:process.execPath,
 script:fileURLToPath(new URL("../../📜️script.ts",import.meta.url))
});

/** 🚀️ Starts the graph optional-cache collector as a detached, low-priority `repo cache gc --area=graph-inference` when the image has less free space than the ledger's soft threshold but more than the hard floor; never waits for it and never fails the writer. Returns the decision taken. */
export async function launchGraphOptionalCollectorV1(workspace:string,availableBytes:number,overrides:Partial<GraphCollectorPortsV1>={},ledger:CacheLedgerV1=cacheLedgerV1()):Promise<GraphCollectorDecisionV1>{
 const ports={...defaults(),...overrides},area=ledger.areas.find(row=>row.id==="graph-inference"),soft=area?.reclaim?.softAvailableBytes;
 if(soft===undefined)return "enough";
 const stamp=repoCacheDirectory(workspace,"build","graph-collector.stamp"),now=ports.now(),decision=graphCollectorDecisionV1({availableBytes,softBytes:soft,floorBytes:quota.definitions.OptionalQuota.policy.minimumAvailableBytes,nowMs:now,lastLaunchMs:await ports.readStamp(stamp),minimumIntervalMs:GRAPH_COLLECTOR_INTERVAL_MS_V1});
 if(decision!=="launch")return decision;
 try{await ports.writeStamp(stamp);ports.start(ports.execPath,[ports.script,"repo","cache","gc","--area=graph-inference"],{cwd:workspace,env:graphCollectorEnvironmentV1(ports.environment)});}catch{}
 return decision;
}
