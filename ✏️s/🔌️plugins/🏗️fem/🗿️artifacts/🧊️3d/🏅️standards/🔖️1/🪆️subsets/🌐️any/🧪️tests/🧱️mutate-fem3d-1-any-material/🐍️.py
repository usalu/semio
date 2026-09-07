#!/usr/bin/env python3
"""🧊 An INDEPENDENT second implementation of the `s.fem.fem3d` structural model and this
subset's typed mutations (`create-material`, `delete-material`, `replace-material`), in Python, serving as this case's differential oracle.
Relocated out of the artifact-level `mutate-fem3d-1` case in ticket
`26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.

**Why a second implementation and not a third-party library.** What this vocabulary edits is the
MODEL, not the analysis: nine id-keyed collections and one settings record, of which this subset
owns 3. A finite-element solver (`code_aster`, `OpenSees`, `anastruct`, `PyNite`) computes
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
* the committed `🎯️outcome` vectors — where the REFUSALS are written down: an identity that is
  already taken, a target that does not resolve, a `replace-` that would rename its target, a
  `delete-` whose target still has referrers, and a value or footprint no solver could assemble.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half only.
"""

# region 🔖️Imports
import copy
import json

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
    "combination": ("combinations", "combination", None),
}
"""🗂️ Per noun: its collection, the argument `create-` carries, and the one `replace-` carries when
the vocabulary has a `replace-` for it at all."""

KINDS = ("create-material", "delete-material", "replace-material")
"""🏷️ This subset's own kinds, in the catalog's declared order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}

REJECT_VECTORS = (
    "dup-material-id-1c0787",
    "nu-at-a-half-8253d2",
    "same-material-950f90",
    "negative-e-84dad7",
    "renames-c24-b60696",
    "glulam-in-use-1208e1",
    "no-such-material-494b10",
)
"""🚫️ The committed vectors of this subset that claim the model does NOT move — every refusal this
vocabulary can raise plus every declared no-op, named by the scenario id the feature's `@id-reject`
rows carry."""

RECORDS = {
    "nodes": {"id", "x", "y", "z"},
    "materials": {"id", "name", "e", "g", "nu", "rho"},
    "sections": {"id", "name", "area", "iy", "iz", "j"},
    "solids": {"id", "name", "outline", "holes", "baseZ", "height", "layers", "meshSize", "materialId"},
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


# region 🔖️Integrity
CASCADE_EXEMPT = ("delete-node",)
"""🕳️ The one verb whose committed vector states that it removes a node under a live element and
leaves the element naming it. Every other `delete-` refuses while a referrer is alive; this one is
specified permissive, so the reference is permissive with it too."""


def every_load(document):
    """🏋️ Every load in the model, case by case, in document order."""
    for case in document["loadCases"]:
        for load in case["loads"]:
            yield load


def referrers(document, collection, identifier):
    """🔗️ Every record id that would be left naming nothing if `identifier` left `collection`.

    Read straight off the record shapes the vectors state: an element names two nodes, a material
    and a section; a support names a node; a solid names a material; the three load variants name a
    node, an element and a solid respectively; a combination weights load cases by id.
    """
    found = []
    if collection == "nodes":
        found += [item["id"] for item in document["elements"] if identifier in (item["start"], item["end"])]
        found += [item["id"] for item in document["supports"] if item["nodeId"] == identifier]
        found += [load["id"] for load in every_load(document) if load["kind"] == "nodal" and load["nodeId"] == identifier]
    elif collection == "elements":
        found += [load["id"] for load in every_load(document) if load["kind"] == "memberUdl" and load["elementId"] == identifier]
    elif collection == "sections":
        found += [item["id"] for item in document["elements"] if item["sectionId"] == identifier]
    elif collection == "solids":
        found += [load["id"] for load in every_load(document) if load["kind"] == "area" and load["solidId"] == identifier]
    elif collection == "materials":
        found += [item["id"] for item in document["elements"] if item["materialId"] == identifier]
        found += [item["id"] for item in document["solids"] if item["materialId"] == identifier]
    elif collection == "loadCases":
        found += [item["id"] for item in document["combinations"] if identifier in item["terms"]]
    return found


def resolves(document, collection, identifier, kind):
    """🔎️ A foreign key the model must already carry — the same demand every verb makes of the same
    reference, whichever verb carries it."""
    if find(document[collection], identifier) is None:
        raise AssertionError("%s: %r is not in %s" % (kind, identifier, collection))


def resolve_element(document, element, kind):
    """🔩️ An element's four foreign keys, in the order the vectors report them."""
    resolves(document, "nodes", element["start"], kind)
    resolves(document, "nodes", element["end"], kind)
    resolves(document, "materials", element["materialId"], kind)
    resolves(document, "sections", element["sectionId"], kind)


def resolve_load(document, load, kind):
    """🎯️ The one thing a load hangs on, per variant."""
    if load["kind"] == "nodal":
        resolves(document, "nodes", load["nodeId"], kind)
    elif load["kind"] == "memberUdl":
        resolves(document, "elements", load["elementId"], kind)
    else:
        resolves(document, "solids", load["solidId"], kind)


def finite(*values):
    """🔢️ JSON cannot spell a non-finite literal, so this only ever fires on a computed value — it is
    stated because the rule is about the model, not about the carrier."""
    return all(isinstance(value, (int, float)) and value == value and abs(value) != float("inf") for value in values)


def ring_area(ring):
    """📏️ Twice the shoelace sum, halved — the signed area of a closed ring."""
    return sum(ring[at][0] * ring[(at + 1) % len(ring)][1] - ring[(at + 1) % len(ring)][0] * ring[at][1] for at in range(len(ring))) / 2.0


def encloses(ring, point):
    """🎯️ Crossing-count containment; a point exactly on an edge is undefined and never authored."""
    inside = False
    for at, corner in enumerate(ring):
        other = ring[(at + 1) % len(ring)]
        if (corner[1] > point[1]) != (other[1] > point[1]) and point[0] < corner[0] + (point[1] - corner[1]) / (other[1] - corner[1]) * (other[0] - corner[0]):
            inside = not inside
    return inside


def admissible(collection, record, kind):
    """🧨️ The values a record must carry to describe something a solver could assemble at all — a
    positive stiffness, a Poisson ratio strictly inside (-1, 0.5), a footprint with area, a positive
    extrusion through at least one layer, and holes that stay inside their outline."""
    if collection == "nodes":
        if not finite(record["x"], record["y"], record["z"]):
            raise AssertionError("%s: node %r must sit at a finite position" % (kind, record["id"]))
    elif collection == "materials":
        if not finite(record["e"], record["g"], record["nu"], record["rho"]) or min(record["e"], record["g"], record["rho"]) <= 0.0:
            raise AssertionError("%s: material %r must carry a positive finite e, g and rho" % (kind, record["id"]))
        if not -1.0 < record["nu"] < 0.5:
            raise AssertionError("%s: material %r must carry a Poisson ratio in (-1, 0.5), not %r" % (kind, record["id"], record["nu"]))
    elif collection == "sections":
        if not finite(record["area"], record["iy"], record["iz"], record["j"]) or min(record["area"], record["iy"], record["iz"], record["j"]) <= 0.0:
            raise AssertionError("%s: section %r must carry a positive finite area, iy, iz and j" % (kind, record["id"]))
    elif collection == "solids":
        if len(record["outline"]) < 3 or not all(finite(point[0], point[1]) for point in record["outline"]) or ring_area(record["outline"]) == 0.0:
            raise AssertionError("%s: solid %r needs a closed outline of at least three points and non-zero area" % (kind, record["id"]))
        if not finite(record["baseZ"], record["height"], record["meshSize"]) or record["height"] <= 0.0 or record["meshSize"] <= 0.0 or record["layers"] < 1:
            raise AssertionError("%s: solid %r needs a positive height, mesh size and layer count" % (kind, record["id"]))
        for hole in record["holes"]:
            if len(hole) < 3 or ring_area(hole) == 0.0 or not all(finite(point[0], point[1]) and encloses(record["outline"], point) for point in hole):
                raise AssertionError("%s: solid %r carries a hole that is degenerate or leaves its outline" % (kind, record["id"]))


def bounded(settings, kind):
    """⚙️ An eigen solve for fewer than one mode returns nothing, and a non-positive display scale
    collapses or mirrors the results view."""
    if settings["modalCount"] < 1 or settings["bucklingCount"] < 1:
        raise AssertionError("%s: at least one modal and one buckling factor are needed, not %r and %r" % (kind, settings["modalCount"], settings["bucklingCount"]))
    if not finite(settings["deformationScale"]) or settings["deformationScale"] <= 0.0:
        raise AssertionError("%s: the deformation scale must be finite and positive, not %r" % (kind, settings["deformationScale"]))


# endregion 🔖️Integrity


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


def apply_mutation(document, mutation):
    """🧬️ Applies one typed mutation, returning the resulting model — or raising when the request is
    one the vocabulary refuses.

    Two verbs are IDEMPOTENT rather than refusing, and their committed vectors say so by declaring a
    no-op warning under an `applied` status: `add-load` with a load id the case already carries, and
    every `replace-`/`change-`/`update-` whose new value is the value already there. Both leave the
    model exactly as it was, which is what a caller asked for either way.
    """
    kind = kind_of(mutation)
    result = copy.deepcopy(document)
    if kind == "update-analysis-settings":
        bounded(mutation["settings"], kind)
        result["analysis"] = copy.deepcopy(mutation["settings"])
    elif kind == "add-load":
        case = case_of(result, mutation["caseId"], kind)
        load = copy.deepcopy(mutation["load"])
        if find(case["loads"], load["id"]) is None:
            resolve_load(result, load, kind)
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
            carried(result, collection, record, kind)
            items.append(record)
        elif kind.startswith("delete-"):
            at = find(items, mutation["id"])
            if at is None:
                raise AssertionError("%s: %r is not in %s" % (kind, mutation["id"], collection))
            blockers = [] if kind in CASCADE_EXEMPT else referrers(result, collection, mutation["id"])
            if blockers:
                raise AssertionError("%s: %r in %s is still referenced by %r" % (kind, mutation["id"], collection, blockers))
            items.pop(at)
        else:
            at = find(items, mutation["id"])
            if at is None:
                raise AssertionError("%s: %r is not in %s" % (kind, mutation["id"], collection))
            record = copy.deepcopy(mutation[replace_argument])
            if record["id"] != mutation["id"]:
                raise AssertionError("%s: a replace selects %r and may not rename it to %r" % (kind, mutation["id"], record["id"]))
            if items[at] != record:
                carried(result, collection, record, kind)
            items[at] = record
    validate(result)
    return result



def carried(document, collection, record, kind):
    """🚚️ Everything a record brings with it — the foreign keys it names and the values it must be
    admissible under — demanded identically of the `create-` and the `replace-` of one noun."""
    if collection == "elements":
        resolve_element(document, record, kind)
    elif collection == "supports":
        resolves(document, "nodes", record["nodeId"], kind)
    elif collection == "solids":
        resolves(document, "materials", record["materialId"], kind)
    elif collection == "loadCases":
        for load in record["loads"]:
            resolve_load(document, load, kind)
    elif collection == "combinations":
        for case_id in sorted(record["terms"]):
            resolves(document, "loadCases", case_id, kind)
    admissible(collection, record, kind)


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
def observable(scenario, before, after, moves=True):
    """👁️ A forward row moves the model, so a forward application must move it — a mutation that
    quietly did nothing would otherwise agree with an unchanged model and report a pass. The law is
    two-sided: pass `moves=False` for a row whose whole claim is that NOTHING happens (a refusal or
    a declared no-op), where a model that moved is the failure."""
    if moves and before == after:
        raise AssertionError("%s: the forward mutation left the model untouched, so nothing was proved" % scenario)
    if not moves and before != after:
        raise AssertionError("%s: the vector claims nothing happens, but the model moved" % scenario)


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
            if token.startswith(("asset://", "local://", "shared://")) and needle in token:
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



def reject_handler():
    """🚫️ Replays a committed vector whose whole claim is that the model does NOT move — either the
    request is refused outright, or it is a declared no-op. Both implementations project the same
    two facts: whether the request was refused, and the model it left behind.

    The refusal REASON is deliberately not projected. Each implementation words it in its own
    language; what the differential can hold them to is the verdict and the document.
    """

    def handler(ctx):
        before = document_of(json_fixture(ctx, "⬅️before"))
        after = document_of(json_fixture(ctx, "➡️after"))
        mutation = json_fixture(ctx, "🦠️mutation")
        kind = kind_of(mutation)
        if kind not in KINDS:
            raise AssertionError("reject: %s is not one of this subset's kinds" % kind)
        equals_committed(kind, before, after)
        try:
            applied = apply_mutation(before, mutation)
        except AssertionError:
            return outcome_of({"refused": True, "model": before})
        observable("reject-%s" % kind, before, applied, moves=False)
        return outcome_of({"refused": False, "model": applied})

    return handler

# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these
    handlers as subjects too would make the reference its own subject and manufacture a green
    self-comparison."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
        built = built.oracle("spec-vector-%s" % kind, spec_vector_handler(kind))
        built = built.oracle("hall-vector-%s" % kind, spec_vector_handler(kind))
    for identifier in REJECT_VECTORS:
        built = built.oracle("reject-%s" % identifier, reject_handler())
    return built


# endregion 🔖️Registration
