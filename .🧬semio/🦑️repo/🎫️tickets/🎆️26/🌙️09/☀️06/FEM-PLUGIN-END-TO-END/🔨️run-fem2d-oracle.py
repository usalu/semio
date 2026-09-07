#!/usr/bin/env python3
"""🔨️ Ticket driver for the fem2d solver-oracle case.

`generate` produces the committed reference `📊️expected.results.json` by calling the committed
oracle's own functions — the reference is never authored by hand and never by this file's own
arithmetic. `run` builds a repository-test plan for every scenario the oracle registers and executes
it through the repository's real Python host
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py`), which is the same
host the coordinator drives, so a green run here is a green run there.

    uv run python 🔨️run-fem2d-oracle.py generate
    uv run python 🔨️run-fem2d-oracle.py run
"""

import importlib.util
import json
import math
import os
import subprocess
import sys

# 🧹️ A test case directory may hold exactly one feature file, its adapters and one 🧫️fixtures
# directory; a `__pycache__` written next to the oracle would be an unknown case child.
sys.dont_write_bytecode = True

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
HOST = os.path.join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py")
CASE = os.path.join(REPO, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧪️tests/🧮️solves-fem2d-1-benchmarks")
FIXTURES = os.path.join(CASE, "🧫️fixtures")
WORK = os.path.join(os.path.dirname(__file__), "🗑️generated", "fem2d-oracle")


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def oracle_module():
    """🔮️ The committed oracle, imported with the repository's own test host bound as
    `semio_repo_test` exactly as the host itself binds it."""
    host = load(HOST, "semio_repo_test")
    sys.modules["semio_repo_test"] = host
    return load(os.path.join(CASE, "🐍️.py"), "fem2d_benchmarks_oracle")


def snapshot(name):
    with open(os.path.join(FIXTURES, name), encoding="utf-8") as handle:
        return json.load(handle)


def closed_forms(oracle, fixture, document):
    """📐️ The hand-derivable claims each benchmark carries, in SI units."""
    if fixture == "📏️steel-cantilever.snapshot.json":
        members = oracle.members_of(document)
        e, _, inertia, _ = oracle.properties(document, members[0])
        span = sum(oracle.geometry(document, member)[0] for member in members)
        load = document["loadCases"][0]["loads"][0]
        return [{"what": "PL³/(3EI) tip deflection", "case": "tip", "quantity": "displacement", "node": load["nodeId"], "dof": "Ty", "value": load["value"] * span**3 / (3.0 * e * inertia), "tolerance": 1e-9}]
    if fixture == "📐️steel-simple-beam.snapshot.json":
        members = oracle.members_of(document)
        e, _, inertia, _ = oracle.properties(document, members[0])
        span = sum(oracle.geometry(document, member)[0] for member in members)
        intensity = document["loadCases"][0]["loads"][0]["wy"]
        return [{"what": "5wL⁴/(384EI) midspan deflection", "case": "udl", "quantity": "displacement", "node": "m", "dof": "Ty", "value": 5.0 * intensity * span**4 / (384.0 * e * inertia), "tolerance": 1e-9}]
    if fixture == "🌉️concrete-two-span.snapshot.json":
        intensity, span = 25000.0, 6.0
        return [
            {"what": "1.25wL centre reaction of a two-span continuous beam", "case": "dead", "quantity": "reactions", "reactions": ["n2.Ty"], "value": 1.25 * intensity * span, "tolerance": 1e-9},
            {"what": "0.375wL end reaction of a two-span continuous beam", "case": "dead", "quantity": "reactions", "reactions": ["n0.Ty"], "value": 0.375 * intensity * span, "tolerance": 1e-9},
        ]
    return []


def generate():
    oracle = oracle_module()
    reference = {
        "schema": "semio.fem2d.analysis-reference/v1",
        "_comment": "🧮️ Committed linear-static, modal and buckling reference values for `s.fem.fem2d`, produced by `🐍️.py` beside this file and required by it to agree with anastruct before emission. SI units: metres, radians (counter-clockwise positive), newtons, newton-metres; +y is up; a reaction is the force the support applies to the structure. `scales` are the normalisation decades the cross-language projection divides by, so one absolute comparison tolerance can serve displacements and reactions at once.",
        "producedBy": {"oracle": "anastruct-fem2d-solver", "package": "anastruct", "eigen": "scipy.linalg", "gravity": oracle.GRAVITY},
        "fixtures": {},
        "mutated": {},
    }

    for _, fixture in oracle.BENCHMARKS:
        document = snapshot(fixture)
        results = oracle.agree_with_anastruct(document, oracle.reference_all(document))
        reference["fixtures"][fixture] = {
            "scales": oracle.scales_of(results),
            "cases": oracle.raw_of(document, results),
            "closedForm": closed_forms(oracle, fixture, document),
        }
        print("[generate] %s: %d result keys, anastruct agrees" % (fixture, len(results)))

    corpus = snapshot("🧬️mutated.snapshots.json")
    for kind in oracle.KINDS:
        document = corpus["kinds"][kind]["after"]
        results = oracle.agree_with_anastruct(document, oracle.reference_all(document))
        reference["mutated"][kind] = {"scales": oracle.scales_of(results), "summary": oracle.summary_of(results)}
        print("[generate] mutated %s: %d result keys, anastruct agrees" % (kind, len(results)))

    cantilever = snapshot("📏️steel-cantilever.snapshot.json")
    frequencies = oracle.modal_frequencies(cantilever, cantilever["analysis"]["modalCount"])
    reference["modal"] = {"scale": oracle.decade(max(frequencies)), "frequenciesHz": frequencies}
    print("[generate] modal frequencies: %s" % ["%.6f" % value for value in frequencies])

    columns = snapshot("🏛️steel-columns.snapshot.json")
    factors = {}
    for case in columns["loadCases"]:
        factors[case["id"]] = oracle.buckling_factors(columns, case["id"], columns["analysis"]["bucklingCount"])[0]
    reference["buckling"] = {"scale": oracle.decade(max(factors.values())), "factors": factors}
    print("[generate] buckling factors: %s" % {key: "%.6f" % value for key, value in factors.items()})

    path = os.path.join(FIXTURES, "📊️expected.results.json")
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(reference, handle, indent=2, ensure_ascii=False)
        handle.write("\n")
    print("[generate] wrote %s (%d bytes)" % (os.path.relpath(path, REPO), os.path.getsize(path)))
    return 0


def scenarios(oracle):
    """🎬️ Every scenario the committed oracle registers, in registration order."""
    built = oracle.adapter()
    return [key[: -len("::oracle")] for key in built._handlers if key.endswith("::oracle")]


def run():
    oracle = oracle_module()
    os.makedirs(WORK, exist_ok=True)
    relative_case = os.path.relpath(CASE, REPO)
    plan = {
        "owner": os.path.relpath(os.path.join(CASE, "..", ".."), REPO),
        "case": "🧮️solves-fem2d-1-benchmarks",
        "implementation": "python",
        "role": "oracle",
        "workDir": WORK,
        "outputDir": os.path.join(WORK, "out"),
        "artifactDir": os.path.join(WORK, "📦️artifacts"),
        "platform": sys.platform,
        "baselineSha": "",
        "featureHash": "",
        "target": {"artifact": "s.fem.fem2d", "standard": "1", "subset": "analysis"},
        "fixtures": [
            {"uri": "local://%s" % name, "path": os.path.join(relative_case, "🧫️fixtures", name)}
            for name in sorted(os.listdir(FIXTURES))
        ],
        "scenarios": [{"id": name, "level": "exhaustive", "steps": [{"text": " ".join("local://%s" % fixture for fixture in sorted(os.listdir(FIXTURES))), "docString": ""}]} for name in scenarios(oracle)],
    }
    plan_path = os.path.join(WORK, "📋️plan.json")
    with open(plan_path, "w", encoding="utf-8") as handle:
        json.dump(plan, handle, ensure_ascii=False)
    results_path = os.path.join(WORK, "📤️results.jsonl")
    environment = dict(os.environ, PYTHONDONTWRITEBYTECODE="1")
    completed = subprocess.run([sys.executable, HOST, "--plan", plan_path, "--out", results_path, "--adapter", os.path.join(CASE, "🐍️.py")], capture_output=True, text=True, env=environment)
    if completed.stdout.strip():
        print(completed.stdout.strip())
    if completed.stderr.strip():
        print(completed.stderr.strip(), file=sys.stderr)
    passed = failed = 0
    with open(results_path, encoding="utf-8") as handle:
        for line in handle:
            result = json.loads(line)
            status = result["status"]
            if status == "passed":
                passed += 1
                print("  ✅ %-44s %8.1f ms" % (result["scenario"], result["durationMs"]))
            else:
                failed += 1
                print("  ❌ %-44s %s" % (result["scenario"], (result["diagnostics"] or [{}])[0].get("message", "")))
    print("[run] %d passed, %d failed (host exit %d)" % (passed, failed, completed.returncode))
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    command = sys.argv[1] if len(sys.argv) > 1 else "run"
    sys.exit(generate() if command == "generate" else run())
