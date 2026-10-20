#!/usr/bin/env python3
# Builds the modified live files as full copies from the CURRENT live text by anchored edits; fails when an anchor is gone or ambiguous. Writes staged/mod/<flat> and staged/bases.json.
import hashlib, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
layout = json.load(open(HERE + "/layout.json", encoding="utf-8"))
MOD = layout["modified"]
def sha(b): return hashlib.sha256(b).hexdigest()
def once(text, anchor, label):
    n = text.count(anchor)
    if n != 1: raise SystemExit("anchor %s matched %d times: %r" % (label, n, anchor[:70]))
def replace(text, anchor, new, label):
    once(text, anchor, label); return text.replace(anchor, new)
def after(text, anchor, add, label): return replace(text, anchor, anchor + add, label)
def before(text, anchor, add, label): return replace(text, anchor, add + anchor, label)

def verify(t):
    return before(t, '    if (segments[0] === "typescript") {', '''    if (segments[0] === "build-parity") {
      const { runBuildParityV1 } = await import("../../🏗️build/🪞️parity/⌨️command/🟦️.ts");
      await runBuildParityV1(this.root, segments.slice(1), this.invocation);
      return;
    }
''', "verify")

def nx_operations(t):
    return after(t, ' if(args[0]==="cache")return args[1]==="clear"?["reset","--onlyCache"]:undefined;', '''
 if(args[0]==="inputs"){if(args.length!==2||!/^[^\\s:]+:[^\\s:]+(?::[^\\s:]+)?$/u.test(args[1]!))throw Error("Expected repo inputs <project:target[:configuration]>");return ["show","target","inputs",args[1]!,"--json"];}''', "nx-operations")

def neutral(t):
    return before(t, '    const selection=await import("./🧭️selection/🟦️.ts");\n    process.env.SEMIO_BUILD_REUSE_GRAPH="true";', '    if(command==="inputs")throw new Error("repo inputs is implemented for Nx; Bazel records the inputs of its spawns for repo verify build-parity");\n', "neutral")

def bazel_adapter(t):
    t = after(t, 'import {bazelBuildEventsFileV1,recordBazelBuildMetricsV1} from "./⚙️operations/🟦️.ts";', '\nimport {bazelEvidenceDirectoryV1,bazelEvidenceFlagsV1,writeBazelClosureEvidenceV1} from "./👁️evidence/🟦️.ts";', "adapter-import")
    t = before(t, '   const batch=bazelCommandPlanV1({mode:"execute",', '   const evidence=bazelEvidenceDirectoryV1(process.env);\n', "adapter-evidence")
    t = replace(t, 'building=runOwnedCommand(bazel,[...startup,batch.verb,...batch.options,...batch.targets]', 'building=runOwnedCommand(bazel,[...startup,batch.verb,...batch.options,...bazelEvidenceFlagsV1(evidence),...batch.targets]', "adapter-flags")
    t = before(t, '   for(const id of plan.selected){const action=emitted.get(id)!;if(action.kind==="task"||action.kind==="hermetic")await restoreBazelSnapshotV1(root,join(bin,action.label+".snapshot"),invocation,phaseControl);}', '   if(evidence!==undefined&&batch)await writeBazelClosureEvidenceV1(evidence,{plan,query:expression=>captureOwnedCommandV1(bazel,[...startup,"cquery",...options.filter(option=>!settings.flags.includes(option)),"--output=label",expression],workspace,{signal,control:phaseControl,env:client()})});\n', "adapter-closure")
    return t

def bazel_operations(t):
    t = replace(t, 'import {chmod,lstat,mkdir,readdir,readFile,rename,rm,rmdir,stat,unlink,writeFile} from "node:fs/promises";', 'import {chmod,copyFile,lstat,mkdir,readdir,readFile,rename,rm,rmdir,stat,unlink,writeFile} from "node:fs/promises";', "ops-fs")
    t = after(t, 'import schema from "./🧬️schema/🔣️.json" with {type:"json"};', '\nimport {bazelEvidenceDirectoryV1,bazelEvidenceEventsPathV1} from "../👁️evidence/🟦️.ts";', "ops-import")
    return replace(t, '  await rm(events,{force:true});\n  const metrics=bazelBuildMetricsV1(text);', '  const evidence=bazelEvidenceDirectoryV1(process.env);\n  if(evidence!==undefined)await copyFile(events,bazelEvidenceEventsPathV1(evidence));\n  await rm(events,{force:true});\n  const metrics=bazelBuildMetricsV1(text);', "ops-events")

INPUTS_OPERATION = '''    {
      "name": "inputs",
      "synopsis": "inputs <project:target[:configuration]>",
      "summary": { "en": "Print what the selected implementation hashes for one task: files, environment, runtime values and external packages.", "de": "Gibt aus, was die gewählte Implementierung für eine Task hasht: Dateien, Umgebung, Laufzeitwerte und externe Pakete." },
      "implementations": { "nx": "adapter", "bazel": "missing" },
      "dryRun": false,
      "progress": true,
      "cancellable": true,
      "flags": [],
      "examples": [{ "command": "repo --tool=nx inputs @teaching/architecture-course:test", "description": { "en": "List the inputs Nx hashes for the course tests.", "de": "Listet die Eingaben auf, die Nx für die Kurstests hasht." } }],
      "notes": ["Prints JSON with files, environment, runtime, depOutputs and external. Bazel does not implement this operation: it records the inputs and environment of its spawns in the evidence directory of repo verify build-parity instead."]
    },
'''
def catalog(t):
    t = before(t, '    {\n      "name": "reset",', INPUTS_OPERATION, "catalog")
    return t

def operations_schema(t):
    return replace(t, '"OperationName": {\n      "enum": ["run", "run-many", "affected", "exec", "watch", "graph", "show", "reset",', '"OperationName": {\n      "enum": ["run", "run-many", "affected", "exec", "watch", "graph", "show", "inputs", "reset",', "operations-schema")

TARGET = '''    "verify-build-parity": {
      "executor": "repo:script",
      "cache": false,
      "dependsOn": [],
      "options": {
        "command": "bun ./📜️script.ts verify build-parity",
        "forwardAllArgs": true
      },
      "outputs": [],
      "metadata": {
        "semio": {
          "cache": { "version": 1, "class": "aggregate", "reason": "reenters-build-tool" }
        }
      }
    },
'''
TOOL = '''          {
            "id": "verify-build-parity",
            "title": { "en": "Verify Nx and Bazel parity", "de": "Parität von Nx und Bazel prüfen" },
            "description": { "en": "Run the sample targets with both build tools into separate output roots and compare task graph, declared outputs, exit codes, test results and inputs; every difference that no documented rule explains fails.", "de": "Die Stichproben mit beiden Build-Werkzeugen in getrennte Ausgabewurzeln ausführen und Aufgabengraph, deklarierte Ausgaben, Exit-Codes, Testergebnisse und Eingaben vergleichen; jede Abweichung ohne dokumentierte Regel schlägt fehl." },
            "verb": "verify",
            "command": ["bun", "repo", "verify", "build-parity", "--only={only}"],
            "parameters": [
              { "id": "only", "title": { "en": "Samples", "de": "Stichproben" }, "description": { "en": "Comma list of sample ids or area:<area>; empty verifies every sample.", "de": "Kommaliste von Stichproben-IDs oder area:<Bereich>; leer prüft jede Stichprobe." }, "kind": "text", "default": "" },
              { "id": "plans-only", "title": { "en": "Compare only the planned task graphs", "de": "Nur die geplanten Aufgabengraphen vergleichen" }, "kind": "flag", "args": ["--plans-only"] }
            ]
          },
'''
def root_project(t):
    t = before(t, '    "verify-rust-warnings": {', TARGET, "root-target")
    t = before(t, '          {\n            "id": "build-deps-sync",', TOOL, "root-tool")
    json.loads(t)
    return t

NX_TEST = '''test("inputs maps to the Nx command that prints the hash inputs of one task and refuses everything else",()=>{
 for(const row of fixture.inputs)expect(nxOperationArgumentsV1(row.args),row.name).toEqual(row.nx);
 for(const args of fixture.inputsRefused)expect(()=>nxOperationArgumentsV1(args),args.join(" ")).toThrow("Expected repo inputs");
 const source=readFileSync(installed.resolve("nx/src/command-line/show/command-object.js"),"utf8");
 expect(source).toContain("inputs [target]");expect(source).toContain("json");
});

'''
def nx_operations_test(t):
    return before(t, 'test("dry-run and graph translate to the Nx graph on stdout or a file and exec exports the neutral names",()=>{', NX_TEST, "nx-test")

NX_FIXTURE = '''  "inputs": [
    { "name": "one task", "args": ["inputs", "@teaching/architecture-course:test"], "nx": ["show", "target", "inputs", "@teaching/architecture-course:test", "--json"] },
    { "name": "one task with a configuration", "args": ["inputs", "p:build:release"], "nx": ["show", "target", "inputs", "p:build:release", "--json"] }
  ],
  "inputsRefused": [
    ["inputs"],
    ["inputs", "plain"],
    ["inputs", "p:t", "extra"],
    ["inputs", "p:t:c:d"]
  ],
'''
def nx_operations_fixture(t):
    t = before(t, '  "planning": [', NX_FIXTURE, "nx-fixture")
    json.loads(t)
    return t

FILES = {"verify.ts": verify, "nx-operations.ts": nx_operations, "nx-operations-test.ts": nx_operations_test, "nx-operations-fixture.json": nx_operations_fixture, "neutral.ts": neutral, "bazel-adapter.ts": bazel_adapter, "bazel-operations.ts": bazel_operations, "catalog.json": catalog, "operations-schema.json": operations_schema, "root-project.json": root_project}

def help_snapshots():
    import subprocess
    if not os.path.exists("/private/tmp/parity/ov/gen-help.ts"):
        subprocess.run([sys.executable, HERE + "/overlay.py"], check=True)
        open("/private/tmp/parity/ov/gen-help.ts", "wb").write(open(HERE + "/gen-help.ts", "rb").read())
    subprocess.run(["bun", "gen-help.ts", HERE + "/staged/mod/catalog.json", HERE + "/staged/mod/help-snapshots.json"], cwd="/private/tmp/parity/ov", check=True)
    live = os.path.join(ROOT, MOD["help-snapshots.json"])
    return sha(open(live, "rb").read())

def main():
    os.makedirs(HERE + "/staged/mod", exist_ok=True)
    bases = {}
    for flat, fn in FILES.items():
        live = os.path.join(ROOT, MOD[flat]); raw = open(live, "rb").read(); bases[flat] = sha(raw)
        out = fn(raw.decode("utf-8"))
        open(HERE + "/staged/mod/" + flat, "w", encoding="utf-8").write(out)
        print("built", flat, len(raw), "->", len(out.encode("utf-8")))
    bases["help-snapshots.json"] = help_snapshots()
    print("built help-snapshots.json")
    json.dump(bases, open(HERE + "/staged/bases.json", "w"), indent=1)

if __name__ == "__main__": main()
