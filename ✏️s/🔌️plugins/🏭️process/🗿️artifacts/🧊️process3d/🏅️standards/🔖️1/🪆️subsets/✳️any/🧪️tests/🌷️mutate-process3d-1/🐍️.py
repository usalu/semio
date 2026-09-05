"""🐍️ `process.process3d`'s second, independent implementation of its own sixteen-kind mutation
vocabulary.

`process.process3d` is a semio-NATIVE artifact and nothing outside this repository reads
`.dsl.semio` — G-code parsers and STEP/BREP kernels were surveyed and DECLINED (kept verbatim below
in this history). The reference is therefore a second IMPLEMENTATION, written from this subset's own
committed `../../🧬️schema/📸️snapshot/🔣️.json` and each mutation's `🧬️.schema.json`, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
`create`/`delete`/`rename`/`change`/`replace`/`move`/`reorder` verb entries and
`📓️derivation-rules.md`'s recipes for an id-keyed ordered timeline (`steps`), an id-keyed unordered
set (`workshop.machines`) and a facet split by field size (`stock`). It imports nothing from the Rust
it judges and transliterates none of it.

⚠️ Honest boundary. `steps` is a content-addressed CHILD HANDLE, and this subset mints a NEW
`childId` for it — and for each entry of `toolSolids` — whenever `stepPayloads` changes, through a
digest algorithm this subset's own schemas do not publish (the feature file names it only as
`process3d_step_timeline_diff`/`process_working_scene_to_snapshot`'s own minting, no formula). The
seven STEP-scoped kinds (`create-step`, `delete-step`, `rename-step`, `change-step-enabled`,
`change-step-origin`, `replace-step-measure`, `reorder-steps`) therefore verify `stepPayloads` itself
— the real, computed content — but do NOT claim to reproduce `steps.childId` or any `toolSolids[]`
entry's `childId`, because no written specification states that hash. The other nine kinds
(`workshop.machines`, `stock`, `resolvedUpTo`) touch no content-addressed field at all — including
`replace-stock-solid`, whose new `childId` is supplied VERBATIM by the mutation's own payload, not
computed — and are verified as a full snapshot equality, exactly like this repository's other
document-scalar/collection vocabularies.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
_ROOT = "asset://🧬️schema/🧬️mutations"
#: 🧫️ (triad dir, fixture name, wire tag, is this kind step-scoped/hash-bearing?)
VECTORS = {
    "create-step": ("🌱create-step", "🐺️accepts-a-rip-cut-step-and-inserts-it", "createStep", True),
    "delete-step": ("🗑️delete-step", "🚫️accepts-a-step-id-and-removes-it", "deleteStep", True),
    "rename-step": ("🏷️rename-step", "🔤️accepts-a-new-label-and-applies-it", "renameStep", True),
    "change-step-enabled": ("🔘change-step-enabled", "🌾️accepts-a-disable-flag-and-applies-it", "changeStepEnabled", True),
    "change-step-origin": ("🧷change-step-origin", "🌳️accepts-a-machine-provenance-and-applies-it", "changeStepOrigin", True),
    "replace-step-measure": ("📐replace-step-measure", "🧭️accepts-a-bore-measure-and-replaces-it", "replaceStepMeasure", True),
    "reorder-steps": ("🔀reorder-steps", "🚪️accepts-a-target-index-and-reorders-them", "reorderSteps", True),
    "create-machine": ("🏭create-machine", "🐯️adds-a-drill-press-to-the-workshop", "createMachine", False),
    "delete-machine": ("❌delete-machine", "🌴️empties-the-workshop-of-the-saw", "deleteMachine", False),
    "rename-machine": ("🔖rename-machine", "⚓️retitles-the-saw", "renameMachine", False),
    "change-machine-icon": ("🎨change-machine-icon", "🟥️swaps-the-saw-icon", "changeMachineIcon", False),
    "replace-machine-capabilities": ("🔁replace-machine-capabilities", "🦉️trades-the-blade-cut-for-a-gated-pocket-cut", "replaceMachineCapabilities", False),
    "move-stock": ("📍move-stock", "🎈️lifts-and-tilts-the-stock", "moveStock", False),
    "change-stock-label": ("🔤change-stock-label", "🔤️relabels-the-oak-beam-as-planed", "changeStockLabel", False),
    "replace-stock-solid": ("🧊replace-stock-solid", "🎫️reissues-the-stock-brep-child-handle", "replaceStockSolid", False),
    "change-cursor": ("⏱️change-cursor", "🟠️pins-the-replay-cursor-to-two-steps", "changeCursor", False),
}


def _read_json(ctx: Context, root: str, fixture: str, leaf: str):
    return json.loads(ctx.fixture_bytes(f"{_ROOT}/{root}/🧪️tests/{fixture}/{leaf}/🔣️.json"))
# endregion 🔖️Fixtures


# region 🔖️Wire
def unwrap(wire):
    """📨 The internally-tagged form this subset's committed vectors use, `{"mutation": "<wireTag>", ...}`."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))
# endregion 🔖️Wire


# region 🔖️Vocabulary — step timeline (verified on `stepPayloads` only, see module docstring)
def step_index(steps, sid):
    return next((i for i, s in enumerate(steps) if s["id"] == sid), None)


def apply_create_step(steps, payload):
    after = copy.deepcopy(steps)
    after.insert(payload["index"], copy.deepcopy(payload["step"]))
    return after


def apply_delete_step(steps, payload):
    return [s for s in steps if s["id"] != payload["id"]]


def apply_rename_step(steps, payload):
    after = copy.deepcopy(steps)
    after[step_index(after, payload["id"])]["label"] = payload["newLabel"]
    return after


def apply_change_step_enabled(steps, payload):
    after = copy.deepcopy(steps)
    after[step_index(after, payload["id"])]["enabled"] = payload["newEnabled"]
    return after


def apply_change_step_origin(steps, payload):
    """🧷 `origin` is OPTIONAL — the committed before-state for this kind's own vector omits the key
    entirely rather than carrying a null, so a `newOrigin` of `None` REMOVES the key (restoring that
    exact absence) instead of writing a literal `null`."""
    after = copy.deepcopy(steps)
    step = after[step_index(after, payload["id"])]
    if payload["newOrigin"] is None:
        step.pop("origin", None)
    else:
        step["origin"] = copy.deepcopy(payload["newOrigin"])
    return after


def apply_replace_step_measure(steps, payload):
    after = copy.deepcopy(steps)
    after[step_index(after, payload["id"])]["measure"] = copy.deepcopy(payload["newMeasure"])
    return after


def apply_reorder_steps(steps, payload):
    after = copy.deepcopy(steps)
    current = step_index(after, payload["id"])
    clamped = min(payload["toIndex"], len(after) - 1)
    item = after.pop(current)
    after.insert(clamped, item)
    return after


STEP_APPLIERS = {
    "create-step": apply_create_step,
    "delete-step": apply_delete_step,
    "rename-step": apply_rename_step,
    "change-step-enabled": apply_change_step_enabled,
    "change-step-origin": apply_change_step_origin,
    "replace-step-measure": apply_replace_step_measure,
    "reorder-steps": apply_reorder_steps,
}


def inverse_step_mutation(kind, before_steps, payload):
    """↩️ Every inverse is computed from BASE, never from the payload or the diff."""
    if kind == "create-step":
        return "deleteStep", {"id": payload["step"]["id"]}
    if kind == "delete-step":
        idx = step_index(before_steps, payload["id"])
        return "createStep", {"index": idx, "step": copy.deepcopy(before_steps[idx])}
    if kind == "rename-step":
        return "renameStep", {"id": payload["id"], "newLabel": before_steps[step_index(before_steps, payload["id"])]["label"]}
    if kind == "change-step-enabled":
        return "changeStepEnabled", {"id": payload["id"], "newEnabled": before_steps[step_index(before_steps, payload["id"])]["enabled"]}
    if kind == "change-step-origin":
        old_origin = before_steps[step_index(before_steps, payload["id"])].get("origin")
        return "changeStepOrigin", {"id": payload["id"], "newOrigin": copy.deepcopy(old_origin) if old_origin is not None else None}
    if kind == "replace-step-measure":
        return "replaceStepMeasure", {"id": payload["id"], "newMeasure": copy.deepcopy(before_steps[step_index(before_steps, payload["id"])]["measure"])}
    if kind == "reorder-steps":
        original = step_index(before_steps, payload["id"])
        return "reorderSteps", {"id": payload["id"], "toIndex": original}
    raise AssertionError(f"no inverse rule for step kind {kind!r}")
# endregion 🔖️Vocabulary — step timeline


# region 🔖️Vocabulary — machine set, stock facet, cursor (verified as a full snapshot)
def machine_index(machines, mid):
    return next((i for i, m in enumerate(machines) if m["id"] == mid), None)


def apply_create_machine(document, payload):
    after = copy.deepcopy(document)
    after["workshop"]["machines"].insert(payload["index"], copy.deepcopy(payload["machine"]))
    return after


def apply_delete_machine(document, payload):
    after = copy.deepcopy(document)
    after["workshop"]["machines"] = [m for m in after["workshop"]["machines"] if m["id"] != payload["id"]]
    return after


def apply_rename_machine(document, payload):
    after = copy.deepcopy(document)
    after["workshop"]["machines"][machine_index(after["workshop"]["machines"], payload["id"])]["label"] = payload["newLabel"]
    return after


def apply_change_machine_icon(document, payload):
    after = copy.deepcopy(document)
    after["workshop"]["machines"][machine_index(after["workshop"]["machines"], payload["id"])]["iconId"] = payload["newIconId"]
    return after


def apply_replace_machine_capabilities(document, payload):
    after = copy.deepcopy(document)
    after["workshop"]["machines"][machine_index(after["workshop"]["machines"], payload["id"])]["capabilities"] = copy.deepcopy(payload["newCapabilities"])
    return after


def apply_move_stock(document, payload):
    after = copy.deepcopy(document)
    after["stockPose"] = copy.deepcopy(payload["newPose"])
    return after


def apply_change_stock_label(document, payload):
    after = copy.deepcopy(document)
    after["stockLabel"] = payload["newLabel"]
    return after


def apply_replace_stock_solid(document, payload):
    """🧊 The new handle is supplied VERBATIM by the payload — not a hash this file computes."""
    after = copy.deepcopy(document)
    after["stockSolid"] = copy.deepcopy(payload["newSolid"])
    return after


def apply_change_cursor(document, payload):
    after = copy.deepcopy(document)
    after["resolvedUpTo"] = payload["newResolvedUpTo"]
    return after


DOC_APPLIERS = {
    "create-machine": apply_create_machine,
    "delete-machine": apply_delete_machine,
    "rename-machine": apply_rename_machine,
    "change-machine-icon": apply_change_machine_icon,
    "replace-machine-capabilities": apply_replace_machine_capabilities,
    "move-stock": apply_move_stock,
    "change-stock-label": apply_change_stock_label,
    "replace-stock-solid": apply_replace_stock_solid,
    "change-cursor": apply_change_cursor,
}


def inverse_doc_mutation(kind, before_document, payload):
    if kind == "create-machine":
        return "deleteMachine", {"id": payload["machine"]["id"]}
    if kind == "delete-machine":
        idx = machine_index(before_document["workshop"]["machines"], payload["id"])
        return "createMachine", {"index": idx, "machine": copy.deepcopy(before_document["workshop"]["machines"][idx])}
    if kind == "rename-machine":
        old = before_document["workshop"]["machines"][machine_index(before_document["workshop"]["machines"], payload["id"])]["label"]
        return "renameMachine", {"id": payload["id"], "newLabel": old}
    if kind == "change-machine-icon":
        old = before_document["workshop"]["machines"][machine_index(before_document["workshop"]["machines"], payload["id"])]["iconId"]
        return "changeMachineIcon", {"id": payload["id"], "newIconId": old}
    if kind == "replace-machine-capabilities":
        old = before_document["workshop"]["machines"][machine_index(before_document["workshop"]["machines"], payload["id"])]["capabilities"]
        return "replaceMachineCapabilities", {"id": payload["id"], "newCapabilities": copy.deepcopy(old)}
    if kind == "move-stock":
        return "moveStock", {"newPose": copy.deepcopy(before_document["stockPose"])}
    if kind == "change-stock-label":
        return "changeStockLabel", {"newLabel": before_document["stockLabel"]}
    if kind == "replace-stock-solid":
        return "replaceStockSolid", {"newSolid": copy.deepcopy(before_document["stockSolid"])}
    if kind == "change-cursor":
        return "changeCursor", {"newResolvedUpTo": before_document["resolvedUpTo"]}
    raise AssertionError(f"no inverse rule for document kind {kind!r}")
# endregion 🔖️Vocabulary — machine set, stock facet, cursor


# region 🔖️Oracle
def _mutate_for(kind):
    root, fixture, wire_tag, step_scoped = VECTORS[kind]

    def handler(ctx: Context) -> Outcome:
        before = _read_json(ctx, root, fixture, "📸️snapshot/⬅️before")
        actual_tag, payload = unwrap(_read_json(ctx, root, fixture, "🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario mutate-{kind}"
        expected_outcome = _read_json(ctx, root, fixture, "🎯️outcome")
        if step_scoped:
            after_steps = STEP_APPLIERS[kind](before["stepPayloads"], payload)
            expected_after = _read_json(ctx, root, fixture, "📸️snapshot/➡️after")
            assert after_steps == expected_after["stepPayloads"], f"mutate-{kind}: {after_steps} != committed stepPayloads {expected_after['stepPayloads']}"
            outcome = {"status": "applied", "messages": []}
            assert outcome == expected_outcome, f"mutate-{kind}: {outcome} != committed outcome {expected_outcome}"
            payload_bytes = json.dumps(after_steps, sort_keys=True, separators=(",", ":")).encode("utf-8")
            return Outcome(projection=after_steps, raw=payload_bytes)
        after = DOC_APPLIERS[kind](before, payload)
        expected_after = _read_json(ctx, root, fixture, "📸️snapshot/➡️after")
        assert after == expected_after, f"mutate-{kind}: document != committed after-snapshot"
        outcome = {"status": "applied", "messages": []}
        assert outcome == expected_outcome, f"mutate-{kind}: {outcome} != committed outcome {expected_outcome}"
        payload_bytes = json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=after, raw=payload_bytes)
    return handler


def _inverse_for(kind):
    root, fixture, wire_tag, step_scoped = VECTORS[kind]

    def handler(ctx: Context) -> Outcome:
        before = _read_json(ctx, root, fixture, "📸️snapshot/⬅️before")
        actual_tag, payload = unwrap(_read_json(ctx, root, fixture, "🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario inverse-{kind}"
        if step_scoped:
            before_steps = before["stepPayloads"]
            after_steps = STEP_APPLIERS[kind](before_steps, payload)
            inv_tag, inv_payload = inverse_step_mutation(kind, before_steps, payload)
            inv_kind = next(k for k, (_r, _f, tag, _s) in VECTORS.items() if tag == inv_tag)
            restored = STEP_APPLIERS[inv_kind](after_steps, inv_payload)
            assert restored == before_steps, f"inverse-{kind}: {restored} != committed before stepPayloads {before_steps}"
            payload_bytes = json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8")
            return Outcome(projection=restored, raw=payload_bytes)
        after = DOC_APPLIERS[kind](before, payload)
        inv_tag, inv_payload = inverse_doc_mutation(kind, before, payload)
        inv_kind = next(k for k, (_r, _f, tag, _s) in VECTORS.items() if tag == inv_tag)
        restored = DOC_APPLIERS[inv_kind](after, inv_payload)
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
