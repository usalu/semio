#!/usr/bin/env repo
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdirSync,readFileSync,readdirSync,statSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {admitFactsV1,expandRustcArgumentsV1,keyEnvironmentV1,normalizeRustcArgumentsV1,parseRustcUnitV1,rustcBypassV1,toolchainIdentityV1,unitKeyPartsV1,unitKeyV1,UNIT_CACHE_POLICY_V1} from "./🔏️key/🟦️.ts";
import {acquireSlotV1,acquireUnitClaimV1,type ClaimV1} from "./🔒️lock/🟦️.ts";
import {digestFileV1,emitUnitEventV1,lookupUnitV1,publishUnitV1,restoreUnitV1,type UnitEventV1} from "./💾️store/🟦️.ts";
import {lanePathsV1} from "./🧵️lane/🟦️.ts";
import {buildScriptsEnabledV1,installBuildScriptShimV1,isBuildScriptUnitV1} from "./🏗️buildscript/🟦️.ts";
import {brokerOfV1,recallProvenanceV1,replayCachedUnitV1} from "./🛰️broker/🟦️.ts";
import {collectGarbageSingleFlightV1,collectGarbageV1,measureGcV1,measureVolumeV1,planGcV1,watchUnitCachePressureV1} from "./🧹️gc/🟦️.ts";
import {fillRatesV1,readFillSamplesV1} from "./🧹️gc/📈️fill/🟦️.ts";
import { isMainModuleV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1;
type Environment=Readonly<Record<string,string|undefined>>;
type Settings=Readonly<{root:string;lane:string;laneRoot:string|undefined;inner:readonly string[];slots:number;focus:ReadonlySet<string>;stamps:string|undefined}>;

/** ⚙️ Reads the wrapper's settings, which the repository launcher exports; `undefined` when the cache is off. */
export function readSettingsV1(environment:Environment):Settings|undefined{
 const root=environment.SEMIO_UNIT_CACHE_ROOT;if(!root||environment.SEMIO_UNIT_CACHE==="off")return undefined;
 const lane=environment.SEMIO_UNIT_CACHE_LANE??"d",inner=environment.SEMIO_UNIT_CACHE_INNER?JSON.parse(environment.SEMIO_UNIT_CACHE_INNER) as string[]:["rustc"];
 if(!Array.isArray(inner)||inner.length<1||inner.some(part=>typeof part!=="string"))throw Error("SEMIO_UNIT_CACHE_INNER must be a JSON array of strings");
 const laneRoot=environment.SEMIO_UNIT_CACHE_LANE_ROOT;
 return Object.freeze({root:resolve(root),lane,laneRoot,inner:Object.freeze(inner),slots:Number(environment.SEMIO_UNIT_CACHE_SLOTS??0),focus:new Set((environment.SEMIO_UNIT_CACHE_FOCUS??"").split(",").filter(Boolean)),stamps:laneRoot?join(laneRoot,policy.layout.stamps):lanePathsV1(resolve(root),lane).stamps});
}

type Outcome=Readonly<{status:number;stdout:string;stderr:string}>;
/** 🎰️ The anonymous jobserver pipe a parent Cargo handed down as inheritable descriptors; the compiler must keep them to draw its codegen tokens from the same pool. Only the stand-in (`🚪️launch`) can tell which descriptors were open before the runtime started and says so in `SEMIO_UNIT_CACHE_JOBSERVER_FDS`; the numbers in `CARGO_MAKEFLAGS` alone prove nothing, because a build script (or its cached stand-in) does not hand them down while its environment still names them, and the runtime opens its own descriptors at exactly those low numbers. */
export function jobserverDescriptorsV1(environment:Environment):readonly number[]{
 const match=/^(\d+),(\d+)$/u.exec(environment.SEMIO_UNIT_CACHE_JOBSERVER_FDS??"");
 return match?[Number(match[1]),Number(match[2])]:[];
}

function run(inner:readonly string[],args:readonly string[],environment:Environment,capture:boolean):Promise<Outcome>{
 return new Promise((done,fail)=>{
  const inherited=jobserverDescriptorsV1(environment),stdio:Array<"inherit"|"pipe"|"ignore"|number>=[ "inherit",capture?"pipe":"inherit",capture?"pipe":"inherit"];
  if(inherited.length){for(let index=3;index<=Math.max(...inherited);index++)stdio[index]=inherited.includes(index)?index:"ignore";}
  const child=spawn(inner[0]!,[...inner.slice(1),...args],{env:environment as NodeJS.ProcessEnv,stdio}),out:Buffer[]=[],err:Buffer[]=[];
  const forward=(signal:NodeJS.Signals)=>{child.kill(signal);};
  for(const signal of["SIGINT","SIGTERM"] as const)process.once(signal,()=>forward(signal));
  if(capture){child.stdout!.on("data",chunk=>{out.push(chunk);process.stdout.write(chunk);});child.stderr!.on("data",chunk=>{err.push(chunk);process.stderr.write(chunk);});}
  child.on("error",fail);
  child.on("close",(status,signal)=>done({status:status??(signal?128+({SIGINT:2,SIGTERM:15,SIGKILL:9} as Record<string,number>)[signal]!:1),stdout:Buffer.concat(out).toString("utf8"),stderr:Buffer.concat(err).toString("utf8")}));
 });
}

function artifactsOf(text:string):string[]{
 const found:string[]=[];
 for(const line of text.split("\n")){if(!line.startsWith("{")||!line.includes("\"artifact\""))continue;try{const row=JSON.parse(line);if(typeof row.artifact==="string"&&["link","metadata","dep-info"].includes(row.emit))found.push(row.artifact);}catch{}}
 return[...new Set(found)];
}

/** 🪞️ Drops the compiler path Cargo puts before the arguments when it runs the stand-in as `<wrapper> <rustc> <arguments>`; every compiler argument list starts with a flag, a response file or a stdin dash. */
export function withoutCompilerPathV1(argv:readonly string[]):readonly string[]{
 const first=argv[0];
 return first!==undefined&&!first.startsWith("-")&&!first.startsWith("@")&&!first.endsWith(".rs")?argv.slice(1):argv;
}

/** 🛠️ Serves one rustc command line from the cache or compiles it exactly once across all lanes. Never throws on cache faults: any fault is a miss. A miss compiles the command line Cargo gave, so a workspace unit Cargo compiles incrementally keeps rustc's incremental session in the lane (the next edit of that crate, also when it is a dependency of the focused one, recompiles incrementally) and is published like every other unit; the focused crates always compile in the lane and are never published. */
export async function wrapRustcV1(argv:readonly string[],environment:Environment,cwd:string):Promise<number>{
 argv=withoutCompilerPathV1(argv);const settings=readSettingsV1(environment);
 if(!settings)return(await run(environment.SEMIO_UNIT_CACHE_INNER?JSON.parse(environment.SEMIO_UNIT_CACHE_INNER):["rustc"],argv,environment,false)).status;
 const args=expandRustcArgumentsV1(argv,cwd),unit=parseRustcUnitV1(args),bypass=rustcBypassV1(unit,args),crate=unit.crateName||"-";
 const event=(kind:UnitEventV1["kind"],key:string|null,milliseconds:number,waitedMs=0,reason?:string)=>emitUnitEventV1(settings.root,{lane:settings.lane,kind,crate,key,milliseconds,waitedMs,...(reason?{reason}:{})});
 const compile=async(compileArgs:readonly string[],capture:boolean):Promise<Outcome>=>{
  let slot:ClaimV1|undefined;
  if(settings.slots>0&&unit.source.endsWith(".rs")){slot=await acquireSlotV1(settings.root,settings.slots,{onWait:waited=>event("slot-wait",null,0,waited)});}
  try{return await run(settings.inner,compileArgs,environment,capture);}finally{slot?.release();}
 };
 const started=Date.now();
 if(bypass){
  const outcome=await compile(args,false);
  if(unit.source.endsWith(".rs"))event("bypass",null,Date.now()-started,0,bypass);
  return outcome.status;
 }
 const focused=unit.incremental!==undefined&&settings.focus.has(environment.CARGO_PKG_NAME??"");
 if(focused){const outcome=await compile(args,false);event("incremental",null,Date.now()-started);return outcome.status;}
 let key:string,parts="";
 const digests=new Map<string,string>();
 try{
  const sourceSha=digestFileV1(resolve(cwd,unit.source),settings.stamps);if(sourceSha===null)throw Error("source");
  for(const row of unit.externs){if(row.path==="")continue;const sha=digestFileV1(resolve(cwd,row.path),settings.stamps);if(sha===null)throw Error("extern");digests.set(row.path,sha);}
  const keyInput={toolchain:toolchainIdentityV1(settings.root,settings.inner,environment),normalized:normalizeRustcArgumentsV1(args,digests,settings.laneRoot),sourceSha256:sourceSha,externs:unit.externs.map(row=>({name:row.flags+row.name,sha256:row.path===""?"builtin":digests.get(row.path)!})),environment:keyEnvironmentV1(environment,settings.laneRoot)};key=unitKeyV1(keyInput);parts=unitKeyPartsV1(keyInput);
 }catch{const outcome=await compile(args,false);event("bypass",null,Date.now()-started,0,"unreadable-input");return outcome.status;}
 const context={cwd,laneRoot:settings.laneRoot,environment,stampDirectory:settings.stamps},outDir=resolve(cwd,unit.outDir),shimming=isBuildScriptUnitV1(unit)&&buildScriptsEnabledV1(environment),facts=brokerOfV1(environment)?admitFactsV1(args,environment,cwd):undefined;
 const shim=(outputs:readonly string[]):void=>{if(!shimming)return;const artifact=outputs.find(path=>!path.endsWith(".d")&&!path.endsWith(".rmeta"));if(artifact)installBuildScriptShimV1(artifact);};
 const serve=async():Promise<boolean>=>{
  const entry=lookupUnitV1(settings.root,key,context);if(!entry)return false;
  const messages=restoreUnitV1(settings.root,entry,{outDir,laneRoot:settings.laneRoot,stampDirectory:settings.stamps});
  shim(entry.outputs.filter(row=>!row.template).map(row=>join(outDir,row.name)));
  if(facts){const replayed=await replayCachedUnitV1(environment,facts,key,entry.provenance);if(!replayed)event("broker-unreachable",key,0,0,"replay");else event("broker-replay",key,0);}
  process.stdout.write(messages.stdout);process.stderr.write(messages.stderr);return true;
 };
 try{if(await serve()){event("hit",key,Date.now()-started);return 0;}}catch{}
 let claim:ClaimV1|undefined;
 try{claim=await acquireUnitClaimV1(join(settings.root,policy.layout.locks),key.slice(0,40),{});}catch{claim=undefined;}
 const waited=claim?.waitedMs??0;
 if(waited>50||!claim)event(claim?"lock-wait":"lock-timeout",key,0,waited);
 try{
  if(claim){try{if(await serve()){event("hit",key,Date.now()-started,waited,"after-wait");return 0;}}catch{}}
  const compiled=Date.now(),outcome=await compile(args,true);
  if(outcome.status!==0){event("failed",key,Date.now()-compiled,waited);return outcome.status;}
  event("miss",key,Date.now()-compiled,waited,unit.incremental===undefined?parts:parts+" incremental:lane");
  const outputs=[...new Set([...artifactsOf(outcome.stderr),...artifactsOf(outcome.stdout)])].map(path=>resolve(cwd,path)).filter(path=>existsSync(path));
  try{
   const provenance=facts?await recallProvenanceV1(environment,facts):undefined;
   const entry=outputs.length?publishUnitV1(settings.root,{key,crate,cwd,outDir,laneRoot:settings.laneRoot,outputs,stdout:outcome.stdout,stderr:outcome.stderr,stampDirectory:settings.stamps,provenance:provenance??null}):undefined;
   if(entry)event("publish",key,Date.now()-compiled,0);
  }catch{}
  try{shim(outputs);}catch{}
  return 0;
 }finally{claim?.release();}
}

/** 🧰️ Operator commands, run as `--semio-unit-cache <command> --root <dir> …`: `stats` summarises the event logs, `plan` and `gc` run the bounded cleanup; `gc --single-flight` is the background collector (one at a time, progress lines on stderr, SIGINT and SIGTERM cancel it), `gc --single-flight --watch` keeps it alive while compiles run (it collects whenever the image falls below its floor), `fill` prints the fill rates of the last half hour. */
export async function manageV1(args:readonly string[]):Promise<unknown>{
 const [command,...rest]=args,value=(flag:string)=>{const index=rest.indexOf(flag);return index<0?undefined:rest[index+1];},root=resolve(value("--root")??"");
 if(command==="stats"){
  const totals:Record<string,number>={};let maximumWait=0,files=0;const directory=join(root,policy.layout.events);
  if(existsSync(directory))for(const name of readdirSync(directory)){files++;for(const line of readFileSync(join(directory,name),"utf8").split("\n")){if(!line)continue;try{const row=JSON.parse(line);totals[row.kind]=(totals[row.kind]??0)+1;maximumWait=Math.max(maximumWait,row.waitedMs);}catch{}}}
  return{lanes:files,totals,maximumWaitMs:maximumWait};
 }
 const capacity=Number(value("--capacity")??0);if(!Number.isSafeInteger(capacity)||capacity<1)throw Error("--capacity <bytes> required");
 if(command==="plan")return planGcV1(measureGcV1(root,capacity,Date.now()));
 if(command==="gc"&&!rest.includes("--single-flight"))return collectGarbageV1(root,capacity);
 if(command==="fill"){const volume=measureVolumeV1(root,capacity);return{...fillRatesV1(readFillSamplesV1(root),Date.now()),freeBytes:volume.freeBytes,volumeBytes:volume.volumeBytes};}
 if(command==="gc"){
  const controller=new AbortController();for(const signal of["SIGINT","SIGTERM"] as const)process.once(signal,()=>controller.abort(new Error("cancelled")));
  if(rest.includes("--watch"))return watchUnitCachePressureV1(root,capacity,{signal:controller.signal});
  return collectGarbageSingleFlightV1(root,capacity,{signal:controller.signal,progress:(done,total)=>{process.stderr.write(JSON.stringify({progress:done,total})+"\n");}});
 }
 throw Error("Unknown unit-cache command");
}

/** 🏁️ Runs the wrapper as a process: `--semio-unit-cache <command> …` is an operator command (its JSON result on standard output), anything else is one rustc command line Cargo hands to the wrapper. Resolves to the exit status; shared by the script entry and the native executable entry (`🏁️entry.ts`), whose `argv[1]` is no file the main-module test could match. */
export async function runUnitCacheMainV1(argv:readonly string[],environment:Environment,cwd:string):Promise<number>{
 try{
  if(argv[0]==="--semio-unit-cache"){console.log(JSON.stringify(await manageV1(argv.slice(1))));return 0;}
  return await wrapRustcV1(argv,environment,cwd);
 }catch(error){console.error(String((error as Error)?.message??error));return 1;}
}

if(isMainModuleV1(import.meta.url))runUnitCacheMainV1(process.argv.slice(2),process.env,process.cwd()).then(status=>{process.exitCode=status;});
