import {mkdirSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import schema from "./@@SCHEMA@@/@@JSON@@" with {type:"json"};
import {validateJsonSchemaSubset} from "@@VALID@@";
import {buildBackendRowV1,type BackendCapabilityV1,type BackendSupportV1} from "@@UP@@@@BACKENDS@@/@@TS@@";
import {stubOutputsV1,type StubSpecV1,type StubTaskV1} from "./@@STUB@@/@@TS@@";
import type {BuildGraphV1} from "@@UP@@@@GRAPH@@/@@TS@@";

/** ⚙️ The authored constants of the conformance suite. */
export const CONFORMANCE_POLICY_V1=schema.definitions.Policy.const;
type Definitions=typeof schema.definitions;
/** 🧩️ A stub task of the conformance workspace. */
export type ConformanceTargetV1=Readonly<{cache:boolean;kind?:"build"|"test"|"check";dependsOn?:readonly string[];reads:readonly string[];outputs:readonly string[];action:Readonly<{mode:"emit"|"fail"|"sleep";milliseconds?:number;normalize?:boolean;report?:boolean}>}>;
/** 🏗️ A project of the conformance workspace. */
export type ConformanceProjectV1=Readonly<{name:string;dependsOn:readonly string[];targets:Readonly<Record<string,ConformanceTargetV1>>}>;
/** 🗂️ The stub workspace: source files and projects. */
export type ConformanceWorkspaceV1=Readonly<{files:Readonly<Record<string,string>>;projects:readonly ConformanceProjectV1[]}>;
/** 📏️ The expectation after one run. */
export type ConformanceExpectV1=Readonly<{exit?:"zero"|"nonzero";executed?:readonly string[];executedEach?:readonly string[];restored?:readonly string[];outputs?:readonly Readonly<{task:string;path:string;state:"expected"|"absent"|"unchanged"|"changed"}>[];ordered?:boolean;maxConcurrent?:number;minConcurrent?:number;events?:"covers-executed";startsOnce?:readonly string[]}>;
/** 🎚️ Options a run step can set. */
export type ConformanceRunOptionsV1=Readonly<{parallel?:number;skipCache?:boolean;remote?:"off"|"read-write"}>;
/** 👣️ One step of a scenario. */
export type ConformanceStepV1=Readonly<{step:"run";targets:readonly string[];options?:ConformanceRunOptionsV1;expect:ConformanceExpectV1}>|Readonly<{step:"edit";file:string;op:"append-line"|"append-comment"}>|Readonly<{step:"delete";paths:readonly string[]}>|Readonly<{step:"plan";target:string;tasks:readonly string[];edges:Readonly<Record<string,readonly string[]>>}>|Readonly<{step:"affected";files:readonly string[];projects:readonly string[]}>|Readonly<{step:"cancel";targets:readonly string[];afterStart:string;settled:string}>|Readonly<{step:"concurrent";runs:readonly (readonly string[])[];expect:ConformanceExpectV1}>|Readonly<{step:"fresh-checkout"}>;
/** 📖️ One scenario. */
export type ConformanceScenarioV1=Readonly<{id:string;capability:BackendCapabilityV1;title:Readonly<{en:string;de:string}>;evidence?:readonly Readonly<{suite:string;id:string}>[];steps:readonly ConformanceStepV1[]}>;
/** 📚️ Workspace and scenarios. */
export type ConformanceSetV1=Readonly<{version:1;workspace:ConformanceWorkspaceV1;scenarios:readonly ConformanceScenarioV1[]}>;
/** 📍️ A start or end a task body left. */
export type ConformanceMarkerV1=Readonly<{kind:"start"|"end"|"fail";task:string;at:number}>;
/** 📣️ A progress event a backend reported. */
export type ConformanceEventV1=Readonly<{kind:"start"|"end";task:string;status?:string}>;
/** 🏁️ What one run returned to the runner. */
export type ConformanceRunV1=Readonly<{exitCode:number;milliseconds:number;output?:string;restored?:readonly string[];events?:readonly ConformanceEventV1[]}>;
/** 🗺️ A plan a backend printed. */
export type ConformancePlanV1=Readonly<{tasks:readonly string[];dependencies:Readonly<Record<string,readonly string[]>>}>;
/** 🧾️ One judged scenario of one backend. */
export type ConformanceRowV1=Readonly<{scenario:string;backend:string;capability:string;claim:BackendSupportV1;measured:"pass"|"fail"|"unobserved"|"error";verdict:"pass"|"regression"|"gap"|"stale-claim"|"unobserved"|"error";failures:readonly string[];milliseconds:number}>;
/** 🧮️ All rows with the counts. */
export type ConformanceReportV1=Readonly<{version:1;rows:readonly ConformanceRowV1[];passed:number;failed:number;unobserved:number}>;

const order=(left:string,right:string)=>left<right?-1:left>right?1:0,sorted=(values:Iterable<string>)=>[...new Set(values)].sort(order);

/** 🔎️ Lists every way a scenario set breaks its own contract: schema, unknown tasks and paths, duplicate ids, capabilities without a scenario, and German texts equal to English. */
export function conformanceIssuesV1(value:unknown):string[]{
 const issues=validateJsonSchemaSubset(schema.definitions.ScenarioSet,value,schema).map(issue=>`schema: ${issue}`);
 if(issues.length)return issues;
 const set=value as ConformanceSetV1,tasks=new Set<string>(),outputs=new Map<string,string>(),projects=new Set(set.workspace.projects.map(project=>project.name));
 for(const project of set.workspace.projects){
  for(const dependency of project.dependsOn)if(!projects.has(dependency))issues.push(`unknown project dependency ${project.name} -> ${dependency}`);
  for(const[target,row]of Object.entries(project.targets)){
   const id=`${project.name}:${target}`;tasks.add(id);
   for(const path of row.outputs){
    if(!path.startsWith(project.name+"/"))issues.push(`output ${path} of ${id} leaves its project`);
    if(outputs.has(path))issues.push(`output ${path} declared by ${outputs.get(path)} and ${id}`);else outputs.set(path,id);
   }
  }
 }
 for(const project of set.workspace.projects)for(const[target,row]of Object.entries(project.targets))for(const dependency of row.dependsOn??[])if(!tasks.has(dependency))issues.push(`unknown task dependency ${project.name}:${target} -> ${dependency}`);
 const ids=new Set<string>(),known=(label:string,list:readonly string[]|undefined)=>{for(const id of list??[])if(!tasks.has(id))issues.push(`${label}: unknown task ${id}`);};
 for(const scenario of set.scenarios){
  if(ids.has(scenario.id))issues.push(`duplicate scenario ${scenario.id}`);ids.add(scenario.id);
  if(scenario.title.de===scenario.title.en)issues.push(`${scenario.id}: German title equals English`);
  for(const step of scenario.steps){
   if(step.step==="run"){known(scenario.id,step.targets);known(scenario.id,step.expect.executed);known(scenario.id,step.expect.restored);known(scenario.id,step.expect.startsOnce);for(const check of step.expect.outputs??[]){known(scenario.id,[check.task]);if(outputs.get(check.path)!==check.task)issues.push(`${scenario.id}: ${check.path} is not an output of ${check.task}`);}}
   else if(step.step==="concurrent"){for(const run of step.runs)known(scenario.id,run);known(scenario.id,step.expect.executed);}
   else if(step.step==="plan"){known(scenario.id,[step.target,...step.tasks,...Object.keys(step.edges)]);}
   else if(step.step==="cancel"){known(scenario.id,[...step.targets,step.afterStart,step.settled]);}
   else if(step.step==="affected"){for(const project of step.projects)if(!projects.has(project))issues.push(`${scenario.id}: unknown project ${project}`);}
  }
 }
 for(const capability of schema.definitions.Scenario.properties.capability.enum)if(!set.scenarios.some(scenario=>scenario.capability===capability))issues.push(`capability ${capability} has no scenario`);
 return issues;
}

/** 🧭️ The ids of every task of a workspace. */
export function conformanceTaskIdsV1(workspace:ConformanceWorkspaceV1):string[]{return workspace.projects.flatMap(project=>Object.keys(project.targets).map(target=>`${project.name}:${target}`)).sort(order);}

/** 🌳️ The dependency closure of tasks, the tasks themselves included. */
export function conformanceClosureV1(workspace:ConformanceWorkspaceV1,targets:readonly string[]):string[]{
 const rows=new Map<string,readonly string[]>();
 for(const project of workspace.projects)for(const[target,row]of Object.entries(project.targets))rows.set(`${project.name}:${target}`,row.dependsOn??[]);
 const seen=new Set<string>(),pending=[...targets];
 while(pending.length){const id=pending.pop()!;if(seen.has(id))continue;seen.add(id);pending.push(...rows.get(id)??[]);}
 return sorted(seen);
}

/** 📒️ The stub spec of a workspace: where markers go and what each task reads and writes. */
export function conformanceSpecV1(workspace:ConformanceWorkspaceV1,markers:string):StubSpecV1{
 const outputs=new Map<string,string[]>(),tasks:Record<string,StubTaskV1>={};
 for(const project of workspace.projects)for(const[target,row]of Object.entries(project.targets))outputs.set(`${project.name}:${target}`,[...row.outputs]);
 for(const project of workspace.projects)for(const[target,row]of Object.entries(project.targets)){
  const id=`${project.name}:${target}`;
  tasks[id]={id,reads:[...row.reads],outputs:[...row.outputs],dependencyOutputs:(row.dependsOn??[]).flatMap(dependency=>outputs.get(dependency)??[]),mode:row.action.mode,...(row.action.milliseconds?{milliseconds:row.action.milliseconds}:{}),...(row.action.normalize?{normalize:true}:{}),...(row.action.report?{report:true}:{})};
 }
 return {markers,tasks};
}

/** 📐️ The content every declared output must have, computed from the source files alone by the reference function: dependency outputs are computed, never read from disk. */
export function conformanceReferenceOutputsV1(spec:StubSpecV1,files:readonly string[],readSource:(path:string)=>string):Record<string,string>{
 const producer=new Map<string,string>(),done=new Map<string,Record<string,string>>();
 for(const task of Object.values(spec.tasks))for(const path of task.outputs)producer.set(path,task.id);
 const compute=(id:string):Record<string,string>=>{
  let known=done.get(id);
  if(!known){
   const task=spec.tasks[id]!,selected=sorted(files.filter(path=>task.reads.some(pattern=>new Bun.Glob(pattern).match(path))));
   known=stubOutputsV1(task,selected,path=>{const owner=producer.get(path);return owner?compute(owner)[path]!:readSource(path);});
   done.set(id,known);
  }
  return known;
 };
 return Object.assign({},...Object.keys(spec.tasks).map(compute));
}

/** 🕸️ The neutral build graph of the workspace, for backends that read it: each task runs the stub through the given runtime without a shell. */
export function conformanceGraphV1(workspace:ConformanceWorkspaceV1,directory:string,runtime:string,stub:string,spec:string):BuildGraphV1{
 const projects:BuildGraphV1["projects"]={};
 for(const project of workspace.projects){
  const targets:BuildGraphV1["projects"][string]["targets"]={};
  for(const[target,row]of Object.entries(project.targets)){
   const id=`${project.name}:${target}`;
   targets[target]={execution:{kind:"command",program:runtime,args:[stub,spec,id],cwd:directory},cache:row.cache,outputs:row.outputs.map(path=>`{workspaceRoot}/${path}`),inputs:[...row.reads.map(pattern=>`{workspaceRoot}/${pattern}`),...((row.dependsOn??[]).length?[{dependentTasksOutputFiles:"**/*"}]:[])],...((row.dependsOn??[]).length?{dependsOn:[...row.dependsOn!]}:{})};
  }
  projects[project.name]={name:project.name,root:project.name,targets};
 }
 return {version:1,projects,dependencies:workspace.projects.flatMap(project=>project.dependsOn.map(target=>({source:project.name,target,type:"static"})))};
}

/** 💾️ Writes the workspace's source files, the stub and its spec under a directory, and returns the spec. */
export function writeConformanceWorkspaceV1(workspace:ConformanceWorkspaceV1,directory:string,stubSource:string,markers:string):Readonly<{spec:StubSpecV1;stub:string;specPath:string}>{
 const folder=join(directory,CONFORMANCE_POLICY_V1.stubDirectory),stub=join(folder,"stub.ts"),specPath=join(folder,"spec.json"),spec=conformanceSpecV1(workspace,markers);
 for(const[path,body]of Object.entries(workspace.files)){mkdirSync(dirname(join(directory,path)),{recursive:true});writeFileSync(join(directory,path),body);}
 mkdirSync(folder,{recursive:true});writeFileSync(stub,stubSource);writeFileSync(specPath,JSON.stringify(spec,null,1));
 return {spec,stub,specPath};
}

/** 🔖️ Reads marker lines (`kind<TAB>task<TAB>epoch ms`) and ignores torn or foreign lines. */
export function conformanceMarkersV1(text:string):ConformanceMarkerV1[]{
 return text.split("\n").flatMap(line=>{const[kind,task,at]=line.split("\t");return (kind==="start"||kind==="end"||kind==="fail")&&task&&at&&Number.isFinite(Number(at))?[{kind,task,at:Number(at)}]:[];});
}

/** 🏃️ The tasks whose body started. */
export function conformanceExecutedV1(markers:readonly ConformanceMarkerV1[]):string[]{return sorted(markers.filter(marker=>marker.kind==="start").map(marker=>marker.task));}

/** 🧵️ The most task bodies that ran at the same time. */
export function conformanceConcurrencyV1(markers:readonly ConformanceMarkerV1[]):number{
 const ticks:(readonly [number,number])[]=markers.flatMap((marker):(readonly [number,number])[]=>marker.kind==="start"?[[marker.at,1]]:[[marker.at,-1]]).sort((left,right)=>left[0]-right[0]||left[1]-right[1]);
 let current=0,peak=0;
 for(const[,delta]of ticks){current+=delta;peak=Math.max(peak,current);}
 return peak;
}

/** 🔬️ What a run gave the evaluator: its exit code, the markers it left, the output hashes before and after, and optional backend records. */
export type ConformanceObservationV1=Readonly<{run:ConformanceRunV1;markers:readonly ConformanceMarkerV1[];before:Readonly<Record<string,string|null>>;after:Readonly<Record<string,string|null>>;expected:Readonly<Record<string,string>>}>;

/** ⚖️ Judges one run against its expectation. `failures` break the contract; `unobserved` names expectations the backend gave no way to check. */
export function conformanceEvaluateRunV1(expect:ConformanceExpectV1,observation:ConformanceObservationV1,edges:Readonly<Record<string,readonly string[]>>):{failures:string[];unobserved:string[]}{
 const failures:string[]=[],unobserved:string[]=[],{run,markers}=observation,executed=conformanceExecutedV1(markers);
 if(expect.exit==="zero"&&run.exitCode!==0)failures.push(`exit code ${run.exitCode}, expected 0`);
 if(expect.exit==="nonzero"&&run.exitCode===0)failures.push("exit code 0, expected a failure");
 if(expect.executed&&JSON.stringify(executed)!==JSON.stringify(sorted(expect.executed)))failures.push(`executed [${executed.join(", ")}], expected [${sorted(expect.executed).join(", ")}]`);
 if(expect.executedEach)for(const id of expect.executedEach)if(!executed.includes(id))failures.push(`${id} did not execute`);
 if(expect.startsOnce)for(const id of expect.startsOnce){const count=markers.filter(marker=>marker.kind==="start"&&marker.task===id).length;if(count!==1)failures.push(`${id} started ${count} times, expected once`);}
 if(expect.restored){if(run.restored===undefined)unobserved.push("restored");else if(JSON.stringify(sorted(run.restored))!==JSON.stringify(sorted(expect.restored)))failures.push(`restored [${sorted(run.restored).join(", ")}], expected [${sorted(expect.restored).join(", ")}]`);}
 for(const check of expect.outputs??[]){
  const now=observation.after[check.path]??null,was=observation.before[check.path]??null;
  if(check.state==="absent"&&now!==null)failures.push(`${check.path} exists, expected absent`);
  if(check.state==="expected"&&now!==observation.expected[check.path])failures.push(`${check.path} ${now===null?"is missing":"has unexpected content"}`);
  if(check.state==="unchanged"&&(now===null||now!==was))failures.push(`${check.path} ${now===null?"is missing":"changed"}, expected unchanged`);
  if(check.state==="changed"&&(now===null||now===was))failures.push(`${check.path} ${now===null?"is missing":"did not change"}, expected a change`);
 }
 if(expect.ordered){
  const end=new Map<string,number>(markers.filter(marker=>marker.kind==="end").map(marker=>[marker.task,marker.at]));
  for(const marker of markers.filter(marker=>marker.kind==="start"))for(const dependency of edges[marker.task]??[])if(end.has(dependency)&&end.get(dependency)!>marker.at)failures.push(`${marker.task} started before ${dependency} ended`);
 }
 const peak=conformanceConcurrencyV1(markers);
 if(expect.maxConcurrent!==undefined&&peak>expect.maxConcurrent)failures.push(`${peak} tasks ran at once, bound ${expect.maxConcurrent}`);
 if(expect.minConcurrent!==undefined&&peak<expect.minConcurrent)failures.push(`${peak} tasks ran at once, expected at least ${expect.minConcurrent}`);
 if(expect.events){
  if(run.events===undefined)unobserved.push("events");
  else for(const id of executed){for(const kind of ["start","end"] as const)if(!run.events.some(event=>event.task===id&&event.kind===kind))failures.push(`no ${kind} event for ${id}`);}
 }
 return {failures,unobserved};
}

/** 🏷️ Combines what a backend claims with what was measured: a claimed capability that fails is a regression, a missing one that fails is a documented gap, a missing one that passes is a stale claim. */
export function conformanceVerdictV1(claim:BackendSupportV1,measured:ConformanceRowV1["measured"]):ConformanceRowV1["verdict"]{
 if(measured==="error")return "error";
 if(measured==="unobserved")return "unobserved";
 if(claim==="missing")return measured==="pass"?"stale-claim":"gap";
 return measured==="pass"?"pass":"regression";
}

/** 📊️ Counts a row set: `pass` and a declared `gap` are satisfied, `unobserved` is counted apart because nothing was proven, everything else fails the report. */
export function conformanceReportV1(rows:readonly ConformanceRowV1[]):ConformanceReportV1{
 const passed=rows.filter(row=>row.verdict==="pass"||row.verdict==="gap").length,unobserved=rows.filter(row=>row.verdict==="unobserved").length;
 return Object.freeze({version:1,rows,passed,failed:rows.length-passed-unobserved,unobserved});
}

/** 🧷️ The registry claim of a backend for a capability; an undeclared capability counts as missing. */
export function conformanceClaimV1(backend:string,capability:BackendCapabilityV1):BackendSupportV1{return buildBackendRowV1(backend).capabilities[capability]?.support??"missing";}

/** 🌍️ Renders a report as a table in one language, with the failing details below it. */
export function renderConformanceReportV1(report:ConformanceReportV1,set:ConformanceSetV1,language:"en"|"de"):string{
 const title=(id:string)=>set.scenarios.find(scenario=>scenario.id===id)?.title[language]??id,words=language==="en"?{pass:"pass",regression:"REGRESSION",gap:"gap (declared)",["stale-claim"]:"STALE CLAIM",unobserved:"UNOBSERVED",error:"ERROR",head:["Scenario","Backend","Claim","Verdict"],totals:`${report.passed} satisfied, ${report.failed} failed, ${report.unobserved} unobserved`}:{pass:"bestanden",regression:"RÜCKSCHRITT",gap:"Lücke (erklärt)",["stale-claim"]:"VERALTETER ANSPRUCH",unobserved:"NICHT BEOBACHTET",error:"FEHLER",head:["Szenario","Backend","Anspruch","Urteil"],totals:`${report.passed} erfüllt, ${report.failed} fehlgeschlagen, ${report.unobserved} nicht beobachtet`};
 const lines=[words.head.join(" | "),...report.rows.map(row=>[row.scenario,row.backend,row.claim,(words as unknown as Record<string,string>)[row.verdict]].join(" | "))];
 for(const row of report.rows)if(row.failures.length)lines.push("",`${row.backend} / ${title(row.scenario)}:`,...row.failures.map(failure=>"  - "+failure));
 return lines.join("\n")+"\n\n"+words.totals+"\n";
}
