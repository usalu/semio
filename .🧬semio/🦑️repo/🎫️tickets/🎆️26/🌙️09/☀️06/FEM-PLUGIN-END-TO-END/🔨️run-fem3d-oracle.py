#!/usr/bin/env python3
"""🔮 Runs the three fem3d analysis-oracle cases out of band and commits their answers.

The repository's own test coordinator runs this adapter in the oracle ROLE against the Rust subject.
This driver does the same thing without the coordinator, for all three of them
(`🧮️solves-fem3d-1-benchmarks` / PyNite, `🎵️solves-fem3d-1-eigen` / SciPy,
`🧱️solves-fem3d-1-solid` / scikit-fem): it builds the plan each scenario would have been given, calls
the very same handlers, and writes their projections to each case's committed
`🧫️fixtures/📊️expected.results.json`. Those files are what the Rust side asserts against, so they must
be produced by the third-party solvers and never by hand. Each scenario contributes both halves of
its outcome: the `projection` the coordinator compares across languages, and the full-precision
`reference` the subject is held to numerically.

Run: `uv run python .🧬semio/…/FEM-PLUGIN-END-TO-END/🔨️run-fem3d-oracle.py`
"""

# region 🔖️Imports
import importlib.util
import json
import os
import sys
import traceback

# endregion 🔖️Imports


# region 🔖️Wiring
REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
HOST = os.path.join(REPO, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🧪️test", "📦️packages", "🐍️python", "🐍️.py")
SUBSETS = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets")
CASES = {name: os.path.join(SUBSETS, "📈️analysis", "🧪️tests", name) for name in ("🧮️solves-fem3d-1-benchmarks", "🎵️solves-fem3d-1-eigen", "🧱️solves-fem3d-1-solid")}


def load(path, name):
    """📦️ Loads one file as a module."""
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


HOST_MODULE = load(HOST, "semio_repo_test")
Context = HOST_MODULE.Context


def scenario_context(case, scenario_id, fixtures):
    """🧭️ The plan slice one scenario would have been handed, with its declared fixtures."""
    steps = [("Given", "the committed model " + uri) for uri, _path in fixtures]
    plan = {"case": case, "workDir": os.path.join(REPO, ".🧬semio", "🦑️repo", "⚡️cache", "tests", "work", "fem3d-analysis"), "fixtures": [{"uri": uri, "path": path} for uri, path in fixtures]}
    return Context(plan, {"id": scenario_id, "steps": [{"keyword": keyword, "text": text} for keyword, text in steps]}, "oracle", REPO)


# endregion 🔖️Wiring


# region 🔖️Run
BENCHMARKS, EIGEN, SOLID = "🧮️solves-fem3d-1-benchmarks", "🎵️solves-fem3d-1-eigen", "🧱️solves-fem3d-1-solid"

PLAN = {
    BENCHMARKS: [
        ("static-steel-frame", ["🧊️steel-frame.snapshot.json"]),
        ("static-space-frame", ["🏢️space-frame-2x2-bay.snapshot.json"]),
        ("static-roof-truss", ["🏛️timber-roof-truss.snapshot.json"]),
        ("closed-form-cantilever-local-z", ["📏️cantilever-tip-load-local-z.snapshot.json"]),
        ("closed-form-cantilever-local-y", ["📐️cantilever-tip-load-local-y.snapshot.json"]),
        ("closed-form-cantilever-torsion", ["🌀️cantilever-tip-torsion.snapshot.json"]),
        ("closed-form-simply-supported-udl", ["🌉️simply-supported-udl.snapshot.json"]),
    ],
    EIGEN: [
        ("modal-cantilever", ["🎵️modal-cantilever.snapshot.json"]),
        ("buckling-column-pinned-pinned", ["🏛️buckling-column-pinned-pinned.snapshot.json"]),
        ("buckling-column-fixed-pinned", ["🏛️buckling-column-fixed-pinned.snapshot.json"]),
        ("buckling-column-fixed-fixed", ["🏛️buckling-column-fixed-fixed.snapshot.json"]),
        ("buckling-column-fixed-free", ["🏛️buckling-column-fixed-free.snapshot.json"]),
    ],
    SOLID: [("solid-prismatic-column", ["🧱️prismatic-solid-column.snapshot.json"])],
}
"""📋️ Every scenario each case declares, with the fixtures its feature names."""


def run_case(name):
    """🔮️ Runs one case's oracle over its whole plan and writes its committed reference."""
    module = load(os.path.join(CASES[name], "🐍️.py"), "semio_fem3d_" + name[1:].replace("-", "_"))
    scenarios = list(PLAN[name])
    if name == BENCHMARKS:
        for kind in module.KINDS:
            scenarios.append(("mutate-solve-" + kind, ["🦠️mutation-base.snapshot.json", "🦠️mutation-after-%s.snapshot.json" % kind]))
    adapter, results, failures = module.adapter(), {}, []
    print("== %s (%d scenarios)" % (name, len(scenarios)))
    for scenario_id, names in scenarios:
        fixtures = [("local://" + fixture, os.path.relpath(os.path.join(CASES[name], "🧫️fixtures", fixture), REPO)) for fixture in names]
        handler = adapter.handler(scenario_id, "oracle")
        if handler is None:
            failures.append((scenario_id, "no oracle handler is registered"))
            continue
        try:
            outcome = handler(scenario_context(name, scenario_id, fixtures))
            results[scenario_id] = {"projection": outcome.projection, "reference": json.loads(outcome.raw.decode("utf-8"))}
            print("  ok   %s" % scenario_id)
        except Exception as error:  # noqa: BLE001 — every failure is reported, none is swallowed
            failures.append((scenario_id, "%s: %s" % (type(error).__name__, error)))
            print("  FAIL %s -> %s: %s" % (scenario_id, type(error).__name__, error))
            traceback.print_exc()
    for scenario_id, reason in failures:
        print("  failed: %s -> %s" % (scenario_id, reason))
    if failures:
        return False
    path = os.path.join(CASES[name], "🧫️fixtures", "📊️expected.results.json")
    with open(path, "w", encoding="utf-8") as handle:
        json.dump({"schema": "semio.fem3d.analysis-reference/v1", "producedBy": ORACLES[name], "scenarios": results}, handle, indent=2, ensure_ascii=False, sort_keys=True)
        handle.write("\n")
    print("  wrote %s (%d bytes)\n" % (os.path.relpath(path, REPO), os.path.getsize(path)))
    return True


ORACLES = {BENCHMARKS: ["pynite-fem3d-solver"], EIGEN: ["scipy-fem3d-eigen"], SOLID: ["skfem-fem3d-solid"]}
"""🔮️ Which registered oracle produced each case's reference."""


def main():
    """🚀️ Runs every case's oracle and commits its reference."""
    return 0 if all([run_case(name) for name in (BENCHMARKS, EIGEN, SOLID)]) else 1


if __name__ == "__main__":
    sys.exit(main())
# endregion 🔖️Run
