import {randomBytes} from "node:crypto";
import {mkdir,readFile,rm,writeFile} from "node:fs/promises";
import {arch,cpus,loadavg,platform,totalmem} from "node:os";
import {dirname,join,resolve} from "node:path";
import schema from "../@@SCHD@@/@@JSON@@" with {type:"json"};
import budgets from "../@@BUDGETSF@@" with {type:"json"};
import {incrementalChildEnvironmentV1} from "../../../@@RUN@@/@@TS@@";
import {incrementalLanguageV1} from "../../../@@CMD@@/@@TS@@";
import type {BenchHostV1} from "../../@@TS@@";
import {acceptanceFailureV1,acceptanceHostV1,acceptanceJudgeV1,acceptanceReportV1,acceptanceSelectV1,acceptanceSetIssuesV1,renderAcceptanceReportV1,type AcceptanceOutcomeV1,type AcceptanceReportV1,type AcceptanceSetV1} from "../@@TS@@";
import {acceptanceRunV1,acceptanceSpawnV1,type AcceptancePortsV1,type AcceptanceRunRequestV1} from "../@@RUN@@/@@TS@@";

const policy=schema.definitions.Policy.const,both=(english:string,german:string)=>`${english}\n${german}`,options=policy.options;

/** 🍤 The closed arguments of `repo bench acceptance`. */
export type AcceptanceArgumentsV1=Readonly<{scenario:readonly string[]|"all";inPlace:boolean;file:string|undefined;markdown:string|undefined;json:boolean;language:"en"|"de"|undefined}>;

/** 🍨 The option names `repo bench acceptance` accepts, so the operations catalogue can be checked against the parser: the options of its own and the ones it shares with `repo bench run`. */
export function acceptanceOptionNamesV1():{valued:readonly string[];plain:readonly string[];shared:readonly string[]}{return {valued:options.valued,plain:options.plain,shared:[...options.shared,...options.flag]};}

/** 🍬 Reads the options after `bench acceptance`; every unknown or repeated option is refused in both languages. */
export function acceptanceArgumentsV1(args:readonly string[]):AcceptanceArgumentsV1{
 const seen=new Set<string>(),values:Record<string,string>={},flags=new Set<string>(),valued=new Set([...options.valued,...options.shared]),plain=new Set([...options.plain,...options.flag]);
 for(let index=0;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(seen.has(name))throw Error(both(`Repeated option: ${name}`,`Wiederholte Option: ${name}`));
  seen.add(name);
  if(valued.has(name)){const value=cut<0?args[++index]:argument.slice(cut+1);if(!value||value.startsWith("--"))throw Error(both(`Option ${name} needs a value`,`Option ${name} braucht einen Wert`));values[name]=value;}
  else if(plain.has(name)&&cut<0)flags.add(name);
  else throw Error(both(`Unknown bench acceptance option: ${argument}`,`Unbekannte Option für bench acceptance: ${argument}`));
 }
 const language=values["--lang"],picked=values["--scenario"]?.split(",").filter(Boolean);
 if(language!==undefined&&language!=="en"&&language!=="de")throw Error(both(`Unknown language: ${language} (en|de)`,`Unbekannte Sprache: ${language} (en|de)`));
 return Object.freeze({scenario:picked===undefined||picked.includes("all")?"all":picked,inPlace:flags.has("--in-place"),file:values["--file"],markdown:values["--markdown"],json:flags.has("--json"),language:language as "en"|"de"|undefined});
}

/** 🍭 What `repo bench acceptance` needs from the host: the way to print, to write files and to read the clock, the environment, the host sample and the official runs for a language. */
export type AcceptanceCommandPortsV1=Readonly<{
 write(line:string):void;
 writeText(path:string,text:string):Promise<void>;
 environment:Readonly<Record<string,string|undefined>>;
 isoNow():string;
 host():BenchHostV1;
 runs(language:"en"|"de"):AcceptancePortsV1;
}>;

/** 🍮 Runs `repo bench acceptance`: selects the scenarios, runs them through the official route, judges every budget and invariant, prints the report as text or JSON, writes the JSON and markdown files and fails (after the files were written) when a blocking budget is broken or none was proven. A cancellation reports what was measured, restores every edit and then fails. */
export async function runAcceptanceCommandV1(args:readonly string[],ports:AcceptanceCommandPortsV1,set:AcceptanceSetV1=budgets as unknown as AcceptanceSetV1):Promise<AcceptanceReportV1>{
 const parsed=acceptanceArgumentsV1(args),issues=acceptanceSetIssuesV1(set);
 if(issues.length)throw Error(`Closed acceptance table required: ${issues.join("; ")}`);
 const language=incrementalLanguageV1(parsed.language,ports.environment),selection=acceptanceSelectV1(set,parsed.scenario,parsed.inPlace),runs=ports.runs(language);
 if(!selection.run.length)throw Error(both("No scenario is left to run","Es bleibt kein Szenario zum Ausführen übrig"));
 if(selection.skipped.length)runs.note(both(`Skipped without --in-place: ${selection.skipped.map(row=>row.id).join(", ")}`,`Ohne --in-place übersprungen: ${selection.skipped.map(row=>row.id).join(", ")}`));
 const startedAt=ports.isoNow(),host=ports.host(),measured=await acceptanceRunV1(set,selection.run,runs);
 const skipped:AcceptanceOutcomeV1[]=selection.skipped.map(row=>({scenario:row.id,status:"skipped",reason:null,restored:null,landed:null,merged:null})),outcomes=[...measured.outcomes,...skipped],selected=[...selection.run,...selection.skipped].map(row=>row.id);
 const report=acceptanceReportV1({startedAt,finishedAt:ports.isoNow(),host,hostAfter:ports.host(),language,selected,records:measured.records,outcomes,rows:acceptanceJudgeV1(set,selected,measured.records,outcomes)});
 if(parsed.file)await ports.writeText(parsed.file,JSON.stringify(report,null,2)+"\n");
 if(parsed.markdown)await ports.writeText(parsed.markdown,renderAcceptanceReportV1(report,set,language,"markdown")+"\n");
 ports.write(parsed.json?JSON.stringify(report,null,2):renderAcceptanceReportV1(report,set,language,"text"));
 if(measured.aborted)throw runs.signal.reason instanceof Error?runs.signal.reason:Error(both("The acceptance run was cancelled","Der Abnahmelauf wurde abgebrochen"));
 const failure=acceptanceFailureV1(report);
 if(failure)throw Error(failure);
 return report;
}

/** 🌃 Builds the environment of an official child run: the parent's environment without any inherited invocation, capability, build or Nx state (as the incremental harness does), the language of the report, and the few build variables an acceptance run declares on purpose (the remote cache, the host capacity declaration, the build budget). */
export function acceptanceEnvironmentV1(parent:Readonly<Record<string,string|undefined>>,language:"en"|"de"):Record<string,string>{
 const kept=incrementalChildEnvironmentV1(parent,{SEMIO_LANGUAGE:language});
 for(const [key,value] of Object.entries(parent))if(value!==undefined&&(policy.environment.keep.includes(key)||policy.environment.prefixes.some(prefix=>key.startsWith(prefix))))kept[key]=value;
 return kept;
}

/** 🌮 The command line of one official run of a product target: the runtime that runs this command, the door of the build and the neutral `repo run` of the target under the pinned implementation. */
export function acceptanceCommandLineV1(root:string,executable:string,target:string):{command:string;args:string[]}{
 return {command:executable,args:[join(root,policy.door),"repo",`--tool=${policy.tool}`,"run",target]};
}

/** 🍲 Binds the command to the running build CLI: its invocation for progress and cancellation, the checkout as the place official runs happen and edits are restored, real clocks and the host. */
export async function acceptanceCommandPortsV1(root:string,invocation:import("@@PROCESS@@/@@ROUTING@@").ScriptInvocation):Promise<AcceptanceCommandPortsV1>{
 const routing=await import("@@PROCESS@@/@@ROUTING@@"),journal=join(root,policy.journalDirectory,policy.journalFile);
 const note=(line:string)=>{process.stderr.write(line+"\n");};
 const absolute=(path:string)=>{const target=join(root,path);if(target.length>policy.maximumPathLength)throw Error(`Path exceeds ${policy.maximumPathLength} characters: ${target}`);return target;};
 const files={
  read:async(path:string):Promise<Uint8Array|null>=>{try{return await readFile(absolute(path));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}},
  write:async(path:string,bytes:Uint8Array):Promise<void>=>{await writeFile(absolute(path),bytes);}
 };
 const store={
  read:async()=>{try{return (JSON.parse(await readFile(journal,"utf8")) as {rows:{file:string;original:string}[]}).rows;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return [];throw error;}},
  write:async(rows:readonly {file:string;original:string}[])=>{await mkdir(dirname(journal),{recursive:true});await writeFile(journal,JSON.stringify({version:1,rows}));},
  clear:async()=>{await rm(journal,{force:true});}
 };
 const host=():BenchHostV1=>acceptanceHostV1({platform:platform(),arch:arch(),cpus:Math.max(1,cpus().length),memoryBytes:totalmem(),load1:platform()==="win32"?null:loadavg()[0]??null});
 return Object.freeze({
  write:(line:string)=>{process.stdout.write(line+"\n");},
  writeText:async(path:string,text:string)=>{const target=resolve(path);if(target.length>policy.maximumPathLength)throw Error(`Path exceeds ${policy.maximumPathLength} characters: ${target}`);await mkdir(dirname(target),{recursive:true});await writeFile(target,text);},
  environment:process.env,
  isoNow:()=>new Date().toISOString(),
  host,
  runs:(language:"en"|"de"):AcceptancePortsV1=>Object.freeze({
   run:(request:AcceptanceRunRequestV1,signal:AbortSignal)=>{const line=acceptanceCommandLineV1(root,process.execPath,request.target);return acceptanceSpawnV1(line.command,line.args,root,acceptanceEnvironmentV1(process.env,language),request,signal);},
   files,
   journal:store,
   host:()=>({loadPerCore:host().loadPerCore}),
   nonce:()=>randomBytes(6).toString("hex"),
   now:()=>performance.now(),
   check:()=>routing.checkScriptInvocation(invocation),
   progress:(stage:string)=>routing.advanceScriptInvocation(invocation,"bench:acceptance:"+stage,"running"),
   note,
   signal:invocation.control.signal
  })
 });
}

/** 🌝 The entry `repo bench acceptance` is routed to: the arguments after `acceptance`. */
export async function runAcceptanceV1(root:string,args:readonly string[],invocation:import("@@PROCESS@@/@@ROUTING@@").ScriptInvocation):Promise<void>{
 await runAcceptanceCommandV1(args,await acceptanceCommandPortsV1(root,invocation));
}
