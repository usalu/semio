import {mkdir,readFile,rm,writeFile} from "node:fs/promises";
import {dirname,join,resolve} from "node:path";
import schema from "../@@SCHEMA@@/@@JSON@@" with {type:"json"};
import slo from "../@@SLO@@/@@JSON@@" with {type:"json"};
import {incrementalDisposableV1} from "../../@@RUNNER@@/@@TS@@";
import {incrementalCommandPortsV1,incrementalDefaultRootV1,incrementalLanguageV1,runIncrementalCommandV1,type IncrementalCommandPortsV1} from "../../@@COMMAND@@/@@TS@@";
import {benchBaselineV1,benchCompareV1,benchSelectV1,benchSloIssuesV1,renderBenchComparisonV1,renderBenchListV1,renderBenchReportV1,type BenchReportV1,type BenchSloSetV1,type BenchToolV1} from "../@@TS@@";
import {benchCliPortsV1,benchRunV1,type BenchHostPortsV1} from "../@@RUNNER@@/@@TS@@";

const policy=schema.definitions.Policy.const,tools:readonly BenchToolV1[]=["nx","bazel"],operations=["list","run","compare"] as const,script="@@BUILDSCRIPT@@",catalog="@@BUILDDIR@@/@@OPERATIONS@@/@@CATALOG@@";

/** 🧱 The closed arguments of `repo bench`. */
export type BenchArgumentsV1=Readonly<{operation:typeof operations[number];slo:readonly string[]|"all";runs:number|undefined;baseline:string|undefined;saveBaseline:string|undefined;current:string|undefined;markdown:string|undefined;maxLoad:number|undefined;tools:readonly BenchToolV1[];root:string|undefined;file:string|undefined;json:boolean;prepare:boolean;language:"en"|"de"|undefined}>;

const both=(english:string,german:string)=>`${english}\n${german}`,valuedOptions=Object.freeze(["--slo","--runs","--baseline","--save-baseline","--current","--markdown","--max-load","--implementations","--root","--file","--lang"]),plainOptions=Object.freeze(["--json","--prepare"]);

/** 🧲 The option names `repo bench` accepts, so the operations catalogue can be checked against the parser. */
export function benchOptionNamesV1():{valued:readonly string[];plain:readonly string[]}{return {valued:valuedOptions,plain:plainOptions};}

/** 🧶 Reads `bench list|run|compare` with its options; every unknown or repeated option is refused in both languages. */
export function benchArgumentsV1(args:readonly string[]):BenchArgumentsV1{
 const operation=args[0] as typeof operations[number];
 if(!operations.includes(operation)||args.length>1&&args[1]!.startsWith("-")===false)throw Error(both("Expected repo bench list|run|compare [options]","Erwartet: repo bench list|run|compare [Optionen]"));
 const seen=new Set<string>(),values:Record<string,string>={},flags=new Set<string>(),valued=new Set(valuedOptions),plain=new Set(plainOptions);
 for(let index=1;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(seen.has(name))throw Error(both(`Repeated option: ${name}`,`Wiederholte Option: ${name}`));
  seen.add(name);
  if(valued.has(name)){const value=cut<0?args[++index]:argument.slice(cut+1);if(!value||value.startsWith("--"))throw Error(both(`Option ${name} needs a value`,`Option ${name} braucht einen Wert`));values[name]=value;}
  else if(plain.has(name)&&cut<0)flags.add(name);
  else throw Error(both(`Unknown bench option: ${argument}`,`Unbekannte Option für bench: ${argument}`));
 }
 const list=(name:string)=>values[name]?.split(",").filter(Boolean),chosen=list("--implementations")??[...tools],language=values["--lang"],picked=list("--slo");
 for(const tool of chosen)if(!tools.includes(tool as BenchToolV1))throw Error(both(`Unknown implementation: ${tool} (nx|bazel)`,`Unbekannte Implementierung: ${tool} (nx|bazel)`));
 if(language!==undefined&&language!=="en"&&language!=="de")throw Error(both(`Unknown language: ${language} (en|de)`,`Unbekannte Sprache: ${language} (en|de)`));
 const runs=values["--runs"]===undefined?undefined:Number(values["--runs"]);
 if(runs!==undefined&&(!/^[1-9][0-9]*$/u.test(values["--runs"]!)||runs>policy.maximumRuns))throw Error(both(`Runs must be a whole number from 1 to ${policy.maximumRuns}`,`Läufe müssen eine ganze Zahl von 1 bis ${policy.maximumRuns} sein`));
 const maxLoad=values["--max-load"]===undefined?undefined:Number(values["--max-load"]);
 if(maxLoad!==undefined&&(!/^[0-9]+(\.[0-9]+)?$/u.test(values["--max-load"]!)||!(maxLoad>0)))throw Error(both("The load limit must be a positive number (load per core)","Die Lastgrenze muss eine positive Zahl sein (Last je Kern)"));
 if(operation==="compare"&&(!values["--current"]||!values["--baseline"]))throw Error(both("bench compare needs --current=<report> and --baseline=<report>","bench compare braucht --current=<Bericht> und --baseline=<Bericht>"));
 if(operation!=="compare"&&values["--current"]!==undefined)throw Error(both("--current belongs to bench compare","--current gehört zu bench compare"));
 if(operation!=="run"&&(flags.has("--prepare")||values["--save-baseline"]!==undefined||runs!==undefined||maxLoad!==undefined))throw Error(both("--prepare, --runs, --max-load and --save-baseline belong to bench run","--prepare, --runs, --max-load und --save-baseline gehören zu bench run"));
 return Object.freeze({operation,slo:picked===undefined||picked.includes("all")?"all":picked,runs,baseline:values["--baseline"],saveBaseline:values["--save-baseline"],current:values["--current"],markdown:values["--markdown"],maxLoad,tools:[...new Set(chosen)] as BenchToolV1[],root:values["--root"],file:values["--file"],json:flags.has("--json"),prepare:flags.has("--prepare"),language:language as "en"|"de"|undefined});
}

/** 📮 What `repo bench` needs besides the incremental command ports: reading and writing plain files such as the baseline and the markdown report. */
export type BenchCommandPortsV1=IncrementalCommandPortsV1&Readonly<{readText(path:string):Promise<string>;writeText(path:string,text:string):Promise<void>;overrides?:Partial<BenchHostPortsV1>}>;

const readReport=async(ports:BenchCommandPortsV1,path:string):Promise<BenchReportV1>=>{
 let value:unknown;
 try{value=JSON.parse(await ports.readText(path));}catch(error){throw Error(both(`Cannot read the report ${path}: ${(error as Error).message}`,`Der Bericht ${path} ist nicht lesbar: ${(error as Error).message}`));}
 return benchBaselineV1(value);
};

/** 🚦 Runs `repo bench`: lists the objectives, measures them in the prepared checkout copy with the chosen implementations and judges them, or compares two stored reports. Reports go to the screen as text or JSON and optionally to a JSON file, a markdown file and a baseline file; the exit code fails only for blocking objectives that failed or regressed. */
export async function runBenchCommandV1(live:string,args:readonly string[],ports:BenchCommandPortsV1):Promise<void>{
 const parsed=benchArgumentsV1(args.slice(args[0]==="bench"?1:0)),set=slo as unknown as BenchSloSetV1,issues=benchSloIssuesV1(set);
 if(issues.length)throw Error(`Closed SLO table required: ${issues.join("; ")}`);
 const lang=incrementalLanguageV1(parsed.language,ports.environment),root=parsed.root??incrementalDefaultRootV1(ports.platform,ports.tmpdir());
 if(parsed.operation==="list"){ports.write(parsed.json?JSON.stringify(set,null,2):renderBenchListV1(set,lang,"text"));return;}
 const emit=async(report:BenchReportV1|null,comparison:ReturnType<typeof benchCompareV1>|null):Promise<void>=>{
  if(report&&parsed.file)await ports.report(parsed.file,report);
  if(report&&parsed.saveBaseline)await ports.writeText(parsed.saveBaseline,JSON.stringify(report,null,2)+"\n");
  if(parsed.markdown)await ports.writeText(parsed.markdown,(report?renderBenchReportV1(report,set,lang,"markdown"):renderBenchComparisonV1(comparison!,set,lang,"markdown"))+"\n");
  ports.write(parsed.json?JSON.stringify(report??comparison,null,2):report?renderBenchReportV1(report,set,lang,"text"):renderBenchComparisonV1(comparison!,set,lang,"text"));
 };
 if(parsed.operation==="compare"){
  await ports.progress("compare");
  const comparison=benchCompareV1(set,await readReport(ports,parsed.current!),await readReport(ports,parsed.baseline!));
  await emit(null,comparison);
  if(comparison.tally.blockingRegressed)throw Error(both(`Performance regressed in ${comparison.tally.blockingRegressed} blocking objectives`,`Die Leistung hat sich bei ${comparison.tally.blockingRegressed} verbindlichen Zielen verschlechtert`));
  return;
 }
 const pairs=benchSelectV1(set,parsed.slo,parsed.tools),baseline=parsed.baseline?await readReport(ports,parsed.baseline):null;
 await incrementalDisposableV1(live,root);
 if(parsed.prepare){
  await ports.progress("prepare");
  await runIncrementalCommandV1(live,["incremental","prepare","--fresh",`--root=${root}`],{...ports,write:()=>{},progress:stage=>ports.progress(`prepare:${stage}`)});
 }
 else if(!await ports.exists(join(root,script)))throw Error(both(`Prepare the disposable root first: repo incremental prepare --root=${root} (or pass --prepare)`,`Bereite zuerst das Wegwerf-Verzeichnis vor: repo incremental prepare --root=${root} (oder --prepare übergeben)`));
 const scratch=await ports.scratch();
 let files:Promise<readonly string[]>|undefined;
 try{
  const host:BenchHostPortsV1={live,root,script,executable:process.execPath,scratch,signal:ports.signal,environment:ports.environment,run:ports.run,nxRecord:ports.nxRecord,adapter:ports.adapter,inventory:()=>files??=ports.files(live,ports.signal),now:ports.now,check:ports.check,progress:ports.progress,fresh:parsed.prepare,catalog,commandPorts:ports,...ports.overrides};
  const report=await benchRunV1(set,pairs,{override:parsed.runs??null,tools:parsed.tools,maxLoad:parsed.maxLoad??null,baseline},benchCliPortsV1(host,ports.signal));
  await emit(report,report.comparison);
  if(report.tally.blockingFailed)throw Error(both(`${report.tally.blockingFailed} blocking performance objectives failed`,`${report.tally.blockingFailed} verbindliche Leistungsziele verletzt`));
  if(report.comparison?.tally.blockingRegressed)throw Error(both(`Performance regressed in ${report.comparison.tally.blockingRegressed} blocking objectives`,`Die Leistung hat sich bei ${report.comparison.tally.blockingRegressed} verbindlichen Zielen verschlechtert`));
 }finally{await rm(scratch,{recursive:true,force:true});}
}

/** 🔩 Binds the command to the running build CLI: the incremental harness's ports with progress published under `bench:`, a high resolution clock and plain file access for baselines and reports. */
export async function benchCommandPortsV1(root:string,invocation:import("@@EIGHT@@/@@MODULES@@/@@PROCESS@@/@@PACKAGES@@/@@TYPESCRIPT@@/@@ROUTING@@/@@TS@@").ScriptInvocation):Promise<BenchCommandPortsV1>{
 const base=await incrementalCommandPortsV1(root,invocation),routing=await import("@@EIGHT@@/@@MODULES@@/@@PROCESS@@/@@PACKAGES@@/@@TYPESCRIPT@@/@@ROUTING@@/@@TS@@");
 return Object.freeze({...base,now:()=>performance.now(),progress:stage=>routing.advanceScriptInvocation(invocation,"bench:"+stage,"running"),readText:path=>readFile(resolve(path),"utf8"),writeText:async(path,text)=>{const target=resolve(path);if(target.length>policy.maximumPathLength)throw Error(`Path exceeds ${policy.maximumPathLength} characters: ${target}`);await mkdir(dirname(target),{recursive:true});await writeFile(target,text);}});
}

/** 🛎️ The entry the build CLI calls for `repo bench`. */
export async function runBenchV1(root:string,args:readonly string[],invocation:import("@@EIGHT@@/@@MODULES@@/@@PROCESS@@/@@PACKAGES@@/@@TYPESCRIPT@@/@@ROUTING@@/@@TS@@").ScriptInvocation):Promise<void>{
 await runBenchCommandV1(root,args,await benchCommandPortsV1(root,invocation));
}
