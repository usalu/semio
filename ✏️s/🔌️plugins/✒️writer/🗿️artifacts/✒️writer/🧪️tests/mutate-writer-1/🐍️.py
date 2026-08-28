#!/usr/bin/env python3
"""✒️ An INDEPENDENT second implementation of the `s.writer.writer` document and its four typed
mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `writer` document holds no prose. It
is a HANDLE RECORD: an id, a language id, a URI, and a composed child handle into an
`s.stdio.semio@v1/document`. Nothing outside this repository models an editor document whose body is
a child artifact addressed by content, and none of them reads `.dsl.semio`. That a semio-native
mutation algebra IS adjudicable was settled in this same wave by the fifteen `📕️norm` references and
the nineteen `🧿️semio` ones.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`` — the five members.
* rule 1 of `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`
  — `rename-<artifact>` for the identity field, `change-<field>` per remaining scalar.
* the four committed `(before, mutation, after, outcome)` vectors.

**No Rust was read to write this.** `🦀️component.rs` beside this file registers the SUBJECT half
only.

**WHAT THIS CASE'S EVIDENCE ACTUALLY COVERS, stated rather than implied.** Three of the four kinds are
document-level scalar setters and are fully adjudicated here. The fourth, `edit-text`, is the only
one that reaches the document's actual CONTENT — and its committed vector pins a `mutation.no-op`
against a body this snapshot does not carry. It is refused; see `UNSTATED_REASON`.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "id", "languageId", "uri", "document")
"""🗂️ The five members `WriterSnapshot` declares — and the cross-language projection."""

SCALARS = {"rename-writer": ("id", "newId"), "change-uri": ("uri", "newUri"), "change-language": ("languageId", "newLanguageId")}
"""✏️ The three document-level scalar setters of rule 1."""

UNSTATED = {"edit-text"}
"""🚧️ The one kind this implementation refuses to state — see `UNSTATED_REASON`."""

UNSTATED_REASON = (
    "this implementation refuses this kind rather than guessing it. `edit-text` writes the document's BODY, and this snapshot does not carry the body: "
    "it carries a composed child handle `{childId, target}` into an `s.stdio.semio@v1/document`. The committed vector pins "
    "`{status: applied, messages: [{level: warn, code: mutation.no-op}]}` — the verb decided the new text was IDENTICAL to what the child already held "
    "— and neither the child's content nor the rule that compares them is stated anywhere a second implementation can read. Nor is the other branch: "
    "no committed vector shows what the handle becomes when the text really does change, so the child-addressing function is unstated in the same way "
    "`mutate-program-1` reports over `knowledge`/`benchmarks`, `mutate-note-1` over `edit-block-text` and `mutate-block-3d-1` over `catalog`. Adding "
    "one vector that carries the child body — the `scene` array `mutate-playbook-1` and `mutate-forms-1` already put in their own doc strings — plus "
    "the child-addressing rule, closes it."
)

KINDS = ("rename-writer", "change-uri", "change-language", "edit-text")
"""🏷️ Every kind the catalog declares, in its declared order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}
# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on: five members and a
    well-formed composed child handle."""
    if set(document) != set(MEMBERS):
        raise AssertionError("%s: a writer document must carry exactly %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    if set(document["document"]) != {"childId", "target"}:
        raise AssertionError("%s: the composed body child handle must carry exactly childId and target, found %r" % (where, sorted(document["document"])))
    dialect = document["document"]["target"]["dialect"]
    if dialect["subset"] != "document":
        raise AssertionError("%s: the body child must be an `s.stdio.semio@v1/document`, found subset %r" % (where, dialect["subset"]))
# endregion 🔖️Document


# region 🔖️Verbs
def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind."""
    if kind in UNSTATED:
        raise AssertionError("mutate-%s: %s" % (kind, UNSTATED_REASON))
    member, argument = SCALARS[kind]
    document = copy.deepcopy(document)
    document[member] = payload[argument]
    return document


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary."""
    if kind in UNSTATED:
        raise AssertionError("inverse-%s: %s" % (kind, UNSTATED_REASON))
    member, argument = SCALARS[kind]
    return [(kind, {argument: document[member]})]
# endregion 🔖️Verbs


# region 🔖️Laws
def declared_codes(outcome):
    """🚨️ The `mutation.*` codes a committed `🎯️outcome` vector declares."""
    listed = [message.get("code") for message in outcome.get("messages", []) if message.get("code")]
    if listed:
        return listed
    single = outcome.get("code")
    return [single] if single else []


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member, with no tolerance and no ignored key."""
    for member in MEMBERS:
        if produced[member] != committed[member]:
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced[member], sort_keys=True)[:300], json.dumps(committed[member], sort_keys=True)[:300]))


def touches_one(kind, before, after):
    """🎯️ Each of the three scalar setters writes exactly ONE of the five members, and never the
    composed child handle."""
    moved = [member for member in MEMBERS if before[member] != after[member]]
    if moved != [SCALARS[kind][0]]:
        raise AssertionError("mutate-%s: moved %r, but this kind writes %r and nothing else" % (kind, moved, SCALARS[kind][0]))


def restores(kind, restored, original):
    """↩️ The full inverse law, field for field."""
    for member in MEMBERS:
        if restored[member] != original[member]:
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, member, json.dumps(restored[member], sort_keys=True)[:300], json.dumps(original[member], sort_keys=True)[:300]))
# endregion 🔖️Laws


# region 🔖️Plan
def doc_json(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
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


def payload_of(ctx, kind):
    """🦠️ The committed payload, checked to carry this kind's own internally tagged discriminator."""
    payload = json_fixture(ctx, "🦠️mutation")
    if payload.get("mutation") != TAGS[kind]:
        raise AssertionError("%s: the committed vector carries a %r payload, not %r" % (ctx.scenario["id"], payload.get("mutation"), TAGS[kind]))
    return {key: value for key, value in payload.items() if key != "mutation"}


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))
# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to its committed before-snapshot and asserts, in role, the committed
    after-snapshot, that the vector raised no diagnostic, and that the verb wrote exactly its own
    member."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec.get("kind") != kind:
            raise AssertionError("mutate-%s: the feature's doc string states %r" % (kind, spec.get("kind")))
        before = json_fixture(ctx, "⬅️before")
        after = json_fixture(ctx, "➡️after")
        outcome = json_fixture(ctx, "🎯️outcome")
        validate(before, "mutate-%s" % kind)
        applied = apply_mutation(before, kind, payload_of(ctx, kind))
        validate(applied, "mutate-%s" % kind)
        if declared_codes(outcome):
            raise AssertionError("mutate-%s: the committed outcome declares %r, but a scalar setter over a member this snapshot holds raises nothing" % (kind, declared_codes(outcome)))
        equals_committed(kind, applied, after)
        touches_one(kind, before, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind and then its OWN computed inverse and requires the committed before-snapshot
    back, field for field."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec.get("kind") != kind:
            raise AssertionError("inverse-%s: the feature's doc string states %r" % (kind, spec.get("kind")))
        before = json_fixture(ctx, "⬅️before")
        payload = payload_of(ctx, kind)
        validate(before, "inverse-%s" % kind)
        current = apply_mutation(before, kind, payload)
        for step_kind, step_payload in inverse_mutation(before, kind, payload):
            current = apply_mutation(current, step_kind, step_payload)
        restores(kind, current, before)
        return outcome_of(current)

    return handler


def refuse_carrier(ctx):
    """🚧️ `identity-round-trip` reads this subset's own `.writer.dsl.semio` text carrier, and this
    implementation refuses it by clause rather than by absence. The committed grammar
    `🚪️io/📸️snapshot/📝️text/📖️component.grammar.semio` is the repository-wide PLACEHOLDER — its whole
    body is `payload = OCTET+` and its `header` production declares `"schema" SP "stdio.json"` — while
    the committed artifact's first line is `semio writer.writer.dsl v1` and its body is four
    HEX-ENCODED scalars plus a two-element `[hex,hex]` child-handle pair. Nothing committed says the
    values are hex, nothing says the pair is `(childId, target)`, and nothing says how the second
    element's `<artifactId>!<kind>@<standard>/<subset>` spelling is split."""
    committed = ctx.fixture_bytes(uri_in(ctx, "🗣️example.dsl.semio"))
    raise AssertionError(
        "identity-round-trip: this subset's `.dsl.semio` carrier cannot be read by a second implementation. Its committed grammar is the "
        "repository-wide placeholder `payload = OCTET+` whose header production declares `\"schema\" SP \"stdio.json\"`, contradicted by the artifact's "
        "own first line `semio writer.writer.dsl v1`; the artifact's body is four HEX-ENCODED scalars and a `[hex,hex]` child-handle pair, and no "
        "committed document says the values are hex, that the pair is `(childId, target)`, or how the second element's "
        "`<artifactId>!<kind>@<standard>/<subset>` spelling is split. Read %d bytes of the committed artifact and refused to guess their meaning. A "
        "real grammar for this carrier closes it — the sibling `mutate-note-1` shows one exists for this family, and `📖️playbook`, `📋️forms`, "
        "`🌿️vcs` and `🔌️wires` report the same gap." % len(committed)
    )
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
    return built.oracle("identity-round-trip", refuse_carrier)
# endregion 🔖️Registration
