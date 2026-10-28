"""🛠️ Anchored, idempotent edits of batch `planfp` on a copy of 📜️script.ts: the planner observes kinds, snapshot capture and fingerprints move to the publisher.
usage: python3 -I edit_script.py <script.ts>"""
import sys
p=sys.argv[1];t=open(p,encoding="utf8").read()
def rep(old,new):
    global t
    if t.count(new)==1 and new not in old: return
    assert t.count(old)==1,("ANCHOR",t.count(old),old[:120]); t=t.replace(old,new)
rep('import {captureBazelSourceSnapshotV1,renderBazelSourceSnapshotV1} from "./📸️snapshot/🟦️.ts";',
    'import {bazelSourceSnapshotDirectoriesV1,bazelSourceSnapshotEntriesV1,bazelSourceSnapshotPlanFileV1,bazelSourceSnapshotPlanTextV1,captureBazelSourceSnapshotV1,readBazelSourceSnapshotPlanV1,renderBazelSourceSnapshotV1} from "./📸️snapshot/🟦️.ts";\nimport {concurrentEditV1} from "../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";')
rep('import {readWorldWitnessSwitchV1,renderWorldWitnessTextV1,worldWitnessEnvironmentV1,',
    'import {readWorldWitnessSwitchV1,renderWorldWitnessTextV1,worldEntryKindV1,worldWitnessEnvironmentV1,')
rep('aliasRule=actions.sources.alias','aliasRule=actions.sources.alias,fingerprintPlanSchema=componentSchema.definitions.FingerprintPlan')
# source classes: kinds only
rep('''/** 🚧️ Splits selected paths into files Bazel can declare by label and paths whose identity the fingerprint must hash instead. */
export async function bazelSourceClassesV1(root:string,paths:readonly string[],classes:Map<string,Promise<"file"|"hash"|"absent">>,boundaries:Map<string,Promise<boolean>>,invocation:ScriptInvocation):Promise<{files:string[];hashed:string[]}>{
 const boundary=(directory:string)=>{let known=boundaries.get(directory);if(!known){known=(async()=>{for(const name of actions.sources.boundaries)try{await lstat(join(root,directory,name));return true;}catch(error){if(!["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))throw error;}return false;})();boundaries.set(directory,known);}return known;};
 const classify=async(path:string):Promise<"file"|"hash"|"absent">=>{
  let info;try{info=await lstat(join(root,path));}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return "absent";throw error;}
  if(info.isSymbolicLink())try{info=await stat(join(root,path));}catch(error){if(["ENOENT","ELOOP","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return "hash";throw error;}
  const parts=path.split("/");if(!info.isFile()||!sourceLabel.test(path)||parts.some(part=>part===""||part==="."||part===".."))return "hash";''',
'''/** 🚧️ Splits selected paths into files Bazel can declare by label and paths whose identity the fingerprint must hash instead. It asks only for entry kinds, so under the witness the class of a file depends on which entries exist and what they are, never on bytes: a content edit keeps the plan. */
export async function bazelSourceClassesV1(root:string,paths:readonly string[],classes:Map<string,Promise<"file"|"hash"|"absent">>,boundaries:Map<string,Promise<boolean>>,invocation:ScriptInvocation):Promise<{files:string[];hashed:string[]}>{
 const unreadable=(path:string,kind:string)=>new Error(`Bazel source entry unreadable (${kind}): ${path}`);
 const boundary=(directory:string)=>{let known=boundaries.get(directory);if(!known){known=(async()=>{for(const name of actions.sources.boundaries){const kind=await worldEntryKindV1(join(root,directory,name));if(!kind.startsWith("!"))return true;if(kind!=="!ENOENT"&&kind!=="!ENOTDIR")throw unreadable(join(directory,name),kind);}return false;})();boundaries.set(directory,known);}return known;};
 const classify=async(path:string):Promise<"file"|"hash"|"absent">=>{
  let kind=await worldEntryKindV1(join(root,path));if(kind==="!ENOENT"||kind==="!ENOTDIR")return "absent";if(kind.startsWith("!"))throw unreadable(path,kind);
  if(kind==="link"){kind=await worldEntryKindV1(join(root,path),true);if(kind==="!ENOENT"||kind==="!ELOOP"||kind==="!ENOTDIR")return "hash";if(kind.startsWith("!"))throw unreadable(path,kind);}
  const parts=path.split("/");if(kind!=="file"||!sourceLabel.test(path)||parts.some(part=>part===""||part==="."||part===".."))return "hash";''')
# fingerprint head + fingerprint + refresh
rep('''/** 📖️ Fingerprints only what Bazel cannot read from declared files: host identity, declared environment and runtime commands (each command runs once per distinct text and build mode, like Nx's runtime hasher), and paths without a declarable label. */
async function taskFingerprint(root:string,graph:BuildGraphV1,task:BazelTaskV1,files:string[],memo:Map<string,Promise<string>>,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,cached:boolean,implementation:string,exactClosures:BazelClosuresV1,resolutions:ReadonlyMap<string,string>):Promise<string>{''',
'''/** 📖️ The head of a task fingerprint, what the planner decides and Bazel cannot read from declared files: host identity, implementation, declared environment and runtime commands (each command runs once per distinct text and build mode, like Nx's runtime hasher); the identities of the paths without a declarable label follow in {@link bazelTaskFingerprintV1}. */
async function taskFingerprintHeadV1(root:string,graph:BuildGraphV1,task:BazelTaskV1,memo:Map<string,Promise<string>>,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,cached:boolean,implementation:string,exactClosures:BazelClosuresV1,resolutions:ReadonlyMap<string,string>):Promise<string>{''')
rep(''' if(!cached)digest.update(randomUUID());
 for(const path of [...files].sort()){checkScriptInvocation(invocation);let identity=memo.get(path);if(!identity){identity=bazelSourceIdentityV1(join(root,path),invocation.control.signal);memo.set(path,identity);}digest.update(path+"\\0"+await identity+"\\0");}
 for(const input of inputs){''',''' for(const input of inputs){''')
rep('''
/** ✍️ Keeps unchanged graph inputs stable for Bazel's own incremental analysis. */''','''
/** 🪙️ The fingerprint of one task: its head followed by the identity of every hashed path in path order; the publisher computes it after every plan, so it follows the bytes while the plan follows the file set. */
export async function bazelTaskFingerprintV1(root:string,head:string,files:readonly string[],memo:Map<string,Promise<string>>,signal:AbortSignal):Promise<string>{
 const digest=createHash("sha256").update(head);
 for(const path of [...files].sort()){signal.throwIfAborted();let identity=memo.get(path);if(!identity){identity=bazelSourceIdentityV1(join(root,path),signal);memo.set(path,identity);}digest.update(path+"\\0"+await identity+"\\0");}
 return digest.digest("hex");
}

/** 🔄️ Publishes what a plan leaves to its publisher, after a fresh plan and after a reused one: captures the planned sources into the workspace (an unchanged file keeps its copy) and writes `<label>.input` from every fingerprint plan by hashing its paths now (an uncached task gets a fresh value every time), so a content edit refreshes copies and fingerprints while the plan stays reusable. A planned file that vanished is a concurrent edit: the plan record is removed so that the retry plans again. */
export async function refreshBazelWorkspaceSourcesV1(root:string,workspace:string,labels:readonly string[],invocation:ScriptInvocation,pool?:string,verbose=false):Promise<{captured:number;reused:number;fingerprints:number}>{
 const planned=await readBazelSourceSnapshotPlanV1(workspace),snapshot=await profilePhaseV1("bazel.source-snapshot",()=>captureBazelSourceSnapshotV1({root,directory:workspace,...pool===undefined?{}:{pool},files:planned.files,real:new Set(planned.real),views:{alias:aliasRule.link,real:actions.sources.link},alias:directory=>bazelSourceAliasV1(directory),check:()=>checkScriptInvocation(invocation),progress:async()=>{await advanceScriptInvocation(invocation,"bazel:source-snapshot","running");await invocation.control.yieldContinuation();checkScriptInvocation(invocation);}}));
 if(verbose||snapshot.vanished.length)console.log(renderBazelSourceSnapshotV1(snapshot,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language));
 if(snapshot.vanished.length){await removeBazelPlanRecordV1(workspace);throw concurrentEditV1("vanished",snapshot.vanished[0]!,undefined,"snapshot");}
 const memo=new Map<string,Promise<string>>();let fingerprints=0;
 await profilePhaseV1("bazel.fingerprint-refresh",async()=>{for(const label of [...new Set(labels)].sort()){
  checkScriptInvocation(invocation);let text:string;try{text=await readFile(join(workspace,label+actions.fingerprints.plan),"utf8");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}
  const plan=JSON.parse(text) as {head:string;cached:boolean;files:string[]};if(validateJsonSchemaSubset(fingerprintPlanSchema,plan).length)throw new Error("Closed Bazel fingerprint plan required: "+label);
  await writeChanged(join(workspace,label+actions.fingerprints.value),plan.cached?await bazelTaskFingerprintV1(root,plan.head,plan.files,memo,invocation.control.signal):hash(randomUUID()));fingerprints++;
 }});
 return {captured:snapshot.captured,reused:snapshot.reused,fingerprints};
}

/** ✍️ Keeps unchanged graph inputs stable for Bazel's own incremental analysis. */''')
rep('''if(!launch)attributes.push(["fingerprint",quote(action.label+".input")]);''','''if(!launch)attributes.push(["fingerprint",quote(action.label+actions.fingerprints.value)]);''')
# plan reuse: the publisher owns .input and the snapshot entries; reuse refreshes them
rep('''ignore=new Set([...emitted.values()].filter(action=>(action.kind==="task"||action.kind==="test")&&!action.cache.cache).map(action=>plannedLabelV1(planned,action.id)+".input"));''',
    '''ignore=new Set([...Object.values(planned.tasks).map(task=>task.label+actions.fingerprints.value),...bazelSourceSnapshotEntriesV1({alias:aliasRule.link,real:actions.sources.link})]);''')
rep('''  for(const name of ignore)await writeChanged(join(workspace,name),hash(randomUUID()));
''','')
rep('''if(reuse.result)return reuse.result;await profilePhaseV1("capacity.upgrade"''','''const refresh=()=>refreshBazelWorkspaceSourcesV1(root,workspace,Object.values(planned.tasks).map(task=>task.label),invocation,storage.paths.pool,request.verbose===true);if(reuse.result){await refresh();return reuse.result;}await profilePhaseV1("capacity.upgrade"''')
rep('''throw new Error("Closed captured Bazel planner result required");checkScriptInvocation(invocation);if(reuse.record)try{''','''throw new Error("Closed captured Bazel planner result required");checkScriptInvocation(invocation);await refresh();if(reuse.record)try{''')
rep('''storage.paths.pool,adapter,scopes);
}''','''storage.paths.pool,adapter,scopes,"defer");
}''')
rep('''adapter?:BazelImplementationV1,scopes:Readonly<Record<string,BazelImplementationV1>>={}):Promise<BazelPublicationV1>{
 const candidates=''','''adapter?:BazelImplementationV1,scopes:Readonly<Record<string,BazelImplementationV1>>={},publish:"capture"|"defer"="capture"):Promise<BazelPublicationV1>{
 const candidates=''')
rep('''/** 🪡 Writes the complete Bazel package for a planned selection into a workspace directory: manifests, fingerprints, source links, rules and BUILD, independent of storage and of any Bazel server. */''',
    '''/** 🪡 Writes the complete Bazel package for a planned selection into a workspace directory: manifests, fingerprint plans, the capture plan, rules and BUILD, independent of storage and of any Bazel server; `capture` then publishes copies and fingerprints itself, `defer` leaves them to {@link refreshBazelWorkspaceSourcesV1} in the publishing process, so the planner never reads source bytes. */''')
rep(''',snapshot=await profilePhaseV1("bazel.source-snapshot",()=>captureBazelSourceSnapshotV1({root,directory:workspace,...pool===undefined?{}:{pool},files:classified.files,real:new Set(declared.filter(action=>action.kind==="hermetic").flatMap(action=>{const{marks,extra}=index.selectAll(inputs.get(action.id)!);return [...index.files.filter((_path,position)=>marks[position]),...extra];})),views:{alias:aliasRule.link,real:actions.sources.link},alias:directory=>bazelSourceAliasV1(directory),check:()=>checkScriptInvocation(invocation),progress:async()=>{await advanceScriptInvocation(invocation,"bazel:source-snapshot","running");await invocation.control.yieldContinuation();checkScriptInvocation(invocation);}})),declarable=new Set(snapshot.files),''',
    ''',declarable=new Set(classified.files),''')
rep(''' if(request.verbose||snapshot.vanished.length)console.log(renderBazelSourceSnapshotV1(snapshot,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language));
''',''' await writeChanged(join(workspace,bazelSourceSnapshotPlanFileV1),bazelSourceSnapshotPlanTextV1(classified.files,declared.filter(action=>action.kind==="hermetic").flatMap(action=>{const{marks,extra}=index.selectAll(inputs.get(action.id)!);return [...index.files.filter((_path,position)=>marks[position]),...extra];}).filter(path=>declarable.has(path))));
''')
rep(''' const aliases=snapshot.directories;''',''' const aliases=bazelSourceSnapshotDirectoriesV1(classified.files,directory=>bazelSourceAliasV1(directory));''')
rep('''if(action.kind==="task"||action.kind==="test"){const fingerprint=await profilePhaseV1("bazel.fingerprint",()=>taskFingerprint(root,graph,task,hashed.get(action.id)??[],memo,invocation,phaseControl,action.cache.cache,chosen.sha256+chosen.loaderSha256,exactClosures,resolutions));await writeChanged(join(workspace,task.label+".input"),fingerprint);}''',
    '''if(action.kind==="task"||action.kind==="test"){const head=await profilePhaseV1("bazel.fingerprint",()=>taskFingerprintHeadV1(root,graph,task,memo,invocation,phaseControl,action.cache.cache,chosen.sha256+chosen.loaderSha256,exactClosures,resolutions));await writeChanged(join(workspace,task.label+actions.fingerprints.plan),JSON.stringify({version:1,head,cached:action.cache.cache,files:[...(hashed.get(action.id)??[])].sort()}));}''')
rep('''return {workspace,plan,implementation,''','''if(publish==="capture")await refreshBazelWorkspaceSourcesV1(root,workspace,Object.values(plan.tasks).map(task=>task.label),invocation,pool,request.verbose===true);return {workspace,plan,implementation,''')
open(p,"w",encoding="utf8").write(t);print("ok")
