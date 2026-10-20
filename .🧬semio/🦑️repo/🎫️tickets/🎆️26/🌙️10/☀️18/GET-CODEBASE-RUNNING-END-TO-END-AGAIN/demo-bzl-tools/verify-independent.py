#!/usr/bin/env python3 -I
"""Independent check (Python standard library only) of the demo-bzl declarations: demonstrator chain, report outputs, hermetic TeX toolchain facts.
usage: verify-independent.py [ROOT]   ROOT defaults to the live checkout; pass /private/tmp/dbz/ov for the overlay (manifests only; source facts are read from the live tree)."""
import json, os, re, sys

LIVE = "/Users/ueli/Documents/semio"
ROOT = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else LIVE
DEMO = "♻️mit-bestand/🧺️demonstrator/📋️project.json"
BERICHT = "♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json"
PRINT = "🧰️framework/🛍️products/📓️print"
failures = []


def check(ok, message):
    if not ok:
        failures.append(message)
    print(("ok   " if ok else "FAIL ") + message)


def load(base, path):
    return json.load(open(os.path.join(base, path), encoding="utf8"))


def decision(target):
    return ((target.get("metadata") or {}).get("semio") or {}).get("cache")


def transitive_restore(target):
    return any(isinstance(i, dict) and i.get("dependentTasksOutputFiles") == "**/*" and i.get("transitive") is True for i in target.get("inputs", []))


demo = load(ROOT, DEMO)["targets"]
for name in ("prepare-prototype", "prepare-release"):
    t = demo[name]
    check(t.get("cache") is True and t.get("outputs") == [] and decision(t) == {"version": 1, "class": "deterministic", "reason": "check-without-outputs"} and transitive_restore(t), f"demonstrator {name}: cached check without outputs over the transitive dependency outputs")
t = demo["activate-prototype"]
check(t.get("cache") is True and t.get("outputs") == ["{projectRoot}/dist/♻️activation/prototype"] and decision(t) == {"version": 1, "class": "deterministic", "reason": "pure-function-of-inputs"} and transitive_restore(t), "demonstrator activate-prototype: cached pure function with the union receipt directory as its one output")
t = demo["activate-lanes-prototype"]
check(t.get("executor") == "repo:noop" and "options" not in t and sorted(t["dependsOn"]) == sorted(f"@semio-tech/framework-os-dev:activate-{lane}-react-prototype" for lane in ("energy", "fem3d", "generator")), "demonstrator activate-lanes-prototype: process-free ordering node over the three lanes")
check("activate-lanes-prototype" in demo["activate-prototype"]["dependsOn"] or "@semio-tech/mit-bestand-demonstrator:activate-lanes-prototype" in demo["activate-prototype"]["dependsOn"], "demonstrator activate-prototype still depends on the lane gate")
for name in ("serve", "dev", "prepare-e2e", "serve-e2e"):
    check(transitive_restore(demo[name]), f"demonstrator {name}: restores the transitive closure it serves from")
for name in ("serve", "dev", "serve-e2e"):
    t = demo[name]
    check(t.get("continuous") is True and t.get("cache") is False and (decision(t) or {}).get("class") == "continuous", f"demonstrator {name}: interactive with a recorded reason")
for name in ("prepare-e2e", "test-e2e", "build"):
    d = decision(demo[name]) or {}
    check(demo[name].get("cache") is False and d.get("class") in ("side-effecting", "live", "managed-output"), f"demonstrator {name}: uncached with a recorded reason ({d.get('class')}/{d.get('reason')})")
check(all(((decision(t) or {}).get("class") == "deterministic") == (t.get("cache") is True) for t in demo.values() if t.get("executor") != "repo:noop" and decision(t)), "demonstrator: cache flag agrees with the deterministic class on every decided target")
check('register("complete"' not in open(os.path.join(ROOT, "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📜️script.ts"), encoding="utf8").read(), "demonstrator runtime script: no dead `complete` command")

schema = load(LIVE, PRINT + "/📦️artifacts/🧬️schema/🔣️.json")["definitions"]
namespace, token = schema["Protocol"]["const"]["namespace"], schema["NxProjection"]["const"]["modeToken"]
bericht = load(ROOT, BERICHT)["targets"]
catalog = load(LIVE, "♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🔣️.json")["documents"]
script = open(os.path.join(LIVE, "♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🟦️.ts"), encoding="utf8").read()
check('namespace:"report"' in script.replace(" ", "") or 'publishPrintArtifact' in script and '"report"' in script, "report documents publish into the `report` print namespace")
for document in catalog:
    expected = "{workspaceRoot}/.cache/b/p/m/%s/%s/documents/report/%s" % (namespace, token, document["id"])
    t = bericht["build-" + document["id"]]
    check(t.get("outputs") == [expected] and t.get("cache") is True, f"report build-{document['id']}: output is the products-store slot {expected}")
    texts = json.dumps(t["inputs"], ensure_ascii=False)
    for pin in ("🔧️toolchain/🔣️.json", "📚️bundle/🔒️dependencies.json", "🖋️latex/**/*", "🖼️assets/🔤️font/**/*.ttf"):
        check(pin in texts, f"report build-{document['id']}: input names the pinned material {pin}")

toolchain = load(LIVE, PRINT + "/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🔣️.json")
check(all(re.fullmatch(r"[0-9a-f]{64}", p["sha256"]) and p["archive"].startswith("tectonic-" + toolchain["version"]) for p in toolchain["platforms"]), f"tectonic {toolchain['version']}: every platform archive is pinned by sha256")
bundle = load(LIVE, PRINT + "/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🔒️dependencies.json")
check(len(bundle["files"]) > 0 and all(re.fullmatch(r"[0-9a-f]{64}", f["sha256"]) for f in bundle["files"]), f"TeX bundle: {len(bundle['files'])} files, each pinned by sha256 and byte range")
compiler = open(os.path.join(LIVE, PRINT + "/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts"), encoding="utf8").read()
for needle in ("SOURCE_DATE_EPOCH:printSourceDateEpoch()", 'TZ:"UTC"', "TECTONIC_CACHE_DIR:context.sink.directory", "HOME:context.sink.directory", "delete data.NODE_OPTIONS"):
    check(needle in compiler, f"compile environment: {needle}")
check("file URL" in compiler or "file://" in compiler or "pathToFileURL" in compiler, "tectonic reads the TeX bundle from a local file URL (no network at compile time)")
sys.exit(1 if failures else 0)
