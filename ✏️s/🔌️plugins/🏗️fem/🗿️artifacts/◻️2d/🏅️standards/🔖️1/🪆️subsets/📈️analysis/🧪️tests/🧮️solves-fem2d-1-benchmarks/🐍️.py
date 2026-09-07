#!/usr/bin/env python3
"""🧮️ THE THIRD-PARTY SOLVER ORACLE for `s.fem.fem2d` linear static, modal and buckling analysis.

**What is different about this case.** Every other fem2d case in this artifact judges the model
DOCUMENT: it applies a typed mutation and compares nine collections. A document comparison is the
only thing a second implementation of the mutation algebra can adjudicate, and the artifact's own
`🌐️any/🔮️oracle/🔣️.json` says so — twenty-two of the twenty-five kinds were recorded there as
owing a qualifying third-party reference, because "a finite-element solver computes displacements
and forces FROM a model; none of them reads `.dsl.semio`".

That reasoning is right about what a solver can read and wrong about what a solver can JUDGE. A
solver cannot say whether `deleteSupport` removed the right record. It can say, with total
independence, what the structure DOES once that record is gone — and that is the thing the
twenty-two kinds exist to change. So this case does not compare documents at all. It compares
ANALYSIS RESULTS: nodal displacements, support reactions and member end forces on real structural
models, and it reaches every mutation kind by mutate-then-solve — the subject applies the mutation
through production dispatch and solves the result with this repository's engine, while this
reference solves the independently committed post-mutation snapshot from scratch.

**The third party.** `anastruct` (https://github.com/ritchie46/anaStruct, GPL-3.0, test-only, never
reachable from production) is a 2D structural analysis package written by someone who has never
seen this repository. Every displacement and every reaction this file emits for a frame model comes
out of anastruct. Nothing here predicts what anastruct will answer.

**The second reference, and why there are two.** anastruct reports nodal results, not the local
end-force six-vector this artifact's `ElementResult::Beam` carries, and it solves no eigenproblem.
So this file also carries `reference`: a from-the-textbook Euler-Bernoulli frame assembly
(Cook/Malkus/Plesha, "Concepts and Applications of Finite Element Analysis", ch. 2 and 6) whose
eigenproblems are handed to `scipy.linalg`. It exists to produce the two quantities anastruct cannot
express, and it is never trusted on its own: `agree_with_anastruct` requires the two to match on
every displacement and every reaction of every case before a single number leaves this file. A
disagreement is a failure, never a fallback.

**Conventions, stated once because two solvers disagree about them.** Everything this file emits is
in the artifact's own frame: `x` to the right, `y` UP, rotation `rz` counter-clockwise positive,
lengths in metres, forces in newtons, moments in newton-metres, and a reaction is the force the
support applies TO the structure. anastruct's `get_node_displacements` already reports `ux`/`uy`
physically but reports `phi_z` clockwise-positive, and its `get_node_results_system` reports the
force the structure applies to the support — so `rz = -phi_z` and `(Fx, Fy, Mz) = -(Fx, Fy, Tz)`.
Those three sign flips were established empirically against a fixed-end cantilever under a tip
force, an axial force and a tip moment, and they are re-established on every run by
`agree_with_anastruct`.

**Member end forces.** `f_end = k_local · u_local − f_udl_local`, local dof order `[u₁,v₁,θ₁,u₂,v₂,θ₂]`,
axial reported tension-positive. That is the textbook recovery, and it is the same six-vector the
subject's `ElementResult::Beam` stations are built from, so the two are comparable without a
convention negotiation. Its magnitude at both member ends is additionally required to agree with
anastruct's own `N`/`Q`/`M` sampling.

**Self weight** is `ρ·A·g` (`g = 9.81 m/s²`) as a member-length uniform load in global `−y`. For a
consistent-mass Euler-Bernoulli element this is exactly `M·gravity`, which is what the artifact's
own multi-case assembler applies — the derivation, not the code, is what is shared.

Regions are deliberately outside the projection: a meshed continuum is not something a 2D frame
package can express, and this artifact's region geometry already has two third-party oracles
(`three-fem2d-mesh-reader`, `manifold-fem2d-mesh-measure`). The projection carries exactly the nodes
and members a frame solver can see — every node referenced by a `bar`/`beam` element, in document
order — so a region mutation is judged here on the invariance it must preserve, and on geometry
there.
"""

# region 🔖️Imports
import json
import math
import os

# 🅰️ anastruct imports matplotlib, whose macOS font enumeration reads the host font database and
# fails outright on macOS 26 (`system_profiler SPFontsDataType` no longer carries `_items`). Nothing
# here draws anything, so the font database is refused before the import rather than depended on.
os.environ.setdefault("MPL_IGNORE_SYSTEM_FONTS", "1")
os.environ.setdefault("MPLBACKEND", "Agg")

import numpy
import scipy.linalg
from anastruct import SystemElements

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
GRAVITY = 9.81
"""🌍️ The gravitational acceleration this artifact's own engine fixes at `[0, -9.81, 0]`."""

DOFS = ("Tx", "Ty", "Rz")
"""🔢️ The three planar degrees of freedom, in the artifact's own index order."""

BEAM_DOFS = ("Tx", "Ty", "Rz")
BAR_DOFS = ("Tx", "Ty")
"""🔢️ What each element kind contributes at a node. A node's active set is the union over the
elements that touch it — a node no element touches carries no equation at all, which is why a
support on such a node is inert in both implementations."""

BENDING_BETA = (1.8751040687, 4.6940911330, 7.8547574382)
"""🎵️ `βₙL` for a fixed-free prismatic Euler-Bernoulli beam (roots of `cos βL · cosh βL + 1 = 0`)."""

EFFECTIVE_LENGTH = {"col_a": 2.0, "col_b": 1.0, "col_c": 0.6992, "col_d": 0.5}
"""🏛️ The standard effective-length factor `K` of each column in `🏛️steel-columns.snapshot.json`.
`col_c` is fixed-pinned, whose exact `K` is `π/4.493409` = 0.69921, not the code value 0.7."""
# endregion 🔖️Vocabulary


# region 🔖️Model
def members_of(document):
    """🧱️ The `bar`/`beam` members, in document order. Regions are not members."""
    return [element for element in document["elements"] if element["kind"] in ("bar", "beam")]


def frame_nodes(document):
    """⚪️ Every node a member references, in DOCUMENT order — the projection's node axis.

    Two element-carrying nodes at the same coordinate would be one node to a coordinate-keyed frame
    package and two to this artifact, so that is refused rather than silently averaged.
    """
    referenced = set()
    for member in members_of(document):
        referenced.add(member["start"])
        referenced.add(member["end"])
    nodes = [node for node in document["nodes"] if node["id"] in referenced]
    seen = {}
    for node in nodes:
        key = (round(node["x"], 12), round(node["y"], 12))
        if key in seen:
            raise AssertionError("nodes %r and %r carry members at the same coordinate %r" % (seen[key], node["id"], key))
        seen[key] = node["id"]
    return nodes


def active_dofs(document):
    """🔢️ Per node id, its active degrees of freedom in `Tx, Ty, Rz` order."""
    active = {node["id"]: set() for node in document["nodes"]}
    for member in members_of(document):
        contributed = BAR_DOFS if member["kind"] == "bar" else BEAM_DOFS
        for end in (member["start"], member["end"]):
            active[end].update(contributed)
    return {node["id"]: tuple(dof for dof in DOFS if dof in active[node["id"]]) for node in document["nodes"]}


def dof_index(document):
    """🗂️ `(node id, dof) -> equation index`, node-major in document order."""
    active = active_dofs(document)
    index = {}
    for node in document["nodes"]:
        for dof in active[node["id"]]:
            index[(node["id"], dof)] = len(index)
    return index


def restrained(document):
    """🛡️ Per node id, the union of every support's restrained dofs, intersected with what is active."""
    active = active_dofs(document)
    fixed = {node["id"]: set() for node in document["nodes"]}
    for support in document["supports"]:
        if support["nodeId"] not in fixed:
            raise AssertionError("support %r names an unknown node %r" % (support["id"], support["nodeId"]))
        fixed[support["nodeId"]].update(support["fixed"])
    return {node: tuple(dof for dof in DOFS if dof in fixed[node] and dof in active[node]) for node in fixed}


def geometry(document, member):
    """📐️ `(length, cos, sin)` of one member."""
    position = {node["id"]: (node["x"], node["y"]) for node in document["nodes"]}
    (x1, y1), (x2, y2) = position[member["start"]], position[member["end"]]
    length = math.hypot(x2 - x1, y2 - y1)
    if length <= 0.0:
        raise AssertionError("member %r has zero length" % member["id"])
    return length, (x2 - x1) / length, (y2 - y1) / length


def properties(document, member):
    """🧱️ `(E, A, I, ρ)` of one member."""
    material = next(entry for entry in document["materials"] if entry["id"] == member["materialId"])
    section = next(entry for entry in document["sections"] if entry["id"] == member["sectionId"])
    return material["e"], section["area"], section["iy"], material["rho"]


def transform(cosine, sine, size):
    """🔄️ The global-to-local rotation for a `size`-dof element (6 for a beam, 4 for a bar)."""
    matrix = numpy.zeros((size, size))
    block = 3 if size == 6 else 2
    for half in range(2):
        offset = half * block
        matrix[offset + 0, offset + 0] = cosine
        matrix[offset + 0, offset + 1] = sine
        matrix[offset + 1, offset + 0] = -sine
        matrix[offset + 1, offset + 1] = cosine
        if block == 3:
            matrix[offset + 2, offset + 2] = 1.0
    return matrix


def beam_local_stiffness(length, ea, ei):
    """🧮️ The 6×6 Euler-Bernoulli frame stiffness, dof order `[u₁,v₁,θ₁,u₂,v₂,θ₂]`."""
    a, b = ea / length, ei / length
    k = numpy.zeros((6, 6))
    k[0, 0] = k[3, 3] = a
    k[0, 3] = k[3, 0] = -a
    k[1, 1] = k[4, 4] = 12.0 * b / length**2
    k[1, 4] = k[4, 1] = -12.0 * b / length**2
    k[1, 2] = k[2, 1] = k[1, 5] = k[5, 1] = 6.0 * b / length
    k[2, 4] = k[4, 2] = k[4, 5] = k[5, 4] = -6.0 * b / length
    k[2, 2] = k[5, 5] = 4.0 * b
    k[2, 5] = k[5, 2] = 2.0 * b
    return k


def beam_local_udl(length, wx, wy):
    """🌬️ The consistent nodal load vector of a member-length uniform load, in local axes."""
    return numpy.array([wx * length / 2.0, wy * length / 2.0, wy * length**2 / 12.0, wx * length / 2.0, wy * length / 2.0, -wy * length**2 / 12.0])


def beam_local_mass(length, area, density):
    """🏋️ The consistent Euler-Bernoulli mass matrix, rotary inertia neglected."""
    m = numpy.zeros((6, 6))
    axial = density * area * length / 6.0
    m[0, 0] = m[3, 3] = 2.0 * axial
    m[0, 3] = m[3, 0] = axial
    scale = density * area * length / 420.0
    bending = numpy.array([
        [156.0, 22.0 * length, 54.0, -13.0 * length],
        [22.0 * length, 4.0 * length**2, 13.0 * length, -3.0 * length**2],
        [54.0, 13.0 * length, 156.0, -22.0 * length],
        [-13.0 * length, -3.0 * length**2, -22.0 * length, 4.0 * length**2],
    ])
    for row, i in enumerate((1, 2, 4, 5)):
        for column, j in enumerate((1, 2, 4, 5)):
            m[i, j] = scale * bending[row, column]
    return m


def beam_local_geometric(length, axial):
    """🏛️ The consistent geometric stiffness of a frame element under axial force `axial`
    (tension positive), rotary terms included — Cook/Malkus/Plesha eq. 14.3-5."""
    g = numpy.zeros((6, 6))
    block = (axial / length) * numpy.array([
        [6.0 / 5.0, length / 10.0, -6.0 / 5.0, length / 10.0],
        [length / 10.0, 2.0 * length**2 / 15.0, -length / 10.0, -length**2 / 30.0],
        [-6.0 / 5.0, -length / 10.0, 6.0 / 5.0, -length / 10.0],
        [length / 10.0, -length**2 / 30.0, -length / 10.0, 2.0 * length**2 / 15.0],
    ])
    for row, i in enumerate((1, 2, 4, 5)):
        for column, j in enumerate((1, 2, 4, 5)):
            g[i, j] = block[row, column]
    return g


def bar_local_stiffness(length, ea):
    """🧮️ The 4×4 axial-only bar stiffness, dof order `[u₁,v₁,u₂,v₂]`."""
    k = numpy.zeros((4, 4))
    k[0, 0] = k[2, 2] = ea / length
    k[0, 2] = k[2, 0] = -ea / length
    return k


def bar_local_mass(length, area, density):
    """🏋️ The consistent bar mass matrix, dof order `[u₁,v₁,u₂,v₂]`."""
    scale = density * area * length / 6.0
    m = numpy.zeros((4, 4))
    for offset in (0, 1):
        m[offset, offset] = m[offset + 2, offset + 2] = 2.0 * scale
        m[offset, offset + 2] = m[offset + 2, offset] = scale
    return m


def member_indices(index, member):
    """🗂️ One member's global equation indices, in its own local dof order."""
    dofs = BAR_DOFS if member["kind"] == "bar" else BEAM_DOFS
    return [index[(end, dof)] for end in (member["start"], member["end"]) for dof in dofs]
# endregion 🔖️Model


# region 🔖️Reference
def stiffness(document):
    """🧮️ The assembled global stiffness matrix over every active degree of freedom."""
    index = dof_index(document)
    k = numpy.zeros((len(index), len(index)))
    for member in members_of(document):
        length, cosine, sine = geometry(document, member)
        e, area, inertia, _ = properties(document, member)
        if member["kind"] == "bar":
            local, size = bar_local_stiffness(length, e * area), 4
        else:
            local, size = beam_local_stiffness(length, e * area, e * inertia), 6
        rotation = transform(cosine, sine, size)
        glob = rotation.T @ local @ rotation
        where = member_indices(index, member)
        k[numpy.ix_(where, where)] += glob
    return k


def mass(document):
    """🏋️ The assembled global consistent mass matrix."""
    index = dof_index(document)
    m = numpy.zeros((len(index), len(index)))
    for member in members_of(document):
        length, cosine, sine = geometry(document, member)
        _, area, _, density = properties(document, member)
        if member["kind"] == "bar":
            local, size = bar_local_mass(length, area, density), 4
        else:
            local, size = beam_local_mass(length, area, density), 6
        rotation = transform(cosine, sine, size)
        where = member_indices(index, member)
        m[numpy.ix_(where, where)] += rotation.T @ local @ rotation
    return m


def member_udls(document, case):
    """🌬️ Per member id, the global `(wx, wy)` this case applies — the case's own uniform loads plus,
    when the case carries self weight, `ρAg` downward."""
    udl = {}
    for load in case["loads"]:
        if load["kind"] == "memberUdl":
            wx, wy = udl.get(load["elementId"], (0.0, 0.0))
            udl[load["elementId"]] = (wx + load["wx"], wy + load["wy"])
    if case["selfWeight"]:
        for member in members_of(document):
            if member["kind"] == "bar":
                continue
            _, area, _, density = properties(document, member)
            wx, wy = udl.get(member["id"], (0.0, 0.0))
            udl[member["id"]] = (wx, wy - density * area * GRAVITY)
    return udl


def load_vector(document, case):
    """🌬️ One case's right-hand side over every active degree of freedom."""
    index = dof_index(document)
    f = numpy.zeros(len(index))
    udl = member_udls(document, case)
    for member in members_of(document):
        length, cosine, sine = geometry(document, member)
        where = member_indices(index, member)
        if member["kind"] == "bar":
            if member["id"] in udl:
                raise AssertionError("member %r is a bar and carries no uniform-load formulation" % member["id"])
            if case["selfWeight"]:
                _, area, _, density = properties(document, member)
                half = density * area * length * GRAVITY / 2.0
                f[index[(member["start"], "Ty")]] -= half
                f[index[(member["end"], "Ty")]] -= half
            continue
        wx, wy = udl.get(member["id"], (0.0, 0.0))
        if wx != 0.0 or wy != 0.0:
            rotation = transform(cosine, sine, 6)
            local = beam_local_udl(length, wx * cosine + wy * sine, -wx * sine + wy * cosine)
            f[where] += rotation.T @ local
    for load in case["loads"]:
        if load["kind"] == "nodal":
            key = (load["nodeId"], load["dof"])
            if key in index:
                f[index[key]] += load["value"]
        elif load["kind"] not in ("memberUdl", "area"):
            raise AssertionError("unknown load kind %r" % load["kind"])
    return f


def partition(document):
    """🛡️ `(free indices, fixed indices)` of the assembled system."""
    index = dof_index(document)
    fixed_of = restrained(document)
    fixed = sorted(index[(node, dof)] for node, dofs in fixed_of.items() for dof in dofs if (node, dof) in index)
    free = [equation for equation in range(len(index)) if equation not in set(fixed)]
    return free, fixed


def smallest_free_eigenvalue(document):
    """🚨️ The smallest eigenvalue of the restrained stiffness matrix, normalised by its largest.

    A mechanism — a model whose supports leave a rigid-body or hinge mode — has a zero here. That is
    the singular condition both implementations must report, and `scipy` decides it, not this file.
    """
    free, _ = partition(document)
    if not free:
        return 1.0
    k = stiffness(document)[numpy.ix_(free, free)]
    values = numpy.linalg.eigvalsh(k)
    return float(abs(values[0]) / max(abs(values[-1]), 1e-300))


def reference_case(document, case):
    """🧮️ One load case, solved from the textbook assembly: displacements, reactions, end forces."""
    index = dof_index(document)
    free, fixed = partition(document)
    k = stiffness(document)
    f = load_vector(document, case)
    u = numpy.zeros(len(index))
    if free:
        u[free] = numpy.linalg.solve(k[numpy.ix_(free, free)], f[free])
    residual = k @ u - f

    displacements = {}
    for node in frame_nodes(document):
        displacements[node["id"]] = [
            float(u[index[(node["id"], dof)]]) if (node["id"], dof) in index else 0.0
            for dof in DOFS
        ]
    reactions = {}
    for node, dofs in restrained(document).items():
        for dof in dofs:
            reactions[(node, dof)] = float(residual[index[(node, dof)]])

    udl = member_udls(document, case)
    elements = {}
    for member in members_of(document):
        length, cosine, sine = geometry(document, member)
        e, area, inertia, _ = properties(document, member)
        where = member_indices(index, member)
        if member["kind"] == "bar":
            local = transform(cosine, sine, 4) @ u[where]
            axial = (e * area / length) * (local[2] - local[0])
            elements[member["id"]] = [float(axial), 0.0, 0.0, float(axial), 0.0, 0.0]
            continue
        wx, wy = udl.get(member["id"], (0.0, 0.0))
        wy_local = -wx * sine + wy * cosine
        local = transform(cosine, sine, 6) @ u[where]
        end = beam_local_stiffness(length, e * area, e * inertia) @ local - beam_local_udl(length, wx * cosine + wy * sine, wy_local)
        elements[member["id"]] = [
            float(-end[0]), float(end[1]), float(-end[2]),
            float(-end[0]), float(end[1] + wy_local * length), float(-end[2] + end[1] * length + wy_local * length**2 / 2.0),
        ]
    return {"displacements": displacements, "reactions": reactions, "elements": elements}


def scale_case(result, factor):
    """✖️ One case result times a combination factor."""
    return {
        "displacements": {node: [value * factor for value in values] for node, values in result["displacements"].items()},
        "reactions": {key: value * factor for key, value in result["reactions"].items()},
        "elements": {member: [value * factor for value in values] for member, values in result["elements"].items()},
    }


def add_case(left, right):
    """➕️ Superposition of two case results — linear statics, exactly as the artifact combines them."""
    return {
        "displacements": {node: [a + b for a, b in zip(values, right["displacements"][node])] for node, values in left["displacements"].items()},
        "reactions": {key: value + right["reactions"][key] for key, value in left["reactions"].items()},
        "elements": {member: [a + b for a, b in zip(values, right["elements"][member])] for member, values in left["elements"].items()},
    }


def reference_all(document):
    """🧮️ Every load case and every combination, keyed by id — the shape `fem2d_solve_all` returns."""
    results = {case["id"]: reference_case(document, case) for case in document["loadCases"]}
    for combination in document["combinations"]:
        total = None
        for term in combination["terms"]:
            if term["caseId"] not in results:
                raise AssertionError("combination %r names an unknown case %r" % (combination["id"], term["caseId"]))
            scaled = scale_case(results[term["caseId"]], term["factor"])
            total = scaled if total is None else add_case(total, scaled)
        if total is None:
            raise AssertionError("combination %r carries no terms" % combination["id"])
        results[combination["id"]] = total
    return results
# endregion 🔖️Reference


# region 🔖️ThirdParty
def anastruct_system(document, case):
    """🅰️ Builds one load case of a frame model in anastruct and solves it.

    Returns `(system, node id -> anastruct node id, our element id -> anastruct element id)`.
    """
    system = SystemElements()
    position = {node["id"]: [node["x"], node["y"]] for node in document["nodes"]}
    element_ids = {}
    for member in members_of(document):
        e, area, inertia, _ = properties(document, member)
        location = [position[member["start"]], position[member["end"]]]
        if member["kind"] == "bar":
            element_ids[member["id"]] = system.add_truss_element(location=location, EA=e * area)
        else:
            element_ids[member["id"]] = system.add_element(location=location, EA=e * area, EI=e * inertia)

    node_ids = {}
    for node in frame_nodes(document):
        found = system.find_node_id(position[node["id"]])
        if found is None:
            raise AssertionError("anastruct did not place a node at %r" % position[node["id"]])
        node_ids[node["id"]] = found

    for node, dofs in restrained(document).items():
        if not dofs or node not in node_ids:
            continue
        fixed, where = set(dofs), node_ids[node]
        if fixed == {"Tx", "Ty", "Rz"}:
            system.add_support_fixed(node_id=where)
        elif fixed == {"Tx", "Ty"}:
            system.add_support_hinged(node_id=where)
        elif fixed in ({"Ty"}, {"Ty", "Rz"}):
            system.add_support_roll(node_id=where, direction="x", rotate="Rz" not in fixed)
        elif fixed in ({"Tx"}, {"Tx", "Rz"}):
            system.add_support_roll(node_id=where, direction="y", rotate="Rz" not in fixed)
        else:
            raise AssertionError("anastruct has no support expressing %r" % sorted(fixed))

    for member, (wx, wy) in member_udls(document, case).items():
        if wx != 0.0:
            raise AssertionError("member %r carries a longitudinal uniform load, which this reference does not model" % member)
        if wy != 0.0:
            system.q_load(q=wy, element_id=element_ids[member], direction="y")
    if case["selfWeight"]:
        for member in members_of(document):
            if member["kind"] != "bar":
                continue
            length, _, _ = geometry(document, member)
            _, area, _, density = properties(document, member)
            half = density * area * length * GRAVITY / 2.0
            for end in (member["start"], member["end"]):
                system.point_load(node_id=node_ids[end], Fy=-half)
    for load in case["loads"]:
        if load["kind"] != "nodal" or load["nodeId"] not in node_ids:
            continue
        where = node_ids[load["nodeId"]]
        if load["dof"] == "Tx":
            system.point_load(node_id=where, Fx=load["value"])
        elif load["dof"] == "Ty":
            system.point_load(node_id=where, Fy=load["value"])
        else:
            system.moment_load(node_id=where, Tz=load["value"])
    system.solve()
    return system, node_ids, element_ids


def anastruct_case(document, case):
    """🅰️ One case's displacements and reactions, as anastruct answers them, in this artifact's frame."""
    system, node_ids, _ = anastruct_system(document, case)
    displacements, reactions = {}, {}
    fixed_of = restrained(document)
    member = {"Tx": "Fx", "Ty": "Fy", "Rz": "Tz"}
    for node in frame_nodes(document):
        where = node_ids[node["id"]]
        moved = system.get_node_displacements(node_id=where)
        displacements[node["id"]] = [float(moved["ux"]), float(moved["uy"]), float(-moved["phi_z"])]
        held = system.get_node_results_system(node_id=where)
        for dof in fixed_of[node["id"]]:
            reactions[(node["id"], dof)] = -float(held[member[dof]])
    return displacements, reactions


def anastruct_member_ends(document, case):
    """🅰️ Per member, anastruct's own `|N|`, `|Q|` and `|M|` AT THE TWO ENDS — the independent
    magnitude check on the end-force recovery this file emits.

    At the ends, not at the extremum: under a member-length uniform load the largest moment sits
    inside the span, and comparing an interior maximum against an end value would compare two
    different quantities.
    """
    system, _, element_ids = anastruct_system(document, case)
    ends = {}
    for member in members_of(document):
        if member["kind"] == "bar":
            continue
        result = system.get_element_results(element_id=element_ids[member["id"]], verbose=True)
        ends[member["id"]] = tuple((abs(float(result[label][0])), abs(float(result[label][-1]))) for label in ("N", "Q", "M"))
    return ends


def relative(left, right, floor):
    """📏️ `|a − b|` measured against the larger of the pair and a stated absolute floor."""
    return abs(left - right) / max(abs(left), abs(right), floor)


def agree_with_anastruct(document, results, tolerance=1e-6):
    """🤝 Requires anastruct to reproduce every displacement and every reaction of every LOAD CASE.

    Combinations are linear superpositions of cases and add no independent information, so they are
    not re-solved. The floors are the case's own peak response: agreement is judged against what the
    structure actually does, not against a number in metres that means something different for a
    41 mm cantilever tip and a 4 µm column shortening.
    """
    for case in document["loadCases"]:
        mine = results[case["id"]]
        theirs_displacements, theirs_reactions = anastruct_case(document, case)
        peak_translation = max([abs(value) for values in mine["displacements"].values() for value in values[:2]] + [1e-18])
        peak_rotation = max([abs(values[2]) for values in mine["displacements"].values()] + [peak_translation * 1e-6, 1e-18])
        peak_reaction = max([abs(value) for value in mine["reactions"].values()] + [1e-12])
        for node, values in mine["displacements"].items():
            for at, (dof, floor) in enumerate(zip(DOFS, (peak_translation, peak_translation, peak_rotation))):
                error = relative(values[at], theirs_displacements[node][at], floor)
                if error > tolerance:
                    raise AssertionError("case %s: anastruct puts %s.%s at %.12g, this reference at %.12g (relative %.3g)" % (case["id"], node, dof, theirs_displacements[node][at], values[at], error))
        for key, value in mine["reactions"].items():
            error = relative(value, theirs_reactions[key], peak_reaction)
            if error > tolerance:
                raise AssertionError("case %s: anastruct puts the reaction at %s.%s at %.12g, this reference at %.12g (relative %.3g)" % (case["id"], key[0], key[1], theirs_reactions[key], value, error))
        ends = anastruct_member_ends(document, case)
        for member in members_of(document):
            if member["kind"] == "bar":
                continue
            axial, shear, moment = ends[member["id"]]
            ours = mine["elements"][member["id"]]
            for label, theirs, mine_pair, floor in (
                ("N", axial, (ours[0], ours[3]), peak_reaction),
                ("Q", shear, (ours[1], ours[4]), peak_reaction),
                ("M", moment, (ours[2], ours[5]), peak_reaction * 10.0),
            ):
                for at in ((0,) if label == "N" else (0, 1)):
                    error = relative(abs(mine_pair[at]), theirs[at], floor)
                    if error > 1e-5:
                        raise AssertionError("case %s: anastruct puts |%s| at end %d of %s at %.12g, this reference at %.12g (relative %.3g)" % (case["id"], label, at, member["id"], theirs[at], abs(mine_pair[at]), error))
    return results
# endregion 🔖️ThirdParty


# region 🔖️Eigen
def modal_frequencies(document, count):
    """🎵️ The lowest `count` natural frequencies in hertz, from `scipy.linalg.eigh` on the restrained
    stiffness and consistent mass matrices."""
    free, _ = partition(document)
    k = stiffness(document)[numpy.ix_(free, free)]
    m = mass(document)[numpy.ix_(free, free)]
    values = scipy.linalg.eigh(k, m, eigvals_only=True)
    return [float(math.sqrt(max(value, 0.0)) / (2.0 * math.pi)) for value in values[:count]]


def buckling_factors(document, case_id, count):
    """🏛️ The lowest `count` linear-buckling load factors of one case, from `scipy.linalg.eig` on the
    pencil `K φ = λ (−K_g) φ` where `K_g` is built from that case's own member axial forces."""
    case = next(entry for entry in document["loadCases"] if entry["id"] == case_id)
    result = reference_case(document, case)
    index = dof_index(document)
    geometric = numpy.zeros((len(index), len(index)))
    for member in members_of(document):
        if member["kind"] == "bar":
            continue
        length, cosine, sine = geometry(document, member)
        axial = result["elements"][member["id"]][0]
        rotation = transform(cosine, sine, 6)
        where = member_indices(index, member)
        geometric[numpy.ix_(where, where)] += rotation.T @ beam_local_geometric(length, axial) @ rotation
    free, _ = partition(document)
    k = stiffness(document)[numpy.ix_(free, free)]
    g = geometric[numpy.ix_(free, free)]
    values = scipy.linalg.eig(k, -g, right=False)
    finite = sorted(float(value.real) for value in values if numpy.isfinite(value) and abs(value.imag) < 1e-6 * max(abs(value.real), 1.0) and value.real > 1e-9)
    return finite[:count]


def euler_load(document, case_id):
    """🏛️ `π²EI/(KL)²` for the one column a `🏛️steel-columns` case compresses."""
    column = case_id.split("_")[1]
    members = [member for member in members_of(document) if member["id"].startswith("e_%s" % column)]
    e, _, inertia, _ = properties(document, members[0])
    height = sum(geometry(document, member)[0] for member in members)
    return math.pi**2 * e * inertia / (EFFECTIVE_LENGTH[case_id] * height) ** 2
# endregion 🔖️Eigen


# region 🔖️Projection
def decade(value):
    """📏️ The power of ten at or above `value` — a stable, human-readable normalisation scale."""
    if not math.isfinite(value) or value <= 0.0:
        return 1.0
    return float(10.0 ** math.ceil(math.log10(value)))


def scales_of(results):
    """📏️ The four normalisation decades of a result map: translation, rotation, force, moment.

    The cross-language comparison profile carries ONE absolute tolerance, so a projection that mixed
    metres at 1e-5 with newtons at 1e5 could not be judged by it at all. Dividing by these committed
    decades makes every projected number order one, and the profile's tolerance then reads as a
    fraction of the model's own peak response.
    """
    translation = max([abs(value) for case in results.values() for values in case["displacements"].values() for value in values[:2]] + [0.0])
    rotation = max([abs(values[2]) for case in results.values() for values in case["displacements"].values()] + [0.0])
    force = max([abs(value) for case in results.values() for value in case["reactions"].values()] + [abs(value) for case in results.values() for values in case["elements"].values() for value in values[:2] + values[3:5]] + [0.0])
    moment = max([abs(values[2]) for case in results.values() for values in case["elements"].values()] + [abs(values[5]) for case in results.values() for values in case["elements"].values()] + [0.0])
    return {"translation": decade(translation), "rotation": decade(rotation), "force": decade(force), "moment": decade(moment)}


def projection_of(document, results, scales):
    """📤️ The scale-normalised cross-language projection: cases in document order, then combinations;
    nodes and members in document order; reactions in `(node, dof)` document order."""
    order = [case["id"] for case in document["loadCases"]] + [combination["id"] for combination in document["combinations"]]
    nodes = [node["id"] for node in frame_nodes(document)]
    fixed_of = restrained(document)
    pairs = [(node, dof) for node in nodes for dof in fixed_of[node]]
    members = [member["id"] for member in members_of(document)]
    cases = []
    for case_id in order:
        result = results[case_id]
        cases.append({
            "id": case_id,
            "displacements": [[result["displacements"][node][0] / scales["translation"], result["displacements"][node][1] / scales["translation"], result["displacements"][node][2] / scales["rotation"]] for node in nodes],
            "reactions": [result["reactions"][pair] / (scales["moment"] if pair[1] == "Rz" else scales["force"]) for pair in pairs],
            "elements": [[
                result["elements"][member][0] / scales["force"], result["elements"][member][1] / scales["force"], result["elements"][member][2] / scales["moment"],
                result["elements"][member][3] / scales["force"], result["elements"][member][4] / scales["force"], result["elements"][member][5] / scales["moment"],
            ] for member in members],
        })
    return {"nodes": nodes, "reactions": ["%s.%s" % pair for pair in pairs], "members": members, "cases": cases}


def summary_of(results):
    """📊️ The compact per-case digest the committed reference file pins, in SI units."""
    digest = {}
    for case_id, result in results.items():
        digest[case_id] = {
            "peakTranslation": max([abs(value) for values in result["displacements"].values() for value in values[:2]] + [0.0]),
            "peakRotation": max([abs(values[2]) for values in result["displacements"].values()] + [0.0]),
            "reactionFx": sum(value for (_, dof), value in result["reactions"].items() if dof == "Tx"),
            "reactionFy": sum(value for (_, dof), value in result["reactions"].items() if dof == "Ty"),
            "peakMoment": max([abs(value) for values in result["elements"].values() for value in (values[2], values[5])] + [0.0]),
            "peakAxial": max([abs(value) for values in result["elements"].values() for value in (values[0], values[3])] + [0.0]),
        }
    return digest


def raw_of(document, results):
    """📊️ The full per-case reference values in SI units, for the committed reference file."""
    nodes = [node["id"] for node in frame_nodes(document)]
    fixed_of = restrained(document)
    members = [member["id"] for member in members_of(document)]
    return {
        case_id: {
            "displacements": {node: results[case_id]["displacements"][node] for node in nodes},
            "reactions": {"%s.%s" % (node, dof): results[case_id]["reactions"][(node, dof)] for node in nodes for dof in fixed_of[node]},
            "elements": {member: results[case_id]["elements"][member] for member in members},
        }
        for case_id in results
    }
# endregion 🔖️Projection


# region 🔖️Plan
def fixture_uri(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "local://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def json_fixture(ctx, needle):
    """🧫️ The declared JSON fixture this scenario names."""
    return json.loads(ctx.fixture_bytes(fixture_uri(ctx, needle)).decode("utf-8"))


def expected(ctx):
    """📊️ The committed reference values."""
    return json_fixture(ctx, "expected.results")


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialisation as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def close_enough(produced, committed, floor, tolerance, what):
    """🎯️ Holds one produced value to the committed reference, relative to a stated floor."""
    error = relative(produced, committed, floor)
    if error > tolerance:
        raise AssertionError("%s: this run produced %.12g, the committed reference says %.12g (relative %.3g > %.3g)" % (what, produced, committed, error, tolerance))
# endregion 🔖️Plan


# region 🔖️Handlers
def solves_handler(fixture):
    """🧮️ Solves one committed benchmark snapshot with anastruct, holds it to the committed reference
    values and to whatever closed form the model has, and projects the normalised result."""

    def handler(ctx):
        document = json_fixture(ctx, fixture)
        results = agree_with_anastruct(document, reference_all(document))
        reference = expected(ctx)["fixtures"][fixture]
        raw = raw_of(document, results)
        for case_id, case in reference["cases"].items():
            floor = max([abs(value) for values in raw[case_id]["displacements"].values() for value in values[:2]] + [1e-18])
            turn = max([abs(values[2]) for values in raw[case_id]["displacements"].values()] + [floor * 1e-6, 1e-18])
            for node, values in case["displacements"].items():
                for at in range(3):
                    close_enough(raw[case_id]["displacements"][node][at], values[at], floor if at < 2 else turn, 1e-9, "%s/%s: %s.%s" % (fixture, case_id, node, DOFS[at]))
            forces = max([abs(value) for value in case["reactions"].values()] + [1e-12])
            for key, value in case["reactions"].items():
                close_enough(raw[case_id]["reactions"][key], value, forces, 1e-9, "%s/%s: reaction %s" % (fixture, case_id, key))
            for member, values in case["elements"].items():
                for at in range(6):
                    close_enough(raw[case_id]["elements"][member][at], values[at], forces * (10.0 if at in (2, 5) else 1.0), 1e-9, "%s/%s: %s[%d]" % (fixture, case_id, member, at))
        for claim in reference.get("closedForm", []):
            produced = raw[claim["case"]]["displacements"][claim["node"]][DOFS.index(claim["dof"])] if claim["quantity"] == "displacement" else sum(raw[claim["case"]]["reactions"][key] for key in claim["reactions"])
            close_enough(produced, claim["value"], abs(claim["value"]), claim["tolerance"], "%s/%s: %s" % (fixture, claim["case"], claim["what"]))
        return outcome_of(projection_of(document, results, reference["scales"]))

    return handler


def mutated_handler(kind):
    """🧬️ Solves the committed post-mutation snapshot of one mutation kind, independently of how — or
    whether — the subject's production mutation produced the same document."""

    def handler(ctx):
        corpus = json_fixture(ctx, "mutated.snapshots")
        entry = corpus["kinds"][kind]
        document = entry["after"]
        results = agree_with_anastruct(document, reference_all(document))
        reference = expected(ctx)["mutated"][kind]
        produced = summary_of(results)
        for case_id, digest in reference["summary"].items():
            floor = max([abs(value) for value in digest.values()] + [1e-12])
            for name, value in digest.items():
                close_enough(produced[case_id][name], value, max(abs(value), floor * 1e-9), 1e-9, "%s/%s: %s" % (kind, case_id, name))
        base = json_fixture(ctx, "steel-frame-base")
        base_summary = summary_of(reference_all(base))
        shared = set(produced) & set(base_summary)
        moved = any(relative(produced[case_id][name], base_summary[case_id][name], 1e-12) > 1e-9 for case_id in shared for name in produced[case_id])
        if entry["effect"] == "changes" and not (moved or set(produced) != set(base_summary)):
            raise AssertionError("%s: the feature declares this kind changes the analysis, but every case answers exactly as the base model does" % kind)
        if entry["effect"] == "invariant" and moved:
            raise AssertionError("%s: the feature declares this kind leaves the frame analysis untouched, but a case moved" % kind)
        return outcome_of(projection_of(document, results, reference["scales"]))

    return handler


def modal_handler(ctx):
    """🎵️ The cantilever's natural frequencies, against `βₙ` closed form and the committed reference."""
    document = json_fixture(ctx, "steel-cantilever")
    reference = expected(ctx)["modal"]
    produced = modal_frequencies(document, document["analysis"]["modalCount"])
    members = members_of(document)
    e, area, inertia, density = properties(document, members[0])
    span = sum(geometry(document, member)[0] for member in members)
    for at, beta in enumerate(BENDING_BETA[: len(produced)]):
        closed = beta**2 / (2.0 * math.pi * span**2) * math.sqrt(e * inertia / (density * area))
        close_enough(produced[at], closed, abs(closed), 0.02, "modal mode %d against βₙL = %.7f" % (at + 1, beta))
        close_enough(produced[at], reference["frequenciesHz"][at], abs(reference["frequenciesHz"][at]), 1e-9, "modal mode %d against the committed reference" % (at + 1))
    return outcome_of({"frequenciesHz": [value / reference["scale"] for value in produced]})


def buckling_handler(ctx):
    """🏛️ Each column's lowest buckling factor, against `π²EI/(KL)²` and the committed reference."""
    document = json_fixture(ctx, "steel-columns")
    reference = expected(ctx)["buckling"]
    produced = {}
    for case in document["loadCases"]:
        applied = abs(case["loads"][0]["value"])
        factors = buckling_factors(document, case["id"], document["analysis"]["bucklingCount"])
        produced[case["id"]] = factors[0]
        closed = euler_load(document, case["id"]) / applied
        close_enough(factors[0], closed, abs(closed), 0.02, "%s against π²EI/(KL)² at K = %.4f" % (case["id"], EFFECTIVE_LENGTH[case["id"]]))
        close_enough(factors[0], reference["factors"][case["id"]], abs(reference["factors"][case["id"]]), 1e-9, "%s against the committed reference" % case["id"])
    return outcome_of({"factors": {case_id: value / reference["scale"] for case_id, value in sorted(produced.items())}})


def mechanism_handler(ctx):
    """🚨️ A cantilever stripped of its only support is a mechanism, and both implementations must
    refuse it rather than answer. `scipy` decides, from the rank of the restrained stiffness matrix."""
    document = json_fixture(ctx, "steel-cantilever")
    held = smallest_free_eigenvalue(document)
    released = smallest_free_eigenvalue(dict(document, supports=[]))
    if released >= 1e-12:
        raise AssertionError("a cantilever with every support deleted was expected to be a mechanism, but its stiffness matrix is regular (%.3g)" % released)
    if held < 1e-12:
        raise AssertionError("the supported cantilever was expected to be regular, but its stiffness matrix is singular (%.3g)" % held)
    return outcome_of({"supported": "regular", "released": "singular"})


def substructure_handler(ctx):
    """🪵️ Holds the derived frame-substructure fixture to the committed real-world timber portal frame
    it was taken from, and states the structural reason the committed document itself is not solved.

    The committed model's `slab_spare` region touches the frame at exactly ONE node position. A
    meshed continuum attached at a single point can rotate rigidly about it, which is a zero-energy
    mode of the full model — the region exists so the `delete-`/`replace-region` verbs have a
    trailing target, not to be analysed. That single-attachment fact is checked here rather than
    asserted in prose; the mechanism it implies belongs to the continuum, which a 2D frame package
    cannot see and therefore cannot judge.
    """
    committed = json_fixture(ctx, "timber-portal-frame")
    derived = json_fixture(ctx, "timber-frame-members")
    for member in ("nodes", "elements", "materials", "sections", "supports"):
        if derived[member] != committed[member]:
            raise AssertionError("the derived frame substructure changed %s, which the derivation may not touch" % member)
    if derived["regions"] or derived["combinations"]:
        raise AssertionError("the derived frame substructure must carry no region and no combination")
    kept = [case for case in committed["loadCases"] if not any(load["kind"] == "area" for load in case["loads"])]
    if [case["id"] for case in derived["loadCases"]] != [case["id"] for case in kept if case["id"] == "snow"]:
        raise AssertionError("the derived frame substructure keeps exactly the committed cases that carry no area load")

    positions = {(round(node["x"], 9), round(node["y"], 9)) for node in committed["nodes"]}
    attachment = {}
    for region in committed["regions"]:
        attachment[region["id"]] = sorted("%.9g,%.9g" % (x, y) for x, y in region["outline"] if (round(x, 9), round(y, 9)) in positions)
    if len(attachment.get("slab_spare", [])) != 1:
        raise AssertionError("slab_spare was expected to touch the frame at exactly one node, it touches %r" % attachment.get("slab_spare"))
    return outcome_of({"regionAttachments": attachment, "cases": [case["id"] for case in derived["loadCases"]]})
# endregion 🔖️Handlers


# region 🔖️Reference file
BENCHMARKS = (
    ("steel-cantilever", "📏️steel-cantilever.snapshot.json"),
    ("steel-simple-beam", "📐️steel-simple-beam.snapshot.json"),
    ("concrete-two-span", "🌉️concrete-two-span.snapshot.json"),
    ("timber-frame-members", "🪵️timber-frame-members.snapshot.json"),
    ("steel-frame-base", "🏢️steel-frame-base.snapshot.json"),
    ("steel-columns", "🏛️steel-columns.snapshot.json"),
)
"""🧫️ `(scenario suffix, committed snapshot)` per benchmark, in the order the feature states them."""

KINDS = (
    "create-node", "delete-node", "create-element", "delete-element", "replace-element",
    "create-material", "delete-material", "replace-material",
    "create-section", "delete-section", "replace-section",
    "create-support", "delete-support", "replace-support",
    "create-region", "delete-region", "replace-region",
    "create-load-case", "delete-load-case", "add-load", "remove-load", "change-load-case-self-weight",
    "create-combination", "delete-combination", "update-analysis-settings",
)
"""🏷️ Every typed fem2d mutation kind, in the artifact's own catalog order."""
# endregion 🔖️Reference file


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only. The subject is `🦀️.rs`
    beside this file: registering these handlers as subjects too would make the reference its own
    subject and manufacture a green self-comparison."""
    built = Adapter("python")
    for scenario, fixture in BENCHMARKS:
        built = built.oracle("solves-%s" % scenario, solves_handler(fixture))
    for kind in KINDS:
        built = built.oracle("solves-after-%s" % kind, mutated_handler(kind))
    built = built.oracle("modal-cantilever-frequencies", modal_handler)
    built = built.oracle("buckling-column-factors", buckling_handler)
    built = built.oracle("refuses-a-mechanism", mechanism_handler)
    built = built.oracle("derives-the-timber-frame-substructure", substructure_handler)
    return built
# endregion 🔖️Registration
