#!/usr/bin/env python3
"""📋️ An INDEPENDENT second implementation of the `s.forms.form` document and its ten typed mutations,
in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `form` document is a HANDLE RECORD
over a step/block survey: the snapshot itself carries only `schema`, `id`, `version`, `title` and two
composed child handles (`structure`, `results`), while the steps and blocks the vocabulary addresses
live in a WORKING SCENE inside the child. No form format — XForms, JSON Schema forms, ODK — models a
survey whose content is a child artifact addressed by content, and none of them reads `.dsl.semio`.
That a semio-native mutation algebra IS adjudicable was settled in this same wave by the fifteen
`📕️norm` references and the nineteen `🧿️semio` ones.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`` — the snapshot's members.
* rules 1, 2 and 3 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`.
* the ten committed `(before, mutation, after, outcome)` vectors AND the `scene` array each scenario
  carries in its own doc string — which is what makes this case adjudicable at all: the scene is the
  child's content, and without it no reader could tell whether `step-outro` exists.

**No Rust was read to write this.** `🦀️component.rs` beside this file registers the SUBJECT half
only.

**WHAT THIS CASE'S EVIDENCE ACTUALLY COVERS, stated plainly rather than implied.** NINE of the ten
committed vectors leave the snapshot BYTE-IDENTICAL, because nine of the ten kinds address records
that live in the child scene and not in this document. What each of those vectors really pins is a
DIAGNOSTIC — a `mutation.no-op` warning, a `mutation.target-missing`, a `mutation.duplicate-id` or a
`mutation.invariant` refusal — and the reference derives that diagnostic from the scene rather than
reading it off the committed outcome, which is the only way the comparison says anything. Only
`change-form-title` moves the document, and what it does is ADD the `title` member the before-snapshot
does not carry. So this case's evidence is one applied mutation and nine diagnostics; no committed
vector in it exercises a create/delete/move/replace that SUCCEEDS.

**A CROSS-CASE DIVERGENCE the reference surfaced.** `s.playbook.playbook` is the same shape with the
same verbs, and the two subsets answer the same situation differently: a duplicate step id is a
REJECTED `mutation.duplicate-id` here (`create-step`) and an APPLIED `mutation.no-op` there
(`add-step`); a block added to a step that does not exist is `mutation.invariant` here
(`create-block`) and `mutation.target-missing` there (`add-block`). Neither divergence is stated
anywhere; both are visible only because one reference was written against both surfaces.

**A SIBLING NOTE, because the count of second implementations must not be overstated.** This file and
`✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🧪️tests/mutate-playbook-1/🐍️.py` are ONE
implementation instantiated twice, differing in the verb names, the diagnostic each situation raises
and the handle members. That the two instantiations disagree on two situations IS the finding above.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
REQUIRED = ("schema", "id", "version", "structure", "results")
"""🗂️ The members every committed form snapshot carries. `title` is ABSENT until `change-form-title`
writes it, which is what its committed vector exercises."""

MEMBERS = REQUIRED + ("title",)

KINDS = ("create-step", "delete-step", "reorder-step", "rename-step", "change-step-description", "create-block", "delete-block", "move-block-to-step", "replace-block", "change-form-title")
"""🏷️ Every kind the catalog declares, in its declared order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}

NO_OP = "mutation.no-op"
TARGET_MISSING = "mutation.target-missing"
DUPLICATE_ID = "mutation.duplicate-id"
INVARIANT = "mutation.invariant"
"""🚨️ The four diagnostic codes this subset's committed vectors raise. Its `📖️playbook` sibling raises
only the first two, and answers two of the same situations with them — see the module docstring."""

REJECTING = (TARGET_MISSING, DUPLICATE_ID, INVARIANT)
"""🚦️ Which of the four refuse the mutation rather than warning about it."""
# endregion 🔖️Vocabulary


# region 🔖️Scene
def step_at(scene, identity):
    """🔎️ The index of a step in the working scene, or `None`."""
    for at, step in enumerate(scene):
        if step["id"] == identity:
            return at
    return None


def block_at(step, identity):
    """🔎️ The index of a block inside one step, or `None`."""
    for at, block in enumerate(step.get("blocks", [])):
        if block["id"] == identity:
            return at
    return None


def numbers_equal(left, right):
    """🔢 Two committed payload values compared as the wire compares them: a scene written `1` and a
    payload written `1.0` are the same number, which is what makes `replace-block`'s committed no-op a
    no-op at all."""
    if isinstance(left, dict) and isinstance(right, dict):
        return set(left) == set(right) and all(numbers_equal(left[key], right[key]) for key in left)
    if isinstance(left, list) and isinstance(right, list):
        return len(left) == len(right) and all(numbers_equal(one, other) for one, other in zip(left, right))
    if isinstance(left, bool) or isinstance(right, bool):
        return left is right
    if isinstance(left, (int, float)) and isinstance(right, (int, float)):
        return float(left) == float(right)
    return left == right
# endregion 🔖️Scene


# region 🔖️Verbs
def diagnose(kind, payload, scene):
    """🚦️ The diagnostic this kind raises against this working scene, derived rather than read off the
    committed outcome. `None` means the verb applies with nothing to say."""
    if kind == "create-step":
        return (DUPLICATE_ID, [payload["step"]["id"]]) if step_at(scene, payload["step"]["id"]) is not None else (None, None)
    if kind == "delete-step":
        return (None, None) if step_at(scene, payload["id"]) is not None else (TARGET_MISSING, [payload["id"]])
    if kind == "reorder-step":
        at = step_at(scene, payload["id"])
        if at is None:
            return (TARGET_MISSING, [payload["id"]])
        return (NO_OP, None) if at == payload["to_index"] else (None, None)
    if kind == "rename-step":
        at = step_at(scene, payload["id"])
        if at is None:
            return (TARGET_MISSING, [payload["id"]])
        return (NO_OP, None) if scene[at].get("title") == payload["new_title"] else (None, None)
    if kind == "change-step-description":
        at = step_at(scene, payload["id"])
        if at is None:
            return (TARGET_MISSING, [payload["id"]])
        return (NO_OP, None) if scene[at].get("description") == payload["new_description"] else (None, None)
    if kind == "create-block":
        at = step_at(scene, payload["step_id"])
        return (None, None) if at is not None else (INVARIANT, [payload["step_id"]])
    if kind == "delete-block":
        at = step_at(scene, payload["step_id"])
        if at is None:
            return (INVARIANT, [payload["step_id"]])
        held = block_at(scene[at], payload["id"])
        return (None, None) if held is not None else (TARGET_MISSING, [payload["step_id"], payload["id"]])
    if kind == "move-block-to-step":
        at = step_at(scene, payload["step_id"])
        if at is None:
            return (INVARIANT, [payload["step_id"]])
        if step_at(scene, payload["to_step_id"]) is None:
            return (INVARIANT, [payload["to_step_id"]])
        held = block_at(scene[at], payload["block_id"])
        if held is None:
            return (TARGET_MISSING, [payload["step_id"], payload["block_id"]])
        unmoved = payload["step_id"] == payload["to_step_id"] and held == payload["index"]
        return (NO_OP, None) if unmoved else (None, None)
    if kind == "replace-block":
        at = step_at(scene, payload["step_id"])
        if at is None:
            return (INVARIANT, [payload["step_id"]])
        held = block_at(scene[at], payload["block"]["id"])
        if held is None:
            return (TARGET_MISSING, [payload["step_id"], payload["block"]["id"]])
        return (NO_OP, None) if numbers_equal(scene[at]["blocks"][held], payload["block"]) else (None, None)
    if kind == "change-form-title":
        return (None, None)
    raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)


def apply_mutation(document, kind, payload, scene):
    """🦠️ Applies one kind to the SNAPSHOT. Eight of the nine kinds address the child scene and cannot
    move a document that holds only handles, so they answer it unchanged; `change-title` is the one
    that writes a member this document really carries."""
    document = copy.deepcopy(document)
    if kind == "change-form-title":
        document["title"] = payload["new_title"]
    return document


def inverse_mutation(document, kind, payload, scene):
    """↩️ The kind's OWN inverse over the snapshot. A verb that could not move the snapshot has no
    inverse to express here — which is exactly why this case's inverse scenarios establish so little,
    and why that is said out loud rather than left to be inferred from a green row."""
    if kind == "change-form-title":
        if "title" not in document:
            raise AssertionError(
                "inverse-change-form-title: this implementation refuses to guess this inverse. The committed vector ADDS the `title` member to a "
                "snapshot that carried none, so undoing it requires REMOVING the member, and nothing committed says whether `change-form-title` "
                "accepts a null argument or what removing a title means. Its `📖️playbook` sibling has no such gap: there `title` is always present "
                "and nullable."
            )
        return [(kind, {"new_title": document["title"]})]
    return []
# endregion 🔖️Verbs


# region 🔖️Laws
def declared(outcome):
    """🚨️ The (status, code, path) a committed `🎯️outcome` vector declares."""
    listed = [message.get("code") for message in outcome.get("messages", []) if message.get("code")]
    code = listed[0] if listed else outcome.get("code")
    return outcome.get("status"), code, outcome.get("path")


def diagnoses_as_committed(kind, produced, outcome):
    """⚖️ The derived diagnostic against the committed one — status, code and path. This is the whole
    of what eight of the nine vectors pin, so it is asserted before anything else."""
    status, code, path = declared(outcome)
    derived_code, derived_path = produced
    derived_status = "rejected" if derived_code in REJECTING else "applied"
    if (derived_status, derived_code) != (status, code):
        raise AssertionError("mutate-%s: this implementation derives %r/%r from the scene, the committed 🎯️outcome vector declares %r/%r" % (kind, derived_status, derived_code, status, code))
    if derived_path is not None and path is not None and derived_path != path:
        raise AssertionError("mutate-%s: this implementation derives the path %r, the committed vector declares %r" % (kind, derived_path, path))


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member, with no tolerance and no ignored key."""
    for member in sorted(set(produced) | set(committed)):
        if produced.get(member, "⌀") != committed.get(member, "⌀"):
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced.get(member), sort_keys=True)[:300], json.dumps(committed.get(member), sort_keys=True)[:300]))


def restores(kind, restored, original):
    """↩️ The full inverse law, member for member."""
    for member in sorted(set(restored) | set(original)):
        if restored.get(member, "⌀") != original.get(member, "⌀"):
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, member, json.dumps(restored.get(member), sort_keys=True)[:300], json.dumps(original.get(member), sort_keys=True)[:300]))


def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on: the five always-present
    members, `title` only beyond them, and two well-formed composed child handles."""
    if not set(REQUIRED) <= set(document):
        raise AssertionError("%s: a form document must carry %r, found %r" % (where, sorted(REQUIRED), sorted(document)))
    if not set(document) <= set(MEMBERS):
        raise AssertionError("%s: a form document may carry only %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    for member in ("structure", "results"):
        if set(document[member]) != {"childId", "target"}:
            raise AssertionError("%s: the composed %s child handle must carry exactly childId and target, found %r" % (where, member, sorted(document[member])))
# endregion 🔖️Laws


# region 🔖️Plan
def doc_json(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own. It carries this
    case's `scene`, the child content without which no diagnostic here is derivable."""
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
    """🎯️ Derives this kind's diagnostic from the working scene, asserts it against the committed
    outcome, and answers the snapshot the verb leaves behind."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec.get("kind") != kind:
            raise AssertionError("mutate-%s: the feature's doc string states %r" % (kind, spec.get("kind")))
        scene = spec.get("scene", [])
        before = json_fixture(ctx, "⬅️before")
        after = json_fixture(ctx, "➡️after")
        outcome = json_fixture(ctx, "🎯️outcome")
        validate(before, "mutate-%s" % kind)
        payload = payload_of(ctx, kind)
        diagnoses_as_committed(kind, diagnose(kind, payload, scene), outcome)
        applied = apply_mutation(before, kind, payload, scene)
        validate(applied, "mutate-%s" % kind)
        equals_committed(kind, applied, after)
        if kind != "change-form-title" and applied != before:
            raise AssertionError("mutate-%s: this kind addresses the child scene, so it cannot move a snapshot that holds only handles, yet the snapshot moved" % kind)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind and then its OWN computed inverse and requires the committed before-snapshot
    back, member for member."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec.get("kind") != kind:
            raise AssertionError("inverse-%s: the feature's doc string states %r" % (kind, spec.get("kind")))
        scene = spec.get("scene", [])
        before = json_fixture(ctx, "⬅️before")
        payload = payload_of(ctx, kind)
        validate(before, "inverse-%s" % kind)
        current = apply_mutation(before, kind, payload, scene)
        for step_kind, step_payload in inverse_mutation(before, kind, payload, scene):
            current = apply_mutation(current, step_kind, step_payload, scene)
        restores(kind, current, before)
        return outcome_of(current)

    return handler


def refuse_carrier(ctx):
    """🚧️ `identity-round-trip` reads this subset's own `.forms.dsl.semio` text carrier, and this
    implementation refuses it by clause rather than by absence. The committed grammar
    `🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` describes a DIFFERENT DOCUMENT: it is the
    generic `family-scene` canvas grammar — `doc-body = schema-line layers-block`,
    `layer = shape-layer | path-layer | text-layer`, `canvas-field = "id" | "x" | "y" | "fill" |
    "stroke" | "opacity"` — and the committed artifact contains no `layers` block, no layer and no
    canvas field. What it does contain is a `steps=[ … ]` list whose members carry nested `blocks=[ … ]`
    lists, `options=[ … ]`, `fields=[ … ]`, `params={ … }` and a bare `condition { }` block, none of
    which the grammar mentions. Four more subsets — `📖️playbook`, `📏️layout`, `🖍️draw` and
    `🖨️raster` — carry the same canvas grammar over four equally unrelated documents, differing from
    this one only in the `grammar`, `extension` and `artifact-mark` lines."""
    committed = ctx.fixture_bytes(uri_in(ctx, "🗣️example.dsl.semio"))
    raise AssertionError(
        "identity-round-trip: this subset's `.dsl.semio` carrier cannot be read by a second implementation. Its committed grammar describes a "
        "DIFFERENT document — the generic `family-scene` canvas grammar, `doc-body = schema-line layers-block` with shape/path/text layers and "
        "`id`/`x`/`y`/`fill`/`stroke`/`opacity` fields — while the committed artifact carries no `layers` block at all, and instead a `steps=[ … ]` "
        "list of nested `blocks=[ … ]`, `options=[ … ]`, `fields=[ … ]`, `params={ … }` and a bare `condition { }` block, none of which the grammar "
        "mentions. Four more subsets — `📖️playbook`, `📏️layout`, `🖍️draw` and `🖨️raster` — carry the same canvas grammar over four equally "
        "unrelated documents, differing only in their `grammar`, `extension` and `artifact-mark` lines. Read %d "
        "bytes of the committed artifact and refused to guess their meaning." % len(committed)
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
