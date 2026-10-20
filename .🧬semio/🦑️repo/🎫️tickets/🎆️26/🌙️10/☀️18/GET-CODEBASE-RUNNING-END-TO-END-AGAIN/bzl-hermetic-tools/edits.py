#!/usr/bin/env python3
"""Anchored edits of the hermetic class set. `build()` returns {live path: new text} for every EXISTING live file the set changes, computed from the live text; every anchor must occur exactly once. Imported by stage.py and apply.py."""
import json, os, sys
sys.dont_write_bytecode = True
ROOT = "/Users/ueli/Documents/semio"
TOOLS = os.path.dirname(os.path.abspath(__file__))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
BZ = B + "/🔧️tools/🏗️bazel"
H = B + "/🏃️execution/🔒️hermetic"
EXEC = BZ + "/🏃️execution/🟦️.ts"
EXEC_SCHEMA = BZ + "/🏃️execution/🧬️schema/🔣️.json"
RUNNER = BZ + "/🏃️execution/🏭️runner/🟦️.ts"
ADAPTER = BZ + "/📜️script.ts"
FIXTURE = BZ + "/🏃️execution/🧪️tests/🧫️fixtures/🔣️.json"
CONTRACT_TEST = BZ + "/🏃️execution/🧪️tests/🟦️.ts"
LAW_TEST = BZ + "/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts"
STAGED = os.path.join(TOOLS, "staged")


def read(path):
    with open(path, encoding="utf8") as handle:
        return handle.read()


def staged(name):
    return read(os.path.join(STAGED, name))


def once(text, old, new, label):
    count = text.count(old)
    if count == 0 and new != old and new in text:
        return text
    if count != 1:
        raise ValueError(f"{label}: anchor found {count} times: {old[:100]!r}")
    return text.replace(old, new)


def between(text, start, end, new, label):
    a, b = text.count(start), text.count(end)
    if a != 1 or b != 1:
        raise ValueError(f"{label}: markers found {a}/{b}")
    i, j = text.index(start), text.index(end)
    if j < i:
        raise ValueError(f"{label}: markers out of order")
    return text[:i] + new + text[j:]


def schema_json(text):
    d = json.loads(text)
    defs = d["definitions"]
    toolchain = defs["Contract"]["const"]["toolchain"]
    toolchain["modulesRunner"] = "hermetic-modules.mjs"
    toolchain["modulesMnemonic"] = "RepoHermeticModules"
    shape = defs["ContractShape"]["properties"]["toolchain"]
    shape["required"] += ["modulesRunner", "modulesMnemonic"]
    shape["properties"]["modulesRunner"] = {"type": "string", "pattern": "^[a-z][a-z0-9.-]*$"}
    shape["properties"]["modulesMnemonic"] = {"type": "string", "pattern": "^[A-Z][A-Za-z0-9]*$"}
    reasons = defs["Reasons"]["const"]
    reasons["excepted"] = "A declared exception removes this proven-unfit target from its hermetic class, with a closed reason the class table records."
    reasons["dependencies"] = "The installed dependency generation cannot be declared as a content-addressed tree: it is absent or an entry leaves its layout."
    m = defs["Manifest"]["properties"]
    m["inputs"] = {"description": "Every declared input file, workspace-relative. Absent means the execution root is already isolated by the caller.", "type": "array", "maxItems": 1000000, "items": {"type": "string", "minLength": 1, "maxLength": 4096}}
    m["class"] = {"type": "string", "pattern": "^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$", "maxLength": 64}
    m["entry"] = {"description": "The module that issues the original process authority of a class member inside the action.", "type": "string", "minLength": 1, "maxLength": 4096}
    m["markers"] = {"type": "array", "maxItems": 16, "items": {"type": "string", "minLength": 1, "maxLength": 4096}}
    m["maximumElapsedMilliseconds"] = {"type": "integer", "minimum": 1000, "maximum": 86400000}
    m["providers"] = {"type": "object", "additionalProperties": {"type": "string", "minLength": 1, "maxLength": 4096}}
    m["inputEnv"] = {"type": "object", "additionalProperties": {"type": "string", "maxLength": 65536}}
    m["nodeModules"] = {"type": "array", "maxItems": 4096, "items": {"$ref": "#/definitions/ModulesRow"}}
    defs["ModulesRow"] = {
        "description": "One `node_modules` directory of the isolated workspace: links to workspace packages, links into the dependency tree and the `.bin` shims of the selected entries.",
        "type": "object", "additionalProperties": False, "required": ["path", "workspace", "tree", "bin"],
        "properties": {
            "path": {"type": "string", "minLength": 1, "maxLength": 4096},
            "workspace": {"type": "object", "additionalProperties": {"type": "string", "minLength": 1, "maxLength": 4096}},
            "tree": {"type": "object", "additionalProperties": {"type": "string", "minLength": 1, "maxLength": 4096}},
            "bin": {"type": "object", "additionalProperties": {"type": "string", "minLength": 1, "maxLength": 4096}},
        },
    }
    defs["DependenciesManifest"] = {
        "description": "Closed manifest of one dependency tree: the installed stores it copies from and the `<store index>/<identity>` entries it holds.",
        "type": "object", "additionalProperties": False, "required": ["version", "stores", "ids"],
        "properties": {
            "version": {"const": 1},
            "stores": {"type": "array", "minItems": 1, "maxItems": 64, "items": {"type": "string", "minLength": 1, "maxLength": 4096}},
            "ids": {"type": "array", "minItems": 1, "maxItems": 100000, "items": {"type": "string", "pattern": "^[0-9]+/[^/]+$", "maxLength": 512}},
        },
    }
    return json.dumps(d, ensure_ascii=False, indent=2) + "\n"


STARLARK_OLD_START = "def _hermetic_impl(ctx):"
STARLARK_NEW = '''def _hermetic_impl(ctx):
    bun = ctx.toolchains["//:${contract.toolchain.type}"].bun
    output = ctx.actions.declare_directory(ctx.label.name + ".snapshot")
    snapshots = [dep[DefaultInfo].files.to_list()[0] for dep in ctx.attr.deps]
    modules = ctx.attr.modules[DefaultInfo].files.to_list()[0] if ctx.attr.modules else None
    ctx.actions.run(executable = bun, tools = [bun], arguments = [ctx.file.runner.path, ctx.file.manifest.path, output.path] + (["--modules=" + modules.path] if modules else []) + [snapshot.path for snapshot in snapshots], inputs = depset([ctx.file.manifest, ctx.file.runner] + snapshots + ([modules] if modules else []), transitive = [src[DefaultInfo].files for src in ctx.attr.srcs]), outputs = [output], mnemonic = "${contract.toolchain.mnemonic}", progress_message = ctx.attr.identity, env = {${environmentLines}}, use_default_shell_env = False, execution_requirements = {"block-network": "1"}, toolchain = "//:${contract.toolchain.type}")
    return [DefaultInfo(files = depset([output]))]

repo_hermetic_task = rule(implementation = _hermetic_impl, toolchains = ["//:${contract.toolchain.type}"], attrs = {"manifest": attr.label(allow_single_file = True), "runner": attr.label(allow_single_file = True), "srcs": attr.label_list(allow_files = True), "deps": attr.label_list(), "modules": attr.label(), "identity": attr.string(mandatory = True)})
`;
'''

MODULES_STARLARK = '''
/** 🌳️ The Starlark rule that builds one dependency tree: a host-state action by design, because it reads the installed dependency stores by the absolute paths of its manifest; its output tree is what hermetic actions take as an input by content, so the stores never enter a hermetic action key. */
export const BAZEL_HERMETIC_MODULES_STARLARK=`
def _hermetic_modules_impl(ctx):
    bun = ctx.toolchains["//:${contract.toolchain.type}"].bun
    output = ctx.actions.declare_directory(ctx.label.name)
    ctx.actions.run(executable = bun, tools = [bun], arguments = [ctx.file.runner.path, ctx.file.manifest.path, output.path], inputs = [ctx.file.manifest, ctx.file.runner], outputs = [output], mnemonic = "${contract.toolchain.modulesMnemonic}", progress_message = "dependency tree " + ctx.label.name, env = {${environmentLines}}, use_default_shell_env = False, execution_requirements = {"no-sandbox": "1", "no-remote-exec": "1"}, toolchain = "//:${contract.toolchain.type}")
    return [DefaultInfo(files = depset([output]))]

repo_hermetic_modules = rule(implementation = _hermetic_modules_impl, toolchains = ["//:${contract.toolchain.type}"], attrs = {"manifest": attr.label(allow_single_file = True), "runner": attr.label(allow_single_file = True)})
`;
'''


def execution(text):
    t = text
    t = once(t, 'import hermeticSchema from"../../../🏃️execution/🧬️schema/🔒️hermetic.json" with {type:"json"};\n', 'import hermeticSchema from"../../../🏃️execution/🧬️schema/🔒️hermetic.json" with {type:"json"};\nimport{HERMETIC_TABLE_V1,hermeticInputsV1,hermeticSelectV1,hermeticStrategiesV1}from"../../../🏃️execution/🔒️hermetic/🟦️.ts";\nimport{hermeticDependenciesV1,type HermeticDependenciesV1,type HermeticModulesRowV1}from"../../../🏃️execution/🔒️hermetic/📦️dependencies/🟦️.ts";\n', "exec import")
    t = once(t, 'export{BAZEL_HERMETIC_RUNNER}from"./🏭️runner/🟦️.ts";', 'export{BAZEL_HERMETIC_DEPENDENCIES,BAZEL_HERMETIC_RUNNER}from"./🏭️runner/🟦️.ts";', "exec export")
    t = once(t, 'export type BazelHermeticDecisionV1={hermetic:true}|{hermetic:false;reason:BazelHermeticReasonV1};', 'export type BazelHermeticDecisionV1={hermetic:true;class?:string}|{hermetic:false;reason:BazelHermeticReasonV1};', "exec decision type")
    t = between(t, "/** 🛡️ Decides whether a cacheable task is a hermetic action", "/** 🔬️ One spawn of Bazel's JSON execution log", staged("execution-new.ts"), "exec decision")
    t = once(t, '"--noremote_local_fallback");\n const flags=', '"--noremote_local_fallback");\n command.push(`--strategy=${contract.toolchain.mnemonic}=${hermeticStrategiesV1(options.host.platform).filter(name=>remote!==undefined||name!=="remote").join(",")}`);\n const flags=', "exec strategy flag")
    t = once(t, 'expected:Readonly<{inputs:readonly string[];tool:string;properties:Readonly<Record<string,string>>}>', 'expected:Readonly<{inputs:readonly string[];tool:string;properties:Readonly<Record<string,string>>;trees?:readonly string[]}>', "exec violations type")
    t = once(t, 'if(!allowed.has(path)&&!/\\.snapshot\\/(snapshot\\.json|files\\/[a-f0-9]{32})$/u.test(path))found.push', 'if(!allowed.has(path)&&!(expected.trees??[]).some(prefix=>path.startsWith(prefix))&&!/\\.snapshot\\/(snapshot\\.json|files\\/[a-f0-9]{32})$/u.test(path))found.push', "exec violations tree")
    i = t.index(STARLARK_OLD_START)
    j = t.index("`;\n", i) + 3
    t = t[:i] + STARLARK_NEW.lstrip("\n") + t[j:]
    k = t.index("/** 🔐️ Reads the pinned bun release row for one platform id. */")
    t = t[:k] + MODULES_STARLARK.lstrip("\n") + "\n" + t[k:]
    return t


def adapter(text):
    t = text
    t = once(t, "import {BAZEL_HERMETIC_RUNNER,BAZEL_HERMETIC_STARLARK,bazelBunStorePathV1,", "import {BAZEL_HERMETIC_DEPENDENCIES,BAZEL_HERMETIC_MODULES_STARLARK,BAZEL_HERMETIC_RUNNER,BAZEL_HERMETIC_STARLARK,bazelBunStorePathV1,bazelHermeticModulesLabelV1,bazelHermeticModulesManifestV1,bazelHermeticPlanV1,type BazelHermeticPlanV1,", "adapter import")
    t = once(t, 'import {bazelPlatformProfileV1,bazelPlatformRulesV1} from "./🌐️platform/🟦️.ts";\n', 'import {bazelPlatformProfileV1,bazelPlatformRulesV1} from "./🌐️platform/🟦️.ts";\nimport {HERMETIC_TABLE_V1} from "../../🏃️execution/🔒️hermetic/🟦️.ts";\n', "adapter table import")
    t = once(t, 'export interface BazelActionV1{id:string;label:string;kind:"task"|"noop"|"launch"|"hermetic"|"test";restore:string[];cache:BazelCacheDecisionV1;worker?:BazelActionWorkerV1;exclusive?:true}', 'export interface BazelActionV1{id:string;label:string;kind:"task"|"noop"|"launch"|"hermetic"|"test";restore:string[];cache:BazelCacheDecisionV1;worker?:BazelActionWorkerV1;exclusive?:true;hermeticClass?:string;modules?:string}', "adapter action type")
    t = once(t, 'execution:{hermetic:boolean}={hermetic:false},tests?:ReadonlySet<string>', 'execution:{hermetic:boolean;demoted?:ReadonlySet<string>}={hermetic:false},tests?:ReadonlySet<string>', "adapter actions signature")
    t = once(t, 'hermetic=kind==="task"&&execution.hermetic&&bazelHermeticDecisionV1(task,cache,()=>bazelTaskInputsV1(graph,task,"ignore")).hermetic;', 'decision=kind==="task"&&execution.hermetic&&!execution.demoted?.has(task.id)?bazelHermeticDecisionV1(task,cache,()=>bazelTaskInputsV1(graph,task,"ignore")):undefined,hermetic=decision?.hermetic===true;', "adapter decision")
    t = once(t, 'cache,...(worker?{worker}:{}),...(kind==="task"&&!hermetic&&bazelCargoClassV1(task)', 'cache,...(worker?{worker}:{}),...(decision?.hermetic===true&&decision.class!==undefined?{hermeticClass:decision.class}:{}),...(kind==="task"&&!hermetic&&bazelCargoClassV1(task)', "adapter action class")
    t = once(t, '...(input.actions.some(action=>action.kind==="hermetic")?["repo_hermetic_task"]:[]),', '...(input.actions.some(action=>action.kind==="hermetic")?["repo_hermetic_task"]:[]),...(input.trees?.length?["repo_hermetic_modules"]:[]),', "render symbols")
    t = once(t, 'execution?:string;remote?:boolean}):string{', 'execution?:string;remote?:boolean;trees?:readonly string[]}):string{', "render signature")
    t = once(t, ' if(input.execution!==undefined)lines.push(input.execution);\n', ' if(input.execution!==undefined)lines.push(input.execution);\n for(const label of input.trees??[])lines.push("repo_hermetic_modules(",`    name = ${quote(label)},`,`    manifest = ${quote(label+".json")},`,`    runner = ${quote("hermetic-modules.mjs")},`,")","");\n', "render trees")
    t = once(t, '["runner",quote("hermetic-runner.mjs")],["identity",quote(action.id)]);', '["runner",quote("hermetic-runner.mjs")],["identity",quote(action.id)]);\n   if(action.modules)attributes.push(["modules",quote(":"+action.modules)]);', "render modules attribute")
    t = once(t, 'inputs=new Map(declared.map(action=>[action.id,bazelTaskInputsV1(graph,plan.tasks[action.id]!,exactClosures)])),bundled=', 'inputs=new Map(declared.map(action=>{const values=bazelTaskInputsV1(graph,plan.tasks[action.id]!,exactClosures);return [action.id,action.hermeticClass===undefined?values:[...values,...HERMETIC_TABLE_V1.runtime.inputs]] as [string,unknown[]];})),bundled=', "generate inputs")
    t = once(t, '(hasHermetic?BAZEL_HERMETIC_STARLARK:"")', '(hasHermetic?BAZEL_HERMETIC_STARLARK+BAZEL_HERMETIC_MODULES_STARLARK:"")', "generate starlark")
    t = once(t, ' if(hasHermetic)await writeChanged(join(workspace,"hermetic-runner.mjs"),BAZEL_HERMETIC_RUNNER);\n', ' if(hasHermetic){await writeChanged(join(workspace,"hermetic-runner.mjs"),BAZEL_HERMETIC_RUNNER);await writeChanged(join(workspace,"hermetic-modules.mjs"),BAZEL_HERMETIC_DEPENDENCIES);}\n', "generate runners")
    t = once(t, ' const emitted=bazelActionsV1(graph,plan,request.skipCache===true,{hermetic:true},tests);', ' const hermeticPlans=new Map<string,BazelHermeticPlanV1>(),demoted=new Set<string>();\n for(const action of candidates.values())if(action.kind==="hermetic"&&action.hermeticClass!==undefined){checkScriptInvocation(invocation);const{marks,extra}=index.selectAll(inputs.get(action.id)!),derived=bazelHermeticPlanV1({root,task:plan.tasks[action.id]!,class:action.hermeticClass,values:inputs.get(action.id)!,declared:[...index.files.filter((_path,position)=>marks[position]),...extra].sort(),environment:process.env,check:()=>checkScriptInvocation(invocation)});if(derived)hermeticPlans.set(action.id,derived);else demoted.add(action.id);}\n const emitted=bazelActionsV1(graph,plan,request.skipCache===true,{hermetic:true,demoted},tests);', "generate demotion")
    t = once(t, ' let authorized:Set<string>|undefined;\n', ' let authorized:Set<string>|undefined;const trees=new Map<string,{version:1;stores:string[];ids:string[]}>(),modules=new Map<string,string>();\n', "generate trees state")
    t = once(t, 'if(action.kind==="hermetic"){checkScriptInvocation(invocation);const task=plan.tasks[action.id]!;await writeChanged(join(workspace,task.label+".json"),JSON.stringify(bazelHermeticManifestV1(task,buildExecutionArgumentsV1({...task.definition.execution,forwardAllArgs:true},task.args))));continue;}',
            'if(action.kind==="hermetic"){checkScriptInvocation(invocation);const task=plan.tasks[action.id]!,derived=hermeticPlans.get(action.id),{marks,extra}=index.selectAll(inputs.get(action.id)!),paths=[...index.files.filter((_path,position)=>marks[position]&&declarable.has(index.files[position]!)),...extra.filter(path=>declarable.has(path))].sort(),label=derived&&derived.dependencies.ids.length?bazelHermeticModulesLabelV1(derived.dependencies):undefined;if(derived&&label){if(!trees.has(label))trees.set(label,bazelHermeticModulesManifestV1(derived.dependencies));modules.set(action.id,label);}await writeChanged(join(workspace,task.label+".json"),JSON.stringify(bazelHermeticManifestV1(task,buildExecutionArgumentsV1({...task.definition.execution,forwardAllArgs:true},task.args),{inputs:paths,...(action.hermeticClass!==undefined&&derived?{class:action.hermeticClass,inputEnv:derived.inputEnv,providers:derived.providers,rows:derived.dependencies.rows}:{})})));continue;}', "generate manifest")
    t = once(t, ' const executionBuild=hasHermetic?', ' for(const[label,manifest]of trees)await writeChanged(join(workspace,label+".json"),JSON.stringify(manifest));\n const executionBuild=hasHermetic?', "generate tree manifests")
    t = once(t, 'actions:[...emitted.values()].map(action=>({...action,...(sources.has(action.id)?{sources:sources.get(action.id)}:{})})),groups:tree.groups,', 'actions:[...emitted.values()].map(action=>({...action,...(sources.has(action.id)?{sources:sources.get(action.id)}:{}),...(modules.has(action.id)?{modules:modules.get(action.id)}:{})})),groups:tree.groups,trees:[...trees.keys()].sort(),', "generate render")
    return t


def fixture(text):
    d = json.loads(text)
    host = {"darwin": "darwin-sandbox", "linux": "linux-sandbox", "win32": None}
    for row in d["flags"]:
        expected = row["expected"]
        if "error" in expected or not row["hermetic"]:
            continue
        platform = row["host"]["platform"]
        names = {"darwin": ["remote", "darwin-sandbox", "local"], "linux": ["remote", "linux-sandbox", "processwrapper-sandbox", "local"], "win32": ["remote", "local"]}[platform]
        remote = expected["remote"]
        expected["command"].append("--strategy=RepoHermetic=" + ",".join(n for n in names if remote or n != "remote"))
    return json.dumps(d, ensure_ascii=False, indent=1) + "\n"


def law(text):
    old = 'if(process.platform!=="win32"){const local=build("base-a",["neg:iso"],["--spawn_strategy=local"]);expect(local.status,"a non-isolated run must be refused").not.toBe(0);expect(local.stderr+local.stdout).toMatch(/isolated execution root is required/u);}'
    new = '{const local=build("base-local",["neg:iso"],["--strategy=RepoHermetic=local"]);expect(local.status,local.stderr).toBe(0);expect(local.stderr).toMatch(/\\d+ local/u);const localLeak=build("base-local",["neg:leak"],["--strategy=RepoHermetic=local"]);expect(localLeak.status,"an undeclared read must fail under the local strategy too").not.toBe(0);expect(localLeak.stderr+localLeak.stdout).toMatch(/secret\\.txt/u);}'
    text = once(text, old, new, "law local strategy")
    return once(text, 'for(const base of ["base-b","base-r"])', 'for(const base of ["base-b","base-r","base-local"])', "law shutdown")


def contract_test(text):
    return text.rstrip("\n") + "\n" + staged("contract-tests.ts")


def build():
    out = {}
    for path, fn in ((EXEC, execution), (EXEC_SCHEMA, schema_json), (ADAPTER, adapter), (FIXTURE, fixture), (LAW_TEST, law), (CONTRACT_TEST, contract_test)):
        out[path] = fn(read(os.path.join(ROOT, path)))
    out[RUNNER] = staged("runner.ts")
    return out


def fixture_roundtrip_ok():
    text = read(os.path.join(ROOT, FIXTURE))
    return json.dumps(json.loads(text), ensure_ascii=False, indent=1) + "\n" == text


if __name__ == "__main__":
    print(fixture_roundtrip_ok())
    result = build()
    for path in result:
        print(path, len(result[path]))
