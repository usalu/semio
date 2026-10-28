import {buildBackendAlternationV1,buildBackendIdsV1} from "../../🔌️backends/🟦️.ts";

import {access,mkdir,mkdtemp,readdir,rm} from "node:fs/promises";
import {homedir,tmpdir} from "node:os";
import {join,posix,win32} from "node:path";
import {resolveLanguageV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import scenarios from "../📋️scenarios/🔣️.json" with {type:"json"};
import {incrementalExpectedV1,incrementalReportV1,incrementalScenarioIssuesV1,renderIncrementalReportV1,renderIncrementalScenariosV1,type IncrementalExpectedV1,type IncrementalPhaseV1,type IncrementalScenarioSetV1,type IncrementalToolV1} from "../🟦️.ts";
import {incrementalChildEnvironmentV1,incrementalCliArgumentsV1,incrementalCliPortsV1,incrementalCloneV1,incrementalDiscardV1,incrementalRefreshV1,incrementalRunV1,incrementalSelectV1,type IncrementalAdapterV1,type IncrementalHostV1} from "../🏃️runner/🟦️.ts";
import { startToolV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";

const policy=schema.definitions.Policy.const,tools:readonly IncrementalToolV1[]=buildBackendIdsV1(),operations=["list","plan","prepare","run","discard"] as const,script="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts";

/** 📝️ The closed arguments of `repo incremental`. */
export type IncrementalArgumentsV1=Readonly<{operation:typeof operations[number];scenarios:readonly string[]|"all";tools:readonly IncrementalToolV1[];root:string|undefined;phases:readonly IncrementalPhaseV1[];file:string|undefined;json:boolean;fresh:boolean;language:"en"|"de"|undefined}>;

const both=(english:string,german:string)=>`${english}\n${german}`,valuedOptions=Object.freeze(["--scenario","--implementations","--root","--phases","--file","--lang"]),plainOptions=Object.freeze(["--json","--fresh"]);

/** 🚩️ The option names `repo incremental` accepts, so the operations catalogue can be checked against the parser. */
export function incrementalOptionNamesV1():{valued:readonly string[];plain:readonly string[]}{return {valued:valuedOptions,plain:plainOptions};}

/** 🧷️ Reads `incremental list|plan|prepare|run|discard` with its options; every unknown or repeated option is refused in both languages. */
export function incrementalArgumentsV1(args:readonly string[]):IncrementalArgumentsV1{
 const operation=args[0] as typeof operations[number];
 if(!operations.includes(operation)||args.length>1&&args[1]!.startsWith("-")===false)throw Error(both("Expected repo incremental list|plan|prepare|run|discard [options]","Erwartet: repo incremental list|plan|prepare|run|discard [Optionen]"));
 const seen=new Set<string>(),values:Record<string,string>={},flags=new Set<string>(),valued=new Set(valuedOptions),plain=new Set(plainOptions);
 for(let index=1;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(seen.has(name))throw Error(both(`Repeated option: ${name}`,`Wiederholte Option: ${name}`));
  seen.add(name);
  if(valued.has(name)){const value=cut<0?args[++index]:argument.slice(cut+1);if(!value||value.startsWith("--"))throw Error(both(`Option ${name} needs a value`,`Option ${name} braucht einen Wert`));values[name]=value;}
  else if(plain.has(name)&&cut<0)flags.add(name);
  else throw Error(both(`Unknown incremental option: ${argument}`,`Unbekannte Option für incremental: ${argument}`));
 }
 const list=(name:string)=>values[name]?.split(",").filter(Boolean);
 const chosen=list("--implementations")??[...tools],phases=list("--phases")??[...policy.phases],language=values["--lang"];
 for(const tool of chosen)if(!tools.includes(tool as IncrementalToolV1))throw Error(both(`Unknown implementation: ${tool} (${buildBackendAlternationV1()})`,`Unbekannte Implementierung: ${tool} (${buildBackendAlternationV1()})`));
 for(const phase of phases)if(!policy.phases.includes(phase))throw Error(both(`Unknown phase: ${phase} (${policy.phases.join("|")})`,`Unbekannte Phase: ${phase} (${policy.phases.join("|")})`));
 if(language!==undefined&&language!=="en"&&language!=="de")throw Error(both(`Unknown language: ${language} (en|de)`,`Unbekannte Sprache: ${language} (en|de)`));
 const picked=list("--scenario");
 return Object.freeze({operation,scenarios:picked===undefined||picked.includes("all")?"all":picked,tools:[...new Set(chosen)] as IncrementalToolV1[],root:values["--root"],phases:[...new Set(phases)] as IncrementalPhaseV1[],file:values["--file"],json:flags.has("--json"),fresh:flags.has("--fresh"),language:language as "en"|"de"|undefined});
}

/** 📍️ The default location of the disposable root: the folder `.sinc` in the home directory (outside the scratch directories whose idle cache images the boot recovery detaches), else the folder `sinc` in the temporary directory of the process (inside a test sandbox that is the sandbox directory), each while its path keeps `maximumRootLength` units so the longest product path still fits the 255-unit budget; otherwise the short `sinc` folder at the top of the temporary volume (`/tmp`, or the drive root of the temporary directory on Windows). */
export function incrementalDefaultRootV1(platform:string,temporaryDirectory:string,home:string):string{
 const flavour=platform==="win32"?win32:posix,candidates=platform==="win32"?[flavour.join(temporaryDirectory,"sinc")]:[flavour.join(home,".sinc"),flavour.join(temporaryDirectory,"sinc")];
 return candidates.find(root=>root.length<=policy.maximumRootLength)??flavour.join(platform==="win32"?flavour.parse(temporaryDirectory).root||"C:\\":"/tmp","sinc");
}

/** 🎛️ Everything the command needs from the host process, so the command itself is testable without a build CLI. */
export type IncrementalCommandPortsV1=Readonly<{
 write(line:string):void;
 progress(stage:string):Promise<void>;
 check():void;
 signal:AbortSignal;
 environment:Readonly<Record<string,string|undefined>>;
 platform:string;
 tmpdir():string;
 home():string;
 now():number;
 run(command:string,args:readonly string[],cwd:string,env:Readonly<Record<string,string>>,signal:AbortSignal):Promise<string>;
 report(file:string,value:unknown):Promise<void>;
 files(root:string,signal:AbortSignal):Promise<readonly string[]>;
 locks(root:string):void;
 adapter():Promise<IncrementalAdapterV1>;
 nxRecord(root:string):string;
 exists(path:string):Promise<boolean>;
 scratch():Promise<string>;
}>;

const text=(set:IncrementalScenarioSetV1,key:string,language:"en"|"de")=>set.labels[key]![language];
/** 🗣️ The language of the output: the requested one, else the shared rule of the CLI; no language is assumed. */
export function incrementalLanguageV1(requested:"en"|"de"|undefined,environment:Readonly<Record<string,string|undefined>>):"en"|"de"{return requested??resolveLanguageV1(["en","de"],[],environment).language as "en"|"de";}

const languageOf=(parsed:IncrementalArgumentsV1,ports:IncrementalCommandPortsV1):"en"|"de"=>incrementalLanguageV1(parsed.language,ports.environment);

/** 🗺️ Renders the expected re-execution sets of a plan, one block per scenario and tool. */
export function renderIncrementalPlanV1(set:IncrementalScenarioSetV1,rows:readonly Readonly<{scenario:string;tool:IncrementalToolV1;expected:IncrementalExpectedV1}>[],lang:"en"|"de"):string{
 const out:string[]=[];
 for(const row of rows)out.push(`${row.scenario} / ${row.tool}: ${text(set,"must",lang)} ${row.expected.must.length}, ${text(set,"may",lang)} ${row.expected.may.length}, ${text(set,"cutoff",lang)} ${row.expected.cutoff.length}`,...row.expected.must.map(id=>`  ${id}`));
 return out.join("\n");
}

const bootstrap:readonly (readonly string[])[]=[["preflight"],["tools","setup","all"],["dependencies","sync","--all"],["tools","setup","all"]];

/** 🚀️ Runs `repo incremental`: lists the scenarios, plans their expected sets, prepares the disposable root, runs the scenarios in it with every implementation and judges minimality, or discards the root. Progress is published per stage and every stage checks for cancellation. */
export async function runIncrementalCommandV1(live:string,args:readonly string[],ports:IncrementalCommandPortsV1):Promise<void>{
 const parsed=incrementalArgumentsV1(args.slice(args[0]==="incremental"?1:0)),set=scenarios as IncrementalScenarioSetV1,issues=incrementalScenarioIssuesV1(set);
 if(issues.length)throw Error(`Closed incremental scenarios required: ${issues.join("; ")}`);
 const lang=languageOf(parsed,ports),root=parsed.root??incrementalDefaultRootV1(ports.platform,ports.tmpdir(),ports.home());
 if(parsed.operation==="list"){ports.write(renderIncrementalScenariosV1(set,lang));return;}
 if(parsed.operation==="discard"){await ports.progress("discard");await incrementalDiscardV1(live,root,ports.signal);ports.write(`${text(set,"progress",lang)}: ${root}`);return;}
 const scratch=await ports.scratch(),exists=await ports.exists(join(root,script));
 const host=(files?:readonly string[]):IncrementalHostV1=>({live,root,script,executable:process.execPath,scratch,signal:ports.signal,environment:ports.environment,run:ports.run,nxRecord:ports.nxRecord,adapter:ports.adapter,inventory:async()=>files??await ports.files(live,ports.signal),now:ports.now,check:ports.check,progress:ports.progress});
 try{
  if(parsed.operation==="prepare"){
   ports.check();
   ports.locks(live);
   if(parsed.fresh)await incrementalDiscardV1(live,root,ports.signal);
   const files=await ports.files(live,ports.signal);
   if(!exists||parsed.fresh)await incrementalCloneV1(live,root,files,{signal:ports.signal,progress:async(done,total)=>{await ports.progress(`clone:${done}/${total}`);}});
   else await incrementalRefreshV1(live,root,files,{signal:ports.signal,progress:async(done,total)=>{await ports.progress(`refresh:${done}/${total}`);}});
   const pre=host();
   for(const step of bootstrap){ports.check();await ports.progress(`bootstrap:${step.join(" ")}`);const cli=incrementalCliArgumentsV1(pre.executable,root,script,tools[0]!,step);await ports.run(cli.command,cli.args,root,incrementalChildEnvironmentV1(ports.environment,{}),ports.signal);}
   ports.write(`${text(set,"progress",lang)}: ${root}`);
   return;
  }
  if(!exists)throw Error(both(`Prepare the disposable root first: repo incremental prepare --root=${root}`,`Bereite zuerst das Wegwerf-Verzeichnis vor: repo incremental prepare --root=${root}`));
  const selected=incrementalSelectV1(set,parsed.scenarios),files=await ports.files(live,ports.signal),port=incrementalCliPortsV1(host(files));
  if(parsed.operation==="plan"){
   const rows=[];
   for(const scenario of selected)for(const tool of parsed.tools.filter(candidate=>!scenario.tools||scenario.tools.includes(candidate))){
    ports.check();await ports.progress(`${scenario.id}:${tool}:plan`);
    const slice=await port.slice(scenario,tool);
    rows.push({scenario:scenario.id,tool,expected:incrementalExpectedV1(slice,tool,{kind:scenario.kind,files:[scenario.mutation.file]},[...(scenario.effects??[]),...set.sensitivity])});
   }
   if(parsed.json)ports.write(JSON.stringify(rows,null,2));else ports.write(renderIncrementalPlanV1(set,rows,lang));
   return;
  }
  const rows=await incrementalRunV1(set,selected,parsed.tools,parsed.phases,port),report=incrementalReportV1(rows);
  if(parsed.file)await ports.report(parsed.file,report);
  ports.write(parsed.json?JSON.stringify(report,null,2):renderIncrementalReportV1(report,set,lang));
  if(report.failed)throw Error(both(`Incremental conformance failed in ${report.failed} of ${report.rows.length} runs`,`Inkrementelle Konformität verletzt in ${report.failed} von ${report.rows.length} Läufen`));
 }finally{await rm(scratch,{recursive:true,force:true});}
}

/** 🏗️ Binds the command to the running build CLI: its invocation for progress and cancellation, its owned process execution, its report store and the live Bazel adapter. */
export async function incrementalCommandPortsV1(root:string,invocation:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<IncrementalCommandPortsV1>{
 const wasm=await import("../../../⚡️caching/🦀️cargo/🕸️wasm-bindgen/🟦️.ts"),nx=await import("../../../⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts"),routing=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),execution=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
 return Object.freeze({
  write:writeResultLineV1,
  progress:stage=>routing.advanceScriptInvocation(invocation,"incremental:"+stage,"running"),
  check:()=>routing.checkScriptInvocation(invocation),
  signal:invocation.control.signal,
  environment:process.env,
  platform:process.platform,
  tmpdir:()=>tmpdir(),
  home:()=>homedir(),
  now:()=>Date.now(),
  run:(command,args,cwd,env,signal)=>execution.captureOwnedCommandV1(command,args,cwd,{env,signal}),
  report:async(file,value)=>{const{withRepositoryBuildReportV1}=await import("../../📊️reporting/🟦️.ts");await withRepositoryBuildReportV1(root,file,value,invocation,async artifact=>{writeResultLineV1(artifact.path);});},
  files:(directory,signal)=>incrementalTrackedFilesV1(directory,signal),
  locks:directory=>wasm.requireWasmBindgenLocksV1(directory),
  adapter:async()=>await import("../../🔧️tools/🏗️bazel/📜️script.ts") as unknown as IncrementalAdapterV1,
  nxRecord:directory=>join(nx.nxCacheDirectoryV1(directory),"run.json"),
  exists:async path=>access(path).then(()=>true,()=>false),
  scratch:async()=>{const directory=join(tmpdir(),"sinc-run");await mkdir(directory,{recursive:true});return mkdtemp(join(directory,"r-"));}
 });
}

/** 📜️ Runs one read-only, bounded and cancellable `git ls-files` and returns its NUL separated paths. */
function incrementalGitListV1(directory:string,arguments_:readonly string[],signal:AbortSignal):Promise<readonly string[]>{
 return new Promise((accept,reject)=>{
  const child=startToolV1("git",["-C",directory,"ls-files","-z",...arguments_],{stdio:["ignore","pipe","pipe"],windowsHide:true,signal}),chunks:Buffer[]=[];let bytes=0;
  child.stdout.on("data",(chunk:Buffer)=>{bytes+=chunk.length;if(bytes>policy.maximumLogBytes){child.kill();reject(Error("Bounded file inventory required"));}else chunks.push(chunk);});
  child.on("error",reject);
  child.on("close",code=>code===0?accept(Buffer.concat(chunks).toString("utf8").split("\0").filter(Boolean)):reject(Error(`git ls-files failed (${code})`)));
 });
}

/** 🤖️ Expands the ignored entries a copy still needs (generated outputs the build graph reads) into files: a listed directory is walked, a link stays a link. */
async function incrementalIgnoredFilesV1(directory:string,entries:readonly string[]):Promise<readonly string[]>{
 const rule=policy.includedIgnored,found:string[]=[],walk=async(relativePath:string):Promise<void>=>{for(const entry of await readdir(join(directory,relativePath),{withFileTypes:true})){const child=posix.join(relativePath,entry.name);if(entry.isDirectory())await walk(child);else found.push(child);}};
 for(const entry of entries){
  const directoryEntry=entry.endsWith("/"),name=posix.basename(directoryEntry?entry.slice(0,-1):entry);
  if(directoryEntry?rule.directoryNames.includes(name):rule.filePrefixes.some(prefix=>name.startsWith(prefix))||rule.files.includes(entry))directoryEntry?await walk(entry.slice(0,-1)):found.push(entry);
 }
 return found;
}

/** 🛤️ Lists the tracked and untracked, non-ignored files of a checkout plus its ignored generated outputs, without the ticket workspace, read-only, bounded and cancellable. */
export async function incrementalTrackedFilesV1(directory:string,signal:AbortSignal):Promise<readonly string[]>{
 const [listed,ignored]=await Promise.all([incrementalGitListV1(directory,["-co","--exclude-standard"],signal),incrementalGitListV1(directory,["-o","-i","--exclude-standard","--directory"],signal)]);
 const files=new Set([...listed,...await incrementalIgnoredFilesV1(directory,ignored)]);
 return [...files].filter(file=>!policy.excludedPrefixes.some(prefix=>file.startsWith(prefix))).sort();
}

/** 🚪️ The entry the build CLI calls for `repo incremental`. */
export async function runIncrementalV1(root:string,args:readonly string[],invocation:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<void>{
 await runIncrementalCommandV1(root,args,await incrementalCommandPortsV1(root,invocation));
}
