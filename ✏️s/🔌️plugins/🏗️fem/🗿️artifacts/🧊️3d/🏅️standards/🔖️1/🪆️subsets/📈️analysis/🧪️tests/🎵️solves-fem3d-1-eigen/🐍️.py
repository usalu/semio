#!/usr/bin/env python3
"""🎵 THIRD-PARTY EIGENSOLVER oracle for `s.fem.fem3d`'s modal and linear-buckling answers.

**What this file adjudicates.** Not the model algebra — its sibling cases do that — and not a static
answer, which `../🧮️solves-fem3d-1-benchmarks` puts to PyNite. This one adjudicates the two
eigenvalue analyses `crate::fem3d_engine::modal_buckling` computes: the natural frequencies of a
free-vibrating frame, and the load factors at which an axially loaded one buckles.

**The third party.** `scipy.linalg.eigh` (SciPy, BSD-3-Clause) — an independent dense symmetric
generalised eigensolver over LAPACK. This artifact's kernel solves both problems with its OWN
hand-rolled subspace iteration (`crate::sparse::subspace_iteration`); nothing of it reaches here.
The matrices the eigensolver is handed are assembled in this file from the textbook Euler-Bernoulli
frame stiffness, consistent mass and consistent geometric stiffness — stated once, in one sign
convention, and then never touched again.

**And a closed form beside it, on every scenario.** A cantilever's natural frequencies are
`βₙ²/(2πL²)·√(EI/ρA)` with `β = 1.8751, 4.6941, 7.8548`; a column's critical load is `π²EI/(KL)²`
with `K = 0.5, 0.7, 1.0, 2.0` for the four textbook end conditions. Both are asserted here, in role,
to two percent over an eight-element discretisation before any answer is projected — so neither
SciPy nor this file's own assembly can quietly become the expectation on its own authority.

**One sign convention, stated once.** In the local `[u, v, w, θx, θy, θz]` ordering both this
artifact and every textbook use, the y-bending plane measures its rotation as `θy = −∂w/∂x`. That
choice shows up as a sign flip on the off-diagonal `L` terms of every matrix in that plane —
formally `S·M·S` with `S = diag(1, −1, 1, −1)` — and it must be carried by the stiffness, the mass
AND the geometric stiffness alike, or the pair's eigenvalues are not the structure's. Each of the
three matrices below is written in that one convention.
"""

# region 🔖️Imports
import json
import math

import numpy as np
from scipy.linalg import eigh

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Constants
DOFS = ("Tx", "Ty", "Tz", "Rx", "Ry", "Rz")
"""🔢️ The six degrees of freedom in this artifact's own index order."""

LINE_DOFS = {"bar": ("Tx", "Ty", "Tz"), "frame": DOFS}
"""🔢️ Which degrees of freedom each element kind activates at the nodes it touches."""

TOLERANCE = 0.02
"""📏️ The agreement demanded of the closed forms: two percent over eight elements."""

# endregion 🔖️Constants


# region 🔖️Frames
def unit(vector):
    """📏️ A unit vector."""
    return np.asarray(vector, dtype=float) / float(np.linalg.norm(vector))


def artifact_triad(start, end, roll):
    """🧭️ The member triad this artifact documents: local x from start to end, local y from
    `normalize(reference × x)` with reference `+Z` unless the member is within about eight degrees of
    vertical (`|x_z| > 0.99`) in which case `+X`, local z completing it, both then rolled about x."""
    cx = unit(np.asarray(end, dtype=float) - np.asarray(start, dtype=float))
    reference = np.array([1.0, 0.0, 0.0]) if abs(cx[2]) > 0.99 else np.array([0.0, 0.0, 1.0])
    y_unrot = unit(np.cross(reference, cx))
    z_unrot = np.cross(cx, y_unrot)
    return cx, y_unrot * math.cos(roll) + z_unrot * math.sin(roll), z_unrot * math.cos(roll) - y_unrot * math.sin(roll)


# endregion 🔖️Frames

# region 🔖️Document
def by_id(records):
    """🗂️ An id-keyed view of one of the document's collections."""
    return {record["id"]: record for record in records}


# endregion 🔖️Document

# region 🔖️BeamMatrices
def beam_local_stiffness(length, e, g, area, iy, iz, j):
    """🧮️ The textbook 12×12 Euler-Bernoulli frame stiffness in local axes, `[u,v,w,θx,θy,θz]` per
    node — the z-bending plane on `iz`, the y-bending plane on `iy` with its own sign convention
    (`θy = −∂w/∂x`), axial on `A` and torsion on `J`."""
    k = np.zeros((12, 12))
    axial, torsion = e * area / length, g * j / length
    k[np.ix_([0, 6], [0, 6])] = axial * np.array([[1.0, -1.0], [-1.0, 1.0]])
    k[np.ix_([3, 9], [3, 9])] = torsion * np.array([[1.0, -1.0], [-1.0, 1.0]])
    bz, l2 = e * iz / length, length * length
    k[np.ix_([1, 5, 7, 11], [1, 5, 7, 11])] = np.array(
        [
            [12.0 * bz / l2, 6.0 * bz / length, -12.0 * bz / l2, 6.0 * bz / length],
            [6.0 * bz / length, 4.0 * bz, -6.0 * bz / length, 2.0 * bz],
            [-12.0 * bz / l2, -6.0 * bz / length, 12.0 * bz / l2, -6.0 * bz / length],
            [6.0 * bz / length, 2.0 * bz, -6.0 * bz / length, 4.0 * bz],
        ]
    )
    by = e * iy / length
    k[np.ix_([2, 4, 8, 10], [2, 4, 8, 10])] = np.array(
        [
            [12.0 * by / l2, -6.0 * by / length, -12.0 * by / l2, -6.0 * by / length],
            [-6.0 * by / length, 4.0 * by, 6.0 * by / length, 2.0 * by],
            [-12.0 * by / l2, 6.0 * by / length, 12.0 * by / l2, 6.0 * by / length],
            [-6.0 * by / length, 2.0 * by, 6.0 * by / length, 4.0 * by],
        ]
    )
    return k


def beam_local_mass(length, rho, area, j):
    """🏋️ The textbook 12×12 consistent mass in the SAME local convention as the stiffness above.

    The y-bending plane's rotational couplings carry the sign flip that `θy = −∂w/∂x` forces —
    formally `S·M·S` with `S = diag(1, −1, 1, −1)` — because the mass and the stiffness must be
    written in one convention or the eigenvalues of the pair are not the structure's.
    """
    m = np.zeros((12, 12))
    axial, torsion = rho * area * length / 6.0, rho * j * length / 6.0
    m[np.ix_([0, 6], [0, 6])] = axial * np.array([[2.0, 1.0], [1.0, 2.0]])
    m[np.ix_([3, 9], [3, 9])] = torsion * np.array([[2.0, 1.0], [1.0, 2.0]])
    factor, l2 = rho * area * length / 420.0, length * length
    block = np.array(
        [
            [156.0, 22.0 * length, 54.0, -13.0 * length],
            [22.0 * length, 4.0 * l2, 13.0 * length, -3.0 * l2],
            [54.0, 13.0 * length, 156.0, -22.0 * length],
            [-13.0 * length, -3.0 * l2, -22.0 * length, 4.0 * l2],
        ]
    )
    flip = np.diag([1.0, -1.0, 1.0, -1.0])
    m[np.ix_([1, 5, 7, 11], [1, 5, 7, 11])] = factor * block
    m[np.ix_([2, 4, 8, 10], [2, 4, 8, 10])] = factor * (flip @ block @ flip)
    return m


def beam_local_geometric(length, axial):
    """🌀️ The textbook 12×12 consistent geometric stiffness under axial force `N` (tension positive),
    in the same convention — the matrix whose lowest eigenvalue against `K` is the Euler load."""
    kg = np.zeros((12, 12))
    block = axial * np.array(
        [
            [6.0 / (5.0 * length), 1.0 / 10.0, -6.0 / (5.0 * length), 1.0 / 10.0],
            [1.0 / 10.0, 2.0 * length / 15.0, -1.0 / 10.0, -length / 30.0],
            [-6.0 / (5.0 * length), -1.0 / 10.0, 6.0 / (5.0 * length), -1.0 / 10.0],
            [1.0 / 10.0, -length / 30.0, -1.0 / 10.0, 2.0 * length / 15.0],
        ]
    )
    flip = np.diag([1.0, -1.0, 1.0, -1.0])
    kg[np.ix_([1, 5, 7, 11], [1, 5, 7, 11])] = block
    kg[np.ix_([2, 4, 8, 10], [2, 4, 8, 10])] = flip @ block @ flip
    return kg


def transform(start, end, roll):
    """🔁️ The 12×12 block-diagonal global→local rotation for one member."""
    axes = np.vstack(artifact_triad(start, end, roll))
    t = np.zeros((12, 12))
    for offset in (0, 3, 6, 9):
        t[offset : offset + 3, offset : offset + 3] = axes
    return t


def assemble(document, axial_forces=None):
    """🏗️ Assembles the document's frame model into dense `(K, M, Kg, index)` in global axes.

    Only frames take part: every eigen benchmark in this case is a frame model, and a pin-ended bar
    has no bending stiffness to buckle or bend.
    """
    positions = {node["id"]: (node["x"], node["y"], node["z"]) for node in document["nodes"]}
    order = [(node["id"], dof) for node in document["nodes"] for dof in DOFS]
    index = {key: i for i, key in enumerate(order)}
    size = len(order)
    k, m, kg = np.zeros((size, size)), np.zeros((size, size)), np.zeros((size, size))
    materials, sections = by_id(document["materials"]), by_id(document["sections"])
    for element in document["elements"]:
        if element["kind"] != "frame":
            continue
        start, end = positions[element["start"]], positions[element["end"]]
        length = float(np.linalg.norm(np.asarray(end) - np.asarray(start)))
        material, section = materials[element["materialId"]], sections[element["sectionId"]]
        t = transform(start, end, element.get("roll", 0.0))
        rows = [index[(element["start"], dof)] for dof in DOFS] + [index[(element["end"], dof)] for dof in DOFS]
        k[np.ix_(rows, rows)] += t.T @ beam_local_stiffness(length, material["e"], material["g"], section["area"], section["iy"], section["iz"], section["j"]) @ t
        m[np.ix_(rows, rows)] += t.T @ beam_local_mass(length, material["rho"], section["area"], section["j"]) @ t
        if axial_forces is not None:
            kg[np.ix_(rows, rows)] += t.T @ beam_local_geometric(length, axial_forces[element["id"]]) @ t
    return k, m, kg, index


def free_indices(document, index):
    """🔓️ The unrestrained rows of the assembled system."""
    restrained = set()
    for support in document["supports"]:
        for dof in support["fixed"]:
            restrained.add(index[(support["nodeId"], dof)])
    touched = set()
    for element in document["elements"]:
        for node_id in (element["start"], element["end"]):
            for dof in LINE_DOFS[element["kind"]]:
                touched.add(index[(node_id, dof)])
    return [i for i in sorted(touched) if i not in restrained]


# endregion 🔖️BeamMatrices


# region 🔖️Plan
def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "shared://🎵️solves-fem3d-1-eigen/", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def json_fixture(ctx, needle):
    """🧫️ The declared JSON fixture this scenario names."""
    return json.loads(ctx.fixture_bytes(uri_in(ctx, needle)).decode("utf-8"))


def significant(value, digits=6):
    """🔢️ One number at `digits` significant figures, as normalized decimal TEXT. The closed forms
    both implementations compute from the same committed fixture agree to their last bits in
    principle and to six figures in practice; quantizing says the second, which is the honest
    demand."""
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


def within(produced, expected, tolerance=TOLERANCE):
    """✅️ Whether one produced eigenvalue sits within the stated fraction of its closed form."""
    return bool(abs(produced - expected) / max(abs(expected), 1e-30) <= tolerance)


def outcome_of(projection, reference):
    """📤️ Hands over the compared projection, carrying the FULL answer as the raw artifact — which is
    what `🔨️run-fem3d-oracle.py` commits as `📊️expected.results.json` for the subject to assert
    against.

    The projection is deliberately NOT the eigenvalues themselves. A subspace iteration and a LAPACK
    factorisation of the same pencil agree to their own convergence tolerance, not to a digit count,
    so what the two implementations are held to jointly is the thing that IS shared: the closed form,
    and whether each of them lands within two percent of it. The eigenvalues themselves are asserted
    numerically by the subject against the committed reference this file produced.
    """
    return Outcome(projection, raw=json.dumps(reference, separators=(",", ":"), ensure_ascii=False, sort_keys=True).encode("utf-8"))


def agrees(label, produced, expected, tolerance=TOLERANCE):
    """📏️ Asserts a closed form in role, before the projection is handed over."""
    scale = max(abs(expected), 1e-30)
    if abs(produced - expected) / scale > tolerance:
        raise AssertionError("%s: the reference solved %.12g where the closed form is %.12g (relative %.3g)" % (label, produced, expected, abs(produced - expected) / scale))


# endregion 🔖️Plan

# region 🔖️Handlers
def modal_handler(needle):
    """🎵️ Cantilever natural frequencies from `scipy.linalg.eigh`, held to `βₙ²/2πL²·√(EI/ρA)`."""

    def handler(ctx):
        document = json_fixture(ctx, needle)
        k, m, _kg, index = assemble(document, None)
        free = free_indices(document, index)
        values, _vectors = eigh(k[np.ix_(free, free)], m[np.ix_(free, free)])
        frequencies = [float(math.sqrt(max(value, 0.0)) / (2.0 * math.pi)) for value in values]
        section, material = document["sections"][0], document["materials"][0]
        length = document["nodes"][-1]["x"]
        betas = (1.8751040687, 4.6940911330, 7.8547574382)
        closed = sorted(
            beta**2 / (2.0 * math.pi * length**2) * math.sqrt(material["e"] * inertia / (material["rho"] * section["area"]))
            for beta in betas
            for inertia in (section["iy"], section["iz"])
        )
        for rank, expected in enumerate(closed[:4]):
            agrees("cantilever mode %d" % (rank + 1), frequencies[rank], expected)
        projection = {"model": needle, "closedFormHz": [significant(value) for value in closed[:4]], "withinTolerance": [within(frequencies[rank], expected) for rank, expected in enumerate(closed[:4])], "modeCount": document["analysis"]["modalCount"]}
        return outcome_of(projection, {"model": needle, "frequenciesHz": frequencies[: document["analysis"]["modalCount"]], "closedFormHz": closed[:4]})

    return handler


def buckling_handler(needle, effective_length_factor):
    """🏛️ Euler load factors from `scipy.linalg.eigh`, held to `π²EI/(KL)²`."""

    def handler(ctx):
        document = json_fixture(ctx, needle)
        axial = {element["id"]: -1.0 for element in document["elements"]}
        k, _m, kg, index = assemble(document, axial)
        free = free_indices(document, index)
        # `−Kg` is singular by construction (axial and torsional rows carry no geometric stiffness at
        # all), so the pencil is posed the other way round — `−Kg v = μ K v` with the positive
        # definite `K` on the right — and the load factor is `1/μ` for every positive `μ`.
        values, _vectors = eigh(-kg[np.ix_(free, free)], k[np.ix_(free, free)])
        factors = sorted(1.0 / float(value) for value in values if value > 1e-12)
        section, material = document["sections"][0], document["materials"][0]
        height = document["nodes"][-1]["z"]
        critical = math.pi**2 * material["e"] * section["iz"] / (effective_length_factor * height) ** 2
        agrees("euler critical load", factors[0], critical)
        projection = {"model": needle, "closedFormN": significant(critical), "withinTolerance": within(factors[0], critical), "factorCount": document["analysis"]["bucklingCount"]}
        return outcome_of(projection, {"model": needle, "factors": factors[: document["analysis"]["bucklingCount"]], "closedFormN": critical})

    return handler


# endregion 🔖️Handlers


# region 🔖️Registration
BUCKLING = {"pinned-pinned": 1.0, "fixed-pinned": 0.7, "fixed-fixed": 0.5, "fixed-free": 2.0}
"""🏛️ The four textbook effective-length factors, by the needle their fixtures name them with."""


def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only."""
    built = Adapter("python")
    built = built.oracle("modal-cantilever", modal_handler("modal-cantilever"))
    for needle, factor in BUCKLING.items():
        built = built.oracle("buckling-column-%s" % needle, buckling_handler("buckling-column-%s" % needle, factor))
    return built


# endregion 🔖️Registration
