#!/usr/bin/env python3
"""Builds every EDIT file of the staged set from the CURRENT live text by anchored replacements: each anchor must occur exactly once or the build fails. Writes staged/<flat> and staged/bases.json (hash of the live text each staged file was built from). Usage: build-staged.py [--check]"""
import hashlib, json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import layout

STAGED = os.path.join(layout.HERE, "staged")

def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()

def apply(flat, text, pairs):
    for number, (old, new) in enumerate(pairs, 1):
        count = text.count(old)
        if count != 1:
            raise SystemExit("%s: anchor %d occurs %d times: %r" % (flat, number, count, old[:120]))
        text = text.replace(old, new)
    return text

def cut(flat, text, start, end_marker, keep_end=True):
    i = text.index(start)
    j = text.index(end_marker, i + len(start))
    if text.count(start) != 1:
        raise SystemExit("%s: cut anchor not unique: %r" % (flat, start[:80]))
    return text[:i] + (text[j:] if keep_end else text[j + len(end_marker):])

SCRIPT = [
    (',UncachedReport:uncachedReportSchema,Kinds:kindsSchema}=componentSchema.definitions;', ',UncachedReport:uncachedReportSchema}=componentSchema.definitions;'),
    ('import {bazelCommandPlanV1,bazelTaskReportOptionsV1,bazelTestTimeoutV1,bazelTestWithinLimitV1} from "./🧭️commands/🟦️.ts";',
     'import {bazelCommandPlanV1,bazelTaskReportOptionsV1} from "./🧭️commands/🟦️.ts";\n'
     'import {bazelTestAttributesV1,bazelTestEnvironmentV1,bazelTestPlanV1,bazelTestWitnessV1,settleBazelTestV1,type BazelTestPlanV1} from "./🚦️test-classes/🟦️.ts";\n'
     'import {BAZEL_WITNESS_RUNNER,BAZEL_WITNESS_STARLARK} from "./🚦️test-classes/🧾️witness/🟦️.ts";\n'
     'import {testProtocolMessageV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🚦️protocol/🟦️.ts";'),
    ('exclusive?:true;hermeticClass?:string;modules?:string}', 'exclusive?:true;hermeticClass?:string;modules?:string;test?:BazelTestPlanV1}'),
    (',execution:{hermetic:boolean;demoted?:ReadonlySet<string>}={hermetic:false},tests?:ReadonlySet<string>):Map<string,BazelActionV1>{',
     ',execution:{hermetic:boolean;demoted?:ReadonlySet<string>}={hermetic:false}):Map<string,BazelActionV1>{'),
    ('&&!consumed.has(action.label)&&(tests===undefined||tests.has(action.id))&&bazelTaskCoverageV1(plan.tasks[action.id]!,action.cache).disposition==="test"){action.kind="test";delete action.worker;}',
     '&&!consumed.has(action.label)&&bazelTaskCoverageV1(plan.tasks[action.id]!,action.cache).disposition==="test"){action.kind="test";action.test=bazelTestPlanV1({project:plan.tasks[action.id]!.project,target:plan.tasks[action.id]!.target,files:0});delete action.worker;}'),
    ('const symbols=["repo_launch","repo_noop","repo_task",...(input.actions.some(action=>action.kind==="test")?["repo_test"]:[]),',
     'const symbols=["repo_launch","repo_noop","repo_task",...(input.actions.some(action=>action.kind==="test")?["repo_test"]:[]),...(input.actions.some(action=>action.test?.witness)?["repo_inputs_witness"]:[]),'),
    (' for(const[name,sources]of [...input.groups].sort(([left],[right])=>order(left,right)))lines.push("filegroup(",',
     ' for(const source of [...new Set(input.actions.filter(action=>action.test?.witness&&action.sources).map(action=>action.sources!))].sort(order))lines.push("repo_inputs_witness(",`    name = ${quote("w_"+source)},`,`    srcs = ${list([":"+source])},`,`    runner = ${quote("witness.mjs")},`,`    bun = ${quote(input.bun)},`,")","");\n'
     ' for(const[name,sources]of [...input.groups].sort(([left],[right])=>order(left,right)))lines.push("filegroup(",'),
    ('   if(action.sources)attributes.push(["srcs",list([":"+action.sources])]);\n   attributes.push(["deps",labels(action.restore)],["runner",quote("📜️script.ts")],',
     '   if(action.sources)attributes.push(action.test?.witness?["witnesses",list([":w_"+action.sources])]:["srcs",list([":"+action.sources])]);\n   attributes.push(["deps",labels(action.restore)],["runner",quote("📜️script.ts")],'),
    (',["timeout",quote(bazelTestTimeoutV1)]);',
     ');\n   if(action.kind==="test"){if(!action.test)throw new Error("A Bazel test action requires its test class: "+action.id);attributes.push(...bazelTestAttributesV1(action.test).map(([key,value])=>[key,value] as [string,string]));}'),
    (' const tests=new Set<string>();\n for(const action of candidates.values())if(action.kind==="test"){const{marks,extra}=index.selectAll(inputs.get(action.id)!);let count=extra.length;for(const mark of marks)count+=mark;if(bazelTestWithinLimitV1(count))tests.add(action.id);}',
     ' const witnessed=new Set<string>();\n for(const action of candidates.values())if(action.kind==="test"){const{marks,extra}=index.selectAll(inputs.get(action.id)!);let count=extra.length;for(const mark of marks)count+=mark;if(bazelTestWitnessV1(count))witnessed.add(action.id);}'),
    ('const emitted=bazelActionsV1(graph,plan,request.skipCache===true,{hermetic:true,demoted},tests);await writeChanged(join(workspace,"kinds.json"),JSON.stringify({version:1,tests:[...tests].sort()}));',
     'const emitted=bazelActionsV1(graph,plan,request.skipCache===true,{hermetic:true,demoted});'),
    ('...(modules.has(action.id)?{modules:modules.get(action.id)}:{})})),groups:tree.groups',
     '...(modules.has(action.id)?{modules:modules.get(action.id)}:{}),...(action.test&&witnessed.has(action.id)?{test:{...action.test,witness:true}}:{})})),groups:tree.groups'),
    (' if(hasHermetic){await writeChanged(join(workspace,"hermetic-runner.mjs"),BAZEL_HERMETIC_RUNNER);',
     ' await writeChanged(join(workspace,"witness.mjs"),BAZEL_WITNESS_RUNNER);\n if(hasHermetic){await writeChanged(join(workspace,"hermetic-runner.mjs"),BAZEL_HERMETIC_RUNNER);'),
    ('STARLARK_RULES+"\\n"+BAZEL_CARGO_RULE_V1+', 'STARLARK_RULES+BAZEL_WITNESS_STARLARK+"\\n"+BAZEL_CARGO_RULE_V1+'),
    ('export function bazelStarlarkRulesV1():string{return STARLARK_RULES;}', 'export function bazelStarlarkRulesV1():string{return STARLARK_RULES+BAZEL_WITNESS_STARLARK;}'),
    ('bazelActionsV1(graph,plan,request.skipCache===true,{hermetic:true},await readBazelTestsV1(workspace))', 'bazelActionsV1(graph,plan,request.skipCache===true,{hermetic:true})'),
    ('    runfiles = ctx.runfiles(files = dependencies + [ctx.file.manifest, ctx.file.fingerprint, ctx.file.runner, ctx.file.source], transitive_files = depset(transitive = [src[DefaultInfo].files for src in ctx.attr.srcs]))\n    return [DefaultInfo(executable = launcher, runfiles = runfiles), testing.ExecutionInfo(_requirements(ctx))]',
     '    witnesses = [witness[DefaultInfo].files for witness in ctx.attr.witnesses]\n    runfiles = ctx.runfiles(files = dependencies + [ctx.file.manifest, ctx.file.fingerprint, ctx.file.runner, ctx.file.source], transitive_files = depset(transitive = [src[DefaultInfo].files for src in ctx.attr.srcs] + witnesses))\n    return [DefaultInfo(executable = launcher, runfiles = runfiles), testing.ExecutionInfo(_requirements(ctx))]'),
    ('repo_test = rule(implementation = _test_impl, test = True, attrs = {"manifest": attr.label(allow_single_file = True), "fingerprint": attr.label(allow_single_file = True), "srcs": attr.label_list(allow_files = True), "deps": attr.label_list(),',
     'repo_test = rule(implementation = _test_impl, test = True, attrs = {"manifest": attr.label(allow_single_file = True), "fingerprint": attr.label(allow_single_file = True), "srcs": attr.label_list(allow_files = True), "witnesses": attr.label_list(), "deps": attr.label_list(),'),
    ('const childEnvironment=scriptInvocationEnvironment(invocation,{...environment,SEMIO_BAZEL_MANIFEST_SHA256:birth.manifestSha256});',
     'const childEnvironment=scriptInvocationEnvironment(invocation,{...environment,SEMIO_BAZEL_MANIFEST_SHA256:birth.manifestSha256,...(kind==="test"?bazelTestEnvironmentV1(process.env):{})});'),
    (' let primaryFailed=false;try{const wire=await joined;', ' let primaryFailed=false,primaryError:unknown;const testStarted=performance.now();try{const wire=await joined;'),
    ('}catch(error){primaryFailed=true;throw error;}finally{clearTimeout(handshake);',
     '}catch(error){primaryFailed=true;primaryError=error;throw error;}finally{if(kind==="test")try{settleBazelTestV1(process.env,{label:manifest.task.id,durationMs:performance.now()-testStarted,...(primaryFailed?{failure:String((primaryError as Error|undefined)?.message??primaryError).slice(0,2000)}:{})});}catch(error){process.stderr.write(testProtocolMessageV1("reportFailed",{reason:String(error)})+"\\n");}clearTimeout(handshake);'),
]

def build_script(text):
    text = apply("script.ts", text, SCRIPT)
    text = cut("script.ts", text, "/** 🧪️ Reads which planned tasks the workspace generator emitted as Bazel tests", "/** 📜 Hands the Starlark rules")
    return text

def build_commands(text):
    text = cut("commands.ts", text, "/** ⏱️ The timeout class of every generated Bazel test", "/** 🧭️ One neutral task as the Bazel batch sees it")
    text = cut("commands.ts", text, "/** 📏️ Tells whether a test action declares few enough source files", "/** 🧾️ Names the compact execution log")
    return text

COMMANDS_SCHEMA = [
    ('"description": "Closed constants. Which task is a test is decided by the neutral coverage contract (family x disposition); a `test` member only selects the `test` verb. `testInputLimit` bounds the declared source files of a Bazel test action: Bazel links every runfile of a test (measured at about 0.13 ms per file on every run, cache hits included), so larger tasks stay cacheable build actions. `sharding` is `unsupported` because the neutral contract defines no shard protocol and no target declares shards. `testOutput` is the summary detail for failing and for verbose runs.",',
     '"description": "Closed constants. Which task is a test is decided by the neutral coverage contract (family x disposition); a `test` member only selects the `test` verb. Size, timeout, shards and the input witness of a test are authored in the test class table next to this contract, never here. `testOutput` is the summary detail for failing and for verbose runs.",'),
    ('        "testOutput": { "default": "errors", "verbose": "all" },\n        "testTimeout": "eternal",\n        "testInputLimit": 10000,\n        "sharding": "unsupported"\n',
     '        "testOutput": { "default": "errors", "verbose": "all" }\n'),
]

def build_commands_schema(text):
    text = apply("commands-schema.json", text, COMMANDS_SCHEMA)
    json.loads(text)
    return text

COMMANDS_TEST = [
    ('import {bazelCommandPlanV1,bazelTestWithinLimitV1} from "../🟦️.ts";', 'import {bazelCommandPlanV1} from "../🟦️.ts";'),
    (' expect(schema.definitions.Policy.const.sharding).toBe("unsupported");\n', ' expect(Object.keys(schema.definitions.Policy.const)).toEqual(["version","testOutput"]);\n'),
]

def build_commands_test(text):
    text = apply("commands-test.ts", text, COMMANDS_TEST)
    return cut("commands-test.ts", text, 'test("the input limit of a Bazel test is authored",()=>{', "const python=`")

def build_commands_fixture(text):
    i = text.index(',\n "limits": [')
    tail = text[i:]
    if not tail.rstrip().endswith("]\n}") and not tail.rstrip().endswith("]\n}".strip()):
        raise SystemExit("commands-fixture.json: limits is not the last key")
    out = text[:i] + "\n}\n"
    value = json.loads(out)
    assert "limits" not in value
    return out

EXECUTION = [
    ('import { guardArgumentsV1, testCheckoutRootV1, testSandboxEnvironmentV1 } from "../🏖️sandbox/🟦️.ts";',
     'import { guardArgumentsV1, testCheckoutRootV1, testSandboxEnvironmentV1 } from "../🏖️sandbox/🟦️.ts";\nimport { prepareTestCommandV1, testProtocolMessageV1 } from "../🚦️protocol/🟦️.ts";'),
    ('import { mkdirSync, mkdtempSync, realpathSync, rmSync } from "node:fs";', 'import { mkdirSync, mkdtempSync, realpathSync, rmSync } from "node:fs";\nimport { tmpdir } from "node:os";'),
    ('  const child = spawn(command, [...guardArgumentsV1(command, args, sandbox.environment)], { stdio: options.captureStdout ? ["inherit", "pipe", "inherit"] : "inherit", cwd: options.cwd, env: sandbox.environment, detached: process.platform !== "win32", windowsHide: true });',
     '  const prepared = prepareTestCommandV1(command, args, sandbox.environment, sandbox.environment.TMPDIR ?? tmpdir());\n  const started = performance.now();\n  const child = spawn(command, [...guardArgumentsV1(command, prepared.args, sandbox.environment)], { stdio: options.captureStdout ? ["inherit", "pipe", "inherit"] : "inherit", cwd: options.cwd, env: { ...sandbox.environment, ...prepared.environment }, detached: process.platform !== "win32", windowsHide: true });'),
    ('    const failure = reason === "timeout" ?',
     '    try { prepared.finish({ code, signal, durationMs: performance.now() - started, ...(reason ? { reason } : {}) }); } catch (error) { console.error(testProtocolMessageV1("reportFailed", { reason: String(error) })); }\n    const failure = reason === "timeout" ?'),
]

OWNED = [
    ('import {runOwnedCommand,type OwnedCommandOptions} from', 'import {tmpdir} from "node:os";\nimport {prepareTestCommandV1,testProtocolMessageV1} from "../../../../../../🔨️modules/🏃️process/🧪️testing/🚦️protocol/🟦️.ts";\nimport {runOwnedCommand,type OwnedCommandOptions} from'),
    ('  await runOwnedCommand(command,args,cwd,label,remaining(),{...transport,env:scriptInvocationEnvironment(invocation,environment)});\n  await publish("complete");',
     '  const prepared=prepareTestCommandV1(command,args,environment,tmpdir()),started=performance.now();\n'
     '  let outcome:{code:number|null;signal:string|null;reason?:string}={code:0,signal:null};\n'
     '  try{await runOwnedCommand(command,[...prepared.args],cwd,label,remaining(),{...transport,env:scriptInvocationEnvironment(invocation,{...environment,...prepared.environment})});}\n'
     '  catch(error){outcome={code:1,signal:null,reason:String(error instanceof Error?error.message:error).slice(0,2000)};throw error;}\n'
     '  finally{try{prepared.finish({...outcome,durationMs:performance.now()-started});}catch(error){process.stderr.write(testProtocolMessageV1("reportFailed",{reason:String(error)})+"\\n");}}\n'
     '  await publish("complete");'),
]

ACTIONS_FIXTURE = [
    (r'''    "    timeout = \"eternal\",",
''', r'''    "    size = \"enormous\",",
    "    timeout = \"eternal\",",
'''),
]

CAPTURE = [
    ('import {captureOwnedCommandBodyV1,writeOwnedCommandOutputV1} from "./📥️capture/🟦️.ts";',
     'import {captureOwnedCommandBodyV1,writeOwnedCommandOutputV1} from "./📥️capture/🟦️.ts";\nimport {tmpdir} from "node:os";\nimport {prepareTestCommandV1,testProtocolMessageV1} from "../🧪️testing/🚦️protocol/🟦️.ts";'),
    (' const {result,tail}=await captureOwnedCommandBodyV1(command,args,cwd,options,false);\n if(result.status===0&&result.signal===null)return tail;',
     ' const prepared=prepareTestCommandV1(command,args,options.env??process.env,tmpdir()),started=performance.now(),granted=Object.keys(prepared.environment).length?{...options,env:{...(options.env??process.env),...prepared.environment}}:options;\n'
     ' let captured:Awaited<ReturnType<typeof captureOwnedCommandBodyV1>>;\n'
     ' try{captured=await captureOwnedCommandBodyV1(command,prepared.args,cwd,granted,false);}\n'
     ' catch(error){try{prepared.finish({code:1,signal:null,reason:String(error instanceof Error?error.message:error).slice(0,2000),durationMs:performance.now()-started});}catch{}throw error;}\n'
     ' const {result,tail}=captured;\n'
     ' try{prepared.finish({code:result.status,signal:result.signal,durationMs:performance.now()-started});}catch(error){process.stderr.write(testProtocolMessageV1("reportFailed",{reason:String(error)})+"\\n");}\n'
     ' if(result.status===0&&result.signal===null)return tail;'),
]

def build_schema(text):
    marker = '    },\n    "Kinds": {'
    if text.count(marker) != 1 or not text.rstrip().endswith("}\n  }\n}"):
        raise SystemExit("schema.json: the Kinds definition is not the last one")
    out = text[: text.index(marker)] + "    }\n  }\n}\n"
    value = json.loads(out)
    assert "Kinds" not in value["definitions"]
    return out

ENVIRONMENT_POLICY = [
    ('    "SEMIO_TEST_ARTIFACT_DIR",\n', '    "SEMIO_TEST_ARTIFACT_DIR",\n    "SEMIO_TEST_SHARD",\n    "SEMIO_TEST_SHARD_STATUS_FILE",\n    "SEMIO_TEST_XML_OUTPUT_FILE",\n    "SEMIO_TEST_REPORT_OWNER",\n    "SEMIO_TEST_REPORT",\n'),
]

def build_environment_policy(text):
    out = apply("environment-policy.json", text, ENVIRONMENT_POLICY)
    json.loads(out)
    return out

BUILDERS = {
    "environment-policy.json": build_environment_policy,
    "schema.json": build_schema,
    "capture.ts": lambda text: apply("capture.ts", text, CAPTURE),
    "actions-fixture.json": lambda text: apply("actions-fixture.json", text, ACTIONS_FIXTURE),
    "script.ts": build_script,
    "commands.ts": build_commands,
    "commands-schema.json": build_commands_schema,
    "commands-test.ts": build_commands_test,
    "commands-fixture.json": build_commands_fixture,
    "execution.ts": lambda text: apply("execution.ts", text, EXECUTION),
    "owned.ts": lambda text: apply("owned.ts", text, OWNED),
}

def main():
    os.makedirs(STAGED, exist_ok=True)
    bases = {}
    for flat in layout.EDIT:
        if flat not in BUILDERS:
            continue
        path = layout.dest(flat)
        live = open(path, encoding="utf-8").read()
        built = BUILDERS[flat](live)
        bases[flat] = sha(live)
        if "--check" not in sys.argv:
            open(os.path.join(STAGED, flat), "w", encoding="utf-8").write(built)
        print("%-22s base %s %6d -> %6d" % (flat, bases[flat][:12], len(live), len(built)))
    if "--check" not in sys.argv:
        json.dump(bases, open(os.path.join(STAGED, "bases.json"), "w"), indent=1)

if __name__ == "__main__":
    main()
