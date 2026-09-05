#!/usr/bin/env python3
"""💠️ An INDEPENDENT second implementation of the `s.lowpoly.lowpoly` document and its seventeen typed
mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `lowpoly` document has exactly two
top-level members — `schema` and an id-keyed `objects` list — and every object carries an
INDEX-keyed, anonymous `paintLayers` stack whose pixels are a base64 byte buffer edited by OFFSET
RUNS. Two levels of addressing, one by id and one by index, in one vocabulary. No mesh or scene
library models a paint stack addressed by index inside an object addressed by id, and none of them
reads `.dsl.semio`. That this algebra IS adjudicable was settled in this same wave by
`mutate-fem3d-1` and `🏔️mutate-gisterrain-1`, which took Python second implementations over this same
carrier.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`` — the two members of
  `LowpolySnapshot` and the shape of an object.
* rules 2, 3 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` — the
  id-keyed object collection, the INDEX-keyed paint-layer collection with `insert`/`remove`, and
  absolute `move`/`rotate`/`scale`.
* the seventeen committed `(before, mutation, diff, outcome, after)` quintets, for the verbs and their
  argument lists and for the three things only they state: that this subset tags its mutations
  EXTERNALLY — the payload is `{"MoveObject": {…}}`, a PascalCase variant name as the single key,
  where every sibling subset in this repository tags internally with a `"mutation"` member; that
  `edit-paint-layer` splices base64 RUNS into the layer's pixel buffer at byte offsets, overwriting in
  place and never resizing it; and that `create-mesh` carries a `meshWorkspace` argument the snapshot
  does not hold at all.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half
only. All seventeen kinds are adjudicated and none is refused: the mesh child handle carries the
caller's own `childId`, so nothing here depends on a content-addressing function no specification
states.
"""

# region 🔖️Imports
import base64
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "objects")
"""🗂️ The two members `LowpolySnapshot` declares — and the cross-language projection."""

OBJECT_MEMBERS = {"id", "name", "transform", "smoothShading", "mesh", "paintLayers"}
"""🧊 The members each object carries, as the committed vectors spell them."""

LAYER_MEMBERS = {"name", "visible", "opacity", "blendMode", "pixels"}
"""🎨 The members each paint layer carries. It has no id: the stack is addressed by INDEX."""

OBJECT_FIELDS = {"rename-object": ("name", "newName"), "change-object-smooth-shading": ("smoothShading", "newSmoothShading")}
"""✏️ The two single-field object setters."""

TRANSFORM_FIELDS = {"move-object": ("position", "newPosition"), "rotate-object": ("rotation", "newRotation"), "scale-object": ("scale", "newScale")}
"""📍 The three placement setters, each writing one member of the object's transform."""

LAYER_FIELDS = {"rename-paint-layer": ("name", "newName"), "change-paint-layer-visible": ("visible", "newVisible"), "change-paint-layer-opacity": ("opacity", "newOpacity"), "change-paint-layer-blend-mode": ("blendMode", "newBlendMode")}
"""🖌️ The four single-field paint-layer setters, all addressed by `(objectId, index)`."""

KINDS = (
    "create-object",
    "delete-object",
    "reorder-objects",
    "rename-object",
    "change-object-smooth-shading",
    "move-object",
    "rotate-object",
    "scale-object",
    "create-mesh",
    "delete-mesh",
    "insert-paint-layer",
    "remove-paint-layer",
    "rename-paint-layer",
    "change-paint-layer-visible",
    "change-paint-layer-opacity",
    "change-paint-layer-blend-mode",
    "edit-paint-layer",
)
"""🏷️ Every kind the catalog declares, in its declared order."""


def variant_of(kind):
    """🔤️ The EXTERNALLY tagged variant name of a kind — PascalCase of its words, and the single key
    the committed payload carries."""
    return "".join(word[:1].upper() + word[1:] for word in kind.split("-"))


VARIANTS = {kind: variant_of(kind) for kind in KINDS}
# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on: two members, unique object
    ids, complete object and layer records, and every layer's pixels a decodable base64 buffer."""
    if set(document) != set(MEMBERS):
        raise AssertionError("%s: a lowpoly document must carry exactly %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    identifiers = []
    for record in document["objects"]:
        if set(record) != OBJECT_MEMBERS:
            raise AssertionError("%s: an object must carry exactly %r, found %r" % (where, sorted(OBJECT_MEMBERS), sorted(record)))
        if set(record["transform"]) != {"position", "rotation", "scale"}:
            raise AssertionError("%s: object %r has no position/rotation/scale transform" % (where, record["id"]))
        if record["mesh"] is not None and set(record["mesh"]) != {"childId", "target"}:
            raise AssertionError("%s: object %r carries a malformed mesh child handle" % (where, record["id"]))
        identifiers.append(record["id"])
        for at, layer in enumerate(record["paintLayers"]):
            if set(layer) != LAYER_MEMBERS:
                raise AssertionError("%s: paint layer %d of object %r must carry exactly %r, found %r" % (where, at, record["id"], sorted(LAYER_MEMBERS), sorted(layer)))
            pixels(layer, "%s: paint layer %d of object %r" % (where, at, record["id"]))
    if len(set(identifiers)) != len(identifiers):
        raise AssertionError("%s: objects carries a duplicate id in %r" % (where, identifiers))


def pixels(layer, where):
    """🎨 A layer's pixel buffer, decoded from its base64 spelling."""
    try:
        return base64.b64decode(layer["pixels"], validate=True)
    except Exception as error:
        raise AssertionError("%s: pixels is not base64 (%s)" % (where, error))


def object_at(document, identity, kind, where):
    """🔎️ The index of the object this kind addresses; an absent id is an error, never a no-op."""
    for at, record in enumerate(document["objects"]):
        if record["id"] == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses object %r, which the before-snapshot does not hold" % (where, kind, identity))


def layer_at(record, index, kind, where):
    """🔎️ One paint layer by INDEX, held to the stack's real bounds."""
    if not 0 <= index < len(record["paintLayers"]):
        raise AssertionError("%s-%s: the committed vector addresses paint layer %d of object %r, whose stack holds %d" % (where, kind, index, record["id"], len(record["paintLayers"])))
    return record["paintLayers"][index]
# endregion 🔖️Document


# region 🔖️Verbs
def spliced(buffer, runs, where):
    """✂️ `edit-paint-layer` overwrites base64 RUNS into the pixel buffer at byte offsets, in the order
    the payload lists them, and never resizes it — a run that would run past the end is an error, not
    a growth."""
    held = bytearray(buffer)
    for run in runs:
        chunk = base64.b64decode(run["bytes"], validate=True)
        offset = run["offset"]
        if offset < 0 or offset + len(chunk) > len(held):
            raise AssertionError("%s: a run of %d bytes at offset %d does not fit a %d-byte layer" % (where, len(chunk), offset, len(held)))
        held[offset:offset + len(chunk)] = chunk
    return bytes(held)


def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind. Every committed vector of this subset declares `status: applied`, so an
    address the document does not hold is an error rather than a rejection outcome."""
    document = copy.deepcopy(document)
    if kind == "create-object":
        index = payload.get("index")
        document["objects"].insert(len(document["objects"]) if index is None else index, copy.deepcopy(payload["object"]))
    elif kind == "delete-object":
        document["objects"].pop(object_at(document, payload["id"], kind, "mutate"))
    elif kind == "reorder-objects":
        at = object_at(document, payload["id"], kind, "mutate")
        record = document["objects"].pop(at)
        document["objects"].insert(payload["toIndex"], record)
    elif kind in OBJECT_FIELDS:
        member, argument = OBJECT_FIELDS[kind]
        document["objects"][object_at(document, payload["id"], kind, "mutate")][member] = payload[argument]
    elif kind in TRANSFORM_FIELDS:
        member, argument = TRANSFORM_FIELDS[kind]
        document["objects"][object_at(document, payload["id"], kind, "mutate")]["transform"][member] = copy.deepcopy(payload[argument])
    elif kind == "create-mesh":
        document["objects"][object_at(document, payload["id"], kind, "mutate")]["mesh"] = {"childId": payload["childId"], "target": copy.deepcopy(payload["target"])}
    elif kind == "delete-mesh":
        document["objects"][object_at(document, payload["id"], kind, "mutate")]["mesh"] = None
    elif kind == "insert-paint-layer":
        record = document["objects"][object_at(document, payload["objectId"], kind, "mutate")]
        index = payload.get("index")
        record["paintLayers"].insert(len(record["paintLayers"]) if index is None else index, copy.deepcopy(payload["layer"]))
    elif kind == "remove-paint-layer":
        record = document["objects"][object_at(document, payload["objectId"], kind, "mutate")]
        layer_at(record, payload["index"], kind, "mutate")
        record["paintLayers"].pop(payload["index"])
    elif kind in LAYER_FIELDS:
        member, argument = LAYER_FIELDS[kind]
        record = document["objects"][object_at(document, payload["objectId"], kind, "mutate")]
        layer_at(record, payload["index"], kind, "mutate")[member] = payload[argument]
    elif kind == "edit-paint-layer":
        record = document["objects"][object_at(document, payload["objectId"], kind, "mutate")]
        layer = layer_at(record, payload["layerIndex"], kind, "mutate")
        where = "mutate-%s: layer %d of object %r" % (kind, payload["layerIndex"], payload["objectId"])
        layer["pixels"] = base64.b64encode(spliced(pixels(layer, where), payload["runs"], where)).decode("ascii")
    else:
        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)
    return document


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary, computed against the
    pre-mutation document. `edit-paint-layer` inverts by capturing the SAME byte ranges out of the
    pre-mutation buffer, which is exact because the verb never resizes the layer."""
    if kind == "create-object":
        return [("delete-object", {"id": payload["object"]["id"]})]
    if kind == "delete-object":
        at = object_at(document, payload["id"], kind, "inverse")
        return [("create-object", {"object": copy.deepcopy(document["objects"][at]), "index": at})]
    if kind == "reorder-objects":
        return [(kind, {"id": payload["id"], "toIndex": object_at(document, payload["id"], kind, "inverse")})]
    if kind in OBJECT_FIELDS:
        member, argument = OBJECT_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: document["objects"][object_at(document, payload["id"], kind, "inverse")][member]})]
    if kind in TRANSFORM_FIELDS:
        member, argument = TRANSFORM_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: copy.deepcopy(document["objects"][object_at(document, payload["id"], kind, "inverse")]["transform"][member])})]
    if kind == "create-mesh":
        held = document["objects"][object_at(document, payload["id"], kind, "inverse")]["mesh"]
        if held is None:
            return [("delete-mesh", {"id": payload["id"]})]
        return [(kind, {"id": payload["id"], "childId": held["childId"], "target": copy.deepcopy(held["target"]), "meshWorkspace": ""})]
    if kind == "delete-mesh":
        held = document["objects"][object_at(document, payload["id"], kind, "inverse")]["mesh"]
        if held is None:
            return []
        return [("create-mesh", {"id": payload["id"], "childId": held["childId"], "target": copy.deepcopy(held["target"]), "meshWorkspace": ""})]
    if kind == "insert-paint-layer":
        index = payload.get("index")
        record = document["objects"][object_at(document, payload["objectId"], kind, "inverse")]
        return [("remove-paint-layer", {"objectId": payload["objectId"], "index": len(record["paintLayers"]) if index is None else index})]
    if kind == "remove-paint-layer":
        record = document["objects"][object_at(document, payload["objectId"], kind, "inverse")]
        return [("insert-paint-layer", {"objectId": payload["objectId"], "index": payload["index"], "layer": copy.deepcopy(layer_at(record, payload["index"], kind, "inverse"))})]
    if kind in LAYER_FIELDS:
        member, argument = LAYER_FIELDS[kind]
        record = document["objects"][object_at(document, payload["objectId"], kind, "inverse")]
        return [(kind, {"objectId": payload["objectId"], "index": payload["index"], argument: layer_at(record, payload["index"], kind, "inverse")[member]})]
    if kind == "edit-paint-layer":
        record = document["objects"][object_at(document, payload["objectId"], kind, "inverse")]
        layer = layer_at(record, payload["layerIndex"], kind, "inverse")
        buffer = pixels(layer, "inverse-%s" % kind)
        runs = []
        for run in payload["runs"]:
            length = len(base64.b64decode(run["bytes"], validate=True))
            runs.append({"offset": run["offset"], "bytes": base64.b64encode(buffer[run["offset"]:run["offset"] + length]).decode("ascii")})
        return [(kind, {"objectId": payload["objectId"], "layerIndex": payload["layerIndex"], "runs": list(reversed(runs))})]
    raise AssertionError("inverse-%s: this implementation declares no inverse for that kind" % kind)
# endregion 🔖️Verbs


# region 🔖️Laws
def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member, with no tolerance and no ignored key."""
    for member in MEMBERS:
        if produced[member] != committed[member]:
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced[member], sort_keys=True)[:400], json.dumps(committed[member], sort_keys=True)[:400]))


def observable(kind, before, after):
    """👁️ Every committed vector of this subset declares `status: applied`, so every one must move
    the compared projection."""
    if before == after:
        raise AssertionError("mutate-%s: the committed vector declares this kind applied, yet the document did not move" % kind)


def restores(kind, restored, original):
    """↩️ The full inverse law: applying the kind and then its OWN computed inverse must land back on
    the committed before-snapshot, member for member, index for index and byte for byte."""
    for member in MEMBERS:
        if restored[member] != original[member]:
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, member, json.dumps(restored[member], sort_keys=True)[:400], json.dumps(original[member], sort_keys=True)[:400]))
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
    """🦠️ The committed payload. This subset tags EXTERNALLY: the whole record is
    `{"<Variant>": {arguments}}`, so the discriminator is the single key rather than a member."""
    if list(spec_mutation) != [VARIANTS[kind]]:
        raise AssertionError("mutate-%s: the committed vector carries %r, not a single %r arm" % (kind, sorted(spec_mutation), VARIANTS[kind]))
    return spec_mutation[VARIANTS[kind]]


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
    additionally requires, in role, that it really is the two-level document this case describes: two
    objects, one of them carrying a mesh child handle and a paint stack whose pixels decode to a real
    byte buffer, the other carrying neither."""
    uri = uri_in(ctx, "⬅️before")
    committed = ctx.fixture_bytes(uri)
    document = json.loads(committed.decode("utf-8"))
    validate(document, "identity-round-trip")
    if len(document["objects"]) < 2:
        raise AssertionError("identity-round-trip: the committed document must carry two objects, found %d" % len(document["objects"]))
    if not any(record["mesh"] is not None and record["paintLayers"] for record in document["objects"]):
        raise AssertionError("identity-round-trip: no object carries both a mesh child handle and a paint stack")
    if not any(record["mesh"] is None and not record["paintLayers"] for record in document["objects"]):
        raise AssertionError("identity-round-trip: no object carries neither a mesh child handle nor a paint stack")
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
