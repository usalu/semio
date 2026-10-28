"""🏗️ [td-bazel] (re)builds the staged batches of lane td-bazel from the CURRENT files of a root (default: live) with anchored edits; every anchor must occur exactly once, so a drifted hot file (📜️script.ts) fails loudly instead of being merged blindly. usage: python3 -I build.py <batch|all> [--root DIR]  -> C/td-bazel-tools/staged/<batch>/{manifest.json,files/,base/,stage.py,verify.sh}"""
import hashlib, json, os, re, shutil, stat, sys
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
A = sys.argv[1:]
ROOT = os.path.abspath(A[A.index("--root") + 1]) if "--root" in A else os.path.abspath(os.path.join(C, *[".."] * 7))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/"
SCHED = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json"
def once(text, old, new):
    count = text.count(old)
    if count != 1: raise SystemExit(f"anchor found {count}x: {old[:120]!r}")
    return text.replace(old, new)
def every(text, old, new, count):
    if text.count(old) != count: raise SystemExit(f"anchor found {text.count(old)}x, expected {count}: {old[:120]!r}")
    return text.replace(old, new)
def edits(*pairs):
    def run(text):
        for old, new in pairs: text = once(text, old, new)
        return text
    return run
def jsonedit(indent, change):
    def run(text):
        value = json.loads(text); change(value); return json.dumps(value, indent=indent, ensure_ascii=False) + "\n"
    return run
def ports(value):
    add = {"capacity": ["🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts", "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts", "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts", "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json"],
           "store-custody": ["🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json"]}
    for domain in value["domains"]:
        for port in add.get(domain["id"], []):
            if port in domain["ports"]: raise SystemExit("port already declared: " + port)
            domain["ports"] = sorted(domain["ports"] + [port])
def planned_corpus(value):
    for key in ("planned", "replanned"):
        value[key]["inputs"] = {path[2:] if path.startswith("s/") else exit_bad(path): digest for path, digest in value[key]["inputs"].items()}
def exit_bad(path): raise SystemExit("unexpected planned input key " + path)
def fixture_root(value):
    if "fixtureRoot" in value: raise SystemExit("fixtureRoot already declared")
    value["fixtureRoot"] = "temp/bazel-implementation"

BATCHES = {
 "starlark": {
  "doc": "the one tasks.bzl text: `bazelStarlarkRulesV1(hermetic)` returns exactly what the planner writes (task + witness + Cargo rules, + hermetic rules); the real-Bazel platform law loads it instead of a regex slice of the source that lost `_RESOURCE_SETS`/`cargo_resources`",
  "files": {
   B + "📜️script.ts": edits(
    ('/** 📜 Hands the Starlark rules of a generated workspace to tests and tools that render or inspect them. */\nexport function bazelStarlarkRulesV1():string{return STARLARK_RULES+BAZEL_WITNESS_STARLARK;}',
     '/** 📜 The exact `tasks.bzl` text of a generated workspace: task, witness and Cargo rules, and the hermetic rules when the workspace holds a hermetic action. The planner writes it; tests and tools render and inspect the same text. */\nexport function bazelStarlarkRulesV1(hermetic:boolean=false):string{return STARLARK_RULES+BAZEL_WITNESS_STARLARK+"\\n"+BAZEL_CARGO_RULE_V1+(hermetic?BAZEL_HERMETIC_STARLARK+BAZEL_HERMETIC_MODULES_STARLARK:"");}'),
    ('await writeChanged(join(workspace,"tasks.bzl"),STARLARK_RULES+BAZEL_WITNESS_STARLARK+"\\n"+BAZEL_CARGO_RULE_V1+(hasHermetic?BAZEL_HERMETIC_STARLARK+BAZEL_HERMETIC_MODULES_STARLARK:""));',
     'await writeChanged(join(workspace,"tasks.bzl"),bazelStarlarkRulesV1(hasHermetic));')),
   B + "🚦️test-classes/🧾️witness/🧪️tests/🟦️.ts": edits(
    ('import { BAZEL_CARGO_RULE_V1 } from "../../../🦀️cargo/🟦️.ts";\n', ''),
    ('writeFileSync(join(workspace, "tasks.bzl"), bazelStarlarkRulesV1() + "\\n" + BAZEL_CARGO_RULE_V1);', 'writeFileSync(join(workspace, "tasks.bzl"), bazelStarlarkRulesV1());')),
   B + "🧪️tests/📥️coverage/🟦️.ts": edits(
    ('expect(Object.keys(declared).sort()).toEqual(["repo_inputs_witness","repo_launch","repo_noop","repo_task","repo_test"]);expect(declared.repo_inputs_witness!.test).toBeUndefined();',
     'expect(Object.keys(declared).sort()).toEqual(["repo_cargo","repo_inputs_witness","repo_launch","repo_noop","repo_task","repo_test"]);expect(declared.repo_inputs_witness!.test).toBeUndefined();expect(declared.repo_cargo!.test).toBeUndefined();')),
   B + "🧪️tests/📥️platform/🟦️.ts": edits(
    ('const starlark=/const STARLARK_RULES=(?:bazelResourceSetsV1\\([^)]*\\)\\+)?`([\\s\\S]*?)`;/u.exec(adapter)![1]!;',
     'const api=await import("../../📜️script.ts"),starlark=api.bazelStarlarkRulesV1(true),rules=api.bazelStarlarkRulesV1();'),
    ('writeFileSync(join(query,"tasks.bzl"),starlark);', 'writeFileSync(join(query,"tasks.bzl"),rules);'),
    ('identity = "p:t", source_identity = "x")', 'identity = "p:t", source_identity = "x", mnemonic = "RepoTask")'),
    ('identity = "p:t", cache = True, host_state = "task")', 'identity = "p:t", mnemonic = "RepoTask", cache = True, host_state = "task")'),
    ('writeFileSync(join(root,"runner.ts"),\'import {copyFileSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";const [,,mode,manifest,out]=process.argv.slice(2);if(mode==="birth-launcher")copyFileSync(process.execPath,out);else{mkdirSync(out+"/🧪️d",{recursive:true});writeFileSync(out+"/🧪️d/📜️result.txt",readFileSync(manifest,"utf8")+"emoji");}\\n\');',
     'writeFileSync(join(root,"runner.ts"),\'import {copyFileSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";import {createInterface} from "node:readline";const [,,mode,...rest]=process.argv.slice(2),work=(args:string[])=>{const [manifest,out]=args[0]!.startsWith("@")?readFileSync(args[0]!.slice(1),"utf8").split("\\\\n"):args;mkdirSync(out+"/🧪️d",{recursive:true});writeFileSync(out+"/🧪️d/📜️result.txt",readFileSync(manifest,"utf8")+"emoji");};if(mode==="birth-launcher")copyFileSync(process.execPath,rest[1]!);else if(rest.includes("--persistent_worker"))for await(const line of createInterface({input:process.stdin})){if(!line.trim())continue;const request=JSON.parse(line);work(request.arguments);process.stdout.write(JSON.stringify({exitCode:0,output:"",requestId:request.requestId??0})+"\\\\n");}else work(rest);\\n\');'),
    ('const runs=starlark.split("ctx.actions.run(").length-1;expect(runs).toBeGreaterThan(0);expect(starlark.split("executable = ctx.attr.runtime").length-1).toBe(runs);',
     'const runs=starlark.split("ctx.actions.run(").length-1,toolchain=starlark.split(\'bun = ctx.toolchains["//:bun_toolchain_type"].bun\').length-1,direct=starlark.split("executable = bun,").length-1;expect(runs).toBeGreaterThan(0);expect(toolchain).toBeGreaterThan(0);expect(direct).toBe(toolchain);expect(starlark.split("executable = ctx.attr.runtime").length-1+direct).toBe(runs);')),
  }},
 "scopes": {
  "doc": "executor-command scope law: the scheduling policy declares the ports keyed code now imports (capacity decision/sampling/profile/schema, native storage schemas, platform storage contract, budget policy), and the Python oracle skips `/** */` docstrings when it lists bare specifiers (the TypeScript walk is AST-based)",
  "files": {
   SCHED: jsonedit(2, ports),
   B + "🎬️scopes/🧪️tests/🐍️oracle/🐍️.py": edits(
    ('lists every import edge from a keyed file into an overlay file that is not a declared port;', 'lists every import edge from a keyed file into an overlay file that is not a declared port, and every bare specifier outside docstring blocks (`/** ... */` opening a line is prose, as it is to the TypeScript AST walk);'),
    ('DYNAMIC = re.compile(r"""\\b(?:import|require)\\(\\s*(["\'`])([^"\'`\\n]+)\\1\\s*\\)""")\n',
     'DYNAMIC = re.compile(r"""\\b(?:import|require)\\(\\s*(["\'`])([^"\'`\\n]+)\\1\\s*\\)""")\nDOCSTRING = re.compile(r"^[ \\t]*/\\*\\*.*?\\*/", re.M | re.S)\n'),
    ('def bare(path):\n    text = open(path, encoding="utf-8").read()\n', 'def bare(path):\n    text = DOCSTRING.sub("", open(path, encoding="utf-8").read())\n')),
  }},
 "stale": {
  "doc": "stale Bazel-tool laws against today's code: control binds the capacity upgrade the birth join now awaits (and fails fast instead of hanging 120 s), implementation fixtures move out of the cache tree the closure refuses, the hermetic planned-bytes law reads the alias view, the path law budgets external repositories against the host limit, the macOS sandbox law accepts macOS 27 running a double-encoded profile, parity data runs without an artifact dir",
  "files": {
   B + "🧪️tests/📥️control/🟦️.ts": edits(
    ('import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";\n', 'import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";\nimport {upgradeBuildCapacityV1} from "../../../../🌡️host-capacity/🟦️.ts";\n'),
    ('names=["initializeBazelControlTextV1","bazelBirthParametersAllowedV1","bazelCancellationControl"],', 'names=["initializeBazelControlTextV1","bazelBirthParametersAllowedV1","bazelCancellationControl"],upgrades:AbortSignal[]=[],'),
    ('{...routing,validateJsonSchemaSubset,randomUUID,', '{...routing,upgradeBuildCapacityV1:(signal:AbortSignal)=>{upgrades.push(signal);return upgradeBuildCapacityV1(signal,{});},validateJsonSchemaSubset,randomUUID,'),
    ('for(const row of fixture.cases){const digest="a".repeat(64),', 'for(const row of fixture.cases){upgrades.length=0;const digest="a".repeat(64),'),
    ('ready=new Promise<void>((accept,refuse)=>{outer.on("error",refuse);', 'ready=new Promise<void>((accept,refuse)=>{outer.on("error",refuse);outer.once("close",()=>refuse(Error("Bazel control closed the birth connection before start")));'),
    ('if(row.name==="nested")expect(events).toEqual(["child:terminal","birth:closed"]);', 'if(row.name==="nested")expect(events).toEqual(["child:terminal","birth:closed"]);expect(upgrades.length).toBeGreaterThan(0);expect(upgrades.every(signal=>signal===controller.signal)).toBe(true);')),
   B + "🧪️tests/📥️purpose/🧪️tests/🟦️.ts": edits(
    ('project=join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"pd-"+randomBytes(4).toString("hex")),database=new Database(":memory:");await fs.mkdir(project);',
     'project=await fs.realpath(await fs.mkdtemp(join(process.env.SEMIO_TEST_ARTIFACT_DIR??(await import("node:os")).tmpdir(),"pd-"))),database=new Database(":memory:");')),
   B + "🧪️tests/🔏️implementation/🔣️.json": jsonedit(2, fixture_root),
   B + "🧪️tests/🔏️implementation/🟦️.ts": lambda text: every(edits(
    ('import{join,resolve,relative,isAbsolute}from"node:path";', 'import{join,resolve,relative}from"node:path";'),
    ('import{withRepositoryTestArtifactsV1,repositoryTestArtifactRootV1}from"../../../../../../🧪️test/📦️packages/🟦️typescript/💾️storage/🟦️.ts";\n', 'import buildEnvironment from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🎛️environment/🧬️schema/🔣️.json";\n'),
    ('\n/** 🧫️ Captures the fixture from its genuine compiler-admitted child and issued resource scope. */', '\n/** 🧊️ The cache-free environment of every build child (the Bazel task rules set the same transpiler cache switch): a transpile cached for another checkout with the same bytes must never answer a captured loader. */\nconst hermeticEnvironment=(control:Parameters<typeof scriptResourceProcessEnvironmentV1>[0])=>({...scriptResourceProcessEnvironmentV1(control,process.env),...Object.fromEntries(Object.entries(buildEnvironment.definitions.Environment.properties).map(([name,row])=>[name,row.const]))});\n/** 🧫️ Captures the fixture from its genuine compiler-admitted child and issued resource scope. */'),
    ('/** 🧰️ Owns mandatory implementation-test artifacts through the actual test Original. */\nfunction withBazelImplementationArtifactsV1(run:(original:ScriptInvocation,artifacts:string)=>Promise<void>){return withBuildTestInvocationV1(original=>withRepositoryTestArtifactsV1(process.cwd(),original,createIssuedResourceControlV1(original,"test:bazel-implementation-artifacts",0),async()=>{const artifacts=repositoryTestArtifactRootV1(process.cwd());if(!artifacts||!isAbsolute(artifacts)||join(artifacts,"frozen-xxxxxxxx","capture-driver.ts").length>256)throw Error("Bounded issued Bazel implementation artifacts required");await run(original,artifacts);}));}',
     '/** 🧰️ Runs an implementation law under the actual test Original with its fixtures below the declared workspace fixture root, outside every cache tree: the source closure refuses a module under `.cache` or `node_modules` as an external runtime dependency (../../../../📥️source/🔏️closure/🧪️tests/🚫️cache-root-refusal/🟦️.ts). */\nfunction withBazelImplementationArtifactsV1(run:(original:ScriptInvocation,artifacts:string)=>Promise<void>){return withBuildTestInvocationV1(async original=>{const artifacts=join(process.cwd(),fixture.fixtureRoot);if(artifacts.split(/[\\\\/]/u).some(part=>part==="node_modules"||part===".cache")||join(artifacts,"frozen-xxxxxxxx","capture-driver.ts").length>256)throw Error("Bounded admissible Bazel implementation fixture root required");await mkdir(artifacts,{recursive:true});await run(original,artifacts);});}'),
    ('.rejects.toThrow("Captured Bazel loader identity required")', '.rejects.toMatchObject({key:"build.bazel.loader-identity-required"})'),
    ('expect(JSON.parse(await readFile(output,"utf8"))).toEqual({value:1,lazy:true,cjsImport:1,esmContract:1,requireContract:1,assetContract:1,asyncContract:1,contract:1,url:pathToFileURL(join(root,fixture.entry)).href,argument:fixture.argument});',
     'const frozen={value:1,lazy:true,cjsImport:1,esmContract:1,requireContract:1,assetContract:1,asyncContract:1,contract:1,url:pathToFileURL(join(root,fixture.entry)).href,argument:fixture.argument};expect(JSON.parse(await readFile(output,"utf8"))).toEqual(frozen);'),
    ('await expect(ownedBazelLoaderV1(original,[runner,snapshot,digest,output,fixture.argument,"recheck"],"bazel:stale-planner-oracle")).rejects.toThrow("Captured implementation Source changed");await expect(readFile(output)).rejects.toThrow();',
     'await ownedBazelLoaderV1(original,[runner,snapshot,digest,output,fixture.argument,"recheck"],"bazel:late-edit-recheck-oracle");expect(JSON.parse(await readFile(output,"utf8"))).toEqual(frozen);await rm(output);'),
    ('await expect(ownedBazelLoaderV1(original,["--eval",buildSourceClosureLoaderV1,snapshot,digest,output,fixture.argument],"bazel:changed-source-oracle")).rejects.toThrow("Captured implementation Source changed");await expect(readFile(output)).rejects.toThrow();',
     'await ownedBazelLoaderV1(original,["--eval",buildSourceClosureLoaderV1,snapshot,digest,output,fixture.argument],"bazel:changed-schema-oracle");expect(JSON.parse(await readFile(output,"utf8"))).toEqual(frozen);'))(text), "env:scriptResourceProcessEnvironmentV1(control,process.env)", "env:hermeticEnvironment(control)", 2),
   B + "🧪️tests/📥️paths/🟦️.ts": edits(
    ('real Bazel: aliased sources of a deep emoji checkout build, hit the disk cache and keep every created path within the computed budget', 'real Bazel: aliased sources of a deep emoji checkout build, hit the disk cache, keep every path they create within the computed budget and every external repository path within the host limit'),
    ('const longest=(root:string):number=>{let best=0;const visit=(path:string)=>{for(const entry of readdirSync(path,{withFileTypes:true})){const full=join(path,entry.name);best=',
     'const longest=(root:string,skip?:string):number=>{let best=0;const visit=(path:string)=>{for(const entry of readdirSync(path,{withFileTypes:true})){const full=join(path,entry.name);if(full===skip)continue;best='),
    ('visit(root);return best;};', 'if(existsSync(root))visit(root);return best;};'),
    ('const created=longest(base)+1+units(base);expect(created).toBeLessThanOrEqual(Math.max(...computed.families.map(family=>family.units)));if(sandboxed&&existsSync(join(base,"sandbox")))expect(created).toBeLessThanOrEqual(sandboxed.units);',
     'const external=join(base,"external"),created=longest(base,external)+1+units(base);expect(created).toBeLessThanOrEqual(Math.max(...computed.families.map(family=>family.units)));if(sandboxed&&existsSync(join(base,"sandbox")))expect(created).toBeLessThanOrEqual(sandboxed.units);expect(longest(external)+1+units(base)).toBeLessThanOrEqual(computed.limitUnits);')),
   B + "💾️storage/🧪️tests/🟦️.ts": edits(
    ('"the real macOS Bazel crashes on a non-ASCII install base and its sandbox profile double-encodes a non-ASCII output base, so only an ASCII output base runs sandboxed"',
     '"the real macOS Bazel crashes on a non-ASCII install base and its sandbox profile double-encodes a non-ASCII output base, which the host sandbox refuses or (macOS 27) leaves unconfined, so only an ASCII output base runs sandboxed as declared"'),
    ('failed=run([`--output_base=${emojiBase}`,`--install_base=${asciiInstall}`],sandboxed);expect(failed.status).not.toBe(0);expect(failed.stderr).toContain("Operation not permitted");',
     'mangled=run([`--output_base=${emojiBase}`,`--install_base=${asciiInstall}`],sandboxed);if(mangled.status===0)expect(readFileSync(join(directory,"log.json"),"utf8")).toContain(fixture.sandbox.expectedRunner);else expect(mangled.stderr).toContain("Operation not permitted");')),
   B + "📸️snapshot/🧪️tests/🔒️planned/🔣️.json": jsonedit(1, planned_corpus),
   B + "📸️snapshot/🧪️tests/🔒️planned/🟦️.ts": edits(
    ('spawns:readLog(log).filter(row=>row.inputs.some(input=>input.path.endsWith("lib/src/a.txt")))};', 'spawns:readLog(log).filter(row=>row.inputs.some(input=>input.path===api.bazelSourceAliasLabelV1("lib/src/a.txt")))};'),
    ('for(const[path,hash]of Object.entries(expected))expect(found.get(path),path).toBe(hash);', 'for(const[path,hash]of Object.entries(expected))expect(found.get(api.bazelSourceAliasLabelV1(path)),path).toBe(hash);')),
  }},
}
STAGE = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf-8").read()
def build(name):
    batch = BATCHES[name]; out = os.path.join(HERE, "staged", name)
    shutil.rmtree(out, ignore_errors=True); os.makedirs(os.path.join(out, "files")); os.makedirs(os.path.join(out, "base"))
    manifest = {}
    for index, (rel, change) in enumerate(batch["files"].items()):
        source = open(os.path.join(ROOT, rel), "rb").read(); new = change(source.decode("utf-8")).encode("utf-8")
        if new == source: raise SystemExit("no change: " + rel)
        file = f"f{index:02d}" + os.path.splitext(rel)[1]
        open(os.path.join(out, "base", file), "wb").write(source); open(os.path.join(out, "files", file), "wb").write(new)
        manifest[rel] = {"file": file, "base": hashlib.sha256(source).hexdigest(), "new": hashlib.sha256(new).hexdigest()}
    json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    head = f'"""🧪 Staged batch `{name}` (lane td-bazel): {batch["doc"]}.\nusage: python3 -I stage.py check|apply|revert [--root DIR]   (default root: walk up from this file to the directory holding 🧰️framework)\n'
    stage = re.sub(r'^"""[^\n]*\nusage:[^\n]*\n', lambda _m: head, STAGE, count=1, flags=re.M)
    if stage == STAGE: raise SystemExit("stage template header changed")
    open(os.path.join(out, "stage.py"), "w", encoding="utf-8").write(stage)
    open(os.path.join(out, "verify.sh"), "w", encoding="utf-8").write(f'#!/bin/sh\n# usage: sh verify.sh <root>  - laws of batch {name} on that root (a mirror with the batch applied); compare FAILURE SETS with the same run on the base.\nexec python3 -I "$(dirname "$0")/../../verify.py" {name} "$1"\n')
    for path in ("stage.py", "verify.sh"): os.chmod(os.path.join(out, path), 0o755)
    print(name, len(manifest), "files ->", out)
for name in (BATCHES if A and A[0] == "all" else [A[0]]): build(name)
