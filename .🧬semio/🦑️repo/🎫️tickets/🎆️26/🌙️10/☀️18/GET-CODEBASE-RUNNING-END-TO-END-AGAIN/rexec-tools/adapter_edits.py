"""Anchored hermetic-execution edits of the Bazel adapter 📜️script.ts: every anchor must occur exactly once; patch() is idempotent."""
import sys

MARKER = "bazelHermeticDecisionV1"

def patch(text):
    if MARKER in text:
        return text

    def rep(old, new, count=1):
        nonlocal text
        found = text.count(old)
        if found != count:
            sys.exit(f"anchor count {found} != {count}: {old[:90]!r}")
        text = text.replace(old, new)

    rep('import {constants} from "node:fs";', 'import {constants,existsSync} from "node:fs";')
    rep('import {bazelBuildEventsFileV1,recordBazelBuildMetricsV1} from "./⚙️operations/🟦️.ts";',
        'import {bazelBuildEventsFileV1,recordBazelBuildMetricsV1} from "./⚙️operations/🟦️.ts";\n'
        'import {BAZEL_HERMETIC_RUNNER,BAZEL_HERMETIC_STARLARK,bazelBunStorePathV1,bazelExecutionFlagsV1,bazelExecutionPlatformV1,bazelHermeticDecisionV1,bazelHermeticManifestV1,bazelRemoteExecutionV1,linkBazelToolchainsV1,prepareBazelBunV1,renderBazelExecutionBuildV1,type BazelExecutionPlanV1} from "./🏃️execution/🟦️.ts";')
    rep('export interface BazelActionV1{id:string;label:string;kind:"task"|"noop"|"launch";',
        'export interface BazelActionV1{id:string;label:string;kind:"task"|"noop"|"launch"|"hermetic";')

    rep('/** 🔖 Resolves each emitted action to the dependency outputs it consumes; pure ordering tasks vanish behind their producers and unselected ones are never emitted. */\n'
        'export function bazelActionsV1(graph:BuildGraphV1,plan:BazelTaskPlanV1,skipCache:boolean):Map<string,BazelActionV1>{',
        '/** 🔖 Resolves each emitted action to the dependency outputs it consumes; pure ordering tasks vanish behind their producers and unselected ones are never emitted. With `hermetic`, a cacheable task carrying the neutral hermetic declaration becomes a `hermetic` action that a sandbox or a remote executor may run. */\n'
        'export function bazelActionsV1(graph:BuildGraphV1,plan:BazelTaskPlanV1,skipCache:boolean,execution:{hermetic:boolean}={hermetic:false}):Map<string,BazelActionV1>{')
    rep('  const kind=continuous?"launch":pure?"noop":"task",restore=kind!=="noop"&&transitive(task)?closure(task):direct(task);\n'
        '  emitted.set(task.id,{id:task.id,label:task.label,kind,restore:restore.map(row=>row.label),cache:bazelCacheDecisionV1(task,skipCache)});',
        '  const kind=continuous?"launch":pure?"noop":"task",restore=kind!=="noop"&&transitive(task)?closure(task):direct(task),cache=bazelCacheDecisionV1(task,skipCache),hermetic=kind==="task"&&execution.hermetic&&bazelHermeticDecisionV1(task,cache,()=>bazelTaskInputsV1(graph,task,"ignore")).hermetic;\n'
        '  emitted.set(task.id,{id:task.id,label:task.label,kind:hermetic?"hermetic":kind,restore:restore.map(row=>row.label),cache});')

    rep('export function renderBazelBuildV1(input:{actions:readonly (BazelActionV1&{sources?:string})[];groups:ReadonlyMap<string,BazelSourceGroupV1>;bun:string;sourceIdentity:string;extension:string}):string{',
        'export function renderBazelBuildV1(input:{actions:readonly (BazelActionV1&{sources?:string})[];groups:ReadonlyMap<string,BazelSourceGroupV1>;bun:string;sourceIdentity:string;extension:string;execution?:string;remote?:boolean}):string{')
    rep(' const lines=[\'load(":tasks.bzl", "repo_launch", "repo_noop", "repo_task")\',""];\n',
        ' const symbols=["repo_launch","repo_noop","repo_task",...(input.actions.some(action=>action.kind==="hermetic")?["repo_hermetic_task"]:[]),...(input.execution===undefined?[]:["repo_bun_toolchain"])].sort(order),lines=[`load(":tasks.bzl", ${symbols.map(quote).join(", ")})`,""];\n'
        ' if(input.execution!==undefined)lines.push(input.execution);\n')
    rep('  if(action.kind==="noop")attributes.push(["deps",labels(action.restore)]);\n  else{',
        '  if(action.kind==="noop")attributes.push(["deps",labels(action.restore)]);\n'
        '  else if(action.kind==="hermetic"){\n'
        '   attributes.push(["manifest",quote(action.label+".json")]);\n'
        '   if(action.sources)attributes.push(["srcs",list([":"+action.sources])]);\n'
        '   attributes.push(["deps",labels(action.restore)],["runner",quote("hermetic-runner.mjs")],["identity",quote(action.id)]);\n'
        '   if(input.remote)attributes.push(["exec_compatible_with",\'[":exec_pool_remote"]\']);\n'
        '  }else{')
    rep('  lines.push(`${action.kind==="noop"?"repo_noop":action.kind==="launch"?"repo_launch":"repo_task"}(`,',
        '  lines.push(`${action.kind==="noop"?"repo_noop":action.kind==="launch"?"repo_launch":action.kind==="hermetic"?"repo_hermetic_task":"repo_task"}(`,')

    rep('/** 📨️ Carries finite task data to the captured planner without granting authority through JSON. */',
        '/** 🧭️ Resolves the provisioned bun executables the hermetic actions of a plan run under, or nothing when the plan has none; a missing executable fails closed before Bazel starts. */\n'
        'export function bazelExecutionPlanV1(toolsRoot:string,emitted:ReadonlyMap<string,BazelActionV1>,environment:Readonly<Record<string,string|undefined>>,host=bazelExecutionPlatformV1(process.platform,process.arch)):BazelExecutionPlanV1|undefined{\n'
        ' if(![...emitted.values()].some(action=>action.kind==="hermetic"))return undefined;\n'
        ' const remote=bazelRemoteExecutionV1(environment),ids=[...new Set([host.id,...(remote?[remote.platform]:[])])];\n'
        ' return{host:host.id,...(remote?{remote:{platform:remote.platform,properties:remote.properties}}:{}),tools:Object.fromEntries(ids.map(id=>{const path=bazelBunStorePathV1(toolsRoot,id);if(!existsSync(path))throw new Error(`Pinned Bun ${id} is not prepared for hermetic actions`);return [id,path];}))};\n'
        '}\n\n'
        '/** 📨️ Carries finite task data to the captured planner without granting authority through JSON. */')

    rep(' return generateBazelWorkspaceV1(root,graph,request,plan,workspace,invocation,phaseControl,implementation,recheck);\n',
        ' return generateBazelWorkspaceV1(root,graph,request,plan,workspace,invocation,phaseControl,implementation,recheck,undefined,bazelExecutionPlanV1(storage.tools.root,bazelActionsV1(graph,plan,request.skipCache===true,{hermetic:true}),process.env));\n')

    rep('implementation:BazelImplementationV1,recheck:()=>void,closureFetch?:BazelClosureFetchV1):Promise<{workspace:string;plan:BazelTaskPlanV1;implementation:BazelImplementationV1}>{\n const emitted=bazelActionsV1(graph,plan,request.skipCache===true),',
        'implementation:BazelImplementationV1,recheck:()=>void,closureFetch?:BazelClosureFetchV1,execution?:BazelExecutionPlanV1):Promise<{workspace:string;plan:BazelTaskPlanV1;implementation:BazelImplementationV1}>{\n const emitted=bazelActionsV1(graph,plan,request.skipCache===true,{hermetic:true}),hasHermetic=[...emitted.values()].some(action=>action.kind==="hermetic"),',
    )
    rep('declared=[...emitted.values()].filter(action=>action.kind==="task"&&action.cache.cache),',
        'declared=[...emitted.values()].filter(action=>(action.kind==="task"||action.kind==="hermetic")&&action.cache.cache),')
    rep(' await mkdir(workspace,{recursive:true,mode:0o700});await chmod(workspace,0o700);if(dirname(implementation.snapshot)!==workspace)throw new Error("Captured Bazel planner workspace changed");recheck();await writeChanged(join(workspace,"MODULE.bazel"),BAZEL_MODULE);await writeChanged(join(workspace,"tasks.bzl"),STARLARK_RULES);await linkBazelSourcesV1(root,workspace);',
        ' if(hasHermetic&&!execution)throw new Error("Hermetic Bazel actions require an execution plan with provisioned toolchains");\n'
        ' await mkdir(workspace,{recursive:true,mode:0o700});await chmod(workspace,0o700);if(dirname(implementation.snapshot)!==workspace)throw new Error("Captured Bazel planner workspace changed");recheck();await writeChanged(join(workspace,"MODULE.bazel"),BAZEL_MODULE);await writeChanged(join(workspace,"tasks.bzl"),STARLARK_RULES+(hasHermetic?BAZEL_HERMETIC_STARLARK:""));await linkBazelSourcesV1(root,workspace);\n'
        ' if(hasHermetic)await writeChanged(join(workspace,"hermetic-runner.mjs"),BAZEL_HERMETIC_RUNNER);')
    rep('  if(row.source)sources.set(action.id,row.source);hashed.set(action.id,row.hashed);\n',
        '  if(action.kind==="hermetic"&&row.hashed.length)throw new Error(`Hermetic task input cannot be declared as a file: ${action.id}: ${row.hashed[0]}`);\n'
        '  if(row.source)sources.set(action.id,row.source);hashed.set(action.id,row.hashed);\n')
    rep('  if(action.kind==="noop")continue;\n  checkScriptInvocation(invocation);const task=plan.tasks[action.id]!;const metadata=',
        '  if(action.kind==="noop")continue;\n'
        '  if(action.kind==="hermetic"){checkScriptInvocation(invocation);const task=plan.tasks[action.id]!;await writeChanged(join(workspace,task.label+".json"),JSON.stringify(bazelHermeticManifestV1(task,buildExecutionArgumentsV1({...task.definition.execution,forwardAllArgs:true},task.args))));continue;}\n'
        '  checkScriptInvocation(invocation);const task=plan.tasks[action.id]!;const metadata=')
    rep(' recheck();checkScriptInvocation(invocation);await writeChanged(join(workspace,"BUILD.bazel"),renderBazelBuildV1({',
        ' const executionBuild=hasHermetic?(await linkBazelToolchainsV1(workspace,execution!.tools),renderBazelExecutionBuildV1(execution!)):undefined;\n'
        ' recheck();checkScriptInvocation(invocation);await writeChanged(join(workspace,"BUILD.bazel"),renderBazelBuildV1({')
    rep('bun:process.execPath,sourceIdentity:implementation.sha256,extension:process.platform==="win32"?".exe":""}));',
        'bun:process.execPath,sourceIdentity:implementation.sha256,extension:process.platform==="win32"?".exe":"",...(executionBuild===undefined?{}:{execution:executionBuild,remote:execution!.remote!==undefined})}));')

    rep('modules=await prepareBazelModulesV1(root,bazel,invocation,storage,phaseControl);\n',
        'modules=await prepareBazelModulesV1(root,bazel,invocation,storage,phaseControl);\n'
        ' const wanted=bazelActionsV1(graph,planBazelTasksV1(graph,request),request.skipCache===true,{hermetic:true}),hermetic=[...wanted.values()].some(action=>action.kind==="hermetic"),remote=hermetic?bazelRemoteExecutionV1(process.env):undefined;\n'
        ' if(hermetic)for(const id of new Set([bazelExecutionPlatformV1(process.platform,process.arch).id,...(remote?[remote.platform]:[])]))await prepareBazelBunV1(root,storage.tools,id,phaseControl,{offline:original.CARGO_NET_OFFLINE==="true",progress:event=>{if(event.phase!=="downloading")console.log(`[bazel] bun ${id} ${event.phase} ${event.bytes}/${event.total}`);}});\n')
    rep('emitted=bazelActionsV1(graph,plan,request.skipCache===true),outputRoot=',
        'emitted=bazelActionsV1(graph,plan,request.skipCache===true,{hermetic:true}),outputRoot=')
    rep('  await writeChanged(join(workspace,"MODULE.bazel.lock"),modules.lock);options[0]="--lockfile_mode=error";',
        '  if(hermetic){const execution=bazelExecutionFlagsV1({host:{platform:process.platform,arch:process.arch},hermetic,jobs:request.parallel??Math.max(1,Math.min(8,require("node:os").availableParallelism())),environment:process.env});options.push(...execution.command);if(execution.remote)options.push(`--jobs=${execution.jobs}`);}\n'
        '  await writeChanged(join(workspace,"MODULE.bazel.lock"),modules.lock);options[0]="--lockfile_mode=error";')
    rep('for(const action of emitted.values()){if(action.kind==="noop")continue;checkScriptInvocation(invocation);const task=plan.tasks[action.id]!,source=await readFile(',
        'for(const action of emitted.values()){if(action.kind==="noop"||action.kind==="hermetic")continue;checkScriptInvocation(invocation);const task=plan.tasks[action.id]!,source=await readFile(')
    rep('for(const id of plan.selected){const action=emitted.get(id)!;if(action.kind==="task")await restoreBazelSnapshotV1(',
        'for(const id of plan.selected){const action=emitted.get(id)!;if(action.kind==="task"||action.kind==="hermetic")await restoreBazelSnapshotV1(')


    rep('inputs = depset([ctx.file.manifest, ctx.file.fingerprint, ctx.file.runner, ctx.file.source] + snapshots, transitive = [src[DefaultInfo].files for src in ctx.attr.srcs]), outputs = [output], mnemonic = "RepoTask", progress_message = ctx.attr.identity, execution_requirements = _requirements(ctx), use_default_shell_env = True)',
        'inputs = depset([ctx.file.manifest, ctx.file.fingerprint, ctx.file.runner, ctx.file.source] + snapshots, transitive = [src[DefaultInfo].files for src in ctx.attr.srcs]), outputs = [output], mnemonic = "RepoTask", progress_message = ctx.attr.identity, execution_requirements = _requirements(ctx), use_default_shell_env = True, env = {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0"})')
    rep('inputs = [ctx.file.manifest, ctx.file.runner, ctx.file.source], outputs = [launcher], mnemonic = "RepoLauncher", execution_requirements = _requirements(ctx))',
        'inputs = [ctx.file.manifest, ctx.file.runner, ctx.file.source], outputs = [launcher], mnemonic = "RepoLauncher", execution_requirements = _requirements(ctx), env = {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0"})')
    return text
