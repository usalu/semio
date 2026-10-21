import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "@@VALID@@";
import {buildModesV1} from "@@PROC@@/@@BUILDMOD@@/@@MODEMOD@@/@@TS@@";
import targetCacheSchema from "@@UP@@@@EXECUTION@@/@@SCHEMA@@/💾️cache.json" with {type:"json"};
import {resolveBuildTargetV1,type BuildGraphV1,type BuildTargetV1} from "@@UP@@@@GRAPH@@/@@TS@@";

/** 📋️ One planned task: the build target resolved for a project and configuration, with the ids of the tasks it depends on. */
export interface BuildTaskV1{id:string;project:string;target:string;configuration?:string;label:string;dependencies:string[];args:string[];definition:BuildTargetV1;root:string}
/** 🗺️ The planned task graph of one request: every task by id and the ids the request selected. */
export interface BuildTaskPlanV1{version:1;tasks:Record<string,BuildTaskV1>;selected:string[]}
/** 📨️ A backend-neutral request to run tasks: the portable selection `selectBuildTasksV1` returns. */
export interface BuildTaskRequestV1{tasks:string[];args?:string[];configuration?:string;authority?:"prototype"|"release";parallel?:number;excludeDependencies?:boolean;skipCache?:boolean;skipRemoteCache?:boolean;bail?:boolean;verbose?:boolean;outputStyle?:"stream"|"stream-without-prefixes"|"static";workers?:boolean}
/** 🚫️ Why a task stays out of every cache. */
export type BuildUncachedReasonV1="continuous"|"declared"|"skipped"|"undeclared"|"unbounded"|"driver-owned"|"managed-output"|"side-effecting"|"live"|"aggregate"|"diagnostic"|"unclassified"|"unsupported-input";
/** ⚖️ Whether a task replays from a cache and, if not, why. */
export interface BuildCacheDecisionV1{cache:boolean;reason?:BuildUncachedReasonV1}

const hash=(value:string)=>createHash("sha256").update(value).digest("hex");
const modeAxis=(name:string|undefined):name is "prototype"|"release"=>name!==undefined&&(buildModesV1() as readonly string[]).includes(name);
const modeOnly=(target:BuildTargetV1)=>target.configurations===undefined||Object.keys(target.configurations).length===0;

/** 🎯️ Resolves a target for one configuration; a compiler mode of a target without configurations only exports the mode to the task environment. */
export function buildTargetForConfigurationV1(target:BuildTargetV1,configuration?:string):BuildTargetV1{
 if(modeAxis(configuration)&&modeOnly(target))return {...target,execution:{...target.execution,env:{...target.execution.env,SEMIO_BUILD_MODE:configuration}}};
 return resolveBuildTargetV1(target,configuration);
}

function identity(graph:BuildGraphV1,value:string,configuration?:string):{id:string;project:string;target:string;configuration?:string}{
 const parts=value.split(":"),project=parts[0]!,target=parts[1]!,definition=graph.projects[project]?.targets[target];
 if(parts.length<2||parts.length>3||!definition)throw Error(`Unknown build task: ${value}`);
 const admitted=(name:string|undefined):name is string=>name!==undefined&&(modeOnly(definition)&&modeAxis(name)||definition.configurations!==undefined&&Object.hasOwn(definition.configurations,name));
 const requested=parts[2]??configuration,selected=admitted(requested)?requested:definition.defaultConfiguration as string|undefined,chosen=admitted(selected)?selected:undefined;
 if(parts[2]&&!admitted(parts[2]))throw Error(`Unknown build configuration: ${value}\nUnbekannte Build-Konfiguration: ${value}`);
 return {project,target,...(chosen?{configuration:chosen}:{}),id:`${project}:${target}${chosen?":"+chosen:""}`};
}

/** 🕸️ Plans the dependency closure of a request over the portable graph: every task, its resolved target and its dependency ids. */
export function planBuildTasksV1(graph:BuildGraphV1,request:BuildTaskRequestV1):BuildTaskPlanV1{
 const tasks:Record<string,BuildTaskV1>={},visiting=new Set<string>(),selected:string[]=[],explicit=new Set(request.tasks.map(value=>identity(graph,value,request.configuration??request.authority).id)),projectDependencies=new Map<string,string[]>(),matchedTargets=new Map<string,string[]>(),targetNames=[...new Set(Object.values(graph.projects).flatMap(project=>Object.keys(project.targets)))];
 for(const edge of graph.dependencies)if(graph.projects[edge.target]){const row=projectDependencies.get(edge.source)??[];row.push(edge.target);projectDependencies.set(edge.source,row);}
 const include=(value:string,args:string[],configuration?:string):string=>{
  const found=identity(graph,value,configuration),{id,project,target}=found,inherited=value.split(":")[2]??configuration??found.configuration;
  if(visiting.has(id))throw Error(`Build task dependency cycle: ${[...visiting,id].join(" -> ")}`);
  if(tasks[id])return id;
  const fixed=(graph.projects[project]!.targets[target]!.metadata as {semio?:{buildMode?:{fixed?:string}}}|undefined)?.semio?.buildMode?.fixed;
  if(modeAxis(found.configuration)){
   if(request.authority!==undefined&&found.configuration!==request.authority)throw Error(`Conflicting build modes: task ${id} selects ${found.configuration}, the original caller selects ${request.authority}\nWidersprüchliche Build-Modi: Task ${id} wählt ${found.configuration}, der ursprüngliche Aufrufer wählt ${request.authority}`);
   if(fixed!==undefined&&fixed!==found.configuration)throw Error(`Conflicting build modes: task ${id} is fixed to ${fixed} but the invocation selects ${found.configuration}; select ${fixed} or another target\nWidersprüchliche Build-Modi: Task ${id} ist fest auf ${fixed}, die Auswahl ist aber ${found.configuration}; wähle ${fixed} oder ein anderes Target`);
  }
  if(explicit.has(id))args=request.args??[];
  visiting.add(id);
  const owner=graph.projects[project]!,definition=buildTargetForConfigurationV1(owner.targets[target]!,found.configuration),dependencies=new Set<string>();
  for(const dependency of definition.dependsOn??[]){
   const specification=typeof dependency==="string"?{target:dependency}:dependency,pattern=specification.target,fromDependencies=pattern.startsWith("^")||specification.projects==="dependencies"||specification.dependencies===true,operation=pattern.replace(/^\^/u,""),forwarded=specification.params==="forward"?args:[];
   const add=(name:string)=>{
    const[dependencyProject,dependencyTarget]=name.split(":");
    if(!graph.projects[dependencyProject!])throw Error(`Unknown build task dependency: ${id} depends on ${name}, but project ${dependencyProject} is not in the build graph`);
    if(!graph.projects[dependencyProject!]!.targets[dependencyTarget??""])throw Error(`Unknown build task dependency: ${id} depends on ${name}, but project ${dependencyProject} defines no target ${dependencyTarget}`);
    const resolved=identity(graph,name,inherited).id;
    if(resolved!==id&&(!request.excludeDependencies||explicit.has(resolved)))dependencies.add(include(name,forwarded,inherited));
   };
   if(operation.includes(":")){add(operation);continue;}
   const projects=specification.projects===undefined||specification.projects==="self"?[project]:typeof specification.projects==="string"?[specification.projects]:specification.projects;
   let operations=matchedTargets.get(operation);
   if(!operations){const matcher=new Bun.Glob(operation);operations=targetNames.filter(name=>matcher.match(name));matchedTargets.set(operation,operations);}
   for(const name of operations){
    if(fromDependencies){const seen=new Set<string>(),pending=[...projectDependencies.get(project)??[]];while(pending.length){const candidate=pending.pop()!;if(seen.has(candidate))continue;seen.add(candidate);if(graph.projects[candidate]!.targets[name])add(`${candidate}:${name}`);else pending.push(...projectDependencies.get(candidate)??[]);}}
    else for(const candidate of Object.keys(graph.projects).filter(candidate=>projects.some(item=>!item.startsWith("!")&&new Bun.Glob(item).match(candidate))&&!projects.some(item=>item.startsWith("!")&&new Bun.Glob(item.slice(1)).match(candidate))))if(graph.projects[candidate]!.targets[name])add(`${candidate}:${name}`);
   }
  }
  tasks[id]={...found,label:`t_${hash(id).slice(0,16)}`,dependencies:[...dependencies].sort(),args:[...args],definition,root:owner.root};
  visiting.delete(id);
  return id;
 };
 for(const value of request.tasks)selected.push(include(value,request.args??[],request.configuration??request.authority));
 if(!selected.length)throw Error("At least one selected build task is required");
 return {version:1,tasks,selected:[...new Set(selected)]};
}

/** 📥️ One expanded declared input of a task. */
export type BuildTaskInputV1=Readonly<{kind:"file";pattern:string;exclude:boolean}>|Readonly<{kind:"environment";name:string}>|Readonly<{kind:"runtime";command:string}>|Readonly<{kind:"external";names:readonly string[]}>|Readonly<{kind:"dependency-outputs"}>|Readonly<{kind:"unsupported";description:string}>;

const POLICY=Object.freeze({maximumDepth:128,maximumEntries:1000000});

/** 🧾️ Expands the declared inputs of a task over the portable graph: named groups, `^` dependency groups, project references, environment variables, runtime commands, external dependencies, exclusions and dependency outputs. Anything it cannot interpret becomes `unsupported`, so a cache never trusts an input it did not understand. */
export function buildTaskInputsV1(graph:BuildGraphV1,task:Pick<BuildTaskV1,"project"|"definition">):BuildTaskInputV1[]{
 const active=new Set<string>(),adjacency=new Map<string,string[]>(),closures=new Map<string,string[]>();let entries=0;
 const groups=(project:string):Record<string,unknown[]>=>{if(!graph.projects[project])throw Error("Unknown build input project: "+project);return{default:["{projectRoot}/**/*"],...graph.namedInputs,...graph.projects[project]!.namedInputs as Record<string,unknown[]>|undefined};};
 for(const edge of graph.dependencies)if(graph.projects[edge.target]){const row=adjacency.get(edge.source)??[];if(!row.includes(edge.target))row.push(edge.target);adjacency.set(edge.source,row);}
 const dependencies=(project:string):string[]=>{let known=closures.get(project);if(!known){known=[];const seen=new Set([project]),pending=[project];for(let at=0;at<pending.length;at++)for(const target of adjacency.get(pending[at]!)??[])if(!seen.has(target)){seen.add(target);known.push(target);pending.push(target);}closures.set(project,known);}return known;};
 const named=(name:string,project:string,depth:number):BuildTaskInputV1[]=>{const values=groups(project)[name],key=project+":"+name;if(!Array.isArray(values))throw Error("Unknown build input group: "+key);if(active.has(key))throw Error("Build input group cycle: "+key);active.add(key);try{return expand(values,project,depth+1);}finally{active.delete(key);}};
 const file=(pattern:string,project:string):BuildTaskInputV1=>{
  const exclude=pattern.startsWith("!"),text=exclude?pattern.slice(1):pattern,root=graph.projects[project]!.root==="."?"":graph.projects[project]!.root;
  return {kind:"file",exclude,pattern:text.replaceAll("{projectRoot}",root).replaceAll("{workspaceRoot}/","").replaceAll("{workspaceRoot}","").replace(/^\.\//u,"").replace(/^\/+/u,"").replace(/^\/+/u,"")};
 };
 const expand=(inputs:readonly unknown[],project:string,depth=0):BuildTaskInputV1[]=>{
  if(depth>POLICY.maximumDepth)throw Error("Build input expansion bound exceeded");
  const declared=groups(project);
  return inputs.flatMap((input):BuildTaskInputV1[]=>{
   if(++entries>POLICY.maximumEntries)throw Error("Build input expansion bound exceeded");
   if(typeof input==="string"){
    if(input.startsWith("^")){const name=input.slice(1);return dependencies(project).flatMap(owner=>name.startsWith("{projectRoot}")||name.startsWith("{workspaceRoot}")?expand([name],owner,depth+1):named(name,owner,depth));}
    if(Object.hasOwn(declared,input))return named(input,project,depth);
    return [file(input,project)];
   }
   if(input&&typeof input==="object"){
    const row=input as Record<string,unknown>;
    if(typeof row.env==="string")return [{kind:"environment",name:row.env}];
    if(typeof row.fileset==="string")return [file(row.fileset,project)];
    if(Array.isArray(row.externalDependencies))return [{kind:"external",names:(row.externalDependencies as unknown[]).filter((name):name is string=>typeof name==="string").sort()}];
    if(typeof row.runtime==="string")return [{kind:"runtime",command:row.runtime}];
    if("dependentTasksOutputFiles" in row)return [{kind:"dependency-outputs"}];
    if(typeof row.input==="string"){const owners=row.dependencies===true?dependencies(project):row.projects===undefined?[project]:typeof row.projects==="string"?[row.projects]:row.projects as string[];return owners.flatMap(owner=>named(row.input as string,owner,depth));}
    return [{kind:"unsupported",description:Object.keys(row).sort().join(",")}];
   }
   return [{kind:"unsupported",description:typeof input}];
  });
 };
 return expand(task.definition.inputs??["{projectRoot}/**/*"],task.project);
}

/** 🏷️ Decides whether a task replays from a cache and records why not; it fails closed: only a deterministic class or an explicit `cache: true` with declared outputs replays, and a pure ordering task always does. */
export function buildCacheDecisionV1(task:Pick<BuildTaskV1,"id"|"definition">,skipCache:boolean):BuildCacheDecisionV1{
 const row=(task.definition as {metadata?:{semio?:{cache?:unknown}}}).metadata?.semio?.cache;
 if(row!==undefined&&validateJsonSchemaSubset(targetCacheSchema,row,targetCacheSchema).length)throw Error(`Closed repository cache decision required: ${task.id}`);
 const klass=row===undefined?undefined:(row as {class:string}).class,forbidden=klass!==undefined&&klass!=="deterministic",noop=task.definition.execution.kind==="noop";
 if(forbidden&&!noop&&task.definition.cache===true)throw Error(`Conflicting cache declaration: ${task.id} declares cache with class ${klass}`);
 return task.definition.continuous?{cache:false,reason:"continuous"}:skipCache?{cache:false,reason:"skipped"}:noop?{cache:true}:forbidden?{cache:false,reason:klass as BuildUncachedReasonV1}:task.definition.cache===false?{cache:false,reason:"declared"}:task.definition.cache!==true&&klass!=="deterministic"?{cache:false,reason:"undeclared"}:!Array.isArray(task.definition.outputs)?{cache:false,reason:"unbounded"}:{cache:true};
}

/** 🪜️ Projects a plan into the neutral dry-run graph: roots have no dependencies, and dependencies on continuous tasks are kept apart because they only need to be running. */
export function buildDryRunPlanV1(plan:BuildTaskPlanV1):Readonly<{tasks:BuildDryRunGraphV1;selected:readonly string[]}>{
 const ids=Object.keys(plan.tasks),continuous=(id:string)=>plan.tasks[id]!.definition.continuous===true,dependencies:Record<string,string[]>={},waiting:Record<string,string[]>={};
 for(const id of ids){
  const owned=plan.tasks[id]!.dependencies.filter(dependency=>dependency!==id);
  dependencies[id]=owned.filter(dependency=>!continuous(dependency));waiting[id]=owned.filter(continuous);
 }
 const tasks:BuildDryRunGraphV1={roots:ids.filter(id=>!dependencies[id]!.length&&!waiting[id]!.length),tasks:Object.fromEntries(ids.map(id=>{const task=plan.tasks[id]!;return [id,{id,target:{project:task.project,target:task.target,...(task.configuration?{configuration:task.configuration}:{})}}];})),dependencies,continuousDependencies:waiting};
 return Object.freeze({tasks,selected:plan.selected});
}
/** 🌐️ The neutral task graph of a dry run. */
export type BuildDryRunGraphV1=Readonly<{roots:readonly string[];tasks:Readonly<Record<string,Readonly<{id:string;target:Readonly<{project:string;target:string;configuration?:string}>}>>>;dependencies:Readonly<Record<string,readonly string[]>>;continuousDependencies?:Readonly<Record<string,readonly string[]>>}>;
