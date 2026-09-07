#!/usr/bin/env python3
"""🔮️ Registers the fem2d third-party SOLVER oracles in
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🔮️oracle/🔣️.json`.

Ticket input file (W5). It reads the registry, adds the two oracle entries and the comparison profile
the new case declares, attaches `anastruct-fem2d-solver` to the twenty-two mutation kinds whose
`fem2d-1-mutate` requirement carried no oracle, and narrows the `noOracleDecisions` entry to what a
2D frame solver genuinely still cannot reach. It is idempotent: running it twice changes nothing.
"""

import json
import os
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
REGISTRY = os.path.join(REPO, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🔮️oracle/🔣️.json")

PLATFORMS = ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"]

ANASTRUCT = {
    "id": "anastruct-fem2d-solver",
    "kind": "third-party-library",
    "ecosystem": "python",
    "package": "anastruct",
    "version": "1.7.0",
    "source": {"repository": "https://github.com/ritchie46/anaStruct", "license": "GPL-3.0-only"},
    "engine": {"family": "anastruct", "implementation": "anastruct 2D direct-stiffness frame solver", "version": "1.7.0"},
    "capabilities": ["fem2d-1-mutate", "fem2d-1-analysis"],
    "comparisonProfiles": ["semantic-fem2d-analysis-v1"],
    "license": "GPL-3.0-only",
    "testOnly": True,
    "productionReachable": False,
    "networkDuringExecution": False,
    "platforms": PLATFORMS,
    "homepage": "https://anastruct.readthedocs.io",
    "rationale": "🧮️ THE SOLVER ORACLE, AND WHY THE EARLIER SURVEY THAT DECLINED IT WAS ASKING THE WRONG QUESTION.\n\n`noOracleDecisions/fem2d-non-geometry-mutation-semantics` declined `code_aster`, `OpenSees`, `anastruct` and `PyNite` on the grounds that they \"compute displacements and forces FROM a model, while every one of these twenty-two kinds edits the model DOCUMENT itself\". That is right about what a solver can READ and wrong about what a solver can JUDGE. A solver cannot say whether `deleteSupport` removed the right record — `📈️mutate-fem2d-1-analysis` and its four subset siblings already adjudicate that against a second implementation of the algebra. What a solver can say, with complete independence, is what the structure DOES once that record is gone, and that is the thing all twenty-two kinds exist to change. A `replaceSection` that lands on the wrong slot, a `deleteSupport` that removes the wrong support, an `addLoad` that attaches to the wrong member: each produces a different STRUCTURE, and a different structure answers differently under load.\n\nHOW IT IS WIRED. `../../../📈️analysis/🧪️tests/🧮️solves-fem2d-1-benchmarks/` solves six committed real-world and benchmark models and, for every one of the twenty-five kinds, applies the mutation through PRODUCTION dispatch (`fem2d_mutated_analysis_report_json`, which runs the same `Mutation::diff(..).apply_to(..)` path the app runs) and solves what it left behind, while anastruct solves the independently committed post-mutation snapshot from scratch. The compared projection is every nodal displacement, every support reaction and every member end force, per load case AND per combination, scale-normalised so one comparison tolerance can serve metres and newtons at once.\n\nWHAT IT IS NOT. anastruct expresses 2D frames — `bar`/`beam` members, nodal loads, member-length uniform loads, the four standard support types. It expresses no meshed continuum, so a `FemRegion`'s plane-stress response is outside its reach and stays with the two mesh oracles (`three-fem2d-mesh-reader`, `manifold-fem2d-mesh-measure`); the projection therefore carries exactly the nodes and members a frame solver can see, and the three region kinds are judged here on the frame INVARIANCE they must preserve. It expresses no eigenproblem, so the modal and buckling benchmarks are `scipy-fem2d-eigen-reference`'s. And it reports nodal results only, so member end forces come from a from-the-textbook Euler-Bernoulli recovery in the same oracle file, which anastruct's own N/Q/M sampling is required to reproduce at both member ends before a single number is emitted.\n\nMEASURED, on the corpus this entry ships with: the reference refuses to emit until anastruct reproduces every displacement and every reaction of every load case to 1e-6 relative to that case's own peak response, and it does — across a 6 m cantilever, an 8 m simply supported beam, a two-span continuous concrete beam, the timber portal frame's substructure, a three-bay two-storey steel frame with four cases and three combinations, and four columns. The closed forms it is additionally held to are exact at the nodes: `PL³/(3EI)`, `5wL⁴/(384EI)` and the `0.375wL : 1.25wL : 0.375wL` two-span reaction split, all at 1e-9.\n\nA DEFECT FOUND BY WIRING IT UP, reported rather than worked around: `⚙️engine/📏️elements2d/🦀️.rs`'s `BeamEb2::recover` built its station moments as `m1 + v1·x`, which puts a NON-ZERO bending moment at the free end of a tip-loaded cantilever (`60000 → 120000 N·m` over a 6 m member instead of `60000 → 0`). The progression is `-m1 + v1·x`: `m1` is the local end-force six-vector's third component, which is the moment the node applies to the member, not the internal moment on the section. The subset's own two-span test never saw it because that model's first element starts at a PIN, where `m1` is zero and the two formulas agree. Fixed in this ticket; the free-end condition is now what the benchmark corpus asserts.\n\nLICENCE. GPL-3.0-only, `testOnly: true`, `productionReachable: false`: anastruct is imported by a Python test adapter only, never by shipped code, and nothing in this repository links or redistributes it.",
}

SCIPY = {
    "id": "scipy-fem2d-eigen-reference",
    "kind": "third-party-library",
    "ecosystem": "python",
    "package": "scipy",
    "version": "1.18.0",
    "source": {"repository": "https://github.com/scipy/scipy", "license": "BSD-3-Clause"},
    "engine": {"family": "lapack", "implementation": "scipy.linalg (LAPACK dsygvd/dggev)", "version": "1.18.0"},
    "capabilities": ["fem2d-1-analysis"],
    "comparisonProfiles": ["semantic-fem2d-analysis-v1"],
    "license": "BSD-3-Clause",
    "testOnly": True,
    "productionReachable": False,
    "networkDuringExecution": False,
    "platforms": PLATFORMS,
    "homepage": "https://scipy.org",
    "rationale": "🎵️ THE EIGENSOLVER, FOR THE TWO ANALYSES anastruct DOES NOT EXPRESS. This artifact's engine solves free vibration and linear buckling with its OWN subspace iteration (`⚙️engine/🔢️sparse/🦀️.rs`), and until this entry the only checks on it were in-repo closed forms at a 10 % tolerance. `scipy.linalg.eigh` (LAPACK's generalised symmetric-definite solver) and `scipy.linalg.eig` (the generalised non-symmetric pencil, for `K φ = λ(−K_g) φ`) are given an independently assembled Euler-Bernoulli stiffness, consistent mass and consistent geometric stiffness, and answer without any knowledge of this repository.\n\nWHAT IT IS HELD TO, and why the closed forms are the real judge: a numerical eigenvalue is only as good as the matrices it is given, so the reference is required to land within 2 % of `βₙ²/(2πL²)·√(EI/ρA)` for `βₙL` of 1.8751, 4.6941 and 7.8548, and within 2 % of `π²EI/(KL)²` for the four standard effective-length factors `K = 2.0` (fixed-free), `1.0` (pinned-pinned), `0.6992` (fixed-pinned, the exact `π/4.493409` rather than the code value 0.7) and `0.5` (fixed-guided). MEASURED on the committed corpus: the three cantilever frequencies come out at 10.0187 / 62.7911 / 175.9097 Hz against closed forms of 10.0184 / 62.7838 / 175.8235, and the four buckling factors at 1.202696 / 4.810932 / 9.842969 / 19.252952 against 1.20279 / 4.81116 / 9.84120 / 19.24463 — worst case 0.05 %. The cantilever is six metres, not three, on purpose: at three metres the first LONGITUDINAL mode (431 Hz) falls between bending modes two and three and `βₙL = 7.8548` would be compared against the wrong mode.\n\nA frame-solver package is the right judge for statics and the wrong one here; a LAPACK eigensolver is the right judge here and says nothing about a frame. Both are registered rather than one being stretched over the other.",
}

PROFILE = {
    "id": "semantic-fem2d-analysis-v1",
    "description": "🧮️ A fem2d linear-static answer as two implementations can compare it: the three projection axes (`nodes`, `reactions`, `members` — every node a `bar`/`beam` element references in document order, every restrained-and-active `(node, dof)` pair, and the members) followed by one entry per load case and per combination, in document order, each carrying nodal displacements `[ux, uy, rz]`, the reactions in axis order, and per member the local end-force six-vector `[N, V, M]` at both ends. Array order is significant everywhere, because both sides derive it from the same document and a reordering would be a real disagreement about which node is which.\n\nWHY EVERY NUMBER IS SCALE-NORMALISED. A comparison profile carries ONE absolute tolerance. A raw fem2d answer mixes displacements at 1e-5 m with reactions at 1e5 N, and no single absolute number can judge both: 1e-9 is meaningless slack on a metre and impossible tightness on a newton. So every projected value is divided by one of four decades — translation, rotation, force, moment — committed per model in the case's own `🧫️fixtures/📊️expected.results.json` and read by BOTH implementations from that same file, which makes every projected number order one and makes this profile's `tolerance` read as a fraction of the model's own peak response.\n\nWHY 1e-6 AND NOT TIGHTER. The two sides are different codes: this repository factors with its own LDLT over an RCM-reordered sparse matrix, the reference solves a dense system through LAPACK. A frame stiffness matrix mixes axial `EA/L` at ~1e9 with bending `12EI/L³` at ~1e5, so its condition number reaches 1e8-1e10 and the achievable agreement is bounded by `ε·κ` at roughly 1e-7 relative. 1e-6 sits one order above that floor and many orders below any real defect: a wrong element, a wrong load attachment, a dropped support or a sign error all move the answer by O(1). The same-implementation half is judged far harder — each side holds its own numbers to the committed SI reference at 1e-9 relative, in role, before the projection is ever built.",
    "arrays": "ordered",
    "tolerance": 1e-06,
}

DECISION = {
    "id": "fem2d-non-geometry-mutation-semantics",
    "capabilities": ["fem2d-1-mutate-uncarried"],
    "rationale": "🧱 WHAT IS LEFT AFTER THE SOLVER ORACLE, stated narrowly because the debt this entry used to carry is now paid. It read: twenty-two of this subset's twenty-five kinds edit fields no mesh carrier can see, a third-party FE solver was surveyed and declined because \"a solver computes displacements FROM a model while these kinds edit the model DOCUMENT\", and a qualifying third-party reference was still owed. `anastruct-fem2d-solver` now discharges those twenty-two `fem2d-1-mutate` requirements by judging the ANALYSIS the mutated document produces rather than the document itself — see that entry's rationale, and `../../../📈️analysis/🧪️tests/🧮️solves-fem2d-1-benchmarks/`, which reaches all twenty-five kinds by mutate-then-solve.\n\nTHREE THINGS ARE STILL OWED, and they are named exactly rather than folded into the discharge.\n\n1. `analysis.deformationScale`. It scales a drawn deformed shape and enters no equation. No solver on earth can witness it; only the carrier oracle (`serde-json-fem2d-carrier-reader`) sees that the field moved, and the cross-implementation reference sees that it moved to the right value. `analysis.modalCount`/`bucklingCount` ARE witnessed — they change how many modes an analysis returns, and the case's modal scenario compares that count and those frequencies.\n\n2. The plane-stress response of a `FemRegion`. anastruct expresses 2D frames and no meshed continuum, so the CST stresses inside a region are judged by nothing here. The region's GEOMETRY has two third-party oracles (`three-fem2d-mesh-reader`, `manifold-fem2d-mesh-measure`) and the three region kinds are additionally held, by the solver case, to the frame invariance they must preserve — but a continuum-capable reference (scikit-fem is the obvious candidate, and is already a declared test dependency) is still owed for the stresses themselves.\n\n3. Human-readable names — `material.name`, `section.name`, `region.name`, `loadCase.name`, `combination.name`. A solver never reads them. They are carrier-level facts and `serde-json-fem2d-carrier-reader` witnesses every one, so this is a note about scope rather than an undischarged requirement.\n\nRecorded as a debt, not a verdict, on the same terms as before: what is owed is item 2, and it is owed against `fem2d-1-analysis` for continuum elements, not against `fem2d-1-mutate`.",
}


def main():
    with open(REGISTRY, encoding="utf-8") as handle:
        registry = json.load(handle)

    for entry in (ANASTRUCT, SCIPY):
        registry["oracles"] = [existing for existing in registry["oracles"] if existing.get("id") != entry["id"]] + [entry]
    registry["comparisonProfiles"] = [existing for existing in registry["comparisonProfiles"] if existing.get("id") != PROFILE["id"]] + [PROFILE]
    registry["noOracleDecisions"] = [DECISION if existing.get("id") == DECISION["id"] else existing for existing in registry["noOracleDecisions"]]

    attached = 0
    for mutation in registry["mutationManifests"][0]["mutations"]:
        for requirement in mutation.get("oracleRequirements", []):
            if requirement.get("capability") == "fem2d-1-mutate" and "oracle" not in requirement:
                requirement["oracle"] = ANASTRUCT["id"]
                attached += 1

    with open(REGISTRY, "w", encoding="utf-8") as handle:
        json.dump(registry, handle, indent=2, ensure_ascii=False)
        handle.write("\n")

    print("[register] oracles: %s" % [entry["id"] for entry in registry["oracles"]])
    print("[register] comparison profiles: %s" % [entry["id"] for entry in registry["comparisonProfiles"]])
    print("[register] attached %s to %d mutation requirements" % (ANASTRUCT["id"], attached))
    undischarged = [mutation["id"] for mutation in registry["mutationManifests"][0]["mutations"] if any("oracle" not in requirement for requirement in mutation.get("oracleRequirements", []))]
    print("[register] mutation kinds still carrying an undischarged requirement: %s" % (undischarged or "none"))
    return 0


if __name__ == "__main__":
    sys.exit(main())
