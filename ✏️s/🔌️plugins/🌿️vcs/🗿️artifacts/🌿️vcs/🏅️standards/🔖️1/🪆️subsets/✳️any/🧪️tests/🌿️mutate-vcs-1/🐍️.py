"""🐍️ `s.vcs.vcs`'s second, independent implementation of its own six-kind mutation vocabulary.

`s.vcs.vcs` is a semio-NATIVE review-checkpoint document — its two wire forms, `.vcs.dsl.semio` and
`.vcs.pack.semio`, are grammars this repository defines and nobody else reads, so no reference
LIBRARY exists. The reference is therefore a second IMPLEMENTATION, written from this subset's own
committed `../../🧬️schema/📸️snapshot/🔣️.json` and each mutation's `🔣️.schema.json`, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
`change`/`rename`/`add`/`remove` verb entries. It imports nothing from the Rust it judges and
transliterates none of it.

🏷️ `tags` is set-LIKE in meaning but ORDERED on the wire: `add-tag` APPENDS, never re-sorts, and
`remove-tag` detaches by VALUE, leaving the remaining members' relative order untouched — the
committed `add-tag` vector deliberately appends after an existing member (`review` then `urgent`) and
`remove-tag`'s inverse is `add-tag` with the SAME captured value, never a re-append at the end that
would silently reorder the list.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
_ROOT = "asset://🧬️schema/🧬️mutations"
VECTORS = {
    "rename-vcs": (f"{_ROOT}/✏️rename-vcs/🧪️tests/✏️retitles-the-document", "renameVcs"),
    "change-counter": (f"{_ROOT}/🔢change-counter/🧪️tests/🔢️sets-counter-to-seven", "changeCounter"),
    "change-notes": (f"{_ROOT}/📝change-notes/🧪️tests/📝️rewrites-the-notes", "changeNotes"),
    "change-status": (f"{_ROOT}/🚦change-status/🧪️tests/🔎️draft-to-review", "changeStatus"),
    "add-tag": (f"{_ROOT}/🏷️add-tag/🧪️tests/🏷️appends-urgent-tag", "addTag"),
    "remove-tag": (f"{_ROOT}/🗑️remove-tag/🧪️tests/➖️detaches-the-review-tag", "removeTag"),
}


def _read_json(ctx: Context, root: str, leaf: str):
    """🧫️ One declared fixture, parsed."""
    return json.loads(ctx.fixture_bytes(f"{root}/{leaf}/🔣️.json"))
# endregion 🔖️Fixtures


# region 🔖️Wire
def unwrap(wire):
    """📨 The internally-tagged form this subset's committed vectors use, `{"mutation": "<wireTag>", ...}`."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))


WIRE_TAG_TO_KIND = {tag: kind for kind, (_root, tag) in VECTORS.items()}
# endregion 🔖️Wire


# region 🔖️Vocabulary
NULL_DIFF = {"artifact": None, "schema": None, "title": None, "counter": None, "notes": None, "status": None, "tags": None, "selectedCheckpointIds": None, "locale": None}


def apply_rename_vcs(snapshot, payload):
    after = copy.deepcopy(snapshot)
    after["title"] = payload["newTitle"]
    diff = dict(NULL_DIFF, title=payload["newTitle"])
    return after, diff, {"status": "applied"}


def apply_change_counter(snapshot, payload):
    after = copy.deepcopy(snapshot)
    after["counter"] = payload["newCounter"]
    diff = dict(NULL_DIFF, counter=payload["newCounter"])
    return after, diff, {"status": "applied"}


def apply_change_notes(snapshot, payload):
    after = copy.deepcopy(snapshot)
    after["notes"] = payload["newNotes"]
    diff = dict(NULL_DIFF, notes=payload["newNotes"])
    return after, diff, {"status": "applied"}


def apply_change_status(snapshot, payload):
    after = copy.deepcopy(snapshot)
    after["status"] = payload["newStatus"]
    diff = dict(NULL_DIFF, status=payload["newStatus"])
    return after, diff, {"status": "applied"}


def apply_add_tag(snapshot, payload):
    """🏷️ Appends — never re-sorts, never de-duplicates. `atIndex`, when present, is used ONLY to
    reconstruct `remove-tag`'s inverse at its captured BASE position — the real `add-tag` wire never
    carries it, and every genuine forward application always appends."""
    after = copy.deepcopy(snapshot)
    tags = after.get("tags", [])
    if "atIndex" in payload:
        tags.insert(payload["atIndex"], payload["tag"])
    else:
        tags.append(payload["tag"])
    after["tags"] = tags
    diff = dict(NULL_DIFF, tags={"added": [payload["tag"]], "removed": []})
    return after, diff, {"status": "applied"}


def apply_remove_tag(snapshot, payload):
    """🗑️ Detaches by VALUE, preserving the relative order of the remaining members."""
    after = copy.deepcopy(snapshot)
    after["tags"] = [tag for tag in after.get("tags", []) if tag != payload["tag"]]
    diff = dict(NULL_DIFF, tags={"added": [], "removed": [payload["tag"]]})
    return after, diff, {"status": "applied"}


APPLIERS = {
    "rename-vcs": apply_rename_vcs,
    "change-counter": apply_change_counter,
    "change-notes": apply_change_notes,
    "change-status": apply_change_status,
    "add-tag": apply_add_tag,
    "remove-tag": apply_remove_tag,
}


def inverse_mutation(kind, before_snapshot, payload):
    """↩️ Every inverse is computed from BASE, never from the payload or the diff."""
    if kind == "rename-vcs":
        return "renameVcs", {"newTitle": before_snapshot["title"]}
    if kind == "change-counter":
        return "changeCounter", {"newCounter": before_snapshot["counter"]}
    if kind == "change-notes":
        return "changeNotes", {"newNotes": before_snapshot["notes"]}
    if kind == "change-status":
        return "changeStatus", {"newStatus": before_snapshot["status"]}
    if kind == "add-tag":
        return "removeTag", {"tag": payload["tag"]}
    if kind == "remove-tag":
        # ↩️`remove`'s inverse is `insert`/`add` with the CAPTURED item AND its BASE-state index
        # (taxonomy.md's `remove` row) — an inverse that only appended would silently reorder the
        # list, which is exactly the failure this subset's own committed vector is built to expose.
        return "addTag", {"tag": payload["tag"], "atIndex": before_snapshot.get("tags", []).index(payload["tag"])}
    raise AssertionError(f"no inverse rule for kind {kind!r}")
# endregion 🔖️Vocabulary


# region 🔖️Oracle
def _mutate_for(kind):
    def handler(ctx: Context) -> Outcome:
        root, wire_tag = VECTORS[kind]
        before = _read_json(ctx, root, "📸️snapshot/⬅️before")
        actual_tag, payload = unwrap(_read_json(ctx, root, "🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario mutate-{kind}"
        after, diff, outcome = APPLIERS[kind](before, payload)
        expected_after = _read_json(ctx, root, "📸️snapshot/➡️after")
        expected_diff = _read_json(ctx, root, "🔺️diff")
        expected_outcome = _read_json(ctx, root, "🎯️outcome")
        assert after == expected_after, f"mutate-{kind}: {after} != committed after-snapshot {expected_after}"
        assert diff == expected_diff, f"mutate-{kind}: {diff} != committed diff {expected_diff}"
        assert outcome == expected_outcome, f"mutate-{kind}: {outcome} != committed outcome {expected_outcome}"
        payload_bytes = json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=after, raw=payload_bytes)
    return handler


def _inverse_for(kind):
    def handler(ctx: Context) -> Outcome:
        root, wire_tag = VECTORS[kind]
        before = _read_json(ctx, root, "📸️snapshot/⬅️before")
        actual_tag, payload = unwrap(_read_json(ctx, root, "🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario inverse-{kind}"
        after, _diff, _outcome = APPLIERS[kind](before, payload)
        inv_tag, inv_payload = inverse_mutation(kind, before, payload)
        inv_kind = WIRE_TAG_TO_KIND[inv_tag]
        restored, _restored_diff, _restored_outcome = APPLIERS[inv_kind](after, inv_payload)
        assert restored == before, f"inverse-{kind}: {restored} != committed before-snapshot {before}"
        payload_bytes = json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=restored, raw=payload_bytes)
    return handler
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only."""
    built = Adapter("python")
    for kind in VECTORS:
        built = built.oracle(f"mutate-{kind}", _mutate_for(kind)).oracle(f"inverse-{kind}", _inverse_for(kind))
    return built
# endregion 🔖️Registration
