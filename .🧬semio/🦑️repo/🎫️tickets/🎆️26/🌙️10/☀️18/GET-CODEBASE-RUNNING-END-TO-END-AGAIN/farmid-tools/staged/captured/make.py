#!/usr/bin/env python3
"""🪆 [farmid] Builds the staged batch `captured` from the CURRENT files of a root (default: live, walked up from here): graph farm workers start from the snapshot their parent runs from. Anchored edits (each anchor exactly once unless a count is given) plus the new files in new/; writes files/, base/, manifest.json beside this script. Rebase = run it again on the moved tree.
usage: python3 -I make.py [--root DIR] [--landed]   (--landed: the batch is live; new = the live bytes, base = the live bytes with every edit reversed, so revert removes exactly this batch from a tree peers edited since)"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
F = B + "🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/"
A = B + "🕸️graph/📥️inference/🧱️project-assembly/"
NEW = {
    F + "🟨️.d.mts": "farm.d.mts",
    F + "🪆️captured/🟨️.mjs": "captured.mjs",
    F + "🪆️captured/🟨️.d.mts": "captured.d.mts",
    F + "🧪️tests/🪆️captured/🟦️.ts": "test.ts",
    F + "🧪️tests/🧫️fixtures/🪆️captured-entry/🟨️.mjs": "fixture-entry.mjs",
    F + "🧪️tests/🧫️fixtures/🪆️captured-service/🟨️.mjs": "fixture-service.mjs",
    F + "🧪️tests/🧫️fixtures/🪆️kernel-role/🟦️.ts": "fixture-kernel-role.ts",
    A + "🧪️tests/🟦️.ts": "precision-test.ts",
}
EDITS = {
    B + "🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs": [
        ("/** 🙈️ The resources the build policy declares presentation-only (progress labels): their edits never change a command's behavior, so they are not command inputs. */\nfunction presentationResourcesV1(cache) {\n  return cache.presentation ??= new Set(JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, \"🏗️build\", \"🔣️policy.json\")).toString(\"utf8\")).presentationResources);\n}",
         "/** 🙈️ The resources the build policy declares presentation-only (progress labels): their edits never change a command's behavior, so they are not command inputs. The policy is read once per script input cache and the facts of that read are replayed into every receiving project, so the rows of a project never depend on which project of the same process asked first. */\nfunction presentationResourcesV1(cache) {\n  return memoizedObservedComputationV1(cache.presentation ??= new Map(), \"presentation\", () => new Set(JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, \"🏗️build\", \"🔣️policy.json\")).toString(\"utf8\")).presentationResources));\n}"),
    ],
    B + "🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs": [
        ('import { observedSourceAnalysisV1, observedRequireResolve } from "../../../../🏗️build/👁️observation/🟨️.mjs";', 'import { memoizedObservedComputationV1, observedSourceAnalysisV1, observedRequireResolve } from "../../../../🏗️build/👁️observation/🟨️.mjs";'),
        ("/** 🧬️ Binds structural parsers to their evaluated implementation and compiler bytes. */\nexport function prepareSourceAnalysesV1(){\n  if(!sourceAnalysesV1){\n",
         "const compilerRevisionsV1=new Map();\n"
         "/** 🔩️ The revision the module parsers carry, bound to the bytes of the TypeScript compiler they use, as every caller records it: resolved and read once per process, and the facts of that read are replayed into each receiving project, so the rows of a project never depend on which project of the same process asked first (a farm worker infers in a fresh process). */\n"
         "function compilerRevisionV1(stamp){return memoizedObservedComputationV1(compilerRevisionsV1,stamp,()=>createHash(\"sha256\").update(stamp).update(readPhysicalSource(observedRequireResolve(fileURLToPath(revisionIdentity.url),\"typescript\"))).digest(\"hex\"));}\n\n"
         "/** 🧬️ Binds structural parsers to their evaluated implementation and compiler bytes. */\nexport function prepareSourceAnalysesV1(){\n  const stamp=inferrerStampV1(\"source-analyses\"),revision=compilerRevisionV1(stamp);\n  if(!sourceAnalysesV1){\n"),
        (',compiler=observedRequireResolve(fileURLToPath(revisionIdentity.url),"typescript"),stamp=inferrerStampV1("source-analyses"),stampRevision=createHash("sha256").update(stamp).digest("hex"),revision=createHash("sha256").update(stamp).update(readPhysicalSource(compiler)).digest("hex");',
         ',stampRevision=createHash("sha256").update(stamp).digest("hex");'),
    ],
    A + "🟨️.mjs": [
        ('import { observedSourceAnalysisV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";', 'import { memoizedObservedComputationV1, observedSourceAnalysisV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";'),
        ("/** 🎯️ The input precision rules and the engine host of one graph construction. */\nfunction precisionContextV1(workspaceRoot, facts, scripts) {\n  const known = precisionContexts.get(scripts);\n  if (known) return known;\n  for (const file of PRECISION_FILES) readFileSync(join(LIBRARY_ROOT, file));\n  const rules = join(LIBRARY_ROOT, PRECISION_RULES), document = existsSync(rules) ? readInputPrecisionV1(JSON.parse(readFileSync(rules, \"utf8\"))) : undefined;\n",
         "const precisionDocumentsV1 = new Map();\n"
         "/** 📏️ The input precision rules as every project that applies them records them: read once per process, and the facts of that read (the engine, its schema and the rules) are replayed into each receiving project, so the rows of a project never depend on which project of the same process asked first (a farm worker infers in a fresh process). */\n"
         "export function readProjectInputPrecisionV1() {\n"
         "  return memoizedObservedComputationV1(precisionDocumentsV1, PRECISION_RULES, () => { for (const file of PRECISION_FILES) readFileSync(join(LIBRARY_ROOT, file)); const rules = join(LIBRARY_ROOT, PRECISION_RULES); return existsSync(rules) ? readInputPrecisionV1(JSON.parse(readFileSync(rules, \"utf8\"))) : null; }) ?? undefined;\n"
         "}\n\n"
         "/** 🎯️ The input precision rules and the engine host of one graph construction. */\nfunction precisionContextV1(workspaceRoot, facts, scripts) {\n  const document = readProjectInputPrecisionV1(), known = precisionContexts.get(scripts);\n  if (known?.document === document) return known;\n"),
    ],
    A + "🟨️.d.mts": [
        ("export function wire(registry: Record<string, any>): void;\n", "export function wire(registry: Record<string, any>): void;\n/** 📏️ The input precision rules, read once per process with their facts replayed into every receiving project; undefined without a rules file. */\nexport function readProjectInputPrecisionV1(): import(\"../../../📥️inputs/🎯️precision/🟦️.ts\").InputPrecisionDocumentV1 | undefined;\n"),
    ],
    B + "🕸️graph/📥️inference/🔖️stamps/🧬️schema/🔣️.json": [
        ('"stamp": "semio.graph-inferrer.project-assembly/1"', '"stamp": "semio.graph-inferrer.project-assembly/2"'),
    ],
    B + "🕸️graph/📥️inference/🧪️tests/🔖️output-stamps/🔣️.json": [
        ('"semio.graph-inferrer.project-assembly/1":', '"semio.graph-inferrer.project-assembly/2":'),
    ],
    F + "🟨️.mjs": [
        ('import { mkdtempSync, realpathSync, rmSync } from "node:fs";', 'import { mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";'),
        ('import { inferenceRecordIdentityV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";\n',
         'import { inferenceRecordIdentityV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";\nimport { adoptedCapturedBuildLaunchV1 } from "../../../../../🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts";\nimport { GRAPH_FARM_WORKER_ROLE_V1, graphFarmIdentityDifferenceV1, graphFarmIdentityV1 } from "./🪆️captured/🟨️.mjs";\n'),
        ("so the farm changes how fast a record appears, never what it holds.\n */",
         "so the farm changes how fast a record appears, never what it holds.\n * In a process that runs from a captured snapshot (`launch`, by default the snapshot this process adopted; `null` starts workers from the files) every worker is started from that same snapshot, written once to a private file it reads as descriptor 3, and serves only when it reports the captured implementation and the record identity of this process; a worker loaded from the files on disk, which peers may have changed since the capture, is refused.\n */"),
        ("processed = () => 0, command = undefined,", "processed = () => 0, launch = adoptedCapturedBuildLaunchV1(), command = undefined,"),
        ('  let state = "idle", started, closing = false, capacity, opening = Promise.resolve();\n',
         '  let state = "idle", started, closing = false, capacity, opening = Promise.resolve(), image, refusal;\n'
         '  const captured = launch ?? undefined, imageFile = () => { if (image !== undefined) return image.file; const directory = mkdtempSync(join(realpathSync(tmpdir()), "semio-graph-farm-image-")), file = join(directory, "snapshot.json"); try { writeFileSync(file, captured.text(), { flag: "wx", mode: 0o600 }); } catch (error) { rmSync(directory, { recursive: true, force: true }); throw error; } image = { directory, file }; return file; };\n'),
        ("  const start = ({ source, workers, megabytes }, release) => {\n",
         "  const start = ({ source, workers, megabytes }, release) => {\n"
         "    if (state === \"off\") { release(); return undefined; }\n"
         "    let auxiliaryFile;\n"
         "    try { auxiliaryFile = captured === undefined ? undefined : imageFile(); } catch (error) { release(); stop(\"degraded\", { reason: String(error?.message ?? error) }); return undefined; }\n"),
        ("      entry: ENTRY, service: SERVICE, workers, megabytes, environment, signal,",
         "      entry: captured === undefined ? ENTRY : GRAPH_FARM_WORKER_ROLE_V1, service: SERVICE, workers, megabytes, environment: captured === undefined ? environment : { ...environment, ...captured.environment }, signal,"),
        ("      accept: identity => identity === inferenceRecordIdentityV1(),",
         "      accept: identity => { const difference = graphFarmIdentityDifferenceV1(graphFarmIdentityV1(inferenceRecordIdentityV1(), captured?.implementation), identity); refusal = difference || undefined; return difference === \"\"; },"),
        ('        else if (event === "disabled") say(String(fields.reason).startsWith("identity") ? "refused" : "degraded", fields);',
         '        else if (event === "disabled") say(String(fields.reason).startsWith("identity") ? "refused" : "degraded", refusal === undefined ? fields : { ...fields, reason: fields.reason + ": " + refusal });'),
        ("      ...(command === undefined ? {} : { command }), ...(commandArguments === undefined ? {} : { commandArguments }), ...(spawnProcess === undefined ? {} : { spawnProcess }),",
         "      ...(captured === undefined ? { ...(command === undefined ? {} : { command }), ...(commandArguments === undefined ? {} : { commandArguments }) } : { command: captured.command, commandArguments: captured.arguments, auxiliaryFile }), ...(spawnProcess === undefined ? {} : { spawnProcess }),"),
        ("finally { rmSync(pool.base, { recursive: true, force: true }); } } }\n",
         "finally { rmSync(pool.base, { recursive: true, force: true }); } } }\n      if (image !== undefined) rmSync(image.directory, { recursive: true, force: true });\n"),
    ],
    F + "🏊️pool/🟨️.mjs": [
        ('import { mkdtempSync, rmSync, writeFileSync } from "node:fs";', 'import { closeSync, constants, mkdtempSync, openSync, rmSync, writeFileSync } from "node:fs";'),
        ("so it can reclaim what that process left.\n */",
         "so it can reclaim what that process left.\n * With `auxiliaryFile` every worker reads that private regular file as descriptor 3 (opened read-only and without following links for each start, so no two workers share a file offset; a file is read in one pass, a pipe needs this process scheduled for every chunk) and the channel moves to descriptor 4.\n */"),
        ("command = process.execPath, commandArguments = [] }) {", "command = process.execPath, commandArguments = [], auxiliaryFile = undefined }) {"),
        ('    const child = spawnProcess(command, [...commandArguments, entry], { stdio: ["ignore", "inherit", "inherit", "ipc"], serialization: "json", env: childEnvironment });\n',
         '    let child, descriptor;\n'
         '    try { descriptor = auxiliaryFile === undefined ? undefined : openSync(auxiliaryFile, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0)); child = spawnProcess(command, [...commandArguments, entry], { stdio: descriptor === undefined ? ["ignore", "inherit", "inherit", "ipc"] : ["ignore", "inherit", "inherit", descriptor, "ipc"], serialization: "json", env: childEnvironment }); }\n'
         '    catch (error) { lose(slot, String(error?.message ?? error)); return; }\n'
         '    finally { if (descriptor !== undefined) closeSync(descriptor); }\n'),
    ],
    F + "🏊️pool/🟨️.d.mts": [
        ("command?:string;commandArguments?:readonly string[]}):GraphFarmV1;", "command?:string;commandArguments?:readonly string[];auxiliaryFile?:string}):GraphFarmV1;"),
    ],
    F + "👷️service/🟨️.mjs": [
        ('import { createProjectInferenceEnvironmentV1, produceProjectRecordV1 } from "../../🟨️.mjs";\n',
         'import { createProjectInferenceEnvironmentV1, produceProjectRecordV1 } from "../../🟨️.mjs";\nimport { graphFarmIdentityV1 } from "../🪆️captured/🟨️.mjs";\n'),
        ("; answers the identity its records are written under. */", "; answers the farm identity it serves under: the captured implementation it runs from and the identity its records are written under. */"),
        ("  return inferenceRecordIdentityV1();\n}", "  return graphFarmIdentityV1(inferenceRecordIdentityV1());\n}"),
        ('import { lstatSync, mkdirSync, readdirSync, rmSync } from "node:fs";', 'import { lstatSync, mkdirSync, readdirSync } from "node:fs";'),
        ("/** 🔌️ Opens one worker session: the same inference implementation as the parent, memos of its own, and a staging directory of its own that nothing else writes to;",
         "/** 🔌️ Opens one worker session: the same inference implementation as the parent, memos of its own, and a staging directory of its own that nothing else writes to and that only the parent removes, once it saw this process exit (a worker recycled right after a job must leave the leaves of that job for the parent to adopt);"),
        ("cpuMilliseconds: Math.round((used.user + used.system) / 1000) };\n}\n\n/** 🧹️ Removes the staging directory. */\nexport function close() { if (staging) rmSync(staging, { recursive: true, force: true }); }\n", "cpuMilliseconds: Math.round((used.user + used.system) / 1000) };\n}\n"),
    ],
    F + "🧪️tests/🟦️.ts": [
        ('import {inferenceRecordIdentityV1} from "../../../../../👁️observation/🟨️.mjs";\n', 'import {inferenceRecordIdentityV1} from "../../../../../👁️observation/🟨️.mjs";\nimport {graphFarmIdentityV1} from "../🪆️captured/🟨️.mjs";\n'),
        ('identity:inferenceRecordIdentityV1()}', 'identity:graphFarmIdentityV1(inferenceRecordIdentityV1())}'),
        ("openProjectFarmV1({workspaceRoot:root,context:{registry:null},", "openProjectFarmV1({workspaceRoot:root,context:{registry:null},launch:null,", 2),
    ],
    F + "🧪️tests/🔒️farm-close/🟦️.ts": [
        ('import{inferenceRecordIdentityV1}from"../../../../../../👁️observation/🟨️.mjs";\n', 'import{inferenceRecordIdentityV1}from"../../../../../../👁️observation/🟨️.mjs";\nimport{graphFarmIdentityV1}from"../../🪆️captured/🟨️.mjs";\n'),
        ('identity:inferenceRecordIdentityV1()}', 'identity:graphFarmIdentityV1(inferenceRecordIdentityV1())}'),
        ("openProjectFarmV1({workspaceRoot:root,context:{registry:null},", "openProjectFarmV1({workspaceRoot:root,context:{registry:null},launch:null,"),
    ],
    B + "📥️source/🔏️closure/🚪️command/🟦️.ts": [
        ("export function adoptCapturedBuildCommandV1(proof:CapturedBuildCommandProofV1):void{adoptedCommandV1=proof;}\n",
         "export function adoptCapturedBuildCommandV1(proof:CapturedBuildCommandProofV1):void{adoptedCommandV1=proof;}\n"
         "/** 🧬️ How this process starts one more process of the implementation it runs from: the stream loader with the digest of the adopted snapshot, the variables every captured process gets, and the snapshot text the new process reads as descriptor 3 (its loader refuses any other bytes); undefined when this process runs from no adopted snapshot. */\n"
         "export function adoptedCapturedBuildLaunchV1():Readonly<{command:string;arguments:readonly string[];environment:Readonly<Record<string,string>>;implementation:string;text:()=>string}>|undefined{const proof=adoptedCommandV1;return proof&&typeof proof.text===\"function\"?Object.freeze({command:process.execPath,arguments:Object.freeze([\"--eval\",buildSourceClosureStreamLoaderV1,\"--\",proof.sha256]),environment:Object.freeze({BUN_RUNTIME_TRANSPILER_CACHE_PATH:\"0\"}),implementation:proof.sha256,text:proof.text}):undefined;}\n"),
        (' const proof=adoptedCommandV1;if(!proof||typeof proof.text!=="function")throw Error("Adopted captured command snapshot required");\n const text=proof.text(),argv=["--eval",buildSourceClosureStreamLoaderV1,"--",proof.sha256,...args],',
         ' const adopted=adoptedCapturedBuildLaunchV1();if(!adopted)throw Error("Adopted captured command snapshot required");\n const text=adopted.text(),argv=[...adopted.arguments,...args],'),
        ('createHash("sha256").update(text).digest("hex")!==proof.sha256||[process.execPath,...argv].map(quote).join(" ").length+1>32767)throw Error("Bounded adopted command publication required");',
         'createHash("sha256").update(text).digest("hex")!==adopted.implementation||[adopted.command,...argv].map(quote).join(" ").length+1>32767)throw Error("Bounded adopted command publication required");'),
        ('(options.launch??runOwnedCommand)(process.execPath,argv,options.cwd,options.label,options.budgetMilliseconds,{control,signal:control.signal,env:{...scriptResourceProcessEnvironmentV1(control,options.environment),BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"},',
         '(options.launch??runOwnedCommand)(adopted.command,argv,options.cwd,options.label,options.budgetMilliseconds,{control,signal:control.signal,env:{...scriptResourceProcessEnvironmentV1(control,options.environment),...adopted.environment},'),
    ],
    B + "⌨️command/📥️entry/🟦️.ts": [
        ('import{exitIfStaleImplementationV1}from"../../🧷️format-stamps/🟨️.mjs";\n',
         'import{exitIfStaleImplementationV1}from"../../🧷️format-stamps/🟨️.mjs";\nimport{GRAPH_FARM_WORKER_ROLE_V1,runCapturedGraphFarmWorkerV1}from"../../🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🪆️captured/🟨️.mjs";\n'),
        ("/** 🔏️ Enters one authenticated verb kernel under the inherited Original without another capacity grant. */",
         "/** 🔏️ Enters one authenticated verb kernel under the inherited Original without another capacity grant; when the whole command line is the graph farm worker role it serves as a farm worker of the snapshot it runs from instead, which answers its parent over its channel and receives no invocation. */"),
        ("requireCapturedBuildCommandV1(proof,identity,loader);adoptCapturedBuildCommandV1(proof);proof.recheck();issueRepositoryScriptProvidersV1(process.env);",
         "requireCapturedBuildCommandV1(proof,identity,loader);adoptCapturedBuildCommandV1(proof);proof.recheck();if(args.length===1&&args[0]===GRAPH_FARM_WORKER_ROLE_V1)return runCapturedGraphFarmWorkerV1();issueRepositoryScriptProvidersV1(process.env);"),
    ],
    B + "🏛️sealed/🚪️entry/🟦️.ts": [
        ('import{sealedPolicyV1}from"../🟦️.ts";\n', 'import{sealedPolicyV1}from"../🟦️.ts";\nimport{GRAPH_FARM_WORKER_ROLE_V1}from"../../🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🪆️captured/🟨️.mjs";\n'),
        ("/** 🏛️ The one entry of the sealed implementation: the front door, the promotion check, or the verb kernel the command line selects;",
         "/** 🏛️ The one entry of the sealed implementation: the front door, the promotion check, a graph farm worker (served through the runtime kernel), or the verb kernel the command line selects;"),
        (" if(args[0]===sealedDoorRoleV1)return runDoorV1(args.slice(1),proof,sealedPolicyV1.readyDescriptor);\n",
         " if(args[0]===sealedDoorRoleV1)return runDoorV1(args.slice(1),proof,sealedPolicyV1.readyDescriptor);\n if(args.length===1&&args[0]===GRAPH_FARM_WORKER_ROLE_V1)return(await sealedKernelsV1.runtime()).runCapturedBuildSourceV1(args,proof);\n"),
    ],
}
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
def sha(data): return hashlib.sha256(data).hexdigest()
def main():
    root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
    for name in ("files", "base"): shutil.rmtree(os.path.join(HERE, name), ignore_errors=True); os.makedirs(os.path.join(HERE, name))
    manifest, failed = {}, False
    def put(rel, base, new):
        flat = "%02d" % len(manifest) + "-" + os.path.basename(os.path.dirname(rel)) + "-" + os.path.basename(rel)
        open(os.path.join(HERE, "files", flat), "wb").write(new)
        if base is not None: open(os.path.join(HERE, "base", flat), "wb").write(base)
        manifest[rel] = {"file": flat, "base": None if base is None else sha(base), "new": sha(new)}
    landed = "--landed" in sys.argv
    for rel, edits in EDITS.items():
        current = open(os.path.join(root, rel), "rb").read(); text = current.decode("utf8")
        for edit in edits:
            old, new = (edit[1], edit[0]) if landed else (edit[0], edit[1]); count = edit[2] if len(edit) > 2 else 1
            if text.count(old) != count: print("ANCHOR", text.count(old), "!=", count, rel[-60:], repr(old[:80])); failed = True; continue
            text = text.replace(old, new)
        put(rel, text.encode("utf8"), current) if landed else put(rel, current, text.encode("utf8"))
    for rel, source in NEW.items():
        target = os.path.join(root, rel)
        if landed: put(rel, None, open(target, "rb").read()); continue
        if os.path.exists(target): print("EXISTS", rel); failed = True; continue
        put(rel, None, open(os.path.join(HERE, "new", source), "rb").read())
    if failed: sys.exit(2)
    json.dump(manifest, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("staged", len(manifest), "files from", root)
main()
