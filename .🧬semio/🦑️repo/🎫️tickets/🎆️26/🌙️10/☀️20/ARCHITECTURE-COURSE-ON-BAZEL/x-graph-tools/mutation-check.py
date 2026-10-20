#!/usr/bin/env python3
"""Red/green proof for the staged suite: each mutation of the applied mirror must turn the named test red; the unmutated mirror must be green.

usage: mutation-check.py MIRROR_DIR      (MIRROR_DIR is a tree built with `apply.py mirror DIR`; every mutation is reverted)
"""
import os
import shutil
import subprocess
import sys

L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
ADMISSION = L + "/🕸️graph/🚪️admission/🟦️.ts"
GRAPH = L + "/🕸️graph/🟦️.ts"
TEST = L + "/🕸️graph/🚪️admission/🧪️tests/🗃️closure-cache/🟦️.ts"
MUTATIONS = [
    ("no persisted owner records", ADMISSION, "const facts=await ports.observe({cachePath:join(ports.directory,record.leaf),key:record.key,owner:admissionCacheV1.recordNamespace+\":\"+owner.name,reusable:value=>ownerFactsValidatorV1().validate(value).length===0},async()=>slimBuildOwnerAdmissionFactsV1(await compute()));", "const facts=slimBuildOwnerAdmissionFactsV1(await compute());", "cross-process reuse"),
    ("records keep generated target bodies", ADMISSION, "row.dependsOn===undefined?{}:{dependsOn:row.dependsOn}", "row", "records keep only what the closure reads"),
    ("implementation missing from the record key", ADMISSION, "[admissionCacheV1.recordVersion,implementation,scope,registry,", "[admissionCacheV1.recordVersion,\"\",scope,registry,", "record leaves and keys"),
    ("registry missing from the record key", ADMISSION, "[admissionCacheV1.recordVersion,implementation,scope,registry,", "[admissionCacheV1.recordVersion,implementation,scope,\"\",", "record leaves and keys"),
    ("targets missing from the record leaf", ADMISSION, "JSON.stringify([targets,inputs])", "JSON.stringify([[],inputs])", "record leaves and keys"),
    ("closure admits every owner", ADMISSION, "if(selection.kind===\"all\")for(const owner of registry.owners)admit(owner.name);else for(", "if(true)for(const owner of registry.owners)admit(owner.name);else for(", "closure selection equals"),
    ("ctime ignored by the racy rule", ADMISSION, "(stamp.mtimeMs??0)>=limit||(stamp.ctimeMs??0)>=limit", "(stamp.mtimeMs??0)>=limit", "receipts are closed"),
    ("failure chain not appended", ADMISSION, "error.message+=\"\\nBuild owner admission chain: \"+chain.join(\" <- \");", "void chain;", "failing owner names"),
    ("receipt scope not checked", ADMISSION, "receipt.scope!==expected.scope||", "", "receipts are closed"),
    ("glue never reuses", GRAPH, "if(reused)return reused;", "void reused;", "graph attempt"),
    ("glue forgets the closure it derived", GRAPH, "admissionMemo={admission,observations:captured.observations,startedAt};", "void admission;", "graph attempt"),
    ("glue reuses without checking facts", GRAPH, "if(!await buildGraphSourceFactsHoldV1(known.observations,invocation))return undefined;", "", "graph attempt"),
    ("glue persists before the graph", GRAPH, "\n  if(selected&&initial.admission&&!initial.persisted)await publishAdmission(initial.admission,initial.observations,initial.startedAt);\n  return graph;", "\n  return graph;", "graph attempt"),
]


def run(mirror, filter_name):
    environment = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=os.environ.get("SEMIO_TEST_ARTIFACT_DIR", os.path.dirname(mirror)))
    command = ["bun", "test", "./" + TEST] + (["-t", filter_name] if filter_name else [])
    result = subprocess.run(command, cwd=mirror, env=environment, capture_output=True, text=True)
    return result.returncode, (result.stdout + result.stderr)[-400:]


def main():
    mirror = os.path.abspath(sys.argv[1])
    code, tail = run(mirror, None)
    print("%-48s %s" % ("baseline (must be green)", "green" if code == 0 else "RED"))
    if code != 0:
        print(tail)
        return 1
    failures = 0
    for label, relative, old, new, name in MUTATIONS:
        path = os.path.join(mirror, relative)
        with open(path, encoding="utf-8") as handle:
            original = handle.read()
        if original.count(old) != 1:
            print("%-48s ANCHOR MISSING (%d)" % (label, original.count(old)))
            failures += 1
            continue
        backup = path + ".mutation-backup"
        shutil.copyfile(path, backup)
        try:
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(original.replace(old, new))
            code, _ = run(mirror, name)
        finally:
            os.replace(backup, path)
        verdict = "red (killed)" if code != 0 else "GREEN (SURVIVED)"
        failures += code == 0
        print("%-48s %s  [%s]" % (label, verdict, name))
    return 1 if failures else 0


sys.exit(main())
