#!/usr/bin/env python3
"""apply-script.py <📜️script.ts> [<out>]: the plan-reuse patch of the Bazel adapter; every anchor must match exactly once."""
import sys
src=sys.argv[1]; dst=sys.argv[2] if len(sys.argv)>2 else src
s=open(src,encoding="utf8").read()
def rep(old,new):
    global s
    assert s.count(old)==1,(old[:90],s.count(old))
    s=s.replace(old,new)

rep('import {availableParallelism,totalmem} from "node:os";','import {availableParallelism,homedir,tmpdir,totalmem} from "node:os";')
rep('import {bazelActionClassV1,bazelScopeByIdV1,bazelScopeOfV1,bazelScopesFromEnvironmentV1} from "./🎬️scopes/🟦️.ts";',
'''import {bazelActionClassV1,bazelScopeByIdV1,bazelScopeOfV1,bazelScopesFromEnvironmentV1} from "./🎬️scopes/🟦️.ts";
import {readWorldWitnessSwitchV1,renderWorldWitnessTextV1,worldWitnessEnvironmentV1,worldWitnessGitDigestV1,worldWitnessPreloadV1,worldWitnessV1,type WorldWitnessRecorderV1} from "../../🔭️witness/🟨️.mjs";
import {bazelPlanFilesV1,bazelPlanKeyV1,bazelPlanOutputsV1,checkBazelPlanReuseV1,compareBazelPlanOutputsV1,recordBazelPlanV1,removeBazelPlanRecordV1,type BazelPlanOutputV1} from "./♻️plan-reuse/🟦️.ts";''')

# shared excluded names of the filesystem fallback
rep('const excluded=new Set([".git",".🧬semio",".cache",".nx","node_modules","target","dist","build","coverage",".venv","__pycache__"]),pending=bazelInputTraversalRootsV1(patterns),files:string[]=[];',
    'const excluded=BAZEL_SOURCE_EXCLUDED_V1,pending=bazelInputTraversalRootsV1(patterns),files:string[]=[];')

# universe observation + sourceFiles hook
rep('/** 📚️ Enumerates admitted source names through Git or a selected portable filesystem walk. */\nasync function sourceFiles(',
'''/** 🚫️ The directory names a source walk never enters; the filesystem fallback and the witness of the git universe agree on them. */
const BAZEL_SOURCE_EXCLUDED_V1:ReadonlySet<string>=new Set([".git",".🧬semio",".cache",".nx","node_modules","target","dist","build","coverage",".venv","__pycache__"]);
const BAZEL_GIT_ENVIRONMENT_V1=["GIT_DIR","GIT_WORK_TREE","GIT_INDEX_FILE","GIT_CONFIG_GLOBAL","GIT_CONFIG_SYSTEM","GIT_CONFIG_NOSYSTEM","GIT_EXCLUDE","GIT_CEILING_DIRECTORIES","GIT_DISCOVERY_ACROSS_FILESYSTEM","HOME","XDG_CONFIG_HOME"] as const;

/** 📋️ Reads the tracked-file listing of the given pathspecs: the answer the index gives and the half of the git universe that the filesystem does not show. */
async function bazelTrackedListingV1(root:string,pathspecs:readonly string[],signal:AbortSignal):Promise<Buffer>{
 const git=startToolV1("git",["-c","core.fsmonitor=false","ls-files","-c","-z",...(pathspecs.length?["--",...pathspecs]:[])],{cwd:root,stdio:["ignore","pipe","pipe"],windowsHide:true}),chunks:Buffer[]=[];let bytes=0;
 const cancel=()=>git.kill();signal.addEventListener("abort",cancel,{once:true});
 try{
  const code=await new Promise<number>((accept,reject)=>{git.stdout.on("data",chunk=>{bytes+=chunk.length;if(bytes>64*1024*1024){git.kill();reject(Error("Bazel source inventory exceeds 64 MiB"));}else chunks.push(chunk);});git.stderr.resume();git.once("error",reject);git.once("close",value=>accept(value??1));});
  signal.throwIfAborted();if(code!==0)throw Error("git ls-files failed");return Buffer.concat(chunks);
 }finally{signal.removeEventListener("abort",cancel);}
}

/** 🧾️ The digest of the tracked listing, or `!failed` where git cannot answer (no repository), which the filesystem fallback of the planner then observes itself. */
async function bazelTrackedDigestV1(root:string,pathspecs:readonly string[],signal:AbortSignal):Promise<string>{
 try{return worldWitnessGitDigestV1(await bazelTrackedListingV1(root,pathspecs,signal));}catch(error){signal.throwIfAborted();return "!failed";}
}

/** 🔭️ Makes the git universe observable before it is asked for: every directory below the traversal roots (their identities say whether a file appeared or vanished), every ignore file git consults, the git environment and the tracked listing; it runs before the planner's own `git ls-files -co`, so a change in between shows up as a difference and never as a silently stale universe. */
async function observeBazelSourceUniverseV1(witness:WorldWitnessRecorderV1,root:string,prefixes:readonly string[],pathspecs:readonly string[],invocation:ScriptInvocation):Promise<void>{
 if(prefixes.includes("")){witness.taint("the source universe is the whole repository");return;}
 for(const name of BAZEL_GIT_ENVIRONMENT_V1)void process.env[name];
 const home=process.env.HOME??homedir(),config=process.env.XDG_CONFIG_HOME||join(home,".config"),single=[join(root,".git/info/exclude"),join(root,".git/config"),join(home,".gitconfig"),join(config,"git/ignore"),join(config,"git/config"),"/etc/gitconfig"];
 await Promise.all(single.map(path=>lstat(path).catch(()=>undefined)));
 const pending:string[]=[];
 for(const prefix of prefixes){
  for(let directory=dirname(prefix);;directory=dirname(directory)){await lstat(join(root,directory==="."?"":directory,".gitignore")).catch(()=>undefined);if(directory==="."||directory==="/")break;}
  pending.push(prefix);
 }
 const seen=new Set<string>();
 const lane=async()=>{
  while(pending.length){
   checkScriptInvocation(invocation);const directory=pending.pop()!;if(seen.has(directory))continue;seen.add(directory);
   if(directory.split("/").some(part=>BAZEL_SOURCE_EXCLUDED_V1.has(part)))continue;
   let entries;try{entries=await readdir(join(root,directory),{withFileTypes:true});}catch{continue;}
   for(const entry of entries){
    if(entry.name===".gitignore")await lstat(join(root,directory,entry.name)).catch(()=>undefined);
    if(entry.isDirectory()&&!BAZEL_SOURCE_EXCLUDED_V1.has(entry.name))pending.push(slash(join(directory,entry.name)));
   }
   await invocation.control.yieldContinuation();
  }
 };
 await Promise.all(Array.from({length:16},lane));
 witness.observe("g",JSON.stringify(pathspecs),await witness.allow("git",()=>bazelTrackedDigestV1(root,pathspecs,invocation.control.signal)));
}

/** 📚️ Enumerates admitted source names through Git or a selected portable filesystem walk. */
async function sourceFiles(''')
rep('const prefixes=bazelInputTraversalRootsV1(patterns),pathspecs=prefixes.includes("")?[]:prefixes;',
    'const prefixes=bazelInputTraversalRootsV1(patterns),pathspecs=prefixes.includes("")?[]:prefixes,witness=worldWitnessV1();\n if(witness)await observeBazelSourceUniverseV1(witness,root,prefixes,pathspecs,invocation);')
rep('const git=startToolV1("git",["-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","-z",...(pathspecs.length?["--",...pathspecs]:[])],{cwd:root,stdio:["ignore","pipe","pipe"],windowsHide:true}),chunks:Buffer[]=[];let bytes=0;',
    'const spawnGit=()=>startToolV1("git",["-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","-z",...(pathspecs.length?["--",...pathspecs]:[])],{cwd:root,stdio:["ignore","pipe","pipe"],windowsHide:true}),git=witness?witness.allow("git",spawnGit):spawnGit(),chunks:Buffer[]=[];let bytes=0;')

# runtime commands
rep('stdout=captureOwnedCommandV1(process.execPath,["exec",row.runtime],root,{signal:phaseControl.signal,control:phaseControl,env:original});',
    'stdout=bazelRuntimeStdoutV1(root,row.runtime,original,phaseControl);')
rep('/** 📖️ Fingerprints only what Bazel cannot read from declared files:',
'''/** 🏃️ Runs the command of one runtime input; under the witness the spawn is allowed and the digest of its output is observed, so that a later run can ask the same question again. */
function bazelRuntimeStdoutV1(root:string,runtime:string,original:NodeJS.ProcessEnv,phaseControl:ResourceLeaseControl):Promise<string>{
 const run=()=>captureOwnedCommandV1(process.execPath,["exec",runtime],root,{signal:phaseControl.signal,control:phaseControl,env:original}),witness=worldWitnessV1();
 if(!witness)return run();
 return witness.allow("runtime",async()=>{const stdout=await run();witness.observe("x",runtime+"\\0"+String(original.SEMIO_BUILD_MODE),hash(JSON.stringify(stdout)));return stdout;});
}

/** 📖️ Fingerprints only what Bazel cannot read from declared files:''')

# planner side: finish
rep('await writeBazelImmutableInputV1(output,JSON.stringify(result),plannerSchema.definitions.Policy.const.maximumDataBytes,original);checkScriptInvocation(original);});});return;}',
    'await writeBazelImmutableInputV1(output,JSON.stringify(result),plannerSchema.definitions.Policy.const.maximumDataBytes,original);checkScriptInvocation(original);worldWitnessV1()?.finish();});});return;}')

# parent side
rep('export async function publishBazelGraphV1(root:string,graph:BuildGraphV1,request:BazelTaskRequestV1,invocation:ScriptInvocation,storage:BazelStorageV1,phaseControl:ResourceLeaseControl,slot:string):Promise<BazelPublicationV1&Pick<BazelImplementationsV1,"adapter">>{',
'''/** 🧮️ The digest of the graph a plan is made from: the one the kernel pinned for this run, or the digest of the graph itself. */
const bazelGraphDigestsV1=new WeakMap<object,string>();
function bazelGraphDigestV1(graph:BuildGraphV1):string{
 const declared=process.env.SEMIO_BUILD_GRAPH_DIGEST;if(declared!==undefined&&/^[a-f0-9]{64}$/u.test(declared))return declared;
 let known=bazelGraphDigestsV1.get(graph);if(!known){known=hash(JSON.stringify(graph));bazelGraphDigestsV1.set(graph,known);}return known;
}

/** ♻️ What a plan reuse decision hands back: the finished publication of an earlier identical plan, or the facts the planner run needs to leave a record behind. */
export interface BazelPlanReuseV1{result?:BazelPublicationV1&Pick<BazelImplementationsV1,"adapter">;record?:{key:string;ignore:ReadonlySet<string>;preload:string;out:string;proven?:readonly BazelPlanOutputV1[]}}

/** ♻️ Decides whether the planner has to run: the key of everything the plan is a function of must be the same as the key of the last plan in this slot, the planner's witness of the world must still hold, and the workspace must be as the planner left it; then the earlier publication is returned and the planner child never starts. `SEMIO_REPO_PLAN_REUSE=off` always plans, `verify` always plans and reports when the earlier plan would have differed. */
export async function prepareBazelPlanReuseV1(root:string,graph:BuildGraphV1,request:BazelTaskRequestV1,planned:BazelTaskPlanV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,workspace:string,scopeIds:readonly string[],emitted:ReadonlyMap<string,BazelActionV1>,identity:string|undefined=(import.meta as {semioCapturedImplementationIdentity?:string}).semioCapturedImplementationIdentity):Promise<BazelPlanReuseV1>{
 const mode=readWorldWitnessSwitchV1(process.env),language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,say=(table:"Messages"|"Reasons",name:string,values:Record<string,string|number>={})=>console.log(renderWorldWitnessTextV1(table,language,name,values));
 if(mode==="off"||identity===undefined){await removeBazelPlanRecordV1(workspace);return {};}
 const preload=worldWitnessPreloadV1(),ignore=new Set([...emitted.values()].filter(action=>(action.kind==="task"||action.kind==="test")&&!action.cache.cache).map(action=>plannedLabelV1(planned,action.id)+".input"));
 const key=bazelPlanKeyV1({version:1,root,workspace,request,graph:bazelGraphDigestV1(graph),executor:identity,scopes:{ids:scopeIds,rows:hash(JSON.stringify(bazelScopesFromEnvironmentV1(process.env)))},capabilities:readIssuedScriptProcessEnvelopeV1(invocation).capabilities,runtime:{execPath:process.execPath,bun:(globalThis as {Bun?:{version?:string}}).Bun?.version??process.version,platform:process.platform,arch:process.arch},preload:hash(preload)});
 const started=performance.now(),ports={
  git:async(subject:string,expected:string)=>(await bazelTrackedDigestV1(root,JSON.parse(subject) as string[],phaseControl.signal))===expected,
  runtime:async(subject:string,expected:string)=>{
   const[runtime,modeName]=subject.split("\\0"),task=Object.values(planned.tasks)[0];if(!runtime||!task)return false;
   const original=issueBazelTaskEnvironmentV1(root,invocation,task,bazelTaskBaseEnvironmentV1(root,task,process.env));
   if(String(original.SEMIO_BUILD_MODE)!==modeName)return false;
   return hash(JSON.stringify(await captureOwnedCommandV1(process.execPath,["exec",runtime],root,{signal:phaseControl.signal,control:phaseControl,env:original})))===expected;
  },
 };
 const verdict=await profilePhaseV1("bazel.reuse",()=>checkBazelPlanReuseV1({workspace,key,ignore,signal:phaseControl.signal,ports,environment:scriptInvocationEnvironment(invocation,process.env),progress:()=>advanceScriptInvocation(invocation,"bazel:plan-reuse","running")}));
 if(verdict.reuse&&mode==="on"){
  for(const name of ignore)await writeChanged(join(workspace,name),hash(randomUUID()));
  const seconds=((performance.now()-started)/1000).toFixed(1);profileMarkV1("bazel.plan","reused");
  say("Messages","reused",{observations:verdict.checked,age:Math.round((Date.now()-verdict.receipt.recordedAtMs)/1000),seconds});
  const scopes=verdict.receipt.scopes;
  return {result:{workspace,plan:planned,implementation:verdict.receipt.implementation,adapter:verdict.receipt.adapter,...(Object.keys(scopes).length?{scopes}:{})}};
 }
 await removeBazelPlanRecordV1(workspace);profileMarkV1("bazel.plan","planned");
 if(!verdict.reuse&&verdict.reason!=="absent")say("Messages","replanned",{reason:renderWorldWitnessTextV1("Reasons",language,verdict.reason==="world"?"world":verdict.reason,{kind:verdict.kind??"",subject:verdict.subject??""})});
 await writeChanged(join(workspace,bazelPlanFilesV1.preload),preload);
 return {record:{key,ignore,preload:join(workspace,bazelPlanFilesV1.preload),out:join(workspace,bazelPlanFilesV1.witness),...(verdict.reuse&&mode==="verify"?{proven:verdict.receipt.outputs}:{})}};
}
function plannedLabelV1(plan:BazelTaskPlanV1,id:string):string{return plan.tasks[id]!.label;}

export async function publishBazelGraphV1(root:string,graph:BuildGraphV1,request:BazelTaskRequestV1,invocation:ScriptInvocation,storage:BazelStorageV1,phaseControl:ResourceLeaseControl,slot:string):Promise<BazelPublicationV1&Pick<BazelImplementationsV1,"adapter">>{''')
rep('await mkdir(workspace,{recursive:true,mode:0o700});const {adapter,implementation,scopes}=await profilePhaseV1("bazel.implementation",()=>publishBazelImplementationV1(root,workspace,invocation,[...new Set(bazelScopeAssignmentsV1(graph,planned,bazelActionsV1(graph,planned,request.skipCache===true,{hermetic:true}),process.env,true).values())])),input=join(workspace,"planner-input.json")',
    'await mkdir(workspace,{recursive:true,mode:0o700});const emitted=bazelActionsV1(graph,planned,request.skipCache===true,{hermetic:true}),scopeIds=[...new Set(bazelScopeAssignmentsV1(graph,planned,emitted,process.env,true).values())],reuse=await prepareBazelPlanReuseV1(root,graph,request,planned,invocation,phaseControl,workspace,scopeIds,emitted).catch((error:unknown):BazelPlanReuseV1=>{if(phaseControl.signal.aborted)throw error;console.warn("[bazel] plan reuse unavailable, planning completely: "+String((error as Error)?.message??error).slice(0,300));return {};});if(reuse.result)return reuse.result;const {adapter,implementation,scopes}=await profilePhaseV1("bazel.implementation",()=>publishBazelImplementationV1(root,workspace,invocation,scopeIds)),input=join(workspace,"planner-input.json")')
rep('await profilePhaseV1("bazel.planner",()=>runOwnedCommand(process.execPath,[adapter.runner,adapter.snapshot,adapter.sha256,"planner",input,inputIdentity,output],root,"bazel:captured-planner",compilerAdmissionBudgetV1(phaseControl),{env:scriptInvocationEnvironment(invocation,process.env),signal:phaseControl.signal,onProgress:()=>advanceScriptInvocation(invocation,"bazel:captured-planner","running")}));',
    '''const runPlanner=(witnessed:boolean)=>runOwnedCommand(process.execPath,[...(witnessed?["--preload",reuse.record!.preload]:[]),adapter.runner,adapter.snapshot,adapter.sha256,"planner",input,inputIdentity,output],root,"bazel:captured-planner",compilerAdmissionBudgetV1(phaseControl),{env:{...scriptInvocationEnvironment(invocation,process.env),...(witnessed?worldWitnessEnvironmentV1({out:reuse.record!.out,root,workspace,writable:[tmpdir()]}):{})},signal:phaseControl.signal,onProgress:()=>advanceScriptInvocation(invocation,"bazel:captured-planner","running")});
await profilePhaseV1("bazel.planner",async()=>{if(!reuse.record){await runPlanner(false);return;}try{await runPlanner(true);}catch(error){if(phaseControl.signal.aborted)throw error;console.warn("[bazel] the planner failed under the world witness, planning again without it: "+String((error as Error)?.message??error).slice(0,300));reuse.record=undefined;await rm(output,{force:true});await runPlanner(false);}});''')
rep('throw new Error("Closed captured Bazel planner result required");checkScriptInvocation(invocation);return result;},async()=>{',
    'throw new Error("Closed captured Bazel planner result required");checkScriptInvocation(invocation);if(reuse.record)try{const outcome=await recordBazelPlanV1({workspace,key:reuse.record.key,implementation,adapter,scopes,ignore:reuse.record.ignore,signal:phaseControl.signal,nowMs:Date.now()}),language=resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language;if(!outcome.recorded)console.log(renderWorldWitnessTextV1("Messages",language,"withheld",{reason:renderWorldWitnessTextV1("Reasons",language,outcome.reason,{detail:outcome.detail})}));else if(reuse.record.proven){const moved=compareBazelPlanOutputsV1(reuse.record.proven.filter(row=>!reuse.record!.ignore.has(row.name)),(await bazelPlanOutputsV1(workspace,reuse.record.ignore,true,phaseControl.signal)));if(moved.length)console.error(renderWorldWitnessTextV1("Messages",language,"verifyFailed",{names:moved.join(", ")}));}}catch(error){if(phaseControl.signal.aborted)throw error;console.warn("[bazel] plan record failed: "+String((error as Error)?.message??error).slice(0,300));}return result;},async()=>{')
open(dst,"w",encoding="utf8").write(s)
print("patched",dst)
