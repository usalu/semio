#!/usr/bin/env python3
"""🧱 THIRD-PARTY CONTINUUM oracle for `s.fem.fem3d`'s solid (`Tet4`) analysis answers.

**What this file adjudicates.** The one analysis path a frame solver structurally cannot see: a
`FemSolid`, which `crate::fem3d_engine::meshing::resolve_geometry` triangulates, extrudes and splits
into `Tet4` elements before solving it as a three-dimensional elastic continuum. Its sibling cases
adjudicate the frame answers (`../🧮️solves-fem3d-1-benchmarks`, PyNite) and the eigenvalue answers
(`../🎵️solves-fem3d-1-eigen`, SciPy).

**The third party.** `skfem` (scikit-fem, BSD-3-Clause) — an independent continuum finite-element
library. It builds its OWN hexahedral mesh of the same prism, assembles its own isotropic linear
elasticity operator from the Lamé parameters, and condenses and solves its own system. It never sees
this artifact's triangulation, its extrusion, its tetrahedra or its solver, which is exactly what
makes the comparison worth making: two different meshes of one continuum must still agree on the
continuum's answer.

**Why THIS fixture, and why the agreement can be exact.** `🧱️prismatic-solid-column.snapshot.json`
gives the solid a `meshSize` larger than its own footprint diagonal, so the footprint triangulates
into exactly its four corners — every base node is then a DOCUMENT node the fixture can name and
restrain. The restraint set it declares (`Tz` at all four corners, the origin held in plane, one edge
held per in-plane axis) is precisely the one the uniform-stress solution already satisfies, so it
removes the six rigid-body freedoms without fighting the Poisson contraction. Under a uniform top
pressure the continuum answer is therefore exactly `σ = −p` and `δ_top = −pH/E`, which a
constant-strain tetrahedron reproduces to machine precision and which scikit-fem reproduces to
`8e-16` on its own mesh; under self weight it is `−ρgH²/2E`, which a discretisation of either kind
approaches rather than hits, and which is held to five percent.
"""

# region 🔖️Imports
import json
import math

import numpy as np

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Constants
GRAVITY = 9.81
"""🌍️ Standard gravity, in m/s². `crate::fem3d_engine::fem3d_solve_all` fixes gravity at
`[0.0, 0.0, -9.81]`; the same number is written down here so the reference is not free to drift."""

# endregion 🔖️Constants


# region 🔖️Document
def by_id(records):
    """🗂️ An id-keyed view of one of the document's collections."""
    return {record["id"]: record for record in records}


# endregion 🔖️Document


# region 🔖️Solid
def solid_reference(document):
    """🧱️ scikit-fem's own 3D linear-elasticity answer for the prismatic solid fixture.

    The prism is meshed by scikit-fem itself, independently of this artifact's footprint
    triangulation, and loaded exactly as the document states: a uniform pressure over its top face,
    or its own weight as a body force. The reference reports the top-face axial displacement, which
    is what the closed forms `−pH/E` and `−ρgH²/2E` predict for a prism free to contract laterally.
    """
    import skfem
    from skfem.helpers import ddot, sym_grad, trace, eye
    from skfem.models.elasticity import lame_parameters

    solid = document["solids"][0]
    material = by_id(document["materials"])[solid["materialId"]]
    width = max(point[0] for point in solid["outline"]) - min(point[0] for point in solid["outline"])
    depth = max(point[1] for point in solid["outline"]) - min(point[1] for point in solid["outline"])
    height = solid["height"]
    mesh = skfem.MeshHex.init_tensor(np.linspace(0.0, width, 5), np.linspace(0.0, depth, 5), np.linspace(0.0, height, 9))
    basis = skfem.Basis(mesh, skfem.ElementVector(skfem.ElementHex1()))
    lam, mu = lame_parameters(material["e"], material["nu"])

    @skfem.BilinearForm
    def stiffness(u, v, _w):
        def sigma(w):
            return 2.0 * mu * sym_grad(w) + lam * eye(trace(sym_grad(w)), w.shape[0])

        return ddot(sigma(u), sym_grad(v))

    k = stiffness.assemble(basis)
    top_facets = skfem.FacetBasis(mesh, basis.elem, facets=mesh.facets_satisfying(lambda x: np.isclose(x[2], height)))

    # The restraint set the fixture itself declares, expressed on scikit-fem's own mesh: the whole
    # base held vertically, the origin held in plane, and one edge held per in-plane axis. That is
    # exactly the fixity the uniform-stress solution satisfies, so it removes the rigid-body modes
    # without fighting the Poisson contraction.
    constrained = np.unique(
        np.concatenate(
            [
                basis.get_dofs(lambda x: np.isclose(x[2], 0.0)).all("u^3"),
                basis.get_dofs(lambda x: np.isclose(x[0], 0.0) & np.isclose(x[1], 0.0) & np.isclose(x[2], 0.0)).all(),
                basis.get_dofs(lambda x: np.isclose(x[1], 0.0) & np.isclose(x[2], 0.0)).all("u^2"),
                basis.get_dofs(lambda x: np.isclose(x[0], 0.0) & np.isclose(x[2], 0.0)).all("u^1"),
            ]
        )
    )
    top_dofs = basis.get_dofs(lambda x: np.isclose(x[2], height)).all("u^3")

    answers = {}
    for case in document["loadCases"]:
        pressure = float(sum(load["pressure"] for load in case["loads"] if load["kind"] == "area"))
        weight = material["rho"] * GRAVITY if case["selfWeight"] else 0.0

        @skfem.LinearForm
        def body(v, _w, weight=weight):
            return -weight * v[2]

        @skfem.LinearForm
        def traction(v, _w, pressure=pressure):
            return -pressure * v[2]

        f = body.assemble(basis) + traction.assemble(top_facets)
        displacement = skfem.solve(*skfem.condense(k, f, D=constrained))
        answers[case["id"]] = float(np.mean(displacement[top_dofs]))
    return answers


# endregion 🔖️Solid


# region 🔖️Plan
def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "local://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def json_fixture(ctx, needle):
    """🧫️ The declared JSON fixture this scenario names."""
    return json.loads(ctx.fixture_bytes(uri_in(ctx, needle)).decode("utf-8"))


def outcome_of(projection, reference):
    """📤️ Hands over the compared projection, carrying the FULL answer as the raw artifact — which is
    what `🔨️run-fem3d-oracle.py` commits as `📊️expected.results.json` for the subject to assert
    against.

    The two implementations are held jointly to the CLOSED FORM, not to each other's digits: under a
    uniform top pressure both are exact and agree to six significant figures, while under self weight
    a hexahedral mesh and a tetrahedral one approach `−ρgH²/2E` from their own sides and only the
    five-percent verdict is shared. The displacement itself is asserted numerically by the subject
    against the committed reference this file produced.
    """
    return Outcome(projection, raw=json.dumps(reference, separators=(",", ":"), ensure_ascii=False, sort_keys=True).encode("utf-8"))


def significant(value, digits=6):
    """🔢️ One number at `digits` significant figures, as the normalized decimal TEXT this case's
    comparison profile compares. Text, not a float: the two implementations must agree to six
    figures, and a float projection would compare their last bits instead, which no two solvers
    share. Under a uniform pressure both
    implementations are exact, so six figures is a demand they genuinely meet."""
    if not math.isfinite(value):
        return "nonfinite"
    if value == 0.0:
        return "0.00000e+0"
    exponent = int(math.floor(math.log10(abs(value))))
    text = "%.*f" % (digits - 1, value / 10.0**exponent)
    if text.lstrip("-").startswith("10"):
        exponent += 1
        text = "%.*f" % (digits - 1, value / 10.0**exponent)
    return "%se%+d" % (text, exponent)


def agrees(label, produced, expected, tolerance):
    """📏️ Asserts a closed form in role, before the projection is handed over."""
    scale = max(abs(expected), 1e-30)
    if abs(produced - expected) / scale > tolerance:
        raise AssertionError("%s: the reference solved %.12g where the closed form is %.12g (relative %.3g)" % (label, produced, expected, abs(produced - expected) / scale))


# endregion 🔖️Plan


# region 🔖️Handlers
def solid_handler(needle):
    """🧱️ scikit-fem's 3D elasticity answer for the prism, held to `−pH/E` and `−ρgH²/2E`."""

    def handler(ctx):
        document = json_fixture(ctx, needle)
        answers = solid_reference(document)
        solid = document["solids"][0]
        material = by_id(document["materials"])[solid["materialId"]]
        height = solid["height"]
        pressure = sum(load["pressure"] for case in document["loadCases"] for load in case["loads"] if load["kind"] == "area")
        closed = {"pressure": -pressure * height / material["e"], "self": -material["rho"] * GRAVITY * height**2 / (2.0 * material["e"])}
        agrees("pressure shortening", answers["pressure"], closed["pressure"], 1e-9)
        agrees("self-weight shortening", answers["self"], closed["self"], 0.05)
        projection = {"model": needle, "closedFormZ": {name: significant(value) for name, value in closed.items()}, "pressureShorteningZ": significant(answers["pressure"]), "selfWeightWithinFivePercent": bool(abs(answers["self"] - closed["self"]) / abs(closed["self"]) <= 0.05)}
        return outcome_of(projection, {"model": needle, "topDisplacementZ": answers, "closedFormZ": closed})

    return handler


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only."""
    return Adapter("python").oracle("solid-prismatic-column", solid_handler("prismatic-solid-column"))


# endregion 🔖️Registration
