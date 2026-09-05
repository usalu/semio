"""🐍️ `s.demonstrator.playground`'s second, independent implementation of its own one-kind mutation
vocabulary.

No third-party library reads or writes `.dsl.semio`/`.pack.semio` — `PlaygroundSnapshot` is a
repository-internal demonstrator document with no published grammar anywhere outside this repository,
so there is no ecosystem to search. The reference is therefore a second IMPLEMENTATION, written from
this subset's own committed `../../🧬️schema/📸️snapshot/🔣️.json` and
`../../🧬️schema/🧬️mutations/✒️change-schema/🧬️.schema.json`, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
`change` verb entry ("set one scalar field to a new value … inverse partner: `change` (old value)").
It imports nothing from the Rust it judges and transliterates none of it.

🔤 WIRE SHAPE. Alone among this repository's mutation vocabularies, `PlaygroundMutation` is encoded
EXTERNALLY tagged with a snake_case payload field — `{"ChangeSchema": {"new_schema": …}}"` — rather
than the internally-tagged, camelCase form every sibling subset uses. `unwrap` below reads exactly
that shape; reusing the internally-tagged reader another subset's second implementation carries would
silently accept the wrong wire form here.

⚠️ Honest boundary. The subset's ONE committed specification vector retags
`playground.playground` → `playground.experiment` — the CHANGED branch. The `mutation.no-op`
degrade-on-unchanged-retag branch is implemented below because the feature file's own prose states it
as a documented rule (a root-scoped equality guard), but no committed vector exercises it, so that
branch is unverified against real evidence — stated here rather than left silent.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
_ROOT = "asset://🧬️schema/🧬️mutations/✒️change-schema/🧪️tests/📅️retags-the-playground-document-schema"
BEFORE_URI = f"{_ROOT}/📸️snapshot/⬅️before/🔣️.json"
MUTATION_URI = f"{_ROOT}/🦠️mutation/🔣️.json"
AFTER_URI = f"{_ROOT}/📸️snapshot/➡️after/🔣️.json"
OUTCOME_URI = f"{_ROOT}/🎯️outcome/🔣️.json"


def _read_json(ctx: Context, uri: str):
    """🧫️ One declared fixture, parsed."""
    return json.loads(ctx.fixture_bytes(uri))
# endregion 🔖️Fixtures


# region 🔖️Wire
def unwrap(wire):
    """📨 The externally-tagged form this subset alone uses: `{"<Variant>": {…}}`."""
    if isinstance(wire, dict) and len(wire) == 1:
        tag = next(iter(wire))
        if isinstance(wire[tag], dict):
            return tag, wire[tag]
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))
# endregion 🔖️Wire


# region 🔖️Vocabulary
def apply_change_schema(snapshot, payload):
    """✒️ `change-schema{new_schema}` — sets the document's one persistent field. An unchanged retag
    is a documented no-op: the root-scoped diff guard downgrades it to one `mutation.no-op` warning
    rather than an error, because there is no addressed target that could be missing."""
    new_schema = payload["new_schema"]
    if snapshot.get("schema") == new_schema:
        after = copy.deepcopy(snapshot)
        diff = {"artifact": None, "schema": None}
        outcome = {"status": "applied", "messages": [{"level": "warn", "code": "mutation.no-op"}]}
        return after, diff, outcome
    after = copy.deepcopy(snapshot)
    after["schema"] = new_schema
    diff = {"artifact": None, "schema": new_schema}
    outcome = {"status": "applied"}
    return after, diff, outcome


def inverse_change_schema(before_snapshot):
    """↩️ `change`'s inverse partner is `change` with the OLD value, read from BASE."""
    return {"ChangeSchema": {"new_schema": before_snapshot.get("schema")}}
# endregion 🔖️Vocabulary


# region 🔖️Oracle
def _mutate(ctx: Context) -> Outcome:
    before = _read_json(ctx, BEFORE_URI)
    tag, payload = unwrap(_read_json(ctx, MUTATION_URI))
    assert tag == "ChangeSchema", f"unexpected wire tag {tag!r} for scenario mutate-change-schema"
    after, diff, outcome = apply_change_schema(before, payload)
    expected_after = _read_json(ctx, AFTER_URI)
    expected_outcome = _read_json(ctx, OUTCOME_URI)
    assert after == expected_after, f"mutate-change-schema: {after} != committed after-document {expected_after}"
    assert outcome == expected_outcome, f"mutate-change-schema: {outcome} != committed outcome {expected_outcome}"
    payload_bytes = json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return Outcome(projection=after, raw=payload_bytes)


def _inverse(ctx: Context) -> Outcome:
    before = _read_json(ctx, BEFORE_URI)
    tag, payload = unwrap(_read_json(ctx, MUTATION_URI))
    assert tag == "ChangeSchema", f"unexpected wire tag {tag!r} for scenario inverse-change-schema"
    after, _diff, _outcome = apply_change_schema(before, payload)
    inverse_mutation = inverse_change_schema(before)
    _inv_tag, inverse_payload = unwrap(inverse_mutation)
    restored, _restored_diff, _restored_outcome = apply_change_schema(after, inverse_payload)
    assert restored == before, f"inverse-change-schema: {restored} != committed before-document {before}"
    payload_bytes = json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return Outcome(projection=restored, raw=payload_bytes)
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only."""
    return Adapter("python").oracle("mutate-change-schema", _mutate).oracle("inverse-change-schema", _inverse)
# endregion 🔖️Registration
