import {parityCompareV1,parityTaskKeyV1,type ParityExtractorsV1,type ParityNxInputsV1,type ParityPlanV1,type ParityRunV1,type ParitySpawnV1} from "../🟦️.ts";

/** 🧾️ One Bazel spawn as the execution log records it. */
export type ParityBazelSpawnRowV1=Readonly<{label:string;mnemonic:string;runner:string;cacheHit:boolean;exit:number;env:readonly string[];inputs:readonly string[]}>;
/** 📡️ What a Bazel build event stream says about the run. */
export type ParityBazelEventsV1=Readonly<{exit:number|null;targets:Readonly<Record<string,boolean>>;tests:Readonly<Record<string,string>>}>;

const ansi=/\u001b\[[0-9;]*[A-Za-z]/gu;
const sorted=(values:Iterable<string>):string[]=>[...new Set(values)].sort(parityCompareV1);
const record=(value:unknown):Record<string,unknown>|undefined=>value!==null&&typeof value==="object"&&!Array.isArray(value)?value as Record<string,unknown>:undefined;
const strings=(value:unknown):string[]=>Array.isArray(value)?value.filter((row):row is string=>typeof row==="string"):[];

/** 🔦️ Reads every top-level JSON object that starts a line, skipping log lines before, between and after them. */
export function parseParityJsonObjectsV1(text:string):unknown[]{
 const found:unknown[]=[];
 for(let start=0;start<text.length;start++){
  if(text[start]!=="{"||start>0&&text[start-1]!=="\n")continue;
  let depth=0,inString=false,escaped=false,end=-1;
  for(let index=start;index<text.length;index++){
   const character=text[index]!;
   if(inString){if(escaped)escaped=false;else if(character==="\\")escaped=true;else if(character==="\"")inString=false;continue;}
   if(character==="\"")inString=true;else if(character==="{")depth++;else if(character==="}"&&--depth===0){end=index;break;}
  }
  if(end<0)continue;
  try{found.push(JSON.parse(text.slice(start,end+1)));start=end;}catch{}
 }
 return found;
}

/** 🕸️ Reads the neutral task graph from what `run --dry-run` printed. */
export function parseParityPlanV1(text:string):ParityPlanV1|null{
 for(const object of parseParityJsonObjectsV1(text)){
  const graph=record(record(object)?.tasks);
  if(graph&&Array.isArray(graph.roots)&&record(graph.tasks)&&record(graph.dependencies))return graph as unknown as ParityPlanV1;
 }
 return null;
}

/** 🧮️ Reads the inputs Nx prints for each requested task, in request order. */
export function parseParityNxInputsV1(text:string,ids:readonly string[]):Record<string,ParityNxInputsV1>{
 const objects=parseParityJsonObjectsV1(text).map(record).filter((row):row is Record<string,unknown>=>row!==undefined&&Array.isArray(row.files)),result:Record<string,ParityNxInputsV1>={};
 objects.forEach((row,index)=>{
  const key=objects.length===ids.length?ids[index]!:`${row.project}:${row.target}`;
  result[key]={files:sorted(strings(row.files)),environment:sorted(strings(row.environment)),runtime:sorted(strings(row.runtime)),depOutputs:sorted(strings(row.depOutputs)),external:sorted(strings(row.external))};
 });
 return result;
}

/** 🔭️ Reads the Bazel execution log: a stream of JSON spawn objects, pretty-printed or one per line. */
export function parseParityBazelExecutionLogV1(text:string):ParityBazelSpawnRowV1[]{
 const rows:ParityBazelSpawnRowV1[]=[];
 for(const object of parseParityJsonObjectsV1(text)){
  const row=record(object);
  if(!row||typeof row.targetLabel!=="string")continue;
  const environment=Array.isArray(row.environmentVariables)?row.environmentVariables.map(entry=>record(entry)?.name).filter((name):name is string=>typeof name==="string"):[],inputs=Array.isArray(row.inputs)?row.inputs.map(entry=>record(entry)?.path).filter((path):path is string=>typeof path==="string"):[];
  rows.push({label:row.targetLabel,mnemonic:typeof row.mnemonic==="string"?row.mnemonic:"",runner:typeof row.runner==="string"?row.runner:"",cacheHit:row.cacheHit===true||row.remoteCacheHit===true,exit:typeof row.exitCode==="number"?row.exitCode:0,env:sorted(environment),inputs:sorted(inputs)});
 }
 return rows;
}

/** 🧺️ Groups the spawns of each label under the task the label belongs to. */
export function groupParityBazelSpawnsV1(rows:readonly ParityBazelSpawnRowV1[],labelToTask:Readonly<Record<string,string>>,declaredEnvironment:Readonly<Record<string,readonly string[]>>={}):Record<string,ParitySpawnV1>{
 const grouped=new Map<string,{inputs:Set<string>;env:Set<string>;exit:number}>();
 for(const[label,names]of Object.entries(declaredEnvironment)){
  const task=labelToTask[label];if(!task)continue;
  const entry=grouped.get(task)??{inputs:new Set<string>(),env:new Set<string>(),exit:0};
  for(const name of names)entry.env.add(name);grouped.set(task,entry);
 }
 for(const row of rows){
  const task=labelToTask[row.label];if(!task)continue;
  const entry=grouped.get(task)??{inputs:new Set<string>(),env:new Set<string>(),exit:0};
  for(const path of row.inputs)entry.inputs.add(path);for(const name of row.env)entry.env.add(name);if(row.exit!==0)entry.exit=row.exit;
  grouped.set(task,entry);
 }
 return Object.fromEntries([...grouped].sort(([left],[right])=>parityCompareV1(left,right)).map(([task,entry])=>[task,{inputs:sorted(entry.inputs),env:sorted(entry.env),exit:entry.exit}]));
}

/** 📻️ Reads the build event stream: the build exit code (`SUCCESS` carries no code), which targets completed (a missing `success` is a failure) and each test summary. */
export function parseParityBazelEventsV1(text:string):ParityBazelEventsV1{
 const targets:Record<string,boolean>={},tests:Record<string,string>={};let exit:number|null=null;
 for(const line of text.split("\n")){
  if(!line.trim())continue;
  let event:Record<string,unknown>|undefined;try{event=record(JSON.parse(line));}catch{continue;}
  const id=record(event?.id);if(!event||!id)continue;
  const completed=record(id.targetCompleted),summary=record(id.testSummary);
  if(completed&&typeof completed.label==="string")targets[completed.label]=record(event.completed)?.success===true;
  if(summary&&typeof summary.label==="string"){const status=record(event.testSummary)?.overallStatus;if(typeof status==="string")tests[summary.label]=status;}
  if(id.buildFinished!==undefined){const code=record(record(event.finished)?.exitCode);if(code)exit=typeof code.code==="number"?code.code:code.name==="SUCCESS"?0:exit;}
 }
 return {exit,targets,tests};
}

/** ❌️ Reads the task ids Nx lists after its failed-tasks heading. */
export function parseParityFailedTasksV1(output:string,extractors:ParityExtractorsV1):string[]{
 const heading=new RegExp(extractors.failedTasksHeading,"u"),item=new RegExp(extractors.failedTask,"u"),failed:string[]=[];let listing=false;
 for(const line of output.replace(ansi,"").split("\n")){
  if(heading.test(line)){listing=true;continue;}
  if(!listing)continue;
  const match=item.exec(line);
  if(match)failed.push(match[1]!);else if(line.trim()&&failed.length)listing=false;
 }
 return sorted(failed);
}

/** 🧪️ Names the tasks of a plan that are tests. */
export function parityTestTasksV1(plan:ParityPlanV1,extractors:ParityExtractorsV1):string[]{
 const test=new RegExp(extractors.testTarget,"u");
 return sorted(Object.keys(plan.tasks).filter(id=>test.test(plan.tasks[id]!.target.target)).map(id=>parityTaskKeyV1(plan,id)));
}

/** 🏁️ Builds the Nx run result from the process outcome and its output. */
export function parityNxRunV1(process_:Readonly<{exit:number|null;reason:ParityRunV1["reason"];durationMs?:number}>,output:string,plan:ParityPlanV1|null,extractors:ParityExtractorsV1):ParityRunV1{
 const failedTasks=parseParityFailedTasksV1(output,extractors),tests:Record<string,"passed"|"failed"|"no-status">={};
 if(plan)for(const task of parityTestTasksV1(plan,extractors))tests[task]=failedTasks.includes(task)?"failed":process_.exit===0?"passed":"no-status";
 return {exit:process_.exit,reason:process_.reason,...(process_.durationMs===undefined?{}:{durationMs:process_.durationMs}),failedTasks,tests};
}

/** 🧱️ Builds the Bazel run result from the process outcome and its build event stream. */
export function parityBazelRunV1(process_:Readonly<{exit:number|null;reason:ParityRunV1["reason"];durationMs?:number}>,events:ParityBazelEventsV1,labelToTask:Readonly<Record<string,string>>,plan:ParityPlanV1|null,extractors:ParityExtractorsV1):ParityRunV1{
 const failed=new Set<string>(),tests:Record<string,"passed"|"failed"|"no-status">={},taskKey=(label:string):string=>labelToTask[label]??label;
 for(const[label,success]of Object.entries(events.targets))if(!success)failed.add(taskKey(label));
 for(const[label,status]of Object.entries(events.tests))if(status!=="PASSED"&&status!=="NO_STATUS")failed.add(taskKey(label));
 if(plan)for(const task of parityTestTasksV1(plan,extractors)){
  const label=Object.keys(labelToTask).find(row=>labelToTask[row]===task),status=label===undefined?undefined:events.tests[label],built=label===undefined?undefined:events.targets[label];
  tests[task]=status!==undefined?status==="PASSED"?"passed":status==="NO_STATUS"?"no-status":"failed":built===undefined?"no-status":built?"passed":"failed";
 }
 return {exit:process_.exit,reason:process_.reason,...(process_.durationMs===undefined?{}:{durationMs:process_.durationMs}),failedTasks:[...failed].sort(parityCompareV1),tests};
}
