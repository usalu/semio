import {existsSync,mkdirSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {conformanceSpawnV1,type ConformanceDriverV1,type ConformancePrepareV1,type ConformanceSessionV1} from "../../../📐️conformance/🏃️runner/🟦️.ts";
import {incrementalNxObservationV1} from "../../../📏️incremental/👁️observers/🟦️.ts";

/** 🔌️ Binds Nx to the conformance suite by running a real Nx in a generated workspace: every stub task becomes a `nx:run-commands` target, and the answers come from Nx's own commands and its run record. `SEMIO_TEST_NX_BIN` names the `nx.js` to run, `SEMIO_TEST_NODE` the Node.js executable. */
export const driver:ConformanceDriverV1=Object.freeze({
 id:"nx",
 async prepare(input:ConformancePrepareV1):Promise<ConformanceSessionV1>{
  const bin=process.env.SEMIO_TEST_NX_BIN,node=process.env.SEMIO_TEST_NODE??"node";
  if(!bin||!existsSync(bin))throw Error("SEMIO_TEST_NX_BIN must name the nx.js of an installed Nx");
  const modules=bin.slice(0,bin.lastIndexOf("node_modules")+"node_modules".length),cache=join(input.cache,"nx"),data=join(input.cache,"nx-data");
  const write=(path:string,value:unknown)=>{mkdirSync(dirname(path),{recursive:true});writeFileSync(path,JSON.stringify(value,null,1));};
  write(join(input.directory,"package.json"),{name:"conformance",private:true});
  write(join(input.directory,"nx.json"),{analytics:false});
  if(!existsSync(join(input.directory,"node_modules")))symlinkSync(modules,join(input.directory,"node_modules"),"dir");
  const command=(id:string)=>`"${input.runtime}" "${input.stub}" "${input.specPath}" ${id}`,ids:string[]=[];
  for(const project of input.workspace.projects){
   const targets:Record<string,unknown>={};
   for(const[target,row]of Object.entries(project.targets)){
    const id=`${project.name}:${target}`;ids.push(id);
    targets[target]={executor:"nx:run-commands",options:{command:command(id),cwd:"{workspaceRoot}"},cache:row.cache,inputs:[...row.reads.map(pattern=>`{workspaceRoot}/${pattern}`),...((row.dependsOn??[]).length?[{dependentTasksOutputFiles:"**/*"}]:[])],outputs:row.outputs.map(path=>`{workspaceRoot}/${path}`),...((row.dependsOn??[]).length?{dependsOn:row.dependsOn!.map(dependency=>({target:dependency.split(":")[1],projects:[dependency.split(":")[0]]}))}:{})};
   }
   write(join(input.directory,project.name,"project.json"),{name:project.name,$schema:"../node_modules/nx/schemas/project-schema.json",sourceRoot:`${project.name}/src`,projectType:"library",targets,implicitDependencies:[...project.dependsOn]});
  }
  const environment=(remote:boolean)=>{
   const base:NodeJS.ProcessEnv={...process.env,NX_DAEMON:"false",NX_NO_CLOUD:"true",NX_TUI:"false",NX_CACHE_DIRECTORY:cache,NX_WORKSPACE_DATA_DIRECTORY:data,NX_ISOLATE_PLUGINS:"false"};
   delete base.NX_SELF_HOSTED_REMOTE_CACHE_SERVER;delete base.NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN;
   if(remote&&input.remote){base.NX_SELF_HOSTED_REMOTE_CACHE_SERVER=input.remote.url;base.NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN="conformance";}
   return base;
  };
  const group=(targets:readonly string[]):{target:string;projects:string[]}=>{
   const names=new Set(targets.map(id=>id.split(":")[1]!));
   if(names.size!==1)throw Error("The Nx binding runs one target name per invocation");
   return {target:[...names][0]!,projects:targets.map(id=>id.split(":")[0]!)};
  };
  return {
   observes:new Set(["run-record"]),
   async plan(target,signal){
    const{target:name,projects}=group([target]),file=join(input.cache,"plan.json");
    rmSync(file,{force:true});
    const result=await conformanceSpawnV1(node,[bin,"run-many","-t",name,"-p",projects.join(","),`--graph=${file}`],{cwd:input.directory,env:environment(false),signal});
    if(result.exitCode!==0||!existsSync(file))throw Error("nx could not print the plan: "+result.output.slice(-400));
    const plan=JSON.parse(readFileSync(file,"utf8")) as {tasks:{tasks:Record<string,unknown>;dependencies:Record<string,string[]>}};
    return {tasks:Object.keys(plan.tasks.tasks),dependencies:plan.tasks.dependencies};
   },
   async run(targets,options,signal){
    const{target,projects}=group(targets),started=Date.now(),record=join(cache,"run.json");
    rmSync(record,{force:true});
    const result=await conformanceSpawnV1(node,[bin,"run-many","-t",target,"-p",projects.join(","),...(options.parallel?[`--parallel=${options.parallel}`]:[]),...(options.skipCache?["--skip-nx-cache"]:[]),"--outputStyle=static"],{cwd:input.directory,env:environment(options.remote==="read-write"),signal});
    let restored:string[]|undefined;
    try{restored=[...incrementalNxObservationV1(JSON.parse(readFileSync(record,"utf8")),ids).restored];}catch{restored=undefined;}
    return {exitCode:result.exitCode,milliseconds:Date.now()-started,output:result.output,...(restored?{restored}:{})};
   },
   async affected(files,signal){
    const result=await conformanceSpawnV1(node,[bin,"show","projects","--affected",`--files=${files.join(",")}`],{cwd:input.directory,env:environment(false),signal}),names=new Set(input.workspace.projects.map(project=>project.name));
    if(result.exitCode!==0)throw Error("nx could not list affected projects: "+result.output.slice(-400));
    const listed=/\[[^\[\]]*\]/u.exec(result.output);
    const found:string[]=listed?(JSON.parse(listed[0]) as string[]):result.output.split("\n").map(line=>line.trim());
    return found.filter(line=>names.has(line));
   },
   async clearLocalCache(){rmSync(cache,{recursive:true,force:true});rmSync(data,{recursive:true,force:true});},
   async close(){}
  };
 }
});
