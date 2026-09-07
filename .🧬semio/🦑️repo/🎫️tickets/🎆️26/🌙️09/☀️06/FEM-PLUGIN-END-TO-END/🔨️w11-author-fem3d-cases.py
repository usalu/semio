#!/usr/bin/env python3
"""🏗️ Author the two additional fixture cases every fem3d mutation kind was missing.

Per kind the tree already carried exactly ONE vector. This script adds

* `🏗️hall-…` — a second REAL-WORLD happy path on a different real model, the glulam workshop
  hall (`hall_model()` below): a 12 m × 12 m two-bay GL24h portal hall on a C25/30 raft slab,
  every number in SI units and taken from EN 14080 / EN 338 / EN 1992 / EN 1993 tables;
* `🚨️…` or `⏸️…` — the edge branch the kind's own `🔺️diff/🦀️.rs` actually implements: a
  `Fatal mutation.duplicate-id`, an `Error mutation.target-missing`, or a
  `Warning mutation.no-op` that leaves the document untouched.

Every bundle is the closed 13-node source shape the test platform's `expectedVectorBundle`
requires (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:1395`):
`🦀️.rs`, `🦠️mutation/🔣️.json`, `📸️snapshot/{⬅️before,➡️after}/🔣️.json`, `🎯️outcome/🔣️.json`
and EXACTLY ONE of `🔺️diff/🔣️.json` / `🔺️diff/🚫️.absent` — the `.absent` alternative being the
platform's own way of saying "this branch produces no delta".

Usage: python3 🔨️w11-author-fem3d-cases.py [--cases] [--patch] [--manifest]
With no flag it does all three.
"""

import hashlib
import json
import os
import re
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *([".."] * 7)))
FEM = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem")
SUBSETS = os.path.join(FEM, "🗿️artifacts", "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets")
ENTRY = os.path.join(FEM, "📦️packages", "🦀️rust", "🦀️.rs")
TAXONOMY = os.path.join(REPO, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🔣️taxonomy.json")

# region 🔖️Model


def node(identifier, x, y, z):
    return {"id": identifier, "x": x, "y": y, "z": z}


def frame(identifier, start, end, material, section, roll=0.0):
    return {"kind": "frame", "id": identifier, "start": start, "end": end, "materialId": material, "sectionId": section, "roll": roll}


def bar(identifier, start, end, material, section):
    return {"kind": "bar", "id": identifier, "start": start, "end": end, "materialId": material, "sectionId": section}


def material(identifier, name, e, g, nu, rho):
    return {"id": identifier, "name": name, "e": e, "g": g, "nu": nu, "rho": rho}


def section(identifier, name, area, iy, iz, j):
    return {"id": identifier, "name": name, "area": area, "iy": iy, "iz": iz, "j": j}


def support(identifier, node_id, fixed):
    return {"id": identifier, "nodeId": node_id, "fixed": list(fixed)}


def solid(identifier, name, outline, holes, base_z, height, layers, mesh_size, material_id):
    return {"id": identifier, "name": name, "outline": outline, "holes": holes, "baseZ": base_z, "height": height, "layers": layers, "meshSize": mesh_size, "materialId": material_id}


def nodal(identifier, node_id, dof, value):
    return {"kind": "nodal", "id": identifier, "nodeId": node_id, "dof": dof, "value": value}


def udl(identifier, element_id, wx, wy, wz):
    return {"kind": "memberUdl", "id": identifier, "elementId": element_id, "wx": wx, "wy": wy, "wz": wz}


def area(identifier, solid_id, pressure):
    return {"kind": "area", "id": identifier, "solidId": solid_id, "pressure": pressure}


def load_case(identifier, name, loads, self_weight):
    return {"id": identifier, "name": name, "loads": loads, "selfWeight": self_weight}


def combination(identifier, name, terms):
    return {"id": identifier, "name": name, "terms": dict(sorted(terms.items()))}


PINNED = ("Tx", "Ty", "Tz")
CLAMPED = ("Tx", "Ty", "Tz", "Rx", "Ry", "Rz")
FRAME_Y = (0.0, 6.0, 12.0)


def hall_model():
    """🏭️ The glulam workshop hall — a 12 m span × 12 m long two-bay GL24h portal hall, eaves at
    4.2 m, ridge at 6.5 m, standing on a 350 mm C25/30 raft slab with a machine pit, braced in the
    first roof bay by M24 steel tie rods. Every quantity is SI: Pa, m, m², m⁴, kg/m³, N, N/m, Pa.

    Six deliberately unreferenced TRAILING spares (`gl32c`, `sec_strut`, `tie_spare`, `slab_apron`,
    `sup_ext_a`, `crane_spare`, `sls_qp_spare`, `n_ext_b`) exist because no `create-` verb in this
    vocabulary carries an index — the inverse of a delete is exact only for a trailing record.
    """
    nodes = []
    for index, y in enumerate(FRAME_Y):
        nodes.append(node("bl_%d" % index, 0.0, y, 0.0))
        nodes.append(node("el_%d" % index, 0.0, y, 4.2))
        nodes.append(node("ap_%d" % index, 6.0, y, 6.5))
        nodes.append(node("er_%d" % index, 12.0, y, 4.2))
        nodes.append(node("br_%d" % index, 12.0, y, 0.0))
    nodes.append(node("n_ext_a", 6.0, 18.0, 0.0))
    nodes.append(node("n_ext_b", 6.0, 18.0, 4.2))

    elements = []
    for index in range(3):
        elements.append(frame("col_l_%d" % index, "bl_%d" % index, "el_%d" % index, "gl24h", "sec_col"))
        elements.append(frame("raf_l_%d" % index, "el_%d" % index, "ap_%d" % index, "gl24h", "sec_raf"))
        elements.append(frame("raf_r_%d" % index, "ap_%d" % index, "er_%d" % index, "gl24h", "sec_raf"))
        elements.append(frame("col_r_%d" % index, "br_%d" % index, "er_%d" % index, "gl24h", "sec_col"))
    for index in range(2):
        elements.append(frame("pur_e_l_%d" % index, "el_%d" % index, "el_%d" % (index + 1), "c24", "sec_pur"))
        elements.append(frame("pur_a_%d" % index, "ap_%d" % index, "ap_%d" % (index + 1), "c24", "sec_pur"))
        elements.append(frame("pur_e_r_%d" % index, "er_%d" % index, "er_%d" % (index + 1), "c24", "sec_pur"))
    elements.append(bar("brc_0", "el_0", "ap_1", "s355", "sec_tie"))
    elements.append(bar("brc_1", "ap_0", "el_1", "s355", "sec_tie"))
    elements.append(bar("tie_spare", "el_2", "er_2", "s355", "sec_tie"))

    return {
        "nodes": nodes,
        "elements": elements,
        "materials": [
            material("gl24h", "GL24h Glulam", 11500000000.0, 650000000.0, 0.3, 420.0),
            material("c24", "C24 Solid Timber", 11000000000.0, 690000000.0, 0.3, 420.0),
            material("s355", "Steel S355", 210000000000.0, 81000000000.0, 0.3, 7850.0),
            material("c25_30", "C25/30 Concrete", 31000000000.0, 12920000000.0, 0.2, 2500.0),
            material("gl32c", "GL32c Glulam", 13500000000.0, 650000000.0, 0.3, 440.0),
        ],
        "sections": [
            section("sec_col", "GL24h Column 200x400", 0.08, 0.00106667, 0.00026667, 0.000733),
            section("sec_raf", "GL24h Rafter 180x600", 0.108, 0.00324, 0.0002916, 0.00092),
            section("sec_pur", "C24 Purlin 100x200", 0.02, 6.667e-05, 1.667e-05, 4.58e-05),
            section("sec_tie", "M24 Tie Rod", 0.000353, 1.03e-08, 1.03e-08, 2.06e-08),
            section("sec_strut", "SHS 100x100x5", 0.00184, 2.79e-06, 2.79e-06, 4.45e-06),
        ],
        "solids": [
            solid("slab_raft", "Workshop Raft Slab", [[0.0, 0.0], [12.0, 0.0], [12.0, 12.0], [0.0, 12.0]], [[[4.0, 4.0], [8.0, 4.0], [8.0, 8.0], [4.0, 8.0]]], -0.35, 0.35, 1, 1.0, "c25_30"),
            solid("slab_apron", "Door Apron Slab", [[12.0, 0.0], [15.0, 0.0], [15.0, 12.0], [12.0, 12.0]], [], -0.2, 0.2, 1, 1.5, "c25_30"),
        ],
        "supports": [
            support("sup_bl_0", "bl_0", PINNED),
            support("sup_br_0", "br_0", PINNED),
            support("sup_bl_1", "bl_1", PINNED),
            support("sup_br_1", "br_1", PINNED),
            support("sup_bl_2", "bl_2", PINNED),
            support("sup_br_2", "br_2", PINNED),
            support("sup_ext_a", "n_ext_a", CLAMPED),
        ],
        "loadCases": [
            load_case("dead", "Dead Load", [udl("ld_roof", "raf_l_1", 0.0, 0.0, -2700.0), area("ld_floor", "slab_raft", 2500.0)], True),
            load_case("snow", "Snow", [udl("ld_snow_l", "raf_l_1", 0.0, 0.0, -4320.0), udl("ld_snow_r", "raf_r_1", 0.0, 0.0, -4320.0)], False),
            load_case("wind_x", "Wind X", [nodal("ld_wx_0", "el_0", "Tx", 5400.0), nodal("ld_wx_1", "el_1", "Tx", 10800.0), nodal("ld_wx_spare", "el_2", "Tx", 5400.0)], False),
            load_case("crane_spare", "Crane Runway", [], False),
        ],
        "combinations": [
            combination("uls_str", "ULS STR", {"dead": 1.35, "snow": 1.5, "wind_x": 0.9}),
            combination("sls_char", "SLS Characteristic", {"dead": 1.0, "snow": 1.0}),
            combination("sls_qp_spare", "SLS Quasi-Permanent", {"dead": 1.0}),
        ],
        "analysis": {"modalCount": 6, "bucklingCount": 4, "deformationScale": 150.0},
    }


MEMBERS = ("nodes", "elements", "materials", "sections", "solids", "supports", "loadCases", "combinations", "analysis")
RUST_MEMBER = {"nodes": "nodes", "elements": "elements", "materials": "materials", "sections": "sections", "solids": "solids", "supports": "supports", "loadCases": "load_cases", "combinations": "combinations", "analysis": "analysis"}
NOUN_MEMBER = {"node": "nodes", "element": "elements", "material": "materials", "section": "sections", "solid": "solids", "support": "supports", "load-case": "loadCases", "combination": "combinations"}

# endregion 🔖️Model

# region 🔖️Diff


def empty_diff():
    return {
        "artifact": None,
        "nodes": None,
        "elements": None,
        "materials": None,
        "sections": None,
        "solids": None,
        "supports": None,
        "loadCases": None,
        "combinations": None,
        "analysis": None,
        "resultSourceId": None,
        "resultMode": None,
        "resultModeIndex": None,
        "camera": None,
        "solverResultsJson": None,
        "meshPreviewJson": None,
    }


def delta(added=(), removed=(), patched=()):
    return {"added": list(added), "removed": list(removed), "patched": [{"id": identifier, "item": item} for identifier, item in patched], "reordered": None}


def member_diff(member, **kwargs):
    built = empty_diff()
    built[member] = delta(**kwargs)
    return built


def analysis_diff(settings):
    built = empty_diff()
    built["analysis"] = settings
    return built


# endregion 🔖️Diff

# region 🔖️Reference


def find(items, identifier):
    for at, item in enumerate(items):
        if item["id"] == identifier:
            return at
    return None


def apply_reference(document, mutation):
    """🧬️ The forward transform, mirroring `🌐️any/🧪️tests/*/🐍️.py`'s own `apply_mutation` —
    used here only to DERIVE each committed `➡️after` from its `⬅️before`, never as the oracle."""
    kind = mutation["mutation"]
    result = json.loads(json.dumps(document))
    if kind == "updateAnalysisSettings":
        result["analysis"] = mutation["settings"]
    elif kind == "addLoad":
        result["loadCases"][find(result["loadCases"], mutation["caseId"])]["loads"].append(mutation["load"])
    elif kind == "removeLoad":
        case = result["loadCases"][find(result["loadCases"], mutation["caseId"])]
        case["loads"].pop(find(case["loads"], mutation["loadId"]))
    elif kind == "changeLoadCaseSelfWeight":
        result["loadCases"][find(result["loadCases"], mutation["caseId"])]["selfWeight"] = mutation["newSelfWeight"]
    elif kind.startswith("create"):
        member, argument = CREATE_ARGUMENT[kind]
        result[member].append(mutation[argument])
    elif kind.startswith("delete"):
        member = DELETE_MEMBER[kind]
        result[member].pop(find(result[member], mutation["id"]))
    else:
        member, argument = REPLACE_ARGUMENT[kind]
        result[member][find(result[member], mutation["id"])] = mutation[argument]
    return result


CREATE_ARGUMENT = {
    "createNode": ("nodes", "node"),
    "createElement": ("elements", "element"),
    "createMaterial": ("materials", "material"),
    "createSection": ("sections", "section"),
    "createSolid": ("solids", "solid"),
    "createSupport": ("supports", "support"),
    "createLoadCase": ("loadCases", "loadCase"),
    "createCombination": ("combinations", "combination"),
}
DELETE_MEMBER = {
    "deleteNode": "nodes",
    "deleteElement": "elements",
    "deleteMaterial": "materials",
    "deleteSection": "sections",
    "deleteSolid": "solids",
    "deleteSupport": "supports",
    "deleteLoadCase": "loadCases",
    "deleteCombination": "combinations",
}
REPLACE_ARGUMENT = {
    "replaceElement": ("elements", "newElement"),
    "replaceMaterial": ("materials", "newMaterial"),
    "replaceSection": ("sections", "newSection"),
    "replaceSolid": ("solids", "newSolid"),
    "replaceSupport": ("supports", "newSupport"),
}

# endregion 🔖️Reference

# region 🔖️Cases

KIND_DIRECTORY = {
    "create-node": ("🕸️mesh", "⚪️create-node"),
    "delete-node": ("🕸️mesh", "🕳️delete-node"),
    "create-element": ("🕸️mesh", "🧩️create-element"),
    "delete-element": ("🕸️mesh", "🗑️delete-element"),
    "replace-element": ("🕸️mesh", "♻️replace-element"),
    "create-section": ("🕸️mesh", "📐️create-section"),
    "delete-section": ("🕸️mesh", "✂️delete-section"),
    "replace-section": ("🕸️mesh", "📏️replace-section"),
    "create-solid": ("🕸️mesh", "🧊️create-solid"),
    "delete-solid": ("🕸️mesh", "🚫️delete-solid"),
    "replace-solid": ("🕸️mesh", "🔄️replace-solid"),
    "create-material": ("🧱️material", "🌱️create-material"),
    "delete-material": ("🧱️material", "🗑️delete-material"),
    "replace-material": ("🧱️material", "🔁️replace-material"),
    "create-support": ("🛡️boundary", "🛡️create-support"),
    "delete-support": ("🛡️boundary", "🗑️delete-support"),
    "replace-support": ("🛡️boundary", "🔁️replace-support"),
    "create-load-case": ("🏋️load", "📋️create-load-case"),
    "delete-load-case": ("🏋️load", "🗑️delete-load-case"),
    "add-load": ("🏋️load", "➕️add-load"),
    "remove-load": ("🏋️load", "➖️remove-load"),
    "change-load-case-self-weight": ("🏋️load", "⚖️change-load-case-self-weight"),
    "create-combination": ("🏋️load", "🔗️create-combination"),
    "delete-combination": ("🏋️load", "✂️delete-combination"),
    "update-analysis-settings": ("📈️analysis", "🎛️update-analysis-settings"),
}

SEMANTICS = {
    "create-node": ("create", "node", "CreatedNode"),
    "delete-node": ("delete", "node", "DeletedNode"),
    "create-element": ("create", "element", "CreatedElement"),
    "delete-element": ("delete", "element", "DeletedElement"),
    "replace-element": ("replace", "element", "ReplacedElement"),
    "create-section": ("create", "section", "CreatedSection"),
    "delete-section": ("delete", "section", "DeletedSection"),
    "replace-section": ("replace", "section", "ReplacedSection"),
    "create-solid": ("create", "solid", "CreatedSolid"),
    "delete-solid": ("delete", "solid", "DeletedSolid"),
    "replace-solid": ("replace", "solid", "ReplacedSolid"),
    "create-material": ("create", "material", "CreatedMaterial"),
    "delete-material": ("delete", "material", "DeletedMaterial"),
    "replace-material": ("replace", "material", "ReplacedMaterial"),
    "create-support": ("create", "support", "CreatedSupport"),
    "delete-support": ("delete", "support", "DeletedSupport"),
    "replace-support": ("replace", "support", "ReplacedSupport"),
    "create-load-case": ("create", "load-case", "CreatedLoadCase"),
    "delete-load-case": ("delete", "load-case", "DeletedLoadCase"),
    "add-load": ("add", "load", "AddedLoad"),
    "remove-load": ("remove", "load", "RemovedLoad"),
    "change-load-case-self-weight": ("change", "load-case", "ChangedLoadCaseSelfWeight"),
    "create-combination": ("create", "combination", "CreatedCombination"),
    "delete-combination": ("delete", "combination", "DeletedCombination"),
    "update-analysis-settings": ("update", "analysis-settings", "UpdatedAnalysisSettings"),
}


def tag_of(kind):
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


def suffix(kind, variant):
    return hashlib.sha256(("w11|fem3d|%s|%s" % (kind, variant)).encode("utf-8")).hexdigest()[:6]


def by_id(items, identifier):
    return items[find(items, identifier)]


def happy_cases(base):
    """🏗️ One second real-world happy path per kind, all on the same glulam workshop hall."""
    nodes, elements, materials, sections, solids, supports, cases, combinations = (base["nodes"], base["elements"], base["materials"], base["sections"], base["solids"], base["supports"], base["loadCases"], base["combinations"])
    return {
        "create-node": (
            {"mutation": "createNode", "node": node("n_ext_c", 12.0, 18.0, 0.0)},
            member_diff("nodes", added=[node("n_ext_c", 12.0, 18.0, 0.0)]),
            "The extension bay's second set-out point joins the grid as a trailing node.",
            "nodes",
            ['assert_eq!(snapshot.nodes.len(), 18, "{label}: the set-out point must be appended, never spliced into the frame grid");', 'assert_eq!(snapshot.nodes[17].id, "n_ext_c", "{label}: a create-node carries no index, so the node lands last");'],
        ),
        "delete-node": (
            {"mutation": "deleteNode", "id": "n_ext_b"},
            member_diff("nodes", removed=["n_ext_b"]),
            "The unused upper set-out point is struck; nothing references it, and no cascade runs.",
            "nodes",
            ['assert_eq!(snapshot.nodes.len(), 16, "{label}: exactly one node leaves the grid");', 'assert!(!snapshot.nodes.iter().any(|item| item.id == "n_ext_b"), "{label}: the struck node must be gone");'],
        ),
        "create-element": (
            {"mutation": "createElement", "element": bar("tie_f0", "el_0", "er_0", "s355", "sec_tie")},
            member_diff("elements", added=[bar("tie_f0", "el_0", "er_0", "s355", "sec_tie")]),
            "A horizontal eaves tie closes the first portal; every node, material and section it names already exists.",
            "elements",
            ['assert_eq!(snapshot.elements.len(), 22, "{label}: the eaves tie is appended behind the bracing");'],
        ),
        "delete-element": (
            {"mutation": "deleteElement", "id": "tie_spare"},
            member_diff("elements", removed=["tie_spare"]),
            "The reserved gable tie is struck — a trailing member, so its inverse re-creates it in place.",
            "elements",
            ['assert_eq!(snapshot.elements.len(), 20, "{label}: exactly one member leaves the frame");'],
        ),
        "replace-element": (
            {"mutation": "replaceElement", "id": "brc_0", "newElement": frame("brc_0", "el_0", "ap_1", "s355", "sec_strut")},
            member_diff("elements", patched=[("brc_0", frame("brc_0", "el_0", "ap_1", "s355", "sec_strut"))]),
            "The first roof brace becomes a compression-capable SHS strut: a `bar` variant is swapped for a `frame` carrying a roll.",
            "elements",
            ['assert_eq!(snapshot.elements.len(), 21, "{label}: a replace patches in place and never changes the member count");'],
        ),
        "create-section": (
            {"mutation": "createSection", "section": section("sec_ridge", "GL24h Beam 200x800", 0.16, 0.00853333, 0.00053333, 0.001798)},
            member_diff("sections", added=[section("sec_ridge", "GL24h Beam 200x800", 0.16, 0.00853333, 0.00053333, 0.001798)]),
            "A 200 × 800 glulam ridge beam profile is coined for the planned extension bay.",
            "sections",
            ['assert_eq!(snapshot.sections.len(), 6, "{label}: the ridge profile is appended behind the spare strut");', 'assert_eq!(snapshot.sections[5].area, 0.16, "{label}: the gross area must survive the round trip exactly");'],
        ),
        "delete-section": (
            {"mutation": "deleteSection", "id": "sec_strut"},
            member_diff("sections", removed=["sec_strut"]),
            "The unreferenced SHS strut profile is struck from the catalogue.",
            "sections",
            ['assert_eq!(snapshot.sections.len(), 4, "{label}: exactly one profile leaves the catalogue");'],
        ),
        "replace-section": (
            {"mutation": "replaceSection", "id": "sec_pur", "newSection": section("sec_pur", "C24 Purlin 100x220", 0.022, 8.873e-05, 1.833e-05, 5.258e-05)},
            member_diff("sections", patched=[("sec_pur", section("sec_pur", "C24 Purlin 100x220", 0.022, 8.873e-05, 1.833e-05, 5.258e-05))]),
            "The purlin is deepened from 200 mm to 220 mm; the profile identity is the slot, so every purlin follows at once.",
            "sections",
            ['assert_eq!(snapshot.sections[2].iy, 8.873e-05, "{label}: the deepened strong-axis inertia must land on the purlin slot");'],
        ),
        "create-solid": (
            {"mutation": "createSolid", "solid": solid("slab_plant", "Plant Room Slab", [[0.0, 12.0], [6.0, 12.0], [6.0, 16.0], [0.0, 16.0]], [], -0.25, 0.25, 1, 1.0, "c25_30")},
            member_diff("solids", added=[solid("slab_plant", "Plant Room Slab", [[0.0, 12.0], [6.0, 12.0], [6.0, 16.0], [0.0, 16.0]], [], -0.25, 0.25, 1, 1.0, "c25_30")]),
            "A 250 mm plant-room slab is extruded north of the hall on the concrete already in the catalogue.",
            "solids",
            ['assert_eq!(snapshot.solids.len(), 3, "{label}: the plant-room slab is appended behind the apron");', 'assert_eq!(snapshot.solids[2].outline.len(), 4, "{label}: the rectangular footprint must survive the round trip");'],
        ),
        "delete-solid": (
            {"mutation": "deleteSolid", "id": "slab_apron"},
            member_diff("solids", removed=["slab_apron"]),
            "The door apron is struck; the area load that exists lands on the raft, never on the apron, so nothing dangles.",
            "solids",
            ['assert_eq!(snapshot.solids.len(), 1, "{label}: only the raft survives");'],
        ),
        "replace-solid": (
            {"mutation": "replaceSolid", "id": "slab_raft", "newSolid": solid("slab_raft", "Workshop Raft Slab", [[0.0, 0.0], [12.0, 0.0], [12.0, 12.0], [0.0, 12.0]], [[[4.0, 4.0], [8.0, 4.0], [8.0, 8.0], [4.0, 8.0]]], -0.45, 0.45, 2, 0.75, "c25_30")},
            member_diff("solids", patched=[("slab_raft", solid("slab_raft", "Workshop Raft Slab", [[0.0, 0.0], [12.0, 0.0], [12.0, 12.0], [0.0, 12.0]], [[[4.0, 4.0], [8.0, 4.0], [8.0, 8.0], [4.0, 8.0]]], -0.45, 0.45, 2, 0.75, "c25_30"))]),
            "The raft is thickened from 350 mm to 450 mm, sunk to keep its top at ±0.00, and meshed twice as finely through two layers.",
            "solids",
            ['assert_eq!(snapshot.solids[0].height, 0.45, "{label}: the thickened raft must land on the raft slot");', 'assert_eq!(snapshot.solids[0].holes.len(), 1, "{label}: the machine pit must survive a whole-entity replace");'],
        ),
        "create-material": (
            {"mutation": "createMaterial", "material": material("s235", "Steel S235", 210000000000.0, 81000000000.0, 0.3, 7850.0)},
            member_diff("materials", added=[material("s235", "Steel S235", 210000000000.0, 81000000000.0, 0.3, 7850.0)]),
            "S235 joins the catalogue for the secondary steelwork, behind the reserved GL32c glulam.",
            "materials",
            ['assert_eq!(snapshot.materials.len(), 6, "{label}: the mild steel is appended behind the reserved glulam");', 'assert_eq!(snapshot.materials[5].g, 81000000000.0, "{label}: the shear modulus must survive the round trip exactly");'],
        ),
        "delete-material": (
            {"mutation": "deleteMaterial", "id": "gl32c"},
            member_diff("materials", removed=["gl32c"]),
            "The reserved GL32c glulam is struck; no member or solid ever named it.",
            "materials",
            ['assert_eq!(snapshot.materials.len(), 4, "{label}: exactly one grade leaves the catalogue");'],
        ),
        "replace-material": (
            {"mutation": "replaceMaterial", "id": "gl24h", "newMaterial": material("gl24h", "GL24h Glulam (5-percentile)", 9600000000.0, 650000000.0, 0.3, 385.0)},
            member_diff("materials", patched=[("gl24h", material("gl24h", "GL24h Glulam (5-percentile)", 9600000000.0, 650000000.0, 0.3, 385.0))]),
            "The glulam is re-based from EN 14080 mean stiffness to its 5-percentile values for the stability check.",
            "materials",
            ['assert_eq!(snapshot.materials[0].e, 9600000000.0, "{label}: the 5-percentile modulus must land on the glulam slot");'],
        ),
        "create-support": (
            {"mutation": "createSupport", "support": support("sup_ext_b", "n_ext_b", PINNED)},
            member_diff("supports", added=[support("sup_ext_b", "n_ext_b", PINNED)]),
            "The free upper set-out point is pinned; the node it names already exists, which is the one thing this verb checks.",
            "supports",
            ['assert_eq!(snapshot.supports.len(), 8, "{label}: the new pin is appended behind the reserved clamp");', 'assert_eq!(snapshot.supports[7].fixed.len(), 3, "{label}: a pin restrains the three translations and nothing else");'],
        ),
        "delete-support": (
            {"mutation": "deleteSupport", "id": "sup_ext_a"},
            member_diff("supports", removed=["sup_ext_a"]),
            "The reserved clamp on the extension set-out point is released.",
            "supports",
            ['assert_eq!(snapshot.supports.len(), 6, "{label}: only the six portal feet remain");'],
        ),
        "replace-support": (
            {"mutation": "replaceSupport", "id": "sup_bl_0", "newSupport": support("sup_bl_0", "bl_0", CLAMPED)},
            member_diff("supports", patched=[("sup_bl_0", support("sup_bl_0", "bl_0", CLAMPED))]),
            "The first portal foot is upgraded from a pin to a moment base — all six DOFs restrained.",
            "supports",
            ['assert_eq!(snapshot.supports[0].fixed.len(), 6, "{label}: a moment base restrains every DOF of the node");'],
        ),
        "create-load-case": (
            {"mutation": "createLoadCase", "loadCase": load_case("snow_drift", "Snow Drift", [udl("ld_drift_l", "raf_l_1", 0.0, 0.0, -8640.0)], False)},
            member_diff("loadCases", added=[load_case("snow_drift", "Snow Drift", [udl("ld_drift_l", "raf_l_1", 0.0, 0.0, -8640.0)], False)]),
            "A drifted-snow case is opened carrying one member UDL; the rafter it loads is resolved before the case is admitted.",
            "loadCases",
            ['assert_eq!(snapshot.load_cases.len(), 5, "{label}: the drift case is appended behind the reserved crane case");'],
        ),
        "delete-load-case": (
            {"mutation": "deleteLoadCase", "id": "crane_spare"},
            member_diff("loadCases", removed=["crane_spare"]),
            "The reserved crane case is struck; no combination ever weighted it.",
            "loadCases",
            ['assert_eq!(snapshot.load_cases.len(), 3, "{label}: exactly one case leaves the schedule");'],
        ),
        "add-load": (
            {"mutation": "addLoad", "caseId": "dead", "load": udl("ld_services", "pur_a_0", 0.0, 0.0, -300.0)},
            member_diff("loadCases", patched=[("dead", load_case("dead", "Dead Load", by_id(cases, "dead")["loads"] + [udl("ld_services", "pur_a_0", 0.0, 0.0, -300.0)], True))]),
            "Hung services are added to the ridge purlin inside the dead case — the verb patches the whole case, never the load list alone.",
            "loadCases",
            ['assert_eq!(snapshot.load_cases[0].loads.len(), 3, "{label}: the services UDL joins the dead case behind the floor pressure");'],
        ),
        "remove-load": (
            {"mutation": "removeLoad", "caseId": "wind_x", "loadId": "ld_wx_spare"},
            member_diff("loadCases", patched=[("wind_x", load_case("wind_x", "Wind X", by_id(cases, "wind_x")["loads"][:2], False))]),
            "The gable wind node load is dropped — the trailing load of its case, so the inverse re-appends it exactly.",
            "loadCases",
            ['assert_eq!(snapshot.load_cases[2].loads.len(), 2, "{label}: the gable node load leaves the wind case");'],
        ),
        "change-load-case-self-weight": (
            {"mutation": "changeLoadCaseSelfWeight", "caseId": "crane_spare", "newSelfWeight": True},
            member_diff("loadCases", patched=[("crane_spare", load_case("crane_spare", "Crane Runway", [], True))]),
            "The crane case starts carrying the runway's own weight; its load list stays empty.",
            "loadCases",
            ['assert!(snapshot.load_cases[3].self_weight, "{label}: the crane case must now carry self-weight");'],
        ),
        "create-combination": (
            {"mutation": "createCombination", "combination": combination("uls_acc", "ULS Accidental", {"dead": 1.0, "snow": 0.2})},
            member_diff("combinations", added=[combination("uls_acc", "ULS Accidental", {"dead": 1.0, "snow": 0.2})]),
            "An accidental combination is opened; every case its terms weight is resolved before the combination is admitted.",
            "combinations",
            ['assert_eq!(snapshot.combinations.len(), 4, "{label}: the accidental combination is appended last");', 'assert_eq!(snapshot.combinations[3].terms.len(), 2, "{label}: both weighted cases must survive the round trip");'],
        ),
        "delete-combination": (
            {"mutation": "deleteCombination", "id": "sls_qp_spare"},
            member_diff("combinations", removed=["sls_qp_spare"]),
            "The reserved quasi-permanent combination is struck.",
            "combinations",
            ['assert_eq!(snapshot.combinations.len(), 2, "{label}: exactly one combination leaves the schedule");'],
        ),
        "update-analysis-settings": (
            {"mutation": "updateAnalysisSettings", "settings": {"modalCount": 12, "bucklingCount": 8, "deformationScale": 100.0}},
            analysis_diff({"modalCount": 12, "bucklingCount": 8, "deformationScale": 100.0}),
            "The modal count is doubled to reach the mass participation a hall of this stiffness needs, and the display exaggeration is halved.",
            "analysis",
            ['assert_eq!(snapshot.analysis.modal_count, 12, "{label}: the requested mode count must land verbatim");', 'assert_eq!(snapshot.analysis.deformation_scale, 100.0, "{label}: the display scale is part of the same inseparable facet");'],
        ),
    }


def edge_cases(base):
    """🚨️ The refusal / no-op branch each kind's own `🔺️diff/🦀️.rs` implements, and nothing else."""
    elements, materials, sections, solids, supports, cases = (base["elements"], base["materials"], base["sections"], base["solids"], base["supports"], base["loadCases"])
    return {
        "create-node": ("🚨️", "dup-node-id", {"mutation": "createNode", "node": node("ap_1", 6.0, 6.0, 6.5)}, "fatal", "mutation.duplicate-id", ["ap_1"], "A second `ap_1` would make the ridge node of the middle frame ambiguous, so the identity collision is FATAL — not the Error the misses use."),
        "delete-node": ("🚨️", "no-such-node", {"mutation": "deleteNode", "id": "ap_9"}, "error", "mutation.target-missing", ["ap_9"], "There is no ninth frame, so the strike addresses nothing and the grid is left exactly as it was."),
        "create-element": ("🚨️", "dangling-start", {"mutation": "createElement", "element": bar("tie_x", "el_9", "er_0", "s355", "sec_tie")}, "error", "mutation.target-missing", ["el_9"], "`create-element` resolves start, end, material and section in that order; the start node is the first to miss, so it is the one reported."),
        "delete-element": ("🚨️", "no-such-element", {"mutation": "deleteElement", "id": "col_l_9"}, "error", "mutation.target-missing", ["col_l_9"], "No such column exists, so the frame is left untouched."),
        "replace-element": ("⏸️", "same-element", {"mutation": "replaceElement", "id": "brc_0", "newElement": by_id(elements, "brc_0")}, "warn", "mutation.no-op", [], "Replacing the brace with the value it already holds is a no-op WARNING, never a rejection: the verb still applies, it simply writes nothing."),
        "create-section": ("🚨️", "dup-section-id", {"mutation": "createSection", "section": section("sec_col", "GL24h Column 200x400", 0.08, 0.00106667, 0.00026667, 0.000733)}, "fatal", "mutation.duplicate-id", ["sec_col"], "Coining a second `sec_col` would leave every column pointing at an ambiguous profile, so the collision is FATAL."),
        "delete-section": ("🚨️", "no-such-section", {"mutation": "deleteSection", "id": "sec_x"}, "error", "mutation.target-missing", ["sec_x"], "No such profile is catalogued, so the catalogue is left untouched."),
        "replace-section": ("⏸️", "same-section", {"mutation": "replaceSection", "id": "sec_raf", "newSection": by_id(sections, "sec_raf")}, "warn", "mutation.no-op", [], "Rewriting the rafter profile with its own value is a no-op WARNING; the verb reserves `target-missing` for a profile that is not catalogued at all."),
        "create-solid": ("🚨️", "dangling-mat", {"mutation": "createSolid", "solid": solid("slab_x", "Yard Slab", [[15.0, 0.0], [21.0, 0.0], [21.0, 12.0], [15.0, 12.0]], [], -0.2, 0.2, 1, 1.5, "c50_60")}, "error", "mutation.target-missing", ["c50_60"], "`create-solid` is the one create in this vocabulary that resolves a foreign key: a solid with no material could never be meshed into `Tet4` elements."),
        "delete-solid": ("🚨️", "no-such-solid", {"mutation": "deleteSolid", "id": "slab_x"}, "error", "mutation.target-missing", ["slab_x"], "No yard slab was ever extruded, so the concrete is left untouched."),
        "replace-solid": ("⏸️", "same-solid", {"mutation": "replaceSolid", "id": "slab_apron", "newSolid": by_id(solids, "slab_apron")}, "warn", "mutation.no-op", [], "Re-extruding the apron at its own footprint, base and thickness is a no-op WARNING — the whole-entity comparison is what decides, not the id."),
        "create-material": ("🚨️", "dup-material-id", {"mutation": "createMaterial", "material": material("gl24h", "GL24h Glulam", 11500000000.0, 650000000.0, 0.3, 420.0)}, "fatal", "mutation.duplicate-id", ["gl24h"], "A second `gl24h` would leave every glulam member pointing at an ambiguous grade, so the collision is FATAL — even when the payload is byte-identical to the row that already exists."),
        "delete-material": ("🚨️", "no-such-material", {"mutation": "deleteMaterial", "id": "gl99"}, "error", "mutation.target-missing", ["gl99"], "No such grade is catalogued, so the catalogue is left untouched."),
        "replace-material": ("⏸️", "same-material", {"mutation": "replaceMaterial", "id": "c24", "newMaterial": by_id(materials, "c24")}, "warn", "mutation.no-op", [], "Re-declaring C24 with its own five properties is a no-op WARNING; only an uncatalogued grade is `target-missing`."),
        "create-support": ("🚨️", "dangling-node", {"mutation": "createSupport", "support": support("sup_x", "bl_9", PINNED)}, "error", "mutation.target-missing", ["bl_9"], "A support restrains a NODE's degrees of freedom, so a support on a node that does not exist restrains nothing and is refused."),
        "delete-support": ("🚨️", "no-such-support", {"mutation": "deleteSupport", "id": "sup_x"}, "error", "mutation.target-missing", ["sup_x"], "No such restraint exists, so the boundary conditions are left untouched."),
        "replace-support": ("⏸️", "same-support", {"mutation": "replaceSupport", "id": "sup_br_2", "newSupport": by_id(supports, "sup_br_2")}, "warn", "mutation.no-op", [], "Re-pinning a foot that is already pinned in exactly those three DOFs is a no-op WARNING."),
        "create-load-case": ("🚨️", "dangling-solid", {"mutation": "createLoadCase", "loadCase": load_case("flood", "Flood", [area("ld_flood", "slab_x", 4000.0)], False)}, "error", "mutation.target-missing", ["slab_x"], "`create-load-case` is the only load verb that resolves the targets of the loads it carries: a nodal load needs its node, a member UDL its element, an area pressure its solid."),
        "delete-load-case": ("🚨️", "no-such-case", {"mutation": "deleteLoadCase", "id": "quake"}, "error", "mutation.target-missing", ["quake"], "No seismic case was ever opened, so the load schedule is left untouched."),
        "add-load": ("⏸️", "dup-load-id", {"mutation": "addLoad", "caseId": "wind_x", "load": nodal("ld_wx_0", "el_0", "Tx", 5400.0)}, "warn", "mutation.no-op", [], "A load id the case already carries is a no-op WARNING, not a duplicate-id FATAL — `add-load` patches a case rather than coining a document-level identity."),
        "remove-load": ("🚨️", "no-such-load", {"mutation": "removeLoad", "caseId": "snow", "loadId": "ld_wx_0"}, "error", "mutation.target-missing", ["ld_wx_0"], "The load id exists — in `wind_x`, not in `snow`. Load identity is scoped to its case, so addressing it from the wrong case is a miss."),
        "change-load-case-self-weight": ("🚨️", "sw-no-such-case", {"mutation": "changeLoadCaseSelfWeight", "caseId": "seismic", "newSelfWeight": True}, "error", "mutation.target-missing", ["seismic"], "Switching self-weight on a case that was never opened is refused before the flag is read; the verb keeps its no-op WARNING for a case that already holds that flag."),
        "create-combination": ("🚨️", "dangling-term", {"mutation": "createCombination", "combination": combination("uls_seis", "ULS Seismic", {"dead": 1.0, "seismic": 1.0})}, "error", "mutation.target-missing", ["seismic"], "Terms are resolved in the map's own sorted order, so `dead` passes and `seismic` — the case that was never opened — is the one reported."),
        "delete-combination": ("🚨️", "no-such-combo", {"mutation": "deleteCombination", "id": "uls_acc"}, "error", "mutation.target-missing", ["uls_acc"], "The accidental combination is only ever created by this kind's sibling happy path; against the unmutated hall it does not exist."),
        "update-analysis-settings": ("⏸️", "same-settings", {"mutation": "updateAnalysisSettings", "settings": dict(base["analysis"])}, "warn", "mutation.no-op", [], "Writing the settings the document already holds is a no-op WARNING — the comparison is on the whole inseparable facet, all three fields at once."),
    }


# endregion 🔖️Cases

# region 🔖️Rust

HEADER = '''//! 🧪️ `{kind}` fixture — `{directory}`.
//!
//! Source of truth is the committed JSON bundle beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🏭️ `⬅️before` is the GLULAM WORKSHOP HALL, the second real fem3d model this artifact carries
//! (ticket `26/09/06/FEM-PLUGIN-END-TO-END`): a 12 m span × 12 m long two-bay GL24h portal hall,
//! eaves at 4.2 m and ridge at 6.5 m, braced by M24 steel rods and standing on a 350 mm C25/30
//! raft slab with a machine pit. Every quantity is SI — Pa, m, m², m⁴, kg/m³, N, N/m.
//!
//! {rationale}

use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::mutations::{{apply_fem3d_mutation, inverse_fem3d_mutation}};
use crate::artifacts::fem3d::Fem3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
{diff_const}const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Fem3dSnapshot {{
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> Fem3dSnapshot {{
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> Fem3dMutation {{
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}}
'''

CANONICAL = '''
/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{label}: committed {{label}} JSON is not canonical");
    }}
    let reencoded = dsl::ToValue::to_value(&mutation());
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{label}: committed mutation JSON is not canonical");
}}
'''

SEMANTICS_ASSERT = '''
/// 🪪️ The fixture is bound to this very kind's own semantic descriptor, never a sibling's.
#[test]
fn semantics_bind_the_declared_kind() {{
    let semantics = <Fem3dMutation as protocol::SemanticMutation<Fem3dSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("{verb}", "{entity}", "{kind}", "{record}"), "{label}: the fixture must be bound to {kind}'s own descriptor");
}}
'''

HAPPY_BODY = '''
/// ▶️ The mutation carries the committed hall from `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("{kind} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{label}: applied state differs from committed after-snapshot");
{extra}
}}

/// 🔀️ This verb writes exactly ONE of the nine members; an implementation that re-derived a sibling
/// collection on every edit would still land on the right value for the member it meant to write.
#[test]
fn touches_only_its_own_member() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("forward applies");
{member_asserts}
}}

/// ↩️ Applying the computed inverse after the forward step lands back on the committed `before`.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {{
        apply_fem3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{label}: inverse did not restore the before-snapshot");
}}

/// 🎯️ The declared outcome holds: applied, with no diagnostic at all.
#[test]
fn declared_outcome_holds() {{
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "{label}: this vector declares an applied outcome");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "{label}: a clean application raises no diagnostic, got {{:?}}", produced.messages());
}}

/// 🔺️ The produced delta is exactly the committed sparse `🔺️diff/🔣️.json`.
#[test]
fn produces_committed_diff() {{
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{label}: produced diff differs from the committed 🔺️diff/🔣️.json");
}}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {{
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{label}: committed diff JSON is not canonical");
}}

/// 🩹 Replaying the committed delta on `before` reproduces the committed `after` on its own.
#[test]
fn committed_diff_applies_to_after() {{
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem3d::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{label}: committed diff did not carry before to after");
}}
'''

NOOP_BODY = '''
/// ▶️ A no-op still APPLIES; it simply writes nothing, so `after` is the committed `before` again.
#[test]
fn applies_to_committed_after() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("{kind}'s identity diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{label}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "{label}: a no-op must leave every one of the nine members exactly as it found them");
}}

/// ⚠️ The branch this vector pins: a Warning `mutation.no-op` with an EMPTY diff and no target
/// address — `MutationOutcome::empty().warn(..)` is the 2-arg builder, which attaches none.
#[test]
fn a_redundant_write_is_a_warning_not_a_rejection() {{
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::artifacts::fem3d::diff::Fem3dDiff::default(), "{label}: a no-op must carry the identity diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "{label}: exactly one diagnostic is expected, got {{messages:?}}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "{label}: a redundant write is reported as no-op, never as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "{label}: a no-op is a Warning — the mutation still applies");
    assert!(messages[0].target.is_empty(), "{label}: the 2-arg warn builder attaches no target address");
}}

/// ↩️ The inverse of a no-op is itself a no-op, so the round trip is the identity twice over.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {{
        apply_fem3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{label}: inverse did not restore the before-snapshot");
}}

/// 🎯️ The declared outcome holds, code and level together.
#[test]
fn declared_outcome_holds() {{
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "{label}: a no-op declares an applied outcome");
    let declared = outcome.get("messages").and_then(dsl::DslValue::as_array).expect("the declared outcome carries messages");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(declared.len(), produced.messages().len(), "{label}: the declared diagnostic count must match the emitted one");
    assert_eq!(declared[0].get("code").and_then(dsl::DslValue::as_str), Some(produced.messages()[0].code.0.as_str()), "{label}: the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(dsl::DslValue::as_str), Some("warn"), "{label}: the declared level must name the Warning the builder raises");
}}

/// 🔺️ The produced delta is exactly the committed all-null `🔺️diff/🔣️.json`.
#[test]
fn produces_committed_diff() {{
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{label}: produced diff differs from the committed 🔺️diff/🔣️.json");
}}

/// 🔣️ The committed identity diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {{
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{label}: committed diff JSON is not canonical");
}}

/// 🩹 Replaying the committed identity delta on `before` reproduces the committed `after`.
#[test]
fn committed_diff_applies_to_after() {{
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem3d::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{label}: committed diff did not carry before to after");
}}
'''

REJECT_BODY = '''
/// ▶️ A refused mutation leaves the document byte-identical to the committed `after`, which is the
/// committed `before` again. `vcs::apply_mutation` is deliberately policy-agnostic — it applies the
/// (empty) diff and returns `Ok`, so REJECTION IS NOT VISIBLE IN THE RESULT, only in the messages.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{label}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "{label}: a refused mutation must leave every one of the nine members untouched");
}}

/// 🚨️ The branch this vector pins, level and address together.
#[test]
fn the_refusal_is_the_declared_diagnostic() {{
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::artifacts::fem3d::diff::Fem3dDiff::default(), "{label}: a refused mutation must carry the empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "{label}: exactly one diagnostic is expected, got {{messages:?}}");
    assert_eq!(messages[0].code.0, "{code}", "{label}: the refusal is reported as {code}");
    assert_eq!(messages[0].level, protocol::Severity::{severity}, "{label}: {severity_rationale}");
    assert_eq!(messages[0].target, vec![{target_literals}], "{label}: {target_rationale}");
}}

/// ↩️ {inverse_rationale}
#[test]
fn inverse_has_the_declared_shape() {{
    let inverse = inverse_fem3d_mutation(&before(), &mutation());
{inverse_asserts}
}}

/// 🎯️ The declared outcome — status, code, level and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {{
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("rejected"), "{label}: this vector declares a rejected outcome");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(dsl::DslValue::as_str), Some(message.code.0.as_str()), "{label}: the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(dsl::DslValue::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "{label}: the declared path must match the emitted target");
}}
'''

INVERSE_SHAPES = {
    "empty": (
        "A BASE-derived inverse reads the record it must restore out of `before`; with nothing there to read, it collapses to no steps at all.",
        ['    assert!(inverse.is_empty(), "{label}: a base-derived inverse of a missing target has nothing to restore, got {{inverse:?}}");'],
    ),
    "one": (
        "A PAYLOAD-derived inverse is computed from the request, not from `before`, so it is emitted even when the request itself was refused.",
        [
            '    assert_eq!(inverse.len(), 1, "{label}: a payload-derived inverse always emits exactly one step, got {{inverse:?}}");',
            '    assert!(matches!(inverse[0], Fem3dMutation::{variant}(_)), "{label}: the inverse of {kind} is a {inverse_kind}, got {{:?}}", inverse[0]);',
        ],
    ),
}

INVERSE_VARIANT = {
    "create-node": "DeleteNode",
    "create-element": "DeleteElement",
    "create-section": "DeleteSection",
    "create-solid": "DeleteSolid",
    "create-material": "DeleteMaterial",
    "create-support": "DeleteSupport",
    "create-load-case": "DeleteLoadCase",
    "create-combination": "DeleteCombination",
    "add-load": "RemoveLoad",
}

# endregion 🔖️Rust

# region 🔖️Emit


def dump(path, payload):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(payload, ensure_ascii=False, indent=2) + "\n")


def stem(directory):
    """🔤️ The kebab identity behind a case directory's leading emoji grapheme."""
    return re.sub(r"^[^a-z0-9]+", "", directory)


def rust_member_asserts(written, label):
    lines = []
    for member in MEMBERS:
        field = RUST_MEMBER[member]
        comparison = "assert_ne!" if member == written else "assert_eq!"
        note = "is the one member this verb writes" if member == written else "must not move when this verb runs"
        lines.append('    %s(snapshot.%s, base.%s, "%s: %s %s");' % (comparison, field, field, label, field, note))
    return "\n".join(lines)


def build():
    base = hall_model()
    happy = happy_cases(base)
    edge = edge_cases(base)
    built = []
    for kind, (subset, kind_dir) in KIND_DIRECTORY.items():
        mutation, diff, rationale, written, extra = happy[kind]
        directory = "🏗️%s-%s" % (HAPPY_SLUG[kind], suffix(kind, "hall"))
        after = apply_reference(base, mutation)
        built.append({
            "kind": kind,
            "subset": subset,
            "kindDirectory": kind_dir,
            "directory": directory,
            "variant": "happy",
            "mutation": mutation,
            "before": base,
            "after": after,
            "diff": diff,
            "outcome": {"status": "applied"},
            "rationale": rationale,
            "written": written,
            "extra": extra,
            "module": "tests_hall_%s" % HAPPY_SLUG[kind].replace("-", "_"),
        })
        emoji, slug, mutation, level, code, target, rationale = edge[kind]
        directory = "%s%s-%s" % (emoji, slug, suffix(kind, "edge"))
        if level == "warn":
            outcome = {"status": "applied", "messages": [{"level": "warn", "code": code}]}
            diff = empty_diff()
        else:
            outcome = {"status": "rejected", "code": code, "path": target, "messages": [{"level": level, "code": code}]}
            diff = None
        built.append({
            "kind": kind,
            "subset": subset,
            "kindDirectory": kind_dir,
            "directory": directory,
            "variant": "noop" if level == "warn" else "reject",
            "mutation": mutation,
            "before": base,
            "after": base,
            "diff": diff,
            "outcome": outcome,
            "rationale": rationale,
            "level": level,
            "code": code,
            "target": target,
            "module": "tests_edge_%s" % slug.replace("-", "_"),
        })
    return built


HAPPY_SLUG = {
    "create-node": "hall-new-node",
    "delete-node": "hall-cut-node",
    "create-element": "hall-new-tie",
    "delete-element": "hall-cut-tie",
    "replace-element": "hall-strut",
    "create-section": "hall-new-beam",
    "delete-section": "hall-cut-shs",
    "replace-section": "hall-deep-purlin",
    "create-solid": "hall-new-slab",
    "delete-solid": "hall-cut-apron",
    "replace-solid": "hall-thick-raft",
    "create-material": "hall-new-steel",
    "delete-material": "hall-cut-gl32c",
    "replace-material": "hall-regrades",
    "create-support": "hall-new-pin",
    "delete-support": "hall-cut-pin",
    "replace-support": "hall-fixes-base",
    "create-load-case": "hall-snow-drift",
    "delete-load-case": "hall-cut-crane",
    "add-load": "hall-adds-udl",
    "remove-load": "hall-cut-wind",
    "change-load-case-self-weight": "hall-crane-sw",
    "create-combination": "hall-new-acc",
    "delete-combination": "hall-cut-qp",
    "update-analysis-settings": "hall-more-modes",
}

TARGET_RATIONALE = {
    "fatal": "the diagnostic addresses exactly the identity that collided",
    "error": "the diagnostic addresses exactly the identity that could not be resolved",
}

SEVERITY_RATIONALE = {
    "fatal": "an identity collision is an invariant breach that no merge policy may absorb",
    "error": "a missed target is an Error, not the Fatal a duplicate identity raises",
}


def rust_source(case):
    kind = case["kind"]
    label = "%s/%s" % (kind, stem(case["directory"]))
    verb, entity, record = SEMANTICS[kind]
    diff_const = 'const DIFF: &str = include_str!("🔺️diff/🔣️.json");\n' if case["diff"] is not None else ""
    text = HEADER.format(kind=kind, directory=case["directory"], rationale=case["rationale"], diff_const=diff_const)
    if case["variant"] == "happy":
        text += HAPPY_BODY.format(kind=kind, label=label, extra="\n".join("    " + line.format(label=label) for line in case["extra"]), member_asserts=rust_member_asserts(case["written"], label))
    elif case["variant"] == "noop":
        text += NOOP_BODY.format(kind=kind, label=label)
    else:
        shape = "one" if kind in INVERSE_VARIANT else "empty"
        rationale, asserts = INVERSE_SHAPES[shape]
        variant = INVERSE_VARIANT.get(kind, "")
        inverse_kind = re.sub(r"(?<!^)(?=[A-Z])", "-", variant).lower() if variant else ""
        text += REJECT_BODY.format(
            kind=kind,
            label=label,
            code=case["code"],
            severity="Fatal" if case["level"] == "fatal" else "Error",
            severity_rationale=SEVERITY_RATIONALE[case["level"]],
            target_rationale=TARGET_RATIONALE[case["level"]],
            target_literals=", ".join('"%s".to_string()' % entry for entry in case["target"]),
            inverse_rationale=rationale,
            inverse_asserts="\n".join(line.format(label=label, kind=kind, variant=variant, inverse_kind=inverse_kind) for line in asserts),
        )
    text += CANONICAL.format(label=label)
    text += SEMANTICS_ASSERT.format(label=label, kind=kind, verb=verb, entity=entity, record=record)
    return text


def write_cases(cases):
    for case in cases:
        root = os.path.join(SUBSETS, case["subset"], "🧬️schema", "🧬️mutations", case["kindDirectory"], "🧪️tests", case["directory"])
        dump(os.path.join(root, "🦠️mutation", "🔣️.json"), case["mutation"])
        dump(os.path.join(root, "📸️snapshot", "⬅️before", "🔣️.json"), case["before"])
        dump(os.path.join(root, "📸️snapshot", "➡️after", "🔣️.json"), case["after"])
        dump(os.path.join(root, "🎯️outcome", "🔣️.json"), case["outcome"])
        os.makedirs(os.path.join(root, "🔺️diff"), exist_ok=True)
        if case["diff"] is None:
            open(os.path.join(root, "🔺️diff", "🚫️.absent"), "w", encoding="utf-8").close()
        else:
            dump(os.path.join(root, "🔺️diff", "🔣️.json"), case["diff"])
        with open(os.path.join(root, "🦀️.rs"), "w", encoding="utf-8") as handle:
            handle.write(rust_source(case))
    print("wrote %d case bundles" % len(cases))


# endregion 🔖️Emit

# region 🔖️Registration


def patch_catalog(cases):
    """📇️ Appends each new scenario to its subset's own `🔮️oracle/🔣️.json`, re-read at edit time."""
    by_subset = {}
    for case in cases:
        by_subset.setdefault(case["subset"], []).append(case)
    for subset, group in by_subset.items():
        path = os.path.join(SUBSETS, subset, "🔮️oracle", "🔣️.json")
        with open(path, encoding="utf-8") as handle:
            text = handle.read()
        for case in group:
            anchor = '          "mutationDirectoryName": "%s",\n          "scenarios": [\n' % case["kindDirectory"]
            if text.count(anchor) != 1:
                raise SystemExit("catalog anchor for %s is not unique in %s" % (case["kindDirectory"], path))
            entry = '            {\n              "id": "%s",\n              "directoryName": "%s"\n            },\n' % (stem(case["directory"]), case["directory"])
            if entry in text:
                continue
            text = text.replace(anchor, anchor + entry)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(text)
    print("patched %d oracle catalogs" % len(by_subset))


def patch_entry(cases):
    """🦀️ Mounts each new `🦀️.rs` beside the kind's existing `#[cfg(test)]` mount in the crate entry."""
    with open(ENTRY, encoding="utf-8") as handle:
        text = handle.read()
    for case in cases:
        prefix = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/%s/🧬️schema/🧬️mutations/%s/🧪️tests/" % (case["subset"], case["kindDirectory"])
        pattern = re.compile(r'( *)#\[cfg\(test\)\]\n( *)#\[path = "' + re.escape(prefix) + r'[^"]+"\]\n( *)mod ([a-z0-9_]+);\n')
        matches = list(pattern.finditer(text))
        matches = [match for match in matches if "🧪️tests/%s/" % case["directory"] not in match.group(0)]
        if not matches:
            raise SystemExit("no existing mount anchor for %s" % case["kindDirectory"])
        anchor = matches[0]
        indent = anchor.group(1)
        block = '%s#[cfg(test)]\n%s#[path = "%s%s/🦀️.rs"]\n%smod %s;\n' % (indent, indent, prefix, case["directory"], indent, case["module"])
        if block in text:
            continue
        text = text[: anchor.end()] + block + text[anchor.end() :]
    with open(ENTRY, "w", encoding="utf-8") as handle:
        handle.write(text)
    print("mounted %d case modules" % len(cases))


def patch_taxonomy(cases):
    """🔣️ Registers each new directory in `members-of-tests`, by textual insertion after one
    existing fem3d member so the diff is exactly the added lines."""
    with open(TAXONOMY, encoding="utf-8") as handle:
        text = handle.read()
    anchor = '        "🪙️appends-an-9fdced",\n'
    if text.count(anchor) != 1:
        raise SystemExit("taxonomy anchor is not unique")
    added = []
    for case in cases:
        line = '        "%s",\n' % case["directory"]
        if line in text:
            continue
        added.append(line)
    if added:
        text = text.replace(anchor, anchor + "".join(added))
        with open(TAXONOMY, "w", encoding="utf-8") as handle:
            handle.write(text)
    print("registered %d taxonomy member names" % len(added))


# endregion 🔖️Registration


def main():
    flags = set(sys.argv[1:])
    cases = build()
    for case in cases:
        if len(case["directory"]) > 28:
            raise SystemExit("directory name too long: %s (%d)" % (case["directory"], len(case["directory"])))
        if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", stem(case["directory"])):
            raise SystemExit("scenario id is not kebab-case: %s" % case["directory"])
    names = [case["directory"] for case in cases]
    if len(set(names)) != len(names):
        raise SystemExit("duplicate case directory names")
    if not flags or "--cases" in flags:
        write_cases(cases)
    if not flags or "--patch" in flags:
        patch_catalog(cases)
        patch_entry(cases)
        patch_taxonomy(cases)
    if "--manifest" in flags:
        print(json.dumps([{k: v for k, v in case.items() if k in ("kind", "subset", "kindDirectory", "directory", "variant", "level", "code")} for case in cases], ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
