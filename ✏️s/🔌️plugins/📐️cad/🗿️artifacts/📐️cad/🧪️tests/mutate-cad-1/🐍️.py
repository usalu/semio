#!/usr/bin/env python3
"""📐️ An INDEPENDENT second implementation of the `s.cad.cad` document and its twenty typed mutations,
in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `cad` document holds no geometry at
all: it is a COMPOSITION — four fixed child slots (`shapeModel`, `buildingModel`, `energyModel`,
`structureClassicModel`), one child collection (`drawings`), a flat node tree, and reference lists
filed per model-definition id. Every one of those children is a HANDLE, `{childId, target}`, and the
target is written on the wire as a single string `"<artifactId>!<artifactKind>@<standard>/<subset>"`
that the snapshot carries expanded into a record. No CAD interchange format models a document whose
whole content is handles to other documents, and none of them reads `.dsl.semio`. That this algebra
IS adjudicable was settled in this same wave by `mutate-gismap-1`, which took a Python second
implementation over this same carrier.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`` — the members of
  `CadSnapshot` and which of them are optional.
* rules 1, 2 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`.
* the twenty committed `(before, mutation, diff, outcome, after)` quintets, for the verbs and their
  argument lists and for the four things only they state: that a `create-<slot>-model` verb carries
  its child id EXPLICITLY rather than content-addressing it, so it can be reproduced; that it
  REHANDLES an occupied slot instead of refusing; that a `delete-<slot>-model` REMOVES the member from
  the document rather than nulling it; and that `replace-reference-media` rewrites five members of one
  reference and leaves its `origin`, `widthWorld`, `hidden` and `locked` alone.

**No Rust was read to write this.** `🦀️component.rs` beside this file registers the SUBJECT half
only. All twenty kinds are adjudicated and none is refused: unlike `s.block.3d` or `s.architect.program`,
every child id in this vocabulary is supplied by the caller, so nothing here depends on a
content-addressing function no specification states.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
REQUIRED = ("schema", "id", "drawings", "referencesByModelDefinitionId", "nodes", "activeModelDefinitionId")
"""🗂️ The members every committed `CadSnapshot` carries."""

SLOTS = {"shape-model": "shapeModel", "building-model": "buildingModel", "energy-model": "energyModel", "structure-classic-model": "structureClassicModel"}
"""🧷 The four fixed child slots. Each is OPTIONAL: `delete-<slot>` removes the member outright, so a
document without it is well formed."""

MEMBERS = REQUIRED + tuple(SLOTS.values())
"""🗂️ Every member that may appear — and the cross-language projection."""

REFERENCE_FIELDS = {"change-reference-hidden": ("hidden", "newHidden"), "change-reference-locked": ("locked", "newLocked"), "change-reference-width": ("widthWorld", "newWidthWorld"), "move-reference": ("origin", "newOrigin")}
"""🖼️ The four single-field reference setters, all addressed by `(modelDefinitionId, referenceId)`."""

MEDIA_FIELDS = (("sourceUrl", "newSourceUrl"), ("mediaKind", "newMediaKind"), ("orientation", "newOrientation"), ("scale", "newScale"), ("opacity", "newOpacity"))
"""🎞️ `replace-reference-media` rewrites exactly these five members of one reference and leaves its
placement and flags alone — which is what separates it from `replace-references`, the whole-list swap."""

KINDS = (
    "create-shape-model",
    "delete-shape-model",
    "create-building-model",
    "delete-building-model",
    "create-energy-model",
    "delete-energy-model",
    "create-structure-classic-model",
    "delete-structure-classic-model",
    "create-drawing",
    "delete-drawing",
    "create-node",
    "delete-node",
    "rename-node",
    "change-reference-hidden",
    "change-reference-locked",
    "change-reference-width",
    "move-reference",
    "replace-reference-media",
    "replace-references",
    "change-active-model-definition",
)
"""🏷️ Every kind the catalog declares, in its declared order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}
# endregion 🔖️Vocabulary


# region 🔖️Document
def parse_target(target, where):
    """🎯️ `"<artifactId>!<artifactKind>@<standard>/<subset>"` expanded into the record a snapshot
    carries. The wire spelling appears only in the mutation payloads; the snapshot always holds it
    expanded, so a reader that never split it could not reproduce a single `create-` vector."""
    if "!" not in target:
        raise AssertionError("%s: a child target must be \"<artifactId>!<artifactKind>@<standard>/<subset>\", found %r" % (where, target))
    artifact_id, dialect = target.split("!", 1)
    if "@" not in dialect or "/" not in dialect:
        raise AssertionError("%s: a child target's dialect must be \"<artifactKind>@<standard>/<subset>\", found %r" % (where, dialect))
    artifact_kind, rest = dialect.split("@", 1)
    standard, subset = rest.split("/", 1)
    return {"artifactId": artifact_id, "dialect": {"artifactKind": artifact_kind, "standard": standard, "subset": subset}}


def print_target(handle):
    """🎯️ The inverse spelling, so a `delete-` verb can be undone with the `create-` verb's own
    argument."""
    dialect = handle["target"]["dialect"]
    return "%s!%s@%s/%s" % (handle["target"]["artifactId"], dialect["artifactKind"], dialect["standard"], dialect["subset"])


def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on: the six always-present
    members, only the four declared optional slots beyond them, well-formed child handles, unique node
    and drawing ids, and unique reference ids inside each model-definition list."""
    if not set(REQUIRED) <= set(document):
        raise AssertionError("%s: a cad document must carry %r, found %r" % (where, sorted(REQUIRED), sorted(document)))
    if not set(document) <= set(MEMBERS):
        raise AssertionError("%s: a cad document may carry only %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    for slot in SLOTS.values():
        if slot in document:
            check_handle(document[slot], where, slot)
    identifiers = []
    for handle in document["drawings"]:
        check_handle(handle, where, "drawings")
        identifiers.append(handle["childId"])
    if len(set(identifiers)) != len(identifiers):
        raise AssertionError("%s: drawings carries a duplicate childId in %r" % (where, identifiers))
    node_ids = [node["id"] for node in document["nodes"]]
    if len(set(node_ids)) != len(node_ids):
        raise AssertionError("%s: nodes carries a duplicate id in %r" % (where, node_ids))
    for definition, references in document["referencesByModelDefinitionId"].items():
        reference_ids = [reference["id"] for reference in references]
        if len(set(reference_ids)) != len(reference_ids):
            raise AssertionError("%s: the %r reference list carries a duplicate id in %r" % (where, definition, reference_ids))


def check_handle(handle, where, member):
    """🧷 One child handle: exactly a `childId` and an expanded `target`."""
    if set(handle) != {"childId", "target"}:
        raise AssertionError("%s: the %s child handle must carry exactly childId and target, found %r" % (where, member, sorted(handle)))
    if set(handle["target"]) != {"artifactId", "dialect"} or set(handle["target"]["dialect"]) != {"artifactKind", "standard", "subset"}:
        raise AssertionError("%s: the %s child handle's target is not an expanded dialect record" % (where, member))


def references_of(document, definition, kind, where):
    """🔎️ The reference list this kind addresses."""
    if definition not in document["referencesByModelDefinitionId"]:
        raise AssertionError("%s-%s: the committed vector addresses the %r reference list, which the before-snapshot does not hold" % (where, kind, definition))
    return document["referencesByModelDefinitionId"][definition]


def reference_at(references, identity, kind, where):
    """🔎️ The index of the reference this kind addresses."""
    for at, reference in enumerate(references):
        if reference["id"] == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses reference %r, which the before-snapshot does not hold" % (where, kind, identity))


def node_at(document, identity, kind, where):
    """🔎️ The index of the node this kind addresses."""
    for at, node in enumerate(document["nodes"]):
        if node["id"] == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses node %r, which the before-snapshot does not hold" % (where, kind, identity))
# endregion 🔖️Document


# region 🔖️Verbs
def slot_of(kind):
    """🧷 The member a slot-lifecycle verb addresses, or `None`."""
    for noun, member in SLOTS.items():
        if kind in ("create-" + noun, "delete-" + noun):
            return member
    return None


def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind. Every committed vector of this subset declares `status: applied`, so an
    address the document does not hold is an error rather than a rejection outcome."""
    document = copy.deepcopy(document)
    slot = slot_of(kind)
    if slot is not None:
        if kind.startswith("create-"):
            document[slot] = {"childId": payload["childId"], "target": parse_target(payload["target"], "mutate-%s" % kind)}
        elif slot in document:
            document.pop(slot)
        else:
            raise AssertionError("mutate-%s: the before-snapshot holds no %s slot to vacate" % (kind, slot))
    elif kind == "create-drawing":
        document["drawings"].append({"childId": payload["childId"], "target": parse_target(payload["target"], "mutate-%s" % kind)})
    elif kind == "delete-drawing":
        at = next((index for index, handle in enumerate(document["drawings"]) if handle["childId"] == payload["childId"]), None)
        if at is None:
            raise AssertionError("mutate-%s: the before-snapshot holds no drawing %r" % (kind, payload["childId"]))
        document["drawings"].pop(at)
    elif kind == "create-node":
        document["nodes"].append(copy.deepcopy(payload["node"]))
    elif kind == "delete-node":
        document["nodes"].pop(node_at(document, payload["nodeId"], kind, "mutate"))
    elif kind == "rename-node":
        document["nodes"][node_at(document, payload["nodeId"], kind, "mutate")]["label"] = payload["newLabel"]
    elif kind in REFERENCE_FIELDS:
        member, argument = REFERENCE_FIELDS[kind]
        references = references_of(document, payload["modelDefinitionId"], kind, "mutate")
        references[reference_at(references, payload["referenceId"], kind, "mutate")][member] = copy.deepcopy(payload[argument])
    elif kind == "replace-reference-media":
        references = references_of(document, payload["modelDefinitionId"], kind, "mutate")
        reference = references[reference_at(references, payload["referenceId"], kind, "mutate")]
        for member, argument in MEDIA_FIELDS:
            reference[member] = copy.deepcopy(payload[argument])
    elif kind == "replace-references":
        references_of(document, payload["modelDefinitionId"], kind, "mutate")
        document["referencesByModelDefinitionId"][payload["modelDefinitionId"]] = copy.deepcopy(payload["references"])
    elif kind == "change-active-model-definition":
        document["activeModelDefinitionId"] = payload["newModelDefinitionId"]
    else:
        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)
    return document


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary, computed against the
    pre-mutation document. A `create-<slot>` over an OCCUPIED slot inverts to another `create-<slot>`
    carrying the displaced handle, not to a `delete-` — which is only expressible because the child id
    is the caller's rather than a content address."""
    slot = slot_of(kind)
    if slot is not None:
        if kind.startswith("create-"):
            if slot not in document:
                return [("delete-" + kind[len("create-"):], {})]
            return [(kind, {"childId": document[slot]["childId"], "target": print_target(document[slot])})]
        if slot not in document:
            raise AssertionError("inverse-%s: the before-snapshot holds no %s slot to vacate" % (kind, slot))
        return [("create-" + kind[len("delete-"):], {"childId": document[slot]["childId"], "target": print_target(document[slot])})]
    if kind == "create-drawing":
        return [("delete-drawing", {"childId": payload["childId"]})]
    if kind == "delete-drawing":
        handle = next(held for held in document["drawings"] if held["childId"] == payload["childId"])
        return [("create-drawing", {"childId": handle["childId"], "target": print_target(handle)})]
    if kind == "create-node":
        return [("delete-node", {"nodeId": payload["node"]["id"]})]
    if kind == "delete-node":
        return [("create-node", {"node": copy.deepcopy(document["nodes"][node_at(document, payload["nodeId"], kind, "inverse")])})]
    if kind == "rename-node":
        return [(kind, {"nodeId": payload["nodeId"], "newLabel": document["nodes"][node_at(document, payload["nodeId"], kind, "inverse")]["label"]})]
    if kind in REFERENCE_FIELDS:
        member, argument = REFERENCE_FIELDS[kind]
        references = references_of(document, payload["modelDefinitionId"], kind, "inverse")
        held = references[reference_at(references, payload["referenceId"], kind, "inverse")]
        return [(kind, {"modelDefinitionId": payload["modelDefinitionId"], "referenceId": payload["referenceId"], argument: copy.deepcopy(held[member])})]
    if kind == "replace-reference-media":
        references = references_of(document, payload["modelDefinitionId"], kind, "inverse")
        held = references[reference_at(references, payload["referenceId"], kind, "inverse")]
        undo = {"modelDefinitionId": payload["modelDefinitionId"], "referenceId": payload["referenceId"]}
        undo.update({argument: copy.deepcopy(held[member]) for member, argument in MEDIA_FIELDS})
        return [(kind, undo)]
    if kind == "replace-references":
        return [(kind, {"modelDefinitionId": payload["modelDefinitionId"], "references": copy.deepcopy(references_of(document, payload["modelDefinitionId"], kind, "inverse"))})]
    if kind == "change-active-model-definition":
        return [(kind, {"newModelDefinitionId": document["activeModelDefinitionId"]})]
    raise AssertionError("inverse-%s: this implementation declares no inverse for that kind" % kind)
# endregion 🔖️Verbs


# region 🔖️Laws
def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member, with no tolerance and no ignored key."""
    for member in sorted(set(produced) | set(committed)):
        if produced.get(member, "⌀") != committed.get(member, "⌀"):
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced.get(member), sort_keys=True)[:400], json.dumps(committed.get(member), sort_keys=True)[:400]))


def observable(kind, before, after):
    """👁️ Every committed vector of this subset declares `status: applied`, so every one must move
    the compared projection. There is no exemption list here, and none is needed."""
    if before == after:
        raise AssertionError("mutate-%s: the committed vector declares this kind applied, yet the document did not move" % kind)


def restores(kind, restored, original):
    """↩️ The full inverse law: applying the kind and then its OWN computed inverse must land back on
    the committed before-snapshot, member for member and index for index."""
    for member in sorted(set(restored) | set(original)):
        if restored.get(member, "⌀") != original.get(member, "⌀"):
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, member, json.dumps(restored.get(member), sort_keys=True)[:400], json.dumps(original.get(member), sort_keys=True)[:400]))
# endregion 🔖️Laws


# region 🔖️Plan
def doc_json(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def leaf(ctx, spec, name):
    """🧫️ One committed leaf of the vector the doc string addresses."""
    return json.loads(ctx.fixture_bytes(spec[name]).decode("utf-8"))


def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "local://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def payload_of(spec_mutation, kind):
    """🦠️ The committed payload, checked to carry this kind's own internally tagged discriminator."""
    if spec_mutation.get("mutation") != TAGS[kind]:
        raise AssertionError("mutate-%s: the committed vector carries a %r payload" % (kind, spec_mutation.get("mutation")))
    return {key: value for key, value in spec_mutation.items() if key != "mutation"}


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))
# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to its committed before-snapshot and asserts, in role, the committed
    after-snapshot, the declared status and observability."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec["kind"] != kind:
            raise AssertionError("mutate-%s: the feature's doc string states %r" % (kind, spec["kind"]))
        before = leaf(ctx, spec, "before")
        after = leaf(ctx, spec, "after")
        outcome = leaf(ctx, spec, "outcome")
        if outcome.get("status") != "applied":
            raise AssertionError("mutate-%s: the committed outcome declares %r; this feature replays applied vectors only" % (kind, outcome.get("status")))
        validate(before, "mutate-%s" % kind)
        applied = apply_mutation(before, kind, payload_of(leaf(ctx, spec, "mutation"), kind))
        validate(applied, "mutate-%s" % kind)
        equals_committed(kind, applied, after)
        observable(kind, before, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind and then its OWN computed inverse and requires the committed before-snapshot
    back — the full inverse law, which the subject half of this case cannot assert because it never
    applies anything."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec["kind"] != kind:
            raise AssertionError("inverse-%s: the feature's doc string states %r" % (kind, spec["kind"]))
        before = leaf(ctx, spec, "before")
        payload = payload_of(leaf(ctx, spec, "mutation"), kind)
        validate(before, "inverse-%s" % kind)
        current = apply_mutation(before, kind, payload)
        for step_kind, step_payload in inverse_mutation(before, kind, payload):
            current = apply_mutation(current, step_kind, step_payload)
        restores(kind, current, before)
        return outcome_of(current)

    return handler


def identity_handler(ctx):
    """🔁️ Reads the committed document and answers with the whole of it. This implementation
    additionally requires, in role, that it really is the composition this case describes: all four
    fixed child slots occupied, a drawing child, a node tree and a reference list filed under the
    active model definition — and that every child target the snapshot carries expanded really does
    round-trip through the single-string wire spelling the mutation payloads use."""
    uri = uri_in(ctx, "⬅️before")
    committed = ctx.fixture_bytes(uri)
    document = json.loads(committed.decode("utf-8"))
    validate(document, "identity-round-trip")
    for slot in SLOTS.values():
        if slot not in document:
            raise AssertionError("identity-round-trip: the committed document must occupy the %s slot" % slot)
    if not document["drawings"] or not document["nodes"]:
        raise AssertionError("identity-round-trip: the committed document must carry a drawing child and a node tree")
    if document["activeModelDefinitionId"] not in document["referencesByModelDefinitionId"]:
        raise AssertionError("identity-round-trip: the active model definition %r has no reference list" % document["activeModelDefinitionId"])
    for member in list(SLOTS.values()) + ["drawings"]:
        for handle in ([document[member]] if member in SLOTS.values() else document[member]):
            if parse_target(print_target(handle), "identity-round-trip") != handle["target"]:
                raise AssertionError("identity-round-trip: the %s child target does not survive the single-string wire spelling" % member)
    reserialized = json.dumps(document, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    if reserialized == committed:
        raise AssertionError("identity-round-trip: the committed file is pretty-printed and this writer is compact, so reproducing its bytes exactly would mean the handler returned the input unread")
    reparsed = json.loads(reserialized.decode("utf-8"))
    if reparsed != document:
        raise AssertionError("identity-round-trip: serializing and re-reading the document moved it")
    return Outcome(reparsed, raw=reserialized)
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
    return built.oracle("identity-round-trip", identity_handler)
# endregion 🔖️Registration
