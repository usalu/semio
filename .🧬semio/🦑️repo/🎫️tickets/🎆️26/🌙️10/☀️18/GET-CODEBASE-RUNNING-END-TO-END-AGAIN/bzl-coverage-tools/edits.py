#!/usr/bin/env python3
"""Single source of truth for the staged bzl-coverage change set: new files and anchored edits of existing files.

Used by apply.py (apply to a root, drift check against bases.json, dry run) and by the overlay builder.
Every anchor must occur exactly once in its target; emoji path parts are built from code points so the anchors cannot drift by invisible selectors."""
import os


def E(code):
    return chr(code) + "️"


HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
LIBRARY = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
BUILD = os.path.join(LIBRARY, E(0x1F3D7) + "build")
EXECUTION = os.path.join(BUILD, E(0x1F3C3) + "execution")
BAZEL = os.path.join(BUILD, E(0x1F527) + "tools", E(0x1F3D7) + "bazel")
TESTS = os.path.join(BAZEL, E(0x1F9EA) + "tests")
SCHEMA = E(0x1F9EC) + "schema"
TS = E(0x1F7E6) + ".ts"
JSON_ = E(0x1F523) + ".json"
SCRIPT = E(0x1F4DC) + "script.ts"
COVERAGE_DIR = E(0x1F3AF) + "coverage"
EXEC_WORD = E(0x1F3C3) + "execution"
FIXTURE_SCHEMA = E(0x1F9EC) + "schema.json"

NEW = {
    "coverage-schema.json": os.path.join(EXECUTION, SCHEMA, COVERAGE_DIR + ".json"),
    "coverage.ts": os.path.join(EXECUTION, TS),
    "coverage-test.ts": os.path.join(EXECUTION, E(0x1F9EA) + "tests", COVERAGE_DIR, TS),
    "coverage-fixture.json": os.path.join(EXECUTION, E(0x1F9EA) + "tests", COVERAGE_DIR, JSON_),
    "coverage-fixture-schema.json": os.path.join(EXECUTION, E(0x1F9EA) + "tests", COVERAGE_DIR, FIXTURE_SCHEMA),
    "bazel-test.ts": os.path.join(TESTS, E(0x1F4E5) + "coverage", TS),
    "bazel-fixture.json": os.path.join(TESTS, E(0x1F4E5) + "coverage", JSON_),
    "bazel-fixture-schema.json": os.path.join(TESTS, E(0x1F4E5) + "coverage", FIXTURE_SCHEMA),
}

SCRIPT_PATH = os.path.join(BAZEL, SCRIPT)
BAZEL_SCHEMA = os.path.join(BAZEL, SCHEMA, JSON_)
MAIN_TEST = os.path.join(TESTS, TS)
ACTIONS_FIXTURE = os.path.join(TESTS, E(0x1F4E5) + "actions", JSON_)
ACTIONS_TEST = os.path.join(TESTS, E(0x1F4E5) + "actions", TS)
CONTRACT_FIXTURE = os.path.join(TESTS, E(0x1F4CB) + "contract", JSON_)
ACTIONS_SCHEMA = os.path.join(TESTS, E(0x1F4E5) + "actions", FIXTURE_SCHEMA)


def sub(text):
    return text.replace("{SCRIPT}", SCRIPT).replace("{EXEC}", EXEC_WORD).replace("{TS}", TS).replace("{SCHEMA}", SCHEMA).replace("{COVERAGE}", COVERAGE_DIR).replace("{TARGET}", E(0x1F3AF))


COVERAGE_FUNCTION = sub('''

/** {TARGET} Classifies one planned task by tool family and neutral disposition from its manifest and cache decision; `script` is the optional entry script text that tells vitest from the Bun runner. */
export function bazelTaskCoverageV1(task:BazelTaskV1,decision:BazelCacheDecisionV1,script?:string):TaskCoverageV1{
 const execution=task.definition.execution as {kind:string;command?:string;commands?:readonly (string|{command:string})[]},row=execution.commands?.[0],first=execution.command??(typeof row==="string"?row:row?.command),outputs=task.definition.outputs;
 return taskCoverageV1({project:task.project,target:task.target,root:task.root,kind:execution.kind==="noop"?"noop":"command",...(first!==undefined?{command:first}:{}),...(script!==undefined?{script}:{}),...(Array.isArray(outputs)?{outputs}:{}),cache:decision.cache?{cache:true}:{cache:false,reason:decision.reason!}});
}

/** {TARGET} Counts a planned selection by tool family and by how Bazel executes it: a task that runs as a Bazel test counts as `test`, a cached task named test that another action consumes counts as `action`. */
export function bazelCoverageReportV1(plan:BazelTaskPlanV1,emitted:ReadonlyMap<string,BazelActionV1>,scripts?:ReadonlyMap<string,string>):TaskCoverageReportV1{
 return taskCoverageReportV1([...emitted.values()].map(action=>{const row=bazelTaskCoverageV1(plan.tasks[action.id]!,action.cache,scripts?.get(action.id));return action.kind==="test"||row.disposition!=="test"?row:{...row,disposition:"action" as const};}));''')

TEST_RULE = '''def _test_impl(ctx):
    launcher = ctx.actions.declare_file(ctx.label.name + ctx.attr.extension)
    ctx.actions.run(executable = ctx.attr.bun, arguments = [ctx.file.runner.path, ctx.file.source.path, ctx.attr.source_identity, "birth-launcher", ctx.file.manifest.path, launcher.path], inputs = [ctx.file.manifest, ctx.file.runner, ctx.file.source], outputs = [launcher], mnemonic = "RepoTestLauncher", execution_requirements = _requirements(ctx), use_default_shell_env = True)
    dependencies = [dep[DefaultInfo].files.to_list()[0] for dep in ctx.attr.deps]
    runfiles = ctx.runfiles(files = dependencies + [ctx.file.manifest, ctx.file.fingerprint, ctx.file.runner, ctx.file.source], transitive_files = depset(transitive = [src[DefaultInfo].files for src in ctx.attr.srcs]))
    return [DefaultInfo(executable = launcher, runfiles = runfiles), testing.ExecutionInfo(_requirements(ctx))]

repo_test = rule(implementation = _test_impl, test = True, attrs = {"manifest": attr.label(allow_single_file = True), "fingerprint": attr.label(allow_single_file = True), "srcs": attr.label_list(allow_files = True), "deps": attr.label_list(), "runner": attr.label(allow_single_file = True), "source": attr.label(allow_single_file = True), "source_identity": attr.string(mandatory = True), "bun": attr.string(mandatory = True), "identity": attr.string(mandatory = True), "extension": attr.string(default = ""), "cache": attr.bool(default = False), "uncached_reason": attr.string(), "host_state": attr.string()})

def _noop_impl(ctx):'''

EDITS = {
    SCRIPT_PATH: [
        ('import targetCacheSchema from', sub('import {taskCoverageV1,taskCoverageReportV1,type TaskCoverageV1,type TaskCoverageReportV1} from "../../{EXEC}/{TS}";\nimport targetCacheSchema from')),
        ('kind:"task"|"noop"|"launch";restore:string[];cache:BazelCacheDecisionV1}', 'kind:"task"|"noop"|"launch"|"test";restore:string[];cache:BazelCacheDecisionV1}'),
        ('cache:bazelCacheDecisionV1(task,skipCache)});\n }\n return emitted;', 'cache:bazelCacheDecisionV1(task,skipCache)});\n }\n const consumed=new Set([...emitted.values()].flatMap(action=>action.restore));\n for(const action of emitted.values())if(action.kind==="task"&&plan.selected.includes(action.id)&&!consumed.has(action.label)&&bazelTaskCoverageV1(plan.tasks[action.id]!,action.cache).disposition==="test")action.kind="test";\n return emitted;\n}' + COVERAGE_FUNCTION),
        ('def _noop_impl(ctx):', TEST_RULE),
        ("'load(\":tasks.bzl\", \"repo_launch\", \"repo_noop\", \"repo_task\")'", "'load(\":tasks.bzl\", \"repo_launch\", \"repo_noop\", \"repo_task\", \"repo_test\")'"),
        ('if(launch)attributes.push(["extension",quote(input.extension)]);else attributes.push(["identity",quote(action.id)]);', sub('if(launch)attributes.push(["extension",quote(input.extension)]);else attributes.push(["identity",quote(action.id)]);\n   if(action.kind==="test")attributes.push(["extension",quote(input.extension)],["args",list(["{SCRIPT}","implementation.json",input.sourceIdentity,"birth-test",action.label+".json",...action.restore.map(label=>label+".snapshot")])],["timeout",quote("eternal")]);')),
        ('"repo_noop":action.kind==="launch"?"repo_launch":"repo_task"}(`', '"repo_noop":action.kind==="launch"?"repo_launch":action.kind==="test"?"repo_test":"repo_task"}(`'),
        ('filter(action=>action.kind==="task"&&action.cache.cache)', 'filter(action=>(action.kind==="task"||action.kind==="test")&&action.cache.cache)'),
        ('if(action.kind==="task"){const fingerprint=await taskFingerprint(', 'if(action.kind==="task"||action.kind==="test"){const fingerprint=await taskFingerprint('),
        ('{action:[[join(bin,task.label+".snapshot"),...action.restore.map(label=>join(bin,label+".snapshot"))]]}});', '{...(action.kind==="test"?{launcher:[[join(bin,task.label+extension)]],test:[action.restore.map(label=>join(bin,task.label+".runfiles","_main",label+".snapshot"))]}:{action:[[join(bin,task.label+".snapshot"),...action.restore.map(label=>join(bin,label+".snapshot"))]]})}});'),
        ('building=runOwnedCommand(bazel,[...startup,"build",...options,', 'const tests=plan.selected.filter(id=>emitted.get(id)!.kind==="test");building=runOwnedCommand(bazel,[...startup,tests.length?"test":"build",...options,...(tests.length?[`--test_output=${tests.length===1?"all":"errors"}`,"--test_summary=short","--enable_runfiles"]:[]),'),
        ('output=command==="launch"?undefined:parameters.shift()', 'output=command==="launch"||command==="test"?undefined:parameters.shift()'),
        ('["action","launch","launcher"].includes(command??"")||!file||command!=="launch"&&!output', '["action","launch","launcher","test"].includes(command??"")||!file||command!=="launch"&&command!=="test"&&!output'),
    ],
    BAZEL_SCHEMA: [
        ('            "action",\n            "launcher",\n            "launch"\n          ]', '            "action",\n            "launcher",\n            "launch",\n            "test"\n          ]'),
    ],
}

FIXTURE_TEST_BLOCK = sub('''    "    identity = \\"app:test\\",",
    "    extension = \\"\\",",
    "    args = [",
    "        \\"{SCRIPT}\\",",
    "        \\"implementation.json\\",",
    "        \\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\\",",
    "        \\"birth-test\\",",
    "        \\"t_71eb00441669d8d8.json\\",",
    "        \\"t_22b308f061d695be.snapshot\\",",
    "        \\"t_b9f6bc1659d098ec.snapshot\\",",
    "    ],",
    "    timeout = \\"eternal\\",",''')

EDITS[ACTIONS_FIXTURE] = [
    ('\\"repo_noop\\", \\"repo_task\\")', '\\"repo_noop\\", \\"repo_task\\", \\"repo_test\\")'),
    ('"repo_task(",\n    "    name = \\"t_71eb00441669d8d8\\",', '"repo_test(",\n    "    name = \\"t_71eb00441669d8d8\\",'),
    ('    "    identity = \\"app:test\\",",', FIXTURE_TEST_BLOCK),
    ('"app:test": {\n      "kind": "task",', '"app:test": {\n      "kind": "test",'),
]

EDITS[ACTIONS_TEST] = [
    ('expect(text.match(/^repo_task\\(/gmu)?.length).toBe(4);', 'expect(text.match(/^repo_task\\(/gmu)?.length).toBe(3);expect(text.match(/^repo_test\\(/gmu)?.length).toBe(1);'),
]

EDITS[ACTIONS_SCHEMA] = [
    ('              "noop",\n              "launch"\n            ]', '              "noop",\n              "launch",\n              "test"\n            ]'),
]

EDITS[CONTRACT_FIXTURE] = [
    ('690de8a1ec83d3bcdcad9552a9577bfd45ba613eb9f15edff7c3290512a5c50e', 'd8221916d9a0e97095846a6c3f21d0813d672575778ada6f3505bc7af927a8ed'),
]
