import {mkdir,mkdtemp,readFile,rm} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {advanceScriptInvocation,checkScriptInvocation,type ScriptInvocation} from "@@PROC@@/@@ROUTING@@/@@TS@@";
import {resolveLanguageV1} from "@@PROC@@/@@BUILDMOD@@/🗣️language/@@TS@@";
import scenarios from "../@@SCENARIOS@@/@@JSON@@" with {type:"json"};
import {buildBackendAlternationV1,buildBackendIdsV1,buildBackendRowV1} from "@@UP@@@@BACKENDS@@/@@TS@@";
import {conformanceIssuesV1,renderConformanceReportV1,type ConformanceSetV1} from "../@@TS@@";
import type {ConformanceDriverV1,ConformanceHostV1} from "../@@RUNNER@@/@@TS@@";

const operations=["list","plan","run"] as const,valuedOptions=Object.freeze(["--implementations","--scenario","--file","--lang"]),plainOptions=Object.freeze(["--json","--keep"]);
const both=(english:string,german:string)=>`${english}\n${german}`;
/** 📝️ The closed arguments of `repo conformance`. */
export type ConformanceArgumentsV1=Readonly<{operation:typeof operations[number];backends:readonly string[]|"all";scenarios:readonly string[]|"all";file:string|undefined;json:boolean;keep:boolean;language:"en"|"de"|undefined}>;

/** 🚩️ The option names `repo conformance` accepts, so the operations catalogue can be checked against the parser. */
export function conformanceOptionNamesV1():{valued:readonly string[];plain:readonly string[]}{return {valued:valuedOptions,plain:plainOptions};}

/** 🧷️ Reads `conformance list|plan|run` with its options; unknown, repeated and unvalued options are refused in both languages. */
export function conformanceArgumentsV1(args:readonly string[]):ConformanceArgumentsV1{
 const operation=args[0] as typeof operations[number];
 if(!operations.includes(operation))throw Error(both("Expected repo conformance list|plan|run [options]","Erwartet: repo conformance list|plan|run [Optionen]"));
 const seen=new Set<string>(),values:Record<string,string>={},flags=new Set<string>();
 for(let index=1;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(seen.has(name))throw Error(both(`Repeated option: ${name}`,`Wiederholte Option: ${name}`));
  seen.add(name);
  if(valuedOptions.includes(name)){const value=cut<0?args[++index]:argument.slice(cut+1);if(!value||value.startsWith("--"))throw Error(both(`Option ${name} needs a value`,`Option ${name} braucht einen Wert`));values[name]=value;}
  else if(plainOptions.includes(name)&&cut<0)flags.add(name);
  else throw Error(both(`Unknown conformance option: ${argument}`,`Unbekannte Option für conformance: ${argument}`));
 }
 const list=(name:string)=>values[name]?.split(",").filter(Boolean),backends=list("--implementations"),chosenScenarios=list("--scenario"),language=values["--lang"];
 for(const id of backends??[])if(id!=="all")buildBackendRowV1(id);
 const set=scenarios as unknown as ConformanceSetV1;
 for(const id of chosenScenarios??[])if(id!=="all"&&!set.scenarios.some(row=>row.id===id))throw Error(both(`Unknown scenario: ${id} (${set.scenarios.map(row=>row.id).join("|")})`,`Unbekanntes Szenario: ${id} (${set.scenarios.map(row=>row.id).join("|")})`));
 if(language!==undefined&&language!=="en"&&language!=="de")throw Error(both(`Unknown language: ${language} (en|de)`,`Unbekannte Sprache: ${language} (en|de)`));
 return Object.freeze({operation,backends:!backends||backends.includes("all")?"all":backends,scenarios:!chosenScenarios||chosenScenarios.includes("all")?"all":chosenScenarios,file:values["--file"],json:flags.has("--json"),keep:flags.has("--keep"),language:language as "en"|"de"|undefined});
}

/** 📋️ Lists the scenarios and what each backend claims for them. */
export function renderConformanceListV1(set:ConformanceSetV1,language:"en"|"de"):string{
 const ids=buildBackendIdsV1(),lines=[["scenario","capability",...ids].join(" | ")];
 for(const scenario of set.scenarios)lines.push([scenario.id,scenario.capability,...ids.map(id=>buildBackendRowV1(id).capabilities[scenario.capability]?.support??"missing")].join(" | "),"   "+scenario.title[language]+((scenario.evidence??[]).length?"  ["+scenario.evidence!.map(item=>item.suite+":"+item.id).join(", ")+"]":""));
 return lines.join("\n")+"\n";
}

/** 🚪️ The entry the build CLI calls for `repo conformance`. */
export async function runConformanceCommandV1(root:string,args:readonly string[],invocation:ScriptInvocation):Promise<void>{
 const parsed=conformanceArgumentsV1(args),set=scenarios as unknown as ConformanceSetV1,issues=conformanceIssuesV1(set);
 if(issues.length)throw Error(both("The conformance scenarios are inconsistent:\n","Die Konformitätsszenarien sind widersprüchlich:\n")+issues.join("\n"));
 const language=parsed.language??(resolveLanguageV1(["en","de"],args,process.env).language as "en"|"de");
 if(parsed.operation==="list"){console.log(renderConformanceListV1(set,language));return;}
 const backends=parsed.backends==="all"?buildBackendIdsV1():parsed.backends,chosen=parsed.scenarios==="all"?set.scenarios.map(row=>row.id):parsed.scenarios;
 if(parsed.operation==="plan"){console.log(backends.flatMap(backend=>chosen.map(scenario=>`${backend}: ${scenario}`)).join("\n"));return;}
 const base=join(tmpdir(),"conformance-run");await mkdir(base,{recursive:true});
 const scratch=await mkdtemp(join(base,"r-")),remotes:{stop():Promise<void>}[]=[];
 const host:ConformanceHostV1={scratch,runtime:process.execPath,stubSource:await readStub(),signal:invocation.control.signal,
  loadDriver:async id=>{const row=buildBackendRowV1(id);const module=await import(new URL(row.driver,import.meta.url).href) as {driver:ConformanceDriverV1};return module.driver;},
  startRemote:async()=>{
   const {startRemoteCacheServerV1}=await import("@@UP@@@@REMOTECACHE@@/🖥️server/@@TS@@"),server=await startRemoteCacheServerV1({directory:join(scratch,"remote-"+remotes.length),host:"127.0.0.1",port:0,maximumBytes:1<<30,maximumEntryBytes:1<<28,maximumAgeDays:1,defaultInstance:"main",tokens:{}});
   remotes.push(server);return {url:server.url+"/main",stop:()=>server.stop()};
  },
  now:()=>Date.now(),
  progress:async stage=>{checkScriptInvocation(invocation);await advanceScriptInvocation(invocation,"conformance:"+stage,"running");}};
 try{
  const {runConformanceV1:run}=await import("../@@RUNNER@@/@@TS@@"),report=await run(set,backends,chosen,host);
  const text=parsed.json?JSON.stringify(report,null,2):renderConformanceReportV1(report,set,language);
  if(parsed.file){const {withRepositoryBuildReportV1}=await import("@@UP@@@@REPORTING@@/@@TS@@");await withRepositoryBuildReportV1(root,parsed.file,report,invocation,async artifact=>{console.log(artifact.path);});}
  else console.log(text);
  if(report.failed>0)process.exitCode=1;
 }finally{
  for(const remote of remotes)await remote.stop().catch(()=>undefined);
  if(!parsed.keep)await rm(scratch,{recursive:true,force:true});
 }
}

async function readStub():Promise<string>{return readFile(new URL("../@@STUB@@/@@TS@@",import.meta.url),"utf8");}
