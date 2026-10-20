# 🩹 Hand integration of semantic overlaps between staged sets: anchored replacements applied to the merged files (merge.py, last stage).
# Each entry: rel (L/ = library root, BZ/ = Bazel adapter dir), old (anchor, must occur exactly `count` times), new, why, optional after (only applied when that set is part of the fold).
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
BZ = L + "/🏗️build/🔧️tools/🏗️bazel"
SCRIPT = "BZ/📜️script.ts"


def expand(rel):
    return rel.replace("BZ/", BZ + "/", 1) if rel.startswith("BZ/") else rel.replace("L/", L + "/", 1) if rel.startswith("L/") else rel


def patch(why, old, new, rel=SCRIPT, **extra):
    return {"why": why, "rel": rel, "old": old, "new": new, **extra}


PATCHES = [
    patch("one test design: bzl-coverage classifies, bzl-lazy plans commands; the planner needs no per-task dependents map",
          'emitted=new Map<string,BazelActionV1>(),dependents=new Map<string,number>();\n for(const task of Object.values(plan.tasks))for(const id of task.dependencies)dependents.set(id,(dependents.get(id)??0)+1);',
          'emitted=new Map<string,BazelActionV1>();', after="bzl-lazy"),
    patch("hermetic (rexec) and worker (bworkers) decisions read the planner kind",
          'const base=continuous?"launch":pure?"noop":"task",restore=base!=="noop"', 'const kind=continuous?"launch":pure?"noop":"task",restore=kind!=="noop"', after="bzl-lazy"),
    patch("a hermetic action runs through its own runner, never in a persistent worker",
          'const worker=kind==="task"?bazelActionWorkerV1(task):undefined;', 'const worker=kind==="task"&&!hermetic?bazelActionWorkerV1(task):undefined;'),
    patch("a test is decided once by the neutral coverage disposition (bzl-coverage); cargo actions stay serialized actions (bzl-rust), a test runs in the test runner, never in a worker (bworkers), and a platform profile without runfiles (Windows without symlinks, bzl-xplat) keeps tests cached actions",
          'for(const action of emitted.values())if(action.kind==="task"&&plan.selected.includes(action.id)&&!consumed.has(action.label)&&bazelTaskCoverageV1(plan.tasks[action.id]!,action.cache).disposition==="test")action.kind="test";',
          'const runfiles=!bazelPlatformRulesV1(process.platform).command.includes("--noenable_runfiles");\n for(const action of emitted.values())if(runfiles&&action.kind==="task"&&!action.exclusive&&plan.selected.includes(action.id)&&!consumed.has(action.label)&&bazelTaskCoverageV1(plan.tasks[action.id]!,action.cache).disposition==="test"){action.kind="test";delete action.worker;}',
          after=["bzl-coverage", "bzl-xplat", "bzl-rust", "bworkers"]),
    patch("the lazy mapping records which tests fit the input limit; only those become tests",
          '&&!consumed.has(action.label)&&bazelTaskCoverageV1(', '&&!consumed.has(action.label)&&(tests===undefined||tests.has(action.id))&&bazelTaskCoverageV1(', after="bzl-lazy"),
    patch("the generated load statement names the test rule",
          'const symbols=["repo_launch","repo_noop","repo_task",', 'const symbols=["repo_launch","repo_noop","repo_task",...(input.actions.some(action=>action.kind==="test")?["repo_test"]:[]),', after="bzl-coverage"),
    patch("a test carries the one timeout constant of the command contract, once, and no worker attributes",
          'if(action.kind==="test")attributes.push(["timeout",quote(bazelTestTimeoutV1)]);\n', '', after="bzl-lazy"),
    patch("a test has identity but no worker attributes; the timeout comes from the command contract",
          'else attributes.push(["identity",quote(action.id)],["mnemonic",quote(action.worker!.mnemonic)],["multiplex",action.worker!.multiplex?"True":"False"]);',
          'else{attributes.push(["identity",quote(action.id)]);if(action.worker)attributes.push(["mnemonic",quote(action.worker.mnemonic)],["multiplex",action.worker.multiplex?"True":"False"]);}',
          after="bzl-coverage"),
    patch("the test timeout is the command contract constant", '["timeout",quote("eternal")]', '["timeout",quote(bazelTestTimeoutV1)]', after="bzl-lazy"),
    patch("the test launcher is a host-state action like the task and launcher actions: no shared transpiler cache across roots",
          'mnemonic = "RepoTestLauncher", execution_requirements = _requirements(ctx), use_default_shell_env = True)',
          'mnemonic = "RepoTestLauncher", execution_requirements = _requirements(ctx), use_default_shell_env = True, env = {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0"})', after=["rexec", "bzl-coverage"]),
    patch("hermetic needs are known from the candidates, before the test set exists",
          'hasHermetic=[...emitted.values()].some(action=>action.kind==="hermetic")', 'hasHermetic=[...candidates.values()].some(action=>action.kind==="hermetic")', after="bzl-lazy"),
    patch("the final plan keeps the hermetic switch and applies the recorded test set",
          'const emitted=bazelActionsV1(graph,plan,request.skipCache===true,tests);', 'const emitted=bazelActionsV1(graph,plan,request.skipCache===true,{hermetic:true},tests);', after="bzl-lazy"),
    patch("the job count of the command plan is the worker plan's (bworkers), raised to the executor's when hermetic actions run remotely (rexec)",
          'const jobs=request.parallel??Math.max(1,Math.min(8,require("node:os").availableParallelism())),{workspace,plan,implementation}=', 'const {workspace,plan,implementation}=', after="bzl-lazy"),
    patch("one startup rendering for run and affected: idle bound, shared install base, heap, rc isolation, the cache contract's own output base and the platform profile; the profile replaces the output user root",
          '/** 🚀 Names the startup options of one Bazel server: its output user root, idle bound, shared install base, heap, rc isolation and the cache contract\'s own output base. */\nfunction bazelStartupOptionsV1(root:string,storage:BazelStorageV1,cache:ReturnType<typeof bazelCacheFlagsV1>):string[]{\n const outputRoot=join(storage.paths.outputs,hash(JSON.stringify({version:BAZEL_VERSION,platform:process.platform,arch:process.arch,idle:policy.serverIdleSeconds,jvm:"-Xmx2048m"})).slice(0,16));\n return [`--output_user_root=${outputRoot}`,`--max_idle_secs',
          '/** 🚀 Names the startup options of one Bazel server: idle bound, shared install base, heap, rc isolation, the cache contract\'s own output base and the platform profile. */\nfunction bazelStartupOptionsV1(root:string,storage:BazelStorageV1,cache:ReturnType<typeof bazelCacheFlagsV1>,profile:ReturnType<typeof requireBazelPlatformProfileV1>):string[]{\n return [`--max_idle_secs',
          after=["bzl-lazy", "bzl-xplat"]),
    patch("the startup options carry the profile's flags", '`--bazelrc=${join(root,".bazelrc")}`,...cache.startup];\n}', '`--bazelrc=${join(root,".bazelrc")}`,...cache.startup,...profile.startup];\n}', after=["bzl-lazy", "bzl-xplat"]),
    patch("affected queries start their server under the same platform profile and locale as every other Bazel call",
          'const startup=bazelStartupOptionsV1(root,storage,bazelCacheFlagsV1(storage.paths,workspace,{environment:process.env})),environment={...storage.products.environment,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})};',
          'const cache=bazelCacheFlagsV1(storage.paths,workspace,{environment:process.env}),profile=requireBazelPlatformProfileV1(cache),startup=bazelStartupOptionsV1(root,storage,cache,profile),environment={...storage.products.environment,...profile.environment};',
          after=["bzl-lazy", "bzl-xplat"]),
]


import json as _json

COMMANDS = BZ + "/🧭️commands"


def _exact(text, old, new, why):
    if text.count(old) != 1:
        raise SystemExit(f"semantic edit ({why}): anchor found {text.count(old)} times: {old[:90]!r}")
    return text.replace(old, new)


def _json_edit(mutate, why):
    def run(text):
        data = _json.loads(text)
        tail = "\n" if text.endswith("\n") else ""
        indent = len(text.split("\n")[1]) - len(text.split("\n")[1].lstrip())
        if _json.dumps(data, indent=indent, ensure_ascii=False) + tail != text:
            raise SystemExit(f"semantic json edit ({why}): file is not in the canonical {indent}-space layout")
        mutate(data)
        return _json.dumps(data, indent=indent, ensure_ascii=False) + tail
    return run


def _schema_text(text):
    text = _exact(text, '"kind": { "enum": ["task", "test", "noop", "launch"] }', '"kind": { "enum": ["task", "hermetic", "test", "noop", "launch"] }', "task kinds")
    text = _exact(text, '        "testTargets": "^test(?:-.+)?$",\n', '', "testTargets")
    text = _exact(text, "`testTargets` names the target family that runs with test semantics (a test action also needs a cache decision, an empty `outputs` declaration and no dependents, see `TestAction`). ", "Which task is a test is decided by the neutral coverage contract (family x disposition); a `test` member only selects the `test` verb. ", "description")
    a = text.index(',\n    "TestAction": {')
    return text[:a] + "\n  }\n}\n"


def _fixture(data):
    del data["classification"]
    data["cases"].insert(1, {"name": "a hermetic action is built like a task", "input": {"mode": "execute", "base": ["--lockfile_mode=off"], "jobs": 4, "bail": False, "verbose": False, "skipCache": False, "tasks": [{"label": "//:t_00000000000000b1", "kind": "hermetic"}]}, "expected": {"version": 1, "steps": [{"verb": "build", "targets": ["//:t_00000000000000b1"], "options": ["--lockfile_mode=off", "--jobs=4", "--keep_going"]}]}})


def _commands(text):
    text = _exact(text, 'kind:"task"|"test"|"noop"|"launch"}>', 'kind:"task"|"hermetic"|"test"|"noop"|"launch"}>', "task kinds")
    text = _exact(text, 'const policy=schema.definitions.Policy.const,family=new RegExp(policy.testTargets,"u");', 'const policy=schema.definitions.Policy.const;', "family")
    a = text.index("/** 🧪️ Tells whether a target name belongs to the family")
    b = text.index("/** 📏️ Tells whether a test action declares few enough")
    text = text[:a] + text[b:]
    a = text.index("/** 🔬️ Decides whether a task action runs as a Bazel test")
    b = text.index("/** 🗺️ Maps a selected batch")
    return text[:a] + text[b:]


def _commands_test(text):
    text = _exact(text, "import {bazelCommandPlanV1,bazelTestActionV1,bazelTestTargetV1,bazelTestWithinLimitV1}", "import {bazelCommandPlanV1,bazelTestWithinLimitV1}", "imports")
    text = _exact(text, ',plan=compile("Plan"),action=compile("TestAction");', ',plan=compile("Plan");', "compile")
    text = _exact(text, ' for(const row of fixture.classification)expect(action({target:row.target,cache:row.cache,outputs:row.outputs,dependents:row.dependents}),row.target).toBe(true);\n', '', "classification schema")
    text = _exact(text, ' for(const row of fixture.classification){const facts={target:row.target,cache:row.cache,outputs:row.outputs,dependents:row.dependents};expect(bazelTestActionV1(facts),row.target).toBe(row.expected);}\n expect(bazelTestTargetV1("test-snapshot-sqlite")).toBe(true);\n expect(bazelTestTargetV1("contest")).toBe(false);\n', '', "classification")
    return _exact(text, 'test("test actions are classified by the authored table",()=>{', 'test("the input limit of a Bazel test is authored",()=>{', "title")


PATCHES += [
    {"why": "bzl-lazy commands: one test authority (neutral coverage contract); hermetic actions are build members", "rel": COMMANDS + "/🟦️.ts", "fn": _commands, "after": "bzl-lazy"},
    {"why": "bzl-lazy commands schema: no second test-name policy, hermetic kind", "rel": COMMANDS + "/🧬️schema/🔣️.json", "fn": _schema_text, "after": "bzl-lazy"},
    {"why": "bzl-lazy commands fixture: drop the classification table, add the hermetic case", "rel": COMMANDS + "/🧪️tests/🧫️fixtures/🔣️.json", "fn": _json_edit(_fixture, "commands fixture"), "after": "bzl-lazy"},
    {"why": "bzl-lazy commands test follows the module", "rel": COMMANDS + "/🧪️tests/🟦️.ts", "fn": _commands_test, "after": "bzl-lazy"},
]


EXCLUDES = {
    BZ + "/🧪️tests/📥️test-actions/🟦️.ts": "bzl-lazy test of its shell-launcher repo_test; the integrated design keeps bzl-coverage's cross-platform repo_test (covered by 📥️coverage and 📥️actions)",
}

PATCHES += [
    patch("register the adapter-local suites of the sets next to the hermetic suites in the Bazel test index",
          'import "../🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts";\n',
          'import "../🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts";\nimport "../🦀️cargo/🧪️tests/🟦️.ts";\nimport "../👷️workers/🧪️tests/🟦️.ts";\nimport "./📥️coverage/🟦️.ts";\nimport "./📥️platform/🟦️.ts";\n',
          rel="BZ/🧪️tests/🟦️.ts", after="bzl-xplat"),
    patch("register the lazy-mapping suites", 'import "./📥️coverage/🟦️.ts";\n', 'import "./📥️coverage/🟦️.ts";\nimport "../🧭️commands/🧪️tests/🟦️.ts";\nimport "../🎯️affected/🧪️tests/🟦️.ts";\n', rel="BZ/🧪️tests/🟦️.ts", after="bzl-lazy"),
]


B = L + "/🏗️build"
RC = B + "/🌐️remote-cache"
README = "🧰️framework/🛍️products/🦑️repo/README.md"


def _rc_fixture(data):
    for row in data["bazel"]:
        row.pop("restoresOutputs", None)
    data["bazel"] = [row for row in data["bazel"] if row["name"] not in ("minimal is refused when the adapter restores snapshots", "minimal is rendered when nothing is restored")]
    data["contract"]["refused"].append({"name": "minimal download would break the restore of cached outputs", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "http://h", "SEMIO_REMOTE_CACHE_DOWNLOAD": "minimal"}})


PATCHES += [
    patch("coordinator: the download mode of the contract is toplevel (default) or all; minimal leaves the snapshot of a cache hit unfetched and the Bazel restore throws ENOENT (cache-fix P8)",
          'Which outputs a build fetches from the cache: minimal only what a later action needs, toplevel the outputs of the requested targets, all every output. Nx always fetches everything; Bazel\'s task adapter restores the snapshots of the requested targets and therefore refuses minimal.",\n      "enum": ["minimal", "toplevel", "all"]',
          'Which outputs a build fetches from the cache: toplevel the outputs of the requested targets, all every output. Nx always fetches everything; Bazel\'s task adapter restores the snapshots of the requested targets from bazel-bin, so a mode that leaves them in the remote store (minimal) is not part of the contract: the restore would fail with ENOENT.",\n      "enum": ["toplevel", "all"]',
          rel="L/🏗️build/🏃️execution/🧬️schema/🌐️remote-cache.json", after="rcache"),
    patch("remote cache contract type follows the schema", 'download:"minimal"|"toplevel"|"all";', 'download:"toplevel"|"all";', rel=RC + "/🟦️.ts", after="rcache"),
    patch("the Bazel rendering needs no restore special case once the contract cannot say minimal", 'options:Readonly<{helperDirectory:string;restoresOutputs?:boolean;platform?:NodeJS.Platform}>', 'options:Readonly<{helperDirectory:string;platform?:NodeJS.Platform}>', rel=BZ + "/🌐️remote-cache/🟦️.ts", after="rcache"),
    patch("the refusal moved into the schema", ' if(options.restoresOutputs&&contract.download==="minimal")throw Error("Bazel task adapter restores snapshots from bazel-bin and needs remote cache download toplevel or all");\n', '', rel=BZ + "/🌐️remote-cache/🟦️.ts", after="rcache"),
    patch("docstring follows", ' `restoresOutputs` refuses `minimal`, since the task adapter restores the snapshots of the requested targets from bazel-bin. */', ' The contract has no `minimal` download, since the task adapter restores the snapshots of the requested targets from bazel-bin. */', rel=BZ + "/🌐️remote-cache/🟦️.ts", after="rcache"),
    patch("remote cache test follows", 'helperDirectory:row.directory,restoresOutputs:row.restoresOutputs,platform', 'helperDirectory:row.directory,platform', rel=RC + "/🧪️tests/🟦️.ts", after="rcache"),
    patch("remote cache test follows", '{helperDirectory:folder,restoresOutputs:true,platform:"linux"}', '{helperDirectory:folder,platform:"linux"}', rel=RC + "/🧪️tests/🟦️.ts", after="rcache"),
    patch("remote cache test follows", '{helperDirectory:base,restoresOutputs:true}', '{helperDirectory:base}', rel=RC + "/🧪️tests/🟦️.ts", after="rcache"),
    patch("storage follows", '{helperDirectory:layout.temporary,restoresOutputs:true}', '{helperDirectory:layout.temporary}', rel=BZ + "/💾️storage/🟦️.ts", after="rcache"),
    {"why": "remote cache fixture: no minimal rows, a refused minimal contract", "rel": RC + "/🧪️tests/🧫️fixtures/🔣️.json", "fn": _json_edit(_rc_fixture, "remote cache fixture"), "after": "rcache"},
    patch("Bazel storage fixture: the refusal is the contract's", '{"name": "the task adapter restores snapshots, so minimal download is refused", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "http://127.0.0.1:6090", "SEMIO_REMOTE_CACHE_DOWNLOAD": "minimal"}, "message": "needs remote cache download toplevel or all"}',
          '{"name": "minimal download is refused by the contract, since the task adapter restores snapshots", "environment": {"SEMIO_REMOTE_CACHE_ENDPOINT": "http://127.0.0.1:6090", "SEMIO_REMOTE_CACHE_DOWNLOAD": "minimal"}, "message": "Closed remote cache contract required"}',
          rel=BZ + "/💾️storage/🧪️tests/🧫️fixtures/🔣️.json", after="rcache"),
    patch("catalogue note", '_DOWNLOAD (minimal|toplevel|all)', '_DOWNLOAD (toplevel|all)', rel="L/🏗️build/⚙️operations/🔣️catalog.json", after="rcache"),
    patch("README (English)", '`download`: `minimal`, `toplevel` or `all`', '`download`: `toplevel` or `all`', rel=README, after="rcache"),
    patch("README (German)", '`download`: `minimal`, `toplevel` oder `all`', '`download`: `toplevel` oder `all`', rel=README, after="rcache"),
]


MISSING_FIXTURE = '''{
 "version": 1,
 "cases": [
  {"name": "a cached task must produce what it declares", "definition": {"cache": true}, "refused": true},
  {"name": "an uncached task may leave a declared output unwritten", "definition": {"cache": false}, "refused": false},
  {"name": "a task without a cache declaration is not replayed", "definition": {}, "refused": false},
  {"name": "optionalOutputs lets a cached task skip an output", "definition": {"cache": true, "metadata": {"semio": {"optionalOutputs": true}}}, "refused": false},
  {"name": "optionalOutputs must be the boolean true", "definition": {"cache": true, "metadata": {"semio": {"optionalOutputs": "yes"}}}, "refused": true},
  {"name": "other semio metadata does not relax the rule", "definition": {"cache": true, "metadata": {"semio": {"hermetic": {"version": 1, "tools": ["bun"]}}}}, "refused": true}
 ]
}
'''

MISSING_TEST = '''/** 🔒 A cached Bazel task must produce every output it declares: the fixture is the contract, an independent Python implementation of the rule is the oracle. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import fixture from "./🔣️.json" with {type:"json"};
import {bazelMissingOutputRefusedV1} from "../../📜️script.ts";

const python=`
import json,sys
def refused(definition):
    semio=(definition.get("metadata") or {}).get("semio") or {}
    return definition.get("cache") is True and semio.get("optionalOutputs") is not True
print(json.dumps([refused(row["definition"]) for row in json.load(sys.stdin)["cases"]]))
`;

test("every fixture case is refused or admitted exactly as authored",()=>{
 for(const row of fixture.cases)expect(bazelMissingOutputRefusedV1(row.definition),row.name).toBe(row.refused);
});

test("an independent Python implementation of the rule agrees on every case",()=>{
 const run=spawnSync("python3",["-I","-c",python],{input:JSON.stringify(fixture),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(fixture.cases.map(row=>row.refused));
});
'''

CREATES = {
    BZ + "/🧪️tests/📥️outputs/🟦️.ts": MISSING_TEST,
    BZ + "/🧪️tests/📥️outputs/🔣️.json": MISSING_FIXTURE,
}

PATCHES += [
    patch("coordinator: a declared output that a cached task did not produce is an error by default (cache-fix finding 2); `metadata.semio.optionalOutputs: true` is the explicit opt-out",
          '}else try{await lstat(workspacePath(manifest.workspace,path));roots.add(path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}}',
          '}else try{await lstat(workspacePath(manifest.workspace,path));roots.add(path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;if(bazelMissingOutputRefusedV1(manifest.task.definition))throw new Error(`Declared output of a replayed Bazel task was not produced: ${manifest.task.id} ${path}`);}}'),
    patch("the rule of the refusal is one pure function",
          '/** 📦️ Flattens declared products so cache paths remain short on every host. */\nexport async function captureBazelSnapshotV1(',
          '/** 🔒 Tells whether a declared output that a run did not produce is an error: a cached task must produce what it declares unless it declares `metadata.semio.optionalOutputs: true`, because an unproduced output is cached as present and lost on every cache hit. */\nexport function bazelMissingOutputRefusedV1(definition:{cache?:unknown;metadata?:unknown}):boolean{\n const semio=(definition.metadata as {semio?:{optionalOutputs?:unknown}}|undefined)?.semio;\n return definition.cache===true&&semio?.optionalOutputs!==true;\n}\n\n/** 📦️ Flattens declared products so cache paths remain short on every host. */\nexport async function captureBazelSnapshotV1('),
    patch("register the output suite", 'import "./📥️coverage/🟦️.ts";\n', 'import "./📥️coverage/🟦️.ts";\nimport "./📥️outputs/🟦️.ts";\n', rel="BZ/🧪️tests/🟦️.ts", after="bzl-xplat"),
]


FINGERPRINT_TEST = BZ + "/🧪️tests/📥️fingerprint/🟦️.ts"

PATCHES += [
    patch("the fingerprint law extracts taskFingerprint and injects its dependencies by name: the Cargo toolchain key of bzl-rust is not part of a non-Cargo fixture task",
          'bindings={createHash,randomUUID,', 'bindings={createHash,randomUUID,bazelCargoClassV1:()=>undefined,bazelCargoToolchainDigestV1:async()=>"",', rel=FINGERPRINT_TEST, count=2, after="bzl-rust"),
    patch("the generated rule set now has three plain requirement uses (launcher, test launcher, test execution info) beside the one worker action",
          'expect(rules.match(/_requirements\\(ctx\\)/gu)?.length).toBe(1);', 'expect(rules.match(/_requirements\\(ctx\\)/gu)?.length).toBe(3);', rel=BZ + "/👷️workers/🧪️tests/🟦️.ts", after=["bworkers", "bzl-coverage"]),
    patch("catalogue examples follow the neutral `repo` spelling the catalogue test requires", '{ "command": "bun repo cache serve",', '{ "command": "repo cache serve",', rel="L/🏗️build/⚙️operations/🔣️catalog.json", after="rcache"),
    patch("catalogue examples follow the neutral `repo` spelling the catalogue schema requires (`^repo `); the endpoint variable is described in the notes",
          'SEMIO_REMOTE_CACHE_ENDPOINT=http://127.0.0.1:6090 bun repo cache remote status --tool=bazel', 'repo cache remote status --tool=bazel', rel="L/🏗️build/⚙️operations/🔣️catalog.json", after="rcache"),
]
