import {mkdir,readFile,readdir,rm,writeFile} from "node:fs/promises";
import {arch,cpus,platform,totalmem} from "node:os";
import {dirname,join,resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import slo from "../🎯️slo/🔣️.json" with {type:"json"};
import {incrementalDisposableV1} from "../../🏃️runner/🟦️.ts";
import {incrementalCommandPortsV1,incrementalDefaultRootV1,incrementalLanguageV1,runIncrementalCommandV1,type IncrementalCommandPortsV1} from "../../⌨️command/🟦️.ts";
import {benchBaselineV1,benchBudgetV1,benchCompareV1,benchHistoryLatestV1,benchHistoryNameV1,benchHostV1,benchSelectV1,benchSloIssuesV1,renderBenchBudgetV1,renderBenchComparisonV1,renderBenchListV1,renderBenchReportV1,type BenchReportV1,type BenchSloSetV1,type BenchToolV1} from "../🟦️.ts";
import {benchCliPortsV1,benchRunV1,type BenchHostPortsV1} from "../🏃️runner/🟦️.ts";

const policy=schema.definitions.Policy.const,tools:readonly BenchToolV1[]=["nx","bazel"],operations=["list","run","compare"] as const,script="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts",catalog="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json";

/** 🍘️ The closed arguments of `repo bench`. */
export type BenchArgumentsV1=Readonly<{operation:typeof operations[number];slo:readonly string[]|"all";runs:number|undefined;baseline:string|undefined;saveBaseline:string|undefined;current:string|undefined;history:string|undefined;markdown:string|undefined;maxLoad:number|undefined;tools:readonly BenchToolV1[];root:string|undefined;file:string|undefined;json:boolean;prepare:boolean;language:"en"|"de"|undefined}>;

const both=(english:string,german:string)=>`${english}\n${german}`,valuedOptions=Object.freeze(["--slo","--runs","--baseline","--save-baseline","--current","--history","--markdown","--max-load","--implementations","--root","--file","--lang"]),plainOptions=Object.freeze(["--json","--prepare"]);

/** 🍙️ The option names `repo bench` accepts, so the operations catalogue can be checked against the parser. */
export function benchOptionNamesV1():{valued:readonly string[];plain:readonly string[]}{return {valued:valuedOptions,plain:plainOptions};}

/** 🍚️ Reads `bench list|run|compare` with its options; every unknown or repeated option is refused in both languages. */
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
 if(operation==="compare"&&(values["--history"]===undefined?!values["--current"]||!values["--baseline"]:values["--current"]!==undefined||values["--baseline"]!==undefined))throw Error(both("bench compare needs --current=<report> and --baseline=<report>, or --history=<folder> alone","bench compare braucht --current=<Bericht> und --baseline=<Bericht> oder allein --history=<Ordner>"));
 if(operation==="list"&&values["--history"]!==undefined)throw Error(both("--history belongs to bench run and bench compare","--history gehört zu bench run und bench compare"));
 if(operation!=="compare"&&values["--current"]!==undefined)throw Error(both("--current belongs to bench compare","--current gehört zu bench compare"));
 if(operation!=="run"&&(flags.has("--prepare")||values["--save-baseline"]!==undefined||runs!==undefined||maxLoad!==undefined))throw Error(both("--prepare, --runs, --max-load and --save-baseline belong to bench run","--prepare, --runs, --max-load und --save-baseline gehören zu bench run"));
 return Object.freeze({operation,slo:picked===undefined||picked.includes("all")?"all":picked,runs,baseline:values["--baseline"],saveBaseline:values["--save-baseline"],current:values["--current"],history:values["--history"],markdown:values["--markdown"],maxLoad,tools:[...new Set(chosen)] as BenchToolV1[],root:values["--root"],file:values["--file"],json:flags.has("--json"),prepare:flags.has("--prepare"),language:language as "en"|"de"|undefined});
}

/** 📮️ What `repo bench` needs besides the incremental command ports: reading and writing plain files such as the baseline and the markdown report. */
export type BenchCommandPortsV1=IncrementalCommandPortsV1&Readonly<{readText(path:string):Promise<string>;writeText(path:string,text:string):Promise<void>;list(folder:string):Promise<readonly string[]>;overrides?:Partial<BenchHostPortsV1>}>;

const readReport=async(ports:BenchCommandPortsV1,path:string):Promise<BenchReportV1>=>{
 let value:unknown;
 try{value=JSON.parse(await ports.readText(path));}catch(error){throw Error(both(`Cannot read the report ${path}: ${(error as Error).message}`,`Der Bericht ${path} ist nicht lesbar: ${(error as Error).message}`));}
 return benchBaselineV1(value);
};

/** 🍛️ Runs `repo bench`: lists the objectives, measures them in the prepared checkout copy with the chosen implementations and judges them, or compares two stored reports. Reports go to the screen as text or JSON and optionally to a JSON file, a markdown file and a baseline file; the exit code fails only for blocking objectives that failed or regressed. */
export async function runBenchCommandV1(live:string,args:readonly string[],ports:BenchCommandPortsV1):Promise<void>{
 const parsed=benchArgumentsV1(args.slice(args[0]==="bench"?1:0)),set=slo as unknown as BenchSloSetV1,issues=benchSloIssuesV1(set);
 if(issues.length)throw Error(`Closed SLO table required: ${issues.join("; ")}`);
 const lang=incrementalLanguageV1(parsed.language,ports.environment),root=parsed.root??incrementalDefaultRootV1(ports.platform,ports.tmpdir(),ports.home());
 if(parsed.operation==="list"){ports.write(parsed.json?JSON.stringify(set,null,2):renderBenchListV1(set,lang,"text"));return;}
 const hostClass=()=>(ports.overrides?.sample?.()??benchHostV1({platform:platform(),arch:arch(),cpus:Math.max(1,cpus().length),memoryBytes:totalmem(),load1:null})).hostClass;
 const emit=async(report:BenchReportV1|null,comparison:ReturnType<typeof benchCompareV1>|null,over:ReturnType<typeof benchBudgetV1>|null=null):Promise<void>=>{
  if(report&&parsed.file)await ports.report(parsed.file,report);
  if(report&&parsed.history)await ports.writeText(join(parsed.history,benchHistoryNameV1(report.startedAt,report.host.hostClass)),JSON.stringify(report,null,2)+"\n");
  if(report&&parsed.saveBaseline)await ports.writeText(parsed.saveBaseline,JSON.stringify(report,null,2)+"\n");
  if(parsed.markdown)await ports.writeText(parsed.markdown,(report?renderBenchReportV1(report,set,lang,"markdown"):renderBenchComparisonV1(comparison!,set,lang,"markdown")+(over?.rows.length?"\n\n"+renderBenchBudgetV1(over.rows,set,lang,"markdown"):""))+"\n");
  ports.write(parsed.json?JSON.stringify(report??comparison,null,2):report?renderBenchReportV1(report,set,lang,"text"):renderBenchComparisonV1(comparison!,set,lang,"text")+(over?.rows.length?"\n\n"+renderBenchBudgetV1(over.rows,set,lang,"text"):""));
 };
 if(parsed.operation==="compare"){
  await ports.progress("compare");
  let currentPath=parsed.current,baselinePath=parsed.baseline;
  if(parsed.history!==undefined){
   const files=await ports.list(parsed.history),newest=benchHistoryLatestV1(files,hostClass()),older=newest===null?null:benchHistoryLatestV1(files,hostClass(),newest);
   if(newest===null||older===null)throw Error(both(`The history ${parsed.history} holds fewer than two reports of this host class`,`Der Verlauf ${parsed.history} enthält weniger als zwei Berichte dieser Rechnerklasse`));
   currentPath=join(parsed.history,newest);baselinePath=join(parsed.history,older);
  }
  const current=await readReport(ports,currentPath!),comparison=benchCompareV1(set,current,await readReport(ports,baselinePath!)),over=benchBudgetV1(current);
  await emit(null,comparison,over);
  if(comparison.tally.blockingRegressed)throw Error(both(`Performance regressed in ${comparison.tally.blockingRegressed} blocking objectives`,`Die Leistung hat sich bei ${comparison.tally.blockingRegressed} verbindlichen Zielen verschlechtert`));
  if(over.blocking)throw Error(both(`${over.blocking} blocking objectives are over budget`,`${over.blocking} verbindliche Ziele liegen über dem Budget`));
  return;
 }
 const pairs=benchSelectV1(set,parsed.slo,parsed.tools),previous=parsed.baseline===undefined&&parsed.history!==undefined?benchHistoryLatestV1(await ports.list(parsed.history),hostClass()):null,baseline=parsed.baseline?await readReport(ports,parsed.baseline):previous===null?null:await readReport(ports,join(parsed.history!,previous));
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

/** 🔩️ Binds the command to the running build CLI: the incremental harness's ports with progress published under `bench:`, a high resolution clock and plain file access for baselines and reports. */
export async function benchCommandPortsV1(root:string,invocation:import("../../../../../../../..//🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<BenchCommandPortsV1>{
 const base=await incrementalCommandPortsV1(root,invocation),routing=await import("../../../../../../../..//🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 return Object.freeze({...base,now:()=>performance.now(),progress:(stage:string)=>routing.advanceScriptInvocation(invocation,"bench:"+stage,"running"),readText:(path:string)=>readFile(resolve(path),"utf8"),list:async(folder:string)=>readdir(resolve(folder)).catch(error=>{if((error as NodeJS.ErrnoException).code==="ENOENT")return [];throw error;}),writeText:async(path:string,text:string)=>{const target=resolve(path);if(target.length>policy.maximumPathLength)throw Error(`Path exceeds ${policy.maximumPathLength} characters: ${target}`);await mkdir(dirname(target),{recursive:true});await writeFile(target,text);}});
}

/** 🍜️ The entry the build CLI calls for `repo bench`. */
export async function runBenchV1(root:string,args:readonly string[],invocation:import("../../../../../../../..//🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<void>{
 await runBenchCommandV1(root,args,await benchCommandPortsV1(root,invocation));
}
