"""🐍️ The ONE independent Python implementation of the norm mutation vocabulary — the second producer
the fifteen `📕️norm` differential comparisons need, written once and imported by all fifteen.

WHY THIS IS ONE MODULE AND NOT FIFTEEN. Every `s.norm.*` subset is a semio-NATIVE artifact and no
third-party library, in any ecosystem, reads or writes one. That was checked rather than assumed:
PyPI serves no `en1990`, `din18599`, `vdi3805` or `iso16757` distribution at all, and the nearest
real packages (`structuralcodes`, `concreteproperties`, `anastruct`) implement design-code FORMULAE
and speak no interchange format, so none of them could judge a `<Standard>Mutation` vocabulary. The
reference is therefore a second IMPLEMENTATION — and it is a second implementation of ONE thing,
because what the fifteen subsets share is not a family resemblance but a single generative document:

* `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md` — the closed
  verb table (`change` sets one scalar field and its inverse is `change` with the old value; `update`
  sets one cohesive multi-field facet and its inverse is `update` with the old facet; `insert` takes a
  FINAL-state index and `remove` a BASE-state one, and they invert each other; `reorder{from,to}`
  inverts to `reorder{from: min(to, len-1), to: from}`), the naming mechanics — "New-value fields are
  `new_<field>`; address fields are bare" — and the addressing convention, "Inverse always computed
  from `base`" and "Missing target ⇒ `inverse` returns `Vec::new()`".
* the same ticket's `📓️derivation-rules.md` — the shape rules that decide which verbs a document-root
  scalar, an index-keyed ordered collection and a composed child slot each earn.

Those two documents say the same thing to all fifteen subsets. Writing them out fifteen times would
have produced fifteen files whose engines were byte-identical — which is exactly what the fifteen
committed adapters were before this module existed, and it made the reference surface look fifteen
times larger than the evidence it carries. **A shared bug in a copied oracle agrees with itself in
every case that shares it**, and fifteen copies hide that where one import states it. So: one engine,
one bug surface, declared. What each subset still contributes for itself is its committed kind list,
its committed specification vectors, its real committed document and its envelope token — the four
things `Subset` below carries, and the only four things that are genuinely per-standard.

Nothing here is imported from or transliterated out of the Rust it judges. The document field a
`new*` argument names is resolved by NORMALISED SPELLING against the document's own keys — which is
exactly what the naming mechanic states and is why the same code reads both the snake_case and the
camelCase payload spellings the fifteen norm subsets committed — never from a mapping table copied
out of `🧬️mutations/**`.

⚠️ Honest boundary, the CARRIER. `.dsl.semio` has no specification here: every norm subset's committed
`🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio` is the repository-wide placeholder whose whole
body is `payload = OCTET+`, and the framework's own DSL notation module publishes no grammar document
either. `identity_handler` below therefore reads the committed artifact at the CARRIER level — the
envelope preamble and the ordered `key=value` fields as written — a shape derived from those
artifacts' own committed bytes and PINNED by re-emitting each file byte for byte, which a misreading
could not do. It deliberately does not map carrier tokens onto the JSON snapshot's enum spellings:
that mapping is stated nowhere, and inferring it would be reverse-engineering rather than a second
reading. Two subsets — `iso16757` and `vdi3805` — continue a field's record or list value over the
following lines; those lines are read the way a table's rows are, as written and balanced by their
unquoted brackets, so the nesting is carried byte for byte and still never interpreted. That nesting
notation has no published grammar either, which stays a standing finding about the specification.

🔗 Reached by the fifteen adapters through the `oracleHostPackage` this plugin's
`🔣️oracle.json` declares, which puts this directory on the generated host's import path.

🧩️ Layout: this file keeps `Subset`, the scenario laws, the handlers and `build_adapter`; the engine proper lives in
the three children loaded below — `📄️document` (wire spelling, document model, derived views, `Refused`), `🔧️verbs` (the
closed verb table, `apply_mutation`, `inverse_mutation`) and `📝️carrier` (the `.dsl.semio` carrier reader and printer).
"""


from __future__ import annotations

# region 🔖️Imports
import copy
import importlib.util
import json
import os
import sys

from semio_repo_test import Adapter, Outcome


def _load(alias, folder):
    """🧩️ Loads one child module of this adapter from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


_load("semio_norm_vocabulary_document", "📄️document")
_load("semio_norm_vocabulary_verbs", "🔧️verbs")
_load("semio_norm_vocabulary_carrier", "📝️carrier")

from semio_norm_vocabulary_document import unwrap, derived_view, Refused
from semio_norm_vocabulary_verbs import apply_mutation, inverse_mutation
from semio_norm_vocabulary_carrier import parse_dsl, carrier_projection, print_dsl
# endregion 🔖️Imports


# region 🔖️Subset
class Subset:
    """📕️ Everything one norm subset contributes to this engine, and the whole of what distinguishes
    the fifteen reference adapters from one another: the closed kind list its committed catalog
    declares, the committed specification vector each kind publishes, the real committed document the
    carrier round-trip reads, and the envelope token that document's preamble must carry. A vector row
    keyed by its kind is `(directory, fixture)`; a further row `<kind>-<slug>` witnessing a refusal is
    `(kind, directory, fixture)` and is applied, never inverted. `schemas` maps a kind to its committed
    leaf payload schema, whose stated bounds a payload must keep; `documents` are the further committed schemas
    those leaf schemas reference by `$id` (the subset's snapshot schema). No verb, no
    addressing rule and no carrier rule is per-subset — the derivation rules this engine implements
    are the same document for all fifteen, so implementing them fifteen times would be fifteen copies
    of one reading, not fifteen readings."""

    def __init__(self, standard, kinds, vectors, dsl_asset, envelope, vector_root=None, schemas=None, documents=()):
        self.standard = standard
        self.kinds = list(kinds)
        self.vectors = {row: tuple(vector[-2:]) for row, vector in dict(vectors).items()}
        self.rows = {row: vector[0] if len(vector) == 3 else row for row, vector in dict(vectors).items()}
        self.dsl_asset = dsl_asset
        self.envelope = envelope
        self.vector_root = vector_root or VECTOR_ROOT
        self.schemas = dict(schemas or {})
        self.documents = {document["$id"]: document for document in [*documents, *self.schemas.values()] if "$id" in document}


#: 📏 The JSON Schema numeric bounds a leaf payload schema states, with the comparison each one demands.
BOUNDS = {
    "minimum": lambda value, bound: value >= bound,
    "exclusiveMinimum": lambda value, bound: value > bound,
    "maximum": lambda value, bound: value <= bound,
    "exclusiveMaximum": lambda value, bound: value < bound,
}


def broken_bound(schema, arguments, documents=None):
    """🚧 The first numeric bound the kind's committed leaf payload schema states — at any depth, through `$ref`s into
    the subset's committed schemas — that `arguments` breaks, or `None` — a payload outside its own schema is the
    `mutation.invariant` refusal, whatever document it meets."""
    properties = (schema or {}).get("properties") or {}
    for name, value in arguments.items():
        breach = bound_breach(name, value, properties.get(name) or {}, schema or {}, documents or {})
        if breach is not None:
            return breach
    return None


def bound_breach(path, value, node, base, documents):
    """🔎 The first bound `value` breaks at schema position `node` (resolved against `base`), descending objects and arrays."""
    for _ in range(16):
        if not isinstance(node, dict) or "$ref" not in node:
            break
        target, _, pointer = node["$ref"].partition("#")
        base = base if target == "" else documents.get(target)
        if base is None:
            return None
        node = base
        for step in [part for part in pointer.split("/") if part]:
            node = node.get(step, {}) if isinstance(node, dict) else {}
    if isinstance(value, bool) or not isinstance(node, dict):
        return None
    if isinstance(value, (int, float)):
        for keyword, holds in BOUNDS.items():
            bound = node.get(keyword)
            if isinstance(bound, (int, float)) and not holds(value, bound):
                return "%s = %r breaks its leaf schema's %s %r" % (path, value, keyword, bound)
        return None
    if isinstance(value, list):
        return next((breach for index, item in enumerate(value) if (breach := bound_breach("%s[%d]" % (path, index), item, node.get("items") or {}, base, documents)) is not None), None)
    if isinstance(value, dict):
        properties = node.get("properties") or {}
        return next((breach for key, item in value.items() if (breach := bound_breach("%s.%s" % (path, key), item, properties.get(key) or {}, base, documents)) is not None), None)
    return None


#: 📂 The `asset://` prefix every specification vector hangs off; identical in all fifteen subsets
#: because the mutation triad's location is fixed by the repository taxonomy, not by the standard.
VECTOR_ROOT = "shared://🧬️mutations"
# endregion 🔖️Subset


# region 🔖️Laws
def conforms(scenario, got, expected, what):
    """📐 The result must BE the committed snapshot, printed in full on disagreement."""
    if got != expected:
        raise AssertionError("%s: %s\n     got: %s\nexpected: %s" % (scenario, what, json.dumps(got, sort_keys=True), json.dumps(expected, sort_keys=True)))


def observable(kind, mutated, base):
    """👁️ An applied mutation must MOVE the document; a handler that quietly did nothing would
    otherwise report a pass having changed nothing."""
    if mutated == base:
        raise AssertionError("mutate-%s: the committed vector declares this mutation applied, yet the document did not move" % kind)


def untouched(kind, mutated, base):
    """🧊 A refused mutation must leave the document bit-identical — the stricter contract of the two."""
    if mutated != base:
        raise AssertionError("mutate-%s: the committed vector declares this mutation rejected, yet the document moved" % kind)


def restores(kind, restored, base):
    """↩️ The mutation followed by its own inverse must restore the base exactly, position included."""
    if restored != base:
        raise AssertionError("inverse-%s: undoing the mutation did not restore the before-snapshot\n     got: %s\nexpected: %s" % (kind, json.dumps(restored, sort_keys=True), json.dumps(base, sort_keys=True)))
# endregion 🔖️Laws


# region 🔖️Handlers
def read_json(ctx, uri):
    """🔣️ One committed JSON fixture, read through the plan so an undeclared URI is an error."""
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def vector(subset, ctx, row):
    """🧫️ The committed `(before, mutation, after, outcome)` quintet of one vector row."""
    directory, fixture = subset.vectors[row]
    stem = "%s/%s/%s" % (subset.vector_root, directory, fixture)
    return (
        read_json(ctx, "%s/📸️snapshot/⬅️before/🔣️.json" % stem),
        read_json(ctx, "%s/🦠️mutation/🔣️.json" % stem),
        read_json(ctx, "%s/📸️snapshot/➡️after/🔣️.json" % stem),
        read_json(ctx, "%s/🎯️outcome/🔣️.json" % stem),
    )


def mutate_handler(subset, row):
    """🎯️ Applies the row's kind to its committed before-snapshot and asserts in role that this
    implementation reaches the committed after-snapshot under the contract the committed outcome
    declares: `applied` moves the document, `rejected` refuses and leaves it untouched — under the
    committed outcome code where this implementation's refusal names one — and `no-op` leaves it
    untouched whether or not it refuses. The projection is the resulting document, which is what parity
    compares."""

    def handler(ctx):
        kind = subset.rows.get(row, row)
        base, wire, expected, outcome = vector(subset, ctx, row)
        _tag, arguments = unwrap(wire)
        refusal, code = None, None
        try:
            broken = broken_bound(subset.schemas.get(kind), arguments, subset.documents)
            if broken is not None:
                raise Refused("%s: %s" % (kind, broken), "mutation.invariant")
            current = apply_mutation(base, kind, arguments)
        except Refused as reason:
            current, refusal, code = copy.deepcopy(base), str(reason), reason.code
        status = outcome.get("status")
        if status not in ("applied", "rejected", "no-op"):
            raise AssertionError("mutate-%s: unknown committed outcome status %r" % (row, status))
        if status == "applied" and refusal is not None:
            raise AssertionError("mutate-%s: the committed vector declares this mutation applied, yet this implementation refused it: %s" % (row, refusal))
        if status == "rejected" and refusal is None:
            raise AssertionError("mutate-%s: the committed vector declares this mutation rejected, yet this implementation applied it" % row)
        if status == "rejected" and code is not None and outcome.get("code") not in (None, code):
            raise AssertionError("mutate-%s: the committed vector refuses with %s, this implementation with %s: %s" % (row, outcome.get("code"), code, refusal))
        conforms("mutate-" + row, current, expected, "the applied document does not match the committed after-snapshot")
        if status == "applied":
            observable(row, current, base)
        else:
            untouched(row, current, base)
        return Outcome(current, raw=json.dumps(current, sort_keys=True).encode("utf-8"))

    return handler


def inverse_handler(subset, kind):
    """↩️ The metamorphic inverse law in role. The projection carries BOTH the mutated and the restored
    document: projecting only the restored one would make every row project the same value and the
    differential would be vacuous."""

    def handler(ctx):
        base, wire, _expected, outcome = vector(subset, ctx, kind)
        _tag, arguments = unwrap(wire)
        try:
            mutated = apply_mutation(base, kind, arguments)
        except Refused as reason:
            if outcome.get("status") == "applied":
                raise AssertionError("inverse-%s: the forward mutation could not be applied to its own committed before-snapshot: %s" % (kind, reason))
            mutated = copy.deepcopy(base)
        steps = inverse_mutation(subset.kinds, base, kind, arguments)
        if outcome.get("status") == "applied" and not steps:
            raise AssertionError("inverse-%s: this kind changes the document, so its computed inverse must not be empty" % kind)
        restored, view = mutated, derived_view(base)
        for step, payload in steps:
            try:
                restored = apply_mutation(restored, step, payload, view)
            except Refused as reason:
                raise AssertionError("inverse-%s: an inverse step was refused: %s" % (kind, reason))
        restores(kind, restored, base)
        projection = {"mutated": mutated, "restored": restored}
        return Outcome(projection, raw=json.dumps(projection, sort_keys=True).encode("utf-8"))

    return handler


def identity_handler(subset):
    """🔁️ The subset's real committed document through the carrier it is committed in. That carrier
    is this repository's own canonical printer output, so an exact re-emission is the correct answer
    and the wave's must-differ tripwire would be backwards here; what keeps the agreement from being a
    codec agreeing with itself is that these bytes are re-emitted by a SECOND implementation, and both
    sides project the digest and the length of what they emitted."""

    def handler(ctx):
        text = ctx.input_bytes(subset.dsl_asset).decode("utf-8")
        document = parse_dsl(subset.envelope, text)
        reprinted = print_dsl(document)
        if reprinted != text:
            raise AssertionError("identity-round-trip: re-printing the parsed document did not reproduce the committed artifact byte for byte\n     got %d bytes\nexpected %d bytes" % (len(reprinted.encode("utf-8")), len(text.encode("utf-8"))))
        if parse_dsl(subset.envelope, reprinted) != document:
            raise AssertionError("identity-round-trip: printing the document back and reparsing it lost content")
        return Outcome(carrier_projection(reprinted), raw=reprinted.encode("utf-8"))

    return handler
# endregion 🔖️Handlers


# region 🔖️Registration
def build_adapter(subset):
    """🧭️ Registration is by FULL expanded scenario id, so this mirrors the feature's `Examples` tables
    exactly. Oracle role only: registering these handlers as subjects as well would make the reference
    its own subject and manufacture a guaranteed-green self-comparison."""
    built = Adapter("python")
    for kind in subset.kinds:
        built = built.oracle("mutate-" + kind, mutate_handler(subset, kind)).oracle("inverse-" + kind, inverse_handler(subset, kind))
    for row in subset.vectors:
        if row not in subset.kinds:
            built = built.oracle("mutate-" + row, mutate_handler(subset, row))
    return built.oracle("identity-round-trip", identity_handler(subset))
# endregion 🔖️Registration
