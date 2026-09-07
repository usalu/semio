#!/usr/bin/env python3
"""🏗️ Authors the fem2d solver-benchmark fixture corpus for the case
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧪️tests/🧮️solves-fem2d-1-benchmarks/🧫️fixtures/`.

Ticket input file (W5). It writes committed fixtures; it never writes results — those come from the
committed oracle `🐍️.py` through `🔨️run-fem2d-oracle.py`.

Three things happen here, in order:

1. `apply_mutation` below is a small independent Python transcription of the fem2d mutation algebra.
   Before it is trusted to build a single fixture it is REPLAYED against every committed
   `(before, mutation, after)` specification vector on disk (all 25 kinds, globbed so the
   hash-truncated case directory names do not matter). If any triple disagrees, this script refuses
   to write anything.
2. The five real-world/benchmark snapshots are authored as data.
3. Each of the 25 mutation kinds is applied to the base frame and the resulting `after` snapshot is
   written into one flat `🧬️mutated.snapshots.json` — a case directory may hold exactly one
   `🧫️fixtures` directory and no nested directories, so the whole mutate-then-solve corpus is one
   file rather than fifty.
"""

import copy
import glob
import json
import os
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
SUBSETS = os.path.join(REPO, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets")
CASE = os.path.join(SUBSETS, "📈️analysis/🧪️tests/🧮️solves-fem2d-1-benchmarks")
FIXTURES = os.path.join(CASE, "🧫️fixtures")

MEMBERS = ("nodes", "elements", "regions", "materials", "sections", "supports", "loadCases", "combinations", "analysis")
COLLECTIONS = {
    "node": ("nodes", "node", "newNode"),
    "element": ("elements", "element", "newElement"),
    "region": ("regions", "region", "newRegion"),
    "material": ("materials", "material", "newMaterial"),
    "section": ("sections", "section", "newSection"),
    "support": ("supports", "support", "newSupport"),
    "load-case": ("loadCases", "loadCase", None),
    "combination": ("combinations", "combination", None),
}
KINDS = (
    "create-node", "delete-node", "create-element", "delete-element", "replace-element",
    "create-material", "delete-material", "replace-material",
    "create-section", "delete-section", "replace-section",
    "create-support", "delete-support", "replace-support",
    "create-region", "delete-region", "replace-region",
    "create-load-case", "delete-load-case", "add-load", "remove-load", "change-load-case-self-weight",
    "create-combination", "delete-combination", "update-analysis-settings",
)


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}


def kind_of(mutation):
    for kind, tag in TAGS.items():
        if tag == mutation["mutation"]:
            return kind
    raise AssertionError("unknown mutation variant %r" % mutation["mutation"])


def find(items, identifier):
    for at, item in enumerate(items):
        if item["id"] == identifier:
            return at
    return None


def apply_mutation(document, mutation):
    """🧬️ Applies one typed mutation, returning the resulting model. No verb cascades."""
    kind = kind_of(mutation)
    result = copy.deepcopy(document)
    if kind == "update-analysis-settings":
        result["analysis"] = copy.deepcopy(mutation["settings"])
        return result
    if kind in ("add-load", "remove-load", "change-load-case-self-weight"):
        at = find(result["loadCases"], mutation["caseId"])
        if at is None:
            raise AssertionError("%s: no load case %r" % (kind, mutation["caseId"]))
        case = result["loadCases"][at]
        if kind == "add-load":
            case["loads"].append(copy.deepcopy(mutation["load"]))
        elif kind == "remove-load":
            where = find(case["loads"], mutation["loadId"])
            if where is None:
                raise AssertionError("%s: case %r carries no load %r" % (kind, case["id"], mutation["loadId"]))
            case["loads"].pop(where)
        else:
            case["selfWeight"] = mutation["newSelfWeight"]
        return result
    noun = kind.split("-", 1)[1]
    collection, create_argument, replace_argument = COLLECTIONS[noun]
    items = result[collection]
    if kind.startswith("create-"):
        record = copy.deepcopy(mutation[create_argument])
        if find(items, record["id"]) is not None:
            raise AssertionError("%s: %r is already in %s" % (kind, record["id"], collection))
        items.append(record)
        return result
    at = find(items, mutation["id"])
    if at is None:
        raise AssertionError("%s: %r is not in %s" % (kind, mutation["id"], collection))
    if kind.startswith("delete-"):
        items.pop(at)
    else:
        items[at] = copy.deepcopy(mutation[replace_argument])
    return result


def replay_committed_vectors():
    """📐️ Replays every committed `(before, mutation, after)` triple on disk through `apply_mutation`.

    This is what earns the right to generate an `after` snapshot below: the transcription is held to
    all 25 handcrafted vectors before it authors anything.
    """
    seen = set()
    for path in sorted(glob.glob(os.path.join(SUBSETS, "*/🧬️schema/🧬️mutations/*/🧪️tests/*/🦠️mutation/🔣️.json"))):
        case = os.path.dirname(os.path.dirname(path))
        with open(path, encoding="utf-8") as handle:
            mutation = json.load(handle)
        with open(os.path.join(case, "📸️snapshot/⬅️before/🔣️.json"), encoding="utf-8") as handle:
            before = json.load(handle)
        with open(os.path.join(case, "📸️snapshot/➡️after/🔣️.json"), encoding="utf-8") as handle:
            after = json.load(handle)
        produced = apply_mutation(before, mutation)
        if produced != after:
            raise AssertionError("committed vector %s disagrees with this transcription" % case)
        seen.add(kind_of(mutation))
    missing = [kind for kind in KINDS if kind not in seen]
    if missing:
        raise AssertionError("no committed vector replayed for %r" % missing)
    return len(seen)


# region 🔖️Corpus
STEEL = {"id": "steel", "name": "Steel S355", "e": 210000000000.0, "nu": 0.3, "rho": 7850.0}
CONCRETE = {"id": "concrete", "name": "Concrete C30/37", "e": 33000000000.0, "nu": 0.2, "rho": 2400.0}
IPE300 = {"id": "ipe300", "name": "IPE 300", "area": 0.005381, "iy": 8.356e-05}
IPE400 = {"id": "ipe400", "name": "IPE 400", "area": 0.00845, "iy": 0.0002313}
HEB300 = {"id": "heb300", "name": "HEB 300", "area": 0.01491, "iy": 0.0002517}
CHS139 = {"id": "chs139", "name": "CHS 139.7x4", "area": 0.001708, "iy": 3.93e-06}
SETTINGS = {"modalCount": 3, "bucklingCount": 3, "deformationScale": 100.0}


def snapshot(nodes, elements, regions, materials, sections, supports, load_cases, combinations, analysis):
    return {
        "nodes": nodes, "elements": elements, "regions": regions, "materials": materials, "sections": sections,
        "supports": supports, "loadCases": load_cases, "combinations": combinations, "analysis": analysis,
    }


def steel_cantilever():
    """📏️ A 6 m IPE 300 steel cantilever with a 10 kN tip load, in eight equal elements.

    Eight elements, not one: the tip deflection `PL³/(3EI)` is exact at the nodes for ANY element
    count, but the free-vibration frequencies are not, and this same fixture carries the modal
    benchmark. Six metres rather than three so the third bending mode (176 Hz) still sits below the
    first LONGITUDINAL mode (216 Hz), which is what a modal solve over a frame element also returns —
    at three metres the axial mode falls between bending modes 2 and 3 and `βₙL = 7.8548` would be
    compared against the wrong mode.
    """
    count = 8
    span = 6.0
    nodes = [{"id": "c%d" % i, "x": span * i / count, "y": 0.0} for i in range(count + 1)]
    elements = [{"kind": "beam", "id": "e%d" % (i + 1), "start": "c%d" % i, "end": "c%d" % (i + 1), "materialId": "steel", "sectionId": "ipe300"} for i in range(count)]
    return snapshot(
        nodes, elements, [], [dict(STEEL)], [dict(IPE300)],
        [{"id": "sup_a", "nodeId": "c0", "fixed": ["Tx", "Ty", "Rz"]}],
        [{"id": "tip", "name": "Tip load", "loads": [{"kind": "nodal", "id": "l_tip", "nodeId": "c%d" % count, "dof": "Ty", "value": -10000.0}], "selfWeight": False}],
        [], dict(SETTINGS),
    )


def steel_columns():
    """🏛️ Four disjoint 6 m IPE 300 columns, one per standard effective-length factor, each with its
    own load case carrying a 1 MN reference compression at its head.

    `K = 2.0` fixed-free, `K = 1.0` pinned-pinned, `K = 0.7` fixed-pinned, `K = 0.5` fixed-guided.
    Because the columns share no node, a case that compresses one column leaves the geometric
    stiffness of the other three at zero, so the lowest buckling factor of that case is that ONE
    column's `π²EI/(KL)² / P_ref`.
    """
    count = 8
    height = 6.0
    reference = 1000000.0
    layout = [
        ("a", 0.0, ["Tx", "Ty", "Rz"], []),
        ("b", 10.0, ["Tx", "Ty"], ["Tx"]),
        ("c", 20.0, ["Tx", "Ty", "Rz"], ["Tx"]),
        ("d", 30.0, ["Tx", "Ty", "Rz"], ["Tx", "Rz"]),
    ]
    nodes, elements, supports, cases = [], [], [], []
    for name, x, base_fixed, head_fixed in layout:
        for i in range(count + 1):
            nodes.append({"id": "%s%d" % (name, i), "x": x, "y": height * i / count})
        for i in range(count):
            elements.append({"kind": "beam", "id": "e_%s%d" % (name, i + 1), "start": "%s%d" % (name, i), "end": "%s%d" % (name, i + 1), "materialId": "steel", "sectionId": "ipe300"})
        supports.append({"id": "sup_%s_base" % name, "nodeId": "%s0" % name, "fixed": list(base_fixed)})
        if head_fixed:
            supports.append({"id": "sup_%s_head" % name, "nodeId": "%s%d" % (name, count), "fixed": list(head_fixed)})
        cases.append({
            "id": "col_%s" % name,
            "name": "Compression on column %s" % name.upper(),
            "loads": [{"kind": "nodal", "id": "l_%s" % name, "nodeId": "%s%d" % (name, count), "dof": "Ty", "value": -reference}],
            "selfWeight": False,
        })
    return snapshot(nodes, elements, [], [dict(STEEL)], [dict(IPE300)], supports, cases, [], {"modalCount": 3, "bucklingCount": 2, "deformationScale": 100.0})


def steel_simple_beam():
    """📐️ An 8 m IPE 400 simply supported beam under a 12 kN/m UDL — the `5wL⁴/(384EI)` closed form.

    Two elements so the midspan is a node: consistent-load Euler-Bernoulli finite elements reproduce
    the exact beam-theory deflection AT THE NODES, so this is an exact comparison, not a converging one.
    """
    return snapshot(
        [{"id": "a", "x": 0.0, "y": 0.0}, {"id": "m", "x": 4.0, "y": 0.0}, {"id": "b", "x": 8.0, "y": 0.0}],
        [
            {"kind": "beam", "id": "e1", "start": "a", "end": "m", "materialId": "steel", "sectionId": "ipe400"},
            {"kind": "beam", "id": "e2", "start": "m", "end": "b", "materialId": "steel", "sectionId": "ipe400"},
        ],
        [], [dict(STEEL)], [dict(IPE400)],
        [{"id": "sup_a", "nodeId": "a", "fixed": ["Tx", "Ty"]}, {"id": "sup_b", "nodeId": "b", "fixed": ["Ty"]}],
        [{"id": "udl", "name": "Uniform load", "loads": [
            {"kind": "memberUdl", "id": "l_1", "elementId": "e1", "wx": 0.0, "wy": -12000.0},
            {"kind": "memberUdl", "id": "l_2", "elementId": "e2", "wx": 0.0, "wy": -12000.0},
        ], "selfWeight": False}],
        [], dict(SETTINGS),
    )


def concrete_two_span():
    """🌉️ A two-span 2×6 m continuous concrete beam, 300×600 mm, dead + live + ULS combination.

    The textbook reaction split for a uniform load over both spans is `0.375wL : 1.25wL : 0.375wL`,
    which the oracle asserts on the dead case.
    """
    section = {"id": "rc300x600", "name": "RC 300x600", "area": 0.18, "iy": 0.0054}
    nodes = [{"id": "n%d" % i, "x": 3.0 * i, "y": 0.0} for i in range(5)]
    elements = [{"kind": "beam", "id": "e%d" % (i + 1), "start": "n%d" % i, "end": "n%d" % (i + 1), "materialId": "concrete", "sectionId": "rc300x600"} for i in range(4)]
    dead = [{"kind": "memberUdl", "id": "l_d%d" % (i + 1), "elementId": "e%d" % (i + 1), "wx": 0.0, "wy": -25000.0} for i in range(4)]
    live = [{"kind": "memberUdl", "id": "l_l%d" % (i + 1), "elementId": "e%d" % (i + 1), "wx": 0.0, "wy": -15000.0} for i in range(2)]
    return snapshot(
        nodes, elements, [], [dict(CONCRETE)], [section],
        [
            {"id": "sup_n0", "nodeId": "n0", "fixed": ["Tx", "Ty"]},
            {"id": "sup_n2", "nodeId": "n2", "fixed": ["Ty"]},
            {"id": "sup_n4", "nodeId": "n4", "fixed": ["Ty"]},
        ],
        [
            {"id": "dead", "name": "Dead", "loads": dead, "selfWeight": False},
            {"id": "live", "name": "Live on span 1", "loads": live, "selfWeight": False},
        ],
        [{"id": "uls", "name": "ULS", "terms": [{"caseId": "dead", "factor": 1.35}, {"caseId": "live", "factor": 1.5}]}],
        dict(SETTINGS),
    )


def timber_frame_members(portal):
    """🪵️ The committed timber portal frame's FRAME SUBSTRUCTURE — the same nodes, members, materials,
    sections, supports and the frame-only `snow` case, with the two slab regions and the two area
    loads dropped.

    Why the committed document itself is not solved here: its `slab_spare` region touches the rest of
    the model at exactly ONE node (`n2` at the origin), so the meshed slab can rotate rigidly about
    that node — a zero-energy mode, i.e. a mechanism. The region was added to give the `delete-`/
    `replace-region` verbs a trailing target, not to be analysed. The whole committed document is
    still carried in this corpus and is asserted, in its own scenario, to be REJECTED as singular.
    """
    frame = copy.deepcopy(portal)
    frame["regions"] = []
    for case in frame["loadCases"]:
        case["loads"] = [load for load in case["loads"] if load["kind"] != "area"]
    frame["loadCases"] = [case for case in frame["loadCases"] if case["id"] == "snow"]
    frame["combinations"] = []
    return frame


def steel_frame_base():
    """🏢️ The mutate-then-solve base: a 3-bay (6 m) two-storey (3.5 m) steel moment frame with a
    detached, four-corner-pinned concrete floor slab and one spare record for every `delete-`/
    `replace-` verb this vocabulary declares."""
    nodes = []
    for level, y in ((0, 0.0), (1, 3.5), (2, 7.0)):
        prefix = ("g", "a", "b")[level]
        for column in range(4):
            nodes.append({"id": "%s%d" % (prefix, column), "x": 6.0 * column, "y": y})
    nodes += [
        {"id": "q1", "x": 24.0, "y": 0.0}, {"id": "q2", "x": 27.0, "y": 0.0},
        {"id": "q3", "x": 27.0, "y": 2.0}, {"id": "q4", "x": 24.0, "y": 2.0},
        {"id": "n_spare", "x": 30.0, "y": 0.0},
    ]
    elements = []
    for column in range(4):
        elements.append({"kind": "beam", "id": "c0_%d" % column, "start": "g%d" % column, "end": "a%d" % column, "materialId": "steel", "sectionId": "heb300"})
    for column in range(4):
        elements.append({"kind": "beam", "id": "c1_%d" % column, "start": "a%d" % column, "end": "b%d" % column, "materialId": "steel", "sectionId": "heb300"})
    for bay in range(3):
        elements.append({"kind": "beam", "id": "b1_%d" % bay, "start": "a%d" % bay, "end": "a%d" % (bay + 1), "materialId": "steel", "sectionId": "ipe400"})
    for bay in range(3):
        elements.append({"kind": "beam", "id": "b2_%d" % bay, "start": "b%d" % bay, "end": "b%d" % (bay + 1), "materialId": "steel", "sectionId": "ipe400"})
    elements.append({"kind": "bar", "id": "br_spare", "start": "g0", "end": "a1", "materialId": "steel", "sectionId": "chs139"})

    outline = [[24.0, 0.0], [27.0, 0.0], [27.0, 2.0], [24.0, 2.0]]
    regions = [{"id": "r1", "name": "Plant room slab", "outline": outline, "holes": [], "thickness": 0.2, "materialId": "concrete", "meshSize": 1.0}]

    supports = [{"id": "sup_g%d" % column, "nodeId": "g%d" % column, "fixed": ["Tx", "Ty", "Rz"]} for column in range(4)]
    supports += [{"id": "sup_q%d" % (corner + 1), "nodeId": "q%d" % (corner + 1), "fixed": ["Tx", "Ty"]} for corner in range(4)]

    floor_beams = ["b1_0", "b1_1", "b1_2", "b2_0", "b2_1", "b2_2"]
    dead = [{"kind": "memberUdl", "id": "l_dead_%d" % (i + 1), "elementId": beam, "wx": 0.0, "wy": -25000.0} for i, beam in enumerate(floor_beams)]
    live = [{"kind": "memberUdl", "id": "l_live_%d" % (i + 1), "elementId": beam, "wx": 0.0, "wy": -15000.0} for i, beam in enumerate(floor_beams)]
    wind = [
        {"kind": "nodal", "id": "l_wind_1", "nodeId": "a0", "dof": "Tx", "value": 30000.0},
        {"kind": "nodal", "id": "l_wind_2", "nodeId": "b0", "dof": "Tx", "value": 45000.0},
    ]
    spare = [{"kind": "nodal", "id": "l_spare_1", "nodeId": "b3", "dof": "Ty", "value": -5000.0}]
    return snapshot(
        nodes, elements, regions,
        [dict(STEEL), dict(CONCRETE), {"id": "m_spare", "name": "C24 Timber", "e": 11000000000.0, "nu": 0.3, "rho": 450.0}],
        [dict(HEB300), dict(IPE400), dict(CHS139), {"id": "s_spare", "name": "SHS 100x100x5", "area": 0.001861, "iy": 2.72e-06}],
        supports,
        [
            {"id": "dead", "name": "Dead", "loads": dead, "selfWeight": False},
            {"id": "live", "name": "Live", "loads": live, "selfWeight": False},
            {"id": "wind", "name": "Wind from the left", "loads": wind, "selfWeight": False},
            {"id": "spare", "name": "Spare case", "loads": spare, "selfWeight": False},
        ],
        [
            {"id": "uls", "name": "ULS", "terms": [{"caseId": "dead", "factor": 1.35}, {"caseId": "live", "factor": 1.5}, {"caseId": "wind", "factor": 0.9}]},
            {"id": "sls", "name": "SLS", "terms": [{"caseId": "dead", "factor": 1.0}, {"caseId": "live", "factor": 1.0}]},
            {"id": "spare_combo", "name": "Spare combination", "terms": [{"caseId": "dead", "factor": 1.0}]},
        ],
        dict(SETTINGS),
    )


SLAB_OUTLINE = [[24.0, 0.0], [27.0, 0.0], [27.0, 2.0], [24.0, 2.0]]

MUTATIONS = {
    "create-node": ({"mutation": "createNode", "node": {"id": "n_new", "x": 30.0, "y": 3.5}}, "invariant"),
    "delete-node": ({"mutation": "deleteNode", "id": "n_spare"}, "invariant"),
    "create-element": ({"mutation": "createElement", "element": {"kind": "bar", "id": "br_new", "start": "g3", "end": "a2", "materialId": "steel", "sectionId": "chs139"}}, "changes"),
    "delete-element": ({"mutation": "deleteElement", "id": "br_spare"}, "changes"),
    "replace-element": ({"mutation": "replaceElement", "id": "br_spare", "newElement": {"kind": "beam", "id": "br_spare", "start": "g0", "end": "a1", "materialId": "steel", "sectionId": "chs139"}}, "changes"),
    "create-material": ({"mutation": "createMaterial", "material": {"id": "m_new", "name": "Aluminium EN AW-6060", "e": 70000000000.0, "nu": 0.33, "rho": 2700.0}}, "invariant"),
    "delete-material": ({"mutation": "deleteMaterial", "id": "m_spare"}, "invariant"),
    "replace-material": ({"mutation": "replaceMaterial", "id": "steel", "newMaterial": {"id": "steel", "name": "Steel S355 at elevated temperature", "e": 190000000000.0, "nu": 0.3, "rho": 7850.0}}, "changes"),
    "create-section": ({"mutation": "createSection", "section": {"id": "s_new", "name": "IPE 240", "area": 0.00391, "iy": 3.892e-05}}, "invariant"),
    "delete-section": ({"mutation": "deleteSection", "id": "s_spare"}, "invariant"),
    "replace-section": ({"mutation": "replaceSection", "id": "ipe400", "newSection": {"id": "ipe400", "name": "IPE 450", "area": 0.00988, "iy": 0.0003374}}, "changes"),
    "create-support": ({"mutation": "createSupport", "support": {"id": "sup_new", "nodeId": "a3", "fixed": ["Tx"]}}, "changes"),
    "delete-support": ({"mutation": "deleteSupport", "id": "sup_g3"}, "changes"),
    "replace-support": ({"mutation": "replaceSupport", "id": "sup_g0", "newSupport": {"id": "sup_g0", "nodeId": "g0", "fixed": ["Tx", "Ty"]}}, "changes"),
    "create-region": ({"mutation": "createRegion", "region": {"id": "r_topping", "name": "Plant room topping", "outline": SLAB_OUTLINE, "holes": [], "thickness": 0.06, "materialId": "concrete", "meshSize": 1.0}}, "invariant"),
    "delete-region": ({"mutation": "deleteRegion", "id": "r1"}, "invariant"),
    "replace-region": ({"mutation": "replaceRegion", "id": "r1", "newRegion": {"id": "r1", "name": "Plant room slab, thickened", "outline": SLAB_OUTLINE, "holes": [], "thickness": 0.3, "materialId": "concrete", "meshSize": 1.0}}, "invariant"),
    "create-load-case": ({"mutation": "createLoadCase", "loadCase": {"id": "snow", "name": "Snow", "loads": [{"kind": "memberUdl", "id": "l_snow_1", "elementId": "b2_0", "wx": 0.0, "wy": -8000.0}, {"kind": "memberUdl", "id": "l_snow_2", "elementId": "b2_1", "wx": 0.0, "wy": -8000.0}], "selfWeight": False}}, "changes"),
    "delete-load-case": ({"mutation": "deleteLoadCase", "id": "spare"}, "changes"),
    "add-load": ({"mutation": "addLoad", "caseId": "wind", "load": {"kind": "nodal", "id": "l_wind_3", "nodeId": "b3", "dof": "Tx", "value": 15000.0}}, "changes"),
    "remove-load": ({"mutation": "removeLoad", "caseId": "wind", "loadId": "l_wind_2"}, "changes"),
    "change-load-case-self-weight": ({"mutation": "changeLoadCaseSelfWeight", "caseId": "dead", "newSelfWeight": True}, "changes"),
    "create-combination": ({"mutation": "createCombination", "combination": {"id": "sls_char", "name": "SLS characteristic", "terms": [{"caseId": "dead", "factor": 1.0}, {"caseId": "wind", "factor": 1.0}]}}, "changes"),
    "delete-combination": ({"mutation": "deleteCombination", "id": "spare_combo"}, "changes"),
    "update-analysis-settings": ({"mutation": "updateAnalysisSettings", "settings": {"modalCount": 4, "bucklingCount": 2, "deformationScale": 75.0}}, "invariant"),
}
# endregion 🔖️Corpus


def write(name, payload):
    path = os.path.join(FIXTURES, name)
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(payload, handle, indent=2, ensure_ascii=False)
        handle.write("\n")
    return path


def main():
    replayed = replay_committed_vectors()
    print("[make] replayed %d committed specification vectors through this transcription — all agree" % replayed)

    os.makedirs(FIXTURES, exist_ok=True)
    portal_source = os.path.join(SUBSETS, "📈️analysis/🧪️tests/📈️mutate-fem2d-1-analysis/🧫️fixtures/🏗️timber-portal-frame.snapshot.json")
    with open(portal_source, encoding="utf-8") as handle:
        portal = json.load(handle)

    written = [
        write("🏗️timber-portal-frame.snapshot.json", portal),
        write("🪵️timber-frame-members.snapshot.json", timber_frame_members(portal)),
        write("📏️steel-cantilever.snapshot.json", steel_cantilever()),
        write("🏛️steel-columns.snapshot.json", steel_columns()),
        write("📐️steel-simple-beam.snapshot.json", steel_simple_beam()),
        write("🌉️concrete-two-span.snapshot.json", concrete_two_span()),
        write("🏢️steel-frame-base.snapshot.json", steel_frame_base()),
    ]

    base = steel_frame_base()
    mutated = {"base": "🏢️steel-frame-base.snapshot.json", "kinds": {}}
    for kind in KINDS:
        mutation, effect = MUTATIONS[kind]
        if kind_of(mutation) != kind:
            raise AssertionError("%s: the authored payload is tagged %r" % (kind, mutation["mutation"]))
        after = apply_mutation(base, mutation)
        if after == base:
            raise AssertionError("%s: the authored payload does not move the document" % kind)
        mutated["kinds"][kind] = {"mutation": mutation, "effect": effect, "after": after}
    written.append(write("🧬️mutated.snapshots.json", mutated))

    for path in written:
        print("[make] wrote %s (%d bytes)" % (os.path.relpath(path, REPO), os.path.getsize(path)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
