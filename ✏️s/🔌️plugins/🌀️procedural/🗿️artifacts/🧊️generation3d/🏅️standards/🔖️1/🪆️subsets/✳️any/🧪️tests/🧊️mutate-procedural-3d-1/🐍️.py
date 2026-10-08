#!/usr/bin/env python3
"""🧊️ An INDEPENDENT second implementation of the `s.procedural.generation3d` document and its twenty-two typed
mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `generation3d` document is a COMPOSITION
of two unrelated halves: a `fixture` — a widget/synapse graph with a camera and a SPARSE layout map
keyed by widget id — and a `generation` — a parameter-set history with a selection and a preview text. Every one
of the twenty-two kinds lands in exactly one of those two members. No node-graph library models a graph whose
layout is a side table and whose second half is an unrelated parameter history, and none of them reads
`.dsl.semio`. That this algebra IS adjudicable was settled in this same wave by `mutate-fem2d-1`,
`mutate-fem3d-1` and `mutate-gismap-1`, which took Python second implementations over this same
carrier.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`` — the two members of the
  snapshot and the shape of each half.
* rules 1, 2, 3 and 4 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`.
* each leaf's payload schema `🧬️schema/🧬️mutations/<kind>/🧬️schema/🔣️.json`, whose descriptions state the six
  gesture intents (`change-slider-value`, `drag`/`rotate`/`scale-transforms`, `move-nodes`, `change-widget-input`) and the
  two document scalars of the generation half (`select-generation`, `change-generation-preview`).
* the twenty-two committed `(before, mutation, diff, outcome, after)` quintets, for the verbs and their
  argument lists and for the four things only they state: that this subset tags its mutations
  with a lowerCamel `mutation` member; that `delete-widget` does NOT cascade — it removes the widget and
  DELIBERATELY leaves both the synapse that named it and its layout entry standing, which is why the layout map has
  its own `delete-widget-position` verb; that `create-generation` appends AND selects; and that
  `delete-generation` falls back to the first remaining generation when the one it removed was
  selected.

**A SIBLING NOTE, because the count of second implementations should not be overstated. This file and
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🧪️tests/🌀️mutate-procedural-2d-1/🐍️.py` are
ONE implementation instantiated twice: the two subsets' documents are the same shape and their
vocabularies differ only in NAMES — three kind names (`update-widget`/`update-synapse`/
`delete-widget-position` here against `replace-widget`/`replace-synapse`/`clear-widget-layout` there)
and three argument names. Counting them as two distinct references would overstate the evidence; they
are two instantiations of one, and what they genuinely do is hold both subsets to the SAME semantics
under different spellings. One real divergence surfaced by writing them side by side: `delete-widget`
raises `mutation.cascade` at level `info` in the 2d sibling and raises NOTHING here, for an effect
that is byte-for-byte identical in both committed vectors. A second one, the 2d sibling's snake_case
`question_id`, was fixed at the source: both subsets now spell the argument `questionId`.**

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half
only. All twenty-two kinds are adjudicated and none is refused: this document holds no composed child,
so nothing here depends on a content-addressing function no specification states. The two ceilings of the
vocabulary are stated rather than hidden: a slider value outside its range widens the range through a rule the
payload schema does not state, and a list-typed `change-widget-input` rebuilds a neural list literal, so this
oracle adjudicates the in-range slider value and the scalar input types and refuses the rest loudly.
"""

# region 🔖️Imports
import copy
import json
import math

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("hostSnapshot", "generation")
"""🗂️ The two members the snapshot declares — and the cross-language projection."""

FIXTURE_MEMBERS = {"schema", "camera", "widgets", "synapses", "layout"}
"""🕸️ The members the fixture half carries."""

GENERATION_MEMBERS = {"generations", "selectedGenerationId"}
"""🧬️ The members the generation half always carries; `previewText` is present only while a preview is set."""

GENERATION_OPTIONAL_MEMBERS = {"previewText"}
"""🧬️ The generation-half member that is omitted, not nulled, while it holds nothing."""

KINDS = (
    "create-widget",
    "update-widget",
    "delete-widget",
    "connect-synapse",
    "update-synapse",
    "disconnect-synapse",
    "move-widget",
    "delete-widget-position",
    "update-camera",
    "change-schema",
    "create-generation",
    "delete-generation",
    "rename-generation",
    "change-generation-value",
    "change-slider-value",
    "drag-transforms",
    "rotate-transforms",
    "scale-transforms",
    "move-nodes",
    "change-widget-input",
    "select-generation",
    "change-generation-preview",
)
"""🏷️ Every kind the catalog declares, in its declared order."""


def variant_of(kind):
    """🔤️ The EXTERNALLY tagged variant name of a kind — PascalCase of its words, and the single key
    the committed payload carries."""
    return "".join(word[:1].upper() + word[1:] for word in kind.split("-"))


VARIANTS = {kind: variant_of(kind) for kind in KINDS}
REPLACE_WIDGET = "update-widget"
REPLACE_SYNAPSE = "update-synapse"
CLEAR_LAYOUT = "delete-widget-position"

TRANSLATE_KINDS = ("brep.xform.translate", "brep.mesh.translate", "brep.mesh.translateComponents")
ROTATE_KINDS = ("brep.xform.rotate", "brep.mesh.rotate", "brep.mesh.rotateComponents")
SCALE_KINDS = ("brep.xform.scale", "brep.mesh.scale", "brep.mesh.scaleComponents")
"""🎛️ The operator kinds one gumball gesture composes into: the B-Rep transform, the mesh transform and the mesh component transform."""

ARGUMENTS = {"schema": "newSchema", "name": "newName", "questionId": "questionId", "value": "newValue"}
"""🔤️ What this subset calls the four arguments; the 2d sibling spells three of them differently (`schema`, `name`, `value`) and `questionId` alike."""
# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on: two halves, unique widget
    and synapse ids, unique generation ids, a selection that names a generation the history holds, and
    a layout map whose keys are strings. It deliberately does NOT require a synapse's endpoints to be
    live widgets: `delete-widget` leaves dangling synapses standing, and that is this subset's own
    rule."""
    if set(document) != set(MEMBERS):
        raise AssertionError("%s: the document must carry exactly %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    fixture, generation = document["hostSnapshot"], document["generation"]
    if set(fixture) != FIXTURE_MEMBERS:
        raise AssertionError("%s: the fixture half must carry exactly %r, found %r" % (where, sorted(FIXTURE_MEMBERS), sorted(fixture)))
    if not GENERATION_MEMBERS <= set(generation) <= GENERATION_MEMBERS | GENERATION_OPTIONAL_MEMBERS:
        raise AssertionError("%s: the generation half must carry %r and at most %r, found %r" % (where, sorted(GENERATION_MEMBERS), sorted(GENERATION_OPTIONAL_MEMBERS), sorted(generation)))
    if set(fixture["camera"]) != {"x", "y", "zoom"}:
        raise AssertionError("%s: the camera must carry exactly x, y and zoom" % where)
    for member in ("widgets", "synapses"):
        identifiers = [record["id"] for record in fixture[member]]
        if len(set(identifiers)) != len(identifiers):
            raise AssertionError("%s: %s carries a duplicate id in %r" % (where, member, identifiers))
    identifiers = [record["id"] for record in generation["generations"]]
    if len(set(identifiers)) != len(identifiers):
        raise AssertionError("%s: generations carries a duplicate id in %r" % (where, identifiers))
    selected = generation["selectedGenerationId"]
    if selected is not None and selected not in identifiers:
        raise AssertionError("%s: the selection names %r, which the history does not hold" % (where, selected))


def index_in(rows, identity, member, kind, where):
    """🔎️ The index of the record this kind addresses; an absent id is an error, never a no-op."""
    for at, record in enumerate(rows):
        if record["id"] == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses %s %r, which the before-snapshot does not hold" % (where, kind, member, identity))
# endregion 🔖️Document


# region 🔖️Gestures
def literal_axes(params, key, fallback):
    """🧮️ The `x`/`y`/`z` of one vector or point literal of an operator's params, or `fallback` where absent."""
    held = params.get(key)
    if not isinstance(held, dict):
        return list(fallback)
    return [held.get(axis, fallback[at]) for at, axis in enumerate("xyz")]


def literal_number(params, key, fallback):
    """🔟️ The `value` of one number literal of an operator's params, or `fallback` where absent."""
    held = params.get(key)
    return held.get("value", fallback) if isinstance(held, dict) else fallback


def vector_literal(schema, axes):
    """🧩️ One typed vector or point literal, `{"$schema": schema, x, y, z}`."""
    return {"$schema": schema, "x": axes[0], "y": axes[1], "z": axes[2]}


def operators(fixture, targets, kinds, kind):
    """🎯️ The index of every target that is an operator of `kinds`. The gesture addresses at least one operator and names
    each once; a target the document does not hold, or one of another kind, is skipped, and none left is an error."""
    if not targets or len(set(targets)) != len(targets) or any(not identity for identity in targets):
        raise AssertionError("mutate-%s: a gesture names at least one non-empty target, each once" % kind)
    held = []
    for identity in targets:
        for at, widget in enumerate(fixture["widgets"]):
            if widget["id"] == identity and widget.get("kind") == "neuron" and widget["neuronKind"] in kinds:
                held.append(at)
    if not held:
        raise AssertionError("mutate-%s: none of the targets %r is an operator this gesture composes into" % (kind, targets))
    return held


def quaternion(axis, angle):
    """🧭️ The unit quaternion `(x, y, z, w)` of a rotation by `angle` about the non-zero `axis`."""
    scale = max(abs(value) for value in axis)
    if scale == 0.0:
        raise AssertionError("a rotation axis cannot be zero")
    unit = [value / scale for value in axis]
    length = math.hypot(math.hypot(unit[0], unit[1]), unit[2])
    sine, cosine = math.sin(angle * 0.5), math.cos(angle * 0.5)
    return [unit[0] / length * sine, unit[1] / length * sine, unit[2] / length * sine, cosine]


def compose_rotation(current, delta):
    """🔄️ The world-axis rotation `delta` applied after `current`, both `(axis, angle)`, as a normalised `(axis, angle)`; a
    rotation that cancels is the identity about +z."""
    x, y, z, w = quaternion(*current)
    a, b, c, d = quaternion(*delta)
    q = [d * x + a * w + b * z - c * y, d * y - a * z + b * w + c * x, d * z + a * y - b * x + c * w, d * w - a * x - b * y - c * z]
    if q[3] < 0.0:
        q = [-value for value in q]
    sine = math.hypot(math.hypot(q[0], q[1]), q[2])
    if sine == 0.0:
        return [0.0, 0.0, 1.0], 0.0
    return [q[0] / sine, q[1] / sine, q[2] / sine], 2.0 * math.atan2(sine, q[3])


def input_literal(payload):
    """🔣️ The typed literal a `change-widget-input` payload states. Scalar types only: a list rebuilds a neural list
    literal this oracle declines to restate."""
    kind, value = payload["type"], payload["value"]
    if kind in ("number", "text", "boolean"):
        return {"$schema": kind, "value": value}
    if kind in ("point", "vector"):
        return vector_literal(kind, value)
    raise AssertionError("mutate-change-widget-input: the type %r is a list, which this oracle declines to adjudicate" % kind)


def set_input(fixture, payload):
    """🎛️ Sets input `channel` of the addressed operator (or the `text` of a note) to the typed literal. An input the record
    does not hold, one a wire drives, and one holding a literal of another type are errors, never no-ops."""
    at = index_in(fixture["widgets"], payload["id"], "widget", "change-widget-input", "mutate")
    widget = fixture["widgets"][at]
    wired = any(synapse["to"] == payload["id"] and synapse["toPort"] == payload["channel"] for synapse in fixture["synapses"])
    literal = input_literal(payload)
    if wired:
        raise AssertionError("mutate-change-widget-input: input %r of %r is driven by a wire" % (payload["channel"], payload["id"]))
    if widget.get("kind") == "inputNote" and payload["channel"] == "text":
        if payload["type"] != "text":
            raise AssertionError("mutate-change-widget-input: a note's text takes a text literal")
        widget["text"] = payload["value"]
        return
    if widget.get("kind") != "neuron" or payload["channel"] not in widget["params"]:
        raise AssertionError("mutate-change-widget-input: widget %r holds no input %r" % (payload["id"], payload["channel"]))
    held = widget["params"][payload["channel"]]
    if held.get("$schema") != literal["$schema"]:
        raise AssertionError("mutate-change-widget-input: input %r holds a %s literal, not %s" % (payload["channel"], held.get("$schema"), literal["$schema"]))
    widget["params"][payload["channel"]] = literal
# endregion 🔖️Gestures


# region 🔖️Verbs
def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind. Every committed vector of this subset declares `status: applied`, so an
    address the document does not hold is an error rather than a rejection outcome."""
    document = copy.deepcopy(document)
    fixture, generation = document["hostSnapshot"], document["generation"]
    if kind == "create-widget":
        index = payload.get("index")
        fixture["widgets"].insert(len(fixture["widgets"]) if index is None else index, copy.deepcopy(payload["widget"]))
    elif kind == REPLACE_WIDGET:
        fixture["widgets"][index_in(fixture["widgets"], payload["widget"]["id"], "widget", kind, "mutate")] = copy.deepcopy(payload["widget"])
    elif kind == "delete-widget":
        fixture["widgets"].pop(index_in(fixture["widgets"], payload["id"], "widget", kind, "mutate"))
    elif kind == "connect-synapse":
        index = payload.get("index")
        fixture["synapses"].insert(len(fixture["synapses"]) if index is None else index, copy.deepcopy(payload["synapse"]))
    elif kind == REPLACE_SYNAPSE:
        fixture["synapses"][index_in(fixture["synapses"], payload["synapse"]["id"], "synapse", kind, "mutate")] = copy.deepcopy(payload["synapse"])
    elif kind == "disconnect-synapse":
        fixture["synapses"].pop(index_in(fixture["synapses"], payload["id"], "synapse", kind, "mutate"))
    elif kind == "move-widget":
        fixture["layout"][payload["id"]] = copy.deepcopy(payload["layout"])
    elif kind == CLEAR_LAYOUT:
        if payload["id"] not in fixture["layout"]:
            raise AssertionError("mutate-%s: the layout map holds no entry for %r" % (kind, payload["id"]))
        fixture["layout"].pop(payload["id"])
    elif kind == "update-camera":
        fixture["camera"] = copy.deepcopy(payload["camera"])
    elif kind == "change-schema":
        fixture["schema"] = payload[ARGUMENTS["schema"]]
    elif kind == "create-generation":
        generation["generations"].append(copy.deepcopy(payload["generation"]))
        generation["selectedGenerationId"] = payload["generation"]["id"]
    elif kind == "delete-generation":
        at = index_in(generation["generations"], payload["id"], "generation", kind, "mutate")
        generation["generations"].pop(at)
        if generation["selectedGenerationId"] == payload["id"]:
            generation["selectedGenerationId"] = generation["generations"][0]["id"] if generation["generations"] else None
    elif kind == "rename-generation":
        generation["generations"][index_in(generation["generations"], payload["id"], "generation", kind, "mutate")]["name"] = payload[ARGUMENTS["name"]]
    elif kind == "change-generation-value":
        generation["generations"][index_in(generation["generations"], payload["id"], "generation", kind, "mutate")]["values"][payload[ARGUMENTS["questionId"]]] = payload[ARGUMENTS["value"]]
    elif kind == "change-slider-value":
        at = index_in(fixture["widgets"], payload["id"], "widget", kind, "mutate")
        widget = fixture["widgets"][at]
        if widget.get("kind") != "inputSlider":
            raise AssertionError("mutate-%s: widget %r is no slider" % (kind, payload["id"]))
        if not widget["min"] <= payload["value"] <= widget["max"]:
            raise AssertionError("mutate-%s: %r leaves the slider range, which widens through a rule the payload schema does not state" % (kind, payload["value"]))
        widget["value"] = payload["value"]
    elif kind == "drag-transforms":
        for at in operators(fixture, payload["targets"], TRANSLATE_KINDS, kind):
            params = fixture["widgets"][at]["params"]
            current = literal_axes(params, "offset", [0.0, 0.0, 0.0])
            params["offset"] = vector_literal("vector", [current[0] + payload["dx"], current[1] + payload["dy"], current[2] + payload["dz"]])
    elif kind == "rotate-transforms":
        for at in operators(fixture, payload["targets"], ROTATE_KINDS, kind):
            params = fixture["widgets"][at]["params"]
            axis, angle = compose_rotation((literal_axes(params, "axis", [0.0, 0.0, 1.0]), literal_number(params, "angle", 0.0)), ([payload["ax"], payload["ay"], payload["az"]], payload["angle"]))
            params["axis"] = vector_literal("vector", axis)
            params["angle"] = {"$schema": "number", "value": angle}
    elif kind == "scale-transforms":
        for at in operators(fixture, payload["targets"], SCALE_KINDS, kind):
            params = fixture["widgets"][at]["params"]
            current = literal_axes(params, "factor", [1.0, 1.0, 1.0])
            params["factor"] = vector_literal("vector", [current[0] * payload["sx"], current[1] * payload["sy"], current[2] * payload["sz"]])
            params["center"] = vector_literal("point", [0.0, 0.0, 0.0])
    elif kind == "move-nodes":
        moved = 0
        for identity in payload["ids"]:
            if any(widget["id"] == identity for widget in fixture["widgets"]) and identity in fixture["layout"]:
                held = fixture["layout"][identity]
                fixture["layout"][identity] = {"x": held["x"] + payload["dx"], "y": held["y"] + payload["dy"]}
                moved += 1
        if not moved:
            raise AssertionError("mutate-%s: none of %r holds a stored position" % (kind, payload["ids"]))
    elif kind == "change-widget-input":
        set_input(fixture, payload)
    elif kind == "select-generation":
        selected = payload["generationId"]
        if selected is not None:
            index_in(generation["generations"], selected, "generation", kind, "mutate")
        generation["selectedGenerationId"] = selected
    elif kind == "change-generation-preview":
        if payload["text"] is None:
            generation.pop("previewText", None)
        else:
            generation["previewText"] = payload["text"]
    else:
        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)
    return document


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary, computed against the
    pre-mutation document. `delete-generation` inverts to `create-generation`, which APPENDS and
    SELECTS — exact only when the removed generation was trailing and selected, which is a property of
    the closed vocabulary rather than of an implementation, and is exactly what the committed vector
    exercises."""
    fixture, generation = document["hostSnapshot"], document["generation"]
    if kind == "create-widget":
        return [("delete-widget", {"id": payload["widget"]["id"]})]
    if kind == REPLACE_WIDGET:
        return [(kind, {"widget": copy.deepcopy(fixture["widgets"][index_in(fixture["widgets"], payload["widget"]["id"], "widget", kind, "inverse")])})]
    if kind == "delete-widget":
        at = index_in(fixture["widgets"], payload["id"], "widget", kind, "inverse")
        return [("create-widget", {"widget": copy.deepcopy(fixture["widgets"][at]), "index": at})]
    if kind == "connect-synapse":
        return [("disconnect-synapse", {"id": payload["synapse"]["id"]})]
    if kind == REPLACE_SYNAPSE:
        return [(kind, {"synapse": copy.deepcopy(fixture["synapses"][index_in(fixture["synapses"], payload["synapse"]["id"], "synapse", kind, "inverse")])})]
    if kind == "disconnect-synapse":
        at = index_in(fixture["synapses"], payload["id"], "synapse", kind, "inverse")
        return [("connect-synapse", {"synapse": copy.deepcopy(fixture["synapses"][at]), "index": at})]
    if kind == "move-widget":
        held = fixture["layout"].get(payload["id"])
        if held is None:
            return [(CLEAR_LAYOUT, {"id": payload["id"]})]
        return [(kind, {"id": payload["id"], "layout": copy.deepcopy(held)})]
    if kind == CLEAR_LAYOUT:
        return [("move-widget", {"id": payload["id"], "layout": copy.deepcopy(fixture["layout"][payload["id"]])})]
    if kind == "update-camera":
        return [(kind, {"camera": copy.deepcopy(fixture["camera"])})]
    if kind == "change-schema":
        return [(kind, {ARGUMENTS["schema"]: fixture["schema"]})]
    if kind == "create-generation":
        return [("delete-generation", {"id": payload["generation"]["id"]})]
    if kind == "delete-generation":
        at = index_in(generation["generations"], payload["id"], "generation", kind, "inverse")
        return [("create-generation", {"generation": copy.deepcopy(generation["generations"][at])})]
    if kind == "rename-generation":
        return [(kind, {"id": payload["id"], ARGUMENTS["name"]: generation["generations"][index_in(generation["generations"], payload["id"], "generation", kind, "inverse")]["name"]})]
    if kind == "change-generation-value":
        held = generation["generations"][index_in(generation["generations"], payload["id"], "generation", kind, "inverse")]["values"]
        question = payload[ARGUMENTS["questionId"]]
        if question not in held:
            raise AssertionError("inverse-%s: the committed vector sets the answer %r, which the generation did not hold, and this vocabulary has no verb that REMOVES an answer" % (kind, question))
        return [(kind, {"id": payload["id"], ARGUMENTS["questionId"]: question, ARGUMENTS["value"]: held[question]})]
    if kind == "change-slider-value":
        return [(REPLACE_WIDGET, {"widget": copy.deepcopy(fixture["widgets"][index_in(fixture["widgets"], payload["id"], "widget", kind, "inverse")])})]
    if kind in ("drag-transforms", "rotate-transforms", "scale-transforms"):
        kinds = {"drag-transforms": TRANSLATE_KINDS, "rotate-transforms": ROTATE_KINDS, "scale-transforms": SCALE_KINDS}[kind]
        return [(REPLACE_WIDGET, {"widget": copy.deepcopy(fixture["widgets"][at])}) for at in operators(fixture, payload["targets"], kinds, kind)]
    if kind == "move-nodes":
        return [("move-widget", {"id": identity, "layout": copy.deepcopy(fixture["layout"][identity])}) for identity in payload["ids"] if any(widget["id"] == identity for widget in fixture["widgets"]) and identity in fixture["layout"]]
    if kind == "change-widget-input":
        return [(REPLACE_WIDGET, {"widget": copy.deepcopy(fixture["widgets"][index_in(fixture["widgets"], payload["id"], "widget", kind, "inverse")])})]
    if kind == "select-generation":
        return [(kind, {"generationId": generation["selectedGenerationId"]})]
    if kind == "change-generation-preview":
        return [(kind, {"text": generation.get("previewText")})]
    raise AssertionError("inverse-%s: this implementation declares no inverse for that kind" % kind)
# endregion 🔖️Verbs


# region 🔖️Laws
def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, half by half, with no tolerance and no ignored key."""
    for member in MEMBERS:
        if produced[member] != committed[member]:
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced[member], sort_keys=True)[:400], json.dumps(committed[member], sort_keys=True)[:400]))


def observable(kind, before, after):
    """👁️ Every committed vector of this subset declares `status: applied`, so every one must move
    the compared projection."""
    if before == after:
        raise AssertionError("mutate-%s: the committed vector declares this kind applied, yet the document did not move" % kind)


def touches_one(kind, before, after):
    """🎯️ Every kind in this vocabulary writes exactly ONE of the two halves — the property the
    no-oracle decision this file replaces named as the reason no reference could exist, restated here
    as a law a reference asserts."""
    moved = [member for member in MEMBERS if before[member] != after[member]]
    if len(moved) != 1:
        raise AssertionError("mutate-%s: moved %r; every kind in this vocabulary writes exactly one half" % (kind, moved))


def restores(kind, restored, original):
    """↩️ The full inverse law: applying the kind and then its OWN computed inverse must land back on
    the committed before-snapshot, half for half and index for index."""
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
    return json.loads(ctx.input_bytes(spec[name]).decode("utf-8"))


def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def payload_of(spec_mutation, kind):
    """🦠️ The committed payload. This subset tags INTERNALLY: the record carries its discriminator as the
    `mutation` member (the lowerCamel variant name) beside its arguments."""
    tag = VARIANTS[kind][:1].lower() + VARIANTS[kind][1:]
    if spec_mutation.get("mutation") != tag:
        raise AssertionError("mutate-%s: the committed vector is tagged %r, not %r" % (kind, spec_mutation.get("mutation"), tag))
    return {key: value for key, value in spec_mutation.items() if key != "mutation"}


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))
# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to its committed before-snapshot and asserts, in role, the committed
    after-snapshot, the declared status, observability and the single-half footprint."""

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
        touches_one(kind, before, applied)
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
    additionally requires, in role, that it really is the two-halved document this case describes: a
    widget graph with a synapse and a SPARSE layout map that does not cover every widget, beside a
    generation history whose selection names a generation it holds."""
    uri = uri_in(ctx, "⬅️before")
    committed = ctx.input_bytes(uri)
    document = json.loads(committed.decode("utf-8"))
    validate(document, "identity-round-trip")
    fixture, generation = document["hostSnapshot"], document["generation"]
    if not fixture["widgets"] or not generation["generations"]:
        raise AssertionError("identity-round-trip: the committed document must carry widgets and a generation history")
    if len(fixture["layout"]) >= len(fixture["widgets"]):
        raise AssertionError("identity-round-trip: the committed layout map covers every widget, so this document would not exercise the sparse-layout rule this vocabulary is built around")
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
