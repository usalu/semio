#!/usr/bin/env python3
"""🔮 Emits `s.wfc.grid3d`'s oracle manifest — the machine-readable index of every committed mutation
vector, plus the recorded, reasoned refusal to lean on a third-party WFC oracle."""

import io
import json
import os

ROOT = "/Users/ueli/Documents/semio"
X = os.path.join(ROOT, "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any")

VECTORS = [
    ("change-seed", "🎲️change-seed", "🎲️reseeds-the-solve-from-7-to-99"),
    ("resize-grid", "📐️resize-grid", "📐️grows-the-grid-to-3x2x2"),
    ("change-cell-sizes", "📏️change-cell-sizes", "📏️stretches-the-x-axis-columns"),
    ("change-periodicity", "🔁️change-periodicity", "🔁️wraps-the-x-axis"),
    ("create-tile", "🧱️create-tile", "🧱️adds-the-roof-tile"),
    ("delete-tile", "🕳️delete-tile", "🕳️removes-the-air-tile-and-cascades"),
    ("change-tile-weight", "⚖️change-tile-weight", "⚖️raises-the-wall-tile-bias"),
    ("change-tile-media", "🖼️change-tile-media", "🖼️replaces-the-wall-tile-mesh"),
    ("create-rule", "🚦️create-rule", "🚦️allows-air-above-air"),
    ("delete-rule", "❌️delete-rule", "❌️removes-the-floor-wall-rule"),
    ("pin-cell", "📌️pin-cell", "📌️pins-the-far-cell-to-wall"),
    ("unpin-cell", "📍️unpin-cell", "📍️releases-the-origin-cell"),
    ("mask-cell", "🚫️mask-cell", "🚫️carves-out-the-far-edge-cell"),
    ("unmask-cell", "🔓️unmask-cell", "🔓️restores-the-masked-corner"),
]

RATIONALE = (
    "A second, independent implementation of this artifact's mutation semantics, written in Python "
    "against the NORMATIVE JSON Schema rather than ported from the Rust, living at "
    "`🧪️tests/🧩️mutate-wfc-grid3d-1/🐍️.py`. It re-derives every sparse diff and re-applies every "
    "committed one, so a Rust change that is wrong in both the builder and the fixture still fails "
    "here. Stdlib only: no `jsonschema`, no `pytest`, no network."
)

NO_ORACLE = (
    "There is no third-party oracle for THIS artifact's semantics. Surveyed, and declined with a "
    "reason each: mxgmn/WaveFunctionCollapse (C#, the reference implementation — solves an "
    "overlapping/simple-tiled 2D model, has no 3D six-neighbour tiled model, no per-axis non-uniform "
    "cell sizes and no notion of a persisted problem document, so it can oracle neither the mutation "
    "algebra nor the grid geometry); marian42/wavefunctioncollapse (Unity, 3D and closest in domain "
    "— but it is a Unity component whose adjacency is derived from imported mesh sockets, not from an "
    "authored allow-list, and it is unlicensed for reuse); fast-wfc (C++, 2D only); wfc rust crates "
    "(`wfc`/`wfc_image`, 2D grid only). Every one of them oracles a SOLVER, and the solver is already "
    "differentially tested inside `semio-s-plugin-wfc-engine` against its own brute-force enumerator. "
    "What this artifact adds on top — the document model, the fourteen point-invertible mutations, the "
    "sparse diff algebra and the non-uniform cell geometry — has no external counterpart at all, so "
    "the honest substitutes are a verified native second implementation plus the metamorphic laws the "
    "fixture suite asserts (apply∘inverse = identity, diff∘apply = after, canonical JSON is a fixed "
    "point)."
)


def manifest():
    return {
        "$schema": "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json",
        "schemaVersion": 2,
        "oracles": [
            {
                "id": "s.wfc.grid3d.python-reference",
                "kind": "verified-native-second-implementation",
                "ecosystem": "python",
                "package": "",
                "capabilities": ["mutation-diff", "diff-apply", "inverse", "canonical-json"],
                "comparisonProfiles": ["exact-json"],
                "license": "in-repo",
                "testOnly": True,
                "rationale": RATIONALE,
                "nativeSecondImplementation": {
                    "noThirdPartySurvey": {
                        "ecosystemsSearched": ["crates.io", "npm", "PyPI", "GitHub", "NuGet"],
                        "candidatesConsidered": ["mxgmn/WaveFunctionCollapse", "marian42/wavefunctioncollapse", "fast-wfc", "wfc (crates.io)", "wfc_image (crates.io)"],
                    },
                    "fixtureCoverage": {"vectors": len(VECTORS)},
                },
            }
        ],
        "noOracleDecisions": [
            {
                "id": "s.wfc.grid3d.no-third-party-oracle",
                "capability": "mutation-algebra",
                "rationale": NO_ORACLE,
                "substitutes": ["specification-vectors", "metamorphic-laws"],
            }
        ],
        "mutationCatalogs": [
            {
                "id": "wfc-grid3d-1-any",
                "capability": "mutation-algebra",
                "standardDirectoryName": "🔖️1",
                "subsetDirectoryName": "✳️any",
                "vectors": [
                    {
                        "mutationId": kind,
                        "sourceMutationDirectoryName": directory,
                        "mutationDirectoryName": directory,
                        "scenarios": [{"id": case.split("️", 1)[-1], "directoryName": case}],
                    }
                    for kind, directory, case in VECTORS
                ],
            }
        ],
    }


path = os.path.join(X, "🔮️oracles", "🔣️.json")
os.makedirs(os.path.dirname(path), exist_ok=True)
io.open(path, "w", encoding="utf-8").write(json.dumps(manifest(), ensure_ascii=False, indent=2) + "\n")
print("wrote", path)
