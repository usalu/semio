#!/usr/bin/env python3
"""⚖️ W13 — the HARDENED fem2d mutation semantics, transcribed once, in Python.

This module is the single place the new rule set is written down for tooling. It is a faithful
transcription of the Rust each kind's `🔺️diff/🦀️.rs` carries after this wave, in the same guard
ORDER, so the authoring script (`🔨️w13-fem2d-semantics.py`) and the verifier
(`🔨️w13-replay-fem2d.py`) can predict, per vector, exactly what the plugin will answer.

It is deliberately NOT the plugin's independent Python oracle — that one lives beside the fixtures
in `🌐️any/🧪️tests/*/🐍️.py` and is re-derived from the schema. This one is a transcription, used
only to author and to audit the committed evidence.

Outcome vocabulary (fem2d + fem3d, W12 matches):

| code                        | level   | when                                                   |
|-----------------------------|---------|--------------------------------------------------------|
| `mutation.duplicate-id`     | Fatal   | a `create-` id already exists                          |
| `mutation.id-mismatch`      | Fatal   | a `replace-`'s new record carries a different id       |
| `mutation.invariant`        | Fatal   | geometry / plausibility bound violated by the payload  |
| `mutation.target-missing`   | Error   | a named record or foreign key does not exist           |
| `mutation.target-referenced`| Error   | a `delete-` target is still named by other records     |
| `mutation.no-op`            | Warning | the payload asks for what the document already says    |
"""

# region 🔖️Imports
import copy
import math

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("nodes", "elements", "regions", "materials", "sections", "supports", "loadCases", "combinations", "analysis")

FATAL, ERROR, WARNING = "Fatal", "Error", "Warning"

DUPLICATE_ID = "mutation.duplicate-id"
ID_MISMATCH = "mutation.id-mismatch"
INVARIANT = "mutation.invariant"
TARGET_MISSING = "mutation.target-missing"
TARGET_REFERENCED = "mutation.target-referenced"
NO_OP = "mutation.no-op"

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
    "create-region",
    "delete-region",
    "replace-region",
    "create-load-case",
    "delete-load-case",
    "add-load",
    "remove-load",
    "change-load-case-self-weight",
    "create-combination",
    "delete-combination",
    "update-analysis-settings",
)


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}
KIND_OF_TAG = {tag: kind for kind, tag in TAGS.items()}


def kind_of(mutation):
    """🏷️ The kind an internally tagged payload names."""
    return KIND_OF_TAG[mutation["mutation"]]


def noun_of(kind):
    return kind.split("-", 1)[1]


# endregion 🔖️Vocabulary


# region 🔖️Reject
class Reject(Exception):
    """🚫️ One refusal — the diagnostic the Rust builder returns beside an empty diff."""

    def __init__(self, code, level, target, message):
        super().__init__(message)
        self.code, self.level, self.target, self.message = code, level, list(target), message


def find(items, identifier):
    for at, item in enumerate(items):
        if item["id"] == identifier:
            return at
    return None


# endregion 🔖️Reject


# region 🔖️Validation
def finite(*values):
    return all(isinstance(v, (int, float)) and math.isfinite(float(v)) for v in values)


def check_node_geometry(node):
    """📍️ A node's coordinates must be real numbers — a NaN/∞ ordinate has no place in a model."""
    if not finite(node["x"], node["y"]):
        raise Reject(INVARIANT, FATAL, [node["id"]], 'Node "%s" carries a non-finite coordinate.' % node["id"])


def check_material_plausibility(record):
    """🧱️ Isotropic-elasticity bounds: a positive modulus and density, and a Poisson ratio inside
    the thermodynamically admissible open interval (-1, 0.5)."""
    identifier = record["id"]
    if not finite(record["e"], record["nu"], record["rho"]):
        raise Reject(INVARIANT, FATAL, [identifier], 'Material "%s" carries a non-finite property.' % identifier)
    if record["e"] <= 0.0:
        raise Reject(INVARIANT, FATAL, [identifier], 'Material "%s" needs a positive Young\'s modulus, got %r.' % (identifier, record["e"]))
    if record["rho"] <= 0.0:
        raise Reject(INVARIANT, FATAL, [identifier], 'Material "%s" needs a positive density, got %r.' % (identifier, record["rho"]))
    if not (-1.0 < record["nu"] < 0.5):
        raise Reject(INVARIANT, FATAL, [identifier], 'Material "%s" needs a Poisson ratio in (-1, 0.5), got %r.' % (identifier, record["nu"]))


def check_section_plausibility(record):
    """📏️ A cross-section has a positive area and a positive strong-axis second moment of area."""
    identifier = record["id"]
    if not finite(record["area"], record["iy"]):
        raise Reject(INVARIANT, FATAL, [identifier], 'Section "%s" carries a non-finite property.' % identifier)
    if record["area"] <= 0.0:
        raise Reject(INVARIANT, FATAL, [identifier], 'Section "%s" needs a positive area, got %r.' % (identifier, record["area"]))
    if record["iy"] <= 0.0:
        raise Reject(INVARIANT, FATAL, [identifier], 'Section "%s" needs a positive second moment of area, got %r.' % (identifier, record["iy"]))


def signed_area(ring):
    """📐️ Twice the signed shoelace area of a closed ring, halved — sign carries the winding."""
    total = 0.0
    for at, (x, y) in enumerate(ring):
        nx, ny = ring[(at + 1) % len(ring)]
        total += x * ny - nx * y
    return total / 2.0


def point_in_ring(point, ring):
    """🎯️ Crossing-number containment, boundary counted as INSIDE (a hole may touch the outline)."""
    x, y = point
    for at, (ax, ay) in enumerate(ring):
        bx, by = ring[(at + 1) % len(ring)]
        cross = (bx - ax) * (y - ay) - (by - ay) * (x - ax)
        within = min(ax, bx) - 1e-12 <= x <= max(ax, bx) + 1e-12 and min(ay, by) - 1e-12 <= y <= max(ay, by) + 1e-12
        if abs(cross) <= 1e-12 and within:
            return True
    inside = False
    for at, (ax, ay) in enumerate(ring):
        bx, by = ring[(at + 1) % len(ring)]
        if (ay > y) != (by > y):
            crossing = ax + (y - ay) * (bx - ax) / (by - ay)
            if x < crossing:
                inside = not inside
    return inside


def check_region_geometry(record):
    """🗺️ A meshable region: an outline that is a real polygon with real area, positive thickness
    and mesh size, and every hole a real polygon lying inside that outline."""
    identifier = record["id"]
    outline = [tuple(point) for point in record["outline"]]
    if len(outline) < 3:
        raise Reject(INVARIANT, FATAL, [identifier], 'Region "%s" needs an outline of at least three points, got %d.' % (identifier, len(outline)))
    for point in outline:
        if not finite(*point):
            raise Reject(INVARIANT, FATAL, [identifier], 'Region "%s" carries a non-finite outline coordinate.' % identifier)
    if abs(signed_area(outline)) <= 0.0:
        raise Reject(INVARIANT, FATAL, [identifier], 'Region "%s" has a degenerate outline of zero area.' % identifier)
    if not finite(record["thickness"]) or record["thickness"] <= 0.0:
        raise Reject(INVARIANT, FATAL, [identifier], 'Region "%s" needs a positive thickness, got %r.' % (identifier, record["thickness"]))
    if not finite(record["meshSize"]) or record["meshSize"] <= 0.0:
        raise Reject(INVARIANT, FATAL, [identifier], 'Region "%s" needs a positive mesh size, got %r.' % (identifier, record["meshSize"]))
    for at, hole in enumerate(record["holes"]):
        ring = [tuple(point) for point in hole]
        if len(ring) < 3:
            raise Reject(INVARIANT, FATAL, [identifier], 'Region "%s" hole %d needs at least three points, got %d.' % (identifier, at, len(ring)))
        for point in ring:
            if not finite(*point):
                raise Reject(INVARIANT, FATAL, [identifier], 'Region "%s" hole %d carries a non-finite coordinate.' % (identifier, at))
        if abs(signed_area(ring)) <= 0.0:
            raise Reject(INVARIANT, FATAL, [identifier], 'Region "%s" hole %d is degenerate.' % (identifier, at))
        for point in ring:
            if not point_in_ring(point, outline):
                raise Reject(INVARIANT, FATAL, [identifier], 'Region "%s" hole %d lies outside the outline.' % (identifier, at))


def check_analysis_bounds(settings):
    """⚙️ At least one mode of each family, and a real positive deformation exaggeration."""
    if settings["modalCount"] < 1:
        raise Reject(INVARIANT, FATAL, [], "Analysis settings need at least one modal mode, got %r." % settings["modalCount"])
    if settings["bucklingCount"] < 1:
        raise Reject(INVARIANT, FATAL, [], "Analysis settings need at least one buckling mode, got %r." % settings["bucklingCount"])
    if not finite(settings["deformationScale"]) or settings["deformationScale"] <= 0.0:
        raise Reject(INVARIANT, FATAL, [], "Analysis settings need a positive deformation scale, got %r." % settings["deformationScale"])


# endregion 🔖️Validation


# region 🔖️References
def element_ends(element):
    return element["start"], element["end"], element["materialId"], element["sectionId"]


def resolve_element(document, element):
    """🔗️ The four foreign keys every element carries, in the order `create-element` reads them."""
    start, end, material_id, section_id = element_ends(element)
    if find(document["nodes"], start) is None:
        raise Reject(TARGET_MISSING, ERROR, [start], 'Node "%s" does not exist.' % start)
    if find(document["nodes"], end) is None:
        raise Reject(TARGET_MISSING, ERROR, [end], 'Node "%s" does not exist.' % end)
    if find(document["materials"], material_id) is None:
        raise Reject(TARGET_MISSING, ERROR, [material_id], 'Material "%s" does not exist.' % material_id)
    if find(document["sections"], section_id) is None:
        raise Reject(TARGET_MISSING, ERROR, [section_id], 'Section "%s" does not exist.' % section_id)


def resolve_load(document, load):
    """🔗️ The one foreign key a load variant carries."""
    if load["kind"] == "nodal":
        if find(document["nodes"], load["nodeId"]) is None:
            raise Reject(TARGET_MISSING, ERROR, [load["nodeId"]], 'Node "%s" does not exist.' % load["nodeId"])
    elif load["kind"] == "memberUdl":
        if find(document["elements"], load["elementId"]) is None:
            raise Reject(TARGET_MISSING, ERROR, [load["elementId"]], 'Element "%s" does not exist.' % load["elementId"])
    else:
        if find(document["regions"], load["regionId"]) is None:
            raise Reject(TARGET_MISSING, ERROR, [load["regionId"]], 'Region "%s" does not exist.' % load["regionId"])


def referrers_of_element(document, identifier):
    return ["%s/%s" % (case["id"], load["id"]) for case in document["loadCases"] for load in case["loads"] if load["kind"] == "memberUdl" and load["elementId"] == identifier]


def referrers_of_material(document, identifier):
    found = [element["id"] for element in document["elements"] if element["materialId"] == identifier]
    return found + [region["id"] for region in document["regions"] if region["materialId"] == identifier]


def referrers_of_section(document, identifier):
    return [element["id"] for element in document["elements"] if element["sectionId"] == identifier]


def referrers_of_region(document, identifier):
    return ["%s/%s" % (case["id"], load["id"]) for case in document["loadCases"] for load in case["loads"] if load["kind"] == "area" and load["regionId"] == identifier]


def referrers_of_load_case(document, identifier):
    return [combination["id"] for combination in document["combinations"] for term in combination["terms"] if term["caseId"] == identifier]


def referrers_of_combination(document, identifier):
    return [combination["id"] for combination in document["combinations"] if combination["id"] != identifier for term in combination["terms"] if term["caseId"] == identifier]


REFERRERS = {
    "delete-element": ("Element", "member UDL", referrers_of_element),
    "delete-material": ("Material", "element or region", referrers_of_material),
    "delete-section": ("Section", "element", referrers_of_section),
    "delete-region": ("Region", "area load", referrers_of_region),
    "delete-load-case": ("Load case", "combination term", referrers_of_load_case),
    "delete-combination": ("Combination", "combination term", referrers_of_combination),
}
"""🧷️ The six `delete-` verbs that now refuse while referrers exist.

`delete-node` is EXEMPT — its own committed vector (`🚫️removes-node-n3-without-6eab3f`) and the
kind's docstring specify cascade-free acceptance, the one documented exception. `delete-support`
needs no guard: no record in the vocabulary names a support id.
"""
# endregion 🔖️References


# region 🔖️Semantics
def apply_hardened(document, mutation):
    """🧬️ Applies one typed mutation under the HARDENED rules, or raises `Reject`."""
    kind = kind_of(mutation)
    result = copy.deepcopy(document)

    if kind == "update-analysis-settings":
        check_analysis_bounds(mutation["settings"])
        if mutation["settings"] == result["analysis"]:
            raise Reject(NO_OP, WARNING, [], "Analysis settings are unchanged.")
        result["analysis"] = copy.deepcopy(mutation["settings"])
        return result

    if kind in ("add-load", "remove-load", "change-load-case-self-weight"):
        at = find(result["loadCases"], mutation["caseId"])
        if at is None:
            raise Reject(TARGET_MISSING, ERROR, [mutation["caseId"]], 'Load case "%s" does not exist.' % mutation["caseId"])
        case = result["loadCases"][at]
        if kind == "add-load":
            load = copy.deepcopy(mutation["load"])
            resolve_load(result, load)
            if find(case["loads"], load["id"]) is not None:
                raise Reject(NO_OP, WARNING, [], 'Load "%s" already exists in case "%s".' % (load["id"], case["id"]))
            case["loads"].append(load)
        elif kind == "remove-load":
            where = find(case["loads"], mutation["loadId"])
            if where is None:
                raise Reject(TARGET_MISSING, ERROR, [mutation["loadId"]], 'Load "%s" does not exist in case "%s".' % (mutation["loadId"], case["id"]))
            case["loads"].pop(where)
        else:
            if case["selfWeight"] == mutation["newSelfWeight"]:
                raise Reject(NO_OP, WARNING, [], 'Load case "%s" self-weight is already %s.' % (case["id"], str(mutation["newSelfWeight"]).lower()))
            case["selfWeight"] = mutation["newSelfWeight"]
        return result

    noun = noun_of(kind)
    collection, create_argument, replace_argument = COLLECTIONS[noun]
    items = result[collection]

    if kind.startswith("create-"):
        record = copy.deepcopy(mutation[create_argument])
        if find(items, record["id"]) is not None:
            raise Reject(DUPLICATE_ID, FATAL, [record["id"]], 'A %s with id "%s" already exists.' % (noun, record["id"]))
        validate_new_record(result, kind, record)
        items.append(record)
        return result

    if kind.startswith("delete-"):
        at = find(items, mutation["id"])
        if at is None:
            raise Reject(TARGET_MISSING, ERROR, [mutation["id"]], '%s "%s" does not exist.' % (noun.capitalize(), mutation["id"]))
        if kind in REFERRERS:
            label, blocker, lookup = REFERRERS[kind]
            found = lookup(result, mutation["id"])
            if found:
                raise Reject(TARGET_REFERENCED, ERROR, [mutation["id"]] + found, '%s "%s" is still referenced by %d %s(s).' % (label, mutation["id"], len(found), blocker))
        items.pop(at)
        return result

    at = find(items, mutation["id"])
    if at is None:
        raise Reject(TARGET_MISSING, ERROR, [mutation["id"]], '%s "%s" does not exist.' % (noun.capitalize(), mutation["id"]))
    record = copy.deepcopy(mutation[replace_argument])
    if record["id"] != mutation["id"]:
        raise Reject(ID_MISMATCH, FATAL, [mutation["id"], record["id"]], 'A replace-%s may not rename "%s" to "%s".' % (noun, mutation["id"], record["id"]))
    validate_new_record(result, kind, record)
    if items[at] == record:
        raise Reject(NO_OP, WARNING, [], '%s "%s" is already equal to the replacement value.' % (noun.capitalize(), mutation["id"]))
    items[at] = record
    return result


def validate_new_record(document, kind, record):
    """🛡️ The per-noun payload validation both halves of a `create-`/`replace-` pair now share."""
    noun = noun_of(kind)
    if noun == "node":
        check_node_geometry(record)
    elif noun == "element":
        resolve_element(document, record)
    elif noun == "material":
        check_material_plausibility(record)
    elif noun == "section":
        check_section_plausibility(record)
    elif noun == "support":
        if find(document["nodes"], record["nodeId"]) is None:
            raise Reject(TARGET_MISSING, ERROR, [record["nodeId"]], 'Node "%s" does not exist.' % record["nodeId"])
    elif noun == "region":
        if find(document["materials"], record["materialId"]) is None:
            raise Reject(TARGET_MISSING, ERROR, [record["materialId"]], 'Material "%s" does not exist.' % record["materialId"])
        check_region_geometry(record)
    elif noun == "load-case":
        for load in record["loads"]:
            resolve_load(document, load)
    elif noun == "combination":
        for term in record["terms"]:
            known = find(document["loadCases"], term["caseId"]) is not None or find(document["combinations"], term["caseId"]) is not None
            if not known:
                raise Reject(TARGET_MISSING, ERROR, [term["caseId"]], 'Load case or combination "%s" does not exist.' % term["caseId"])


# endregion 🔖️Semantics
