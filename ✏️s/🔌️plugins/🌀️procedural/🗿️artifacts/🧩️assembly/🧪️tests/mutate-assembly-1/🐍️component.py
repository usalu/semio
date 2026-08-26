#!/usr/bin/env python3
"""🧩️ An INDEPENDENT second implementation of the `s.procedural.assembly` document and its nine typed
mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** An `assembly` document is the INPUT to
a wave-function-collapse solve, not its output: a slot lattice, the edges between slots, a set of
module CHILD HANDLES, a per-module weight table and an adjacency rule set, all under one `seed`. A
WFC library computes a collapse; none of them carries the problem statement as a document, and none of
them reads `.dsl.semio`. That this algebra IS adjudicable was settled in this same wave by
`mutate-fem3d-1` and `mutate-gisterrain-1`, which took Python second implementations over this same
carrier.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`` — the six members of the
  snapshot.
* rules 2, 4 and 8 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` — the
  id-keyed slot and rule collections, `connect`/`disconnect` for the edge collection, and the domain
  verbs `change-weight`/`remove-weight` registered as an inverse PAIR.
* the nine committed `(before, mutation, diff, outcome, after)` quintets, for the verbs and their
  argument lists and for the three things only they state: that this subset tags its mutations
  EXTERNALLY, the payload being `{"CreateSlot": {…}}` with a PascalCase variant name as its single
  key; that `delete-slot` DOES cascade into the edges naming it and says so with an `info`-level
  `mutation.cascade` diagnostic — the opposite of `s.procedural.procedural2d`'s `delete-widget`, which
  raises the same code for leaving a dangling synapse standing; and that `change-weight` UPSERTS,
  writing an existing module's weight in place and appending an entry when the table holds none.

**No Rust was read to write this.** `🦀️component.rs` beside this file registers the SUBJECT half
only. All nine kinds are adjudicated and none is refused: the module child handles are only ever read
here, never re-addressed, so nothing depends on a content-addressing function no specification states.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "seed", "slots", "edges", "modules", "weights", "rules")
"""🗂️ The members the snapshot declares — and the cross-language projection."""

KINDS = ("create-slot", "delete-slot", "create-rule", "delete-rule", "change-weight", "remove-weight", "connect-slots", "disconnect-slots", "change-seed")
"""🏷️ Every kind the catalog declares, in its declared order."""


def variant_of(kind):
    """🔤️ The EXTERNALLY tagged variant name of a kind — PascalCase of its words, and the single key
    the committed payload carries."""
    return "".join(word[:1].upper() + word[1:] for word in kind.split("-"))


VARIANTS = {kind: variant_of(kind) for kind in KINDS}

COLLECTIONS = {"create-slot": ("slots", "slot"), "create-rule": ("rules", "rule"), "connect-slots": ("edges", "edge")}
"""🌱 The three indexed append verbs: which member each writes and what its whole-record payload is
called."""

REMOVALS = {"delete-slot": "slots", "delete-rule": "rules", "disconnect-slots": "edges"}
"""🗑️ The three id-addressed removals. `delete-slot` additionally cascades into the edges."""
# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on: seven members, unique slot,
    edge, module, rule and weight ids, every edge between slots the lattice really holds, and every
    weight and rule naming a module the document declares."""
    if set(document) != set(MEMBERS):
        raise AssertionError("%s: an assembly document must carry exactly %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    slots = {slot["id"] for slot in document["slots"]}
    if len(slots) != len(document["slots"]):
        raise AssertionError("%s: slots carries a duplicate id" % where)
    edges = [edge["id"] for edge in document["edges"]]
    if len(set(edges)) != len(edges):
        raise AssertionError("%s: edges carries a duplicate id in %r" % (where, edges))
    for edge in document["edges"]:
        for end in ("fromSlotId", "toSlotId"):
            if edge[end] not in slots:
                raise AssertionError("%s: edge %r names %s %r, which the lattice does not hold" % (where, edge["id"], end, edge[end]))
    modules = {module["childId"] for module in document["modules"]}
    if len(modules) != len(document["modules"]):
        raise AssertionError("%s: modules carries a duplicate childId" % where)
    weighted = [entry["moduleId"] for entry in document["weights"]]
    if len(set(weighted)) != len(weighted):
        raise AssertionError("%s: weights carries a duplicate moduleId in %r" % (where, weighted))
    for entry in document["weights"]:
        if entry["moduleId"] not in modules:
            raise AssertionError("%s: the weight table names module %r, which the document does not declare" % (where, entry["moduleId"]))
    rules = [rule["id"] for rule in document["rules"]]
    if len(set(rules)) != len(rules):
        raise AssertionError("%s: rules carries a duplicate id in %r" % (where, rules))
    for rule in document["rules"]:
        for end in ("moduleAId", "moduleBId"):
            if rule[end] not in modules:
                raise AssertionError("%s: rule %r names %s %r, which the document does not declare" % (where, rule["id"], end, rule[end]))


def index_in(rows, identity, member, kind, where, key="id"):
    """🔎️ The index of the record this kind addresses; an absent id is an error, never a no-op."""
    for at, record in enumerate(rows):
        if record[key] == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses %s %r, which the before-snapshot does not hold" % (where, kind, member, identity))
# endregion 🔖️Document


# region 🔖️Verbs
def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind. Every committed vector of this subset declares `status: applied`, so an
    address the document does not hold is an error rather than a rejection outcome."""
    document = copy.deepcopy(document)
    if kind in COLLECTIONS:
        member, argument = COLLECTIONS[kind]
        index = payload.get("index")
        document[member].insert(len(document[member]) if index is None else index, copy.deepcopy(payload[argument]))
    elif kind in REMOVALS:
        member = REMOVALS[kind]
        at = index_in(document[member], payload["id"], member, kind, "mutate")
        document[member].pop(at)
        if kind == "delete-slot":
            document["edges"] = [edge for edge in document["edges"] if edge["fromSlotId"] != payload["id"] and edge["toSlotId"] != payload["id"]]
    elif kind == "change-weight":
        at = next((index for index, entry in enumerate(document["weights"]) if entry["moduleId"] == payload["module_id"]), None)
        if at is None:
            document["weights"].append({"moduleId": payload["module_id"], "weight": payload["weight"]})
        else:
            document["weights"][at]["weight"] = payload["weight"]
    elif kind == "remove-weight":
        at = index_in(document["weights"], payload["module_id"], "weights", kind, "mutate", key="moduleId")
        document["weights"].pop(at)
    elif kind == "change-seed":
        document["seed"] = payload["seed"]
    else:
        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)
    return document


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary, computed against the
    pre-mutation document. `delete-slot` inverts to SEVERAL steps, because it cascades: the slot is put
    back at its own index first and every severed edge is reconnected after it, at ITS own index."""
    if kind in COLLECTIONS:
        member, argument = COLLECTIONS[kind]
        undo = {"slots": "delete-slot", "rules": "delete-rule", "edges": "disconnect-slots"}[member]
        return [(undo, {"id": payload[argument]["id"]})]
    if kind in REMOVALS:
        member = REMOVALS[kind]
        at = index_in(document[member], payload["id"], member, kind, "inverse")
        redo = {"slots": ("create-slot", "slot"), "rules": ("create-rule", "rule"), "edges": ("connect-slots", "edge")}[member]
        steps = [(redo[0], {redo[1]: copy.deepcopy(document[member][at]), "index": at})]
        if kind == "delete-slot":
            for index, edge in enumerate(document["edges"]):
                if edge["fromSlotId"] == payload["id"] or edge["toSlotId"] == payload["id"]:
                    steps.append(("connect-slots", {"edge": copy.deepcopy(edge), "index": index}))
        return steps
    if kind == "change-weight":
        at = next((index for index, entry in enumerate(document["weights"]) if entry["moduleId"] == payload["module_id"]), None)
        if at is None:
            return [("remove-weight", {"module_id": payload["module_id"]})]
        return [(kind, {"module_id": payload["module_id"], "weight": document["weights"][at]["weight"]})]
    if kind == "remove-weight":
        at = index_in(document["weights"], payload["module_id"], "weights", kind, "inverse", key="moduleId")
        if at != len(document["weights"]) - 1:
            raise AssertionError("inverse-%s: the removed weight entry was not TRAILING, and `change-weight` can only APPEND a missing one, so this vocabulary cannot restore its position" % kind)
        return [("change-weight", {"module_id": payload["module_id"], "weight": document["weights"][at]["weight"]})]
    if kind == "change-seed":
        return [(kind, {"seed": document["seed"]})]
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


def cascades(kind, before, after, declared):
    """🌊 The cascade claim, read off the committed outcome rather than off a list this file keeps: a
    vector that declares `mutation.cascade` must have moved a SECOND member beyond the one its own verb
    addresses, and a vector that does not declare it must have moved exactly one."""
    moved = [member for member in MEMBERS if before[member] != after[member]]
    if declared and len(moved) < 2:
        raise AssertionError("mutate-%s: the committed outcome declares mutation.cascade, yet only %r moved" % (kind, moved))
    if not declared and len(moved) != 1:
        raise AssertionError("mutate-%s: the committed outcome declares no cascade, yet %r moved" % (kind, moved))


def restores(kind, restored, original):
    """↩️ The full inverse law: applying the kind and then its OWN computed inverse must land back on
    the committed before-snapshot, member for member and index for index."""
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


def declares_cascade(outcome):
    """🌊 Whether the committed outcome itself records that the mutation reached a second member."""
    return any(message.get("code") == "mutation.cascade" for message in outcome.get("messages", []))


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))
# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to its committed before-snapshot and asserts, in role, the committed
    after-snapshot, the declared status, observability and the cascade the outcome declares."""

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
        cascades(kind, before, applied, declares_cascade(outcome))
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
    additionally requires, in role, that it really is a WFC PROBLEM STATEMENT and not a solve: two
    slots joined by an edge, two module child handles, a weight table that covers only SOME of them,
    and an adjacency rule naming both."""
    uri = uri_in(ctx, "⬅️before")
    committed = ctx.fixture_bytes(uri)
    document = json.loads(committed.decode("utf-8"))
    validate(document, "identity-round-trip")
    if len(document["slots"]) < 2 or not document["edges"] or len(document["modules"]) < 2 or not document["rules"]:
        raise AssertionError("identity-round-trip: the committed document must carry two slots joined by an edge, two module handles and a rule")
    if len(document["weights"]) >= len(document["modules"]):
        raise AssertionError("identity-round-trip: the committed weight table covers every module, so this document would not exercise the partial-override rule `change-weight` is built around")
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
