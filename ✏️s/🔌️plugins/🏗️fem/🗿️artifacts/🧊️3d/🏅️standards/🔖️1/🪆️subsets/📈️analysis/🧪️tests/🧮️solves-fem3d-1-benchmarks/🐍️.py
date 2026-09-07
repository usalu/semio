#!/usr/bin/env python3
"""🧮 THIRD-PARTY FEM SOLVER oracle for `s.fem.fem3d`'s analysis results.

**What this file is, and why it is not another second implementation.** Every other reference in
this artifact adjudicates the model ALGEBRA — what a typed mutation writes into the document. This
one adjudicates the ANSWER: given a committed structural model, what displacements, reactions and
member forces does it have. That question has a real third-party arbiter, and this file uses one,
not written here:

* **PyNite** (`PyNiteFEA`, MIT, `Pynite.FEModel3D`) — an independent 3D frame/truss finite-element
  solver. It assembles its own stiffness, applies its own boundary conditions and solves its own
  system; nothing of this repository's kernel reaches it.
Alongside it sits a CLOSED FORM. A library can be wrong; a library that agrees with `PL³/3EI`,
`PL²/2EI`, `5wL⁴/384EI`, `TL/GJ` and global equilibrium on every case is not. Each closed-form value
is stated in the scenario it belongs to and asserted here, in role, before the projection is handed
over — so a silent regression in PyNite itself cannot quietly become this repository's new
expectation. The two analyses PyNite is NOT asked about have their own sibling cases and their own
third-party references: `../🧱️solves-fem3d-1-solid` (scikit-fem, 3D continuum elasticity) and
`../🎵️solves-fem3d-1-eigen` (SciPy, modal and linear buckling).

**Coordinates.** This artifact is Z-up (`FemNode.{x,y,z}`, gravity `[0, 0, -9.81]`); PyNite is Y-up
(its vertical-member and horizontal-member local-axis rules are written around global `Y`). The two
are related by the proper rotation `(X, Y, Z)ₚ = (x, z, −y)`, whose determinant is `+1`, so
handedness — and therefore every cross product either solver takes — is preserved. Displacement,
rotation, force and moment vectors map back through its inverse `(x, y, z) = (X, −Z, Y)ₚ`.

**Member local axes.** Both solvers build a member's local triad from a reference direction and then
roll it about the member axis, but they choose different references (this artifact: global `+Z`,
falling back to `+X` for a near-vertical member; PyNite: global `+Y`). Rather than assume the two
land on the same axes — they do not — this file computes the artifact's OWN triad from its
documented rule, maps it into PyNite's frame, and solves for the `rotation` angle that makes
PyNite's triad coincide with it. The construction is then checked against PyNite's own `T()` matrix
to `1e-12` per member, so `Iy`/`Iz` genuinely mean the same thing on both sides or the scenario
fails before it compares anything.

**What a `bar` is to PyNite.** `FemElement::Bar` is a two-force member with three translational
degrees of freedom per node and no bending stiffness at all. PyNite's `Spring3D` is exactly that —
an axial `ks` along the line between two nodes — so a bar maps to `add_spring(ks = EA/L)`, and every
rotational degree of freedom at a node that only bars touch is restrained, mirroring this artifact's
own active-degree-of-freedom rule (`crate::model::build_dof_map`: a node carries the union of the
degrees of freedom of the elements incident on it, and nothing else).

**The line-element projection.** One committed model this case solves — the shared
`🧊️steel-frame.snapshot.json` every fem3d subset case uses — also carries a meshed `FemSolid`, and a
frame solver cannot see a tetrahedron. Every scenario here is therefore declared over the model's
LINE-ELEMENT sub-document: the same document with `solids` emptied and every `area` load dropped.
Both implementations apply that projection, to the same committed bytes, before solving — it scopes
the verdict, it does not soften it. The solid path has its own oracle in the sibling case
`../🧱️solves-fem3d-1-solid`, on a fixture built for it.

**Why the mutate-then-solve scenarios carry their own pair.** The committed specification vectors of
the mutation subsets are algebra fixtures, and their line-element sub-model is a MECHANISM — the one
frame they hold hangs off a node whose only other stiffness comes from the solid's tetrahedra, so no
frame solver, this repository's included, can read them. Solving them would report "singular" on both
sides and prove nothing. So this case commits its own `🦠️mutation-base.snapshot.json`: a portal frame
that actually stands up, carrying the same spares the mutation corpus needs, plus one after-model per
kind. The feature states the typed payload that relates each pair, the subject reaches its
after-model through PRODUCTION dispatch and is held to the committed one, and this reference simply
solves the committed one.

**Distributed load and self weight on a bar.** A two-force member cannot carry a moment, so a
uniform load along it — its own weight included — reaches its two ends in equal halves, `wL/2`. That
is the classical statics of a pin-ended member, not a reading of this repository's kernel, and it is
the only load rule this file states rather than delegating to PyNite (which carries member loads on
`Member3D` only, never on `Spring3D`).
"""

# region 🔖️Imports
import json
import math

import numpy as np
from Pynite.FEModel3D import FEModel3D
from Pynite.PhysMember import PhysMember

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Members
class LiteralMember(PhysMember):
    """🔩️ A PyNite member that stays the one member the document declares.

    PyNite's `add_member` builds a PHYSICAL member, which subdivides itself at every model node that
    happens to lie on its axis and treats each piece as a separate element. This artifact's element
    list is literal: a node an element does not name is not attached to it, and
    `crate::model::build_dof_map` gives such a node no degrees of freedom at all. Subdivision would
    therefore invent a connection — in the shared steel-frame fixture it turns the spare node `n3`,
    which sits on an upper column, into a mid-height support carrying 8.3 kN that this artifact's own
    model never puts there. The subdivision search runs over `model.nodes`, so it is narrowed to the
    member's own two ends and PyNite's own code then produces exactly one sub-member.
    """

    def descritize(self):
        """✂️ Runs PyNite's own subdivision against this member's two end nodes and nothing else."""
        every_node = self.model.nodes
        self.model.nodes = {self.i_node.name: self.i_node, self.j_node.name: self.j_node}
        try:
            super().descritize()
        finally:
            self.model.nodes = every_node


# endregion 🔖️Members


# region 🔖️Constants
GRAVITY = 9.81
"""🌍️ Standard gravity, in m/s². `crate::fem3d_engine::fem3d_solve_all` fixes gravity at
`[0.0, 0.0, -9.81]`; the same number is written down here so the reference is not free to drift."""

DOFS = ("Tx", "Ty", "Tz", "Rx", "Ry", "Rz")
"""🔢️ The six degrees of freedom in this artifact's own index order."""

LINE_DOFS = {"bar": ("Tx", "Ty", "Tz"), "frame": DOFS}
"""🔢️ Which degrees of freedom each element kind activates at the nodes it touches."""

PYNITE_REACTION = {"Tx": "RxnFX", "Ty": "RxnFY", "Tz": "RxnFZ", "Rx": "RxnMX", "Ry": "RxnMY", "Rz": "RxnMZ"}
"""🔁️ Reaction accessor per degree of freedom, named in PyNite's own frame (mapped, not renamed)."""

TOLERANCE = 1e-6
"""📏️ Relative agreement demanded of a closed form this file checks in role, before projecting."""

# endregion 🔖️Constants


# region 🔖️Frames
def to_pynite(vector):
    """🧭️ Maps a Z-up vector into PyNite's Y-up frame: `(X, Y, Z) = (x, z, −y)`, determinant `+1`."""
    return np.array([vector[0], vector[2], -vector[1]], dtype=float)


def from_pynite(vector):
    """🧭️ The inverse map, `(x, y, z) = (X, −Z, Y)`, for displacement/rotation/force/moment vectors."""
    return np.array([vector[0], -vector[2], vector[1]], dtype=float)


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


def pynite_base_triad(start, end):
    """🧭️ PyNite's own unrolled triad, transcribed from `Member3D.T()` so the roll angle this file
    hands it can be solved for rather than guessed."""
    xi, xj = to_pynite(start), to_pynite(end)
    x = unit(xj - xi)
    if math.isclose(xi[0], xj[0]) and math.isclose(xi[2], xj[2]):
        y = np.array([-1.0, 0.0, 0.0]) if xj[1] > xi[1] else np.array([1.0, 0.0, 0.0])
        z = np.array([0.0, 0.0, 1.0])
    elif math.isclose(xi[1], xj[1]):
        y = np.array([0.0, 1.0, 0.0])
        z = unit(np.cross(x, y))
    else:
        projection = np.array([xj[0] - xi[0], 0.0, xj[2] - xi[2]])
        z = unit(np.cross(projection, x) if xj[1] > xi[1] else np.cross(x, projection))
        y = unit(np.cross(z, x))
    return x, y, z


def rotation_degrees(start, end, roll):
    """🌀️ The PyNite `rotation` that makes its triad coincide with this artifact's, so `Iy` and `Iz`
    mean the same axes on both sides. PyNite rolls with Rodrigues about local x, i.e.
    `y(θ) = y₀cosθ + z₀sinθ`, so the angle is read straight off the target's components."""
    _cx, ly, _lz = artifact_triad(start, end, roll)
    _x0, y0, z0 = pynite_base_triad(start, end)
    target = to_pynite(ly)
    return math.degrees(math.atan2(float(np.dot(target, z0)), float(np.dot(target, y0))))


def assert_triads_agree(model, document):
    """🔍️ Checks, per member, that PyNite's own `T()` really did land on this artifact's triad.

    This is the load-bearing check of the whole coordinate argument: if it holds, `Iy`/`Iz`, the two
    bending planes and every moment sign are the same object in both solvers.
    """
    positions = {n["id"]: (n["x"], n["y"], n["z"]) for n in document["nodes"]}
    for element in document["elements"]:
        if element["kind"] != "frame":
            continue
        start, end = positions[element["start"]], positions[element["end"]]
        expected = [to_pynite(axis) for axis in artifact_triad(start, end, element.get("roll", 0.0))]
        produced = np.asarray(model.members[element["id"]].T())[0:3, 0:3]
        for axis, row in zip(expected, produced):
            if float(np.linalg.norm(axis - row)) > 1e-12:
                raise AssertionError("member %s: PyNite's local axis %s is not this artifact's %s" % (element["id"], row.tolist(), axis.tolist()))


# endregion 🔖️Frames


# region 🔖️Document
def line_element_projection(document):
    """✂️ The model's line-element sub-document: `solids` emptied, every `area` load dropped.

    Stated once, applied identically by both implementations, and named in every scenario that uses
    it. A frame solver has no tetrahedron; the solid path is judged by `skfem-fem3d-solid` instead.
    """
    projected = json.loads(json.dumps(document))
    projected["solids"] = []
    for case in projected["loadCases"]:
        case["loads"] = [load for load in case["loads"] if load["kind"] != "area"]
    return projected


def active_dofs(document):
    """🔢️ Per node, the union of the degrees of freedom of the elements incident on it — this
    artifact's own `build_dof_map` rule, restated from the element definitions."""
    active = {node["id"]: set() for node in document["nodes"]}
    for element in document["elements"]:
        for node_id in (element["start"], element["end"]):
            active[node_id].update(LINE_DOFS[element["kind"]])
    return active


def combination_terms(combination):
    """📦️ A combination's `caseId → factor` terms, accepting both committed encodings of a map:
    the object form the shared frame fixtures carry and the `[{caseId, factor}]` list form the
    specification vectors carry."""
    terms = combination["terms"]
    if isinstance(terms, dict):
        return {str(key): float(value) for key, value in terms.items()}
    return {str(term["caseId"]): float(term["factor"]) for term in terms}


def by_id(records):
    """🗂️ An id-keyed view of one of the document's collections."""
    return {record["id"]: record for record in records}


# endregion 🔖️Document


# region 🔖️PyNite
def build_pynite(document):
    """🏗️ Builds the PyNite model of one line-element document, in PyNite's own Y-up frame.

    Frames become `Member3D`s with the roll that aligns the two solvers' local axes; bars become
    `Spring3D`s of stiffness `EA/L`; every degree of freedom a node does not activate is restrained,
    which is how PyNite is told the same thing this artifact's degree-of-freedom map already knows.
    """
    model = FEModel3D()
    positions = {}
    for node in document["nodes"]:
        point = to_pynite((node["x"], node["y"], node["z"]))
        model.add_node(node["id"], float(point[0]), float(point[1]), float(point[2]))
        positions[node["id"]] = (node["x"], node["y"], node["z"])
    for material in document["materials"]:
        model.add_material(material["id"], material["e"], material["g"], material["nu"], material["rho"])
    for section in document["sections"]:
        model.add_section(section["id"], section["area"], section["iy"], section["iz"], section["j"])

    materials, sections = by_id(document["materials"]), by_id(document["sections"])
    springs = {}
    for element in document["elements"]:
        start, end = positions[element["start"]], positions[element["end"]]
        if element["kind"] == "frame":
            roll = rotation_degrees(start, end, element.get("roll", 0.0))
            model.add_member(element["id"], element["start"], element["end"], element["materialId"], element["sectionId"], rotation=roll)
            model.members[element["id"]] = LiteralMember(model, element["id"], model.nodes[element["start"]], model.nodes[element["end"]], element["materialId"], element["sectionId"], rotation=roll)
        else:
            length = float(np.linalg.norm(np.asarray(end) - np.asarray(start)))
            section, material = sections[element["sectionId"]], materials[element["materialId"]]
            model.add_spring(element["id"], element["start"], element["end"], material["e"] * section["area"] / length)
            springs[element["id"]] = (length, material, section)

    active = active_dofs(document)
    fixities = {node["id"]: set(DOFS) - active[node["id"]] for node in document["nodes"]}
    for support in document["supports"]:
        fixities[support["nodeId"]].update(support["fixed"])
    for node_id, fixed in fixities.items():
        model.def_support(node_id, "Tx" in fixed, "Tz" in fixed, "Ty" in fixed, "Rx" in fixed, "Rz" in fixed, "Ry" in fixed)
    assert_triads_agree(model, document)
    return model, springs, positions


def apply_loads(model, document, springs, positions):
    """🏋️ Puts every load case on the PyNite model under its own case name.

    A nodal load is a force or moment vector mapped through `(X, Y, Z) = (x, z, −y)`; a member UDL is
    the same map applied to `(wx, wy, wz)` and handed to PyNite in GLOBAL directions, which is how
    this artifact states it; self weight is `ρ·A·g` downward. Bars — PyNite springs — carry neither,
    so their share arrives as `wL/2` at each end.
    """
    axis_names = ("FX", "FY", "FZ")
    moment_names = ("MX", "MY", "MZ")
    for case in document["loadCases"]:
        name = case["id"]
        for load in case["loads"]:
            if load["kind"] == "nodal":
                index = DOFS.index(load["dof"])
                vector = np.zeros(3)
                vector[index % 3] = load["value"]
                mapped = to_pynite(vector)
                names = moment_names if index >= 3 else axis_names
                for component, direction in zip(mapped, names):
                    if component != 0.0:
                        model.add_node_load(load["nodeId"], direction, float(component), case=name)
            elif load["kind"] == "memberUdl":
                mapped = to_pynite((load["wx"], load["wy"], load["wz"]))
                if load["elementId"] in springs:
                    length, _material, _section = springs[load["elementId"]]
                    element = next(e for e in document["elements"] if e["id"] == load["elementId"])
                    for node_id in (element["start"], element["end"]):
                        for component, direction in zip(mapped, axis_names):
                            if component != 0.0:
                                model.add_node_load(node_id, direction, float(component) * length / 2.0, case=name)
                else:
                    for component, direction in zip(mapped, axis_names):
                        if component != 0.0:
                            model.add_member_dist_load(load["elementId"], direction, float(component), float(component), case=name)
        if case["selfWeight"]:
            model.add_member_self_weight("FY", -GRAVITY, case=name)
            for element_id, (length, material, section) in springs.items():
                element = next(e for e in document["elements"] if e["id"] == element_id)
                weight = material["rho"] * section["area"] * length * GRAVITY / 2.0
                for node_id in (element["start"], element["end"]):
                    model.add_node_load(node_id, "FY", -weight, case=name)
        model.add_load_combo(name, {name: 1.0})
    for combination in document["combinations"]:
        model.add_load_combo(combination["id"], combination_terms(combination))


def solved_results(model, document, springs, positions):
    """📊️ Reads every combination's answer back out of PyNite and into this artifact's frame.

    Displacements carry all six components per node in this artifact's own index order; reactions are
    reported only where a degree of freedom is BOTH active and restrained, which is the exact set
    `crate::model::solve_linear_static` reports; a bar's axial force is `EA/L` times the axial
    stretch its two end displacements imply, tension positive.
    """
    active = active_dofs(document)
    fixed = {node["id"]: set() for node in document["nodes"]}
    for support in document["supports"]:
        fixed[support["nodeId"]].update(support["fixed"])
    names = [combination for combination in model.load_combos]
    answers = {}
    for combination in names:
        displacements, reactions, axial = {}, {}, {}
        for node in document["nodes"]:
            handle = model.nodes[node["id"]]
            translation = from_pynite((handle.DX[combination], handle.DY[combination], handle.DZ[combination]))
            rotation = from_pynite((handle.RX[combination], handle.RY[combination], handle.RZ[combination]))
            displacements[node["id"]] = [float(v) for v in list(translation) + list(rotation)]
            force = from_pynite((handle.RxnFX[combination], handle.RxnFY[combination], handle.RxnFZ[combination]))
            moment = from_pynite((handle.RxnMX[combination], handle.RxnMY[combination], handle.RxnMZ[combination]))
            values = list(force) + list(moment)
            for index, dof in enumerate(DOFS):
                if dof in active[node["id"]] and dof in fixed[node["id"]]:
                    reactions["%s|%s" % (node["id"], dof)] = float(values[index])
        for element in document["elements"]:
            if element["kind"] != "bar":
                continue
            length, material, section = springs[element["id"]]
            axis = unit(np.asarray(positions[element["end"]]) - np.asarray(positions[element["start"]]))
            stretch = np.asarray(displacements[element["end"]][0:3]) - np.asarray(displacements[element["start"]][0:3])
            axial[element["id"]] = float(material["e"] * section["area"] / length * float(np.dot(stretch, axis)))
        answers[combination] = {"displacements": displacements, "reactions": reactions, "axial": axial}
    return answers


def applied_resultant(document, case_id):
    """⚖️ The resultant force a load case puts on the structure, in this artifact's own frame.

    Independent of any solver: nodal forces as stated, a member UDL as `w·L`, self weight as
    `ρ·A·L·g` downward over every line element. It exists so [`solve_document`] can check global
    equilibrium — `Σ reactions + Σ applied = 0` — before any answer is projected, which is the one
    check that catches a mis-mapped load direction without a closed form to compare against.
    """
    positions = {n["id"]: np.array([n["x"], n["y"], n["z"]]) for n in document["nodes"]}
    materials, sections = by_id(document["materials"]), by_id(document["sections"])
    lengths = {e["id"]: float(np.linalg.norm(positions[e["end"]] - positions[e["start"]])) for e in document["elements"]}
    case = next(record for record in document["loadCases"] if record["id"] == case_id)
    total = np.zeros(3)
    for load in case["loads"]:
        if load["kind"] == "nodal" and DOFS.index(load["dof"]) < 3:
            total[DOFS.index(load["dof"])] += load["value"]
        elif load["kind"] == "memberUdl":
            total += np.array([load["wx"], load["wy"], load["wz"]]) * lengths[load["elementId"]]
    if case["selfWeight"]:
        for element in document["elements"]:
            weight = materials[element["materialId"]]["rho"] * sections[element["sectionId"]]["area"] * lengths[element["id"]] * GRAVITY
            total[2] -= weight
    return total


def assert_equilibrium(document, answers):
    """⚖️ Every case's and every combination's reaction resultant must cancel its applied resultant."""
    cases = {record["id"] for record in document["loadCases"]}
    resultants = {case_id: applied_resultant(document, case_id) for case_id in cases}
    for combination in document["combinations"]:
        resultants[combination["id"]] = sum((factor * resultants[case_id] for case_id, factor in combination_terms(combination).items()), np.zeros(3))
    for name, answer in answers.items():
        reaction = np.zeros(3)
        for key, value in answer["reactions"].items():
            dof = key.split("|")[1]
            if DOFS.index(dof) < 3:
                reaction[DOFS.index(dof)] += value
        residual = reaction + resultants[name]
        scale = max(float(np.max(np.abs(resultants[name]))), 1.0)
        if float(np.max(np.abs(residual))) / scale > 1e-9:
            raise AssertionError("%s: reactions %s do not carry the applied %s" % (name, reaction.tolist(), (-resultants[name]).tolist()))


def solve_document(document):
    """🚀️ The whole PyNite pass over one line-element document, keyed by case id ∪ combination id."""
    projected = line_element_projection(document)
    model, springs, positions = build_pynite(projected)
    apply_loads(model, projected, springs, positions)
    model.analyze_linear(check_stability=True, sparse=False)
    answers = solved_results(model, projected, springs, positions)
    assert_equilibrium(projected, answers)
    return projected, answers


# endregion 🔖️PyNite



# region 🔖️Plan
def doc_string(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


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


def significant(value, digits=6):
    """🔢️ One number at `digits` significant figures, as the normalized decimal TEXT this case's
    comparison profile compares. Text, not a float: the two implementations must agree to six
    figures, and a float projection would compare their last bits instead, which no two solvers
    share. Six figures is a millionfold margin over the `1e-12`-scale disagreement two direct
    double-precision factorisations of one linear system actually show."""
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


def significant_relative(value, scale, digits=6):
    """🔢️ The same text, with a component that is zero only up to round-off snapped to zero.

    A resultant that cancels by symmetry lands on `-2.5e-11` in one solver and `+8.1e-12` in another;
    six significant figures OF ROUND-OFF is noise, not agreement. Anything a billionth of its own
    group's largest component is therefore reported as the zero it is.
    """
    return "0.00000e+0" if abs(value) <= 1e-9 * scale else significant(value, digits)


def reduction(answers):
    """📐️ The compared projection: each case's answer as four scale-carrying scalars at six
    significant figures. The FULL field is not compared here — two solvers' last bits never agree —
    it is asserted, with the tolerances the feature states, by the subject against this case's
    committed `📊️expected.results.json`, which this same reference produced."""
    reduced = {}
    for name, answer in answers.items():
        values = [value for node in sorted(answer["displacements"]) for value in answer["displacements"][node]]
        translations = [abs(value) for node in sorted(answer["displacements"]) for value in answer["displacements"][node][0:3]]
        rotations = [abs(value) for node in sorted(answer["displacements"]) for value in answer["displacements"][node][3:6]]
        force = [0.0, 0.0, 0.0]
        for key, value in answer["reactions"].items():
            index = DOFS.index(key.split("|")[1])
            if index < 3:
                force[index] += value
        scale = max((abs(component) for component in force), default=0.0)
        reduced[name] = {
            "displacementNorm": significant(float(np.linalg.norm(values))),
            "maxTranslation": significant(max(translations, default=0.0)),
            "maxRotation": significant(max(rotations, default=0.0)),
            "reactionForceSum": [significant_relative(component, scale) for component in force],
            "maxAxial": significant(max((abs(value) for value in answer["axial"].values()), default=0.0)),
        }
    return reduced


def outcome_of(projection, reference):
    """📤️ Hands over the compared projection, carrying the FULL answer as the raw artifact — which is
    what `🔨️run-fem3d-oracle.py` commits as `📊️expected.results.json` for the subject to assert
    against."""
    return Outcome(projection, raw=json.dumps(reference, separators=(",", ":"), ensure_ascii=False, sort_keys=True).encode("utf-8"))


def agrees(label, produced, expected, tolerance=TOLERANCE):
    """📏️ Asserts a closed form in role, before the projection is handed over."""
    scale = max(abs(expected), 1e-30)
    if abs(produced - expected) / scale > tolerance:
        raise AssertionError("%s: the reference solved %.12g where the closed form is %.12g (relative %.3g)" % (label, produced, expected, abs(produced - expected) / scale))


# endregion 🔖️Plan


# region 🔖️Handlers
def static_handler(needle):
    """📊️ Solves one committed real-world model with PyNite, every case and every combination."""

    def handler(ctx):
        document = json_fixture(ctx, needle)
        _projected, answers = solve_document(document)
        return outcome_of({"model": needle, "cases": reduction(answers)}, {"model": needle, "cases": answers})

    return handler


def cantilever_handler(needle, axis):
    """📏️ A tip-loaded cantilever, held to `PL³/3EI`, `PL²/2EI` and `PL` before it is projected."""

    def handler(ctx):
        document = json_fixture(ctx, needle)
        _projected, answers = solve_document(document)
        section = document["sections"][0]
        material = document["materials"][0]
        length = document["nodes"][1]["x"]
        load = document["loadCases"][0]["loads"][0]["value"]
        inertia = section["iy"] if axis == "Tz" else section["iz"]
        tip = answers["tip"]["displacements"]["tip"]
        agrees("tip deflection", tip[DOFS.index(axis)], load * length**3 / (3.0 * material["e"] * inertia), 2e-9)
        moment_dof = "Ry" if axis == "Tz" else "Rz"
        agrees("base moment reaction", abs(answers["tip"]["reactions"]["root|" + moment_dof]), abs(load) * length, 1e-9)
        agrees("base shear reaction", answers["tip"]["reactions"]["root|" + axis], -load, 1e-9)
        return outcome_of({"model": needle, "cases": reduction(answers)}, {"model": needle, "cases": answers})

    return handler


def torsion_handler(needle):
    """🌀️ A tip-twisted cantilever, held to `TL/GJ` before it is projected."""

    def handler(ctx):
        document = json_fixture(ctx, needle)
        _projected, answers = solve_document(document)
        section, material = document["sections"][0], document["materials"][0]
        length = document["nodes"][1]["x"]
        torque = document["loadCases"][0]["loads"][0]["value"]
        agrees("tip twist", answers["tip"]["displacements"]["tip"][DOFS.index("Rx")], torque * length / (material["g"] * section["j"]), 1e-9)
        agrees("base torque reaction", answers["tip"]["reactions"]["root|Rx"], -torque, 1e-9)
        return outcome_of({"model": needle, "cases": reduction(answers)}, {"model": needle, "cases": answers})

    return handler


def simply_supported_handler(needle):
    """🌉️ A simply supported beam under a UDL, held to `5wL⁴/384EI` and `wL/2` before projection."""

    def handler(ctx):
        document = json_fixture(ctx, needle)
        _projected, answers = solve_document(document)
        section, material = document["sections"][0], document["materials"][0]
        length = document["nodes"][2]["x"]
        w = document["loadCases"][0]["loads"][0]["wz"]
        agrees("midspan deflection", answers["udl"]["displacements"]["mid"][DOFS.index("Tz")], 5.0 * w * length**4 / (384.0 * material["e"] * section["iy"]), 2e-3)
        agrees("left reaction", answers["udl"]["reactions"]["left|Tz"], -w * length / 2.0, 1e-9)
        agrees("right reaction", answers["udl"]["reactions"]["right|Tz"], -w * length / 2.0, 1e-9)
        return outcome_of({"model": needle, "cases": reduction(answers)}, {"model": needle, "cases": answers})

    return handler


def mutate_solve_handler(kind):
    """🦠️ Solves one kind's committed `(base, after)` pair and reports how the answer moved.

    The verdict is the AFTER model's analysis — an independent solver's reading of the document the
    mutation produced. This side never applies the mutation: the subject reaches its after-model
    through production dispatch and is held to the committed one, and the reference simply solves the
    committed one. The base solve is what makes the mechanism visible: a kind that changes the
    structure moves the answer, a kind that edits an unreferenced record or a display setting does
    not, a kind that adds or drops a load case makes a whole column of answers appear or disappear —
    and both implementations must say the same thing about which of those happened.
    """

    def handler(ctx):
        base = json_fixture(ctx, "mutation-base")
        after = json_fixture(ctx, "mutation-after-" + kind)
        _pb, before_answers = solve_document(base)
        _pa, after_answers = solve_document(after)
        return outcome_of({"kind": kind, "after": reduction(after_answers), "mechanism": mechanism_of(before_answers, after_answers)}, {"kind": kind, "after": after_answers, "mechanism": mechanism_of(before_answers, after_answers)})

    return handler


def mechanism_of(before, after):
    """🔬️ How the answer moved: which combinations changed, and by how much, relatively."""
    moved = {}
    for name in sorted(set(before) | set(after)):
        if name not in before or name not in after:
            moved[name] = "appeared" if name not in before else "disappeared"
            continue
        left = np.array([value for node in sorted(before[name]["displacements"]) for value in before[name]["displacements"][node]])
        right = np.array([value for node in sorted(after[name]["displacements"]) for value in after[name]["displacements"][node]])
        if left.shape != right.shape:
            moved[name] = "reshaped"
            continue
        scale = max(float(np.max(np.abs(left))), float(np.max(np.abs(right))), 1e-30)
        moved[name] = "unchanged" if float(np.max(np.abs(left - right))) / scale < 1e-9 else "changed"
    return moved


# endregion 🔖️Handlers


# region 🔖️Registration
MODELS = {
    "steel-frame": "🧊️steel-frame.snapshot.json",
    "space-frame": "🏢️space-frame-2x2-bay.snapshot.json",
    "roof-truss": "🏛️timber-roof-truss.snapshot.json",
}
"""🏗️ The real-world models this case solves whole, by the needle its scenarios name them with."""

KINDS = (
    "create-node",
    "delete-node",
    "create-element",
    "delete-element",
    "replace-element",
    "create-material",
    "delete-material",
    "replace-material",
    "create-section",
    "delete-section",
    "replace-section",
    "create-support",
    "delete-support",
    "replace-support",
    "create-load-case",
    "delete-load-case",
    "add-load",
    "remove-load",
    "change-load-case-self-weight",
    "create-combination",
    "delete-combination",
    "update-analysis-settings",
)
"""🏷️ Every kind of this artifact's vocabulary except `create-`/`delete-`/`replace-solid`, whose
subject is geometry and whose oracles are `three-fem3d-mesh-reader` and
`manifold-fem3d-mesh-measure` — the twenty-two this file's solver can genuinely adjudicate."""


def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only."""
    built = Adapter("python")
    for needle in MODELS:
        built = built.oracle("static-%s" % needle, static_handler(needle))
    built = built.oracle("closed-form-cantilever-local-z", cantilever_handler("cantilever-tip-load-local-z", "Tz"))
    built = built.oracle("closed-form-cantilever-local-y", cantilever_handler("cantilever-tip-load-local-y", "Ty"))
    built = built.oracle("closed-form-cantilever-torsion", torsion_handler("cantilever-tip-torsion"))
    built = built.oracle("closed-form-simply-supported-udl", simply_supported_handler("simply-supported-udl"))
    for kind in KINDS:
        built = built.oracle("mutate-solve-%s" % kind, mutate_solve_handler(kind))
    return built


# endregion 🔖️Registration
