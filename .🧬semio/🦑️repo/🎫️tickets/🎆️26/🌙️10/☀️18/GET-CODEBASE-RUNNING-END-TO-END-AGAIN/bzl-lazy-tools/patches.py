# Anchored edits of the live files; every anchor must occur exactly once or the build of the staged copy refuses.
import json, re

def once(text, old, new, what=""):
    n = text.count(old)
    if n != 1:
        raise SystemExit("anchor count %d for %s: %r" % (n, what, old[:90]))
    return text.replace(old, new)

TEST_REASON = "The test process runs in the real checkout like a task: its working directory, the declared source-tree reads, the loopback control socket and the out-of-tree runtime channel are host state that a Bazel sandbox hides or blocks; the result is cached as a Bazel test result and as a remote cache entry."

TEST_RULE = '''
def _quote(value):
    if "'" in value:
        fail("A quote cannot be passed to a test launcher: " + value)
    return "'" + value + "'"

def _test_impl(ctx):
    name = ctx.label.name
    launcher = ctx.actions.declare_file(name + ".test")
    snapshots = [dep[DefaultInfo].files.to_list()[0] for dep in ctx.attr.deps]
    own = '"$bin/' + name + '.snapshot"'
    command = [_quote(ctx.attr.bun), _quote(ctx.file.runner.short_path), _quote(ctx.file.source.short_path), _quote(ctx.attr.source_identity), "birth-action", _quote(ctx.file.manifest.short_path), own] + ['"$bin/' + snapshot.basename + '"' for snapshot in snapshots]
    lines = ["#!/bin/sh", 'bin=$(dirname "$TEST_SRCDIR")', "rm -rf " + own + " && mkdir -p " + own + " || exit 1", 'cd "$TEST_SRCDIR/$TEST_WORKSPACE" || exit 1', "exec " + " ".join(command)]
    ctx.actions.write(launcher, "\\\\n".join(lines) + "\\\\n", is_executable = True)
    runfiles = ctx.runfiles(files = [ctx.file.manifest, ctx.file.fingerprint, ctx.file.runner, ctx.file.source] + snapshots, transitive_files = depset(transitive = [src[DefaultInfo].files for src in ctx.attr.srcs]))
    return [DefaultInfo(executable = launcher, runfiles = runfiles), testing.ExecutionInfo(_requirements(ctx))]

repo_test = rule(implementation = _test_impl, test = True, attrs = {"manifest": attr.label(allow_single_file = True), "fingerprint": attr.label(allow_single_file = True), "srcs": attr.label_list(allow_files = True), "deps": attr.label_list(), "runner": attr.label(allow_single_file = True), "source": attr.label(allow_single_file = True), "source_identity": attr.string(mandatory = True), "bun": attr.string(mandatory = True), "identity": attr.string(mandatory = True), "cache": attr.bool(default = False), "uncached_reason": attr.string(), "host_state": attr.string()})
'''

def adapter(text):
    t = text
    t = once(t, 'import {bazelBuildEventsFileV1,recordBazelBuildMetricsV1} from "./⚙️operations/🟦️.ts";',
        'import {bazelBuildEventsFileV1,recordBazelBuildMetricsV1} from "./⚙️operations/🟦️.ts";\nimport {bazelCommandPlanV1,bazelTestActionV1,bazelTestTimeoutV1,bazelTestWithinLimitV1} from "./🧭️commands/🟦️.ts";\nimport {bazelAffectedBuildV1,bazelAffectedExpressionV1,bazelAffectedProjectsFromOutputV1,bazelAffectedWorkspaceV1} from "./🎯️affected/🟦️.ts";', "imports")
    t = once(t, 'UncachedReport:uncachedReportSchema}=componentSchema.definitions;', 'UncachedReport:uncachedReportSchema,Kinds:kindsSchema}=componentSchema.definitions;', "kinds schema")
    t = once(t, 'export function bazelActionsV1(graph:BuildGraphV1,plan:BazelTaskPlanV1,skipCache:boolean):Map<string,BazelActionV1>{', 'export function bazelActionsV1(graph:BuildGraphV1,plan:BazelTaskPlanV1,skipCache:boolean,tests?:ReadonlySet<string>):Map<string,BazelActionV1>{', "actions signature")
    t = once(t, 'kind:"task"|"noop"|"launch";restore:string[]', 'kind:"task"|"test"|"noop"|"launch";restore:string[]', "kind union")
    t = once(t, 'producers=new Map<string,BazelTaskV1[]>(),emitted=new Map<string,BazelActionV1>();',
        'producers=new Map<string,BazelTaskV1[]>(),emitted=new Map<string,BazelActionV1>(),dependents=new Map<string,number>();\n for(const task of Object.values(plan.tasks))for(const id of task.dependencies)dependents.set(id,(dependents.get(id)??0)+1);', "dependents")
    t = once(t, 'const kind=continuous?"launch":pure?"noop":"task",restore=kind!=="noop"&&transitive(task)?closure(task):direct(task);\n  emitted.set(task.id,{id:task.id,label:task.label,kind,restore:restore.map(row=>row.label),cache:bazelCacheDecisionV1(task,skipCache)});',
        'const base=continuous?"launch":pure?"noop":"task",restore=base!=="noop"&&transitive(task)?closure(task):direct(task),cache=bazelCacheDecisionV1(task,skipCache),kind=base==="task"&&bazelTestActionV1({target:task.target,cache:cache.cache,outputs:task.definition.outputs??null,dependents:dependents.get(task.id)??0})&&(tests===undefined||tests.has(task.id))?"test":base;\n  emitted.set(task.id,{id:task.id,label:task.label,kind,restore:restore.map(row=>row.label),cache});', "kinds")
    t = once(t, "\ndef _noop_impl(ctx):", TEST_RULE + "\ndef _noop_impl(ctx):", "rules")
    t = once(t, "const lines=['load(\":tasks.bzl\", \"repo_launch\", \"repo_noop\", \"repo_task\")',\"\"];", "const lines=['load(\":tasks.bzl\", \"repo_launch\", \"repo_noop\", \"repo_task\", \"repo_test\")',\"\"];", "load")
    t = once(t, 'reason=launch?actions.kinds.launcher.reason:actions.kinds.task.reason;', 'reason=launch?actions.kinds.launcher.reason:action.kind==="test"?actions.kinds.test.reason:actions.kinds.task.reason;', "reason")
    t = once(t, 'const attributes:[string,string][]=[["name",quote(action.label)]];', 'const attributes:[string,string][]=[["name",quote(action.label)]];\n  if(action.kind==="test")attributes.push(["timeout",quote(bazelTestTimeoutV1)]);', "timeout")
    t = once(t, '${action.kind==="noop"?"repo_noop":action.kind==="launch"?"repo_launch":"repo_task"}(', '${action.kind==="noop"?"repo_noop":action.kind==="launch"?"repo_launch":action.kind==="test"?"repo_test":"repo_task"}(', "rule name")
    t = once(t, 'const emitted=bazelActionsV1(graph,plan,request.skipCache===true),exactClosures=await loadBazelClosuresV1(', 'const candidates=bazelActionsV1(graph,plan,request.skipCache===true),exactClosures=await loadBazelClosuresV1(', "candidates")
    t = once(t, 'declared=[...emitted.values()].filter(action=>action.kind==="task"&&action.cache.cache)', 'declared=[...candidates.values()].filter(action=>(action.kind==="task"||action.kind==="test")&&action.cache.cache)', "declared")
    t = once(t, 'sources=new Map<string,string>(),hashed=new Map<string,string[]>();', 'sources=new Map<string,string>(),hashed=new Map<string,string[]>();\n const tests=new Set<string>();\n for(const action of candidates.values())if(action.kind==="test"){const{marks,extra}=index.selectAll(inputs.get(action.id)!);let count=extra.length;for(const mark of marks)count+=mark;if(bazelTestWithinLimitV1(count))tests.add(action.id);}\n const emitted=bazelActionsV1(graph,plan,request.skipCache===true,tests);await writeChanged(join(workspace,"kinds.json"),JSON.stringify({version:1,tests:[...tests].sort()}));', "tests")
    t = once(t, 'emitted=bazelActionsV1(graph,plan,request.skipCache===true),outputRoot=join(', 'emitted=bazelActionsV1(graph,plan,request.skipCache===true,await readBazelTestsV1(workspace)),outputRoot=join(', "runner tests")
    t = once(t, 'if(action.kind==="task"){const fingerprint=await taskFingerprint(', 'if(action.kind==="task"||action.kind==="test"){const fingerprint=await taskFingerprint(', "fingerprint")
    t = once(t, 'outputRoot=join(storage.paths.outputs,hash(JSON.stringify({version:BAZEL_VERSION,platform:process.platform,arch:process.arch,idle:policy.serverIdleSeconds,jvm:"-Xmx2048m"})).slice(0,16)),cache=bazelCacheFlagsV1(storage.paths,workspace,{environment:process.env,skipCache:request.skipCache,skipRemoteCache:request.skipRemoteCache}),startup=[`--output_user_root=${outputRoot}`,`--max_idle_secs=${policy.serverIdleSeconds}`,`--install_base=${join(storage.paths.install,BAZEL_VERSION,process.platform+"-"+process.arch)}`,"--host_jvm_args=-Xmx2048m","--nosystem_rc","--nohome_rc",`--bazelrc=${join(root,".bazelrc")}`,...cache.startup],',
        'cache=bazelCacheFlagsV1(storage.paths,workspace,{environment:process.env,skipCache:request.skipCache,skipRemoteCache:request.skipRemoteCache}),startup=bazelStartupOptionsV1(root,storage,cache),', "startup")
    t = once(t, '...cache.command,`--jobs=${request.parallel??Math.max(1,Math.min(8,require("node:os").availableParallelism()))}`];', '...cache.command,`--jobs=${jobs}`];', "jobs option")
    t = once(t, 'const {workspace,plan,implementation}=await publishBazelGraphV1(root,graph,request,invocation,storage,phaseControl),', 'const jobs=request.parallel??Math.max(1,Math.min(8,require("node:os").availableParallelism())),{workspace,plan,implementation}=await publishBazelGraphV1(root,graph,request,invocation,storage,phaseControl),', "jobs const")
    t = once(t, 'building=runOwnedCommand(bazel,[...startup,"build",...options,`--build_event_json_file=${bazelBuildEventsFileV1(storage.paths,workspace)}`,...plan.selected.map(id=>"//:"+plan.tasks[id]!.label)],workspace,"bazel:build",compilerAdmissionBudgetV1(phaseControl),{...rendering,signal:control.commandSignal,env:environment}).finally(()=>recordBazelBuildMetricsV1(root,storage.paths,workspace,phaseControl));\n   await Promise.race([building,failed]);',
        'const batch=bazelCommandPlanV1({mode:"execute",base:options.filter(option=>!option.startsWith("--jobs=")&&option!=="--keep_going"&&option!=="--subcommands"),jobs,bail:request.bail===true,verbose:request.verbose===true,skipCache:request.skipCache===true,events:bazelBuildEventsFileV1(storage.paths,workspace),tasks:plan.selected.map(id=>({label:"//:"+plan.tasks[id]!.label,kind:emitted.get(id)!.kind}))}).steps[0];\n   if(batch){building=runOwnedCommand(bazel,[...startup,batch.verb,...batch.options,...batch.targets],workspace,"bazel:"+batch.verb,compilerAdmissionBudgetV1(phaseControl),{...rendering,signal:control.commandSignal,env:environment}).finally(()=>recordBazelBuildMetricsV1(root,storage.paths,workspace,phaseControl));await Promise.race([building,failed]);}', "batch")
    t = once(t, '/** 🧱️ Runs every generated Bazel writer and original child under retained fixed backing. */',
        '''/** 🚀 Names the startup options of one Bazel server: its output user root, idle bound, shared install base, heap, rc isolation and the cache contract's own output base. */
function bazelStartupOptionsV1(root:string,storage:BazelStorageV1,cache:ReturnType<typeof bazelCacheFlagsV1>):string[]{
 const outputRoot=join(storage.paths.outputs,hash(JSON.stringify({version:BAZEL_VERSION,platform:process.platform,arch:process.arch,idle:policy.serverIdleSeconds,jvm:"-Xmx2048m"})).slice(0,16));
 return [`--output_user_root=${outputRoot}`,`--max_idle_secs=${policy.serverIdleSeconds}`,`--install_base=${join(storage.paths.install,BAZEL_VERSION,process.platform+"-"+process.arch)}`,"--host_jvm_args=-Xmx2048m","--nosystem_rc","--nohome_rc",`--bazelrc=${join(root,".bazelrc")}`,...cache.startup];
}

/** 🎯️ Answers `affected` with Bazel itself: one generated package of project and changed-file nodes, one `rdeps` query over it, served by its own persistent server; the touching files come from the shared neutral rule. */
export async function bazelAffectedProjectsV1(root:string,graph:BuildGraphV1,touches:ReadonlyMap<string,readonly string[]>,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<string[]>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);
 if(!touches.size)return [];
 return withBazelStorageV1(root,invocation,phaseControl,async(storage,actualPhase)=>{
  await requireBazelStorageV1(root,storage);checkResourceLeaseControl(actualPhase);checkScriptInvocation(invocation);
  const fixed=bazelAffectedWorkspaceV1(),original=issueBuildTaskEnvironmentV1(root,invocation,{kind:"noop"},[],process.env),bazel=await prepareBazel(root,storage.tools,actualPhase,event=>{if(event.phase!=="downloading")console.log(`[bazel] ${event.phase} ${event.bytes}/${event.total}`);},original.CARGO_NET_OFFLINE==="true"),workspace=join(storage.paths.workspace,fixed.directory);
  return withResourceLeases({directory:join(root,".cache/b/l/m/data"),signal:actualPhase.signal,control:actualPhase,resources:[{resource:"bazel-workspace-"+fixed.directory,mode:"exclusive"}]},async()=>{
   await mkdir(workspace,{recursive:true,mode:0o700});await writeChanged(join(workspace,"MODULE.bazel"),fixed.module);await writeChanged(join(workspace,"BUILD.bazel"),bazelAffectedBuildV1(graph,touches));
   const startup=bazelStartupOptionsV1(root,storage,bazelCacheFlagsV1(storage.paths,workspace,{environment:process.env})),environment={...storage.products.environment,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})};
   await advanceScriptInvocation(invocation,"bazel:affected","running");
   const output=await captureOwnedCommandV1(bazel,[...startup,"query",...fixed.options,bazelAffectedExpressionV1(touches)],workspace,{signal:actualPhase.signal,control:actualPhase,env:environment});
   return bazelAffectedProjectsFromOutputV1(graph,output);
  });
 });
}

/** 🧪️ Reads which planned tasks the workspace generator emitted as Bazel tests; a workspace without the record has none. */
async function readBazelTestsV1(workspace:string):Promise<ReadonlySet<string>>{
 try{const value=JSON.parse(await readFile(join(workspace,"kinds.json"),"utf8"));if(validateJsonSchemaSubset(kindsSchema,value).length)throw new Error("Closed Bazel kinds record required");return new Set<string>(value.tests);}
 catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return new Set<string>();throw error;}
}

/** 📜 Hands the Starlark rules of a generated workspace to tests and tools that render or inspect them. */
export function bazelStarlarkRulesV1():string{return STARLARK_RULES;}

/** 🧱️ Runs every generated Bazel writer and original child under retained fixed backing. */''', "helpers")
    return t

def schema_adapter(text):
    d = json.loads(text)
    kinds = d["definitions"]["Actions"]["const"]["kinds"]
    kinds["test"] = {"sandbox": False, "reason": TEST_REASON}
    policy_kinds = d["definitions"]["ActionPolicy"]["properties"]["kinds"]
    policy_kinds["required"] = ["task", "test", "launcher", "noop"]
    props = policy_kinds["properties"]
    ordered = {"task": props["task"], "test": json.loads(json.dumps(props["task"])), "launcher": props["launcher"], "noop": props["noop"]}
    policy_kinds["properties"] = ordered
    d["definitions"]["Kinds"] = {"description": "Which planned tasks of a generated workspace run as Bazel tests; the generator records it because it needs the declared file counts that the runner never sees.", "type": "object", "additionalProperties": False, "required": ["version", "tests"], "properties": {"version": {"const": 1}, "tests": {"type": "array", "uniqueItems": True, "items": {"type": "string", "minLength": 1}}}}
    return json.dumps(d, indent=2, ensure_ascii=False) + "\n"

SELECTION_OLD_START = "/** 🧭️ Maps changed files to the projects Nx's affected locators touch"
SELECTION_OLD_END = "  return [...affected].sort();\n}\n"

def selection(text):
    t = text
    i = t.index(SELECTION_OLD_START)
    j = t.index(SELECTION_OLD_END, i) + len(SELECTION_OLD_END)
    old = t[i:j]
    new = '''/** 🧭️ Maps changed files to the projects they touch directly (the deepest owning root, projects whose target inputs name a changed `{workspaceRoot}/…` file with named inputs expanded, and every project when `nx.json` changes) and keeps one or more touching files per project; the one rule every build tool shares before it closes over dependents. */
export function affectedTouchesV1(graph:BuildGraphV1,files:readonly string[]):Map<string,string[]> {
  const names=Object.keys(graph.projects),owners=[...names].sort((a,b)=>graph.projects[b]!.root.length-graph.projects[a]!.root.length||(a<b?-1:1)),touches=new Map<string,string[]>(),prefix="{workspaceRoot}/";
  const touch=(name:string,file:string)=>{const row=touches.get(name);if(!row)touches.set(name,[file]);else if(!row.includes(file))row.push(file);};
  if(files.includes("nx.json"))for(const name of names)touch(name,"nx.json");
  else if(files.length){
    for(const file of files){const owner=owners.find(name=>graph.projects[name]!.root==="."||file.startsWith(graph.projects[name]!.root+"/"));if(owner)touch(owner,file);}
    const extract=(inputs:readonly unknown[],named:Record<string,readonly unknown[]>,seen:ReadonlySet<string>):string[]=>inputs.flatMap(input=>typeof input==="string"?Object.hasOwn(named,input)?seen.has(input)?[]:extract(named[input]!,named,new Set(seen).add(input)):input.startsWith(prefix)?[input.slice(prefix.length)]:[]:typeof input==="object"&&input!==null&&typeof (input as {fileset?:unknown}).fileset==="string"&&(input as {fileset:string}).fileset.startsWith(prefix)?[(input as {fileset:string}).fileset.slice(prefix.length)]:[]),matchers=new Map<string,(file:string)=>boolean>();
    const matcher=(pattern:string)=>{let known=matchers.get(pattern);if(!known){if(/[?*+@!]\\(/u.test(pattern))known=()=>true;else{const glob=new Bun.Glob(pattern);known=(file:string)=>glob.match(file);}matchers.set(pattern,known);}return known;};
    for(const name of names){
      if(touches.has(name))continue;
      const named={...graph.namedInputs,...graph.projects[name]!.namedInputs} as Record<string,readonly unknown[]>,patterns=new Set(Object.values(graph.projects[name]!.targets).flatMap(target=>target.inputs?extract(target.inputs,named,new Set()):[]));
      const hit=files.find(file=>[...patterns].some(pattern=>matcher(pattern)(file)));
      if(hit!==undefined)touch(name,hit);
    }
  }
  return touches;
}

/** 🧭️ Closes the projects touched by changed files over their dependents, as Nx's affected does; the neutral answer every build tool must reproduce. */
export function affectedBuildProjectsV1(graph:BuildGraphV1,files:readonly string[]):string[] {
  const affected=new Set(affectedTouchesV1(graph,files).keys());
  let changed=true;while(changed){changed=false;for(const edge of graph.dependencies)if(affected.has(edge.target)&&!affected.has(edge.source)&&graph.projects[edge.source]){affected.add(edge.source);changed=true;}}
  return [...affected].sort();
}
'''
    t = t[:i] + new + t[j:]
    t = once(t, "export async function selectBuildTasksV1(root:string,graph:BuildGraphV1,args:readonly string[],invocation:ScriptInvocation):Promise<",
        "export async function selectBuildTasksV1(root:string,graph:BuildGraphV1,args:readonly string[],invocation:ScriptInvocation,affectedProjects:(files:readonly string[])=>Promise<readonly string[]>|readonly string[]=files=>affectedBuildProjectsV1(graph,files)):Promise<", "select signature")
    t = once(t, "const affected=new Set(affectedBuildProjectsV1(graph,files));", "const affected=new Set(enabled([\"--all\"])?[]:await affectedProjects(files));", "affected call")
    return t

def neutral(text):
    t = text
    t = once(t, '    const request=await selection.selectBuildTasksV1(root,graph,resolved.args,invocation);\n    process.env.SEMIO_BUILD_REUSE_GRAPH="true";\n    const adapter=await import("./🔧️tools/🏗️bazel/📜️script.ts");',
        '    process.env.SEMIO_BUILD_REUSE_GRAPH="true";\n    const adapter=await import("./🔧️tools/🏗️bazel/📜️script.ts");\n    const head=resolved.args.slice(0,resolved.args.indexOf("--")<0?undefined:resolved.args.indexOf("--")),dryRun=head.some(argument=>argument==="--dry-run"||argument==="--dry-run=true"||argument==="--graph"||argument.startsWith("--graph="));\n    const request=await selection.selectBuildTasksV1(root,graph,resolved.args,invocation,files=>dryRun?selection.affectedBuildProjectsV1(graph,files):adapter.bazelAffectedProjectsV1(root,graph,selection.affectedTouchesV1(graph,files),invocation,artifactPhase));', "neutral request")
    return t

def actions_fixture(text):
    d = json.loads(text)
    d["expectedActions"]["app:test"]["kind"] = "test"
    b = d["build"]
    b[0] = 'load(":tasks.bzl", "repo_launch", "repo_noop", "repo_task", "repo_test")'
    i = b.index('    name = "t_71eb00441669d8d8",') - 1
    assert b[i] == "repo_task("
    b[i] = "repo_test("
    b.insert(i + 2, '    timeout = "eternal",')
    j = next(k for k in range(i, len(b)) if b[k].startswith("    host_state = "))
    b[j] = "    host_state = " + json.dumps(TEST_REASON, ensure_ascii=False) + ","
    return json.dumps(d, indent=2, ensure_ascii=False) + "\n"

def actions_schema(text):
    return once(text, '"enum": [\n              "task",\n              "noop",\n              "launch"\n            ]', '"enum": [\n              "task",\n              "test",\n              "noop",\n              "launch"\n            ]', "kind enum")

def actions_test(text):
    return once(text, 'expect(text.match(/^repo_task\\(/gmu)?.length).toBe(4);', 'expect(text.match(/^repo_task\\(/gmu)?.length).toBe(3);expect(text.match(/^repo_test\\(/gmu)?.length).toBe(1);', "counts")

def arguments_test(text):
    return once(text, '["task.id",\'"bazel:build"\']', '["task.id",\'"bazel:"+batch.verb\']', "build marker")

def selection_test(text):
    return once(text, '"buildChangedFilesCommandsV1","affectedBuildProjectsV1"]', '"buildChangedFilesCommandsV1","affectedTouchesV1","affectedBuildProjectsV1"]', "function list")

def catalog(text):
    t = once(text, '"notes": ["--dry-run prints a DryRunPlan on stdout and executes nothing; --graph is the same flag for every tool and takes an optional file."]',
        '"notes": [\n        "--dry-run prints a DryRunPlan on stdout and executes nothing; --graph is the same flag for every tool and takes an optional file.",\n        "Bazel: a selection builds exactly the transitive closure of its tasks in one generated workspace; a cached test-family task without declared outputs and without dependents runs as a Bazel test (cached test results, --build_tests_only when only tests are selected, --nocache_test_results with --skip-cache), every other task as a cacheable build action. Test sharding is not offered because the neutral contract defines no shard protocol."\n      ]', "run note")
    t = once(t, "Project closure stays tool-native (Nx project graph, Bazel plan).", "Project closure stays tool-native (Nx project graph; Bazel rdeps over a generated package of project and changed-file nodes, which asserts the same answer).", "affected note")
    return t

def apply(key, text):
    if key == "script.ts": return adapter(text)
    if key == "schema.json": return schema_adapter(text)
    if key == "selection.ts": return selection(text)
    if key == "neutral.ts": return neutral(text)
    if key == "catalog.json": return catalog(text)
    if key == "actions-fixture.json": return actions_fixture(text)
    if key == "actions-schema.json": return actions_schema(text)
    if key == "actions-test.ts": return actions_test(text)
    if key == "arguments-test.ts": return arguments_test(text)
    if key == "selection-test.ts": return selection_test(text)
    raise SystemExit("unknown key " + key)
