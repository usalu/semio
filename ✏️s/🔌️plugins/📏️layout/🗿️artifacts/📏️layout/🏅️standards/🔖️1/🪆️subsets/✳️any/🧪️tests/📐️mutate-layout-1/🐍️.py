"""🐍️ `s.layout.layout`'s second, independent implementation of its own 25-kind mutation vocabulary.

`s.layout.layout` is a semio-NATIVE page-layout document — its two wire forms, `.dsl.semio` and
`.pack.semio`, are grammars this repository defines and nobody else reads (confirmed, again, by the
carrier-side re-examination recorded in this subset's own `layout-mutation-semantics` no-oracle
decision: none of the five export serializers this repository already links as third-party test
oracles — dxf 0.6, png 0.18, plus svg/dwg/pdf — reads this artifact's own shape; each either coerces
it into a permanently empty document, errors outright, or re-emits the artifact's own internal DSL
text unparsed). The reference is therefore a second IMPLEMENTATION, written from this subset's own
committed `../../🧬️schema/📸️snapshot/🔣️.json` (in practice the full `../../🧬️schema/🔣️.json`
document shape the committed vectors actually use) and each mutation's own committed
`(before, mutation, after)` specification vector, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
`create`/`delete`/`rename`/`change`/`update`/`reorder`/`edit`/`move`/`resize` verb entries and
`📓️derivation-rules.md`'s per-collection shape rules. It imports nothing from the Rust it judges and
transliterates none of it.

🪆️ A layout document is FOUR pools at TWO nesting depths: three root scalars (`name`, `printTarget`,
`dataFieldsJson`), three id-keyed root collections (`pages`, `stories`, `links`), and two collections
that live ONE LEVEL INSIDE a page (`frames`, `layers`) — a frame is addressed by `(page_id, frame_id)`,
never by id alone. `create-frame`'s forward effect touches TWO places at once — `page.frames` (by the
declared `index`) and the named layer's `objectIds` (by APPENDING, never inserting positionally: the
committed vector's `frame-badge` lands at `frames[1]` but `layer.objectIds[2]`, i.e. last, which is the
only order this single vector can pin — stated as an inferred convention, not a proven general rule);
`delete-frame`'s inverse therefore has to recreate both — the frame's own captured `layerId` supplies
the `layer_id` argument `create-frame` needs, it is never re-derived by searching every layer's
`objectIds`.

↩️ Every inverse is computed from BASE (the committed before-document), never from the payload or a
diff: `delete-page`/`delete-story`/`delete-link`/`delete-frame`'s inverses are `create-*` with the
FULL captured member and its BASE-state list position; `reorder-pages`'s inverse is
`reorder-pages{id, to_index: <captured BASE index>}`, matching `taxonomy.md`'s addressing-convention
rule 3 read for an id-keyed reorder rather than an index-keyed one.

🐛 A first standalone run against the committed vectors caught a real bug: `inverse-change-data-fields`
restored `dataFieldsJson` as an explicit `null` rather than OMITTING the key, and the committed
before-document genuinely omits it (`printTarget` is present-with-`null`; `dataFieldsJson` is not a
key at all) — so the restored document disagreed with the committed one on key presence even though
every value looked equal at a glance. Fixed in `apply_change_data_fields` before this reference was
registered as an oracle.

🚧 Scope, stated rather than concealed: this reference reproduces the (before, mutation, after)
document transformation for all 25 kinds — the substantive mutation-semantics claim this subset's own
no-oracle decision names as the debt — and does NOT reproduce the separate, considerably richer
`🔺️diff` sparse-patch algebra (`pages.added/removed/patched[].patch.frame_added/frame_removed/
frame_patched…`) that this subset's committed `🔺️diff` leaves also carry; that diff shape is a
distinct claim this reference does not make and is not asked to by
`nativeSecondImplementationBreaches`' fixture-coverage check, which reads vector count and covered
capabilities, not a diff-algebra reproduction.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
_ROOT = "asset://🧬️schema/🧬️mutations"

# 🗺️ kind -> (declared-fixture root, wire tag). The wire tag is the externally-tagged key each
# committed `🦠️mutation/🔣️.json` uses at its top level — read literally off the committed files,
# never invented.
VECTORS = {
    "rename-layout": (f"{_ROOT}/✏️rename-layout/🧪️tests/📃️renames-the-document", "RenameLayout"),
    "change-print-target": (f"{_ROOT}/🖨️change-print-target/🧪️tests/🖨️sets-a-cmyk-print-target", "ChangePrintTarget"),
    "change-data-fields": (f"{_ROOT}/🧾change-data-fields/🧪️tests/⛵️attaches-a-data-fields-payload", "ChangeDataFields"),
    "create-page": (f"{_ROOT}/🌱create-page/🧪️tests/📃️appends-page-3", "CreatePage"),
    "delete-page": (f"{_ROOT}/🗑️delete-page/🧪️tests/🚫️removes-page-2", "DeletePage"),
    "rename-page": (f"{_ROOT}/🏷️rename-page/🧪️tests/📃️renames-page-1", "RenamePage"),
    "change-page-width": (f"{_ROOT}/↔️change-page-width/🧪️tests/📃️widens-page-1", "ChangePageWidth"),
    "change-page-height": (f"{_ROOT}/↕️change-page-height/🧪️tests/📃️lengthens-page-1", "ChangePageHeight"),
    "update-page-margins": (f"{_ROOT}/📐update-page-margins/🧪️tests/📃️sets-asymmetric-margins-on-page-1", "UpdatePageMargins"),
    "update-page-columns": (f"{_ROOT}/🏛️update-page-columns/🧪️tests/📃️splits-page-1-into-three-columns", "UpdatePageColumns"),
    "reorder-pages": (f"{_ROOT}/🔀reorder-pages/🧪️tests/📃️moves-page-1-behind-page-2", "ReorderPages"),
    "create-story": (f"{_ROOT}/📖create-story/🧪️tests/🟤️appends-story-3", "CreateStory"),
    "delete-story": (f"{_ROOT}/📕delete-story/🧪️tests/🚫️removes-story-2", "DeleteStory"),
    "edit-story": (f"{_ROOT}/✍️edit-story/🧪️tests/🟦️rewrites-story-1-body", "EditStory"),
    "create-link": (f"{_ROOT}/🖇️create-link/🧪️tests/🔗️appends-link-3", "CreateLink"),
    "delete-link": (f"{_ROOT}/✂️delete-link/🧪️tests/🔗️removes-link-2", "DeleteLink"),
    "change-link-path": (f"{_ROOT}/🛤️change-link-path/🧪️tests/🔗️relinks-link-1-to-a-new-file", "ChangeLinkPath"),
    "create-frame": (f"{_ROOT}/➕create-frame/🧪️tests/🚪️inserts-a-rect-frame-at-index-1", "CreateFrame"),
    "delete-frame": (f"{_ROOT}/➖delete-frame/🧪️tests/🚫️removes-the-text-frame-and-its-layer-membership", "DeleteFrame"),
    "move-frame": (f"{_ROOT}/🕹️move-frame/🧪️tests/🔵️moves-the-rect-frame", "MoveFrame"),
    "resize-frame": (f"{_ROOT}/📏resize-frame/🧪️tests/📐️resizes-the-rect-frame", "ResizeFrame"),
    "change-frame-fill": (f"{_ROOT}/🎨change-frame-fill/🧪️tests/🍀️repaints-the-rect-frame-fill", "ChangeFrameFill"),
    "change-frame-stroke": (f"{_ROOT}/🖊️change-frame-stroke/🧪️tests/🦅️adds-a-stroke-to-the-rect-frame", "ChangeFrameStroke"),
    "change-frame-wrap-mode": (f"{_ROOT}/🔤change-frame-wrap-mode/🧪️tests/🔤️switches-the-text-frame-to-column-wrap", "ChangeFrameWrapMode"),
    "change-frame-columns": (f"{_ROOT}/🔢change-frame-columns/🧪️tests/🔤️splits-the-text-frame-into-two-columns", "ChangeFrameColumns"),
}

WIRE_TAG_TO_KIND = {tag: kind for kind, (_root, tag) in VECTORS.items()}


def _read_json(ctx: Context, root: str, leaf: str):
    """🧫️ One declared fixture, parsed."""
    return json.loads(ctx.fixture_bytes(f"{root}/{leaf}/🔣️.json"))


def unwrap(wire):
    """📨 The externally-tagged form every committed vector uses: `{"<WireTag>": {...payload}}`."""
    if isinstance(wire, dict) and len(wire) == 1:
        ((tag, payload),) = wire.items()
        if isinstance(payload, dict):
            return tag, payload
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))
# endregion 🔖️Fixtures


# region 🔖️Addressing
def _find(items, item_id, key="id"):
    """🔎 Locates an id-keyed member; the committed vectors never address a missing one."""
    for index, item in enumerate(items):
        if item.get(key) == item_id:
            return index, item
    raise AssertionError(f"no member with {key}={item_id!r} among {[i.get(key) for i in items]!r}")


def _page(doc, page_id):
    return _find(doc["pages"], page_id)


def _frame(page, frame_id):
    return _find(page["frames"], frame_id)
# endregion 🔖️Addressing


# region 🔖️Vocabulary — forward appliers
def apply_rename_layout(doc, p):
    after = copy.deepcopy(doc)
    after["name"] = p["new_name"]
    return after


def apply_change_print_target(doc, p):
    after = copy.deepcopy(doc)
    after["printTarget"] = p.get("new_print_target")
    return after


def apply_change_data_fields(doc, p):
    """🧾 `dataFieldsJson` is OPTIONAL (genuinely ABSENT when unset, confirmed against the committed
    before-document: `printTarget` is present-with-`null`, `dataFieldsJson` is not a key at all) —
    unlike `change-print-target`'s scalar, so a `None` inverse must POP the key rather than set it to
    `null`; a first standalone run against the committed vector caught exactly this (an inverse that
    wrote `null` instead of omitting the key failed the before-document comparison)."""
    after = copy.deepcopy(doc)
    value = p.get("new_json")
    if value is None:
        after.pop("dataFieldsJson", None)
    else:
        after["dataFieldsJson"] = value
    return after


def apply_create_page(doc, p):
    after = copy.deepcopy(doc)
    after["pages"].insert(p["index"], copy.deepcopy(p["page"]))
    return after


def apply_delete_page(doc, p):
    after = copy.deepcopy(doc)
    idx, _ = _page(after, p["id"])
    after["pages"].pop(idx)
    return after


def apply_rename_page(doc, p):
    after = copy.deepcopy(doc)
    _, page = _page(after, p["id"])
    page["name"] = p["new_name"]
    return after


def apply_change_page_width(doc, p):
    after = copy.deepcopy(doc)
    _, page = _page(after, p["id"])
    page["width"] = p["new_width"]
    return after


def apply_change_page_height(doc, p):
    after = copy.deepcopy(doc)
    _, page = _page(after, p["id"])
    page["height"] = p["new_height"]
    return after


def apply_update_page_margins(doc, p):
    after = copy.deepcopy(doc)
    _, page = _page(after, p["id"])
    page["margins"] = {"top": p["top"], "right": p["right"], "bottom": p["bottom"], "left": p["left"]}
    return after


def apply_update_page_columns(doc, p):
    after = copy.deepcopy(doc)
    _, page = _page(after, p["id"])
    page["columns"] = {"count": p["count"], "gutter": p["gutter"]}
    return after


def apply_reorder_pages(doc, p):
    """🔀 Id-keyed reorder: pop the whole subtree at its current index, reinsert at `to_index`,
    clamped to the post-removal length — moving the page moves everything nested under it (frames,
    layers) as one unit, never rebuilt member-by-member."""
    after = copy.deepcopy(doc)
    idx, page = _page(after, p["id"])
    after["pages"].pop(idx)
    target = min(p["to_index"], len(after["pages"]))
    after["pages"].insert(target, page)
    return after


def apply_create_story(doc, p):
    after = copy.deepcopy(doc)
    after["stories"].insert(p["index"], copy.deepcopy(p["story"]))
    return after


def apply_delete_story(doc, p):
    after = copy.deepcopy(doc)
    idx, _ = _find(after["stories"], p["id"])
    after["stories"].pop(idx)
    return after


def apply_edit_story(doc, p):
    after = copy.deepcopy(doc)
    _, story = _find(after["stories"], p["id"])
    story["content"] = p["new_content"]
    return after


def apply_create_link(doc, p):
    after = copy.deepcopy(doc)
    after["links"].insert(p["index"], copy.deepcopy(p["link"]))
    return after


def apply_delete_link(doc, p):
    after = copy.deepcopy(doc)
    idx, _ = _find(after["links"], p["id"])
    after["links"].pop(idx)
    return after


def apply_change_link_path(doc, p):
    after = copy.deepcopy(doc)
    _, link = _find(after["links"], p["id"])
    link["path"] = p["new_path"]
    return after


def apply_create_frame(doc, p):
    """➕ Inserts into `page.frames` at the declared index AND appends the new frame's id to the
    named layer's `objectIds` — the two places one frame lives, touched together."""
    after = copy.deepcopy(doc)
    _, page = _page(after, p["page_id"])
    page["frames"].insert(p["index"], copy.deepcopy(p["frame"]))
    _, layer = _find(page["layers"], p["layer_id"])
    layer["objectIds"].append(p["frame"]["id"])
    return after


def apply_delete_frame(doc, p):
    """➖ Removes the frame from `page.frames` AND detaches its id from whichever layer's
    `objectIds` names it — the cascade `➖delete-frame`'s own committed vector exists to pin."""
    after = copy.deepcopy(doc)
    _, page = _page(after, p["page_id"])
    idx, _frame_obj = _frame(page, p["frame_id"])
    page["frames"].pop(idx)
    for layer in page["layers"]:
        if p["frame_id"] in layer["objectIds"]:
            layer["objectIds"].remove(p["frame_id"])
    return after


def apply_move_frame(doc, p):
    after = copy.deepcopy(doc)
    _, page = _page(after, p["page_id"])
    _, frame = _frame(page, p["frame_id"])
    frame["bounds"]["x"] = p["new_x"]
    frame["bounds"]["y"] = p["new_y"]
    return after


def apply_resize_frame(doc, p):
    after = copy.deepcopy(doc)
    _, page = _page(after, p["page_id"])
    _, frame = _frame(page, p["frame_id"])
    frame["bounds"]["w"] = p["new_width"]
    frame["bounds"]["h"] = p["new_height"]
    return after


def apply_change_frame_fill(doc, p):
    after = copy.deepcopy(doc)
    _, page = _page(after, p["page_id"])
    _, frame = _frame(page, p["frame_id"])
    frame["fill"] = list(p["new_fill"]) if p.get("new_fill") is not None else None
    return after


def apply_change_frame_stroke(doc, p):
    after = copy.deepcopy(doc)
    _, page = _page(after, p["page_id"])
    _, frame = _frame(page, p["frame_id"])
    frame["stroke"] = list(p["new_stroke"]) if p.get("new_stroke") is not None else None
    return after


def apply_change_frame_wrap_mode(doc, p):
    after = copy.deepcopy(doc)
    _, page = _page(after, p["page_id"])
    _, frame = _frame(page, p["frame_id"])
    frame["wrapMode"] = p["new_wrap_mode"]
    return after


def apply_change_frame_columns(doc, p):
    after = copy.deepcopy(doc)
    _, page = _page(after, p["page_id"])
    _, frame = _frame(page, p["frame_id"])
    frame["columns"] = p["new_columns"]
    return after


APPLIERS = {
    "rename-layout": apply_rename_layout,
    "change-print-target": apply_change_print_target,
    "change-data-fields": apply_change_data_fields,
    "create-page": apply_create_page,
    "delete-page": apply_delete_page,
    "rename-page": apply_rename_page,
    "change-page-width": apply_change_page_width,
    "change-page-height": apply_change_page_height,
    "update-page-margins": apply_update_page_margins,
    "update-page-columns": apply_update_page_columns,
    "reorder-pages": apply_reorder_pages,
    "create-story": apply_create_story,
    "delete-story": apply_delete_story,
    "edit-story": apply_edit_story,
    "create-link": apply_create_link,
    "delete-link": apply_delete_link,
    "change-link-path": apply_change_link_path,
    "create-frame": apply_create_frame,
    "delete-frame": apply_delete_frame,
    "move-frame": apply_move_frame,
    "resize-frame": apply_resize_frame,
    "change-frame-fill": apply_change_frame_fill,
    "change-frame-stroke": apply_change_frame_stroke,
    "change-frame-wrap-mode": apply_change_frame_wrap_mode,
    "change-frame-columns": apply_change_frame_columns,
}
# endregion 🔖️Vocabulary — forward appliers


# region 🔖️Vocabulary — inverse rule
def inverse_mutation(kind, before, payload):
    """↩️ Every inverse is computed from BASE — the committed before-document — never from the
    payload or a diff, per `taxonomy.md` rule 5. Returns `(wire_tag, inverse_payload)`."""
    if kind == "rename-layout":
        return "RenameLayout", {"new_name": before["name"]}
    if kind == "change-print-target":
        return "ChangePrintTarget", {"new_print_target": before.get("printTarget")}
    if kind == "change-data-fields":
        return "ChangeDataFields", {"new_json": before.get("dataFieldsJson")}
    if kind == "create-page":
        return "DeletePage", {"id": payload["page"]["id"]}
    if kind == "delete-page":
        idx, page = _page(before, payload["id"])
        return "CreatePage", {"page": page, "index": idx}
    if kind == "rename-page":
        _, page = _page(before, payload["id"])
        return "RenamePage", {"id": payload["id"], "new_name": page["name"]}
    if kind == "change-page-width":
        _, page = _page(before, payload["id"])
        return "ChangePageWidth", {"id": payload["id"], "new_width": page["width"]}
    if kind == "change-page-height":
        _, page = _page(before, payload["id"])
        return "ChangePageHeight", {"id": payload["id"], "new_height": page["height"]}
    if kind == "update-page-margins":
        _, page = _page(before, payload["id"])
        m = page["margins"]
        return "UpdatePageMargins", {"id": payload["id"], "top": m["top"], "right": m["right"], "bottom": m["bottom"], "left": m["left"]}
    if kind == "update-page-columns":
        _, page = _page(before, payload["id"])
        c = page["columns"]
        return "UpdatePageColumns", {"id": payload["id"], "count": c["count"], "gutter": c["gutter"]}
    if kind == "reorder-pages":
        idx, _page_obj = _page(before, payload["id"])
        return "ReorderPages", {"id": payload["id"], "to_index": idx}
    if kind == "create-story":
        return "DeleteStory", {"id": payload["story"]["id"]}
    if kind == "delete-story":
        idx, story = _find(before["stories"], payload["id"])
        return "CreateStory", {"story": story, "index": idx}
    if kind == "edit-story":
        _, story = _find(before["stories"], payload["id"])
        return "EditStory", {"id": payload["id"], "new_content": story["content"]}
    if kind == "create-link":
        return "DeleteLink", {"id": payload["link"]["id"]}
    if kind == "delete-link":
        idx, link = _find(before["links"], payload["id"])
        return "CreateLink", {"link": link, "index": idx}
    if kind == "change-link-path":
        _, link = _find(before["links"], payload["id"])
        return "ChangeLinkPath", {"id": payload["id"], "new_path": link["path"]}
    if kind == "create-frame":
        return "DeleteFrame", {"page_id": payload["page_id"], "frame_id": payload["frame"]["id"]}
    if kind == "delete-frame":
        _, page = _page(before, payload["page_id"])
        idx, frame = _frame(page, payload["frame_id"])
        return "CreateFrame", {"page_id": payload["page_id"], "frame": frame, "index": idx, "layer_id": frame["layerId"]}
    if kind == "move-frame":
        _, page = _page(before, payload["page_id"])
        _, frame = _frame(page, payload["frame_id"])
        return "MoveFrame", {"page_id": payload["page_id"], "frame_id": payload["frame_id"], "new_x": frame["bounds"]["x"], "new_y": frame["bounds"]["y"]}
    if kind == "resize-frame":
        _, page = _page(before, payload["page_id"])
        _, frame = _frame(page, payload["frame_id"])
        return "ResizeFrame", {"page_id": payload["page_id"], "frame_id": payload["frame_id"], "new_width": frame["bounds"]["w"], "new_height": frame["bounds"]["h"]}
    if kind == "change-frame-fill":
        _, page = _page(before, payload["page_id"])
        _, frame = _frame(page, payload["frame_id"])
        return "ChangeFrameFill", {"page_id": payload["page_id"], "frame_id": payload["frame_id"], "new_fill": frame.get("fill")}
    if kind == "change-frame-stroke":
        _, page = _page(before, payload["page_id"])
        _, frame = _frame(page, payload["frame_id"])
        return "ChangeFrameStroke", {"page_id": payload["page_id"], "frame_id": payload["frame_id"], "new_stroke": frame.get("stroke")}
    if kind == "change-frame-wrap-mode":
        _, page = _page(before, payload["page_id"])
        _, frame = _frame(page, payload["frame_id"])
        return "ChangeFrameWrapMode", {"page_id": payload["page_id"], "frame_id": payload["frame_id"], "new_wrap_mode": frame["wrapMode"]}
    if kind == "change-frame-columns":
        _, page = _page(before, payload["page_id"])
        _, frame = _frame(page, payload["frame_id"])
        return "ChangeFrameColumns", {"page_id": payload["page_id"], "frame_id": payload["frame_id"], "new_columns": frame["columns"]}
    raise AssertionError(f"no inverse rule for kind {kind!r}")
# endregion 🔖️Vocabulary — inverse rule


# region 🔖️Oracle
def _mutate_for(kind):
    def handler(ctx: Context) -> Outcome:
        root, wire_tag = VECTORS[kind]
        before = _read_json(ctx, root, "📸️snapshot/⬅️before")
        actual_tag, payload = unwrap(_read_json(ctx, root, "🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario mutate-{kind}"
        after = APPLIERS[kind](before, payload)
        expected_after = _read_json(ctx, root, "📸️snapshot/➡️after")
        assert after == expected_after, f"mutate-{kind}: {after} != committed after-document {expected_after}"
        raw = json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=after, raw=raw)

    return handler


def _inverse_for(kind):
    def handler(ctx: Context) -> Outcome:
        root, wire_tag = VECTORS[kind]
        before = _read_json(ctx, root, "📸️snapshot/⬅️before")
        actual_tag, payload = unwrap(_read_json(ctx, root, "🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario inverse-{kind}"
        after = APPLIERS[kind](before, payload)
        assert after != before, f"inverse-{kind}: the forward mutation left the document untouched, so restoring it proves nothing"
        inv_tag, inv_payload = inverse_mutation(kind, before, payload)
        inv_kind = WIRE_TAG_TO_KIND[inv_tag]
        restored = APPLIERS[inv_kind](after, inv_payload)
        assert restored == before, f"inverse-{kind}: {restored} != committed before-document {before}"
        raw = json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=restored, raw=raw)

    return handler
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only — `identity-round-trip` stays subject-only, exactly as the Rust
    adapter already treats it, because the real committed artifact is `.dsl.semio` text only and
    turning it into a document needs this subset's own codec, which this reference does not carry."""
    built = Adapter("python")
    for kind in VECTORS:
        built = built.oracle(f"mutate-{kind}", _mutate_for(kind)).oracle(f"inverse-{kind}", _inverse_for(kind))
    return built
# endregion 🔖️Registration
