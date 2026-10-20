import {constants} from "node:fs";
import {copyFile,lstat,mkdir,readFile,readlink,readdir,realpath,rm,symlink,writeFile} from "node:fs/promises";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {incrementalBazelObservationV1,incrementalBazelSpawnsV1,incrementalNxObservationV1} from "../👁️observers/🟦️.ts";
import {incrementalExpectedV1,incrementalMutateV1,incrementalQuiescentV1,incrementalSliceV1,incrementalVerdictV1,type IncrementalExpectedV1,type IncrementalFamilyV1,type IncrementalObservationV1,type IncrementalPhaseV1,type IncrementalRowV1,type IncrementalScenarioSetV1,type IncrementalScenarioV1,type IncrementalSliceV1,type IncrementalSliceTaskV1,type IncrementalToolV1} from "../🟦️.ts";

const policy=schema.definitions.Policy.const,order=(left:string,right:string)=>left<right?-1:left>right?1:0;

/** 🔌️ Everything a scenario run needs from the outside: the disposable copy, the closure of a target, one execution with its observation, the clock and cooperative cancellation. */
export type IncrementalPortsV1=Readonly<{
 read(path:string):Promise<string>;
 write(path:string,text:string):Promise<void>;
 slice(scenario:IncrementalScenarioV1,tool:IncrementalToolV1):Promise<IncrementalSliceV1>;
 execute(tool:IncrementalToolV1,target:string,slice:IncrementalSliceV1):Promise<IncrementalObservationV1>;
 now():number;
 nonce():string;
 check():void;
 progress(stage:string):Promise<void>;
}>;

const row=(scenario:string,tool:IncrementalToolV1,phase:IncrementalPhaseV1,expected:IncrementalExpectedV1,observation:IncrementalObservationV1,milliseconds:number,note?:string):IncrementalRowV1=>
 Object.freeze({scenario,tool,phase,expected,observation,verdict:incrementalVerdictV1(expected,observation.executed),milliseconds,...(note?{note}:{})});

const empty=(tool:IncrementalToolV1):IncrementalObservationV1=>Object.freeze({tool,executed:[],restored:[],cached:[],unmapped:[]});
const nothing:IncrementalExpectedV1=Object.freeze({direct:[],must:[],may:[],cutoff:[]});
const outcome=(scenario:string,tool:IncrementalToolV1,phase:IncrementalPhaseV1,status:"failed"|"skipped",note:string):IncrementalRowV1=>
 Object.freeze({scenario,tool,phase,expected:nothing,observation:empty(tool),verdict:Object.freeze({status,stale:[],excess:[],exact:false}),milliseconds:0,note});

/** 🎬️ Runs one scenario for one tool in the disposable copy: prime the cache, re-run warm, apply the edit, run, then revert the edit and run again. The original text is restored whatever happens, and a cancellation stops after the restore. */
export async function incrementalRunScenarioV1(set:IncrementalScenarioSetV1,scenario:IncrementalScenarioV1,tool:IncrementalToolV1,phases:readonly IncrementalPhaseV1[],ports:IncrementalPortsV1):Promise<IncrementalRowV1[]>{
 if(scenario.tools&&!scenario.tools.includes(tool))return phases.map(phase=>outcome(scenario.id,tool,phase,"skipped",`${tool} is not part of ${scenario.id}`));
 const rows:IncrementalRowV1[]=[],sensitivity=[...(scenario.effects??[]),...set.sensitivity],file=scenario.mutation.file;
 ports.check();await ports.progress(`${scenario.id}:${tool}:slice`);
 let slice:IncrementalSliceV1;
 try{slice=incrementalSliceV1(await ports.slice(scenario,tool));}catch(error){ports.check();return phases.map(phase=>outcome(scenario.id,tool,phase,"failed",`slice: ${(error as Error).message}`));}
 const run=async(phase:IncrementalPhaseV1,expected:IncrementalExpectedV1,note?:string):Promise<void>=>{
  ports.check();await ports.progress(`${scenario.id}:${tool}:${phase}`);
  const started=ports.now();
  try{const observation=await ports.execute(tool,scenario.target,slice);rows.push(row(scenario.id,tool,phase,expected,observation,Math.max(0,Math.round(ports.now()-started)),note));}
  catch(error){ports.check();rows.push(outcome(scenario.id,tool,phase,"failed",(error as Error).message));}
 };
  ports.check();await ports.progress(`${scenario.id}:${tool}:prime`);
  try{await ports.execute(tool,scenario.target,slice);}catch(error){ports.check();return phases.map(phase=>outcome(scenario.id,tool,phase,"failed",`prime: ${(error as Error).message}`));}
  if(phases.includes("warm"))await run("warm",incrementalQuiescentV1(slice));
  if(phases.includes("edit")){
   const original=await ports.read(file);
   try{
    await ports.write(file,incrementalMutateV1(original,scenario.mutation.op,scenario.mutation.language,ports.nonce()));
    await run("edit",incrementalExpectedV1(slice,tool,{kind:scenario.kind,files:[file]},sensitivity));
   }finally{await ports.write(file,original);}
   if(phases.includes("revert"))await run("revert",incrementalQuiescentV1(slice));
  }
 return rows;
}

/** 🏁️ Runs every selected scenario with every selected tool in order and returns all judged rows. */
export async function incrementalRunV1(set:IncrementalScenarioSetV1,scenarios:readonly IncrementalScenarioV1[],tools:readonly IncrementalToolV1[],phases:readonly IncrementalPhaseV1[],ports:IncrementalPortsV1):Promise<IncrementalRowV1[]>{
 const rows:IncrementalRowV1[]=[];
 for(const scenario of scenarios)for(const tool of tools)rows.push(...await incrementalRunScenarioV1(set,scenario,tool,phases,ports));
 return rows;
}

/** 🧹️ Builds the environment of a child CLI: the parent's environment without any inherited invocation, capability, build or Nx state, plus the explicit additions. */
export function incrementalChildEnvironmentV1(parent:Readonly<Record<string,string|undefined>>,extra:Readonly<Record<string,string>>):Record<string,string>{
 const kept:Record<string,string>={};
 for(const[key,value]of Object.entries(parent))if(value!==undefined&&(key==="SEMIO_LANGUAGE"||!/^(SEMIO_|NX_)/u.test(key)))kept[key]=value;
 return {...kept,...extra};
}

/** 🧭️ The command line of one `repo` invocation inside the disposable root: its own copy of the build CLI, never the live one. */
export function incrementalCliArgumentsV1(executable:string,root:string,script:string,tool:IncrementalToolV1,args:readonly string[]):{command:string;args:string[]}{
 return {command:executable,args:[join(root,script),"repo",`--tool=${tool}`,...args]};
}

/** 🚧️ Refuses a disposable root that is, contains or lies inside the live checkout, so no edit can ever reach the live tree. */
export async function incrementalDisposableV1(live:string,disposable:string):Promise<void>{
 const real=async(path:string):Promise<string>=>{let at=resolve(path);const tail:string[]=[];for(;;){try{return join(await realpath(at),...tail.reverse());}catch{const parent=dirname(at);if(parent===at)return resolve(path);tail.push(at.slice(parent.length+1));at=parent;}}};
 const a=await real(live),b=await real(disposable),inside=(inner:string,outer:string)=>inner===outer||inner.startsWith(outer.endsWith(sep)?outer:outer+sep);
 if(inside(b,a)||inside(a,b))throw Error(`The disposable root must be separate from the live checkout: ${disposable}\nDas Wegwerf-Verzeichnis muss vom lebenden Checkout getrennt sein: ${disposable}`);
}

/** 📏️ Resolves a repository-relative file inside the disposable root and refuses anything that leaves it. */
export function incrementalInsideV1(root:string,path:string):string{
 if(isAbsolute(path)||path.split(/[\\/]/u).includes(".."))throw Error(`A repository-relative path inside the disposable root is required: ${path}`);
 const resolved=resolve(root,path),inner=relative(resolve(root),resolved);
 if(inner===""||inner.startsWith("..")||isAbsolute(inner))throw Error(`A repository-relative path inside the disposable root is required: ${path}`);
 if(resolved.length>policy.maximumPathLength)throw Error(`Path exceeds ${policy.maximumPathLength} characters: ${resolved}`);
 return resolved;
}

/** 🪞️ Copies tracked files into an empty disposable root with copy-on-write where the filesystem offers it, keeping symbolic links as links; progress every thousand files, cancellation between batches. */
export async function incrementalCloneV1(source:string,destination:string,files:readonly string[],hooks:Readonly<{signal:AbortSignal;progress(done:number,total:number):Promise<void>}>):Promise<number>{
 await incrementalDisposableV1(source,destination);
 try{if((await readdir(destination)).length)throw Error(`The disposable root must be empty: ${destination}\nDas Wegwerf-Verzeichnis muss leer sein: ${destination}`);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
 for(const file of files)incrementalInsideV1(destination,file);
 const made=new Set<string>(),ensure=async(directory:string)=>{if(!made.has(directory)){await mkdir(directory,{recursive:true});made.add(directory);}};
 await ensure(destination);
 let done=0;
 for(let start=0;start<files.length;start+=policy.copyConcurrency){
  if(hooks.signal.aborted)throw Error("Cancelled\nAbgebrochen");
  await Promise.all(files.slice(start,start+policy.copyConcurrency).map(async file=>{
   const from=join(source,file),to=join(destination,file),meta=await lstat(from);
   await ensure(dirname(to));
   if(meta.isSymbolicLink())await symlink(await readlink(from),to);
   else if(meta.isFile())await copyFile(from,to,constants.COPYFILE_FICLONE);
  }));
  const before=done;done=Math.min(files.length,start+policy.copyConcurrency);
  if(Math.floor(done/1000)!==Math.floor(before/1000))await hooks.progress(done,files.length);
 }
 await hooks.progress(files.length,files.length);
 return files.length;
}

/** 🗑️ Removes a disposable root after a run unless it is kept; it refuses the live checkout. */
export async function incrementalDiscardV1(live:string,disposable:string):Promise<void>{await incrementalDisposableV1(live,disposable);await rm(disposable,{recursive:true,force:true});}

type AdapterTaskV1=Readonly<{id:string;dependencies:readonly string[];definition:Readonly<{inputs?:readonly unknown[];outputs?:unknown}>}>;
/** 🧩️ The part of the Bazel adapter the slice builder reads: the planner, the cache decision, the coverage classifier, the source index and the input expansion. */
export type IncrementalAdapterV1=Readonly<{
 planBazelTasksV1(graph:never,request:never):{selected:readonly string[];tasks:Readonly<Record<string,AdapterTaskV1>>};
 bazelCacheDecisionV1(task:never,skipCache:boolean):{cache:boolean};
 bazelTaskCoverageV1(task:never,decision:never,script?:string):{family:string};
 bazelSourceIndexV1(inventory:readonly string[]):{files:readonly string[];selectAll(inputs:readonly unknown[]):{marks:Uint8Array}};
 bazelTaskInputsV1(graph:never,task:never,closures:"ignore"):unknown[];
}>;

const families=new Set<string>(schema.definitions.Family.enum);

/** 🔪️ Builds the slice of a target from the neutral graph with the adapter's own planner and input expansion, keeping only the declared inputs that name the files an edit touches. Tasks that consume dependency outputs through `dependentTasksOutputFiles` are recorded, transitively when the input says so. */
export function incrementalSliceFromGraphV1(adapter:IncrementalAdapterV1,graph:unknown,target:string,relevant:readonly string[],inventory:readonly string[]):IncrementalSliceV1{
 const plan=adapter.planBazelTasksV1(graph as never,{tasks:[target],args:[]} as never),index=adapter.bazelSourceIndexV1(inventory),position=new Map(index.files.map((file,at)=>[file,at])),wanted=relevant.filter(file=>position.has(file));
 const reach=(id:string,transitive:boolean):string[]=>{const seen=new Set<string>(),pending=[...plan.tasks[id]!.dependencies];while(pending.length){const next=pending.pop()!;if(seen.has(next)||!plan.tasks[next])continue;seen.add(next);if(transitive)pending.push(...plan.tasks[next]!.dependencies);}return [...seen].sort(order);};
 const tasks:Record<string,IncrementalSliceTaskV1>={};
 for(const id of Object.keys(plan.tasks).sort(order)){
  const task=plan.tasks[id]!,decision=adapter.bazelCacheDecisionV1(task as never,false),family=adapter.bazelTaskCoverageV1(task as never,decision as never).family;
  if(!families.has(family))throw Error(`Unknown task family ${family} for ${id}`);
  const marks=decision.cache?index.selectAll(adapter.bazelTaskInputsV1(graph as never,task as never,"ignore")).marks:new Uint8Array(),declared=task.definition.inputs??[];
  const consumers=declared.filter((value):value is {dependentTasksOutputFiles:unknown;transitive?:boolean}=>typeof value==="object"&&value!==null&&"dependentTasksOutputFiles" in value);
  const consumes=consumers.length?reach(id,consumers.some(value=>value.transitive===true)):[];
  tasks[id]={family:family as IncrementalFamilyV1,cache:decision.cache,outputs:Array.isArray(task.definition.outputs)&&task.definition.outputs.length>0,dependsOn:[...task.dependencies].filter(other=>plan.tasks[other]).sort(order),consumes,inputs:wanted.filter(file=>marks[position.get(file)!]===1)};
 }
 return incrementalSliceV1({version:1,selected:[...plan.selected].sort(order),tasks});
}

/** 📃️ Reads a dry-run plan printed by `repo run --dry-run` into the sorted task ids and their dependency map. */
export function incrementalPlanTasksV1(output:string):{ids:string[];dependencies:Record<string,string[]>}{
 const lines=output.split("\n"),start=lines.findIndex(line=>line==="{");
 if(start<0)throw Error("A dry-run plan is required");
 const plan=JSON.parse(lines.slice(start).join("\n")) as {tasks?:{tasks?:Record<string,unknown>;dependencies?:Record<string,string[]>}};
 if(!plan.tasks?.tasks||!plan.tasks.dependencies)throw Error("A dry-run plan is required");
 return {ids:Object.keys(plan.tasks.tasks).sort(order),dependencies:plan.tasks.dependencies};
}

/** 🏠️ Everything the real ports need that differs between hosts: where the copy lives, how to run the CLI, where Nx records its run. */
export type IncrementalHostV1=Readonly<{
 live:string;
 root:string;
 script:string;
 executable:string;
 scratch:string;
 signal:AbortSignal;
 environment:Readonly<Record<string,string|undefined>>;
 run(command:string,args:readonly string[],cwd:string,env:Readonly<Record<string,string>>,signal:AbortSignal):Promise<string>;
 nxRecord(root:string):string;
 adapter():Promise<IncrementalAdapterV1>;
 inventory():Promise<readonly string[]>;
 now():number;
 check():void;
 progress(stage:string):Promise<void>;
}>;

/** 🏭️ The real ports: reads and writes inside the disposable root, slices from the CLI's own graph and plan, executions through the disposable root's own CLI observed through Bazel's compact execution log or Nx's run record. */
export function incrementalCliPortsV1(host:IncrementalHostV1):IncrementalPortsV1{
 let sequence=0,graph:unknown,graphPath:string|undefined;
 const cli=(tool:IncrementalToolV1,args:readonly string[],extra:Readonly<Record<string,string>>={})=>{const command=incrementalCliArgumentsV1(host.executable,host.root,host.script,tool,args);return host.run(command.command,command.args,host.root,incrementalChildEnvironmentV1(host.environment,extra),host.signal);};
 const lastLine=(output:string)=>output.trim().split("\n").at(-1)!.trim();
 return Object.freeze({
  read:async path=>readFile(incrementalInsideV1(host.root,path),"utf8"),
  write:async(path,text)=>{await writeFile(incrementalInsideV1(host.root,path),text);},
  now:host.now,
  nonce:()=>host.now().toString(36).slice(-8).padStart(8,"0")+(++sequence).toString(36),
  check:host.check,
  progress:host.progress,
  slice:async(scenario,tool)=>{
   if(graph===undefined){const output=await cli(tool,["graph",`--file=${join(host.scratch,"graph.json")}`]);graphPath=lastLine(output);graph=JSON.parse(await readFile(graphPath,"utf8"));}
   const planned=incrementalPlanTasksV1(await cli(tool,["run",scenario.target,"--dry-run"])),slice=incrementalSliceFromGraphV1(await host.adapter(),graph,scenario.target,[scenario.mutation.file],await host.inventory());
   const missing=planned.ids.filter(id=>!slice.tasks[id]);
   if(missing.length)throw Error(`The graph planner and ${tool} disagree about the closure of ${scenario.target}: ${missing.join(", ")}`);
   return slice;
  },
  execute:async(tool,target,slice)=>{
   const ids=Object.keys(slice.tasks);
   if(tool==="bazel"){
    const report=join(host.scratch,`bazel-${++sequence}.zst`);
    await rm(report,{force:true});
    await cli("bazel",["run",target],{[policy.bazelReport]:report});
    const observation=incrementalBazelObservationV1(incrementalBazelSpawnsV1(await readFile(report)),ids);
    await rm(report,{force:true});
    return observation;
   }
   const record=host.nxRecord(host.root);
   await rm(record,{force:true});
   await cli("nx",["run",target]);
   return incrementalNxObservationV1(JSON.parse(await readFile(record,"utf8")),ids);
  }
 });
}

/** 🔖️ Selects scenarios by id; `all` selects every scenario, an unknown id is refused. */
export function incrementalSelectV1(set:IncrementalScenarioSetV1,ids:readonly string[]|"all"):IncrementalScenarioV1[]{
 if(ids==="all")return [...set.scenarios];
 const unknown=ids.filter(id=>!set.scenarios.some(row=>row.id===id));
 if(unknown.length)throw Error(`Unknown scenario: ${unknown.join(", ")} (${set.scenarios.map(row=>row.id).join("|")})`);
 return ids.map(id=>set.scenarios.find(row=>row.id===id)!);
}
