#!/usr/bin/env python3
"""🏗️ Authors the committed `s.fem.fem3d` analysis-benchmark snapshot fixtures.

Every fixture this writes is a hand-designed structural model expressed in the artifact's own
snapshot schema (`🌐️any/🧬️schema/📸️snapshot/🔣️.json` — nine members, `additionalProperties: false`).
The script exists because two of the models carry more than forty members each and a hand-typed
JSON of that size is a transcription hazard, not because the models are generated: every dimension,
section, material and load below is stated here literally, once.

Run: `uv run python .🧬semio/…/FEM-PLUGIN-END-TO-END/🔨️author-fem3d-benchmark-fixtures.py`
"""

# region 🔖️Imports
import json
import os

# endregion 🔖️Imports


# region 🔖️Paths
REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
TESTS = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets", "📈️analysis", "🧪️tests")
BENCHMARKS, EIGEN, SOLID = "🧮️solves-fem3d-1-benchmarks", "🎵️solves-fem3d-1-eigen", "🧱️solves-fem3d-1-solid"

STEEL_S355 = {"id": "steel", "name": "Steel S355", "e": 210000000000.0, "g": 80769230769.0, "nu": 0.3, "rho": 7850.0}
TIMBER_C24 = {"id": "timber", "name": "C24 Timber", "e": 11000000000.0, "g": 690000000.0, "nu": 0.3, "rho": 420.0}
CONCRETE_C30 = {"id": "concrete", "name": "C30/37 Concrete", "e": 33000000000.0, "g": 13750000000.0, "nu": 0.2, "rho": 2400.0}

HEB240 = {"id": "heb240", "name": "HEB 240", "area": 0.0106, "iy": 0.00011259, "iz": 3.923e-05, "j": 1.027e-06}
IPE400 = {"id": "ipe400", "name": "IPE 400", "area": 0.00845, "iy": 0.00023128, "iz": 1.318e-05, "j": 5.11e-07}
IPE330 = {"id": "ipe330", "name": "IPE 330", "area": 0.00626, "iy": 0.00011767, "iz": 7.88e-06, "j": 2.815e-07}
TIMBER_100X200 = {"id": "tim100x200", "name": "Timber 100x200", "area": 0.02, "iy": 6.6667e-05, "iz": 1.6667e-05, "j": 4.5776e-05}
HEA200 = {"id": "hea200", "name": "HEA 200", "area": 0.00538, "iy": 3.692e-05, "iz": 1.336e-05, "j": 2.098e-07}
SHS150 = {"id": "shs150", "name": "SHS 150x150x8", "area": 0.004416, "iy": 1.4595e-05, "iz": 9.7301e-06, "j": 2.2646e-05}

DOF_ALL = ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"]


def frame(eid, start, end, material, section, roll=0.0):
    """🔩️ One 6-DOF frame member record."""
    return {"kind": "frame", "id": eid, "start": start, "end": end, "materialId": material, "sectionId": section, "roll": roll}


def bar(eid, start, end, material, section):
    """🔩️ One axial-only bar member record."""
    return {"kind": "bar", "id": eid, "start": start, "end": end, "materialId": material, "sectionId": section}


def node(nid, x, y, z):
    """📍️ One node record."""
    return {"id": nid, "x": float(x), "y": float(y), "z": float(z)}


def support(sid, node_id, fixed):
    """🔒️ One support record."""
    return {"id": sid, "nodeId": node_id, "fixed": list(fixed)}


def nodal(lid, node_id, dof, value):
    """🏋️ One concentrated nodal load."""
    return {"kind": "nodal", "id": lid, "nodeId": node_id, "dof": dof, "value": float(value)}


def udl(lid, element_id, wx, wy, wz):
    """🌬️ One member uniformly distributed load, global components."""
    return {"kind": "memberUdl", "id": lid, "elementId": element_id, "wx": float(wx), "wy": float(wy), "wz": float(wz)}


def area(lid, solid_id, pressure):
    """🌬️ One uniform pressure over a solid's top face."""
    return {"kind": "area", "id": lid, "solidId": solid_id, "pressure": float(pressure)}


def case(cid, name, loads, self_weight=False):
    """📦️ One load case."""
    return {"id": cid, "name": name, "loads": loads, "selfWeight": self_weight}


def snapshot(nodes, elements, materials, sections, solids, supports, cases, combinations, modal=4, buckling=3, scale=100.0):
    """📸️ The nine-member document, in the schema's own member order."""
    return {
        "nodes": nodes,
        "elements": elements,
        "materials": materials,
        "sections": sections,
        "solids": solids,
        "supports": supports,
        "loadCases": cases,
        "combinations": combinations,
        "analysis": {"modalCount": modal, "bucklingCount": buckling, "deformationScale": scale},
    }


def write(case, name, document):
    """💾️ Writes one fixture into its own case, sorted-key and newline-terminated like every sibling."""
    directory = os.path.join(TESTS, case, "🧫️fixtures")
    os.makedirs(directory, exist_ok=True)
    path = os.path.join(directory, name)
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(document, handle, indent=2, ensure_ascii=False, sort_keys=True)
        handle.write("\n")
    print("wrote %s/%s (%d bytes)" % (case, name, os.path.getsize(path)))


# endregion 🔖️Paths


# region 🔖️SpaceFrame
def space_frame():
    """🏢️ A 2 × 2-bay, 2-storey steel space frame on a 6 m × 5 m grid, storeys at 3.5 m.

    Nine column lines (x ∈ {0, 6, 12}, y ∈ {0, 5, 10}) rise through two storeys; every storey
    carries an orthogonal beam grid — IPE 400 spanning x, IPE 330 spanning y — over HEB 240 columns.
    Loads are the three a building actually sees: floor dead (self-weight plus a 12 kN/m beam UDL),
    floor live (8 kN/m on the first storey only) and a wind pressure resultant pushed into the
    windward frame line as storey forces.
    """
    xs, ys, zs = [0.0, 6.0, 12.0], [0.0, 5.0, 10.0], [0.0, 3.5, 7.0]
    nodes, elements, supports, dead, live, wind = [], [], [], [], [], []
    for i, x in enumerate(xs):
        for j, y in enumerate(ys):
            for k, z in enumerate(zs):
                nodes.append(node("n%d%d%d" % (i, j, k), x, y, z))
            supports.append(support("sup%d%d" % (i, j), "n%d%d0" % (i, j), DOF_ALL))
            for k in (0, 1):
                elements.append(frame("col%d%d%d" % (i, j, k), "n%d%d%d" % (i, j, k), "n%d%d%d" % (i, j, k + 1), "steel", "heb240"))
    for k in (1, 2):
        for j in range(3):
            for i in (0, 1):
                eid = "bx%d%d%d" % (i, j, k)
                elements.append(frame(eid, "n%d%d%d" % (i, j, k), "n%d%d%d" % (i + 1, j, k), "steel", "ipe400"))
                dead.append(udl("d" + eid, eid, 0.0, 0.0, -12000.0))
                if k == 1:
                    live.append(udl("l" + eid, eid, 0.0, 0.0, -8000.0))
        for i in range(3):
            for j in (0, 1):
                eid = "by%d%d%d" % (i, j, k)
                elements.append(frame(eid, "n%d%d%d" % (i, j, k), "n%d%d%d" % (i, j + 1, k), "steel", "ipe330"))
                dead.append(udl("d" + eid, eid, 0.0, 0.0, -12000.0))
                if k == 1:
                    live.append(udl("l" + eid, eid, 0.0, 0.0, -8000.0))
    for i in range(3):
        wind.append(nodal("w%d1" % i, "n%d01" % i, "Ty", 15000.0))
        wind.append(nodal("w%d2" % i, "n%d02" % i, "Ty", 22000.0))
    combinations = [
        {"id": "uls", "name": "ULS", "terms": {"dead": 1.35, "live": 1.5, "wind": 0.9}},
        {"id": "sls", "name": "SLS characteristic", "terms": {"dead": 1.0, "live": 1.0}},
    ]
    cases = [case("dead", "Dead Load", dead, self_weight=True), case("live", "Live Load", live), case("wind", "Wind Y", wind)]
    return snapshot(nodes, elements, [STEEL_S355], [HEB240, IPE400, IPE330], [], supports, cases, combinations, modal=6, buckling=4, scale=200.0)


# endregion 🔖️SpaceFrame


# region 🔖️RoofTruss
def roof_truss():
    """🏛️ A 12 m timber king-post roof truss of pin-jointed C24 members, braced out of plane.

    Bottom chord at 3 m panel points, rafters rising to a 2.4 m ridge, three verticals and two
    diagonals — thirteen axial members. Every node is held in `Ty` (the roof plane's bracing), the
    left shoe is pinned and the right shoe is a roller, so the model is a planar truss carried in a
    3D document. Loads are panel-point snow and a permanent roof build-up; both are stated as node
    forces, which is how a truss is actually analysed and what keeps this fixture free of any load
    distribution rule.
    """
    coords = {"b0": (0.0, 0.0, 0.0), "b1": (3.0, 0.0, 0.0), "b2": (6.0, 0.0, 0.0), "b3": (9.0, 0.0, 0.0), "b4": (12.0, 0.0, 0.0), "t1": (3.0, 0.0, 1.2), "t2": (6.0, 0.0, 2.4), "t3": (9.0, 0.0, 1.2)}
    nodes = [node(nid, *coords[nid]) for nid in ("b0", "b1", "b2", "b3", "b4", "t1", "t2", "t3")]
    pairs = [("bc0", "b0", "b1"), ("bc1", "b1", "b2"), ("bc2", "b2", "b3"), ("bc3", "b3", "b4"), ("rf0", "b0", "t1"), ("rf1", "t1", "t2"), ("rf2", "t2", "t3"), ("rf3", "t3", "b4"), ("vt1", "b1", "t1"), ("vt2", "b2", "t2"), ("vt3", "b3", "t3"), ("dg1", "b1", "t2"), ("dg2", "b3", "t2")]
    elements = [bar(eid, a, b, "timber", "tim100x200") for eid, a, b in pairs]
    supports = [support("shoe_l", "b0", ["Tx", "Ty", "Tz"]), support("shoe_r", "b4", ["Ty", "Tz"])]
    supports += [support("brace_" + nid, nid, ["Ty"]) for nid in ("b1", "b2", "b3", "t1", "t2", "t3")]
    snow = [nodal("s1", "t1", "Tz", -9000.0), nodal("s2", "t2", "Tz", -12000.0), nodal("s3", "t3", "Tz", -9000.0), nodal("s0", "b0", "Tz", -4500.0), nodal("s4", "b4", "Tz", -4500.0)]
    roof = [nodal("r1", "t1", "Tz", -3600.0), nodal("r2", "t2", "Tz", -4800.0), nodal("r3", "t3", "Tz", -3600.0)]
    combinations = [{"id": "uls", "name": "ULS", "terms": {"roof": 1.35, "snow": 1.5}}]
    cases = [case("roof", "Roof Build-Up", roof), case("snow", "Snow", snow)]
    return snapshot(nodes, elements, [TIMBER_C24], [TIMBER_100X200], [], supports, cases, combinations, modal=3, buckling=3, scale=60.0)


# endregion 🔖️RoofTruss


# region 🔖️ClosedForm
def cantilever(load_dof, value, name):
    """📏️ A 4 m HEA 200 cantilever along global X, fixed at its root, one tip action.

    Its local triad is unambiguous: local x is +X, the reference "up" is +Z (the member is not
    vertical), so local y is +Y and local z is +Z. A tip `Tz` force therefore bends the member about
    its local y axis (`iy`), a tip `Ty` force about its local z axis (`iz`), and a tip `Rx` moment
    twists it about its own axis (`j`).
    """
    nodes = [node("root", 0.0, 0.0, 0.0), node("tip", 4.0, 0.0, 0.0)]
    elements = [frame("member", "root", "tip", "steel", "hea200")]
    supports = [support("fixity", "root", DOF_ALL)]
    cases = [case("tip", name, [nodal("p", "tip", load_dof, value)])]
    return snapshot(nodes, elements, [STEEL_S355], [HEA200], [], supports, cases, [])


def simply_supported_udl():
    """🌉️ An 8 m simply supported HEA 200 beam under a 6 kN/m downward UDL, split at midspan.

    The left shoe holds `Tx`,`Ty`,`Tz`,`Rx`, the right one `Ty`,`Tz`,`Rx`: bending about both
    principal axes is released at both ends, which is what makes `5wL⁴/384EI`, `wL/2` and `wL²/8`
    the exact answers. Two elements put a node at midspan so the deflection there is a nodal value.
    """
    nodes = [node("left", 0.0, 0.0, 0.0), node("mid", 4.0, 0.0, 0.0), node("right", 8.0, 0.0, 0.0)]
    elements = [frame("span_l", "left", "mid", "steel", "hea200"), frame("span_r", "mid", "right", "steel", "hea200")]
    supports = [support("shoe_l", "left", ["Tx", "Ty", "Tz", "Rx"]), support("shoe_r", "right", ["Ty", "Tz", "Rx"])]
    loads = [udl("w_l", "span_l", 0.0, 0.0, -6000.0), udl("w_r", "span_r", 0.0, 0.0, -6000.0)]
    return snapshot(nodes, elements, [STEEL_S355], [HEA200], [], supports, [case("udl", "Uniform Load", loads)], [])


# endregion 🔖️ClosedForm


# region 🔖️Solid
def prismatic_solid_column():
    """🧱️ A 2 m × 2 m × 4 m C30/37 prism, meshed into `Tet4` by the artifact's own solid pipeline.

    `meshSize` is deliberately larger than the footprint diagonal so the footprint triangulates into
    exactly its four corners — every base node is then a DOCUMENT node this fixture can restrain,
    which is what lets the restraint set be the one the uniform-stress solution itself satisfies:
    `Tz` at all four corners, and just enough in-plane fixity (`Tx`+`Ty` at the origin, `Ty` at the
    +x corner, `Tx` at the +y corner) to remove the remaining rigid-body freedoms without fighting
    the Poisson contraction. Under a uniform top pressure the answer is therefore exactly
    `σ = -p`, `δ_top = -pH/E`, which a constant-strain tetrahedron reproduces to machine precision.
    """
    nodes = [node("c00", 0.0, 0.0, 0.0), node("c10", 2.0, 0.0, 0.0), node("c11", 2.0, 2.0, 0.0), node("c01", 0.0, 2.0, 0.0)]
    solids = [{"id": "sol1", "name": "Prismatic Column", "outline": [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]], "holes": [], "baseZ": 0.0, "height": 4.0, "layers": 4, "meshSize": 4.0, "materialId": "concrete"}]
    supports = [support("base_00", "c00", ["Tx", "Ty", "Tz"]), support("base_10", "c10", ["Ty", "Tz"]), support("base_01", "c01", ["Tx", "Tz"]), support("base_11", "c11", ["Tz"])]
    cases = [case("pressure", "Top Pressure", [area("p", "sol1", 500000.0)]), case("self", "Self Weight", [], self_weight=True)]
    return snapshot(nodes, elements=[], materials=[CONCRETE_C30], sections=[], solids=solids, supports=supports, cases=cases, combinations=[])


# endregion 🔖️Solid


# region 🔖️Eigen
def modal_cantilever():
    """🎵️ The same 4 m HEA 200 cantilever, discretised into eight elements for modal analysis.

    Eight elements is enough for the first three Euler-Bernoulli bending frequencies to land within
    a fraction of a percent of `βₙ²/(2πL²)·√(EI/ρA)`. `iy ≠ iz`, so the two bending planes give two
    distinct frequency families and no mode is a double root that an eigensolver could return in
    either order.
    """
    count, length = 8, 4.0
    nodes = [node("n%d" % i, length * i / count, 0.0, 0.0) for i in range(count + 1)]
    elements = [frame("e%d" % i, "n%d" % i, "n%d" % (i + 1), "steel", "hea200") for i in range(count)]
    supports = [support("fixity", "n0", DOF_ALL)]
    cases = [case("idle", "No Load", [])]
    return snapshot(nodes, elements, [STEEL_S355], [HEA200], [], supports, cases, [], modal=4, buckling=1, scale=1.0)


BUCKLING_CASES = {
    "🏛️buckling-column-pinned-pinned.snapshot.json": ("pinned-pinned", 1.0, ["Tx", "Ty", "Tz", "Rz"], ["Tx", "Ty"]),
    "🏛️buckling-column-fixed-pinned.snapshot.json": ("fixed-pinned", 0.7, DOF_ALL, ["Tx", "Ty"]),
    "🏛️buckling-column-fixed-fixed.snapshot.json": ("fixed-fixed", 0.5, DOF_ALL, ["Tx", "Ty", "Rx", "Ry"]),
    "🏛️buckling-column-fixed-free.snapshot.json": ("fixed-free", 2.0, DOF_ALL, []),
}
"""🏛️ The four textbook effective-length cases, as `(name, K, base fixity, head fixity)`."""


def buckling_column(base_fixed, head_fixed):
    """🏛️ A 5 m SHS 150 column along global Z under a unit axial reference load, eight elements.

    The reference load is exactly 1 N so the solver's lowest load factor IS the critical load in
    newtons, directly comparable with `π²EI/(KL)²`. The section's `iz < iy`, so the first mode is the
    weak-axis one and the factor is unambiguous.
    """
    count, height = 8, 5.0
    nodes = [node("n%d" % i, 0.0, 0.0, height * i / count) for i in range(count + 1)]
    elements = [frame("e%d" % i, "n%d" % i, "n%d" % (i + 1), "steel", "shs150") for i in range(count)]
    supports = [support("base", "n0", base_fixed)]
    if head_fixed:
        supports.append(support("head", "n%d" % count, head_fixed))
    cases = [case("axial", "Axial Reference", [nodal("p", "n%d" % count, "Tz", -1.0)])]
    return snapshot(nodes, elements, [STEEL_S355], [SHS150], [], supports, cases, [], modal=2, buckling=2, scale=1.0)


# endregion 🔖️Eigen


# region 🔖️MutationCorpus
def mutation_base():
    """🦠️ The solvable model every mutate-then-solve scenario starts from.

    A single-bay 6 m × 5 m steel portal frame, four fixed feet, a closed eave ring and one diagonal
    bar brace — a model that actually stands up, which the committed specification vectors of the
    mutation subsets do not have to be and are not (their line-element sub-model is a mechanism, so
    no solver can read them). It carries the same spares the mutation corpus relies on: an
    unreferenced material, an unreferenced section, an unattached node, a redundant roller at one
    eave and a load case no combination cites, so every `delete-` and `replace-` verb has an
    unambiguous target and the model still stands afterwards.
    """
    ground = {"c1": (0.0, 0.0, 0.0), "c2": (6.0, 0.0, 0.0), "c3": (6.0, 5.0, 0.0), "c4": (0.0, 5.0, 0.0)}
    eaves = {"h1": (0.0, 0.0, 4.0), "h2": (6.0, 0.0, 4.0), "h3": (6.0, 5.0, 4.0), "h4": (0.0, 5.0, 4.0)}
    nodes = [node(nid, *ground[nid]) for nid in ("c1", "c2", "c3", "c4")]
    nodes += [node(nid, *eaves[nid]) for nid in ("h1", "h2", "h3", "h4")]
    nodes.append(node("spare_node", 10.0, 10.0, 0.0))
    elements = [frame("col_%d" % i, "c%d" % i, "h%d" % i, "steel", "col") for i in (1, 2, 3, 4)]
    elements += [frame("roof_%d" % i, "h%d" % i, "h%d" % (i % 4 + 1), "steel", "beam") for i in (1, 2, 3, 4)]
    elements.append(bar("brace", "c1", "h2", "steel", "beam"))
    supports = [support("foot_%d" % i, "c%d" % i, DOF_ALL) for i in (1, 2, 3, 4)]
    supports.append(support("roller", "h1", ["Tx"]))
    dead = [udl("g_roof_%d" % i, "roof_%d" % i, 0.0, 0.0, -10000.0) for i in (1, 2, 3, 4)]
    live = [nodal("q_h2", "h2", "Tz", -20000.0), nodal("q_h3", "h3", "Tz", -20000.0)]
    wind = [nodal("w_h1", "h1", "Ty", 12000.0), nodal("w_h2", "h2", "Ty", 12000.0)]
    cases = [case("dead", "Dead Load", dead, self_weight=True), case("live", "Live Load", live), case("wind", "Wind Y", wind)]
    combinations = [{"id": "uls", "name": "ULS", "terms": {"dead": 1.35, "live": 1.5}}, {"id": "sls", "name": "SLS characteristic", "terms": {"dead": 1.0, "live": 1.0}}]
    materials = [STEEL_S355, dict(TIMBER_C24, id="spare_material")]
    sections = [dict(HEB240, id="col", name="HEB 240 Column"), dict(IPE400, id="beam", name="IPE 400 Roof Beam"), dict(IPE330, id="spare_section", name="IPE 330 Spare")]
    return snapshot(nodes, elements, materials, sections, [], supports, cases, combinations, modal=4, buckling=3, scale=100.0)


MUTATIONS = {
    "create-node": {"mutation": "createNode", "node": {"id": "added_node", "x": 12.0, "y": 12.0, "z": 0.0}},
    "delete-node": {"mutation": "deleteNode", "id": "spare_node"},
    "create-element": {"mutation": "createElement", "element": {"kind": "bar", "id": "brace2", "start": "c4", "end": "h3", "materialId": "steel", "sectionId": "beam"}},
    "delete-element": {"mutation": "deleteElement", "id": "brace"},
    "replace-element": {"mutation": "replaceElement", "id": "roof_1", "newElement": {"kind": "frame", "id": "roof_1", "start": "h1", "end": "h2", "materialId": "steel", "sectionId": "col", "roll": 0.0}},
    "create-material": {"mutation": "createMaterial", "material": {"id": "alu", "name": "Aluminium EN AW-6082", "e": 70000000000.0, "g": 26000000000.0, "nu": 0.25, "rho": 2700.0}},
    "delete-material": {"mutation": "deleteMaterial", "id": "spare_material"},
    "replace-material": {"mutation": "replaceMaterial", "id": "steel", "newMaterial": {"id": "steel", "name": "Steel S235", "e": 190000000000.0, "g": 73076923077.0, "nu": 0.3, "rho": 7850.0}},
    "create-section": {"mutation": "createSection", "section": {"id": "shs120", "name": "SHS 120x120x6", "area": 0.00266, "iy": 5.56e-05, "iz": 5.56e-05, "j": 8.89e-05}},
    "delete-section": {"mutation": "deleteSection", "id": "spare_section"},
    "replace-section": {"mutation": "replaceSection", "id": "beam", "newSection": {"id": "beam", "name": "IPE 450 Roof Beam", "area": 0.00988, "iy": 0.00033743, "iz": 1.676e-05, "j": 6.69e-07}},
    "create-support": {"mutation": "createSupport", "support": {"id": "eave_stay", "nodeId": "h4", "fixed": ["Tz"]}},
    "delete-support": {"mutation": "deleteSupport", "id": "roller"},
    "replace-support": {"mutation": "replaceSupport", "id": "roller", "newSupport": {"id": "roller", "nodeId": "h1", "fixed": ["Tx", "Ty"]}},
    "create-load-case": {"mutation": "createLoadCase", "loadCase": {"id": "snow", "name": "Snow", "loads": [{"kind": "nodal", "id": "s_h3", "nodeId": "h3", "dof": "Tz", "value": -14000.0}], "selfWeight": False}},
    "delete-load-case": {"mutation": "deleteLoadCase", "id": "wind"},
    "add-load": {"mutation": "addLoad", "caseId": "live", "load": {"kind": "nodal", "id": "q_h4", "nodeId": "h4", "dof": "Tz", "value": -15000.0}},
    "remove-load": {"mutation": "removeLoad", "caseId": "live", "loadId": "q_h3"},
    "change-load-case-self-weight": {"mutation": "changeLoadCaseSelfWeight", "caseId": "live", "newSelfWeight": True},
    "create-combination": {"mutation": "createCombination", "combination": {"id": "als", "name": "Accidental", "terms": {"dead": 1.0, "wind": 1.0}}},
    "delete-combination": {"mutation": "deleteCombination", "id": "sls"},
    "update-analysis-settings": {"mutation": "updateAnalysisSettings", "settings": {"modalCount": 8, "bucklingCount": 5, "deformationScale": 250.0}},
}
"""🦠️ One typed edit per kind, in the vocabulary's own payload grammar — the same grammar the
committed specification vectors are written in, retargeted at this model's own ids."""

COLLECTIONS = {"Node": "nodes", "Element": "elements", "Material": "materials", "Section": "sections", "Support": "supports"}
"""🗂️ Which collection each `create-`/`delete-`/`replace-` noun writes."""


def apply_mutation(document, payload):
    """🦠️ Applies one typed edit, per the vocabulary's own documented semantics: `create-` appends,
    `delete-` removes the record with that id, `replace-` overwrites it in its own slot."""
    after = json.loads(json.dumps(document))
    verb = payload["mutation"]
    for noun, collection in COLLECTIONS.items():
        argument = noun[0].lower() + noun[1:]
        if verb == "create" + noun:
            after[collection].append(payload[argument])
            return after
        if verb == "delete" + noun:
            after[collection] = [record for record in after[collection] if record["id"] != payload["id"]]
            return after
        if verb == "replace" + noun:
            after[collection] = [payload["new" + noun] if record["id"] == payload["id"] else record for record in after[collection]]
            return after
    if verb == "createLoadCase":
        after["loadCases"].append(payload["loadCase"])
    elif verb == "deleteLoadCase":
        after["loadCases"] = [record for record in after["loadCases"] if record["id"] != payload["id"]]
    elif verb == "createCombination":
        after["combinations"].append(payload["combination"])
    elif verb == "deleteCombination":
        after["combinations"] = [record for record in after["combinations"] if record["id"] != payload["id"]]
    elif verb == "addLoad":
        next(record for record in after["loadCases"] if record["id"] == payload["caseId"])["loads"].append(payload["load"])
    elif verb == "removeLoad":
        target = next(record for record in after["loadCases"] if record["id"] == payload["caseId"])
        target["loads"] = [load for load in target["loads"] if load["id"] != payload["loadId"]]
    elif verb == "changeLoadCaseSelfWeight":
        next(record for record in after["loadCases"] if record["id"] == payload["caseId"])["selfWeight"] = payload["newSelfWeight"]
    elif verb == "updateAnalysisSettings":
        after["analysis"] = payload["settings"]
    else:
        raise AssertionError("no semantics are stated for the verb %r" % verb)
    return after


def write_mutation_corpus():
    """🦠️ Writes the base model, one after-model per kind, and the payload table the feature states."""
    base = mutation_base()
    write(BENCHMARKS, "🦠️mutation-base.snapshot.json", base)
    table = {}
    for kind, payload in MUTATIONS.items():
        write(BENCHMARKS, "🦠️mutation-after-%s.snapshot.json" % kind, apply_mutation(base, payload))
        table[kind] = json.dumps(payload, ensure_ascii=False, separators=(",", ":"))
    print("\n--- feature Examples rows ---")
    for kind, line in table.items():
        print("| %s | %s |" % (kind, line))


# endregion 🔖️MutationCorpus


# region 🔖️Main
def main():
    """🚀️ Writes every benchmark fixture."""
    write(BENCHMARKS, "🏢️space-frame-2x2-bay.snapshot.json", space_frame())
    write(BENCHMARKS, "🏛️timber-roof-truss.snapshot.json", roof_truss())
    write(BENCHMARKS, "📏️cantilever-tip-load-local-z.snapshot.json", cantilever("Tz", -8000.0, "Tip Load Local Z"))
    write(BENCHMARKS, "📐️cantilever-tip-load-local-y.snapshot.json", cantilever("Ty", -8000.0, "Tip Load Local Y"))
    write(BENCHMARKS, "🌀️cantilever-tip-torsion.snapshot.json", cantilever("Rx", 2500.0, "Tip Torsion"))
    write(BENCHMARKS, "🌉️simply-supported-udl.snapshot.json", simply_supported_udl())
    write(SOLID, "🧱️prismatic-solid-column.snapshot.json", prismatic_solid_column())
    write(EIGEN, "🎵️modal-cantilever.snapshot.json", modal_cantilever())
    for name, (_label, _k, base, head) in BUCKLING_CASES.items():
        write(EIGEN, name, buckling_column(base, head))
    write_mutation_corpus()


if __name__ == "__main__":
    main()
# endregion 🔖️Main
