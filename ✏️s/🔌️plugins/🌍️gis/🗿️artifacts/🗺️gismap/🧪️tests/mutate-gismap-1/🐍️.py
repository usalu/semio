#!/usr/bin/env python3
"""🗺️ An INDEPENDENT second implementation of the `s.gis.gismap` document and its twelve typed
mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `gis.gismap` document is a
semio-native artifact carried in `.dsl.semio`, and its three collections hold `GisMapFeature`
records whose `data` member the committed schema declares an OPAQUE object
(`"additionalProperties": true`, never inspected by the artifact). `geojson`, `geo` and `gdal` were
surveyed by an earlier wave and declined for exactly that reason: none of them reads this carrier,
and none of them is authoritative over an untyped payload. What a reference CAN adjudicate here is
the collection algebra — insert-at-index, delete-by-id, replace-payload-by-id, move-to-index and
the inverse of each — and that is what this file implements, from the specification, in another
language.

**What it was written from, exhaustively.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`` — `GisMapSnapshot` is
  `positions`, `routes` and `regions`, each an array of `GisMapFeature`; a `GisMapFeature` is a
  string `id` and an open `data` object, `additionalProperties: false` on the record itself.
* ``…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`` — the twelve verbs and their
  positional argument lists: `create-<noun> index`, `delete-<noun> id`,
  `replace-<noun>-data id block`, `reorder-<noun>s id index`.
* the committed `(before, mutation, after, diff, outcome)` specification vectors under
  ``…/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`` — the JSON wire form of each verb
  (externally tagged: ``{"CreatePosition": {"index": …, "item": …}}``) and, in the three
  `reorder-` vectors, the three different displacements that pin MOVE semantics rather than swap.

**What was derived rather than read, and how it is pinned.** The `.dsl.semio` carrier for this
artifact has no prose document of its own, so its layout was derived from the committed example's
bytes: a banner line ``semio gis.gismap.dsl v1``, then one ``key=value`` line per member, where a
feature collection is the hex of the UTF-8 bytes of its compact JSON and a composed child handle is
``[hex(childId),hex(target)]``. The derivation is pinned by `identity-round-trip`, which re-encodes
the committed file byte for byte — which a misreading could not do.

**No Rust was read to write this.** `🦀️component.rs` in this directory registers the SUBJECT half
only; the two answers are produced by two implementations in two languages from one written
specification, which is the entire point of the comparison.

**What the cross-language projection carries, and what it deliberately does not.** The projection is
the three `x-semio-state: artifact` collections the committed JSON Schema declares, and nothing
else. `drawing` and `value` are composed children whose `childId` is a `std::hash::DefaultHasher`
digest, and the standard library documents that hasher's output as UNSPECIFIED — no second
implementation in any language can reproduce it, and pretending otherwise would be the fabrication
this exercise exists to prevent. Those two handles are still asserted exactly, in role, by the Rust
subject against the committed after-snapshot, exactly as before this case was converted; nothing was
relaxed, no `ignoreKeys` was added and no comparison profile was touched.
"""

# region 🔖️Imports
import json

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Vocabulary
COLLECTIONS = ("positions", "routes", "regions")
"""🗂️ The three parallel id-keyed collections `GisMapSnapshot` declares, in schema order."""

NOUNS = {"position": "positions", "route": "routes", "region": "regions"}
"""🏷️ The grammar's singular noun per collection — `create-position` edits `positions`."""

KINDS = (
    "create-position",
    "delete-position",
    "replace-position-data",
    "reorder-positions",
    "create-route",
    "delete-route",
    "replace-route-data",
    "reorder-routes",
    "create-region",
    "delete-region",
    "replace-region-data",
    "reorder-regions",
)
"""🏷️ Every kind the catalog declares, spelled as the feature's expanded scenario ids spell them."""

TAGS = {
    "create-position": "CreatePosition",
    "delete-position": "DeletePosition",
    "replace-position-data": "ReplacePositionData",
    "reorder-positions": "ReorderPositions",
    "create-route": "CreateRoute",
    "delete-route": "DeleteRoute",
    "replace-route-data": "ReplaceRouteData",
    "reorder-routes": "ReorderRoutes",
    "create-region": "CreateRegion",
    "delete-region": "DeleteRegion",
    "replace-region-data": "ReplaceRegionData",
    "reorder-regions": "ReorderRegions",
}
"""🔤️ The externally tagged wire name of each kind, as the committed vectors spell it."""

# endregion 🔖️Vocabulary


# region 🔖️Carrier
BANNER = "semio gis.gismap.dsl v1"


def decode_hex_json(payload):
    """🔎️ A collection member of the carrier: hex of the UTF-8 bytes of compact JSON."""
    if payload == "":
        return []
    return json.loads(bytes.fromhex(payload).decode("utf-8"))


def encode_hex_json(value):
    """#⃣ The inverse of :func:`decode_hex_json`, in the carrier's own compact form."""
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False).encode("utf-8").hex()


def parse_carrier(text):
    """📖️ Reads a `gis.gismap.dsl v1` document into `(document, opaque-tail)`.

    The tail is every member the snapshot schema does not declare — `drawing`, `image`, `value` —
    kept verbatim so the document can be printed back without inventing a digest this language
    cannot compute.
    """
    lines = text.split("\n")
    if not lines or lines[0] != BANNER:
        raise AssertionError("carrier banner must be %r, found %r" % (BANNER, lines[0] if lines else ""))
    document = {name: [] for name in COLLECTIONS}
    tail = []
    seen = set()
    for line in lines[1:]:
        if line == "":
            continue
        key, separator, value = line.partition("=")
        if separator != "=":
            raise AssertionError("carrier line %r is not a key=value member" % line)
        if key in COLLECTIONS:
            document[key] = decode_hex_json(value)
            seen.add(key)
        else:
            tail.append((key, value))
    missing = [name for name in COLLECTIONS if name not in seen]
    if missing:
        raise AssertionError("the carrier declares no %s member(s)" % ", ".join(missing))
    validate(document)
    return document, tail


def print_carrier(document, tail):
    """🖨️ Prints a document back in the carrier's canonical order: banner, three collections, tail."""
    rendered = [BANNER]
    rendered.extend("%s=%s" % (name, encode_hex_json(document[name])) for name in COLLECTIONS)
    rendered.extend("%s=%s" % (key, value) for key, value in tail)
    return "\n".join(rendered) + "\n"


# endregion 🔖️Carrier


# region 🔖️Document
def validate(document):
    """✅️ Holds the document to the committed JSON Schema: three arrays of `{id, data}` records."""
    for name in COLLECTIONS:
        members = document.get(name)
        if not isinstance(members, list):
            raise AssertionError("%s must be an array, found %r" % (name, type(members).__name__))
        for member in members:
            if not isinstance(member, dict) or set(member) != {"id", "data"}:
                raise AssertionError("a %s member must be exactly {id, data}, found %r" % (name, sorted(member) if isinstance(member, dict) else member))
            if not isinstance(member["id"], str):
                raise AssertionError("a %s member's id must be a string, found %r" % (name, member["id"]))
            if not isinstance(member["data"], dict):
                raise AssertionError("a %s member's data must be an object, found %r" % (name, member["data"]))
        identifiers = [member["id"] for member in members]
        if len(set(identifiers)) != len(identifiers):
            raise AssertionError("%s carries a duplicate id: %r" % (name, identifiers))


def document_of(payload):
    """📥️ Reads the three declared collections out of a committed snapshot JSON value."""
    document = {name: payload.get(name) or [] for name in COLLECTIONS}
    validate(document)
    return document


def index_of(members, identifier):
    """🔎️ The position of an id in a collection, or `None` when it is absent."""
    for position, member in enumerate(members):
        if member["id"] == identifier:
            return position
    return None


# endregion 🔖️Document


# region 🔖️Mutations
def split(kind):
    """✂️ The `(collection, verb)` a kind names, from the grammar's own naming recipe."""
    for noun, collection in NOUNS.items():
        if kind == "create-%s" % noun:
            return collection, "create"
        if kind == "delete-%s" % noun:
            return collection, "delete"
        if kind == "replace-%s-data" % noun:
            return collection, "replace"
        if kind == "reorder-%ss" % noun:
            return collection, "reorder"
    raise AssertionError("no such kind in this vocabulary: %r" % kind)


def kind_of(mutation):
    """🏷️ The kind an externally tagged mutation payload names."""
    if not isinstance(mutation, dict) or len(mutation) != 1:
        raise AssertionError("a mutation is exactly one externally tagged variant, found %r" % mutation)
    tag = next(iter(mutation))
    for kind, name in TAGS.items():
        if name == tag:
            return kind
    raise AssertionError("unknown mutation variant %r" % tag)


def apply_mutation(document, mutation):
    """🧬️ Applies one typed mutation, returning the resulting document.

    Every rejection is an error rather than a silent no-op: a quietly skipped mutation would report
    a pass having done nothing, which is precisely the failure a differential exists to catch.
    """
    kind = kind_of(mutation)
    collection, verb = split(kind)
    payload = mutation[TAGS[kind]]
    members = [dict(member) for member in document[collection]]
    if verb == "create":
        item = payload["item"]
        if index_of(members, item["id"]) is not None:
            raise AssertionError("%s: %r is already in %s" % (kind, item["id"], collection))
        at = int(payload["index"])
        if at < 0 or at > len(members):
            raise AssertionError("%s: index %d is outside 0..=%d" % (kind, at, len(members)))
        members.insert(at, json.loads(json.dumps(item)))
    elif verb == "delete":
        at = index_of(members, payload["id"])
        if at is None:
            raise AssertionError("%s: %r is not in %s" % (kind, payload["id"], collection))
        members.pop(at)
    elif verb == "replace":
        at = index_of(members, payload["id"])
        if at is None:
            raise AssertionError("%s: %r is not in %s" % (kind, payload["id"], collection))
        members[at] = {"id": payload["id"], "data": json.loads(json.dumps(payload["newData"]))}
    else:
        at = index_of(members, payload["id"])
        if at is None:
            raise AssertionError("%s: %r is not in %s" % (kind, payload["id"], collection))
        to = int(payload["toIndex"])
        if to < 0 or to >= len(members):
            raise AssertionError("%s: toIndex %d is outside 0..%d" % (kind, to, len(members)))
        members.insert(to, members.pop(at))
    result = dict(document)
    result[collection] = members
    validate(result)
    return result


def inverse_mutation(document, mutation):
    """↩️ The mutation that undoes one application, computed against the document it applies to."""
    kind = kind_of(mutation)
    collection, verb = split(kind)
    payload = mutation[TAGS[kind]]
    members = document[collection]
    if verb == "create":
        return {TAGS["delete-%s" % singular(collection)]: {"id": payload["item"]["id"]}}
    if verb == "delete":
        at = index_of(members, payload["id"])
        if at is None:
            raise AssertionError("inverse of %s: %r is not in %s" % (kind, payload["id"], collection))
        return {TAGS["create-%s" % singular(collection)]: {"index": at, "item": json.loads(json.dumps(members[at]))}}
    if verb == "replace":
        at = index_of(members, payload["id"])
        if at is None:
            raise AssertionError("inverse of %s: %r is not in %s" % (kind, payload["id"], collection))
        return {TAGS["replace-%s-data" % singular(collection)]: {"id": payload["id"], "newData": json.loads(json.dumps(members[at]["data"]))}}
    at = index_of(members, payload["id"])
    if at is None:
        raise AssertionError("inverse of %s: %r is not in %s" % (kind, payload["id"], collection))
    return {TAGS["reorder-%s" % collection]: {"id": payload["id"], "toIndex": at}}


def singular(collection):
    """🏷️ The grammar's singular noun for a collection name."""
    for noun, name in NOUNS.items():
        if name == collection:
            return noun
    raise AssertionError("no singular noun for %r" % collection)


# endregion 🔖️Mutations


# region 🔖️Laws
def observable(kind, before, after):
    """👁️ Every kind in this vocabulary edits a collection, so a forward application must move the
    document. A mutation that quietly did nothing would otherwise agree with an unchanged
    document and report a pass."""
    if before == after:
        raise AssertionError("mutate-%s: the forward mutation left the document untouched, so nothing was proved" % kind)


def restores(kind, restored, original):
    """↩️ The metamorphic inverse law, reported by the first collection and index that diverges."""
    if restored == original:
        return
    for name in COLLECTIONS:
        was, now = original[name], restored[name]
        if was == now:
            continue
        if len(was) != len(now):
            raise AssertionError("inverse-%s: %s came back with %d member(s), not %d" % (kind, name, len(now), len(was)))
        for at, (left, right) in enumerate(zip(was, now)):
            if left != right:
                raise AssertionError("inverse-%s: %s[%d] came back as %s, not %s" % (kind, name, at, json.dumps(right, sort_keys=True), json.dumps(left, sort_keys=True)))
    raise AssertionError("inverse-%s: the document did not come back" % kind)


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, restricted to the three collections the schema declares."""
    for name in COLLECTIONS:
        if produced[name] != committed[name]:
            raise AssertionError("spec-vector-%s: %s is %s, the committed after-snapshot says %s" % (kind, name, json.dumps(produced[name], sort_keys=True), json.dumps(committed[name], sort_keys=True)))


# endregion 🔖️Laws


# region 🔖️Plan
def doc_string(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def declared_mutation(ctx):
    """🦠️ The mutation payload the feature states for this scenario, read from the plan."""
    return json.loads(doc_string(ctx))


def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "local://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def carrier_of(ctx, needle):
    """📖️ Parses the declared carrier document this scenario names."""
    return parse_carrier(ctx.fixture_bytes(uri_in(ctx, needle)).decode("utf-8"))


def projection_of(document):
    """📤️ What parity compares: the three collections the committed JSON Schema declares."""
    return {name: document[name] for name in COLLECTIONS}


# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to the REAL Liège document and answers with the resulting collections."""

    def handler(ctx):
        document, _tail = carrier_of(ctx, "liege")
        mutation = declared_mutation(ctx)
        if kind_of(mutation) != kind:
            raise AssertionError("mutate-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable(kind, document, applied)
        payload = projection_of(applied)
        return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind to the REAL Liège document and then its OWN computed inverse.

    The projection carries BOTH the mutated and the restored document. Projecting the restored one
    alone would make all twelve rows project the same value and the differential would be vacuous.
    """

    def handler(ctx):
        document, _tail = carrier_of(ctx, "liege")
        mutation = declared_mutation(ctx)
        if kind_of(mutation) != kind:
            raise AssertionError("inverse-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable(kind, document, applied)
        undo = inverse_mutation(document, mutation)
        restored = apply_mutation(applied, undo)
        restores(kind, restored, document)
        payload = {"mutated": projection_of(applied), "restored": projection_of(restored)}
        return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))

    return handler


def spec_vector_handler(kind):
    """📐️ Replays the committed handcrafted `(before, mutation, after)` triple for one kind."""

    def handler(ctx):
        before = document_of(json.loads(ctx.fixture_bytes(uri_in(ctx, "⬅️before")).decode("utf-8")))
        mutation = json.loads(ctx.fixture_bytes(uri_in(ctx, "🦠️mutation")).decode("utf-8"))
        after = document_of(json.loads(ctx.fixture_bytes(uri_in(ctx, "➡️after")).decode("utf-8")))
        if kind_of(mutation) != kind:
            raise AssertionError("spec-vector-%s: the committed vector carries a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(before, mutation)
        equals_committed(kind, applied, after)
        undo = inverse_mutation(before, mutation)
        restores(kind, apply_mutation(applied, undo), before)
        payload = projection_of(applied)
        return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))

    return handler


def identity_handler(ctx):
    """🔁️ Reads the artifact's own committed real example and prints it back byte for byte.

    Byte exactness is asserted here, in role: this implementation and the committed file are two
    independent productions of one carrier layout, so anything short of identity is a misreading.
    The projection is the document itself, which is what lets the two languages be compared on what
    they each read out of the same real bytes.
    """
    text = ctx.fixture_bytes(uri_in(ctx, "📚️examples")).decode("utf-8")
    document, tail = parse_carrier(text)
    printed = print_carrier(document, tail)
    if printed != text:
        at = next((offset for offset, (left, right) in enumerate(zip(printed, text)) if left != right), min(len(printed), len(text)))
        raise AssertionError("identity-round-trip: re-encoding the committed example produced %d byte(s) against %d, first difference at %d" % (len(printed.encode("utf-8")), len(text.encode("utf-8")), at))
    reparsed, _ = parse_carrier(printed)
    if reparsed != document:
        raise AssertionError("identity-round-trip: printing and reparsing moved the document")
    payload = projection_of(document)
    return Outcome(payload, raw=printed.encode("utf-8"), diagnostics=[{"severity": "info", "message": "committed example reproduced byte for byte: %d bytes, digest %s" % (len(printed.encode("utf-8")), digest(printed.encode("utf-8")))}])


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
    return built.oracle("identity-round-trip", identity_handler)


# endregion 🔖️Registration
