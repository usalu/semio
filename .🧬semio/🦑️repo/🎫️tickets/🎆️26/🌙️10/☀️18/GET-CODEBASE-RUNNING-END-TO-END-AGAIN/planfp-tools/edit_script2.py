"""🛠️ Anchored, idempotent edits of batch `planfp` round 2 on a copy of 📜️script.ts: the fingerprint head becomes the exported `bazelTaskFingerprintHeadV1` with an explicit host (environment + runtime command capturer), so laws import the real function.
usage: python3 -I edit_script2.py <script.ts>"""
import sys
p=sys.argv[1];t=open(p,encoding="utf8").read()
def rep(old,new):
    global t
    if t.count(new)==1 and new not in old: return
    assert t.count(old)==1,("ANCHOR",t.count(old),old[:120]); t=t.replace(old,new)
rep('import {runBazelRuntimeCommandV1} from "./🌿️environment/🟦️.ts";','import {runBazelRuntimeCommandV1,type BazelCommandCapturerV1} from "./🌿️environment/🟦️.ts";')
rep('''function bazelRuntimeStdoutV1(root:string,runtime:string,original:NodeJS.ProcessEnv,phaseControl:ResourceLeaseControl):Promise<string>{
 const run=()=>runBazelRuntimeCommandV1(root,runtime,original,phaseControl),witness=worldWitnessV1();''','''function bazelRuntimeStdoutV1(root:string,runtime:string,original:NodeJS.ProcessEnv,phaseControl:ResourceLeaseControl,capture?:BazelCommandCapturerV1):Promise<string>{
 const run=()=>runBazelRuntimeCommandV1(root,runtime,original,phaseControl,...(capture?[capture]:[])),witness=worldWitnessV1();''')
rep('''/** 📖️ The head of a task fingerprint, what the planner decides and Bazel cannot read from declared files: host identity, implementation, declared environment and runtime commands (each command runs once per distinct text and build mode, like Nx's runtime hasher); the identities of the paths without a declarable label follow in {@link bazelTaskFingerprintV1}. */
async function taskFingerprintHeadV1(root:string,graph:BuildGraphV1,task:BazelTaskV1,memo:Map<string,Promise<string>>,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,cached:boolean,implementation:string,exactClosures:BazelClosuresV1,resolutions:ReadonlyMap<string,string>):Promise<string>{''',
'''/** 📖️ The head of a task fingerprint, what the planner decides and Bazel cannot read from declared files: host identity, implementation, declared environment and runtime commands (each command runs once per distinct text and build mode, like Nx's runtime hasher); the identities of the paths without a declarable label follow in {@link bazelTaskFingerprintV1}. `host` names the environment the task environment is issued from and the runtime command capturer; both default to the live process. */
export async function bazelTaskFingerprintHeadV1(root:string,graph:BuildGraphV1,task:BazelTaskV1,memo:Map<string,Promise<string>>,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,cached:boolean,implementation:string,exactClosures:BazelClosuresV1|undefined,resolutions:ReadonlyMap<string,string>,host:Readonly<{environment:NodeJS.ProcessEnv;capture?:BazelCommandCapturerV1}>={environment:process.env}):Promise<string>{''')
rep('''bazelTaskBaseEnvironmentV1(root,task,process.env));digest.update(JSON.stringify({platform:process.platform''','''bazelTaskBaseEnvironmentV1(root,task,host.environment));digest.update(JSON.stringify({platform:process.platform''')
rep('''known=bazelGoToolchainDigestV1({...process.env,...original});''','''known=bazelGoToolchainDigestV1({...host.environment,...original});''')
rep('''known=Promise.resolve(bazelNativeToolchainDigestV1(kind,{...process.env,...original},task.root));''','''known=Promise.resolve(bazelNativeToolchainDigestV1(kind,{...host.environment,...original},task.root));''')
rep('''stdout=bazelRuntimeStdoutV1(root,row.runtime,original,phaseControl);''','''stdout=bazelRuntimeStdoutV1(root,row.runtime,original,phaseControl,host.capture);''')
rep('''()=>taskFingerprintHeadV1(root,graph,task,memo,invocation,phaseControl,action.cache.cache,chosen.sha256+chosen.loaderSha256,exactClosures,resolutions)''','''()=>bazelTaskFingerprintHeadV1(root,graph,task,memo,invocation,phaseControl,action.cache.cache,chosen.sha256+chosen.loaderSha256,exactClosures,resolutions)''')
assert "taskFingerprintHeadV1(" not in t.replace("bazelTaskFingerprintHeadV1(","")
open(p,"w",encoding="utf8").write(t);print("ok")
