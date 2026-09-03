"""🐍️ `s.flow.flow`'s second, independent implementation of its own ten-kind mutation vocabulary.

`s.flow.flow` is a semio-NATIVE artifact — the `flow.flow` envelope is defined by this repository
alone and no package in any ecosystem reads it. This subset's own recorded survey already argues why
a generic JSON/DOM reader is declined even though the body is plain JSON: it knows nothing of a
widget discriminant, a synapse port pair or the cascade `delete-widget` performs. The reference is
therefore a second IMPLEMENTATION, written from this subset's own committed
`../../🧬️schema/📸️snapshot/🔣️.json` and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
`create`/`delete`/`reorder`/`replace`/`connect`/`disconnect`/`update`/`move`/`duplicate` verb
entries and `📓️derivation-rules.md`'s per-ordered-collection recipe (`widgets` and `synapses` each
get the full `create`/`delete`/`reorder`/`replace`-or-`update` set). It imports nothing from the Rust
it judges and transliterates none of it.

🗂️ SHAPE. `widgets`/`synapses`/`layout` are read from `../../../🧪️tests/mutate-flow-1/🧫️fixtures/🔣️.json`
— this case's own committed, derived-once base graph (its own file records which committed per-kind
leaf fixture each entry came from) — the SAME `local://🔣️.json` this feature's `Given` step declares,
so both implementations start from the identical committed bytes. The ten mutation payloads are
transcribed VERBATIM from this feature's own committed `Examples` `params` column (committed,
checked-in material, not invented here) because the framework does not declare per-kind
`(before, mutation, after, outcome)` asset fixtures for this vocabulary — the scenario's own
assertion shape is a property (`differs from base` / `inverse restores base exactly`), not a
byte-for-byte match against a THIRD committed value, so this file only needs to be internally
consistent forward-and-back, which is what both scenarios below actually check.

⚠️ `duplicate-widget` names no port DIRECTION anywhere in this repository's schemas or taxonomy — it
is a domain-native COMPOSITE with no framework-generic counterpart. This implementation reads it as
wiring the ORIGINAL to the COPY (`from = source_id`, `to = new_id`), the natural "branch an idea"
reading of a duplicate-and-link gesture; that choice is stated here rather than left implicit.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
BASE_URI = "local://🔣️.json"

#: 🧫️ Transcribed verbatim from this feature's `Examples` `params` column.
PARAMS = {
    "create-widget": {"mutation": "createWidget", "index": 0, "widget": {"kind": "inputNote", "id": "note-delta", "text": "Delta"}},
    "delete-widget": {"mutation": "deleteWidget", "id": "note-omega"},
    "reorder-widgets": {"mutation": "reorderWidgets", "id": "note-beta", "toIndex": 9},
    "replace-widget": {"mutation": "replaceWidget", "id": "note-alpha", "widget": {"kind": "inputNote", "id": "note-alpha", "text": "Alpha Prime"}},
    "connect-widgets": {"mutation": "connectWidgets", "index": 2, "id": "synapse-2", "from": "note-gamma", "fromPort": "out", "to": "note-omega", "toPort": "in"},
    "disconnect-widgets": {"mutation": "disconnectWidgets", "id": "synapse-3"},
    "reorder-synapses": {"mutation": "reorderSynapses", "id": "synapse-1", "toIndex": 1},
    "update-synapse-endpoints": {"mutation": "updateSynapseEndpoints", "id": "synapse-1", "from": "note-alpha", "fromPort": "out", "to": "note-gamma", "toPort": "in"},
    "move-widgets": {"mutation": "moveWidgets", "entries": [{"id": "note-alpha", "layout": {"x": 40.0, "y": 80.0}}]},
    "duplicate-widget": {"mutation": "duplicateWidget", "source_id": "note-alpha", "new_id": "note-copy", "synapse_id": "synapse-alpha-to-copy", "from_port": "out", "to_port": "in"},
}


def _read_base(ctx: Context):
    return json.loads(ctx.fixture_bytes(BASE_URI))
# endregion 🔖️Fixtures


# region 🔖️Tree
def widget_index(graph, wid):
    return next((i for i, w in enumerate(graph["widgets"]) if w["id"] == wid), None)


def synapse_index(graph, sid):
    return next((i for i, s in enumerate(graph["synapses"]) if s["id"] == sid), None)
# endregion 🔖️Tree


# region 🔖️Vocabulary
def apply_create_widget(graph, payload):
    after = copy.deepcopy(graph)
    after["widgets"].insert(payload["index"], copy.deepcopy(payload["widget"]))
    return after


def apply_delete_widget(graph, payload):
    """🗑️ Cascades: severs every synapse touching the removed widget and drops its layout entry."""
    after = copy.deepcopy(graph)
    wid = payload["id"]
    after["widgets"] = [w for w in after["widgets"] if w["id"] != wid]
    after["synapses"] = [s for s in after["synapses"] if s["from"] != wid and s["to"] != wid]
    after["layout"].pop(wid, None)
    return after


def apply_reorder_widgets(graph, payload):
    after = copy.deepcopy(graph)
    current = widget_index(after, payload["id"])
    clamped = min(payload["toIndex"], len(after["widgets"]) - 1)
    item = after["widgets"].pop(current)
    after["widgets"].insert(clamped, item)
    return after


def apply_replace_widget(graph, payload):
    after = copy.deepcopy(graph)
    after["widgets"][widget_index(after, payload["id"])] = copy.deepcopy(payload["widget"])
    return after


def apply_connect_widgets(graph, payload):
    after = copy.deepcopy(graph)
    synapse = {"id": payload["id"], "from": payload["from"], "to": payload["to"], "fromPort": payload["fromPort"], "toPort": payload["toPort"]}
    after["synapses"].insert(payload["index"], synapse)
    return after


def apply_disconnect_widgets(graph, payload):
    after = copy.deepcopy(graph)
    after["synapses"] = [s for s in after["synapses"] if s["id"] != payload["id"]]
    return after


def apply_reorder_synapses(graph, payload):
    after = copy.deepcopy(graph)
    current = synapse_index(after, payload["id"])
    clamped = min(payload["toIndex"], len(after["synapses"]) - 1)
    item = after["synapses"].pop(current)
    after["synapses"].insert(clamped, item)
    return after


def apply_update_synapse_endpoints(graph, payload):
    after = copy.deepcopy(graph)
    synapse = after["synapses"][synapse_index(after, payload["id"])]
    synapse["from"], synapse["fromPort"], synapse["to"], synapse["toPort"] = payload["from"], payload["fromPort"], payload["to"], payload["toPort"]
    return after


def apply_move_widgets(graph, payload):
    after = copy.deepcopy(graph)
    for entry in payload["entries"]:
        after["layout"][entry["id"]] = dict(entry["layout"])
    return after


def apply_duplicate_widget(graph, payload):
    """➕ Composite: plans a widget INSERT (a copy of `source_id` under `new_id`) and a synapse
    INSERT (wiring the original to the copy) together — the one variant with no framework-generic
    counterpart at all."""
    after = copy.deepcopy(graph)
    source = after["widgets"][widget_index(after, payload["source_id"])]
    copied = dict(source, id=payload["new_id"])
    after["widgets"].append(copied)
    after["synapses"].append({"id": payload["synapse_id"], "from": payload["source_id"], "to": payload["new_id"], "fromPort": payload["from_port"], "toPort": payload["to_port"]})
    return after


APPLIERS = {
    "create-widget": apply_create_widget,
    "delete-widget": apply_delete_widget,
    "reorder-widgets": apply_reorder_widgets,
    "replace-widget": apply_replace_widget,
    "connect-widgets": apply_connect_widgets,
    "disconnect-widgets": apply_disconnect_widgets,
    "reorder-synapses": apply_reorder_synapses,
    "update-synapse-endpoints": apply_update_synapse_endpoints,
    "move-widgets": apply_move_widgets,
    "duplicate-widget": apply_duplicate_widget,
}


def inverse_apply(kind, base_graph, payload, mutated_graph):
    """↩️ Every inverse is computed from BASE, never from the diff — `taxonomy.md`'s addressing
    convention rule 5 — and applied directly to the MUTATED graph to restore it."""
    after = copy.deepcopy(mutated_graph)
    if kind == "create-widget":
        after["widgets"] = [w for w in after["widgets"] if w["id"] != payload["widget"]["id"]]
        return after
    if kind == "delete-widget":
        wid = payload["id"]
        original_index = widget_index(base_graph, wid)
        after["widgets"].insert(original_index, copy.deepcopy(next(w for w in base_graph["widgets"] if w["id"] == wid)))
        for synapse in base_graph["synapses"]:
            if synapse["from"] == wid or synapse["to"] == wid:
                after["synapses"].insert(synapse_index(base_graph, synapse["id"]), copy.deepcopy(synapse))
        if wid in base_graph["layout"]:
            after["layout"][wid] = dict(base_graph["layout"][wid])
        return after
    if kind == "reorder-widgets":
        original_index = widget_index(base_graph, payload["id"])
        current = widget_index(after, payload["id"])
        clamped = min(original_index, len(after["widgets"]) - 1)
        item = after["widgets"].pop(current)
        after["widgets"].insert(clamped, item)
        return after
    if kind == "replace-widget":
        after["widgets"][widget_index(after, payload["id"])] = copy.deepcopy(next(w for w in base_graph["widgets"] if w["id"] == payload["id"]))
        return after
    if kind == "connect-widgets":
        after["synapses"] = [s for s in after["synapses"] if s["id"] != payload["id"]]
        return after
    if kind == "disconnect-widgets":
        sid = payload["id"]
        original_index = synapse_index(base_graph, sid)
        after["synapses"].insert(original_index, copy.deepcopy(next(s for s in base_graph["synapses"] if s["id"] == sid)))
        return after
    if kind == "reorder-synapses":
        original_index = synapse_index(base_graph, payload["id"])
        current = synapse_index(after, payload["id"])
        clamped = min(original_index, len(after["synapses"]) - 1)
        item = after["synapses"].pop(current)
        after["synapses"].insert(clamped, item)
        return after
    if kind == "update-synapse-endpoints":
        base_synapse = next(s for s in base_graph["synapses"] if s["id"] == payload["id"])
        synapse = after["synapses"][synapse_index(after, payload["id"])]
        synapse["from"], synapse["fromPort"], synapse["to"], synapse["toPort"] = base_synapse["from"], base_synapse["fromPort"], base_synapse["to"], base_synapse["toPort"]
        return after
    if kind == "move-widgets":
        for entry in payload["entries"]:
            wid = entry["id"]
            if wid in base_graph["layout"]:
                after["layout"][wid] = dict(base_graph["layout"][wid])
            else:
                after["layout"].pop(wid, None)
        return after
    if kind == "duplicate-widget":
        after["widgets"] = [w for w in after["widgets"] if w["id"] != payload["new_id"]]
        after["synapses"] = [s for s in after["synapses"] if s["id"] != payload["synapse_id"]]
        return after
    raise AssertionError(f"no inverse rule for kind {kind!r}")
# endregion 🔖️Vocabulary


# region 🔖️Oracle
def _mutate_for(kind):
    def handler(ctx: Context) -> Outcome:
        base = _read_base(ctx)
        after = APPLIERS[kind](base, PARAMS[kind])
        assert after != base, f"mutate-{kind}: the mutation must move the projection, but it produced the base graph unchanged"
        payload_bytes = json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=after, raw=payload_bytes)
    return handler


def _inverse_for(kind):
    def handler(ctx: Context) -> Outcome:
        base = _read_base(ctx)
        mutated = APPLIERS[kind](base, PARAMS[kind])
        restored = inverse_apply(kind, base, PARAMS[kind], mutated)
        assert restored == base, f"inverse-{kind}: {restored} != committed base graph {base}"
        payload_bytes = json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=restored, raw=payload_bytes)
    return handler
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only."""
    built = Adapter("python")
    for kind in PARAMS:
        built = built.oracle(f"mutate-{kind}", _mutate_for(kind)).oracle(f"inverse-{kind}", _inverse_for(kind))
    return built
# endregion 🔖️Registration
