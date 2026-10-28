import {isAbsolute,join,resolve} from "node:path";
import {runtimeCollectGarbageV1} from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {refuseProcessV1} from "../../../📖️texts/🟦️.ts";

const retentionPolicy=schema.definitions.RetentionPolicy.const;

/** 🧩️ The part of the TypeScript compiler API the engine drives; a caller passes the module resolved from the project's own dependency tree, so no compiler type leaves this file. */
export type TypeScriptApiV1=Readonly<{
 version:string;
 sys:any;
 parseCommandLine(args:readonly string[]):{options:any;errors:readonly any[]};
 getParsedCommandLineOfConfigFile(file:string,extend:any,host:any):any;
 getConfigFileParsingDiagnostics(parsed:any):readonly any[];
 createIncrementalCompilerHost(options:any,system?:any):any;
 createSemanticDiagnosticsBuilderProgram(rootNames:readonly string[],options:any,host:any,old:any,diagnostics:readonly any[],references:any):any;
 sortAndDeduplicateDiagnostics(diagnostics:readonly any[]):readonly any[];
 formatDiagnostics(diagnostics:readonly any[],host:any):string;
 flattenDiagnosticMessageText(message:unknown,newLine:string):string;
 OperationCanceledException?:new()=>Error;
}>;

/** 🎚️ What the engine keeps between checks: how many programs, the bound on the collected JavaScript heap above which everything is released (the resident set is no measure: a runtime keeps freed pages mapped for seconds), and the idle time after which a host releases. */
export type TypeScriptCheckRetentionV1=Readonly<{retainedPrograms:number;maximumHeapBytes:number;idleMilliseconds:number}>;

/** 🏷️ The shipped retention: two programs, 2 GiB of collected heap (one 3,000-file program holds about 1 GiB), two idle minutes. */
export const TYPESCRIPT_CHECK_RETENTION_V1:TypeScriptCheckRetentionV1=Object.freeze({retainedPrograms:retentionPolicy.retainedPrograms,maximumHeapBytes:retentionPolicy.maximumHeapBytes,idleMilliseconds:retentionPolicy.idleMilliseconds});

/** 📨️ One check: the project directory, the tsconfig path relative to it, the admitted diagnostic switches, an optional file system view and the cancellation and progress ports. */
export type TypeScriptCheckRequestV1=Readonly<{cwd:string;project:string;args?:readonly string[];system?:any;signal?:AbortSignal;onPhase?:(phase:"configuration"|"program"|"diagnostics")=>void}>;

/** 📊️ What a check cost: files, source files re-parsed, how the old program was reused (0 not, 1 safe modules, 2 completely) and the milliseconds per phase. */
export type TypeScriptCheckStatsV1=Readonly<{files:number;parsed:number;reused:number;warm:boolean;releasedFor?:"memory";configurationMs:number;programMs:number;diagnosticsMs:number}>;

/** 💼️ The verdict: the exit code of `tsc --noEmit -p` (0 clean, 2 diagnostics, 1 unusable invocation), its diagnostics as the compiler prints them without colour, and the stats. */
export type TypeScriptCheckResultV1=Readonly<{exitCode:0|1|2;output:string;stats:TypeScriptCheckStatsV1}>;

/** 🔁️ A compiler that keeps its builder program, and the parsed source files of the retained programs, between checks. */
export type TypeScriptCheckEngineV1=Readonly<{check(request:TypeScriptCheckRequestV1):Promise<TypeScriptCheckResultV1>;release():number;retained():number}>;

type Slot={builder:any;files:Map<string,{text:string;key:string;file:any}>};

/** 🧮️ Whether the engine honours the switches itself: only options that change nothing but message text; listing, tracing and statistics switches print through the compiler's own reporter and stay with the cold command. */
export function typeScriptCheckSwitchesAdmittedV1(args:readonly string[]):boolean{return args.every(argument=>argument==="--noErrorTruncation");}

/** 🏗️ Creates the engine over one compiler module. Diagnostics equal those of a cold `tsc --noEmit -p` on the same tree: source files are reused only while their text is byte-equal and the compiler itself agrees that the options still permit it, module resolutions are redone on every check, and a cancelled or failed check drops its program. */
export function createTypeScriptCheckEngineV1(ts:TypeScriptApiV1,retention:Partial<TypeScriptCheckRetentionV1>={}):TypeScriptCheckEngineV1{
 const limits:TypeScriptCheckRetentionV1=Object.freeze({...TYPESCRIPT_CHECK_RETENTION_V1,...retention});
 if(!Number.isSafeInteger(limits.retainedPrograms)||limits.retainedPrograms<1||limits.retainedPrograms>retentionPolicy.maximumRetainedPrograms)throw refuseProcessV1("invalid","typescript.retained-programs-range",{maximum:retentionPolicy.maximumRetainedPrograms});
 const slots=new Map<string,Slot>(),heap=()=>process.memoryUsage().heapUsed,collect=():void=>runtimeCollectGarbageV1(true),format=(cwd:string)=>({getCurrentDirectory:()=>cwd,getCanonicalFileName:(name:string)=>ts.sys.useCaseSensitiveFileNames?name:name.toLowerCase(),getNewLine:()=>"\n"});
 const cancellation=(signal?:AbortSignal)=>({isCancellationRequested:()=>signal?.aborted===true,throwIfCancellationRequested(){if(signal?.aborted)throw ts.OperationCanceledException?new ts.OperationCanceledException():signal.reason??refuseProcessV1("cancelled","typescript.check-cancelled");}});
 async function check(request:TypeScriptCheckRequestV1):Promise<TypeScriptCheckResultV1>{
  const system=request.system??ts.sys,args=request.args??[],started=performance.now(),cwd=resolve(request.cwd);
  request.signal?.throwIfAborted();request.onPhase?.("configuration");
  const cli=ts.parseCommandLine(["--noEmit","-p",request.project,...args]),text=(rows:readonly any[])=>ts.formatDiagnostics(ts.sortAndDeduplicateDiagnostics(rows),format(cwd));
  const empty=(exitCode:0|1|2,output:string,rest:Partial<TypeScriptCheckStatsV1>={}):TypeScriptCheckResultV1=>({exitCode,output,stats:{files:0,parsed:0,reused:0,warm:false,configurationMs:Math.round(performance.now()-started),programMs:0,diagnosticsMs:0,...rest}});
  if(!typeScriptCheckSwitchesAdmittedV1(args))throw refuseProcessV1("unsupported","typescript.check-switches-not-admitted",{switches:args.join(" ")});
  if(cli.errors.length)return empty(1,text(cli.errors));
  const direct=isAbsolute(request.project)?request.project:join(cwd,request.project),config=system.directoryExists(direct)?join(direct,"tsconfig.json"):direct;
  let unusable:any;
  const parsed=ts.getParsedCommandLineOfConfigFile(config,cli.options,{...system,onUnRecoverableConfigFileDiagnostic:(diagnostic:any)=>{unusable=diagnostic;}});
  if(!parsed)return empty(1,text(unusable?[unusable]:[]));
  const key=JSON.stringify([config,args]),slot:Slot=slots.get(key)??{builder:undefined,files:new Map()};
  slots.delete(key);
  const configurationMs=performance.now()-started,counters={parsed:0},seen=new Set<string>(),host=ts.createIncrementalCompilerHost(parsed.options,system),parse=host.getSourceFile.bind(host);
  host.getSourceFile=(name:string,language:any,onError:unknown,fresh:boolean)=>{
   const content=system.readFile(name);if(content===undefined)return undefined;
   const mode=typeof language==="object"?`${language.languageVersion}:${language.impliedNodeFormat}`:String(language),hit=slot.files.get(name);seen.add(name);
   if(!fresh&&hit&&hit.text===content&&hit.key===mode)return hit.file;
   counters.parsed++;const file=parse(name,language,onError,fresh);if(file)slot.files.set(name,{text:content,key:mode,file});return file;
  };
  host.hasInvalidatedResolutions=()=>true;
  try{
   request.signal?.throwIfAborted();request.onPhase?.("program");
   const programStarted=performance.now(),old=slot.builder,builder=ts.createSemanticDiagnosticsBuilderProgram(parsed.fileNames,parsed.options,host,old,ts.getConfigFileParsingDiagnostics(parsed),parsed.projectReferences),programMs=performance.now()-programStarted;
   slot.builder=builder;request.onPhase?.("diagnostics");
   const token=cancellation(request.signal),all:any[]=[...builder.getConfigFileParsingDiagnostics()],fixed=all.length;
   all.push(...builder.getSyntacticDiagnostics(undefined,token));
   if(all.length===fixed){all.push(...builder.getOptionsDiagnostics(token));all.push(...builder.getGlobalDiagnostics(token));if(all.length===fixed)all.push(...builder.getSemanticDiagnostics(undefined,token));}
   for(const name of slot.files.keys())if(!seen.has(name))slot.files.delete(name);
   let wide=heap()>limits.maximumHeapBytes;if(wide||old===undefined){collect();wide=heap()>limits.maximumHeapBytes;}
   slots.set(key,slot);while(slots.size>limits.retainedPrograms)slots.delete(slots.keys().next().value!);
   if(wide)slots.clear();
   const diagnosticsMs=performance.now()-programStarted-programMs;
   return {exitCode:all.length?2:0,output:all.length?text(all):"",stats:{files:seen.size,parsed:counters.parsed,reused:builder.getProgram().structureIsReused??0,warm:old!==undefined,...(wide?{releasedFor:"memory" as const}:{}),configurationMs:Math.round(configurationMs),programMs:Math.round(programMs),diagnosticsMs:Math.round(diagnosticsMs)}};
  }catch(error){slots.delete(key);throw error;}
 }
 return Object.freeze({check,release:()=>{const count=slots.size;slots.clear();if(count)collect();return count;},retained:()=>slots.size});
}

/** 🛎️ The request a hermetic action's own command sends to the service: a project directory, its tsconfig and the admitted switches. */
export type TypeScriptCheckServiceRequestV1=Readonly<{cwd:string;project:string;args?:readonly string[]}>;

/** 🛰️ The service a persistent hermetic worker hosts for the actions of a class (see the `services` table of the hermetic classes): it resolves the compiler from the requesting project's own dependency tree, keeps one engine per compiler module and drops all of them when the host asks for it. */
export function createHermeticServiceV1(options:Readonly<{resolve(directory:string,name:string):unknown;retention?:Partial<TypeScriptCheckRetentionV1>}>):Readonly<{handle(request:TypeScriptCheckServiceRequestV1,signal?:AbortSignal):Promise<Pick<TypeScriptCheckResultV1,"exitCode"|"output"|"stats">>;release():number}>{
 let compiler:unknown,engine:TypeScriptCheckEngineV1|undefined;
 return Object.freeze({
  async handle(request,signal){
   const module=options.resolve(resolve(request.cwd),"typescript");
   if(module!==compiler||!engine){engine?.release();compiler=module;engine=createTypeScriptCheckEngineV1(module as TypeScriptApiV1,options.retention);}
   const result=await engine.check({...request,...(signal?{signal}:{})});
   return {exitCode:result.exitCode,output:result.output,stats:result.stats};
  },
  release:()=>engine?.release()??0
 });
}
