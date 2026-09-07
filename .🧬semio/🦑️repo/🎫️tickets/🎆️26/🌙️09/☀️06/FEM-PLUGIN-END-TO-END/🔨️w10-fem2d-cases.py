#!/usr/bin/env python3
"""🏗️ W10 — authors the two ADDITIONAL fixture cases every fem2d mutation kind was missing.

Each of the 25 `s.fem.fem2d@1` mutation kinds shipped exactly ONE committed vector. "Exhaustive"
needs three: the existing happy path, a second happy path on a DIFFERENT real-world model, and an
edge case that the implementation refuses (or provably leaves untouched).

* the second real-world model is `MODEL` below — a two-storey braced steel frame, 6.0 m bay,
  3.5 m storey heights, real EN commercial profiles (HEB 200 / IPE 270 / IPE 240 / CHS 88.9x4.0),
  S355 steel and C30/37 concrete, everything in SI base units (m, m², m⁴, Pa, kg/m³, N, N/m, Pa).
  It is shared by all 25 second-happy-path cases, exactly as the timber portal frame is shared by
  the subset-level differential cases.
* the edge case per kind is whatever that kind's OWN `🔺️diff/🦀️.rs` really does — an `Error`
  `mutation.target-missing`, a `Fatal` `mutation.duplicate-id`, or (for the two verbs with no
  rejection branch at all) the `Warning` `mutation.no-op` that leaves the document untouched.
  Nothing here invents a refusal the code does not implement; the gaps are reported in
  `📓️w10-fem2d-cases.md` instead.

`after` and `🔺️diff/🔣️.json` are derived here by a transcription of each kind's Rust diff builder
plus `apply_delta`'s remove→append→patch order; `🔨️w10-replay-fem2d.py` then re-derives every
triple through the plugin's own committed INDEPENDENT Python model
(`🌐️any/🧪️tests/*/🐍️.py`), which was written from the schema without reading any Rust.

Usage:
    python3 🔨️w10-fem2d-cases.py [--check]
"""

# region 🔖️Imports
import hashlib
import os
import sys
from decimal import Decimal

# endregion 🔖️Imports


# region 🔖️Paths
REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *([".."] * 7)))
SUBSETS = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "◻️2d", "🏅️standards", "🔖️1", "🪆️subsets")

SUBSET_OF = {
    "create-node": "🕸️mesh",
    "delete-node": "🕸️mesh",
    "create-element": "🕸️mesh",
    "delete-element": "🕸️mesh",
    "replace-element": "🕸️mesh",
    "create-section": "🕸️mesh",
    "delete-section": "🕸️mesh",
    "replace-section": "🕸️mesh",
    "create-region": "🕸️mesh",
    "delete-region": "🕸️mesh",
    "replace-region": "🕸️mesh",
    "create-material": "🧱️material",
    "delete-material": "🧱️material",
    "replace-material": "🧱️material",
    "create-support": "🛡️boundary",
    "delete-support": "🛡️boundary",
    "replace-support": "🛡️boundary",
    "create-load-case": "🏋️load",
    "delete-load-case": "🏋️load",
    "add-load": "🏋️load",
    "remove-load": "🏋️load",
    "change-load-case-self-weight": "🏋️load",
    "create-combination": "🏋️load",
    "delete-combination": "🏋️load",
    "update-analysis-settings": "📈️analysis",
}
"""🗂️ Which subset owns each kind's `🧬️mutations/<kind>` directory."""

KIND_DIR = {
    "create-node": "⚪️create-node",
    "delete-node": "🕳️delete-node",
    "create-element": "🧩️create-element",
    "delete-element": "🗑️delete-element",
    "replace-element": "♻️replace-element",
    "create-section": "📐️create-section",
    "delete-section": "✂️delete-section",
    "replace-section": "📏️replace-section",
    "create-region": "🗺️create-region",
    "delete-region": "🚫️delete-region",
    "replace-region": "🔄️replace-region",
    "create-material": "🌱️create-material",
    "delete-material": "🗑️delete-material",
    "replace-material": "🔁️replace-material",
    "create-support": "🛡️create-support",
    "delete-support": "🗑️delete-support",
    "replace-support": "🔁️replace-support",
    "create-load-case": "📋️create-load-case",
    "delete-load-case": "🗑️delete-load-case",
    "add-load": "➕️add-load",
    "remove-load": "➖️remove-load",
    "change-load-case-self-weight": "⚖️change-load-case-self-weight",
    "create-combination": "🔗️create-combination",
    "delete-combination": "✂️delete-combination",
    "update-analysis-settings": "🎛️update-analysis-settings",
}
"""🗂️ The on-disk directory basename of each kind."""

MEMBERS = ("nodes", "elements", "regions", "materials", "sections", "supports", "loadCases", "combinations", "analysis")
RUST_MEMBER = {
    "nodes": "nodes",
    "elements": "elements",
    "regions": "regions",
    "materials": "materials",
    "sections": "sections",
    "supports": "supports",
    "loadCases": "load_cases",
    "combinations": "combinations",
    "analysis": "analysis",
}
DIFF_SLOTS = ("artifact", "nodes", "elements", "regions", "materials", "sections", "supports", "loadCases", "combinations", "analysis", "resultSourceId", "resultMode", "resultModeIndex", "camera", "locale", "solverResultsJson", "meshPreviewJson")
# endregion 🔖️Paths


# region 🔖️Numbers
class F(float):
    """🔢️ A float that must reach JSON with an explicit decimal point and no exponent — the wire
    distinction `DslValue`'s `Number::Float` vs `Number::UInt` rests on (`🌱️value/🦀️.rs:68`)."""


def render_float(value):
    """🔢️ Shortest round-tripping decimal, never in exponent form (what Rust's `f64: Display` emits)."""
    text = format(Decimal(repr(float(value))), "f")
    return text if "." in text else text + ".0"


def dump(value, indent=0):
    """🧾️ `json.dumps(..., indent=2)`-shaped text with full control over float rendering."""
    pad, inner = " " * indent, " " * (indent + 2)
    if value is None:
        return "null"
    if value is True:
        return "true"
    if value is False:
        return "false"
    if isinstance(value, F):
        return render_float(value)
    if isinstance(value, float):
        return render_float(value)
    if isinstance(value, int):
        return str(value)
    if isinstance(value, str):
        return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'
    if isinstance(value, (list, tuple)):
        if not value:
            return "[]"
        body = ",\n".join(inner + dump(item, indent + 2) for item in value)
        return "[\n" + body + "\n" + pad + "]"
    if isinstance(value, dict):
        if not value:
            return "{}"
        body = ",\n".join(inner + dump(key, 0) + ": " + dump(item, indent + 2) for key, item in value.items())
        return "{\n" + body + "\n" + pad + "}"
    raise AssertionError("cannot render %r" % (value,))


# endregion 🔖️Numbers


# region 🔖️Model
def node(identifier, x, y):
    return {"id": identifier, "x": F(x), "y": F(y)}


def beam(identifier, start, end, material, section):
    return {"kind": "beam", "id": identifier, "start": start, "end": end, "materialId": material, "sectionId": section}


def bar(identifier, start, end, material, section):
    return {"kind": "bar", "id": identifier, "start": start, "end": end, "materialId": material, "sectionId": section}


def material(identifier, name, e, nu, rho):
    return {"id": identifier, "name": name, "e": F(e), "nu": F(nu), "rho": F(rho)}


def section(identifier, name, area, iy):
    return {"id": identifier, "name": name, "area": F(area), "iy": F(iy)}


def support(identifier, node_id, fixed):
    return {"id": identifier, "nodeId": node_id, "fixed": list(fixed)}


def nodal(identifier, node_id, dof, value):
    return {"kind": "nodal", "id": identifier, "nodeId": node_id, "dof": dof, "value": F(value)}


def udl(identifier, element_id, wx, wy):
    return {"kind": "memberUdl", "id": identifier, "elementId": element_id, "wx": F(wx), "wy": F(wy)}


def area(identifier, region_id, pressure):
    return {"kind": "area", "id": identifier, "regionId": region_id, "pressure": F(pressure)}


def load_case(identifier, name, loads, self_weight):
    return {"id": identifier, "name": name, "loads": list(loads), "selfWeight": self_weight}


def region(identifier, name, outline, holes, thickness, material_id, mesh_size):
    return {
        "id": identifier,
        "name": name,
        "outline": [[F(x), F(y)] for x, y in outline],
        "holes": [[[F(x), F(y)] for x, y in hole] for hole in holes],
        "thickness": F(thickness),
        "materialId": material_id,
        "meshSize": F(mesh_size),
    }


def term(case_id, factor):
    return {"caseId": case_id, "factor": F(factor)}


def combination(identifier, name, terms):
    return {"id": identifier, "name": name, "terms": list(terms)}


WINDOW = [(2.4, 1.0), (3.6, 1.0), (3.6, 2.4), (2.4, 2.4)]
WIDER_WINDOW = [(2.2, 1.0), (3.8, 1.0), (3.8, 2.4), (2.2, 2.4)]

MODEL = {
    "nodes": [
        node("n1", 0.0, 0.0),
        node("n2", 6.0, 0.0),
        node("n3", 0.0, 3.5),
        node("n4", 6.0, 3.5),
        node("n5", 0.0, 7.0),
        node("n6", 6.0, 7.0),
        node("n7", 8.0, 7.0),
    ],
    "elements": [
        beam("c1", "n1", "n3", "steel_s355", "heb200"),
        beam("c2", "n2", "n4", "steel_s355", "heb200"),
        beam("c3", "n3", "n5", "steel_s355", "heb200"),
        beam("c4", "n4", "n6", "steel_s355", "heb200"),
        beam("b1", "n3", "n4", "steel_s355", "ipe270"),
        beam("b2", "n5", "n6", "steel_s355", "ipe240"),
        bar("br1", "n2", "n3", "steel_s355", "chs889"),
    ],
    "regions": [
        region("wall1", "Infill Wall Panel", [(0.0, 0.0), (6.0, 0.0), (6.0, 3.5), (0.0, 3.5)], [WINDOW], 0.2, "concrete_c30", 0.5),
        region("panel_spare", "Spare Side Panel", [(6.0, 3.5), (8.0, 3.5), (8.0, 7.0), (6.0, 7.0)], [], 0.15, "concrete_c30", 0.5),
    ],
    "materials": [
        material("steel_s355", "Steel S355", 210000000000.0, 0.3, 7850.0),
        material("concrete_c30", "Concrete C30/37", 33000000000.0, 0.2, 2500.0),
        material("steel_s235_spare", "Steel S235", 210000000000.0, 0.3, 7850.0),
    ],
    "sections": [
        section("heb200", "HEB 200", 0.007808, 0.00005696),
        section("ipe270", "IPE 270", 0.004595, 0.0000579),
        section("ipe240", "IPE 240", 0.003912, 0.00003892),
        section("chs889", "CHS 88.9x4.0", 0.0010669, 0.0000009634),
        section("ipe200_spare", "IPE 200", 0.002848, 0.00001943),
    ],
    "supports": [
        support("sup1", "n1", ["Tx", "Ty", "Rz"]),
        support("sup2", "n2", ["Tx", "Ty", "Rz"]),
        support("sup_tie", "n6", ["Tx"]),
    ],
    "loadCases": [
        load_case("dead", "Dead", [udl("ld1", "b1", 0.0, -18500.0), udl("ld2", "b2", 0.0, -9200.0)], True),
        load_case("live", "Imposed", [udl("ll1", "b1", 0.0, -15000.0)], False),
        load_case("wind", "Wind", [nodal("lw1", "n5", "Tx", 12500.0), area("lw2", "wall1", 640.0)], False),
        load_case("snow_spare", "Snow (spare)", [udl("ls1", "b2", 0.0, -4800.0)], False),
    ],
    "combinations": [
        combination("uls1", "ULS 6.10b", [term("dead", 1.35), term("live", 1.5)]),
        combination("sls1", "SLS characteristic", [term("dead", 1.0), term("live", 1.0), term("wind", 0.6)]),
        combination("uls_spare", "ULS 6.10a (spare)", [term("dead", 1.35), term("wind", 1.5)]),
    ],
    "analysis": {"modalCount": 6, "bucklingCount": 4, "deformationScale": F(120.0)},
}
"""🏢️ Two-storey braced steel frame — the second real-world fem2d model this wave introduces.

Geometry: one 6.0 m bay, storey heights 3.5 m, plus an 2.0 m canopy tip at roof level. Columns are
HEB 200, the first-floor beam IPE 270, the roof beam IPE 240 and the lower-storey tension brace a
CHS 88.9x4.0, all S355. The bay is infilled with a 200 mm C30/37 panel carrying a 1.2 x 1.4 m
window opening. Loads are characteristic EN values: 18.5 kN/m dead and 15.0 kN/m imposed on the
floor beam, 9.2 kN/m on the roof, a 12.5 kN wind point load at roof level and 640 Pa wind pressure
on the panel. `n7`, `steel_s235_spare`, `ipe200_spare`, `sup_tie`, `panel_spare`, `snow_spare` and
`uls_spare` are trailing spares: no `create-` verb in this vocabulary carries an index, so a
`delete-` is exactly invertible only for a TRAILING record — the same limitation the timber portal
frame handles the same way."""
# endregion 🔖️Model


# region 🔖️Diff
def find(items, identifier):
    for at, item in enumerate(items):
        if item["id"] == identifier:
            return at
    return None


def empty_diff():
    return {slot: None for slot in DIFF_SLOTS}


def delta(added=None, removed=None, patched=None):
    return {"added": added or [], "removed": removed or [], "patched": patched or [], "reordered": None}


def rejection(code, message, target, level):
    return None, [{"level": level, "code": code, "message": message, "target": list(target)}]


def build_diff(mutation, base):
    """🔺️ A transcription of the 25 `🔺️diff/🦀️.rs` builders — returns `(diff, messages)`, with
    `diff = None` standing for the `D::default()` every `error`/`fatal`/`empty` outcome carries."""
    tag = mutation["mutation"]
    if tag == "createNode":
        item = mutation["node"]
        if find(base["nodes"], item["id"]) is not None:
            return rejection("mutation.duplicate-id", 'A node with id "%s" already exists.' % item["id"], [item["id"]], "fatal")
        return {**empty_diff(), "nodes": delta(added=[item])}, []
    if tag == "deleteNode":
        if find(base["nodes"], mutation["id"]) is None:
            return rejection("mutation.target-missing", 'Node "%s" does not exist.' % mutation["id"], [mutation["id"]], "error")
        return {**empty_diff(), "nodes": delta(removed=[mutation["id"]])}, []
    if tag == "createElement":
        item = mutation["element"]
        if find(base["elements"], item["id"]) is not None:
            return rejection("mutation.duplicate-id", 'An element with id "%s" already exists.' % item["id"], [item["id"]], "fatal")
        for key, collection, label in (("start", "nodes", "Node"), ("end", "nodes", "Node"), ("materialId", "materials", "Material"), ("sectionId", "sections", "Section")):
            if find(base[collection], item[key]) is None:
                return rejection("mutation.target-missing", '%s "%s" does not exist.' % (label, item[key]), [item[key]], "error")
        return {**empty_diff(), "elements": delta(added=[item])}, []
    if tag == "deleteElement":
        if find(base["elements"], mutation["id"]) is None:
            return rejection("mutation.target-missing", 'Element "%s" does not exist.' % mutation["id"], [mutation["id"]], "error")
        return {**empty_diff(), "elements": delta(removed=[mutation["id"]])}, []
    if tag == "replaceElement":
        at = find(base["elements"], mutation["id"])
        if at is None:
            return rejection("mutation.target-missing", 'Element "%s" does not exist.' % mutation["id"], [mutation["id"]], "error")
        if base["elements"][at] == mutation["newElement"]:
            return None, [{"level": "warn", "code": "mutation.no-op", "message": 'Element "%s" is already equal to the replacement value.' % mutation["id"], "target": []}]
        return {**empty_diff(), "elements": delta(patched=[{"id": mutation["id"], "item": mutation["newElement"]}])}, []
    if tag in ("createMaterial", "createSection", "createSupport", "createRegion"):
        noun = {"createMaterial": ("material", "materials", "A material"), "createSection": ("section", "sections", "A section"), "createSupport": ("support", "supports", "A support"), "createRegion": ("region", "regions", "A region")}[tag]
        argument, collection, article = noun
        item = mutation[argument]
        if find(base[collection], item["id"]) is not None:
            return rejection("mutation.duplicate-id", '%s with id "%s" already exists.' % (article, item["id"]), [item["id"]], "fatal")
        if tag == "createSupport" and find(base["nodes"], item["nodeId"]) is None:
            return rejection("mutation.target-missing", 'Node "%s" does not exist.' % item["nodeId"], [item["nodeId"]], "error")
        if tag == "createRegion" and find(base["materials"], item["materialId"]) is None:
            return rejection("mutation.target-missing", 'Material "%s" does not exist.' % item["materialId"], [item["materialId"]], "error")
        return {**empty_diff(), collection: delta(added=[item])}, []
    if tag in ("deleteMaterial", "deleteSection", "deleteSupport", "deleteRegion"):
        collection, label = {"deleteMaterial": ("materials", "Material"), "deleteSection": ("sections", "Section"), "deleteSupport": ("supports", "Support"), "deleteRegion": ("regions", "Region")}[tag]
        if find(base[collection], mutation["id"]) is None:
            return rejection("mutation.target-missing", '%s "%s" does not exist.' % (label, mutation["id"]), [mutation["id"]], "error")
        return {**empty_diff(), collection: delta(removed=[mutation["id"]])}, []
    if tag in ("replaceMaterial", "replaceSection", "replaceSupport", "replaceRegion"):
        collection, label, argument = {
            "replaceMaterial": ("materials", "Material", "newMaterial"),
            "replaceSection": ("sections", "Section", "newSection"),
            "replaceSupport": ("supports", "Support", "newSupport"),
            "replaceRegion": ("regions", "Region", "newRegion"),
        }[tag]
        at = find(base[collection], mutation["id"])
        if at is None:
            return rejection("mutation.target-missing", '%s "%s" does not exist.' % (label, mutation["id"]), [mutation["id"]], "error")
        if base[collection][at] == mutation[argument]:
            return None, [{"level": "warn", "code": "mutation.no-op", "message": '%s "%s" is already equal to the replacement value.' % (label, mutation["id"]), "target": []}]
        return {**empty_diff(), collection: delta(patched=[{"id": mutation["id"], "item": mutation[argument]}])}, []
    if tag == "createLoadCase":
        item = mutation["loadCase"]
        if find(base["loadCases"], item["id"]) is not None:
            return rejection("mutation.duplicate-id", 'A load case with id "%s" already exists.' % item["id"], [item["id"]], "fatal")
        for load in item["loads"]:
            reference, collection, label = {"nodal": ("nodeId", "nodes", "Node"), "memberUdl": ("elementId", "elements", "Element"), "area": ("regionId", "regions", "Region")}[load["kind"]]
            if find(base[collection], load[reference]) is None:
                return rejection("mutation.target-missing", '%s "%s" does not exist.' % (label, load[reference]), [load[reference]], "error")
        return {**empty_diff(), "loadCases": delta(added=[item])}, []
    if tag == "deleteLoadCase":
        if find(base["loadCases"], mutation["id"]) is None:
            return rejection("mutation.target-missing", 'Load case "%s" does not exist.' % mutation["id"], [mutation["id"]], "error")
        return {**empty_diff(), "loadCases": delta(removed=[mutation["id"]])}, []
    if tag == "addLoad":
        at = find(base["loadCases"], mutation["caseId"])
        if at is None:
            return rejection("mutation.target-missing", 'Load case "%s" does not exist.' % mutation["caseId"], [mutation["caseId"]], "error")
        existing = base["loadCases"][at]
        if find(existing["loads"], mutation["load"]["id"]) is not None:
            return None, [{"level": "warn", "code": "mutation.no-op", "message": 'Load "%s" already exists in case "%s".' % (mutation["load"]["id"], mutation["caseId"]), "target": []}]
        item = {**existing, "loads": existing["loads"] + [mutation["load"]]}
        return {**empty_diff(), "loadCases": delta(patched=[{"id": mutation["caseId"], "item": item}])}, []
    if tag == "removeLoad":
        at = find(base["loadCases"], mutation["caseId"])
        if at is None:
            return rejection("mutation.target-missing", 'Load case "%s" does not exist.' % mutation["caseId"], [mutation["caseId"]], "error")
        existing = base["loadCases"][at]
        if find(existing["loads"], mutation["loadId"]) is None:
            return rejection("mutation.target-missing", 'Load "%s" does not exist in case "%s".' % (mutation["loadId"], mutation["caseId"]), [mutation["loadId"]], "error")
        item = {**existing, "loads": [load for load in existing["loads"] if load["id"] != mutation["loadId"]]}
        return {**empty_diff(), "loadCases": delta(patched=[{"id": mutation["caseId"], "item": item}])}, []
    if tag == "changeLoadCaseSelfWeight":
        at = find(base["loadCases"], mutation["caseId"])
        if at is None:
            return rejection("mutation.target-missing", 'Load case "%s" does not exist.' % mutation["caseId"], [mutation["caseId"]], "error")
        existing = base["loadCases"][at]
        if existing["selfWeight"] == mutation["newSelfWeight"]:
            return None, [{"level": "warn", "code": "mutation.no-op", "message": 'Load case "%s" self-weight is already %s.' % (mutation["caseId"], str(mutation["newSelfWeight"]).lower()), "target": []}]
        item = {**existing, "selfWeight": mutation["newSelfWeight"]}
        return {**empty_diff(), "loadCases": delta(patched=[{"id": mutation["caseId"], "item": item}])}, []
    if tag == "createCombination":
        item = mutation["combination"]
        if find(base["combinations"], item["id"]) is not None:
            return rejection("mutation.duplicate-id", 'A combination with id "%s" already exists.' % item["id"], [item["id"]], "fatal")
        for entry in item["terms"]:
            if find(base["loadCases"], entry["caseId"]) is None and find(base["combinations"], entry["caseId"]) is None:
                return rejection("mutation.target-missing", 'Load case or combination "%s" does not exist.' % entry["caseId"], [entry["caseId"]], "error")
        return {**empty_diff(), "combinations": delta(added=[item])}, []
    if tag == "deleteCombination":
        if find(base["combinations"], mutation["id"]) is None:
            return rejection("mutation.target-missing", 'Combination "%s" does not exist.' % mutation["id"], [mutation["id"]], "error")
        return {**empty_diff(), "combinations": delta(removed=[mutation["id"]])}, []
    if tag == "updateAnalysisSettings":
        if mutation["settings"] == base["analysis"]:
            return None, [{"level": "warn", "code": "mutation.no-op", "message": "Analysis settings are unchanged.", "target": []}]
        return {**empty_diff(), "analysis": mutation["settings"]}, []
    raise AssertionError("unknown mutation tag %r" % tag)


def apply_diff(base, diff):
    """▶️ `Fem2dDiff::apply` — per collection: drop `removed`, append `added`, patch in place."""
    result = {name: [dict(item) for item in base[name]] if name != "analysis" else dict(base[name]) for name in MEMBERS}
    if diff is None:
        return result
    for name in MEMBERS[:-1]:
        piece = diff[name]
        if piece is None:
            continue
        items = [item for item in result[name] if item["id"] not in piece["removed"]]
        for added in piece["added"]:
            at = find(items, added["id"])
            if at is None:
                items.append(added)
            else:
                items[at] = added
        for entry in piece["patched"]:
            at = find(items, entry["id"])
            if at is not None:
                items[at] = entry["item"]
        result[name] = items
    if diff["analysis"] is not None:
        result["analysis"] = diff["analysis"]
    return result


# endregion 🔖️Diff


# region 🔖️Cases
def suffix(long_name):
    """🔢️ The 6-hex disambiguator the Windows-path truncation convention appends."""
    return hashlib.sha1(long_name.encode("utf-8")).hexdigest()[:6]


def case(kind, family, emoji, slug, long_name, mutation, written, note, extra=()):
    identity = "%s-%s" % (slug, suffix(long_name))
    return {
        "kind": kind,
        "family": family,
        "id": identity,
        "directory": emoji + identity,
        "module": "tests_" + long_name.replace("-", "_"),
        "long": long_name,
        "mutation": mutation,
        "written": written,
        "note": note,
        "extra": list(extra),
    }


ROOF_BEAM_IPE270 = beam("b2", "n5", "n6", "steel_s355", "ipe270")
CRACKED_C30 = material("concrete_c30", "Concrete C30/37 (cracked)", 16500000000.0, 0.2, 2500.0)
THICK_CHS = section("chs889", "CHS 88.9x5.0", 0.001318, 0.00000116376)
PINNED_BASE = support("sup1", "n1", ["Tx", "Ty"])
WIDENED_WALL = region("wall1", "Infill Wall Panel", [(0.0, 0.0), (6.0, 0.0), (6.0, 3.5), (0.0, 3.5)], [WIDER_WINDOW], 0.2, "concrete_c30", 0.4)

CASES = [
    case("create-node", "happy", "🏢️", "appends-the-canopy", "appends-the-canopy-strut-head-node-to-the-steel-frame",
         {"mutation": "createNode", "node": node("n8", 8.0, 3.5)}, "nodes",
         "🏢️ The canopy strut head is appended at the tail of the node table — `create-node` never inserts."),
    case("create-node", "reject", "🚫️", "rejects-a-duplicate", "rejects-a-duplicate-node-id-on-the-steel-frame",
         {"mutation": "createNode", "node": node("n3", 0.5, 3.5)}, "nodes",
         "🚫️ A colliding node id is FATAL `mutation.duplicate-id`, not the Error-level `target-missing` the delete/replace verbs raise — a duplicate identity is an invariant breach, not a miss."),
    case("delete-node", "happy", "🗑️", "drops-the-spare", "drops-the-spare-canopy-node-from-the-steel-frame",
         {"mutation": "deleteNode", "id": "n7"}, "nodes",
         "🗑️ `n7` is the TRAILING node and is referenced by nothing, so the `create-node` inverse — which always appends — restores the table exactly."),
    case("delete-node", "reject", "⛔️", "rejects-a-missing", "rejects-deleting-a-node-the-steel-frame-never-had",
         {"mutation": "deleteNode", "id": "n42"}, "nodes",
         "⛔️ `delete-node` guards only the target's EXISTENCE. It has no cascade and no referential guard at all — deleting a node an element still names is accepted (see the sibling `🚫️removes-node-n3-without-6eab3f` vector); only a node that is not there at all is refused."),
    case("create-element", "happy", "📐️", "braces-the-upper", "braces-the-upper-storey-with-a-chs-diagonal",
         {"mutation": "createElement", "element": bar("br2", "n4", "n5", "steel_s355", "chs889")}, "elements",
         "📐️ A second CHS tension brace closes the upper storey. All four references — both nodes, the material and the section — resolve, which is exactly what `create-element` checks."),
    case("create-element", "reject", "🚫️", "rejects-a-dangling", "rejects-an-element-whose-start-node-is-missing",
         {"mutation": "createElement", "element": bar("br9", "n42", "n5", "steel_s355", "chs889")}, "elements",
         "🚫️ `create-element` is the only `create-` verb in this vocabulary that validates FOUR references; the `start` node is checked first, so `n42` is the address the diagnostic carries even though the element is otherwise well formed."),
    case("delete-element", "happy", "✂️", "cuts-the-lower", "cuts-the-lower-storey-bracing-diagonal",
         {"mutation": "deleteElement", "id": "br1"}, "elements",
         "✂️ `br1` is the TRAILING element, so the `create-element` inverse — which appends — lands it back in its own slot."),
    case("delete-element", "reject", "⛔️", "rejects-a-missing", "rejects-deleting-an-element-the-steel-frame-never-had",
         {"mutation": "deleteElement", "id": "e42"}, "elements",
         "⛔️ Existence is the only guard: `delete-element` does not look at the loads that name the element, so a member UDL can be orphaned by a successful delete."),
    case("replace-element", "happy", "🔧️", "regrades-the-roof", "regrades-the-roof-beam-onto-the-ipe270-profile",
         {"mutation": "replaceElement", "id": "b2", "newElement": ROOF_BEAM_IPE270}, "elements",
         "🔧️ Upgrading the roof beam from IPE 240 to IPE 270 is a whole-entity patch in place — the element keeps its id and its position in the collection."),
    case("replace-element", "reject", "⛔️", "rejects-a-missing", "rejects-replacing-an-element-the-steel-frame-never-had",
         {"mutation": "replaceElement", "id": "e42", "newElement": bar("e42", "n1", "n2", "steel_s355", "chs889")}, "elements",
         "⛔️ `replace-element` validates only that the TARGET exists; it never checks the replacement's own node/material/section references, so a well-addressed replace can introduce dangling ones."),
    case("create-material", "happy", "🏗️", "adds-the-c25-slab", "adds-the-c25-30-material-for-the-ground-slab",
         {"mutation": "createMaterial", "material": material("concrete_c25", "Concrete C25/30", 31000000000.0, 0.2, 2500.0)}, "materials",
         "🏗️ C25/30 for the ground slab: E_cm = 31 GPa, nu = 0.2, rho = 2500 kg/m³ — appended at the tail."),
    case("create-material", "reject", "🚫️", "rejects-a-duplicate", "rejects-a-duplicate-material-id-on-the-steel-frame",
         {"mutation": "createMaterial", "material": material("steel_s355", "Steel S355 duplicate", 210000000000.0, 0.3, 7850.0)}, "materials",
         "🚫️ An id already in the table is FATAL, whatever the payload's other fields say — the guard compares identity only."),
    case("delete-material", "happy", "🗑️", "drops-the-spare", "drops-the-unreferenced-s235-material-from-the-steel-frame",
         {"mutation": "deleteMaterial", "id": "steel_s235_spare"}, "materials",
         "🗑️ The S235 spare is trailing and referenced by no element and no region, so this delete leaves nothing dangling and inverts exactly."),
    case("delete-material", "reject", "⛔️", "rejects-a-missing", "rejects-deleting-a-material-the-steel-frame-never-had",
         {"mutation": "deleteMaterial", "id": "aluminium"}, "materials",
         "⛔️ Only existence is guarded. Deleting `steel_s355`, which seven elements name, is accepted — this vocabulary has no referential-integrity verb at all."),
    case("replace-material", "happy", "📉️", "cracks-the-c30", "cracks-the-c30-37-stiffness-in-half-for-the-infill-panel",
         {"mutation": "replaceMaterial", "id": "concrete_c30", "newMaterial": CRACKED_C30}, "materials",
         "📉️ Halving E_cm to 16.5 GPa is the usual cracked-section allowance for an RC infill panel; density and Poisson's ratio are unchanged."),
    case("replace-material", "reject", "⛔️", "rejects-a-missing", "rejects-replacing-a-material-the-steel-frame-never-had",
         {"mutation": "replaceMaterial", "id": "aluminium", "newMaterial": material("aluminium", "EN AW-6082 T6", 70000000000.0, 0.33, 2700.0)}, "materials",
         "⛔️ `replace-material` addresses an existing slot; a missing id is an Error-level `mutation.target-missing`, never an implicit create."),
    case("create-section", "happy", "➕️", "adds-the-hea220", "adds-the-hea220-profile-to-the-steel-frame",
         {"mutation": "createSection", "section": section("hea220", "HEA 220", 0.006434, 0.0000541)}, "sections",
         "➕️ HEA 220: A = 64.34 cm², I_y = 5410 cm⁴, both carried in SI base units."),
    case("create-section", "reject", "🚫️", "rejects-a-duplicate", "rejects-a-duplicate-section-id-on-the-steel-frame",
         {"mutation": "createSection", "section": section("heb200", "HEB 200 duplicate", 0.007808, 0.00005696)}, "sections",
         "🚫️ Even a byte-identical payload is refused: `create-section` rejects on identity, not on value."),
    case("delete-section", "happy", "✂️", "drops-the-spare", "drops-the-unreferenced-ipe200-section-from-the-steel-frame",
         {"mutation": "deleteSection", "id": "ipe200_spare"}, "sections",
         "✂️ The IPE 200 spare is trailing and unreferenced, the one shape a `create-section` inverse can restore exactly."),
    case("delete-section", "reject", "⛔️", "rejects-a-missing", "rejects-deleting-a-section-the-steel-frame-never-had",
         {"mutation": "deleteSection", "id": "hea300"}, "sections",
         "⛔️ Existence is the whole guard; deleting a section six elements still name would be accepted."),
    case("replace-section", "happy", "🛠️", "thickens-the-chs", "thickens-the-chs-brace-wall-to-five-millimetres",
         {"mutation": "replaceSection", "id": "chs889", "newSection": THICK_CHS}, "sections",
         "🛠️ CHS 88.9x4.0 to 88.9x5.0: A rises to 13.18 cm² and I_y to 116.4 cm⁴, the outer diameter unchanged, so the id still reads true."),
    case("replace-section", "reject", "⛔️", "rejects-a-missing", "rejects-replacing-a-section-the-steel-frame-never-had",
         {"mutation": "replaceSection", "id": "hea300", "newSection": section("hea300", "HEA 300", 0.01125, 0.00018260)}, "sections",
         "⛔️ A replace never creates: the missing id is reported as an Error-level `mutation.target-missing` and the section table is untouched."),
    case("create-support", "happy", "🔻️", "props-the-canopy", "props-the-canopy-tip-on-a-vertical-roller",
         {"mutation": "createSupport", "support": support("sup_canopy", "n7", ["Ty"])}, "supports",
         "🔻️ A vertical roller under the canopy tip. `create-support` is one of only two `create-` verbs that validate a foreign reference — the node must exist, and `n7` does."),
    case("create-support", "reject", "🚫️", "rejects-a-dangling", "rejects-a-support-on-a-node-the-steel-frame-never-had",
         {"mutation": "createSupport", "support": support("sup_ghost", "n42", ["Tx", "Ty"])}, "supports",
         "🚫️ The id is free, so the duplicate guard passes and the NODE guard is what fires: an Error-level `mutation.target-missing` addressing `n42`."),
    case("delete-support", "happy", "🕊️", "frees-the-roof-tie", "frees-the-roof-level-lateral-tie-of-the-steel-frame",
         {"mutation": "deleteSupport", "id": "sup_tie"}, "supports",
         "🕊️ Releasing the roof-level lateral tie leaves the frame on its two base fixities. `sup_tie` is trailing, so `create-support` restores it in place."),
    case("delete-support", "reject", "⛔️", "rejects-a-missing", "rejects-deleting-a-support-the-steel-frame-never-had",
         {"mutation": "deleteSupport", "id": "sup9"}, "supports",
         "⛔️ `delete-support` is the plainest verb in the vocabulary: one existence guard, no cascade, no second branch."),
    case("replace-support", "happy", "🔩️", "pins-the-left-base", "pins-the-left-column-base-by-releasing-its-rotation",
         {"mutation": "replaceSupport", "id": "sup1", "newSupport": PINNED_BASE}, "supports",
         "🔩️ Dropping `Rz` turns the left column base from a full fixity into a pin — the single most common modelling change on a frame like this."),
    case("replace-support", "reject", "⛔️", "rejects-a-missing", "rejects-replacing-a-support-the-steel-frame-never-had",
         {"mutation": "replaceSupport", "id": "sup9", "newSupport": support("sup9", "n5", ["Ty"])}, "supports",
         "⛔️ Only the target id is validated; the replacement's own `nodeId` is never checked, so a support can be re-pointed at a node that does not exist."),
    case("create-region", "happy", "🏢️", "infills-the-upper", "infills-the-upper-storey-bay-with-a-concrete-panel",
         {"mutation": "createRegion", "region": region("panel_l2", "Upper Infill Panel", [(0.0, 3.5), (6.0, 3.5), (6.0, 7.0), (0.0, 7.0)], [], 0.2, "concrete_c30", 0.5)}, "regions",
         "🏢️ A second 200 mm C30/37 infill panel over the upper storey. `create-region` validates the material reference and nothing else — the outline itself is never inspected."),
    case("create-region", "reject", "🚫️", "rejects-a-duplicate", "rejects-a-duplicate-region-id-on-the-steel-frame",
         {"mutation": "createRegion", "region": region("wall1", "Infill Wall Panel duplicate", [(0.0, 0.0), (6.0, 0.0), (6.0, 3.5), (0.0, 3.5)], [], 0.2, "concrete_c30", 0.5)}, "regions",
         "🚫️ The duplicate-id guard runs BEFORE the material guard, so this payload is FATAL even though its material resolves. A self-intersecting outline, by contrast, is accepted — there is no geometric validation in this verb."),
    case("delete-region", "happy", "🧹️", "drops-the-spare", "drops-the-spare-side-panel-region-from-the-steel-frame",
         {"mutation": "deleteRegion", "id": "panel_spare"}, "regions",
         "🧹️ The spare side panel is trailing and carries no area load, so nothing is orphaned and `create-region` inverts it exactly."),
    case("delete-region", "reject", "⛔️", "rejects-a-missing", "rejects-deleting-a-region-the-steel-frame-never-had",
         {"mutation": "deleteRegion", "id": "panel9"}, "regions",
         "⛔️ Existence only. Deleting `wall1`, which the wind case's area load names, is accepted and leaves that load pointing at nothing."),
    case("replace-region", "happy", "🪟️", "widens-the-window", "widens-the-window-opening-in-the-infill-wall-panel",
         {"mutation": "replaceRegion", "id": "wall1", "newRegion": WIDENED_WALL}, "regions",
         "🪟️ The opening grows from 1.2 x 1.4 m to 1.6 x 1.4 m and the mesh is refined to 0.4 m — a whole-entity patch, outline, holes and mesh size together."),
    case("replace-region", "reject", "⛔️", "rejects-a-missing", "rejects-replacing-a-region-the-steel-frame-never-had",
         {"mutation": "replaceRegion", "id": "panel9", "newRegion": region("panel9", "Ghost Panel", [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], [], 0.1, "concrete_c30", 0.5)}, "regions",
         "⛔️ `replace-region` checks the target id and, when it resolves, only whether the value is already equal; the replacement's own material reference and outline are never validated."),
    case("create-load-case", "happy", "❄️", "appends-the-snow", "appends-the-snow-case-over-the-roof-beam",
         {"mutation": "createLoadCase", "loadCase": load_case("snow", "Snow", [udl("lsn1", "b2", 0.0, -3600.0)], False)}, "loadCases",
         "❄️ A 3.6 kN/m characteristic snow UDL on the roof beam. `create-load-case` validates every load it carries — this one names `b2`, which exists."),
    case("create-load-case", "reject", "🚫️", "rejects-a-dangling", "rejects-a-load-case-whose-udl-names-a-missing-element",
         {"mutation": "createLoadCase", "loadCase": load_case("seismic", "Seismic", [udl("lse1", "e42", 0.0, -2000.0)], False)}, "loadCases",
         "🚫️ `create-load-case` is the ONLY load verb that validates the references its loads carry, and it does so per variant — `nodal` against nodes, `memberUdl` against elements, `area` against regions. Its `add-load` sibling performs no such check."),
    case("delete-load-case", "happy", "🗑️", "drops-the-spare", "drops-the-spare-snow-case-with-its-single-load",
         {"mutation": "deleteLoadCase", "id": "snow_spare"}, "loadCases",
         "🗑️ Deleting a case takes its loads with it — they have no collection of their own. `snow_spare` is trailing and named by no combination, so nothing is orphaned."),
    case("delete-load-case", "reject", "⛔️", "rejects-a-missing", "rejects-deleting-a-load-case-the-steel-frame-never-had",
         {"mutation": "deleteLoadCase", "id": "seismic"}, "loadCases",
         "⛔️ Existence only: deleting `dead`, which two combinations name, is accepted and leaves those terms dangling."),
    case("add-load", "happy", "💨️", "pushes-a-wind-load", "pushes-a-wind-point-load-onto-the-first-floor-of-the-steel-frame",
         {"mutation": "addLoad", "caseId": "wind", "load": nodal("lw3", "n3", "Tx", 7500.0)}, "loadCases",
         "💨️ Loads have no collection of their own: attaching one re-emits the WHOLE owning case as a single `loadCases.patched` entry, never a nested load delta."),
    case("add-load", "reject", "🚫️", "rejects-a-missing", "rejects-adding-a-load-to-a-load-case-that-does-not-exist",
         {"mutation": "addLoad", "caseId": "seismic", "load": nodal("lse1", "n5", "Tx", 9000.0)}, "loadCases",
         "🚫️ The owning case is the only reference `add-load` validates — the load's own `nodeId`/`elementId`/`regionId` is never checked, unlike `create-load-case`'s. A load id already present in the case is a Warning-level no-op, not a rejection."),
    case("remove-load", "happy", "✂️", "strips-the-roof-udl", "strips-the-trailing-roof-udl-from-the-dead-case",
         {"mutation": "removeLoad", "caseId": "dead", "loadId": "ld2"}, "loadCases",
         "✂️ `ld2` is the TRAILING load of the dead case, so the `add-load` inverse — which appends — puts it back in its own slot."),
    case("remove-load", "reject", "⛔️", "rejects-a-missing", "rejects-removing-a-load-the-dead-case-never-carried",
         {"mutation": "removeLoad", "caseId": "dead", "loadId": "lz9"}, "loadCases",
         "⛔️ `remove-load` guards twice — the case, then the load within it. This case resolves, so it is the SECOND guard that fires and the diagnostic addresses the load id, not the case id."),
    case("change-load-case-self-weight", "happy", "🏋️", "switches-self", "switches-self-weight-on-for-the-imposed-case",
         {"mutation": "changeLoadCaseSelfWeight", "caseId": "live", "newSelfWeight": True}, "loadCases",
         "🏋️ Self-weight is a flag on the case, so flipping it re-emits the whole case as one patch entry with its loads unchanged."),
    case("change-load-case-self-weight", "noop", "🔁️", "keeps-self-weight", "keeps-self-weight-on-for-the-dead-case",
         {"mutation": "changeLoadCaseSelfWeight", "caseId": "dead", "newSelfWeight": True}, "loadCases",
         "🔁️ The dead case already carries self-weight, so this is `change-load-case-self-weight`'s no-op branch: APPLIED with a Warning, an empty diff, and a document that does not move. `change-load-case-self-weight` has no Fatal branch at all."),
    case("create-combination", "happy", "➕️", "appends-the-6-10a", "appends-the-six-ten-a-combination-over-dead-and-snow",
         {"mutation": "createCombination", "combination": combination("uls2", "ULS 6.10a", [term("dead", 1.35), term("snow_spare", 1.5)])}, "combinations",
         "➕️ EN 1990 expression 6.10a over the dead and snow cases. Every term is validated against the load cases AND the existing combinations, so a nested combination is legal."),
    case("create-combination", "reject", "🚫️", "rejects-a-dangling", "rejects-a-combination-term-naming-a-load-case-that-is-absent",
         {"mutation": "createCombination", "combination": combination("uls9", "ULS with seismic", [term("dead", 1.0), term("seismic", 1.0)])}, "combinations",
         "🚫️ Terms are checked in order, so the first term resolves and the diagnostic addresses `seismic`. A combination that names ITSELF is refused for the same reason — its own record is not in the base yet."),
    case("delete-combination", "happy", "🗑️", "drops-the-spare", "drops-the-spare-uls-combination-from-the-steel-frame",
         {"mutation": "deleteCombination", "id": "uls_spare"}, "combinations",
         "🗑️ The spare ULS combination is trailing and no other combination nests it, so `create-combination` restores it exactly."),
    case("delete-combination", "reject", "⛔️", "rejects-a-missing", "rejects-deleting-a-combination-the-steel-frame-never-had",
         {"mutation": "deleteCombination", "id": "uls9"}, "combinations",
         "⛔️ Existence only — a combination nested by another one can still be deleted, leaving the outer term dangling."),
    case("update-analysis-settings", "happy", "🎚️", "raises-the-mode", "raises-the-mode-counts-and-tightens-the-deformation-scale",
         {"mutation": "updateAnalysisSettings", "settings": {"modalCount": 10, "bucklingCount": 6, "deformationScale": F(80.0)}}, "analysis",
         "🎚️ Analysis settings are one inseparable facet: the verb writes the whole record, so the diff carries a value rather than a collection delta."),
    case("update-analysis-settings", "noop", "🔁️", "keeps-the-analysis", "keeps-the-analysis-settings-exactly-as-they-are",
         {"mutation": "updateAnalysisSettings", "settings": {"modalCount": 6, "bucklingCount": 4, "deformationScale": F(120.0)}}, "analysis",
         "🔁️ `update-analysis-settings` is the ONE kind in this vocabulary with no rejection branch whatsoever: its only guard is the equality no-op pinned here. Nothing validates the counts or the scale, so absurd-but-decodable settings are accepted."),
]
# endregion 🔖️Cases


# region 🔖️Rust
HEADER = """//! 🧪️ `{kind}` fixture — `{directory}`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🏢️ The model is the two-storey braced steel frame (6.0 m bay, 3.5 m storeys, HEB 200 columns,
//! IPE 270/IPE 240 beams, a CHS 88.9x4.0 brace, an RC infill panel with a window opening), the
//! SECOND real-world fem2d model — the first is the timber portal frame the subset-level
//! differential cases share. Every value is in SI base units.
//!
{note}

use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::mutations::{{apply_fem2d_mutation, inverse_fem2d_mutation}};
use crate::artifacts::fem2d::Fem2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
{diff_const}const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {{
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> Fem2dSnapshot {{
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> Fem2dMutation {{
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}}
"""

CANONICAL = """
/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{label}: committed {{label}} JSON is not canonical");
    }}
{identical}    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{label}: committed mutation JSON is not canonical");
}}
"""

TOUCHES = """
/// 🔀️ Each verb writes exactly ONE of the nine members. An after-snapshot comparison cannot make
/// this check on its own: an implementation that re-derived a sibling collection on every edit
/// would still land on the right value for the member it meant to write.
#[test]
fn touches_only_its_own_member() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("{kind} applies to its committed before-snapshot");
{lines}}}
"""

DIFF_TESTS = """
/// 🔺️ The delta must be exactly the committed one, on exactly the `{member}` slot.
#[test]
fn produces_committed_diff() {{
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
{shape}    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{label}: produced diff differs from the committed 🔺️diff/🔣️.json");
}}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {{
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{label}: committed diff JSON is not canonical");
}}

/// 🩹 Replaying the committed delta on `before` must reproduce the committed `after`.
#[test]
fn committed_diff_applies_to_after() {{
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{label}: committed diff did not carry before to after");
}}
"""

HAPPY_BODY = """
/// ▶️ `{kind}` carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("{kind} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{label}: applied state differs from committed after-snapshot");
    assert_ne!(snapshot, base, "{label}: the forward mutation left the model untouched, so nothing was proved");
{effect}}}

/// ↩️ Applying the computed inverse after the forward step lands back on `before`.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "{label}: {kind} undoes with exactly one step, got {{inverse:?}}");
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {{
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{label}: inverse did not restore the before-snapshot");
}}

/// 🎯️ The declared outcome — applied, with no diagnostic at all — is what this kind really emits.
#[test]
fn declared_outcome_holds() {{
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "{label} declares an applied outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "{label}: a clean application raises no diagnostic, got {{:?}}", produced.messages());
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("{label}: declared applied but the mutation was rejected");
}}
"""

REJECT_BODY = """
/// ▶️ A refused `{kind}` still applies cleanly — the refusal is carried as a diagnostic beside an
/// EMPTY diff (§C2 LAW 1/2), so `apply` is a no-op rather than an `Err`. The document therefore
/// comes out byte-identical to the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_untouched() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{label}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "{label}: a refused mutation must leave the snapshot exactly where it was");
}}

/// 🚨️ The refusal is the diagnostic this kind's own diff builder raises, at its own level.
#[test]
fn the_refusal_is_the_declared_diagnostic() {{
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::artifacts::fem2d::diff::Fem2dDiff::default(), "{label}: a rejecting {kind} must carry the empty diff, never a half-built delta");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {{messages:?}}");
    assert_eq!(messages[0].code.0, "{code}", "{label}: the refusal is reported as {code}");
    assert_eq!(messages[0].level, protocol::Severity::{severity}, "{severity_note}");
    assert_eq!(messages[0].target, vec![{target}], "the diagnostic addresses the offending id");
    let semantics = <Fem2dMutation as protocol::SemanticMutation<Fem2dSnapshot>>::semantics(&mutation());
    assert_eq!(semantics.kind, "{kind}", "the fixture must be bound to {kind}'s own descriptor");
}}

/// ↩️ {inverse_doc}
#[test]
fn inverse_of_the_refused_mutation() {{
    let inverse = inverse_fem2d_mutation(&before(), &mutation());
{inverse_assert}}}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {{
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("rejected"), "{label} declares a rejected outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(dsl::DslValue::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared: Vec<String> = outcome.get("path").and_then(dsl::DslValue::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared, message.target, "the declared path must match the emitted target");
}}
"""

NOOP_BODY = """
/// ▶️ A no-op `{kind}` is APPLIED, not rejected — it simply changes nothing, so the document comes
/// out byte-identical to the committed `after`, which is the committed `before`.
#[test]
fn no_op_leaves_the_document_untouched() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("{kind}'s no-op diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{label}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "{label}: an APPLIED no-op still leaves the document exactly where it was");
}}

/// ⚠️ A value that is already what the payload asks for is a Warning-level `mutation.no-op`, never
/// an Error and never a Fatal — the mutation applies, it just carries no change.
#[test]
fn the_no_op_is_a_warning_not_a_rejection() {{
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::artifacts::fem2d::diff::Fem2dDiff::default(), "{label}: a no-op {kind} must carry the empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {{messages:?}}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "{label}: an unchanged value is reported as no-op");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a no-op is a Warning — the mutation still APPLIES, it simply changes nothing");
    assert!(messages[0].target.is_empty(), "{kind} raises its no-op through the 2-arg `warn` builder, which attaches no target address");
}}

/// ↩️ {inverse_doc}
#[test]
fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "{label}: {kind} always emits exactly one inverse step, even for a no-op, got {{inverse:?}}");
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {{
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{label}: inverse did not restore the before-snapshot");
}}

/// 🎯️ The declared outcome — applied, with exactly one `warn`-level `mutation.no-op` — is what this
/// kind really emits here.
#[test]
fn declared_outcome_holds() {{
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "{label} declares an applied outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    let declared = outcome.get("messages").and_then(dsl::DslValue::as_array).expect("a no-op outcome declares its diagnostics");
    assert_eq!(declared.len(), produced.messages().len(), "the declared diagnostic count must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(dsl::DslValue::as_str), Some("warn"), "{kind}'s no-op is declared at warn level");
    assert_eq!(declared[0].get("code").and_then(dsl::DslValue::as_str), Some(produced.messages()[0].code.0.as_str()), "the declared code must match the emitted one");
}}
"""


def literal(text):
    return '"%s".to_string()' % text


def effect_lines(entry, diff, base, after):
    """✅️ The kind-specific state assertions the happy-path `applies_to_committed_after` carries."""
    label, member = entry["label"], entry["written"]
    field = RUST_MEMBER[member]
    lines = []
    if member == "analysis":
        settings = entry["mutation"]["settings"]
        lines.append('    assert_eq!(snapshot.analysis.modal_count, %d, "%s: the modal count must be the one the payload states");\n' % (settings["modalCount"], label))
        lines.append('    assert_eq!(snapshot.analysis.buckling_count, %d, "%s: the buckling count must be the one the payload states");\n' % (settings["bucklingCount"], label))
        lines.append('    assert_eq!(snapshot.analysis.deformation_scale, %s, "%s: the deformation scale must be the one the payload states");\n' % (render_float(settings["deformationScale"]), label))
        return "".join(lines)
    piece = diff[member]
    grew = len(after[member]) - len(base[member])
    lines.append('    assert_eq!(snapshot.%s.len(), %d, "%s: the %s collection must end up %d records long");\n' % (field, len(after[member]), label, member, len(after[member])))
    if piece["added"]:
        added = piece["added"][0]["id"]
        lines.append('    assert_eq!(%s, "%s", "%s: the new record must be appended at the tail — no create- verb in this vocabulary carries an index");\n' % (tail_id(member, field), added, label))
        assert grew == 1
    if piece["removed"]:
        gone = piece["removed"][0]
        lines.append('    assert!(!snapshot.%s.iter().any(|item| %s == "%s"), "%s: no record named %s may survive");\n' % (field, id_of(member, "item"), gone, label, gone))
    if piece["patched"]:
        entry_id = piece["patched"][0]["id"]
        at = find(base[member], entry_id)
        lines.append('    assert_eq!(%s, "%s", "%s: a patch replaces the record in place and never re-orders the collection");\n' % (index_id(member, field, at), entry_id, label))
    return "".join(lines)


def id_of(member, binding):
    if member == "elements":
        return "crate::artifacts::fem2d::element_id(%s)" % binding
    return "%s.id" % binding


def tail_id(member, field):
    if member == "elements":
        return 'crate::artifacts::fem2d::element_id(snapshot.elements.last().expect("the collection is not empty"))'
    return 'snapshot.%s.last().expect("the collection is not empty").id' % field


def index_id(member, field, at):
    if member == "elements":
        return "crate::artifacts::fem2d::element_id(&snapshot.elements[%d])" % at
    return "snapshot.%s[%d].id" % (field, at)


def touch_lines(entry):
    label, member = entry["label"], entry["written"]
    lines = []
    for name in MEMBERS:
        if name == member:
            continue
        field = RUST_MEMBER[name]
        lines.append('    assert_eq!(snapshot.%s, base.%s, "%s: this verb writes %s and nothing else, but %s moved");\n' % (field, field, label, member, name))
    return "".join(lines)


def diff_shape(entry, diff):
    label, member = entry["label"], entry["written"]
    field = RUST_MEMBER[member]
    if member == "analysis":
        return '    assert!(outcome.diff().analysis.is_some(), "%s: the analysis facet is written as a whole value, never as a collection delta");\n' % label
    piece = diff[member]
    lines = ['    let delta = outcome.diff().%s.as_ref().expect("%s delta");\n' % (field, member)]
    lines.append('    assert_eq!((delta.added.len(), delta.removed.len(), delta.patched.len()), (%d, %d, %d), "%s: the delta must be exactly one %s entry");\n' % (len(piece["added"]), len(piece["removed"]), len(piece["patched"]), label, "added" if piece["added"] else ("removed" if piece["removed"] else "patched")))
    lines.append('    assert!(delta.reordered.is_none(), "%s: no verb in this vocabulary re-orders a collection");\n' % label)
    for name in MEMBERS[:-1]:
        if name == member:
            continue
        lines.append('    assert!(outcome.diff().%s.is_none(), "%s: no %s delta may be opened by this verb");\n' % (RUST_MEMBER[name], label, name))
    return "".join(lines)


CREATE_INVERSE = {
    "create-node": ("delete-node", "DeleteNode"),
    "create-element": ("delete-element", "DeleteElement"),
    "create-material": ("delete-material", "DeleteMaterial"),
    "create-section": ("delete-section", "DeleteSection"),
    "create-support": ("delete-support", "DeleteSupport"),
    "create-region": ("delete-region", "DeleteRegion"),
    "create-load-case": ("delete-load-case", "DeleteLoadCase"),
    "create-combination": ("delete-combination", "DeleteCombination"),
}


def reject_inverse(entry):
    """↩️ What each kind's `↩️inverse/🦀️.rs` really returns for the refused payload."""
    kind, label = entry["kind"], entry["label"]
    if kind in CREATE_INVERSE:
        verb, variant = CREATE_INVERSE[kind]
        identity = entry["reject_id"]
        doc = "`%s`'s inverse is PAYLOAD-derived, not base-derived: it is a `%s` of the id it was asked to create even when the create itself was refused." % (kind, verb)
        assertion = (
            '    assert_eq!(inverse.len(), 1, "%s: %s always undoes with exactly one step, got {inverse:?}");\n'
            '    let Fem2dMutation::%s(undo) = &inverse[0] else {\n'
            '        panic!("%s\'s inverse must be a %s, got {:?}", inverse[0]);\n'
            '    };\n'
            '    assert_eq!(undo.id, "%s", "the inverse addresses exactly the id the payload carried");\n'
        ) % (label, kind, variant, kind, verb, identity)
        return doc, assertion
    doc = "`%s`'s inverse is BASE-derived: with no such record on `before` there is nothing to restore, so the inverse collapses to `Vec::new()`." % kind
    assertion = '    assert!(inverse.is_empty(), "%s: a target BASE never held leaves nothing to restore, got {inverse:?}");\n' % label
    return doc, assertion


def noop_inverse(entry):
    kind = entry["kind"]
    if kind == "update-analysis-settings":
        return "`update-analysis-settings` always emits an inverse carrying `base.analysis` — for a no-op that step is the identity, and it must still be exactly one step."
    return "`change-load-case-self-weight` inverts from BASE: the case exists, so the inverse is a step restoring the flag it already had."


def render_rust(entry, diff, base, after):
    label = entry["label"]
    note = "\n".join("//! " + line for line in entry["note"].split("\n"))
    if entry["family"] == "happy":
        header = HEADER.format(kind=entry["kind"], directory=entry["directory"], note=note, diff_const='const DIFF: &str = include_str!("🔺️diff/🔣️.json");\n')
        body = HAPPY_BODY.format(kind=entry["kind"], label=label, effect=effect_lines(entry, diff, base, after))
        body += TOUCHES.format(kind=entry["kind"], lines=touch_lines(entry))
        body += CANONICAL.format(label=label, identical="")
        body += DIFF_TESTS.format(label=label, member=entry["written"], shape=diff_shape(entry, diff))
        return header + body
    identical = '    assert_eq!(BEFORE, AFTER, "%s changes nothing: the two committed snapshots must be byte-identical");\n' % label
    if entry["family"] == "reject":
        header = HEADER.format(kind=entry["kind"], directory=entry["directory"], note=note, diff_const="")
        doc, assertion = reject_inverse(entry)
        body = REJECT_BODY.format(
            kind=entry["kind"],
            label=label,
            code=entry["code"],
            severity="Fatal" if entry["level"] == "fatal" else "Error",
            severity_note=("a duplicate identity is an invariant breach — Fatal, and no merge policy may absorb it" if entry["level"] == "fatal" else "a missing target is an Error, the level a merge policy may still choose to tolerate"),
            target=", ".join(literal(item) for item in entry["target"]),
            inverse_doc=doc,
            inverse_assert=assertion,
        )
        body += CANONICAL.format(label=label, identical=identical)
        return header + body
    header = HEADER.format(kind=entry["kind"], directory=entry["directory"], note=note, diff_const='const DIFF: &str = include_str!("🔺️diff/🔣️.json");\n')
    body = NOOP_BODY.format(kind=entry["kind"], label=label, inverse_doc=noop_inverse(entry))
    body += CANONICAL.format(label=label, identical=identical)
    body += """
/// 🔺️ A no-op produces the artifact's `Default` diff — all seventeen sparse slots left `None`.
#[test]
fn produces_committed_diff() {{
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{label}: produced diff differs from the committed 🔺️diff/🔣️.json");
    let typed: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes into Fem2dDiff");
    assert_eq!(typed, crate::artifacts::fem2d::diff::Fem2dDiff::default(), "{label}: a no-op delta is the artifact's Default diff");
}}

/// 🔣️ The committed diff is itself canonical. `Fem2dDiff` carries a container-level `default` and no
/// per-field skip, so all seventeen sparse slots must be present as `null`.
#[test]
fn committed_diff_is_canonical() {{
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{label}: committed diff JSON is not canonical");
    let slots = original.as_object().expect("the committed diff is a JSON object");
    assert_eq!(slots.len(), 17, "Fem2dDiff emits all seventeen sparse slots, got {{slots:?}}");
}}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`. For a no-op that
/// is the identity — and still a real assertion: `apply` must leave every other member alone too.
#[test]
fn committed_diff_applies_to_after() {{
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{label}: committed diff did not carry before to after");
}}
""".format(label=label)
    return header + body


# endregion 🔖️Rust


# region 🔖️Emit
def payload_identity(mutation):
    """🪪️ The id a `create-` payload carries, for the refused-inverse assertion."""
    for key in ("node", "element", "material", "section", "support", "region", "loadCase", "combination"):
        if key in mutation:
            return mutation[key]["id"]
    return None


def prepare():
    for entry in CASES:
        entry["label"] = "%s/%s" % (entry["kind"], entry["id"])
        entry["reject_id"] = payload_identity(entry["mutation"])
        diff, messages = build_diff(entry["mutation"], MODEL)
        after = apply_diff(MODEL, diff)
        entry["diff"], entry["messages"], entry["after"] = diff, messages, after
        if entry["family"] == "happy":
            assert diff is not None and not messages, entry["label"]
            assert after != MODEL, entry["label"]
            entry["outcome"] = {"status": "applied"}
        elif entry["family"] == "reject":
            assert diff is None and len(messages) == 1 and messages[0]["level"] in ("error", "fatal"), entry["label"]
            entry["code"], entry["level"], entry["target"] = messages[0]["code"], messages[0]["level"], messages[0]["target"]
            entry["outcome"] = {"status": "rejected", "code": entry["code"], "path": entry["target"]}
            assert after == MODEL, entry["label"]
        else:
            assert diff is None and len(messages) == 1 and messages[0]["level"] == "warn", entry["label"]
            entry["outcome"] = {"status": "applied", "messages": [{"level": "warn", "code": "mutation.no-op"}]}
            assert after == MODEL, entry["label"]
    return CASES


def leading_emoji(name):
    """🪪️ The handpicked leading emoji of a directory identity — everything before the kebab stem."""
    at = 0
    while at < len(name) and not ("a" <= name[at] <= "z" or "0" <= name[at] <= "9"):
        at += 1
    return name[:at]


def case_root(entry):
    return os.path.join(SUBSETS, SUBSET_OF[entry["kind"]], "🧬️schema", "🧬️mutations", KIND_DIR[entry["kind"]], "🧪️tests", entry["directory"])


def write(path, text, check):
    if check:
        existing = open(path, encoding="utf-8").read() if os.path.exists(path) else None
        print(("SAME  " if existing == text else "DIFF  ") + os.path.relpath(path, REPO))
        return
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def emit(check=False):
    prepare()
    names = {}
    for entry in CASES:
        root = case_root(entry)
        assert len(entry["directory"]) <= 28, (entry["directory"], len(entry["directory"]))
        names.setdefault(os.path.dirname(root), []).append(entry["directory"])
        write(os.path.join(root, "🦠️mutation", "🔣️.json"), dump(entry["mutation"]) + "\n", check)
        write(os.path.join(root, "🎯️outcome", "🔣️.json"), dump(entry["outcome"]) + "\n", check)
        write(os.path.join(root, "📸️snapshot", "⬅️before", "🔣️.json"), dump(MODEL) + "\n", check)
        write(os.path.join(root, "📸️snapshot", "➡️after", "🔣️.json"), dump(entry["after"]) + "\n", check)
        if entry["family"] == "reject":
            write(os.path.join(root, "🔺️diff", "🚫️.absent"), "", check)
        else:
            write(os.path.join(root, "🔺️diff", "🔣️.json"), dump(entry["diff"] if entry["diff"] is not None else empty_diff()) + "\n", check)
        write(os.path.join(root, "🦀️.rs"), render_rust(entry, entry["diff"], MODEL, entry["after"]), check)
    for parent, siblings in names.items():
        present = sorted(set(siblings) | set(os.listdir(parent))) if os.path.isdir(parent) else sorted(siblings)
        emojis = [leading_emoji(name) for name in present]
        assert len(set(emojis)) == len(emojis), (parent, sorted(zip(emojis, present)))
    print("cases: %d" % len(CASES))


if __name__ == "__main__":
    emit(check="--check" in sys.argv[1:])
# endregion 🔖️Emit
