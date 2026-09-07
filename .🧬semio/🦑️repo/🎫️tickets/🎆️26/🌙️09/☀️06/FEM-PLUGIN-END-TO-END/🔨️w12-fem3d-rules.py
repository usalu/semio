#!/usr/bin/env python3
"""📐️ W12 — the fem3d mutation rule set, in Python, as the ticket's own predictor.

This module is the SPECIFICATION half of W12's hardening: for one `(before, mutation)` pair it
returns the outcome the hardened Rust must produce, plus the resulting model. It is deliberately
separate from the plugin's own independent Python reference (`🌐️any/🧪️tests/*/🐍️.py`) — that one
stays an independent re-implementation; this one is the ticket-side oracle used to audit the
committed vectors and to author new ones.

Codes, in the vocabulary the plugin and its sibling plugins already use:

* ``mutation.duplicate-id``     Fatal   — an identity already taken (pre-existing)
* ``mutation.target-missing``   Error   — a named identity that does not resolve (pre-existing)
* ``mutation.no-op``            Warning — the request is already satisfied (pre-existing)
* ``mutation.id-mismatch``      Fatal   — a `replace-` whose new record renames the target (new)
* ``mutation.target-referenced``Error   — a `delete-` whose target still has referrers (new)
* ``mutation.invariant``        Fatal   — a value/geometry invariant breach; the repo-wide name,
                                          mirrored from `🔱️trinity` and `📸️remodel` (new here)
"""

import copy
import math

# region 🔖️Vocabulary
MEMBERS = ("nodes", "elements", "materials", "sections", "solids", "supports", "loadCases", "combinations", "analysis")

TAGS = {
    "create-node": "createNode",
    "delete-node": "deleteNode",
    "create-element": "createElement",
    "delete-element": "deleteElement",
    "replace-element": "replaceElement",
    "create-section": "createSection",
    "delete-section": "deleteSection",
    "replace-section": "replaceSection",
    "create-solid": "createSolid",
    "delete-solid": "deleteSolid",
    "replace-solid": "replaceSolid",
    "create-material": "createMaterial",
    "delete-material": "deleteMaterial",
    "replace-material": "replaceMaterial",
    "create-support": "createSupport",
    "delete-support": "deleteSupport",
    "replace-support": "replaceSupport",
    "create-load-case": "createLoadCase",
    "delete-load-case": "deleteLoadCase",
    "add-load": "addLoad",
    "remove-load": "removeLoad",
    "change-load-case-self-weight": "changeLoadCaseSelfWeight",
    "create-combination": "createCombination",
    "delete-combination": "deleteCombination",
    "update-analysis-settings": "updateAnalysisSettings",
}
KIND_OF_TAG = {tag: kind for kind, tag in TAGS.items()}

COLLECTIONS = {
    "node": ("nodes", "node", None),
    "element": ("elements", "element", "newElement"),
    "solid": ("solids", "solid", "newSolid"),
    "material": ("materials", "material", "newMaterial"),
    "section": ("sections", "section", "newSection"),
    "support": ("supports", "support", "newSupport"),
    "load-case": ("loadCases", "loadCase", None),
    "combination": ("combinations", "combination", None),
}

LABEL = {"nodes": "Node", "elements": "Element", "materials": "Material", "sections": "Section", "solids": "Solid", "supports": "Support", "loadCases": "Load case", "combinations": "Combination"}

# 🕳️ `delete-node` is the ONE deliberate exception to the referential-integrity rule: its committed
# fixture `🚫️removes-the-column-head-056295` explicitly specifies that a node under a live frame is
# removed and the frame keeps naming it ("delete-node is cascade-free"). W12's brief keeps a
# documented behaviour when a test states it, so `delete-node` still refuses only on target-missing.
CASCADE_EXEMPT = {"delete-node"}
# endregion 🔖️Vocabulary


# region 🔖️Outcome
def ok(diff_member=None):
    return {"status": "applied", "messages": []}


def refuse(level, code, message, path):
    return {"status": "rejected", "code": code, "path": list(path), "messages": [{"level": level, "code": code}], "message": message}


def noop(message):
    return {"status": "applied", "messages": [{"level": "warn", "code": "mutation.no-op"}], "message": message}


def fatal(code, message, path):
    return refuse("fatal", code, message, path)


def error(code, message, path):
    return refuse("error", code, message, path)


def rejected(outcome):
    return outcome["status"] == "rejected"


def is_noop(outcome):
    return any(entry["code"] == "mutation.no-op" for entry in outcome["messages"])


# endregion 🔖️Outcome


# region 🔖️Lookup
def find(items, identifier):
    for at, item in enumerate(items):
        if item["id"] == identifier:
            return at
    return None


def loads_of(document):
    for case in document["loadCases"]:
        for load in case["loads"]:
            yield case, load


def referrers(document, collection, identifier):
    """🔗️ Every record id that would dangle if `identifier` left `collection`, in member order."""
    found = []
    if collection == "nodes":
        found += [element["id"] for element in document["elements"] if element["start"] == identifier or element["end"] == identifier]
        found += [support["id"] for support in document["supports"] if support["nodeId"] == identifier]
        found += [load["id"] for _, load in loads_of(document) if load["kind"] == "nodal" and load["nodeId"] == identifier]
    elif collection == "elements":
        found += [load["id"] for _, load in loads_of(document) if load["kind"] == "memberUdl" and load["elementId"] == identifier]
    elif collection == "sections":
        found += [element["id"] for element in document["elements"] if element["sectionId"] == identifier]
    elif collection == "solids":
        found += [load["id"] for _, load in loads_of(document) if load["kind"] == "area" and load["solidId"] == identifier]
    elif collection == "materials":
        found += [element["id"] for element in document["elements"] if element["materialId"] == identifier]
        found += [solid["id"] for solid in document["solids"] if solid["materialId"] == identifier]
    elif collection == "loadCases":
        found += [combination["id"] for combination in document["combinations"] if identifier in combination["terms"]]
    return found


# endregion 🔖️Lookup


# region 🔖️Invariants
def finite(*values):
    return all(isinstance(value, (int, float)) and math.isfinite(value) for value in values)


def node_invariant(node):
    if not finite(node["x"], node["y"], node["z"]):
        return 'Node "%s" must sit at a finite position, got (%s, %s, %s).' % (node["id"], node["x"], node["y"], node["z"])
    return None


def material_invariant(material):
    if not finite(material["e"], material["g"], material["nu"], material["rho"]):
        return 'Material "%s" must carry finite properties.' % material["id"]
    if material["e"] <= 0.0 or material["g"] <= 0.0 or material["rho"] <= 0.0:
        return 'Material "%s" must carry a positive e, g and rho, got e=%s, g=%s, rho=%s.' % (material["id"], material["e"], material["g"], material["rho"])
    if not -1.0 < material["nu"] < 0.5:
        return 'Material "%s" must carry a Poisson ratio in (-1, 0.5), got %s.' % (material["id"], material["nu"])
    return None


def section_invariant(section):
    if not finite(section["area"], section["iy"], section["iz"], section["j"]):
        return 'Section "%s" must carry finite properties.' % section["id"]
    if section["area"] <= 0.0 or section["iy"] <= 0.0 or section["iz"] <= 0.0 or section["j"] <= 0.0:
        return 'Section "%s" must carry a positive area, iy, iz and j, got area=%s, iy=%s, iz=%s, j=%s.' % (section["id"], section["area"], section["iy"], section["iz"], section["j"])
    return None


def ring_area(ring):
    total = 0.0
    for at, point in enumerate(ring):
        other = ring[(at + 1) % len(ring)]
        total += point[0] * other[1] - other[0] * point[1]
    return total / 2.0


def inside(ring, point):
    """🎯️ Ray-cast crossing count — a point exactly on an edge is undefined and never authored."""
    crossings = False
    for at, corner in enumerate(ring):
        other = ring[(at + 1) % len(ring)]
        if (corner[1] > point[1]) != (other[1] > point[1]):
            x = corner[0] + (point[1] - corner[1]) / (other[1] - corner[1]) * (other[0] - corner[0])
            if point[0] < x:
                crossings = not crossings
    return crossings


def solid_invariant(solid):
    outline = solid["outline"]
    if len(outline) < 3:
        return 'Solid "%s" needs at least three outline points, got %d.' % (solid["id"], len(outline))
    if not all(finite(point[0], point[1]) for point in outline):
        return 'Solid "%s" must carry a finite outline.' % solid["id"]
    if abs(ring_area(outline)) <= 0.0:
        return 'Solid "%s" has a degenerate outline of zero area.' % solid["id"]
    if not finite(solid["height"], solid["meshSize"], solid["baseZ"]):
        return 'Solid "%s" must carry a finite baseZ, height and meshSize.' % solid["id"]
    if solid["height"] <= 0.0:
        return 'Solid "%s" must be extruded by a positive height, got %s.' % (solid["id"], solid["height"])
    if solid["layers"] < 1:
        return 'Solid "%s" must be meshed through at least one layer, got %s.' % (solid["id"], solid["layers"])
    if solid["meshSize"] <= 0.0:
        return 'Solid "%s" must carry a positive mesh size, got %s.' % (solid["id"], solid["meshSize"])
    for hole in solid["holes"]:
        if len(hole) < 3 or abs(ring_area(hole)) <= 0.0:
            return 'Solid "%s" carries a degenerate hole.' % solid["id"]
        for point in hole:
            if not finite(point[0], point[1]) or not inside(outline, point):
                return 'Solid "%s" carries a hole that leaves its outline.' % solid["id"]
    return None


def analysis_invariant(settings):
    if settings["modalCount"] < 1 or settings["bucklingCount"] < 1:
        return "Analysis settings need at least one modal and one buckling factor, got %s and %s." % (settings["modalCount"], settings["bucklingCount"])
    if not finite(settings["deformationScale"]) or settings["deformationScale"] <= 0.0:
        return "Analysis settings need a finite positive deformation scale, got %s." % settings["deformationScale"]
    return None


# endregion 🔖️Invariants


# region 🔖️Rules
def element_keys(element):
    return element["start"], element["end"], element["materialId"], element["sectionId"]


def resolve_element(document, element):
    start, end, material_id, section_id = element_keys(element)
    for label, collection, identifier in (("Node", "nodes", start), ("Node", "nodes", end), ("Material", "materials", material_id), ("Section", "sections", section_id)):
        if find(document[collection], identifier) is None:
            return error("mutation.target-missing", '%s "%s" does not exist.' % (label, identifier), [identifier])
    return None


def resolve_load(document, load):
    if load["kind"] == "nodal":
        collection, label, identifier = "nodes", "Node", load["nodeId"]
    elif load["kind"] == "memberUdl":
        collection, label, identifier = "elements", "Element", load["elementId"]
    else:
        collection, label, identifier = "solids", "Solid", load["solidId"]
    if find(document[collection], identifier) is None:
        return error("mutation.target-missing", '%s "%s" does not exist.' % (label, identifier), [identifier])
    return None


def outcome_of(document, mutation):
    """🎯️ The outcome the hardened fem3d diff builders must produce for `mutation` on `document`."""
    kind = KIND_OF_TAG[mutation["mutation"]]
    if kind == "update-analysis-settings":
        if mutation["settings"] == document["analysis"]:
            return noop("Analysis settings already have that value.")
        breach = analysis_invariant(mutation["settings"])
        return fatal("mutation.invariant", breach, []) if breach else ok()
    if kind == "add-load":
        case_at = find(document["loadCases"], mutation["caseId"])
        if case_at is None:
            return error("mutation.target-missing", 'Load case "%s" does not exist.' % mutation["caseId"], [mutation["caseId"]])
        case = document["loadCases"][case_at]
        if find(case["loads"], mutation["load"]["id"]) is not None:
            return noop('Load "%s" already exists in case "%s".' % (mutation["load"]["id"], mutation["caseId"]))
        return resolve_load(document, mutation["load"]) or ok()
    if kind == "remove-load":
        case_at = find(document["loadCases"], mutation["caseId"])
        if case_at is None:
            return error("mutation.target-missing", 'Load case "%s" does not exist.' % mutation["caseId"], [mutation["caseId"]])
        if find(document["loadCases"][case_at]["loads"], mutation["loadId"]) is None:
            return error("mutation.target-missing", 'Load "%s" does not exist in case "%s".' % (mutation["loadId"], mutation["caseId"]), [mutation["loadId"]])
        return ok()
    if kind == "change-load-case-self-weight":
        case_at = find(document["loadCases"], mutation["caseId"])
        if case_at is None:
            return error("mutation.target-missing", 'Load case "%s" does not exist.' % mutation["caseId"], [mutation["caseId"]])
        if document["loadCases"][case_at]["selfWeight"] == mutation["newSelfWeight"]:
            return noop('Load case "%s" already has self-weight %s.' % (mutation["caseId"], str(mutation["newSelfWeight"]).lower()))
        return ok()

    noun = kind.split("-", 1)[1]
    collection, create_argument, replace_argument = COLLECTIONS[noun]

    if kind.startswith("create-"):
        record = mutation[create_argument]
        if find(document[collection], record["id"]) is not None:
            return fatal("mutation.duplicate-id", 'A %s with id "%s" already exists.' % (noun, record["id"]), [record["id"]])
        if kind == "create-element":
            return resolve_element(document, record) or ok()
        if kind == "create-support":
            if find(document["nodes"], record["nodeId"]) is None:
                return error("mutation.target-missing", 'Node "%s" does not exist.' % record["nodeId"], [record["nodeId"]])
            return ok()
        if kind == "create-solid":
            if find(document["materials"], record["materialId"]) is None:
                return error("mutation.target-missing", 'Material "%s" does not exist.' % record["materialId"], [record["materialId"]])
            breach = solid_invariant(record)
            return fatal("mutation.invariant", breach, [record["id"]]) if breach else ok()
        if kind == "create-load-case":
            for load in record["loads"]:
                refusal = resolve_load(document, load)
                if refusal:
                    return refusal
            return ok()
        if kind == "create-combination":
            for case_id in sorted(record["terms"]):
                if find(document["loadCases"], case_id) is None:
                    return error("mutation.target-missing", 'Load case "%s" does not exist.' % case_id, [case_id])
            return ok()
        breach = {"create-node": node_invariant, "create-material": material_invariant, "create-section": section_invariant}[kind](record)
        return fatal("mutation.invariant", breach, [record["id"]]) if breach else ok()

    if kind.startswith("delete-"):
        if find(document[collection], mutation["id"]) is None:
            return error("mutation.target-missing", '%s "%s" does not exist.' % (LABEL[collection], mutation["id"]), [mutation["id"]])
        if kind not in CASCADE_EXEMPT:
            blockers = referrers(document, collection, mutation["id"])
            if blockers:
                return error("mutation.target-referenced", '%s "%s" is still referenced by %s.' % (LABEL[collection], mutation["id"], ", ".join('"%s"' % blocker for blocker in blockers)), [mutation["id"]] + blockers)
        return ok()

    at = find(document[collection], mutation["id"])
    if at is None:
        return error("mutation.target-missing", '%s "%s" does not exist.' % (LABEL[collection], mutation["id"]), [mutation["id"]])
    record = mutation[replace_argument]
    if record["id"] != mutation["id"]:
        return fatal("mutation.id-mismatch", '%s "%s" cannot be renamed to "%s" by a replace.' % (LABEL[collection], mutation["id"], record["id"]), [mutation["id"], record["id"]])
    if document[collection][at] == record:
        return noop('%s "%s" already has that value.' % (LABEL[collection], mutation["id"]))
    if kind == "replace-element":
        return resolve_element(document, record) or ok()
    if kind == "replace-support":
        if find(document["nodes"], record["nodeId"]) is None:
            return error("mutation.target-missing", 'Node "%s" does not exist.' % record["nodeId"], [record["nodeId"]])
        return ok()
    if kind == "replace-solid":
        if find(document["materials"], record["materialId"]) is None:
            return error("mutation.target-missing", 'Material "%s" does not exist.' % record["materialId"], [record["materialId"]])
        breach = solid_invariant(record)
        return fatal("mutation.invariant", breach, [record["id"]]) if breach else ok()
    breach = {"replace-material": material_invariant, "replace-section": section_invariant}[kind](record)
    return fatal("mutation.invariant", breach, [record["id"]]) if breach else ok()


def apply_mutation(document, mutation):
    """▶️ The model the hardened plugin lands on — the input again whenever the outcome refuses."""
    outcome = outcome_of(document, mutation)
    if rejected(outcome) or is_noop(outcome):
        return copy.deepcopy(document), outcome
    kind = KIND_OF_TAG[mutation["mutation"]]
    result = copy.deepcopy(document)
    if kind == "update-analysis-settings":
        result["analysis"] = copy.deepcopy(mutation["settings"])
    elif kind == "add-load":
        result["loadCases"][find(result["loadCases"], mutation["caseId"])]["loads"].append(copy.deepcopy(mutation["load"]))
    elif kind == "remove-load":
        case = result["loadCases"][find(result["loadCases"], mutation["caseId"])]
        case["loads"].pop(find(case["loads"], mutation["loadId"]))
    elif kind == "change-load-case-self-weight":
        result["loadCases"][find(result["loadCases"], mutation["caseId"])]["selfWeight"] = mutation["newSelfWeight"]
    else:
        noun = kind.split("-", 1)[1]
        collection, create_argument, replace_argument = COLLECTIONS[noun]
        items = result[collection]
        if kind.startswith("create-"):
            items.append(copy.deepcopy(mutation[create_argument]))
        elif kind.startswith("delete-"):
            items.pop(find(items, mutation["id"]))
        else:
            items[find(items, mutation["id"])] = copy.deepcopy(mutation[replace_argument])
    return result, outcome


def written_member(kind):
    if kind == "update-analysis-settings":
        return "analysis"
    if kind in ("add-load", "remove-load", "change-load-case-self-weight"):
        return "loadCases"
    return COLLECTIONS[kind.split("-", 1)[1]][0]


# endregion 🔖️Rules
