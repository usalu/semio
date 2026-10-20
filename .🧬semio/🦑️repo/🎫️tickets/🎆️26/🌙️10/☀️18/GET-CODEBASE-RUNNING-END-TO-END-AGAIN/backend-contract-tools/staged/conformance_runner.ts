import {existsSync,mkdirSync,readFileSync,rmSync,appendFileSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {spawn} from "node:child_process";
import {join} from "node:path";
import {buildBackendIdsV1,type BackendCapabilityV1} from "../../🔌️backends/🟦️.ts";
import {CONFORMANCE_POLICY_V1 as policy,conformanceClaimV1,conformanceEvaluateRunV1,conformanceMarkersV1,conformanceReferenceOutputsV1,conformanceReportV1,conformanceVerdictV1,writeConformanceWorkspaceV1,type ConformanceEventV1,type ConformanceExpectV1,type ConformancePlanV1,type ConformanceReportV1,type ConformanceRowV1,type ConformanceRunOptionsV1,type ConformanceRunV1,type ConformanceScenarioV1,type ConformanceSetV1,type ConformanceStepV1,type ConformanceWorkspaceV1} from "../🟦️.ts";
import type {StubSpecV1} from "../🎭️stub/🟦️.ts";

/** 🧰️ Everything a driver gets to prepare one isolated workspace. */
export type ConformancePrepareV1=Readonly<{workspace:ConformanceWorkspaceV1;directory:string;cache:string;markers:string;spec:StubSpecV1;specPath:string;stub:string;runtime:string;remote?:Readonly<{url:string}>}>;
/** 🔌️ One prepared workspace as a backend sees it. */
export interface ConformanceSessionV1{
 readonly observes:ReadonlySet<string>;
 plan(target:string,signal:AbortSignal):Promise<ConformancePlanV1>;
 run(targets:readonly string[],options:ConformanceRunOptionsV1,signal:AbortSignal):Promise<ConformanceRunV1>;
 affected(files:readonly string[],signal:AbortSignal):Promise<readonly string[]>;
 clearLocalCache():Promise<void>;
 close():Promise<void>;
}
/** 🤝️ A backend's binding to the suite; the module named by its registry row exports one as `driver`. */
export interface ConformanceDriverV1{readonly id:string;prepare(input:ConformancePrepareV1):Promise<ConformanceSessionV1>}
/** 🛰️ A shared cache server a scenario can use. */
export type ConformanceRemoteV1=Readonly<{url:string;stop():Promise<void>}>;
/** 🏠️ What the runner needs from the outside. */
export type ConformanceHostV1=Readonly<{scratch:string;runtime:string;stubSource:string;signal:AbortSignal;loadDriver(id:string):Promise<ConformanceDriverV1>;startRemote():Promise<ConformanceRemoteV1>;now():number;progress(stage:string):Promise<void>}>;

const order=(left:string,right:string)=>left<right?-1:left>right?1:0,sorted=(values:Iterable<string>)=>[...new Set(values)].sort(order);
const sleep=(milliseconds:number,signal:AbortSignal)=>new Promise<void>(accept=>{const timer=setTimeout(accept,milliseconds);signal.addEventListener("abort",()=>{clearTimeout(timer);accept();},{once:true});});
const digestOf=(path:string):string|null=>existsSync(path)?createHash("sha256").update(readFileSync(path)).digest("hex"):null;

/** 🎬️ Runs one scenario for one backend in an isolated workspace and returns its failures and unobserved expectations. */
export async function conformanceRunScenarioV1(set:ConformanceSetV1,scenario:ConformanceScenarioV1,backend:string,host:ConformanceHostV1):Promise<{failures:string[];unobserved:string[]}>{
 const base=join(host.scratch,backend,scenario.id),directory=join(base,"ws"),cache=join(base,"cache"),markers=join(base,"markers.log");
 rmSync(base,{recursive:true,force:true});mkdirSync(directory,{recursive:true});mkdirSync(cache,{recursive:true});writeFileSync(markers,"");
 const written=writeConformanceWorkspaceV1(set.workspace,directory,host.stubSource,markers),spec=written.spec,paths=Object.values(spec.tasks).flatMap(task=>task.outputs),edges:Record<string,string[]>={};
 for(const project of set.workspace.projects)for(const[target,row]of Object.entries(project.targets))edges[`${project.name}:${target}`]=[...(row.dependsOn??[])];
 const failures:string[]=[],unobserved:string[]=[],controller=new AbortController(),signal=AbortSignal.any([host.signal,controller.signal]);
 let remote:ConformanceRemoteV1|undefined,session:ConformanceSessionV1|undefined,nonce=0;
 const driver=await host.loadDriver(backend),sourceFiles=Object.keys(set.workspace.files);
 const snapshot=()=>Object.fromEntries(paths.map(path=>[path,digestOf(join(directory,path))]));
 const reference=()=>conformanceReferenceOutputsV1(spec,sourceFiles,path=>readFileSync(join(directory,path),"utf8"));
 const hashed=()=>Object.fromEntries(Object.entries(reference()).map(([path,body])=>[path,createHash("sha256").update(body).digest("hex")]));
 const marks=()=>conformanceMarkersV1(existsSync(markers)?readFileSync(markers,"utf8"):"");
 const judge=(label:string,expect:ConformanceExpectV1,run:ConformanceRunV1,before:Record<string,string|null>,from:number)=>{
  const result=conformanceEvaluateRunV1(expect,{run,markers:marks().slice(from),before,after:snapshot(),expected:hashed()},edges);
  failures.push(...result.failures.map(failure=>`${label}: ${failure}`));unobserved.push(...result.unobserved.map(name=>`${label}: ${name}`));
 };
 const limited=()=>AbortSignal.any([signal,AbortSignal.timeout(policy.maximumRunMilliseconds)]);
 try{
  if(scenario.steps.some(step=>step.step==="run"&&step.options?.remote==="read-write"))remote=await host.startRemote();
  const prepare=async()=>driver.prepare({workspace:set.workspace,directory,cache,markers,spec,specPath:written.specPath,stub:written.stub,runtime:host.runtime,...(remote?{remote:{url:remote.url}}:{})});
  session=await prepare();
  let index=0;
  for(const step of scenario.steps as readonly ConformanceStepV1[]){
   const label=`step ${++index} ${step.step}`;
   await host.progress(`${backend}:${scenario.id}:${label}`);
   if(signal.aborted)throw Error("Cancelled");
   if(step.step==="edit"){
    const file=join(directory,step.file);nonce++;
    appendFileSync(file,step.op==="append-comment"?`# edit ${host.now()}-${nonce}\n`:`edit ${host.now()}-${nonce}\n`);
   }else if(step.step==="delete"){for(const path of step.paths)rmSync(join(directory,path),{force:true});
   }else if(step.step==="fresh-checkout"){for(const path of paths)rmSync(join(directory,path),{force:true});await session.clearLocalCache();
   }else if(step.step==="plan"){
    const plan=await session.plan(step.target,limited());
    if(JSON.stringify(sorted(plan.tasks))!==JSON.stringify(sorted(step.tasks)))failures.push(`${label}: tasks [${sorted(plan.tasks).join(", ")}], expected [${sorted(step.tasks).join(", ")}]`);
    for(const task of plan.tasks){const want=sorted(step.edges[task]??[]),have=sorted(plan.dependencies[task]??[]);if(JSON.stringify(want)!==JSON.stringify(have))failures.push(`${label}: ${task} depends on [${have.join(", ")}], expected [${want.join(", ")}]`);}
   }else if(step.step==="affected"){
    const projects=sorted(await session.affected(step.files,limited()));
    if(JSON.stringify(projects)!==JSON.stringify(sorted(step.projects)))failures.push(`${label}: affected [${projects.join(", ")}], expected [${sorted(step.projects).join(", ")}]`);
   }else if(step.step==="run"){
    const before=snapshot(),from=marks().length,run=await session.run(step.targets,step.options??{},limited());
    judge(label,step.expect,run,before,from);
   }else if(step.step==="concurrent"){
    const before=snapshot(),from=marks().length,runs=await Promise.all(step.runs.map(targets=>session!.run(targets,{},limited())));
    const worst=runs.find(run=>run.exitCode!==0)??runs[0]!;
    judge(label,step.expect,{...worst,milliseconds:Math.max(...runs.map(run=>run.milliseconds))},before,from);
   }else if(step.step==="cancel"){
    const from=marks().length,stopper=new AbortController(),pending=session.run(step.targets,{},AbortSignal.any([signal,stopper.signal])).catch(error=>({exitCode:130,milliseconds:0,output:String(error)} as ConformanceRunV1));
    const started=host.now();
    while(!marks().slice(from).some(marker=>marker.kind==="start"&&marker.task===step.afterStart)){if(host.now()-started>30000){failures.push(`${label}: ${step.afterStart} never started`);break;}await sleep(25,signal);}
    const cancelledAt=host.now();stopper.abort(Error("Cancelled by the conformance suite"));
    const run=await Promise.race([pending,sleep(policy.cancelWithinMilliseconds*2,signal).then(()=>undefined)]);
    if(run===undefined)failures.push(`${label}: the run did not return within ${policy.cancelWithinMilliseconds*2} ms of the cancellation`);
    else{
     if(host.now()-cancelledAt>policy.cancelWithinMilliseconds)failures.push(`${label}: the run needed ${host.now()-cancelledAt} ms to stop, bound ${policy.cancelWithinMilliseconds}`);
     if(run.exitCode===0)failures.push(`${label}: a cancelled run exited 0`);
    }
    await sleep(policy.cancelSettleMilliseconds,signal);
    const after=marks().slice(from);
    if(after.some(marker=>marker.kind==="end"&&marker.task===step.settled))failures.push(`${label}: ${step.settled} kept running after the cancellation`);
    const produced=spec.tasks[step.settled]!.outputs.filter(path=>existsSync(join(directory,path)));
    if(produced.length)failures.push(`${label}: cancelled task published ${produced.join(", ")}`);
   }
  }
 }catch(error){
  failures.push(`error: ${(error as Error).stack?.split("\n").slice(0,2).join(" ")??String(error)}`);
  unobserved.length=0;
  return {failures:[...failures,"@error"],unobserved};
 }finally{
  controller.abort();
  await session?.close().catch(()=>undefined);
  await remote?.stop().catch(()=>undefined);
 }
 if(!failures.length&&scenario.capability==="progress"&&!session.observes.has("progress"))unobserved.push("progress");
 return {failures,unobserved};
}

/** 🏁️ Runs the selected scenarios for the selected backends and judges each against the registry's claim. */
export async function runConformanceV1(set:ConformanceSetV1,backends:readonly string[]|"all",scenarios:readonly string[]|"all",host:ConformanceHostV1):Promise<ConformanceReportV1>{
 const chosen=backends==="all"?buildBackendIdsV1():backends,known=new Set(set.scenarios.map(scenario=>scenario.id)),wanted=scenarios==="all"?set.scenarios:scenarios.map(id=>{if(!known.has(id))throw Error(`Unknown scenario: ${id} (${[...known].join("|")})`);return set.scenarios.find(scenario=>scenario.id===id)!;});
 const rows:ConformanceRowV1[]=[];
 for(const backend of chosen)for(const scenario of wanted){
  if(host.signal.aborted)throw Error("Cancelled");
  await host.progress(`${backend}:${scenario.id}`);
  const started=host.now(),claim=conformanceClaimV1(backend,scenario.capability as BackendCapabilityV1),result=await conformanceRunScenarioV1(set,scenario,backend,host);
  const errored=result.failures.includes("@error"),failures=result.failures.filter(failure=>failure!=="@error"),measured=errored?"error":failures.length?"fail":result.unobserved.length?"unobserved":"pass";
  rows.push(Object.freeze({scenario:scenario.id,backend,capability:scenario.capability,claim,measured,verdict:conformanceVerdictV1(claim,measured),failures:[...failures,...result.unobserved.map(name=>`not observable: ${name}`)],milliseconds:Math.max(0,host.now()-started)}));
 }
 return conformanceReportV1(rows);
}

/** 🧵️ Runs a process for a driver and returns its exit code and combined output. On abort it interrupts the whole process tree, then terminates it, then kills it, so a cancelled backend leaves nothing running. */
export function conformanceSpawnV1(command:string,args:readonly string[],options:Readonly<{cwd:string;env:NodeJS.ProcessEnv;signal:AbortSignal}>):Promise<{exitCode:number;output:string}>{
 return new Promise(accept=>{
  const child=spawn(command,[...args],{cwd:options.cwd,env:options.env,stdio:["ignore","pipe","pipe"],detached:process.platform!=="win32",windowsHide:true}),chunks:string[]=[];
  const send=(name:NodeJS.Signals)=>{if(child.pid===undefined)return;if(process.platform==="win32"){spawn("taskkill",["/pid",String(child.pid),"/T","/F"],{stdio:"ignore",windowsHide:true});return;}try{process.kill(-child.pid,name);}catch{try{child.kill(name);}catch{}}};
  const timers:ReturnType<typeof setTimeout>[]=[],stop=()=>{send("SIGINT");timers.push(setTimeout(()=>send("SIGTERM"),1500),setTimeout(()=>send("SIGKILL"),4000));};
  if(options.signal.aborted)stop();else options.signal.addEventListener("abort",stop,{once:true});
  child.stdout!.setEncoding("utf8").on("data",chunk=>chunks.push(chunk));child.stderr!.setEncoding("utf8").on("data",chunk=>chunks.push(chunk));
  child.once("error",error=>{options.signal.removeEventListener("abort",stop);timers.forEach(clearTimeout);accept({exitCode:127,output:String(error)});});
  child.once("close",code=>{options.signal.removeEventListener("abort",stop);timers.forEach(clearTimeout);accept({exitCode:code??130,output:chunks.join("")});});
 });
}
