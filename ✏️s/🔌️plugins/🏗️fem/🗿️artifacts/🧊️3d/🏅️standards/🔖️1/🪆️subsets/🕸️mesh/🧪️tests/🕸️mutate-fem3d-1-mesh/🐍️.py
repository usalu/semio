#!/usr/bin/env python3
"""🧊 An INDEPENDENT second implementation of the `s.fem.fem3d` structural model and this
subset's typed mutations (`create-node`, `delete-node`, `create-element`, `delete-element`, `replace-element`, `create-section`, `delete-section`, `replace-section`, `create-solid`, `delete-solid`, `replace-solid`), in Python, serving as this case's differential oracle.
Relocated out of the artifact-level `mutate-fem3d-1` case in ticket
`26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.

**Why a second implementation and not a third-party library.** What this vocabulary edits is the
MODEL, not the analysis: nine id-keyed collections and one settings record, of which this subset
owns 11. A finite-element solver (`code_aster`, `OpenSees`, `anastruct`, `PyNite`) computes
displacements and forces from a model; none of them reads `.dsl.semio`, none defines this document.
What a reference genuinely can adjudicate is the model algebra, and that is what this file
implements, from the specification, in another language. It carries the FULL nine-member model
shape — not only this subset's own collections — because every scenario asserts, in role, that a
mutation moved exactly the one member it was meant to and left the other eight untouched.

**What it was written from.**

* ``../../../🌐️any/🧬️schema/📸️snapshot/🔣️.json`` — `Fem3dSnapshot` is exactly those nine
  members, `additionalProperties: false`.
* the committed `(before, mutation, after, outcome)` specification vectors — where the RECORD
  shapes are actually written down, including the one only they state: an `element` is a `frame`
  carrying a `roll` about its own axis OR a `bar` carrying none.
* ``…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`` — this subset's own verbs.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half only.
"""

# region 🔖️Imports
import copy
import json
import math

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("nodes", "elements", "materials", "sections", "solids", "supports", "loadCases", "combinations", "analysis")
"""🗂️ The nine members `Fem3dSnapshot` declares — and the cross-language projection. Every
member is validated on every scenario regardless of which one this subset's kinds write, because
the model always carries all nine."""

COLLECTIONS = {
    "node": ("nodes", "node", "newNode"),
    "element": ("elements", "element", "newElement"),
    "solid": ("solids", "solid", "newSolid"),
    "material": ("materials", "material", "newMaterial"),
    "section": ("sections", "section", "newSection"),
    "support": ("supports", "support", "newSupport"),
    "load-case": ("loadCases", "loadCase", None),
    "combination": ("combinations", "combination", "newCombination"),
}
"""🗂️ Per noun: its collection, the argument `create-` carries, and the one `replace-` carries when
the vocabulary has a `replace-` for it at all."""

KINDS = ("create-node", "delete-node", "create-element", "delete-element", "replace-element", "create-section", "delete-section", "replace-section", "create-solid", "delete-solid", "replace-solid", "replace-node")
"""🏷️ This subset's own kinds, in the catalog's declared order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}

RECORDS = {
    "nodes": {"id", "x", "y", "z"},
    "materials": {"id", "name", "e", "g", "nu", "rho"},
    "sections": {"id", "name", "area", "iy", "iz", "j"},
    "solids": {"id", "name", "outline", "holes", "baseZ", "height", "layers", "meshSize", "materialId", "axis"},
    "supports": {"id", "nodeId", "fixed"},
    "loadCases": {"id", "name", "loads", "selfWeight"},
    "combinations": {"id", "name", "terms"},
}
"""🧱️ The members each record carries, as the committed vectors spell them — the FULL nine-member
model shape, needed to validate the eight collections this subset's own kinds do not write.
`elements` is absent here on purpose — its shape depends on the element kind, see [`ELEMENTS`]."""

ELEMENTS = {"bar": {"kind", "id", "start", "end", "materialId", "sectionId"}, "frame": {"kind", "id", "start", "end", "materialId", "sectionId", "roll"}}
"""🧩️ The two element variants: a `frame` carries a `roll` about its own axis, a `bar` does not."""

LOADS = {"nodal": {"kind", "id", "nodeId", "dof", "value"}, "memberUdl": {"kind", "id", "elementId", "wx", "wy", "wz"}, "area": {"kind", "id", "solidId", "pressure"}}
"""🏋️ The three load variants, as the committed vectors spell them."""

# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document):
    """✅️ Holds the model to the shape the committed vectors agree on, and to id uniqueness within
    every collection — including load ids within one case."""
    if set(document) != set(MEMBERS):
        raise AssertionError("a fem3d model must carry exactly %r, found %r" % (sorted(MEMBERS), sorted(document)))
    if set(document["analysis"]) != {"modalCount", "bucklingCount", "deformationScale"}:
        raise AssertionError("analysis must carry exactly the three declared settings, found %r" % sorted(document["analysis"]))
    identifiers = []
    for element in document["elements"]:
        if element.get("kind") not in ELEMENTS or set(element) != ELEMENTS[element["kind"]]:
            raise AssertionError("an element must be a bar or a frame with exactly its declared members, found %r" % element)
        identifiers.append(element["id"])
    if len(set(identifiers)) != len(identifiers):
        raise AssertionError("elements carries a duplicate id: %r" % identifiers)
    for name, expected in RECORDS.items():
        identifiers = []
        for record in document[name]:
            if set(record) != expected:
                raise AssertionError("a %s record must carry exactly %r, found %r" % (name, sorted(expected), sorted(record)))
            identifiers.append(record["id"])
        if len(set(identifiers)) != len(identifiers):
            raise AssertionError("%s carries a duplicate id: %r" % (name, identifiers))
    for combination in document["combinations"]:
        if not isinstance(combination["terms"], dict):
            raise AssertionError("combination %r carries a case-keyed term MAP, found %r" % (combination["id"], combination["terms"]))
    for case in document["loadCases"]:
        loads = []
        for load in case["loads"]:
            if load.get("kind") not in LOADS or set(load) != LOADS[load["kind"]]:
                raise AssertionError("load %r of case %r is not one of the three declared variants" % (load, case["id"]))
            loads.append(load["id"])
        if len(set(loads)) != len(loads):
            raise AssertionError("case %r carries a duplicate load id: %r" % (case["id"], loads))


def document_of(payload):
    """📥️ Reads a fem3d model out of a snapshot JSON value."""
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


# region 🔖️Mutations
def kind_of(mutation):
    """🏷️ The kind an internally tagged mutation payload names."""
    if not isinstance(mutation, dict) or "mutation" not in mutation:
        raise AssertionError("a mutation carries an internally tagged `mutation` member, found %r" % mutation)
    for kind, tag in TAGS.items():
        if tag == mutation["mutation"]:
            return kind
    raise AssertionError("unknown mutation variant %r" % mutation["mutation"])


def case_of(document, identifier, kind):
    """📋️ One load case, or a rejection — a mutation that addressed nothing is never a silent no-op."""
    at = find(document["loadCases"], identifier)
    if at is None:
        raise AssertionError("%s: no load case %r in the model" % (kind, identifier))
    return document["loadCases"][at]


REFERENCES = {"element": (("start", "nodes"), ("end", "nodes"), ("materialId", "materials"), ("sectionId", "sections")), "solid": (("materialId", "materials"),), "support": (("nodeId", "nodes"),)}
"""🪢️ The foreign keys a record of each noun names, in the order they resolve, and the collection each resolves in."""
CARRIERS = {"nodal": ("nodeId", "nodes", ("value",)), "memberUdl": ("elementId", "elements", ("wx", "wy", "wz")), "area": ("solidId", "solids", ("pressure",))}
"""🪝️ The one carrier each load variant hangs on, and the magnitudes it carries."""


def finite(*values):
    """♾️ Every value is a finite JSON number — an infinite or undefined one poisons every solve it enters."""
    return all(isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value) for value in values)


def ring_area(ring):
    """⭕️ The signed shoelace area of a closed ring, negative when it winds clockwise."""
    return sum(ring[at][0] * ring[(at + 1) % len(ring)][1] - ring[(at + 1) % len(ring)][0] * ring[at][1] for at in range(len(ring))) / 2.0


def ring_contains(ring, point):
    """🎈️ Crossing-count containment; a point exactly on an edge stays undefined, as the classical ray cast leaves it."""
    inside = False
    for at in range(len(ring)):
        corner, other = ring[at], ring[(at + 1) % len(ring)]
        if (corner[1] > point[1]) != (other[1] > point[1]) and point[0] < corner[0] + (point[1] - corner[1]) / (other[1] - corner[1]) * (other[0] - corner[0]):
            inside = not inside
    return inside


def solid_holds(solid):
    """🪨️ The footprint an extrusion mesher can tet-split: a closed ring of non-zero area, a positive height through
    at least one layer, a positive mesh size, and every hole a non-degenerate ring strictly inside the outline."""
    outline = solid["outline"]
    if len(outline) < 3 or not all(finite(*point) for point in outline) or ring_area(outline) == 0:
        return False
    if not finite(solid["baseZ"], solid["height"], solid["meshSize"]) or solid["height"] <= 0 or solid["layers"] < 1 or solid["meshSize"] <= 0:
        return False
    return all(len(hole) >= 3 and ring_area(hole) != 0 and all(finite(*point) and ring_contains(outline, point) for point in hole) for hole in solid["holes"])


def holds(noun, record):
    """🩺️ The value bounds a record must keep for the model to stay solvable — a finite node position; a linear-elastic
    isotropic material (positive e, g and rho, a Poisson ratio in the open interval (-1, 0.5)); a cross-section with a
    positive area and second moments; a meshable solid; finite combination factors."""
    if noun == "node":
        return finite(record["x"], record["y"], record["z"])
    if noun == "material":
        return finite(record["e"], record["g"], record["nu"], record["rho"]) and min(record["e"], record["g"], record["rho"]) > 0 and -1 < record["nu"] < 0.5
    if noun == "section":
        return finite(record["area"], record["iy"], record["iz"], record["j"]) and min(record["area"], record["iy"], record["iz"], record["j"]) > 0
    if noun == "solid":
        return solid_holds(record)
    if noun == "combination":
        return finite(*record["terms"].values())
    return True


def referrers(document, noun, identifier):
    """🧷️ Every record that would dangle if `identifier` left its collection. A node, a support and a combination keep
    none by statement: `delete-node` is cascade-free (its committed vector keeps a frame naming the deleted node), and
    supports and combinations are leaves nothing points back at."""
    loads = [load for case in document["loadCases"] for load in case["loads"]]
    if noun == "element":
        return [load["id"] for load in loads if load["kind"] == "memberUdl" and load["elementId"] == identifier]
    if noun == "solid":
        return [load["id"] for load in loads if load["kind"] == "area" and load["solidId"] == identifier]
    if noun == "section":
        return [element["id"] for element in document["elements"] if element["sectionId"] == identifier]
    if noun == "material":
        return [record["id"] for name in ("elements", "solids") for record in document[name] if record["materialId"] == identifier]
    if noun == "load-case":
        return [combination["id"] for combination in document["combinations"] if identifier in combination["terms"]]
    return []


def land(document, kind, loads):
    """⚓️ Every load names exactly one carrier — a node, an element or a solid — and it must be in the model."""
    for load in loads:
        field, collection, _ = CARRIERS[load["kind"]]
        if find(document[collection], load[field]) is None:
            raise AssertionError("%s: load %r names %r, which is not in %s" % (kind, load["id"], load[field], collection))


def refuse(document, kind, mutation):
    """🛡️ The refusals owed before a mutation may move the model: `mutation.id-mismatch` for a replace that renames its
    target, `mutation.target-missing` for a reference that does not resolve, `mutation.target-referenced` for a delete
    that would leave a referrer dangling, `mutation.invariant` for a value the solver cannot take. Raises on the first;
    an identical replace or settings update is a no-op and passes."""
    if kind == "update-analysis-settings":
        settings = mutation["settings"]
        if settings != document["analysis"] and not (settings["modalCount"] >= 1 and settings["bucklingCount"] >= 1 and finite(settings["deformationScale"]) and settings["deformationScale"] > 0):
            raise AssertionError("%s: %r asks for no mode, no buckling factor or no positive deformation scale" % (kind, settings))
        return
    if kind in ("add-load", "replace-load"):
        load = mutation["load"] if kind == "add-load" else mutation["newLoad"]
        case = find(document["loadCases"], mutation["caseId"])
        if kind == "add-load" and case is not None and find(document["loadCases"][case]["loads"], load["id"]) is not None:
            return
        land(document, kind, [load])
        if kind == "replace-load" and not finite(*(load[name] for name in CARRIERS[load["kind"]][2])):
            raise AssertionError("%s: load %r carries a magnitude that is not finite" % (kind, load["id"]))
        return
    verb, noun = kind.split("-", 1)
    if verb not in ("create", "delete", "replace") or noun not in COLLECTIONS:
        return
    collection, create_argument, replace_argument = COLLECTIONS[noun]
    if verb == "delete":
        held = referrers(document, noun, mutation["id"])
        if held:
            raise AssertionError("%s: %r is still referenced by %r" % (kind, mutation["id"], held))
        return
    record = mutation[create_argument if verb == "create" else replace_argument]
    if verb == "replace":
        if record["id"] != mutation["id"]:
            raise AssertionError("%s: a replace selects %r and may not rename it to %r" % (kind, mutation["id"], record["id"]))
        if record in document[collection]:
            return
    for field, target in REFERENCES.get(noun, ()):
        if find(document[target], record[field]) is None:
            raise AssertionError("%s: %r names %s %r, which is not in %s" % (kind, record["id"], field, record[field], target))
    if noun == "load-case":
        land(document, kind, record["loads"])
    if noun == "combination":
        missing = [case for case in record["terms"] if find(document["loadCases"], case) is None]
        if missing:
            raise AssertionError("%s: %r weights %r, which is not in loadCases" % (kind, record["id"], missing))
    if not holds(noun, record):
        raise AssertionError("%s: %r breaks the %s bounds the solver needs" % (kind, record["id"], noun))


def apply_mutation(document, mutation):
    """🧬️ Applies one typed mutation, returning the resulting model."""
    kind = kind_of(mutation)
    refuse(document, kind, mutation)
    result = copy.deepcopy(document)
    if kind == "update-analysis-settings":
        result["analysis"] = copy.deepcopy(mutation["settings"])
    elif kind == "add-load":
        case = case_of(result, mutation["caseId"], kind)
        load = copy.deepcopy(mutation["load"])
        if find(case["loads"], load["id"]) is None:
            case["loads"].append(load)
    elif kind == "remove-load":
        case = case_of(result, mutation["caseId"], kind)
        at = find(case["loads"], mutation["loadId"])
        if at is None:
            raise AssertionError("%s: case %r carries no load %r" % (kind, case["id"], mutation["loadId"]))
        case["loads"].pop(at)
    elif kind == "change-load-case-self-weight":
        case_of(result, mutation["caseId"], kind)["selfWeight"] = mutation["newSelfWeight"]
    else:
        noun = noun_of(kind)
        collection, create_argument, replace_argument = COLLECTIONS[noun]
        items = result[collection]
        if kind.startswith("create-"):
            record = copy.deepcopy(mutation[create_argument])
            if find(items, record["id"]) is not None:
                raise AssertionError("%s: %r is already in %s" % (kind, record["id"], collection))
            items.append(record)
        elif kind.startswith("delete-"):
            at = find(items, mutation["id"])
            if at is None:
                raise AssertionError("%s: %r is not in %s" % (kind, mutation["id"], collection))
            items.pop(at)
        else:
            at = find(items, mutation["id"])
            if at is None:
                raise AssertionError("%s: %r is not in %s" % (kind, mutation["id"], collection))
            items[at] = copy.deepcopy(mutation[replace_argument])
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
        case = case_of(document, mutation["caseId"], "inverse of %s" % kind)
        at = find(case["loads"], mutation["loadId"])
        if at is None:
            raise AssertionError("inverse of %s: case %r carries no load %r" % (kind, case["id"], mutation["loadId"]))
        return {"mutation": TAGS["add-load"], "caseId": mutation["caseId"], "load": copy.deepcopy(case["loads"][at])}
    if kind == "change-load-case-self-weight":
        return {"mutation": TAGS[kind], "caseId": mutation["caseId"], "newSelfWeight": case_of(document, mutation["caseId"], "inverse of %s" % kind)["selfWeight"]}
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
    """👁️ Every row below moves the model, so a forward application must move it. A mutation that
    quietly did nothing would otherwise agree with an unchanged model and report a pass."""
    if before == after:
        raise AssertionError("%s: the forward mutation left the model untouched, so nothing was proved" % scenario)


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
            if token.startswith(("asset://", "shared://🕸️mutate-fem3d-1-mesh/", "shared://")) and needle in token:
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
    """🎯️ Applies one kind to the real derived steel frame."""

    def handler(ctx):
        document = document_of(json_fixture(ctx, "steel-frame"))
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
        document = document_of(json_fixture(ctx, "steel-frame"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("inverse-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable("inverse-%s" % kind, document, applied)
        restored = apply_mutation(applied, inverse_mutation(document, mutation))
        restores(kind, restored, document)
        return outcome_of({"mutated": applied, "restored": restored})

    return handler


def spec_vector_handler(kind):
    """📐️ Replays the committed handcrafted `(before, mutation, after)` triple for one kind."""

    def handler(ctx):
        before = document_of(json_fixture(ctx, "⬅️before"))
        mutation = json_fixture(ctx, "🦠️mutation")
        after = document_of(json_fixture(ctx, "➡️after"))
        if kind_of(mutation) != kind:
            raise AssertionError("spec-vector-%s: the committed vector carries a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(before, mutation)
        equals_committed(kind, applied, after)
        observable("spec-vector-%s" % kind, before, applied)
        touches_one("spec-vector-%s" % kind, kind, before, applied)
        restores(kind, apply_mutation(applied, inverse_mutation(before, mutation)), before)
        return outcome_of(applied)

    return handler


# endregion 🔖️Handlers


# region 🔖️Registration
def hall_vector(ctx):
    """🏗️ The committed glulam-hall vector for the kind the row names — this reference's spec-vector law on the
    hall model (`@id-hall-vector`)."""
    return spec_vector_handler(ctx.row())(ctx)


def reject(ctx):
    """🚨️ A committed vector both implementations must refuse or declare a no-op (`@id-reject`): the reference either
    raises on it or applies it without moving the model, and the committed after-model is the before-model."""
    before = document_of(json_fixture(ctx, "⬅️before"))
    mutation = json_fixture(ctx, "🦠️mutation")
    if document_of(json_fixture(ctx, "➡️after")) != before:
        raise AssertionError("reject-%s: the committed after-model is not the committed before-model" % ctx.row())
    try:
        applied = apply_mutation(before, mutation)
    except AssertionError:
        return outcome_of(before)
    if applied != before:
        raise AssertionError("reject-%s: the reference applied a mutation the committed vector refuses" % ctx.row())
    return outcome_of(applied)


def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these
    handlers as subjects too would make the reference its own subject and manufacture a green
    self-comparison."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
        built = built.oracle("spec-vector-%s" % kind, spec_vector_handler(kind))
    return built.oracle("hall-vector", hall_vector).oracle("reject", reject)


# endregion 🔖️Registration
