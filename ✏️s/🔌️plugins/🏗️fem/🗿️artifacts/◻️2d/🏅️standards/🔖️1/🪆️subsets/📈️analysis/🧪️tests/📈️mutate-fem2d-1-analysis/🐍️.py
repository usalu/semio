#!/usr/bin/env python3
"""🏗️ An INDEPENDENT second implementation of the `s.fem.fem2d` structural model and this
subset's typed mutations (`update-analysis-settings`), in Python, serving as this case's differential oracle.
Relocated out of the artifact-level `mutate-fem2d-1` case in ticket
`26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`, and
extended to the vocabulary's REFUSALS in ticket `26/09/06/FEM-PLUGIN-END-TO-END`.

**Why a second implementation and not a third-party library.** What this vocabulary edits is the
MODEL, not the analysis: nine id-keyed collections and one settings record. A finite-element solver
(`code_aster`, `OpenSees`, `anastruct`, `PyNite`) computes displacements and forces from a model;
none of them reads `.dsl.semio`, none defines this document, and none of them has an opinion about
whether deleting a still-referenced material is legal. What a reference genuinely can adjudicate is
the model algebra, and that is what this file implements, from the specification, in another
language. It carries the FULL nine-member model shape — not only this subset's own collections —
because every scenario asserts, in role, that a mutation moved exactly the one member it was meant
to and left the other eight untouched.

**What it was written from.**

* ``../../../🌐️any/🧬️schema/📸️snapshot/🔣️.json`` — `Fem2dSnapshot` is exactly those nine
  members, `additionalProperties: false`. Since the 26/09/06 wave its record `$defs` are no longer
  empty: they carry the field shapes AND the admissibility bounds (`exclusiveMinimum` on a modulus,
  a density, an area, a second moment, a thickness, a mesh size and the deformation scale; the open
  Poisson interval; `minItems: 3` on a region outline; `minimum: 1` on each mode count). Every bound
  this file enforces is read from there.
* ``…/🧬️schema/🧬️mutations/<kind>/🧬️schema/🔣️.json`` — the per-kind wire payloads, internally tagged
  with `mutation`.
* the committed `(before, mutation, after, outcome)` specification vectors — where the referential
  rules are written down: which `delete-` refuses while referrers exist and which is deliberately
  cascade-free, and which diagnostic code and address each refusal carries.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half only.
"""

# region 🔖️Imports
import copy
import json
import math

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("nodes", "elements", "regions", "materials", "sections", "supports", "loadCases", "combinations", "analysis")
"""🗂️ The nine members `Fem2dSnapshot` declares — and the cross-language projection. Every
member is validated on every scenario regardless of which one this subset's kinds write, because
the model always carries all nine."""

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
"""🗂️ Per noun: its collection, the argument `create-` carries, and the one `replace-` carries when
the vocabulary has a `replace-` for it at all."""

KINDS = ("update-analysis-settings",)
"""🏷️ This subset's own kinds, in the catalog's declared order."""

REFUSALS = {"update-analysis-settings": 2}
"""🚫️ How many refusal-or-no-op vectors each kind declares — the `reject-<kind>-<n>` rows the
feature's `@id-reject` Outline carries, numbered in the catalog's own order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}

RECORDS = {
    "nodes": {"id", "x", "y"},
    "elements": {"kind", "id", "start", "end", "materialId", "sectionId"},
    "regions": {"id", "name", "outline", "holes", "thickness", "materialId", "meshSize"},
    "materials": {"id", "name", "e", "nu", "rho"},
    "sections": {"id", "name", "area", "iy"},
    "supports": {"id", "nodeId", "fixed"},
    "loadCases": {"id", "name", "loads", "selfWeight"},
    "combinations": {"id", "name", "terms"},
}
"""🧱️ The members each record carries, as the snapshot schema spells them — the FULL nine-member
model shape, needed to validate the collections this subset's own kinds do not write."""

LOADS = {"nodal": {"kind", "id", "nodeId", "dof", "value"}, "memberUdl": {"kind", "id", "elementId", "wx", "wy"}, "area": {"kind", "id", "regionId", "pressure"}}
"""🏋️ The three load variants, as the schema and the committed vectors spell them."""

DUPLICATE_ID, ID_MISMATCH, INVARIANT = "mutation.duplicate-id", "mutation.id-mismatch", "mutation.invariant"
TARGET_MISSING, TARGET_REFERENCED, NO_OP = "mutation.target-missing", "mutation.target-referenced", "mutation.no-op"
"""🚦️ The closed diagnostic vocabulary. The three Fatal codes say the PAYLOAD is inadmissible on any
base; the two Error codes say THIS base cannot host it; `no-op` is a Warning beside an applied,
empty change."""


class Refusal(AssertionError):
    """🚫️ One diagnostic raised instead of a change — an `AssertionError` so a caller that only
    knows the scenario failed still learns why."""

    def __init__(self, code, level, target, message):
        super().__init__("%s [%s] %s" % (code, level, message))
        self.code, self.level, self.target, self.message = code, level, list(target), message


def fatal(code, target, message):
    raise Refusal(code, "fatal", target, message)


def error(code, target, message):
    raise Refusal(code, "error", target, message)


def warn(code, message):
    raise Refusal(code, "warning", [], message)


# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document):
    """✅️ Holds the model to the shape and the bounds the snapshot schema declares, and to id
    uniqueness within every collection — including load ids within one case."""
    if set(document) != set(MEMBERS):
        raise AssertionError("a fem2d model must carry exactly %r, found %r" % (sorted(MEMBERS), sorted(document)))
    if set(document["analysis"]) != {"modalCount", "bucklingCount", "deformationScale"}:
        raise AssertionError("analysis must carry exactly the three declared settings, found %r" % sorted(document["analysis"]))
    for name, expected in RECORDS.items():
        identifiers = []
        for record in document[name]:
            if set(record) != expected:
                raise AssertionError("a %s record must carry exactly %r, found %r" % (name, sorted(expected), sorted(record)))
            identifiers.append(record["id"])
        if len(set(identifiers)) != len(identifiers):
            raise AssertionError("%s carries a duplicate id: %r" % (name, identifiers))
    for case in document["loadCases"]:
        loads = []
        for load in case["loads"]:
            if load.get("kind") not in LOADS or set(load) != LOADS[load["kind"]]:
                raise AssertionError("load %r of case %r is not one of the three declared variants" % (load, case["id"]))
            loads.append(load["id"])
        if len(set(loads)) != len(loads):
            raise AssertionError("case %r carries a duplicate load id: %r" % (case["id"], loads))


def document_of(payload):
    """📥️ Reads a fem2d model out of a snapshot JSON value."""
    document = copy.deepcopy(payload)
    validate(document)
    return document


def find(items, identifier):
    """🔎️ The index of an id in a collection, or `None`."""
    for at, item in enumerate(items):
        if item["id"] == identifier:
            return at
    return None


def noun_of(kind):
    """🏷️ The noun a `create-`/`delete-`/`replace-` kind names."""
    return kind.split("-", 1)[1]


# endregion 🔖️Document


# region 🔖️Bounds
def finite(*values):
    return all(isinstance(value, (int, float)) and math.isfinite(float(value)) for value in values)


def check_node(record):
    """📍️ The snapshot schema's node coordinates are plain numbers, so a NaN or an infinity is not
    one — a non-finite ordinate poisons every stiffness matrix the node enters."""
    if not finite(record["x"], record["y"]):
        fatal(INVARIANT, [record["id"]], 'Node "%s" carries a non-finite coordinate.' % record["id"])


def check_material(record):
    """🧱️ `e` and `rho` carry `exclusiveMinimum: 0`; `nu` carries the open interval (-1, 0.5), where
    the upper limit is incompressibility and a singular plane constitutive matrix."""
    identifier = record["id"]
    if not finite(record["e"], record["nu"], record["rho"]):
        fatal(INVARIANT, [identifier], 'Material "%s" carries a non-finite property.' % identifier)
    if record["e"] <= 0.0:
        fatal(INVARIANT, [identifier], 'Material "%s" needs a positive Young\'s modulus.' % identifier)
    if record["rho"] <= 0.0:
        fatal(INVARIANT, [identifier], 'Material "%s" needs a positive density.' % identifier)
    if not -1.0 < record["nu"] < 0.5:
        fatal(INVARIANT, [identifier], 'Material "%s" needs a Poisson ratio in (-1, 0.5).' % identifier)


def check_section(record):
    """📏️ `area` and `iy` carry `exclusiveMinimum: 0` — at zero the member has no axial or no
    bending stiffness while still claiming to be there."""
    identifier = record["id"]
    if not finite(record["area"], record["iy"]):
        fatal(INVARIANT, [identifier], 'Section "%s" carries a non-finite property.' % identifier)
    if record["area"] <= 0.0:
        fatal(INVARIANT, [identifier], 'Section "%s" needs a positive area.' % identifier)
    if record["iy"] <= 0.0:
        fatal(INVARIANT, [identifier], 'Section "%s" needs a positive second moment of area.' % identifier)


def ring_area(ring):
    """📐️ The signed shoelace area of a closed ring; the sign carries the winding."""
    total = 0.0
    for at, point in enumerate(ring):
        following = ring[(at + 1) % len(ring)]
        total += point[0] * following[1] - following[0] * point[1]
    return total / 2.0


def inside(point, ring):
    """🎯️ Crossing-number containment with the boundary counted as INSIDE, so a hole may touch the
    outline it is cut from (a notch) but not leave it."""
    for at, start in enumerate(ring):
        end = ring[(at + 1) % len(ring)]
        cross = (end[0] - start[0]) * (point[1] - start[1]) - (end[1] - start[1]) * (point[0] - start[0])
        span = min(start[0], end[0]) - 1e-12 <= point[0] <= max(start[0], end[0]) + 1e-12 and min(start[1], end[1]) - 1e-12 <= point[1] <= max(start[1], end[1]) + 1e-12
        if abs(cross) <= 1e-12 and span:
            return True
    crossings = False
    for at, start in enumerate(ring):
        end = ring[(at + 1) % len(ring)]
        if (start[1] > point[1]) != (end[1] > point[1]):
            crossing = start[0] + (point[1] - start[1]) * (end[0] - start[0]) / (end[1] - start[1])
            if point[0] < crossing:
                crossings = not crossings
    return crossings


def check_region(record):
    """🗺️ `outline` carries `minItems: 3`, `thickness` and `meshSize` carry `exclusiveMinimum: 0`,
    and a hole is a polygon lying inside the outline — otherwise the meshing pass produces an empty
    or self-overlapping triangulation with no diagnostic naming the edit that caused it."""
    identifier = record["id"]
    outline = [tuple(point) for point in record["outline"]]
    if len(outline) < 3:
        fatal(INVARIANT, [identifier], 'Region "%s" needs an outline of at least three points.' % identifier)
    if any(not finite(*point) for point in outline):
        fatal(INVARIANT, [identifier], 'Region "%s" carries a non-finite outline coordinate.' % identifier)
    if ring_area(outline) == 0.0:
        fatal(INVARIANT, [identifier], 'Region "%s" has a degenerate outline enclosing zero area.' % identifier)
    if not finite(record["thickness"]) or record["thickness"] <= 0.0:
        fatal(INVARIANT, [identifier], 'Region "%s" needs a positive thickness.' % identifier)
    if not finite(record["meshSize"]) or record["meshSize"] <= 0.0:
        fatal(INVARIANT, [identifier], 'Region "%s" needs a positive mesh size.' % identifier)
    for at, hole in enumerate(record["holes"]):
        ring = [tuple(point) for point in hole]
        if len(ring) < 3:
            fatal(INVARIANT, [identifier], 'Region "%s" hole %d needs at least three points.' % (identifier, at))
        if any(not finite(*point) for point in ring):
            fatal(INVARIANT, [identifier], 'Region "%s" hole %d carries a non-finite coordinate.' % (identifier, at))
        if ring_area(ring) == 0.0:
            fatal(INVARIANT, [identifier], 'Region "%s" hole %d is degenerate.' % (identifier, at))
        if any(not inside(point, outline) for point in ring):
            fatal(INVARIANT, [identifier], 'Region "%s" hole %d leaves the outline.' % (identifier, at))


def check_analysis(settings):
    """⚙️ Both counts carry `minimum: 1` and the scale `exclusiveMinimum: 0` — asking for zero modes
    asks the solver for an empty spectrum."""
    if settings["modalCount"] < 1:
        fatal(INVARIANT, [], "Analysis settings need at least one modal mode.")
    if settings["bucklingCount"] < 1:
        fatal(INVARIANT, [], "Analysis settings need at least one buckling mode.")
    if not finite(settings["deformationScale"]) or settings["deformationScale"] <= 0.0:
        fatal(INVARIANT, [], "Analysis settings need a positive deformation scale.")


# endregion 🔖️Bounds


# region 🔖️References
def resolve_node(document, node_id):
    if find(document["nodes"], node_id) is None:
        error(TARGET_MISSING, [node_id], 'Node "%s" does not exist.' % node_id)


def resolve_material(document, material_id):
    if find(document["materials"], material_id) is None:
        error(TARGET_MISSING, [material_id], 'Material "%s" does not exist.' % material_id)


def resolve_element(document, record):
    """🔗️ The four foreign keys every element carries, in the order the committed vectors read
    them: `start`, `end`, `materialId`, `sectionId`."""
    resolve_node(document, record["start"])
    resolve_node(document, record["end"])
    resolve_material(document, record["materialId"])
    if find(document["sections"], record["sectionId"]) is None:
        error(TARGET_MISSING, [record["sectionId"]], 'Section "%s" does not exist.' % record["sectionId"])


def resolve_load(document, load):
    """🔗️ The one target a load variant carries — the same resolution whether the load arrives
    inside a new case or is attached to an existing one."""
    if load["kind"] == "nodal":
        resolve_node(document, load["nodeId"])
    elif load["kind"] == "memberUdl":
        if find(document["elements"], load["elementId"]) is None:
            error(TARGET_MISSING, [load["elementId"]], 'Element "%s" does not exist.' % load["elementId"])
    elif find(document["regions"], load["regionId"]) is None:
        error(TARGET_MISSING, [load["regionId"]], 'Region "%s" does not exist.' % load["regionId"])


def loads_naming(document, variant, key, target):
    return ["%s/%s" % (case["id"], load["id"]) for case in document["loadCases"] for load in case["loads"] if load["kind"] == variant and load[key] == target]


REFERRERS = {
    "delete-element": ("Element", "member UDL", lambda document, target: loads_naming(document, "memberUdl", "elementId", target)),
    "delete-material": ("Material", "element or region", lambda document, target: [item["id"] for item in document["elements"] if item["materialId"] == target] + [item["id"] for item in document["regions"] if item["materialId"] == target]),
    "delete-section": ("Section", "element", lambda document, target: [item["id"] for item in document["elements"] if item["sectionId"] == target]),
    "delete-region": ("Region", "area load", lambda document, target: loads_naming(document, "area", "regionId", target)),
    "delete-load-case": ("Load case", "combination term", lambda document, target: [item["id"] for item in document["combinations"] if any(term["caseId"] == target for term in item["terms"])]),
    "delete-combination": ("Combination", "combination term", lambda document, target: [item["id"] for item in document["combinations"] if item["id"] != target and any(term["caseId"] == target for term in item["terms"])]),
}
"""🧷️ The six `delete-` verbs that refuse while referrers exist.

`delete-node` is EXEMPT and the committed vector says so in its own directory name: a plan node is a
drafting coordinate and dropping one an element still names is the SPECIFIED behaviour.
`delete-support` needs no guard at all — no record in this vocabulary names a support id.
"""


def check_referrers(document, kind, target):
    if kind not in REFERRERS:
        return
    label, blocker, lookup = REFERRERS[kind]
    found = lookup(document, target)
    if found:
        error(TARGET_REFERENCED, [target] + found, '%s "%s" is still referenced by %d %s(s): %s.' % (label, target, len(found), blocker, ", ".join(found)))


# endregion 🔖️References


# region 🔖️Mutations
def kind_of(mutation):
    """🏷️ The kind an internally tagged mutation payload names."""
    if not isinstance(mutation, dict) or "mutation" not in mutation:
        raise AssertionError("a mutation carries an internally tagged `mutation` member, found %r" % mutation)
    for kind, tag in TAGS.items():
        if tag == mutation["mutation"]:
            return kind
    raise AssertionError("unknown mutation variant %r" % mutation["mutation"])


def case_of(document, identifier):
    """📋️ One load case, or a refusal — a mutation that addressed nothing is never a silent no-op."""
    at = find(document["loadCases"], identifier)
    if at is None:
        error(TARGET_MISSING, [identifier], 'Load case "%s" does not exist.' % identifier)
    return document["loadCases"][at]


def check_record(document, noun, record):
    """🛡️ The payload validation a noun's `create-`/`replace-` twins SHARE. Running the same
    function from both is the whole point: the two used to disagree, and a reader could not tell
    which one was right."""
    if noun == "node":
        check_node(record)
    elif noun == "element":
        resolve_element(document, record)
    elif noun == "material":
        check_material(record)
    elif noun == "section":
        check_section(record)
    elif noun == "support":
        resolve_node(document, record["nodeId"])
    elif noun == "region":
        resolve_material(document, record["materialId"])
        check_region(record)
    elif noun == "load-case":
        for load in record["loads"]:
            resolve_load(document, load)
    elif noun == "combination":
        for term in record["terms"]:
            known = find(document["loadCases"], term["caseId"]) is not None or find(document["combinations"], term["caseId"]) is not None
            if not known:
                error(TARGET_MISSING, [term["caseId"]], 'Load case or combination "%s" does not exist.' % term["caseId"])


def apply_mutation(document, mutation):
    """🧬️ Applies one typed mutation, returning the resulting model or raising a [`Refusal`]."""
    kind = kind_of(mutation)
    result = copy.deepcopy(document)
    if kind == "update-analysis-settings":
        check_analysis(mutation["settings"])
        if mutation["settings"] == result["analysis"]:
            warn(NO_OP, "Analysis settings are unchanged.")
        result["analysis"] = copy.deepcopy(mutation["settings"])
    elif kind in ("add-load", "remove-load", "change-load-case-self-weight"):
        case = case_of(result, mutation["caseId"])
        if kind == "add-load":
            load = copy.deepcopy(mutation["load"])
            resolve_load(result, load)
            if find(case["loads"], load["id"]) is not None:
                warn(NO_OP, 'Load "%s" already exists in case "%s".' % (load["id"], case["id"]))
            case["loads"].append(load)
        elif kind == "remove-load":
            at = find(case["loads"], mutation["loadId"])
            if at is None:
                error(TARGET_MISSING, [mutation["loadId"]], 'Load "%s" does not exist in case "%s".' % (mutation["loadId"], case["id"]))
            case["loads"].pop(at)
        else:
            if case["selfWeight"] == mutation["newSelfWeight"]:
                warn(NO_OP, 'Load case "%s" self-weight is already set.' % case["id"])
            case["selfWeight"] = mutation["newSelfWeight"]
    else:
        noun = noun_of(kind)
        collection, create_argument, replace_argument = COLLECTIONS[noun]
        items = result[collection]
        if kind.startswith("create-"):
            record = copy.deepcopy(mutation[create_argument])
            if find(items, record["id"]) is not None:
                fatal(DUPLICATE_ID, [record["id"]], 'A %s with id "%s" already exists.' % (noun, record["id"]))
            check_record(result, noun, record)
            items.append(record)
        elif kind.startswith("delete-"):
            at = find(items, mutation["id"])
            if at is None:
                error(TARGET_MISSING, [mutation["id"]], '%s "%s" does not exist.' % (noun.capitalize(), mutation["id"]))
            check_referrers(result, kind, mutation["id"])
            items.pop(at)
        else:
            at = find(items, mutation["id"])
            if at is None:
                error(TARGET_MISSING, [mutation["id"]], '%s "%s" does not exist.' % (noun.capitalize(), mutation["id"]))
            record = copy.deepcopy(mutation[replace_argument])
            if record["id"] != mutation["id"]:
                fatal(ID_MISMATCH, [mutation["id"], record["id"]], 'A replace-%s may not rename "%s" to "%s".' % (noun, mutation["id"], record["id"]))
            check_record(result, noun, record)
            if items[at] == record:
                warn(NO_OP, '%s "%s" is already equal to the replacement value.' % (noun.capitalize(), mutation["id"]))
            items[at] = record
    validate(result)
    return result


def inverse_mutation(document, mutation):
    """↩️ The mutation that undoes one application, computed against the model it applies to.

    Note what the vocabulary can and cannot express: no `create-` verb carries an index, so the
    inverse of a delete is exact only for a TRAILING record — the feature's rows are chosen
    accordingly and say so.
    """
    kind = kind_of(mutation)
    if kind == "update-analysis-settings":
        return {"mutation": TAGS[kind], "settings": copy.deepcopy(document["analysis"])}
    if kind == "add-load":
        return {"mutation": TAGS["remove-load"], "caseId": mutation["caseId"], "loadId": mutation["load"]["id"]}
    if kind == "remove-load":
        case = case_of(document, mutation["caseId"])
        at = find(case["loads"], mutation["loadId"])
        if at is None:
            raise AssertionError("inverse of %s: case %r carries no load %r" % (kind, case["id"], mutation["loadId"]))
        return {"mutation": TAGS["add-load"], "caseId": mutation["caseId"], "load": copy.deepcopy(case["loads"][at])}
    if kind == "change-load-case-self-weight":
        return {"mutation": TAGS[kind], "caseId": mutation["caseId"], "newSelfWeight": case_of(document, mutation["caseId"])["selfWeight"]}
    noun = noun_of(kind)
    collection, create_argument, replace_argument = COLLECTIONS[noun]
    if kind.startswith("create-"):
        return {"mutation": TAGS["delete-%s" % noun], "id": mutation[create_argument]["id"]}
    at = find(document[collection], mutation["id"])
    if at is None:
        raise AssertionError("inverse of %s: %r is not in %s" % (kind, mutation["id"], collection))
    held = copy.deepcopy(document[collection][at])
    if kind.startswith("delete-"):
        return {"mutation": TAGS["create-%s" % noun], create_argument: held}
    return {"mutation": TAGS[kind], "id": mutation["id"], replace_argument: held}


# endregion 🔖️Mutations


# region 🔖️Laws
def observable(scenario, before, after):
    """👁️ Every FORWARD row moves the model, so a forward application must move it. A mutation that
    quietly did nothing would otherwise agree with an unchanged model and report a pass.

    Deliberately NOT applied to the `reject-` rows: their whole claim is that the model does not
    move, and `refused` below is the law that holds them to it instead."""
    if before == after:
        raise AssertionError("%s: the forward mutation left the model untouched, so nothing was proved" % scenario)


def refused(scenario, before, after, declared, raised):
    """🚫️ The law the refusal rows carry in place of `observable`: the document must be exactly
    where it was, and the diagnostic must be the one the committed `🎯️outcome` declares — code,
    level and address. An implementation that refused for the WRONG reason would otherwise pass on
    an unchanged document alone."""
    if before != after:
        raise AssertionError("%s: a refused or no-op mutation must leave the model exactly where it was" % scenario)
    if declared["status"] == "rejected":
        expected, address = declared["code"], declared.get("path", [])
        if raised["level"] not in ("error", "fatal"):
            raise AssertionError("%s: the vector declares a rejection, this implementation raised it at %r" % (scenario, raised["level"]))
    else:
        messages = declared.get("messages", [])
        if len(messages) != 1:
            raise AssertionError("%s: an applied-but-unchanged vector declares exactly one diagnostic, found %r" % (scenario, messages))
        expected, address = messages[0]["code"], messages[0].get("target", [])
    if raised["code"] != expected:
        raise AssertionError("%s: the vector declares %r, this implementation raised %r" % (scenario, expected, raised["code"]))
    if list(address) != raised["target"]:
        raise AssertionError("%s: the vector declares the address %r, this implementation reported %r" % (scenario, list(address), raised["target"]))


def touches_one(scenario, kind, before, after):
    """🔀️ Each verb writes exactly ONE of the nine members. That is the check an after-snapshot
    comparison cannot make on its own: an implementation that re-derived a sibling collection on
    every edit — renumbering ids, re-sorting sections — would still land on the right value for the
    member it meant to write."""
    if kind == "update-analysis-settings":
        written = "analysis"
    elif kind in ("add-load", "remove-load", "change-load-case-self-weight"):
        written = "loadCases"
    else:
        written = COLLECTIONS[noun_of(kind)][0]
    moved = [name for name in MEMBERS if before[name] != after[name]]
    if moved != [written]:
        raise AssertionError("%s: this verb writes %s and nothing else, but %r moved" % (scenario, written, moved))


def restores(kind, restored, original):
    """↩️ The metamorphic inverse law, reported by the member and index that failed to come back."""
    if restored == original:
        return
    for name in MEMBERS:
        if restored[name] == original[name]:
            continue
        if name == "analysis":
            raise AssertionError("inverse-%s: analysis came back as %r, not %r" % (kind, restored[name], original[name]))
        was = [record["id"] for record in original[name]]
        now = [record["id"] for record in restored[name]]
        if was != now:
            raise AssertionError("inverse-%s: %s came back as %r, not %r" % (kind, name, now, was))
        for at, (left, right) in enumerate(zip(original[name], restored[name])):
            if left != right:
                raise AssertionError("inverse-%s: %s[%d] (%s) came back as %s, not %s" % (kind, name, at, left["id"], json.dumps(right, sort_keys=True)[:200], json.dumps(left, sort_keys=True)[:200]))


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member."""
    for name in MEMBERS:
        if produced[name] != committed[name]:
            raise AssertionError("spec-vector-%s: %s is %s, the committed after-snapshot says %s" % (kind, name, json.dumps(produced[name], sort_keys=True)[:300], json.dumps(committed[name], sort_keys=True)[:300]))


# endregion 🔖️Laws


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
            if token.startswith(("asset://", "shared://📈️mutate-fem2d-1-analysis/", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def json_fixture(ctx, needle):
    """🧫️ The declared JSON fixture this scenario names."""
    return json.loads(ctx.fixture_bytes(uri_in(ctx, needle)).decode("utf-8"))


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to the real derived timber portal frame."""

    def handler(ctx):
        document = document_of(json_fixture(ctx, "timber-portal-frame"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("mutate-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable("mutate-%s" % kind, document, applied)
        touches_one("mutate-%s" % kind, kind, document, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind to the real derived frame and then its OWN computed inverse.

    The projection carries BOTH models; projecting only the restored one would make every row
    project the same value and the differential would be vacuous.
    """

    def handler(ctx):
        document = document_of(json_fixture(ctx, "timber-portal-frame"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("inverse-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable("inverse-%s" % kind, document, applied)
        restored = apply_mutation(applied, inverse_mutation(document, mutation))
        restores(kind, restored, document)
        return outcome_of({"mutated": applied, "restored": restored})

    return handler


def spec_vector_handler(kind, scenario):
    """📐️ Replays one committed handcrafted `(before, mutation, after)` triple for one kind."""

    def handler(ctx):
        before = document_of(json_fixture(ctx, "⬅️before"))
        mutation = json_fixture(ctx, "🦠️mutation")
        after = document_of(json_fixture(ctx, "➡️after"))
        if kind_of(mutation) != kind:
            raise AssertionError("%s: the committed vector carries a %s payload" % (scenario, kind_of(mutation)))
        applied = apply_mutation(before, mutation)
        equals_committed(kind, applied, after)
        observable(scenario, before, applied)
        touches_one(scenario, kind, before, applied)
        restores(kind, apply_mutation(applied, inverse_mutation(before, mutation)), before)
        return outcome_of(applied)

    return handler


def reject_handler(kind, scenario):
    """🚫️ Replays one committed REFUSAL or no-op vector.

    The projection deliberately carries the diagnostic as well as the model. Two implementations
    that both merely decline to move the document would agree vacuously; making them agree on the
    `code`, the `level` and the `target` is what turns these rows into evidence that they refuse
    for the SAME reason.
    """

    def handler(ctx):
        before = document_of(json_fixture(ctx, "⬅️before"))
        mutation = json_fixture(ctx, "🦠️mutation")
        after = document_of(json_fixture(ctx, "➡️after"))
        declared = json_fixture(ctx, "🎯️outcome")
        if kind_of(mutation) != kind:
            raise AssertionError("%s: the committed vector carries a %s payload" % (scenario, kind_of(mutation)))
        if before != after:
            raise AssertionError("%s: a refusal vector's committed after-model must equal its before-model" % scenario)
        try:
            applied = apply_mutation(before, mutation)
            raise AssertionError("%s: this implementation APPLIED a payload the vector declares refused" % scenario)
        except Refusal as raised:
            applied = before
            reported = {"code": raised.code, "level": raised.level, "target": raised.target}
        refused(scenario, before, applied, declared, reported)
        return outcome_of({"model": applied, "refusal": reported})

    return handler


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these
    handlers as subjects too would make the reference its own subject and manufacture a green
    self-comparison.

    Four families, matching the feature's four `Scenario Outline`s: the two real-model laws, the two
    committed happy paths (`spec-vector-` and `frame-vector-`) and every refusal (`reject-…-<n>`).
    """
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
        for prefix in ("spec-vector", "frame-vector"):
            scenario = "%s-%s" % (prefix, kind)
            built = built.oracle(scenario, spec_vector_handler(kind, scenario))
        for index in range(1, REFUSALS[kind] + 1):
            scenario = "reject-%s-%d" % (kind, index)
            built = built.oracle(scenario, reject_handler(kind, scenario))
    return built


# endregion 🔖️Registration
