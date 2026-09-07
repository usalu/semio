#!/usr/bin/env python3
"""🔮 Registers the three fem3d ANALYSIS oracles and retires the debt they discharge.

`🌐️any/🔮️oracle/🔣️.json` is the artifact's oracle registry. This edits it in place — read, modify,
write, with the same two-space JSON formatting the file already uses, verified byte-identical on a
round trip before anything is changed, so a concurrent editor's work in the same file survives.

What it writes:
  * three `oracles` entries — `pynite-fem3d-solver`, `scipy-fem3d-eigen`, `skfem-fem3d-solid`;
  * three `comparisonProfiles` the cases that use them declare;
  * the `oracleRequirements` of all twenty-five mutations, each now naming the oracle that
    adjudicates what the edited document MEANS;
  * three `fixtureManifests` recording the provenance of the committed `📊️expected.results.json`
    each case's reference produced;
  * the removal of `fem3d-non-geometry-mutation-semantics` from `noOracleDecisions`, which those
    twenty-two kinds no longer owe.

Run: `uv run python .🧬semio/…/FEM-PLUGIN-END-TO-END/🔨️register-fem3d-solver-oracles.py`
"""

# region 🔖️Imports
import hashlib
import json
import os

# endregion 🔖️Imports


# region 🔖️Paths
REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
SUBSETS = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets")
REGISTRY = os.path.join(SUBSETS, "🌐️any", "🔮️oracle", "🔣️.json")
TESTS = os.path.join(SUBSETS, "📈️analysis", "🧪️tests")

SOLID_KINDS = ("create-solid", "delete-solid", "replace-solid")
"""🧱️ The three kinds whose subject is geometry — already discharged by `three` and `manifold-3d`,
and now additionally by a continuum solver on the analysis half."""

PLATFORMS = ["darwin-arm64", "darwin-x64", "linux-x64", "linux-arm64", "win32-x64"]
# endregion 🔖️Paths


# region 🔖️Entries
PYNITE = {
    "id": "pynite-fem3d-solver",
    "kind": "third-party-library",
    "ecosystem": "python",
    "package": "PyNiteFEA",
    "version": "3.0.0",
    "source": {"repository": "https://github.com/JWock82/Pynite", "license": "MIT"},
    "engine": {"family": "pynite", "implementation": "Pynite.FEModel3D direct-stiffness frame and truss solver", "version": "3.0.0"},
    "capabilities": ["fem3d-1-mutate"],
    "comparisonProfiles": ["semantic-fem3d-analysis-v1"],
    "license": "MIT",
    "testOnly": True,
    "productionReachable": False,
    "networkDuringExecution": False,
    "platforms": PLATFORMS,
    "homepage": "https://pynite.readthedocs.io",
    "rationale": "🧮 An independent 3D frame and truss finite-element solver, pure Python, MIT. It assembles its own stiffness, applies its own boundary conditions and solves its own system; nothing of this repository's kernel reaches it, and it shares no engine family with `crate::sparse`'s hand-rolled LDLT.\n\nWHAT IT ADJUDICATES, AND WHY THAT DISCHARGES A DEBT THIS FILE CARRIED FOR A LONG TIME. `fem3d-non-geometry-mutation-semantics` recorded, correctly, that a finite-element solver cannot adjudicate document ALGEBRA — a solver reads a model, it does not edit one — and concluded that twenty-two of this vocabulary's twenty-five kinds therefore had no qualifying reference available at all. That survey asked only half the question. A solver adjudicates what the edited document MEANS, and every one of those twenty-two kinds changes what the model means: a section that carries less, a support that has stopped holding, a material that has got softer, a load case that no longer exists. `📈️analysis/🧪️tests/🧮️solves-fem3d-1-benchmarks` therefore mutates through PRODUCTION dispatch, holds the applied model to the committed after-model, and then SOLVES it against this oracle — displacements, reactions, member forces, every load case and every combination — and additionally requires both implementations to report the same MECHANISM: which combinations moved, which are untouched, which appeared and which are gone.\n\nHOW THE TWO COORDINATE SYSTEMS ARE RECONCILED, which is the load-bearing part of trusting it. This artifact is Z-up and PyNite is Y-up, and the two build a member's local axes from different reference directions (this artifact from global +Z, PyNite from global +Y). The reference maps between the frames with the proper rotation `(X, Y, Z)ₚ = (x, z, −y)` — determinant +1, so every cross product either solver takes keeps its handedness — computes this artifact's OWN local triad from its documented rule, and solves for the PyNite roll angle that makes the two coincide. It then checks the result against PyNite's own transformation matrix to `1e-12` per member, so a scenario stops before it compares anything if `Iy` and `Iz` have stopped naming the same axes.\n\nTWO DEFECT-CLASS FINDINGS THE INTEGRATION PRODUCED, reported rather than worked around. (1) PyNite's `add_member` builds a PHYSICAL member that silently subdivides itself at any model node lying on its axis; on the shared `🧊️steel-frame` fixture that turns the spare node `n3` into a mid-height support carrying 8.3 kN this artifact's own model never puts there. The reference narrows the subdivision search to the member's own two ends so the two element sets are the same set. (2) A `bar` is a two-force member with three translational degrees of freedom and no bending stiffness; PyNite's `Spring3D` is exactly that, so a bar maps to `add_spring(ks = EA/L)` and every rotational freedom at a node only bars touch is restrained — mirroring `crate::model::build_dof_map`'s own union-of-incident-elements rule rather than assuming PyNite's default.\n\nWHY IT CAN BE TRUSTED AS THE REFERENCE. Alongside it, on the cases where one exists, sits a CLOSED FORM: `PL³/3EI`, `PL²/2EI`, `TL/GJ`, `5wL⁴/384EI`, `wL/2`, `wL`. The reference asserts every one of them in role before it projects anything, and asserts global equilibrium — reactions cancel the applied resultant to `1e-9` relative — on every case and every combination of every model, including the ones with no closed form. A library can be wrong; a library that agrees with statics on a twenty-seven-node, forty-two-member space frame under three load cases and two combinations is not.",
}

SCIPY = {
    "id": "scipy-fem3d-eigen",
    "kind": "third-party-library",
    "ecosystem": "python",
    "package": "scipy",
    "version": "1.18.0",
    "source": {"repository": "https://github.com/scipy/scipy", "license": "BSD-3-Clause"},
    "engine": {"family": "lapack", "implementation": "scipy.linalg.eigh dense symmetric generalised eigensolver", "version": "1.18.0"},
    "capabilities": ["fem3d-1-mutate"],
    "comparisonProfiles": ["semantic-fem3d-eigen-v1"],
    "license": "BSD-3-Clause",
    "testOnly": True,
    "productionReachable": False,
    "networkDuringExecution": False,
    "platforms": PLATFORMS,
    "homepage": "https://scipy.org",
    "rationale": "🎵 A dense symmetric generalised eigensolver over LAPACK, adjudicating the two eigenvalue analyses `crate::fem3d_engine::modal_buckling` computes — free-vibration frequencies and linear-buckling load factors — which this repository answers with its own hand-rolled subspace iteration (`crate::sparse::subspace_iteration`) and which had, until `📈️analysis/🧪️tests/🎵️solves-fem3d-1-eigen`, no external reference of any kind. The kernel's own benchmarks compared them with a closed form at a TEN-percent tolerance and with nothing else at all.\n\nWHAT THE EIGENSOLVER IS HANDED. `K`, `M` and `Kg` assembled by the reference itself from the textbook Euler-Bernoulli frame matrices, in ONE stated sign convention — in the local `[u, v, w, θx, θy, θz]` ordering both this artifact and every textbook use, the y-bending plane measures its rotation as `θy = −∂w/∂x`, which puts a sign flip on the off-diagonal `L` terms of every matrix in that plane (formally `S·M·S` with `S = diag(1, −1, 1, −1)`), and the stiffness, the consistent mass AND the consistent geometric stiffness must all carry it or the pair's eigenvalues are not the structure's.\n\nWHAT IS COMPARED, AND WHY NOT THE EIGENVALUES THEMSELVES. A subspace iteration and a LAPACK factorisation of the same pencil agree to their own convergence tolerance, not to a digit count, so a projection demanding digits would be measuring convergence settings rather than correctness. What the two implementations are held to jointly is the CLOSED FORM — `βₙ²/(2πL²)·√(EI/ρA)` for a cantilever's frequencies, `π²EI/(KL)²` for a column's critical load at `K = 0.5, 0.7, 1.0, 2.0` — and whether each of them independently lands within two percent of it over an eight-element discretisation. This oracle's own agreement is what makes that a fair demand rather than a wish: it lands inside 0.3 % on every one of the five scenarios. The eigenvalues themselves are asserted numerically by the subject against the committed reference this oracle produced.",
}

SKFEM = {
    "id": "skfem-fem3d-solid",
    "kind": "third-party-library",
    "ecosystem": "python",
    "package": "scikit-fem",
    "version": "12.0.2",
    "source": {"repository": "https://github.com/kinnala/scikit-fem", "license": "BSD-3-Clause"},
    "engine": {"family": "scikit-fem", "implementation": "scikit-fem 3D isotropic linear elasticity over a hexahedral mesh", "version": "12.0.2"},
    "capabilities": ["fem3d-1-mutate"],
    "comparisonProfiles": ["semantic-fem3d-solid-v1"],
    "license": "BSD-3-Clause",
    "testOnly": True,
    "productionReachable": False,
    "networkDuringExecution": False,
    "platforms": PLATFORMS,
    "homepage": "https://scikit-fem.readthedocs.io",
    "rationale": "🧱 An independent continuum finite-element library, adjudicating the one analysis path a frame solver structurally cannot see: a `FemSolid`, which `crate::fem3d_engine::meshing::resolve_geometry` triangulates, extrudes and splits into `Tet4` elements before solving it as a three-dimensional elastic continuum. It builds its OWN hexahedral mesh of the same prism, assembles its own isotropic elasticity operator from the Lamé parameters, and condenses and solves its own system; it never sees this artifact's triangulation, its extrusion, its tetrahedra or its solver — which is what makes the comparison worth making, since two different meshes of one continuum must still agree on the continuum's answer.\n\nWHY THE FIXTURE IS SHAPED THE WAY IT IS. `🧱️prismatic-solid-column.snapshot.json` gives its solid a `meshSize` larger than its own footprint diagonal, so the footprint triangulates into exactly its four corners — every base node is then a DOCUMENT node the fixture can name and restrain, which no ordinary solid fixture can do, because `resolve_geometry` synthesizes the rest. The restraint set it declares (`Tz` at all four corners, `Tx`+`Ty` at the origin, `Ty` at the +x corner, `Tx` at the +y corner) is precisely the one the uniform-stress solution already satisfies, so it removes the six rigid-body freedoms without fighting the Poisson contraction. Under a uniform 0.5 MPa top pressure the continuum answer is therefore exactly `σ = −p` and `δ_top = −pH/E`, which a constant-strain tetrahedron reproduces to machine precision and which this oracle reproduces to `8e-16` on its own mesh. Under self weight the answer is `−ρgH²/2E`, which a hexahedral mesh and a tetrahedral one approach from their own sides rather than hit — this oracle lands 0.26 % below it — so that case is a five-percent verdict, and nothing tighter would be honest about either discretisation.\n\nIT DOES NOT REPLACE THE GEOMETRY ORACLES. `three-fem3d-mesh-reader` and `manifold-fem3d-mesh-measure` adjudicate what the solid's mesh IS; this one adjudicates what it DOES under load. The three `create-`/`delete-`/`replace-solid` kinds carry all three requirements for that reason.",
}

PROFILES = [
    {
        "id": "semantic-fem3d-analysis-v1",
        "description": "One solved model's answer as each case's four scale-carrying scalars — displacement norm, largest translation, largest rotation, the reaction force resultant and the largest bar force — each written as NORMALIZED DECIMAL TEXT at six significant figures, plus, for a mutate-then-solve scenario, the categorical mechanism the edit produced per combination (`changed`, `unchanged`, `appeared`, `disappeared`, `reshaped`). Text and not a float on purpose: two solvers' last bits never agree, and a float projection would compare their round-off rather than their answers. Six figures is a millionfold margin over the `1e-12`-scale disagreement two direct double-precision factorisations of one linear system actually show, and a resultant that cancels by symmetry — `−2.5e-11` in one solver, `+8.1e-12` in the other — is snapped to zero when it is under a billionth of its own group's largest component, because six significant figures OF ROUND-OFF is noise rather than agreement. The FULL displacement, reaction and member-force field is not compared here: it is asserted, at `1e-6` relative, by the subject against the case's committed `📊️expected.results.json`, which this profile's oracle produced.",
        "arrays": "ordered",
        "text": True,
    },
    {
        "id": "semantic-fem3d-eigen-v1",
        "description": "A modal or buckling answer as the CLOSED FORM both implementations are held to — normalized decimal text at six significant figures — together with each implementation's own verdict on whether it landed within two percent of it, and the mode or factor count the document asked for. The eigenvalues themselves are deliberately not compared: a subspace iteration and a LAPACK factorisation of the same pencil agree to their own convergence tolerance, not to a digit count, so comparing digits would measure convergence settings rather than correctness. They are asserted, at `1e-3` relative, by the subject against the case's committed `📊️expected.results.json`.",
        "arrays": "ordered",
        "text": True,
    },
    {
        "id": "semantic-fem3d-solid-v1",
        "description": "A meshed-solid answer as the closed forms `−pH/E` and `−ρgH²/2E` in normalized decimal text at six significant figures, the pressure case's top-face shortening at the same six figures — where a constant-strain tetrahedron and a trilinear hexahedron are both EXACT, so six figures is a demand both genuinely meet — and the shared five-percent verdict on the self-weight case, which is all two different meshes of one continuum honestly share. The displacements themselves are asserted by the subject against the case's committed `📊️expected.results.json`.",
        "arrays": "ordered",
        "text": True,
    },
]
# endregion 🔖️Entries


# region 🔖️Apply
def digest(path):
    """#⃣ The full content address of one committed file."""
    with open(path, "rb") as handle:
        return "sha256:" + hashlib.sha256(handle.read()).hexdigest()


def fixture_manifest(case, oracle, package_version, engine_family, comparison, notes):
    """🧫️ One case's committed third-party reference, recorded with its provenance."""
    path = os.path.join(TESTS, case, "🧫️fixtures", "📊️expected.results.json")
    relative = os.path.relpath(path, os.path.dirname(REGISTRY)).replace(os.sep, "/")
    return {
        "schema": "semio.repository-test.fixture/v2",
        "id": case.split("solves-fem3d-1-")[-1] + "-analysis-reference",
        "class": "third-party-generated",
        "target": {"artifact": "s.fem.3d", "standard": "1", "subset": "analysis"},
        "outcome": "applied",
        "units": {"length": "metre", "angle": "radian", "handedness": "right", "up": "z"},
        "files": [{"role": "expected-analysis", "path": relative, "mediaType": "application/json", "sha256": digest(path), "bytes": os.path.getsize(path)}],
        "generator": {
            "oracle": oracle,
            "packageVersion": package_version,
            "engineFamily": engine_family,
            "engineVersion": package_version,
            "command": "uv run python .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/FEM-PLUGIN-END-TO-END/🔨️run-fem3d-oracle.py",
            "platform": "darwin-arm64",
        },
        "provenance": {"source": "generated", "license": "MIT (PyNiteFEA) / BSD-3-Clause (SciPy, scikit-fem)", "attribution": "Solved with PyNiteFEA, SciPy and scikit-fem; every value additionally held to a closed form or to global equilibrium in role", "security": "scanned-clean", "privacy": "no-personal-data"},
        "comparisonProfile": comparison,
        "reproducible": True,
        "family": "analysis",
        "notes": notes,
    }


def main():
    """🚀️ Applies every registry change, in place."""
    raw = open(REGISTRY, encoding="utf-8").read()
    registry = json.loads(raw)
    if json.dumps(registry, indent=2, ensure_ascii=False) + "\n" != raw:
        raise SystemExit("the registry does not round-trip byte-identically; edit it by hand rather than through this script")

    known = {entry["id"] for entry in registry["oracles"]}
    for entry in (PYNITE, SCIPY, SKFEM):
        if entry["id"] not in known:
            registry["oracles"].append(entry)

    profiles = {entry["id"] for entry in registry.setdefault("comparisonProfiles", [])}
    for profile in PROFILES:
        if profile["id"] not in profiles:
            registry["comparisonProfiles"].append(profile)

    for manifest in registry["mutationManifests"]:
        for mutation in manifest["mutations"]:
            requirements = mutation["oracleRequirements"]
            if mutation["id"] in SOLID_KINDS:
                if not any(requirement.get("oracle") == SKFEM["id"] for requirement in requirements):
                    requirements.append({"capability": "fem3d-1-mutate", "qualifyingKind": "third-party-library", "oracle": SKFEM["id"]})
                continue
            for requirement in requirements:
                if requirement["capability"] == "fem3d-1-mutate" and "oracle" not in requirement:
                    requirement["oracle"] = PYNITE["id"]
            if mutation["id"] == "update-analysis-settings" and not any(requirement.get("oracle") == SCIPY["id"] for requirement in requirements):
                requirements.append({"capability": "fem3d-1-mutate", "qualifyingKind": "third-party-library", "oracle": SCIPY["id"]})

    registry.setdefault("fixtureManifests", [])
    for case, oracle, version, family, comparison, notes in (
        ("🧮️solves-fem3d-1-benchmarks", PYNITE["id"], "3.0.0", "pynite", "semantic-fem3d-analysis-v1", "displacements, reactions and bar forces for three real structures, four closed-form cases and one mutate-then-solve pair per non-geometry kind — twenty-nine scenarios, each additionally held to global equilibrium in role."),
        ("🎵️solves-fem3d-1-eigen", SCIPY["id"], "1.18.0", "lapack", "semantic-fem3d-eigen-v1", "cantilever natural frequencies and Euler load factors at K = 0.5, 0.7, 1.0 and 2.0, each within 0.3 % of its closed form."),
        ("🧱️solves-fem3d-1-solid", SKFEM["id"], "12.0.2", "scikit-fem", "semantic-fem3d-solid-v1", "top-face shortening of a prismatic column under uniform pressure (exact) and under self weight (0.26 % below the closed form)."),
    ):
        manifest = fixture_manifest(case, oracle, version, family, comparison, notes)
        # 🔁️Replaced, never merely appended: the entry's whole purpose is to pin the reference's
        # content address, so re-running this script after re-running the oracle must refresh it.
        registry["fixtureManifests"] = [entry for entry in registry["fixtureManifests"] if entry["id"] != manifest["id"]]
        registry["fixtureManifests"].append(manifest)

    registry["noOracleDecisions"] = [decision for decision in registry["noOracleDecisions"] if decision["id"] != "fem3d-non-geometry-mutation-semantics"]

    with open(REGISTRY, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(registry, indent=2, ensure_ascii=False) + "\n")
    print("oracles: %d, comparisonProfiles: %d, fixtureManifests: %d, noOracleDecisions: %d" % (len(registry["oracles"]), len(registry["comparisonProfiles"]), len(registry["fixtureManifests"]), len(registry["noOracleDecisions"])))
    discharged = sum(1 for mutation in registry["mutationManifests"][0]["mutations"] if any(requirement["capability"] == "fem3d-1-mutate" and "oracle" in requirement for requirement in mutation["oracleRequirements"]))
    print("mutations with a third-party fem3d-1-mutate oracle: %d/%d" % (discharged, len(registry["mutationManifests"][0]["mutations"])))


if __name__ == "__main__":
    main()
# endregion 🔖️Apply
