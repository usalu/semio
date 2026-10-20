# 🧩 Builds the staged neutral policy, invocation, selection, root build script, nx bootstrap and operations catalogue edits.
import json
import lib

# build policy: the authored worker policy lives with the neutral build config
policy_path = lib.BUILD + "/\U0001F523️policy.json"
policy = json.loads(lib.read(policy_path))
workers = json.loads(lib.staged("workers-policy.json"))
policy["workers"] = workers
lib.write_staged("policy.json", json.dumps(policy, indent=2, ensure_ascii=False) + "\n")

# invocation: a global --workers on|off next to --tool
inv_path = lib.BUILD + "/\U0001F9ED️invocation/" + lib.TS
e = lib.Edit(lib.read(inv_path), "invocation")
e.once('import policy from "../🔣️policy.json" with { type: "json" };',
       'import policy from "../🔣️policy.json" with { type: "json" };\nimport {readWorkerSwitchV1} from "../👷️workers/🟦️.ts";')
e.once('export function parseBuildInvocationV1(args:readonly string[],environment:NodeJS.ProcessEnv=process.env):{tool:"nx"|"bazel";args:string[]} {\n  let tool=environment.SEMIO_BUILD_TOOL??policy.defaultTool;',
       'export function parseBuildInvocationV1(args:readonly string[],environment:NodeJS.ProcessEnv=process.env):{tool:"nx"|"bazel";args:string[];workers?:"on"|"off"} {\n  let tool=environment.SEMIO_BUILD_TOOL??policy.defaultTool,workers=environment.SEMIO_BUILD_WORKERS;')
e.once('    if(argument.startsWith("--tool=")){tool=argument.slice(7);continue;}\n',
       '    if(argument.startsWith("--tool=")){tool=argument.slice(7);continue;}\n    if(argument==="--workers"){workers=args[++index]??"";continue;}\n    if(argument.startsWith("--workers=")){workers=argument.slice(10);continue;}\n')
e.once('  if(tool!=="nx"&&tool!=="bazel")throw Error(`Unknown build tool: ${tool}`);\n  return {tool,args:selected};',
       '  if(tool!=="nx"&&tool!=="bazel")throw Error(`Unknown build tool: ${tool}`);\n  readWorkerSwitchV1(workers);\n  return {tool,args:selected,...(workers?{workers:workers as "on"|"off"}:{})};')
lib.write_staged("invocation.ts", e.text)

# selection: the worker switch reaches the Bazel request like the job count
sel_path = lib.BUILD + "/\U0001F9ED️selection/" + lib.TS
e = lib.Edit(lib.read(sel_path), "selection")
e.once('parallel?:number;skipCache:boolean;skipRemoteCache:boolean;excludeDependencies:boolean;bail:boolean;verbose:boolean;outputStyle?:"stream"|"stream-without-prefixes"|"static"}> {',
       'parallel?:number;workers?:boolean;skipCache:boolean;skipRemoteCache:boolean;excludeDependencies:boolean;bail:boolean;verbose:boolean;outputStyle?:"stream"|"stream-without-prefixes"|"static"}> {')
e.once('  if(parallel!==undefined&&(!Number.isSafeInteger(parallel)||parallel<1))throw Error("Build parallelism must be a positive integer");\n',
       '  if(parallel!==undefined&&(!Number.isSafeInteger(parallel)||parallel<1))throw Error("Build parallelism must be a positive integer");\n  const workers=readWorkerSwitchV1(process.env.SEMIO_BUILD_WORKERS);\n')
e.once('...(parallel?{parallel}:{}),skipCache:enabled(["--skip-cache"])', '...(parallel?{parallel}:{}),...(workers===undefined?{}:{workers}),skipCache:enabled(["--skip-cache"])')
head = e.text.split("\n")
# add import after the first import of cachePolicy
e.once('import cachePolicy from "../../⚡️caching/🔣️policy.json" with { type: "json" };',
       'import cachePolicy from "../../⚡️caching/🔣️policy.json" with { type: "json" };\nimport {readWorkerSwitchV1} from "../👷️workers/🟦️.ts";')
lib.write_staged("selection.ts", e.text)

# selection test: the evaluated selection body now reads the worker switch, so the test binds it like its other collaborators
sel_test_path = lib.BUILD + "/\U0001F9ED\uFE0Fselection/" + lib.TESTS + "/" + lib.TS
t = lib.read(sel_test_path)
e = lib.Edit(t.replace("bindings={checkScriptInvocation,buildOptionV1,", "bindings={checkScriptInvocation,readWorkerSwitchV1,buildOptionV1,"), "selection test")
assert e.text.count("readWorkerSwitchV1,buildOptionV1") == 3 == t.count("bindings={checkScriptInvocation,buildOptionV1,")
e.once('import*as selectionOwner from"../🟦️.ts";', 'import*as selectionOwner from"../🟦️.ts";\nimport {readWorkerSwitchV1} from "../../👷️workers/🟦️.ts";')
lib.write_staged("selection-test.ts", e.text)

# root build script: the resolved worker switch is part of the invocation environment, like the tool
root_path = lib.BUILD + "/" + lib.SCRIPT
e = lib.Edit(lib.read(root_path), "root script")
e.once('SEMIO_BUILD_TOOL:selected.tool,SEMIO_WORKSPACE_ROOT:root,', 'SEMIO_BUILD_TOOL:selected.tool,SEMIO_BUILD_WORKERS:selected.workers,SEMIO_WORKSPACE_ROOT:root,')
lib.write_staged("build-script.ts", e.text)

# nx bootstrap: the local job count and batch mode come from the neutral plan
nx_path = lib.BOOT + "/" + lib.SCRIPT
e = lib.Edit(lib.read(nx_path), "nx bootstrap")
e.once('import {resolveBuildSelectionV1} from "../../🏗️build/🧭️invocation/🟦️.ts";',
       'import {resolveBuildSelectionV1} from "../../🏗️build/🧭️invocation/🟦️.ts";\nimport buildPolicy from "../../🏗️build/🔣️policy.json" with { type: "json" };\nimport nxExecutors from "../../📦️packages/🟦️typescript/📋️executors.json" with { type: "json" };\nimport {readWorkerPolicyV1,planWorkersV1,currentWorkerHostV1,readWorkerSwitchV1} from "../../🏗️build/👷️workers/🟦️.ts";\nimport {nxBatchCapableV1,nxWorkerSettingsV1} from "./🛠️tools/👷️workers/🟦️.ts";')
e.once(' env.NX_PARALLEL=environment.SEMIO_BUILD_PARALLEL??environment.NX_PARALLEL;\n',
       ''' const explicit=Number(environment.SEMIO_BUILD_PARALLEL??environment.NX_PARALLEL),workers=nxWorkerSettingsV1(planWorkersV1(readWorkerPolicyV1(buildPolicy.workers),currentWorkerHostV1(),{...(Number.isSafeInteger(explicit)&&explicit>0?{jobs:explicit}:{}),...(readWorkerSwitchV1(environment.SEMIO_BUILD_WORKERS)===undefined?{}:{workers:readWorkerSwitchV1(environment.SEMIO_BUILD_WORKERS)!})}),{batchCapable:nxBatchCapableV1(nxExecutors)});
 env.NX_PARALLEL=environment.SEMIO_BUILD_PARALLEL??environment.NX_PARALLEL??workers.environment.NX_PARALLEL;
 if(workers.environment.NX_BATCH_MODE!==undefined)env.NX_BATCH_MODE??=workers.environment.NX_BATCH_MODE;
''')
lib.write_staged("nx-bootstrap.ts", e.text)

# operations catalogue: the global --workers flag with English and German help
cat_path = lib.BUILD + "/\u2699\uFE0Foperations/\U0001F523\uFE0Fcatalog.json"
e = lib.Edit(lib.read(cat_path), "catalogue")
e.once('        "de": "Wählt die Build-Implementierung; sonst SEMIO_BUILD_TOOL, dann der Standard der Policy (nx)."\n      }\n    },\n',
       '        "de": "Wählt die Build-Implementierung; sonst SEMIO_BUILD_TOOL, dann der Standard der Policy (nx)."\n      }\n    },\n'
       '    {\n      "name": "--workers",\n      "value": "required",\n      "valueName": "on|off",\n      "description": {\n'
       '        "en": "Run actions in persistent workers (on) or in one process each (off); otherwise SEMIO_BUILD_WORKERS, then the worker policy of the build policy.",\n'
       '        "de": "Führt Aktionen in persistenten Workern (on) oder je in einem eigenen Prozess (off) aus; sonst SEMIO_BUILD_WORKERS, dann die Worker-Policy der Build-Policy."\n      }\n    },\n')
lib.write_staged("catalog.json", e.text)
print("ok")
