#!/usr/bin/env python3
"""🏔️ An INDEPENDENT second implementation of the `s.gis.gisterrain` document and both of its typed
mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** `GisTerrainSnapshot` persists exactly
two fields: an `f64` vertical `exaggeration` and a raw `importedFeaturesJson` string that is the
`map:in` port's insertion point and that the artifact never interprets. `geo`, `geojson` and `gdal`
were surveyed by an earlier wave and declined: none reads `.dsl.semio`, and none is authoritative
over a scalar exaggeration or over an opaque string the format itself does not parse. What a
reference can adjudicate is the two setters, their independence, and the inverse of each — and that
is what this file implements, from the specification, in another language.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`` — the document is
  `{"exaggeration": double, "importedFeaturesJson": string}`, `additionalProperties: false`, both
  `x-semio-state: artifact`.
* ``…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`` — the two verbs:
  `change-exaggeration <number>` and `change-imported-features`.
* the two committed specification vectors, which give the externally tagged wire form
  (``{"ChangeExaggeration": {"newExaggeration": …}}`` and
  ``{"ChangeImportedFeatures": {"newImportedFeaturesJson": …}}``) and demonstrate that the two
  setters move their fields INDEPENDENTLY.

**What this implementation deliberately does not do, and why.** It does not read `.dsl.semio`. The
`gis.gisterrain` carrier has no prose document, and unlike its `gismap` sibling — whose members are
plainly hex-encoded JSON, a layout that can be derived from the committed bytes and then pinned by
byte-exact re-encoding — this document's only committed example carries an EMPTY
`importedFeaturesJson`, so the encoding of a non-empty string value cannot be read off it at all.
Guessing it and calling the guess a specification would be exactly the fabrication this exercise
exists to prevent. The real-document scenarios therefore read a snapshot fixture derived once from
committed real content (see the feature description for its full provenance), and the carrier's own
laws stay where they can honestly be asserted: in role, on the Rust side, against the committed
example.

**No Rust was read to write this.** `🦀️component.rs` beside this file registers the SUBJECT half
only.
"""

# region 🔖️Imports
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
FIELDS = ("exaggeration", "importedFeaturesJson")
"""🗂️ The two fields `GisTerrainSnapshot` declares — and the cross-language projection."""

KINDS = ("change-exaggeration", "change-imported-features")
"""🏷️ Every kind the catalog declares."""

TAGS = {"change-exaggeration": "ChangeExaggeration", "change-imported-features": "ChangeImportedFeatures"}
"""🔤️ The externally tagged wire name of each kind, as the committed vectors spell it."""

ARGUMENTS = {"change-exaggeration": "newExaggeration", "change-imported-features": "newImportedFeaturesJson"}
"""🔤️ The single argument each verb carries, as the committed vectors spell it."""

# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document):
    """✅️ Holds the document to the committed JSON Schema: a double and a string, both required."""
    if not isinstance(document.get("exaggeration"), (int, float)) or isinstance(document.get("exaggeration"), bool):
        raise AssertionError("exaggeration must be a number, found %r" % document.get("exaggeration"))
    if not isinstance(document.get("importedFeaturesJson"), str):
        raise AssertionError("importedFeaturesJson must be a string, found %r" % document.get("importedFeaturesJson"))


def document_of(payload):
    """📥️ Reads the two declared fields out of a snapshot JSON value."""
    document = {"exaggeration": float(payload["exaggeration"]), "importedFeaturesJson": payload["importedFeaturesJson"]}
    validate(document)
    return document


# endregion 🔖️Document


# region 🔖️Mutations
def kind_of(mutation):
    """🏷️ The kind an externally tagged mutation payload names."""
    if not isinstance(mutation, dict) or len(mutation) != 1:
        raise AssertionError("a mutation is exactly one externally tagged variant, found %r" % mutation)
    tag = next(iter(mutation))
    for kind, name in TAGS.items():
        if name == tag:
            return kind
    raise AssertionError("unknown mutation variant %r" % tag)


def field_of(kind):
    """🗂️ The one field a kind writes — the grammar names one verb per persisted field."""
    return "exaggeration" if kind == "change-exaggeration" else "importedFeaturesJson"


def apply_mutation(document, mutation):
    """🧬️ Applies one typed mutation, returning the resulting document."""
    kind = kind_of(mutation)
    value = mutation[TAGS[kind]][ARGUMENTS[kind]]
    result = dict(document)
    result[field_of(kind)] = float(value) if kind == "change-exaggeration" else value
    validate(result)
    return result


def inverse_mutation(document, mutation):
    """↩️ The mutation that undoes one application: the same setter carrying the previous value."""
    kind = kind_of(mutation)
    return {TAGS[kind]: {ARGUMENTS[kind]: document[field_of(kind)]}}


# endregion 🔖️Mutations


# region 🔖️Laws
def observable(kind, before, after):
    """👁️ A setter that wrote the value it already held would agree with an unchanged document and
    report a pass having done nothing. Both committed vectors and both real-document parameter sets
    move their field, so a forward application must move the document."""
    if before == after:
        raise AssertionError("mutate-%s: the forward mutation left the document untouched, so nothing was proved" % kind)


def independent(kind, before, after):
    """🔀️ The two setters are independent: writing one field must leave the OTHER exactly as it was.
    An implementation that reset a sibling field on every edit passes a field-only check."""
    other = "importedFeaturesJson" if field_of(kind) == "exaggeration" else "exaggeration"
    if before[other] != after[other]:
        raise AssertionError("mutate-%s: writing %s also moved %s, from %r to %r" % (kind, field_of(kind), other, before[other], after[other]))


def restores(kind, restored, original):
    """↩️ The metamorphic inverse law, reported by the field that failed to come back."""
    for name in FIELDS:
        if restored[name] != original[name]:
            raise AssertionError("inverse-%s: %s came back as %r, not %r" % (kind, name, restored[name], original[name]))


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, over the two fields the schema declares."""
    for name in FIELDS:
        if produced[name] != committed[name]:
            raise AssertionError("spec-vector-%s: %s is %r, the committed after-snapshot says %r" % (kind, name, produced[name], committed[name]))


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
    """🎯️ Applies one kind to the real derived Liège terrain document."""

    def handler(ctx):
        document = document_of(json_fixture(ctx, "liege-terrain"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("mutate-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable(kind, document, applied)
        independent(kind, document, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind to the real derived document and then its OWN computed inverse.

    The projection carries BOTH documents; projecting only the restored one would make both rows
    project the same value and the differential would be vacuous.
    """

    def handler(ctx):
        document = document_of(json_fixture(ctx, "liege-terrain"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("inverse-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable(kind, document, applied)
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
        independent(kind, before, applied)
        restores(kind, apply_mutation(applied, inverse_mutation(before, mutation)), before)
        return outcome_of(applied)

    return handler


def identity_handler(ctx):
    """🔁️ Reads the derived real terrain document and answers with the two declared fields.

    This is a JSON-level identity: the two languages must read the same document out of the same
    bytes, and this implementation additionally requires the payload string to be well-formed JSON
    carrying the two real Liège positions, which a codec that dropped or re-serialized it would
    fail. The `.dsl.semio` carrier's own fixpoint and pack-agreement laws are asserted in role on
    the Rust side, against the artifact's committed example — see this file's module docstring for
    why they are not mirrored here.
    """
    payload = json_fixture(ctx, "liege-terrain")
    document = document_of(payload)
    imported = json.loads(document["importedFeaturesJson"])
    identifiers = [feature["id"] for feature in imported["positions"]]
    if identifiers != ["p_institut_de_botanique_ulg_liege", "p_lycee_block_3000"]:
        raise AssertionError("identity-round-trip: the imported descriptor must carry the two real Liège positions, found %r" % identifiers)
    reread = document_of(json.loads(json.dumps(document)))
    if reread != document:
        raise AssertionError("identity-round-trip: serializing and re-reading the document moved it")
    return outcome_of(document)


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
