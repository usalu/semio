#!/usr/bin/env python3
# Authors staged/fixture.json: language-agnostic cases whose expected answers are written by hand here, never computed by an implementation.
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
H = lambda c: c * 64
T = lambda en, de: {"en": en, "de": de}
def plan(tasks, deps=None, roots=None, cont=None):
    tasks = {k: {"id": k, "target": ({"project": v[0], "target": v[1]} | ({"configuration": v[2]} if len(v) > 2 else {}))} for k, v in tasks.items()}
    out = {"roots": roots if roots is not None else [], "tasks": tasks, "dependencies": deps or {k: [] for k in tasks}}
    if cont is not None: out["continuousDependencies"] = cont
    return out
def F(mode=0o644, sha="a", size=3, **kw): return {"kind": "file", "mode": mode, "size": size, "sha256": H(sha)} | kw
D = lambda mode=0o755: {"kind": "directory", "mode": mode}
def rule(id, kind, surface, **kw): return {"id": id, "kind": kind, "surface": surface, **kw, "justification": T("Documented in the fixture.", "Im Fixture dokumentiert."), "evidence": "fixture"}
RULES = {"version": 1, "extractors": {"failedTasksHeading": "^\\s*(?:NX\\s+)?Failed tasks:?\\s*$", "failedTask": "^\\s*-\\s+(\\S+:\\S+)\\s*$", "testTarget": "^test(?:-.+)?$"}, "rules": []}
def rules(*rs): return RULES | {"rules": list(rs)}
div = lambda check, code, subject, nx=None, bazel=None: {"check": check, "code": code, "subject": subject} | ({"nx": nx} if nx is not None else {}) | ({"bazel": bazel} if bazel is not None else {})

glob = [
 ("a/*.ts", "a/b.ts", True, True), ("a/*.ts", "a/b/c.ts", False, True), ("**/*.ts", "x/y/z.ts", True, True), ("**/*.ts", "z.ts", True, True),
 ("a/**", "a/b/c", True, True), ("t_*.json", "t_abc.json", True, True), ("t_*.json", "t_abc.jsonx", False, True), ("MODULE.bazel*", "MODULE.bazel.lock", True, True),
 ("?.txt", "a.txt", True, True), ("?.txt", "ab.txt", False, True), ("a.b", "aXb", False, True), ("src/(x)/*", "src/(x)/f", True, False),
 ("**/dist/**", "p/dist/a/b.js", True, True), ("**/dist/**", "p/distx/a.js", False, True), ("🧾️store/*.json", "🧾️store/build.json", True, True),
 ("a+b/*", "a+b/c", True, False), ("*", "a/b", False, True), ("**", "a/b", True, True), ("bazel-out/**", "bazel-out/k8/bin/x", True, True), ("s/**/*.ts", "s/a.ts", True, True),
]
fixture = {"globCases": [{"glob": g, "path": p, "match": m, "bun": b} for g, p, m, b in glob]}

task = {"project": "p", "root": "a/b", "execution": {"kind": "command", "env": {"SEMIO_BUILD_MODE": "release"}, "name": "n"}}
fixture["outputPatternCases"] = [
 {"patterns": ["{projectRoot}/dist", "{workspaceRoot}/out/{projectName}"], "task": task, "environment": {}, "expected": ["a/b/dist", "out/p"]},
 {"patterns": ["{projectRoot}/dist", "{projectRoot}/dist/x"], "task": task, "environment": {}, "expected": ["a/b/dist"]},
 {"patterns": ["{workspaceRoot}/bin/{options.env.SEMIO_BUILD_MODE}/x"], "task": task, "environment": {}, "expected": ["bin/release/x"]},
 {"patterns": ["{workspaceRoot}/bin/{options.env.SEMIO_BUILD_MODE}/x"], "task": task | {"execution": {"kind": "command"}}, "environment": {"SEMIO_BUILD_MODE": "prototype"}, "expected": ["bin/prototype/x"]},
 {"patterns": ["{projectRoot}/../../gen"], "task": task, "environment": {}, "expected": ["gen"]},
 {"patterns": ["{projectRoot}/dist"], "task": task | {"root": "."}, "environment": {}, "expected": ["dist"]},
 {"patterns": ["{projectRoot}/{options.name}"], "task": task, "environment": {}, "expected": ["a/b/n"]},
 {"patterns": ["{workspaceRoot}/🧾️store/x"], "task": task, "environment": {}, "expected": ["🧾️store/x"]},
]
fixture["outputPatternRefused"] = [
 {"patterns": ["{projectRoot}/../../../x"], "task": task, "environment": {}},
 {"patterns": ["{options.missing}"], "task": task, "environment": {}},
 {"patterns": ["{workspaceRoot}"], "task": task, "environment": {}},
 {"patterns": ["{options.env.UNSET}/x"], "task": task, "environment": {}},
]

r_text = rule("stamp", "text-replace", "output", glob="**/*.txt", pattern="built at [0-9:-]+", flags="g", replacement="built at <time>")
r_json = rule("json-time", "json-redact", "output", glob="**/*.json", pointers=["/meta/generatedAt", "/meta/host"])
fixture["normalizeCases"] = [
 {"name": "text replace equal after replacing", "path": "a/x.txt", "rules": rules(r_text), "left": "hello built at 10:20:30\nbye", "right": "hello built at 11:21:31\nbye", "equal": True},
 {"name": "text replace keeps other differences", "path": "a/x.txt", "rules": rules(r_text), "left": "hello built at 10:20:30", "right": "HELLO built at 11:21:31", "equal": False},
 {"name": "text replace replaces every occurrence with g", "path": "x.txt", "rules": rules(r_text), "left": "built at 1 built at 2", "right": "built at 3 built at 4", "equal": True},
 {"name": "glob keeps the rule away from other paths", "path": "a/x.md", "rules": rules(r_text), "left": "built at 1", "right": "built at 2", "equal": None},
 {"name": "json redact ignores key order and redacted fields", "path": "d/o.json", "rules": rules(r_json), "left": '{"meta":{"generatedAt":"1","host":"h1"},"v":[1,2],"a":true}', "right": '{"a":true,"v":[1,2],"meta":{"host":"h2","generatedAt":"2"}}', "equal": True},
 {"name": "json redact keeps unredacted differences", "path": "d/o.json", "rules": rules(r_json), "left": '{"meta":{"generatedAt":"1"},"v":[1,2]}', "right": '{"meta":{"generatedAt":"2"},"v":[1,3]}', "equal": False},
 {"name": "json redact skips invalid json", "path": "d/o.json", "rules": rules(r_json), "left": "{not json", "right": "{not json!", "equal": None},
 {"name": "json redact tolerates missing pointers", "path": "d/o.json", "rules": rules(r_json), "left": '{"v":1}', "right": '{"v":1}', "equal": True},
]

p_a = plan({"p:build": ("p", "build"), "q:gen": ("q", "gen")}, {"p:build": ["q:gen"], "q:gen": []}, ["q:gen"])
p_a_cfg = plan({"p:build:prototype": ("p", "build", "prototype"), "q:gen": ("q", "gen")}, {"p:build:prototype": ["q:gen"], "q:gen": []}, ["q:gen"])
p_b_cfg = plan({"p:build:prototype": ("p", "build", "prototype"), "q:gen": ("q", "gen")}, {"p:build:prototype": ["q:gen"], "q:gen": []}, ["q:gen"])
fixture["graphCases"] = [
 {"name": "equal graphs", "nx": p_a, "bazel": p_a, "expected": []},
 {"name": "different task id spelling with equal targets", "nx": p_a_cfg, "bazel": plan({"x1": ("p", "build", "prototype"), "x2": ("q", "gen")}, {"x1": ["x2"], "x2": []}, ["x2"]), "expected": []},
 {"name": "task missing in bazel", "nx": p_a, "bazel": plan({"q:gen": ("q", "gen")}, None, ["q:gen"]), "expected": [div("graph", "edge-missing-bazel", "p:build -> q:gen", "present", "absent"), div("graph", "task-missing-bazel", "p:build")]},
 {"name": "task missing in nx", "nx": plan({"q:gen": ("q", "gen")}, None, ["q:gen"]), "bazel": p_a, "expected": [div("graph", "edge-missing-nx", "p:build -> q:gen", "absent", "present"), div("graph", "task-missing-nx", "p:build")]},
 {"name": "edge only in nx", "nx": p_a, "bazel": plan({"p:build": ("p", "build"), "q:gen": ("q", "gen")}, None, ["q:gen", "p:build"]), "expected": [div("graph", "edge-missing-bazel", "p:build -> q:gen", "present", "absent"), div("graph", "root-differs", "roots", "q:gen", "p:build,q:gen")]},
 {"name": "continuous edge differs", "nx": plan({"s:serve": ("s", "serve"), "t:e2e": ("t", "e2e")}, None, ["s:serve"], {"t:e2e": ["s:serve"]}), "bazel": plan({"s:serve": ("s", "serve"), "t:e2e": ("t", "e2e")}, None, ["s:serve"], {"t:e2e": []}), "expected": [div("graph", "continuous-edge-differs", "t:e2e -> s:serve", "present", "absent")]},
 {"name": "plan unavailable", "nx": None, "bazel": p_a, "expected": [div("graph", "plan-unavailable", "nx")]},
]
fixture["closureCases"] = [
 {"name": "query equals plan", "plan": p_a, "closure": {"queried": ["p:build", "q:gen"]}, "expected": []},
 {"name": "query lacks a planned task and adds another", "plan": p_a, "closure": {"queried": ["q:gen", "z:extra"]}, "expected": [div("closure", "closure-task-extra-in-query", "z:extra"), div("closure", "closure-task-missing-in-query", "p:build")]},
 {"name": "no query", "plan": p_a, "closure": None, "expected": [div("closure", "closure-unavailable", "cquery")]},
]

same = {"roots": {"p:build": ["dist"]}, "files": {"dist": D(), "dist/a.js": F(sha="a"), "dist/run.sh": F(0o755, "b"), "dist/l": {"kind": "symlink", "mode": 0o755, "link": "a.js"}}}
def varied(path, rec, drop=None):
    files = {k: v for k, v in same["files"].items() if k != drop}
    files[path] = rec
    return {"roots": same["roots"], "files": files}
mask = rule("exec-only", "mode-mask", "output", mask=0o111)
ign = rule("skip-map", "ignore-glob", "output", glob="**/*.map")
fixture["outputCases"] = [
 {"name": "equal outputs", "nx": same, "bazel": same, "rules": rules(), "expected": [], "applied": {}},
 {"name": "file only in nx", "nx": varied("dist/extra", F(sha="c")), "bazel": same, "rules": rules(), "expected": [div("outputs", "output-missing-bazel", "dist/extra", "file", "absent")], "applied": {}},
 {"name": "file only in bazel", "nx": same, "bazel": varied("dist/extra", F(sha="c")), "rules": rules(), "expected": [div("outputs", "output-missing-nx", "dist/extra", "absent", "file")], "applied": {}},
 {"name": "kind differs", "nx": same, "bazel": varied("dist/a.js", D()), "rules": rules(), "expected": [div("outputs", "output-kind-differs", "dist/a.js", "file", "directory")], "applied": {}},
 {"name": "link target differs", "nx": same, "bazel": varied("dist/l", {"kind": "symlink", "mode": 0o755, "link": "b.js"}), "rules": rules(), "expected": [div("outputs", "output-link-differs", "dist/l", "a.js", "b.js")], "applied": {}},
 {"name": "execute bit differs", "nx": same, "bazel": varied("dist/run.sh", F(0o644, "b")), "rules": rules(), "expected": [div("outputs", "output-mode-differs", "dist/run.sh", "755", "644")], "applied": {}},
 {"name": "other permission bits differ without a mask rule", "nx": same, "bazel": varied("dist/a.js", F(0o600, "a")), "rules": rules(), "expected": [div("outputs", "output-mode-differs", "dist/a.js", "644", "600")], "applied": {}},
 {"name": "mask rule explains other permission bits", "nx": same, "bazel": varied("dist/a.js", F(0o600, "a")), "rules": rules(mask), "expected": [], "applied": {"exec-only": 1}},
 {"name": "mask rule still sees the execute bit", "nx": same, "bazel": varied("dist/run.sh", F(0o644, "b")), "rules": rules(mask), "expected": [div("outputs", "output-mode-differs", "dist/run.sh", "111", "0")], "applied": {}},
 {"name": "bytes differ", "nx": same, "bazel": varied("dist/a.js", F(sha="d", size=4)), "rules": rules(), "expected": [div("outputs", "output-bytes-differ", "dist/a.js", "3 " + "a" * 12, "4 " + "d" * 12)], "applied": {}},
 {"name": "bytes differ but a rule normalizes both equal", "nx": varied("dist/a.js", F(sha="d", normalized={"stamp": H("e")})), "bazel": varied("dist/a.js", F(sha="f", normalized={"stamp": H("e")})), "rules": rules(r_text), "expected": [], "applied": {"stamp": 1}},
 {"name": "bytes differ and the normalized digests differ too", "nx": varied("dist/a.js", F(sha="d", normalized={"stamp": H("e")})), "bazel": varied("dist/a.js", F(sha="f", normalized={"stamp": H("1")})), "rules": rules(r_text), "expected": [div("outputs", "output-bytes-differ", "dist/a.js", "3 " + "d" * 12, "3 " + "f" * 12)], "applied": {}},
 {"name": "ignored paths are neither missing nor different", "nx": varied("dist/a.js.map", F(sha="d")), "bazel": same, "rules": rules(ign), "expected": [], "applied": {"skip-map": 1}},
 {"name": "roots attributed to different tasks", "nx": same, "bazel": {"roots": {"p:build": ["dist", "extra"]}, "files": same["files"]}, "rules": rules(), "expected": [div("outputs", "output-root-differs", "p:build", "dist", "dist,extra")], "applied": {}},
]
fixture["restoreCases"] = [
 {"name": "restore equal", "fresh": same, "restored": same, "rules": rules(), "expected": []},
 {"name": "restore changes bytes", "fresh": same, "restored": varied("dist/a.js", F(sha="d", size=9)), "rules": rules(), "expected": [div("restore", "restore-bytes-differ", "dist/a.js", "3 " + "a" * 12, "9 " + "d" * 12)]},
 {"name": "restore forgets a file", "fresh": same, "restored": {"roots": same["roots"], "files": {k: v for k, v in same["files"].items() if k != "dist/a.js"}}, "rules": rules(), "expected": [div("restore", "restore-missing", "dist/a.js", "file", "absent")]},
]

run = lambda exit=0, reason="exit", failed=None, tests=None: {"exit": exit, "reason": reason, "failedTasks": failed or [], "tests": tests or {}}
fixture["resultCases"] = [
 {"name": "equal passing runs", "nx": run(tests={"p:test": "passed"}), "bazel": run(tests={"p:test": "passed"}), "expectExit": 0, "expected": []},
 {"name": "exit codes differ", "nx": run(0), "bazel": run(1), "expectExit": 0, "expected": [div("results", "exit-differs", "exit", "0", "1"), div("results", "exit-unexpected", "bazel expected 0", None, "1")]},
 {"name": "both fail as expected", "nx": run(1, failed=["p:test"], tests={"p:test": "failed"}), "bazel": run(1, failed=["p:test"], tests={"p:test": "failed"}), "expectExit": 1, "expected": []},
 {"name": "both fail but the sample expects success", "nx": run(1), "bazel": run(1), "expectExit": 0, "expected": [div("results", "exit-unexpected", "bazel expected 0", None, "1"), div("results", "exit-unexpected", "nx expected 0", "1")]},
 {"name": "failed tasks differ", "nx": run(1, failed=["a:test"]), "bazel": run(1, failed=["b:test"]), "expectExit": 1, "expected": [div("results", "failed-tasks-differ", "failedTasks", "a:test", "b:test")]},
 {"name": "reasons differ", "nx": run(None, "timeout"), "bazel": run(None, "cancelled"), "expectExit": 0, "expected": [div("results", "exit-unexpected", "bazel expected 0", None, "null"), div("results", "exit-unexpected", "nx expected 0", "null"), div("results", "reason-differs", "reason", "timeout", "cancelled")]},
 {"name": "test statuses differ and absent counts as no status", "nx": run(0, tests={"p:test": "passed", "q:test": "passed"}), "bazel": run(0, tests={"p:test": "failed"}), "expectExit": 0, "expected": [div("results", "test-status-differs", "p:test", "passed", "failed"), div("results", "test-status-differs", "q:test", "passed", "no-status")]},
 {"name": "run unavailable", "nx": None, "bazel": run(), "expectExit": 0, "expected": [div("results", "run-unavailable", "nx")]},
]

nxi = lambda files=(), env=(), runtime=(), external=(): {"files": list(files), "environment": list(env), "runtime": list(runtime), "depOutputs": [], "external": list(external)}
spawn = lambda inputs=(), env=(), exit=0: {"inputs": list(inputs), "env": list(env), "exit": exit}
strip = rule("src", "strip-prefix", "input-bazel", prefix="s/")
infra = rule("infra", "ignore-glob", "input-bazel", glob="BUILD.bazel")
nxdoc = rule("nx-readme", "ignore-glob", "input-nx", glob="**/README.md")
amb = rule("ambient", "ambient-env", "env-bazel", names=["PATH", "SEMIO_*"])
cov = rule("lock", "covered-by", "input-nx", category="external", anyOf=["bun.lock"])
cov_rt = rule("tool", "covered-by", "input-nx", category="runtime", anyOf=["external/bun/**"])
fixture["inputCases"] = [
 {"name": "equal after stripping and infrastructure", "nx": {"p:t": nxi(["a/x.ts", "a/y.ts"])}, "bazel": {"p:t": spawn(["s/a/x.ts", "s/a/y.ts", "BUILD.bazel"])}, "rules": rules(strip, infra), "expected": [], "applied": {"src": 2, "infra": 1}},
 {"name": "file only in nx", "nx": {"p:t": nxi(["a/x.ts", "a/z.ts"])}, "bazel": {"p:t": spawn(["s/a/x.ts"])}, "rules": rules(strip), "expected": [div("inputs", "input-file-only-nx", "p:t a/z.ts")], "applied": {"src": 1}},
 {"name": "file only in bazel", "nx": {"p:t": nxi(["a/x.ts"])}, "bazel": {"p:t": spawn(["s/a/x.ts", "s/a/w.ts"])}, "rules": rules(strip), "expected": [div("inputs", "input-file-only-bazel", "p:t a/w.ts")], "applied": {"src": 2}},
 {"name": "nx-only documentation is ignored by rule", "nx": {"p:t": nxi(["a/x.ts", "a/README.md"])}, "bazel": {"p:t": spawn(["s/a/x.ts"])}, "rules": rules(strip, nxdoc), "expected": [], "applied": {"src": 1, "nx-readme": 1}},
 {"name": "environment hashed by nx but invisible to bazel", "nx": {"p:t": nxi(env=["TOKEN"])}, "bazel": {"p:t": spawn(env=["PATH"])}, "rules": rules(amb), "expected": [div("inputs", "env-hashed-not-visible", "p:t TOKEN")], "applied": {"ambient": 1}},
 {"name": "environment visible to bazel and explained by an ambient rule", "nx": {"p:t": nxi()}, "bazel": {"p:t": spawn(env=["PATH", "SEMIO_PROJECT"])}, "rules": rules(amb), "expected": [], "applied": {"ambient": 2}},
 {"name": "environment visible to bazel without a rule", "nx": {"p:t": nxi()}, "bazel": {"p:t": spawn(env=["SECRET"])}, "rules": rules(amb), "expected": [div("inputs", "env-visible-not-hashed", "p:t SECRET")], "applied": {}},
 {"name": "external packages covered by the lock file", "nx": {"p:t": nxi(["bun.lock"], external=["npm:left-pad@1"])}, "bazel": {"p:t": spawn(["bun.lock"])}, "rules": rules(cov), "expected": [], "applied": {"lock": 1}},
 {"name": "external packages without a lock file", "nx": {"p:t": nxi(["a.ts"], external=["npm:left-pad@1"])}, "bazel": {"p:t": spawn(["a.ts"])}, "rules": rules(cov), "expected": [div("inputs", "external-uncovered", "p:t", "npm:left-pad@1")], "applied": {}},
 {"name": "runtime value without a matching toolchain", "nx": {"p:t": nxi(["a.ts"], runtime=["bun --version"])}, "bazel": {"p:t": spawn(["a.ts"])}, "rules": rules(cov_rt), "expected": [div("inputs", "runtime-uncovered", "p:t", "bun --version")], "applied": {}},
 {"name": "task known to one tool only", "nx": {"p:t": nxi()}, "bazel": {}, "rules": rules(), "expected": [div("inputs", "inputs-unavailable", "p:t", "present", "absent")], "applied": {}},
 {"name": "inputs unavailable", "nx": None, "bazel": {"p:t": spawn()}, "rules": rules(), "expected": [div("inputs", "inputs-unavailable", "nx")], "applied": {}},
]
fixture["parserCases"] = {
 "plans": [
  {"name": "plan between log lines", "text": "[nx] starting\n{\"tasks\":{\"roots\":[\"a:b\"],\"tasks\":{\"a:b\":{\"id\":\"a:b\",\"target\":{\"project\":\"a\",\"target\":\"b\"}}},\"dependencies\":{\"a:b\":[]}},\"graph\":{}}\nDone\n", "expected": plan({"a:b": ("a", "b")}, None, ["a:b"])},
  {"name": "braces inside strings", "text": "{\"note\":\"}{\"}\n{\"tasks\":{\"roots\":[],\"tasks\":{},\"dependencies\":{}}}\n", "expected": plan({}, None, [])},
  {"name": "no plan", "text": "error: nothing\n{\"other\":1}\n", "expected": None},
 ],
 "nxInputs": [
  {"name": "two tasks in request order", "ids": ["a:b", "c:d:release"], "text": "{\n  \"project\": \"a\",\n  \"target\": \"b\",\n  \"files\": [\"z\", \"y\"],\n  \"runtime\": [\"node -v\"],\n  \"environment\": [\"E\"],\n  \"depOutputs\": [],\n  \"external\": []\n}\nnoise\n{\"project\":\"c\",\"target\":\"d\",\"files\":[\"q\"],\"runtime\":[],\"environment\":[],\"depOutputs\":[\"o\"],\"external\":[\"npm:x@1\"]}\n", "expected": {"a:b": nxi(["y", "z"], ["E"], ["node -v"]) , "c:d:release": nxi(["q"], external=["npm:x@1"]) | {"depOutputs": ["o"]}}},
  {"name": "custom hasher warning is skipped", "ids": ["a:b"], "text": "{\"project\":\"a\",\"target\":\"b\",\"warning\":\"custom\"}\n", "expected": {}},
 ],
 "executionLog": [
  {"name": "spawns with inputs and environment, one per line", "text": "{\"targetLabel\":\"//:t_1\",\"mnemonic\":\"RepoTask\",\"runner\":\"local\",\"exitCode\":0,\"environmentVariables\":[{\"name\":\"PATH\",\"value\":\"/bin\"},{\"name\":\"A\",\"value\":\"1\"}],\"inputs\":[{\"path\":\"s/a.ts\"},{\"path\":\"BUILD.bazel\"}]}\nnot json\n{\"targetLabel\":\"//:t_1\",\"mnemonic\":\"TestRunner\",\"exitCode\":3,\"inputs\":[{\"path\":\"s/b.ts\"}],\"cacheHit\":true}\n{\"mnemonic\":\"NoLabel\"}\n{\"targetLabel\":\"//:t_2\",\"exitCode\":0,\"inputs\":[]}\n", "labelToTask": {"//:t_1": "p:t", "//:t_2": "q:u"}, "declaredEnvironment": {"//:t_1": ["DECLARED"]}, "expected": {"p:t": {"inputs": ["BUILD.bazel", "s/a.ts", "s/b.ts"], "env": ["A", "DECLARED", "PATH"], "exit": 3}, "q:u": {"inputs": [], "env": [], "exit": 0}}},
  {"name": "pretty printed spawn exactly as Bazel 8.4.2 writes it", "text": "{\n  \"commandArgs\": [\"/bin/bash\", \"-c\", \"cat $@ \\u003e out\", \"\", \"a.txt\"],\n  \"environmentVariables\": [{\n    \"name\": \"FOO\",\n    \"value\": \"bar\"\n  }],\n  \"platform\": {\n    \"properties\": []\n  },\n  \"inputs\": [{\n    \"path\": \"a.txt\",\n    \"digest\": {\n      \"hash\": \"5891\",\n      \"sizeBytes\": \"6\",\n      \"hashFunctionName\": \"SHA-256\"\n    },\n    \"isTool\": false,\n    \"symlinkTargetPath\": \"\"\n  }],\n  \"listedOutputs\": [\"bazel-out/darwin_arm64-fastbuild/bin/t_one.out\"],\n  \"mnemonic\": \"Action\",\n  \"runner\": \"local\",\n  \"cacheHit\": false,\n  \"status\": \"\",\n  \"exitCode\": 0,\n  \"targetLabel\": \"//:t_one\"\n}\n{\n  \"commandArgs\": [\"/bin/true\"],\n  \"mnemonic\": \"Action\",\n  \"exitCode\": 1,\n  \"targetLabel\": \"//:t_two\"\n}\n", "labelToTask": {"//:t_one": "p:t", "//:t_two": "q:u"}, "declaredEnvironment": {}, "expected": {"p:t": {"inputs": ["a.txt"], "env": ["FOO"], "exit": 0}, "q:u": {"inputs": [], "env": [], "exit": 1}}},
 ],
 "events": [
  {"name": "summary, completion and exit of a failed build", "text": "{\"id\":{\"targetCompleted\":{\"label\":\"//:t_1\"}},\"completed\":{\"success\":true}}\n{\"id\":{\"targetCompleted\":{\"label\":\"//:t_2\"}},\"completed\":{}}\n{\"id\":{\"testSummary\":{\"label\":\"//:t_3\"}},\"testSummary\":{\"overallStatus\":\"FAILED\"}}\n{\"id\":{\"buildFinished\":{}},\"finished\":{\"exitCode\":{\"name\":\"BUILD_FAILURE\",\"code\":1}}}\nbroken\n", "expected": {"exit": 1, "targets": {"//:t_1": True, "//:t_2": False}, "tests": {"//:t_3": "FAILED"}}},
  {"name": "a successful build carries a name and no code", "text": "{\"id\":{\"targetCompleted\":{\"label\":\"//:t_one\",\"configuration\":{\"id\":\"7f88\"}}},\"completed\":{\"success\":true,\"outputGroup\":[]}}\n{\"id\":{\"buildFinished\":{}},\"children\":[],\"finished\":{\"overallSuccess\":true,\"exitCode\":{\"name\":\"SUCCESS\"}}}\n", "expected": {"exit": 0, "targets": {"//:t_one": True}, "tests": {}}},
 ],
 "failedTasks": [
  {"name": "ansi colored nx listing", "text": "\u001b[1m NX \u001b[22m Ran target test for 2 projects\n\n   Failed tasks:\n\n   - @s/a:test\n   - b:test\n\nHint: rerun\n   - not:listed\n", "expected": ["@s/a:test", "b:test"]},
  {"name": "no listing", "text": "all good\n", "expected": []},
 ],
}
fixture["refusedSamples"] = [
 {"name": "unknown area", "value": {"version": 1, "defaults": {"phases": ["fresh"], "checks": ["graph"], "expectExit": 0, "timeoutSeconds": 10, "maximumOutputBytes": 10}, "samples": [{"id": "a", "area": "nowhere", "target": "p:t", "note": T("n", "n")}]}},
 {"name": "duplicate ids", "value": {"version": 1, "defaults": {"phases": ["fresh"], "checks": ["graph"], "expectExit": 0, "timeoutSeconds": 10, "maximumOutputBytes": 10}, "samples": [{"id": "a", "area": "hub", "target": "p:t", "note": T("n", "n")}, {"id": "a", "area": "hub", "target": "q:t", "note": T("n", "n")}]}},
 {"name": "note lacks German", "value": {"version": 1, "defaults": {"phases": ["fresh"], "checks": ["graph"], "expectExit": 0, "timeoutSeconds": 10, "maximumOutputBytes": 10}, "samples": [{"id": "a", "area": "hub", "target": "p:t", "note": {"en": "n"}}]}},
 {"name": "target without colon", "value": {"version": 1, "defaults": {"phases": ["fresh"], "checks": ["graph"], "expectExit": 0, "timeoutSeconds": 10, "maximumOutputBytes": 10}, "samples": [{"id": "a", "area": "hub", "target": "plain", "note": T("n", "n")}]}},
]
fixture["refusedRules"] = [
 {"name": "justification lacks German", "value": rules(rule("x", "ignore-glob", "output", glob="a") | {"justification": {"en": "only"}})},
 {"name": "duplicate rule ids", "value": rules(ign, ign)},
 {"name": "unknown kind", "value": rules(rule("x", "shrug", "output", glob="a"))},
 {"name": "broken regular expression", "value": rules(rule("x", "text-replace", "output", glob="a", pattern="(", flags="g", replacement=""))},
 {"name": "text replace on an input surface", "value": rules(rule("x", "text-replace", "input-bazel", glob="a", pattern="a", flags="g", replacement=""))},
 {"name": "strip prefix without a trailing slash", "value": rules(rule("x", "strip-prefix", "input-bazel", prefix="s"))},
]
json.dump(fixture, open(HERE + "/staged/fixture.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False); open(HERE + "/staged/fixture.json", "a").write("\n")
print({k: len(v) if hasattr(v, "__len__") else 1 for k, v in fixture.items()})
