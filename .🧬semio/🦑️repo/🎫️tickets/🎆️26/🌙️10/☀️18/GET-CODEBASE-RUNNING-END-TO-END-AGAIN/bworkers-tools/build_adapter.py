# 🧩 Builds the staged Bazel adapter: the live adapter plus the worker pool patch, every edit anchored exactly once.
import lib

path = lib.BAZEL + "/" + lib.SCRIPT
live = lib.read(path)
e = lib.Edit(live, "adapter")

e.once('import{captureBuildSourceClosureV1,buildSourceClosureIdentityV1,buildSourceClosureLoaderV1}from"../../📥️source/🔏️closure/🟦️.ts";',
'''import{captureBuildSourceClosureV1,buildSourceClosureIdentityV1,buildSourceClosureLoaderV1}from"../../📥️source/🔏️closure/🟦️.ts";
import buildPolicy from "../../🔣️policy.json" with { type: "json" };
import {readWorkerPolicyV1,planWorkersV1,currentWorkerHostV1,requireWorkerPlanV1,workerActionKindV1,type WorkerPlanV1} from "../../👷️workers/🟦️.ts";
import {installWorkerOutputCaptureV1,readWorkerInputV1,serveWorkerV1} from "../../👷️workers/📨️protocol/🟦️.ts";
import {bazelWorkerMnemonicV1,bazelWorkerSettingsV1} from "./👷️workers/🟦️.ts";
const workerPolicy=readWorkerPolicyV1(buildPolicy.workers);''')

e.once('outputStyle?:"stream"|"stream-without-prefixes"|"static"}\ntype BazelImplementationV1',
       'outputStyle?:"stream"|"stream-without-prefixes"|"static";workers?:boolean}\ntype BazelImplementationV1')

e.once('export interface BazelActionV1{id:string;label:string;kind:"task"|"noop"|"launch";restore:string[];cache:BazelCacheDecisionV1}',
       'export interface BazelActionWorkerV1{kind:"test"|"check"|"generate"|"build"|"script"|"service";mnemonic:string;multiplex:boolean}\nexport interface BazelActionV1{id:string;label:string;kind:"task"|"noop"|"launch";restore:string[];cache:BazelCacheDecisionV1;worker?:BazelActionWorkerV1}')

e.once('emitted.set(task.id,{id:task.id,label:task.label,kind,restore:restore.map(row=>row.label),cache:bazelCacheDecisionV1(task,skipCache)});',
       'const worker=kind==="task"?bazelActionWorkerV1(task):undefined;\n  emitted.set(task.id,{id:task.id,label:task.label,kind,restore:restore.map(row=>row.label),cache:bazelCacheDecisionV1(task,skipCache),...(worker?{worker}:{})});')

e.once('/** 🔖 Resolves each emitted action to the dependency outputs it consumes;',
'''/** 👷️ Classifies a task into its neutral action kind and names the Bazel mnemonic and multiplexing its worker pool is keyed by; only the authored policy decides, never the host. */
export function bazelActionWorkerV1(task:BazelTaskV1):BazelActionWorkerV1{
 const kind=workerActionKindV1(workerPolicy,task.target,task.definition.continuous===true);
 return {kind,mnemonic:bazelWorkerMnemonicV1(kind),multiplex:workerPolicy.kinds[kind].multiplex};
}

/** 🔖 Resolves each emitted action to the dependency outputs it consumes;''')

# Starlark rules
e.once('''def _requirements(ctx):
    requirements = {"no-remote-exec": "1"}
''','''def _requirements(ctx, worker = False):
    requirements = {"no-remote-exec": "1"}
    if worker:
        requirements["supports-workers"] = "1"
        requirements["requires-worker-protocol"] = "json"
        requirements["supports-worker-cancellation"] = "1"
        if ctx.attr.multiplex:
            requirements["supports-multiplex-workers"] = "1"
''')
e.once('''    args = [ctx.file.runner.path, ctx.file.source.path, ctx.attr.source_identity, "birth-action", ctx.file.manifest.path, output.path] + [snapshot.path for snapshot in snapshots]
    ctx.actions.run(executable = ctx.attr.bun, arguments = args, inputs = depset([ctx.file.manifest, ctx.file.fingerprint, ctx.file.runner, ctx.file.source] + snapshots, transitive = [src[DefaultInfo].files for src in ctx.attr.srcs]), outputs = [output], mnemonic = "RepoTask", progress_message = ctx.attr.identity, execution_requirements = _requirements(ctx), use_default_shell_env = True)''',
'''    request = ctx.actions.args()
    request.use_param_file("@%s", use_always = True)
    request.set_param_file_format("multiline")
    request.add(ctx.file.manifest.path)
    request.add(output.path)
    request.add_all([snapshot.path for snapshot in snapshots])
    ctx.actions.run(executable = ctx.attr.bun, arguments = [ctx.file.runner.path, ctx.file.source.path, ctx.attr.source_identity, "work-action", request], inputs = depset([ctx.file.manifest, ctx.file.fingerprint] + snapshots, transitive = [src[DefaultInfo].files for src in ctx.attr.srcs]), tools = [ctx.file.runner, ctx.file.source], outputs = [output], mnemonic = ctx.attr.mnemonic, progress_message = ctx.attr.identity, execution_requirements = _requirements(ctx, True), use_default_shell_env = True)''')
e.once('"identity": attr.string(mandatory = True), "cache": attr.bool(default = False), "uncached_reason": attr.string(), "host_state": attr.string()})\n\ndef _noop_impl',
       '"identity": attr.string(mandatory = True), "mnemonic": attr.string(mandatory = True), "multiplex": attr.bool(default = False), "cache": attr.bool(default = False), "uncached_reason": attr.string(), "host_state": attr.string()})\n\ndef _noop_impl')

# BUILD rendering
e.once('if(launch)attributes.push(["extension",quote(input.extension)]);else attributes.push(["identity",quote(action.id)]);',
       'if(launch)attributes.push(["extension",quote(input.extension)]);else attributes.push(["identity",quote(action.id)],["mnemonic",quote(action.worker!.mnemonic)],["multiplex",action.worker!.multiplex?"True":"False"]);')

# options: local jobs and worker flags from the neutral plan
e.once('const {workspace,plan,implementation}=await publishBazelGraphV1(root,graph,request,invocation,storage,phaseControl),emitted=bazelActionsV1(graph,plan,request.skipCache===true),',
       'const {workspace,plan,implementation}=await publishBazelGraphV1(root,graph,request,invocation,storage,phaseControl),emitted=bazelActionsV1(graph,plan,request.skipCache===true),workers=planWorkersV1(workerPolicy,currentWorkerHostV1(),{...(request.parallel?{jobs:request.parallel}:{}),...(request.workers===undefined?{}:{workers:request.workers})}),settings=bazelWorkerSettingsV1(workers),')
e.once('"--incompatible_strict_action_env",...cache.command,`--jobs=${request.parallel??Math.max(1,Math.min(8,require("node:os").availableParallelism()))}`];',
       '"--incompatible_strict_action_env",...cache.command,...settings.flags];')
e.once('const environment={...storage.products.environment,SEMIO_BAZEL_SELECTION_CHAIN:[...chain,namespace].join(":"),',
       'const environment={...storage.products.environment,SEMIO_BAZEL_SELECTION_CHAIN:[...chain,namespace].join(":"),SEMIO_BAZEL_WORKER_PLAN:JSON.stringify(workers),')
e.once('...options.filter(option=>!option.startsWith("--jobs="))],workspace,{signal,control:phaseControl,env:environment})).trim()',
       '...options.filter(option=>!settings.flags.includes(option))],workspace,{signal,control:phaseControl,env:environment})).trim()')

# stdin isolation of the leaf command inside a worker
e.once('return runOwnedCommand(process.execPath,["exec",source],cwd,task.id,compilerAdmissionBudgetV1(actual),{env:environment,signal});',
       'return runOwnedCommand(process.execPath,["exec",source],cwd,task.id,compilerAdmissionBudgetV1(actual),{env:environment,signal,...(workerChildInputV1===undefined?{}:{input:workerChildInputV1})});')
e.once('/** 🚀️ Executes the direct owner command with the original caller\'s native capabilities. */',
       '/** 👷️ Gives the commands a persistent worker starts an empty standard input, because the worker\'s own standard input carries the protocol. */\nlet workerChildInputV1:string|undefined;\n\n/** 🚀️ Executes the direct owner command with the original caller\'s native capabilities. */')

# cancellation of the action body by a work request
e.once('async function withBazelCancellation<T>(environment:NodeJS.ProcessEnv,invocation:ScriptInvocation,run:(owned:ScriptInvocation)=>Promise<T>):Promise<T>{',
       'async function withBazelCancellation<T>(environment:NodeJS.ProcessEnv,invocation:ScriptInvocation,run:(owned:ScriptInvocation)=>Promise<T>,cancel?:AbortSignal):Promise<T>{')
e.once('invocation.control.signal.addEventListener("abort",interrupt,{once:true});if(invocation.control.signal.aborted)interrupt();',
       'invocation.control.signal.addEventListener("abort",interrupt,{once:true});if(invocation.control.signal.aborted)interrupt();cancel?.addEventListener("abort",()=>cancelled.abort(cancel.reason??Error("Bazel work request cancelled")),{once:true});if(cancel?.aborted)cancelled.abort(cancel.reason);')
e.once('"shared"}]},run);});});', '"shared"}]},run);},cancel);});')

# birth: optional in-process action body for persistent workers
e.once('async function launchBazelActionBirthV1(kind:string,file:string,parameters:readonly string[],implementation:BazelImplementationV1):Promise<void>{',
       'async function launchBazelActionBirthV1(kind:string,file:string,parameters:readonly string[],implementation:BazelImplementationV1,worker?:AbortSignal):Promise<void>{')
e.once('if(typeof expected!=="string")throw new Error("Bazel birth wire expectation required");',
       'if(typeof expected!=="string")throw new Error("Bazel birth wire expectation required");const inProcess=worker!==undefined&&bazelWorkerInProcessV1(manifest,environment);')
e.once(' let primaryFailed=false;try{const wire=await joined;',
       ' worker?.addEventListener("abort",()=>fail(worker.reason??Error("Bazel work request cancelled")),{once:true});if(worker?.aborted)fail(worker.reason??Error("Bazel work request cancelled"));\n let primaryFailed=false;try{const wire=await joined;')
e.once('await runOwnedCommand(process.execPath,[implementation.runner,implementation.snapshot,implementation.sha256,kind,manifestPath,...birth.parameters],process.cwd(),"bazel:owned-action-child",scriptInvocationBudget(invocation,0),{env:scriptInvocationEnvironment(invocation,{...environment,SEMIO_BAZEL_MANIFEST_SHA256:birth.manifestSha256}),signal:invocation.control.signal,onProgress:()=>advanceScriptInvocation(invocation,"bazel:owned-action-child","running")});',
       'const childEnvironment=scriptInvocationEnvironment(invocation,{...environment,SEMIO_BAZEL_MANIFEST_SHA256:birth.manifestSha256});if(inProcess)await inProcessLockV1(()=>runBazelActionInProcessV1([kind,manifestPath,...birth.parameters],implementation,childEnvironment,worker));else await runOwnedCommand(process.execPath,[implementation.runner,implementation.snapshot,implementation.sha256,kind,manifestPath,...birth.parameters],process.cwd(),"bazel:owned-action-child",scriptInvocationBudget(invocation,0),{env:childEnvironment,signal:invocation.control.signal,onProgress:()=>advanceScriptInvocation(invocation,"bazel:owned-action-child","running")});')

# runner entry points; the action body becomes its own function
head = e.text
marker = 'if(argv[0]?.startsWith("birth-")){await launchBazelActionBirthV1(argv[0].slice(6),argv[1]!,argv.slice(2),implementation);}else{\n'
i = head.index(marker)
tail_marker = '\n}\n}\n\nif(import.meta.main)throw new Error("Bazel task entry requires its captured implementation loader");'
j = head.index(tail_marker)
body = head[i + len(marker):j]
new_entry = 'if(argv[0]==="work-action"){await serveBazelWorkV1(argv.slice(1),implementation,proof.recheck);return;}if(argv[0]?.startsWith("birth-")){await launchBazelActionBirthV1(argv[0].slice(6),argv[1]!,argv.slice(2),implementation);}else await runBazelActionV1(argv,implementation);\n}\n'
worker_code = '''
/** 🏃️ Runs one captured action, launcher or launch under the original birth the caller holds. */
async function runBazelActionV1(argv:string[],implementation:BazelImplementationV1,cancel?:AbortSignal):Promise<void>{
''' + body + '''
}

/** 👷️ Runs the action body in this process under the environment its own child process would have received. */
async function runBazelActionInProcessV1(argv:string[],implementation:BazelImplementationV1,environment:NodeJS.ProcessEnv,cancel?:AbortSignal):Promise<void>{
 const previous=process.env;process.env={...environment};
 try{await runBazelActionV1(argv,implementation,cancel);}finally{process.env=previous;}
}

let workerPlanV1:WorkerPlanV1|undefined,inProcessTailV1:Promise<unknown>=Promise.resolve();

/** 👷️ Runs in-process action bodies one at a time, because they borrow the process environment; actions that run in child processes stay concurrent. */
function inProcessLockV1<T>(run:()=>Promise<T>):Promise<T>{
 const next=inProcessTailV1.then(run,run);inProcessTailV1=next.catch(()=>{});return next;
}

/** 👷️ Tells whether the plan of this invocation runs the task's action kind in the worker process. */
function bazelWorkerInProcessV1(manifest:BazelTaskManifestV1,environment:NodeJS.ProcessEnv):boolean{
 const encoded=environment.SEMIO_BAZEL_WORKER_PLAN;if(!encoded)return false;
 const plan=JSON.parse(encoded);requireWorkerPlanV1(plan);workerPlanV1=plan;
 return plan.kinds[workerActionKindV1(workerPolicy,manifest.task.target,manifest.task.definition.continuous===true)].inProcess;
}

/** 👷️ Handles one work request: manifest, output and dependency snapshots name one action birth. */
async function runBazelWorkRequestV1(parameters:readonly string[],implementation:BazelImplementationV1,worker?:AbortSignal):Promise<void>{
 const [manifest,output,...snapshots]=parameters;
 if(!manifest||!output)throw new Error("Bazel work request requires a manifest and an output");
 await launchBazelActionBirthV1("action",manifest,[output,...snapshots],implementation,worker);
}

/** 👷️ Serves a persistent Bazel worker over the JSON worker protocol, or runs the single request of one flagfile like any other action. Everything the actions print is returned in their responses, so standard output carries nothing but responses. */
async function serveBazelWorkV1(args:readonly string[],implementation:BazelImplementationV1,recheck?:()=>void):Promise<void>{
 if(!args.includes("--persistent_worker")){
  const flagfile=args[0];if(args.length!==1||!flagfile?.startsWith("@"))throw new Error("Expected Bazel work-action --persistent_worker or one @flagfile");
  await runBazelWorkRequestV1((await readFile(flagfile.slice(1),"utf8")).split("\\n").filter(Boolean),implementation);return;
 }
 const capture=installWorkerOutputCaptureV1();workerChildInputV1="";
 try{
  await serveWorkerV1({input:readWorkerInputV1(),write:capture.writeProtocol,concurrency:1024,
   handler:async(request,{signal})=>{const result=await capture.run(async()=>{try{recheck?.();await runBazelWorkRequestV1(request.arguments,implementation,signal);return 0;}catch(error){process.stderr.write(String((error as Error)?.stack??error)+"\\n");return 1;}});return {exitCode:result.value,output:result.output};},
   afterRequest:served=>{const plan=workerPlanV1;if(plan&&(served%plan.resetAfterRequests===0||process.memoryUsage.rss()>plan.memory.perWorkerMegabytes*1048576))Bun.gc(true);}
  });
 }finally{capture.restore();workerChildInputV1=undefined;}
}
'''
head = head[:i] + new_entry + worker_code + head[j + len('\n}\n}\n'):]
lib.write_staged("script.ts", head)
print("staged adapter", len(head), "bytes (live", len(live), ")")
