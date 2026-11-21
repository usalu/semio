"""🧷 Stages closure-scoped failed pass provenance against exact live preimages."""

import hashlib
import json
from pathlib import Path
import sys

ROOT = Path("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts").parent
TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated" / "partial-prov"


def candidates(baseline=False):
    driver = ROOT / "🟦️.ts"
    test = ROOT / "🧪️tests/🟦️.ts"
    fixture = ROOT / "🧪️tests/🧫️fixtures/🔣️.json"
    source = driver.read_bytes()
    original = source.decode("utf8")
    first = "        const written = await session.write(observationPath,"
    last = "        const closures = unitGraphClosuresV1(graph, pass);"
    assert original.count(first) == original.count(last) == 1
    start, end = original.index(first), original.index(last) + len(last)
    updated = original if baseline else original[:start] + "        const graph = receiptGraph ?? await graphOf(pass.id);\n        let closures = unitGraphClosuresV1(graph, pass);\n        let observedUnits = units, observedScripts = buildScripts;\n        const partial = exit !== 0 && linked.size > 0;\n        if (partial) {\n          const linkedClosures = new Map<string, ReadonlySet<string>>();\n          const selected = new Set<string>();\n          for (const name of linked.keys()) {\n            const closure = closures.get(name);\n            if (!closure?.size) throw Error(`${name}: the unit graph of ${pass.id} has no linked root`);\n            linkedClosures.set(name, closure);\n            for (const identity of closure) selected.add(identity);\n          }\n          closures = linkedClosures;\n          const missing = new Set(selected);\n          const includes = (identity: string): boolean => {\n            missing.delete(identity);\n            return selected.has(identity);\n          };\n          observedUnits = units.filter(({message}) => includes(announcedUnitIdentityV1(message, pass.target)));\n          observedScripts = buildScripts.filter((message) => {\n            const run = message as { package_id?: unknown; out_dir?: unknown } | null;\n            if (typeof run?.package_id !== \"string\" || typeof run.out_dir !== \"string\") throw Error(\"Unidentified Cargo build-script run in a partial pass\");\n            return includes(announcedRunIdentityV1(run.package_id, run.out_dir, pass.target));\n          });\n          if (missing.size) throw Error(`${pass.id}: linked Cargo provenance missing ${missing.size} unit(s), including ${missing.values().next().value}`);\n        }\n        const written = await session.write(observationPath, { manifest: pass.manifest, cwd: options.cwd, command: port.command, args: [...port.args, ...args], buildDirectory: generation.build, builtAtMs: startMs, status: exit, cancelled: false, units: observedUnits, buildScripts: observedScripts }, stagedPaths, cargoHome,inputs.control);\n        if (partial) {\n          const verified = new Set(written.units.flatMap(unit => unit.artifacts.filter(artifact => artifact.sha256 && artifact.stagedSha256 === artifact.sha256 && stagedPaths.get(artifact.path) === artifact.stagedPath).map(artifact => artifact.path)));\n          for (const [name, wasm] of linked) if (!verified.has(wasm)) throw Error(`${name}: linked component bytes lack an observed matching staged artifact`);\n        }\n        const { leaves, sources } = invocationProvenanceLeavesV1(written, pass.target, roots);" + original[end:]
    cases = json.loads((TICKET / "🔣️partial-provenance.json").read_text(encoding="utf8"))
    payload = fixture.read_bytes()
    previous = payload.decode("utf8")
    assert previous.startswith("{") and "partialProvenance" not in json.loads(previous)
    next_fixture = "{\n  \"partialProvenance\": " + json.dumps(cases, ensure_ascii=False, separators=(",", ":")) + "," + previous[1:]
    suite = test.read_bytes()
    next_suite = suite + b"\n" + (TICKET / "📜️partial-provenance-law.ts").read_bytes()
    return {driver:(source,updated.encode("utf8")),test:(suite,next_suite),fixture:(payload,next_fixture.encode("utf8"))}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def stage(action):
    baseline = action == "--baseline"
    out = (OUTPUT / "base") if baseline else OUTPUT
    files = candidates(baseline)
    paths = dict(zip(files, ("driver.ts","test.ts","cases.json")))
    receipt = {str(path):{"before":digest(before),"after":digest(after),"overlay":paths[path]} for path,(before,after) in files.items()}
    stored = out / "receipt.json"
    if action in ("--baseline", "--prepare"):
        out.mkdir(parents=True, exist_ok=True)
        for path, (_, after) in files.items():
            (out / paths[path]).write_bytes(after)
        stored.write_text(json.dumps(receipt, ensure_ascii=False) + "\n", encoding="utf8")
    elif action in ("--verify", "--apply-reviewed"):
        assert json.loads(stored.read_text(encoding="utf8")) == receipt
        for path,(before,after) in files.items():
            assert len(str(path.resolve()).encode("utf-16-le")) // 2 <= 256, path
            assert path.read_bytes() == before and (out / paths[path]).read_bytes() == after
        if action == "--apply-reviewed":
            for path,(before,after) in files.items():
                assert path.read_bytes() == before, path
                path.write_bytes(after)
            assert all(path.read_bytes() == after for path, (_, after) in files.items()), "released provenance sources changed"
    else:
        raise ValueError(action)
    print(f"[DEBUG] {action} {len(files)} exact provenance paths in {out}")


if __name__ == "__main__":
    stage(sys.argv[1])
