#!/usr/bin/env python3
"""Offline stand-in for `committed_diff_applies_to_after` (cargo is unusable). Re-implements the two
artifacts' `MutationDiff::apply` — transcribed from
`🔺️diff/📝️text/🦀️component.rs` in each plugin — and checks committed diff ∘ before == after for
all 45 cases. Catches transcription errors in the hand-written diff JSONs."""
import copy, json, os, pathlib, re, sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
LMUT = ROOT / "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
CMUT = ROOT / "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

# the four cad slots whose Some(None) is indistinguishable from None on the wire
VACATES = {"delete-shape-model": "shapeModel", "delete-building-model": "buildingModel",
           "delete-energy-model": "energyModel", "delete-structure-classic-model": "structureClassicModel"}


def apply_identified(items, d, patch_fn, id_key="id"):
    """apply_identified_delta: remove, then push added at the END, then patch, then reorder."""
    nxt = copy.deepcopy(items)
    for rid in d["removed"]:
        assert any(i[id_key] == rid for i in nxt), f"removed {rid} not present"
        nxt = [i for i in nxt if i[id_key] != rid]
    for item in d["added"]:
        assert not any(i[id_key] == item[id_key] for i in nxt), f"added {item[id_key]} already present"
        nxt.append(copy.deepcopy(item))
    for entry in d["patched"]:
        target = next(i for i in nxt if i[id_key] == entry["id"])
        patch_fn(target, entry["patch"])
    if d["reordered"] is not None:
        order = d["reordered"]
        assert len(order) == len(nxt) and len(set(order)) == len(order)
        nxt = [next(i for i in nxt if i[id_key] == oid) for oid in order]
    return nxt


def apply_frame_field_patch(frame, p):
    b = frame["bounds"]
    for src, dst in (("x", "x"), ("y", "y"), ("width", "w"), ("height", "h")):
        if p[src] is not None:
            b[dst] = p[src]
    if frame["kind"] == "rect":
        if p["fill"] is not None:
            frame["fill"] = p["fill"]
        if p["stroke"] is not None:
            frame["stroke"] = p["stroke"]
    elif frame["kind"] == "text":
        if p["wrap_mode"] is not None:
            frame["wrapMode"] = p["wrap_mode"]
        if p["columns"] is not None:
            frame["columns"] = p["columns"]


def apply_page_patch(page, p):
    if p["name"] is not None: page["name"] = p["name"]
    if p["width"] is not None: page["width"] = p["width"]
    if p["height"] is not None: page["height"] = p["height"]
    for f, k in (("margin_top", "top"), ("margin_right", "right"), ("margin_bottom", "bottom"), ("margin_left", "left")):
        if p[f] is not None: page["margins"][k] = p[f]
    if p["columns_count"] is not None: page["columns"]["count"] = p["columns_count"]
    if p["columns_gutter"] is not None: page["columns"]["gutter"] = p["columns_gutter"]
    if p["frame_added"] is not None:
        a = p["frame_added"]
        at = min(len(page["frames"]) if a["index"] is None else a["index"], len(page["frames"]))
        page["frames"].insert(at, copy.deepcopy(a["frame"]))
        if a["layer_id"] is not None:
            for layer in page["layers"]:
                if layer["id"] == a["layer_id"]:
                    layer["objectIds"].append(a["frame"]["id"])
    if p["frame_removed"] is not None:
        fid = p["frame_removed"]
        page["frames"] = [f for f in page["frames"] if f["id"] != fid]
        for layer in page["layers"]:
            layer["objectIds"] = [i for i in layer["objectIds"] if i != fid]
    if p["frame_patched"] is not None:
        e = p["frame_patched"]
        frame = next(f for f in page["frames"] if f["id"] == e["frame_id"])
        apply_frame_field_patch(frame, e["patch"])


def apply_layout(before, diff):
    n = copy.deepcopy(before)
    assert diff["artifact"] is None and diff["schema"] is None and diff["grid"] is None
    if diff["name"] is not None: n["name"] = diff["name"]
    if diff["pages"] is not None:
        n["pages"] = apply_identified(n["pages"], diff["pages"], apply_page_patch)
    if diff["stories"] is not None:
        n["stories"] = apply_identified(n["stories"], diff["stories"],
                                        lambda s, p: s.__setitem__("content", p["content"]) if p["content"] is not None else None)
    if diff["links"] is not None:
        n["links"] = apply_identified(n["links"], diff["links"],
                                      lambda l, p: l.__setitem__("path", p["path"]) if p["path"] is not None else None)
    if diff["printTarget"] is not None: n["printTarget"] = diff["printTarget"]
    if diff["dataFieldsJson"] is not None: n["dataFieldsJson"] = diff["dataFieldsJson"]
    return n


def apply_cad(before, diff, vacate_field=None):
    n = copy.deepcopy(before)
    assert diff["artifact"] is None and diff["schema"] is None and diff["id"] is None
    for f in ("shapeModel", "buildingModel", "energyModel", "structureClassicModel"):
        if f == vacate_field:               # Some(None): the in-memory vacate arm
            n.pop(f, None)
        elif diff[f] is not None:
            n[f] = copy.deepcopy(diff[f])
    if diff["drawings"] is not None:
        n["drawings"] = copy.deepcopy(diff["drawings"]["values"])
    if diff["referencesByModelDefinitionId"] is not None:
        for k, rows in diff["referencesByModelDefinitionId"].items():
            n["referencesByModelDefinitionId"][k] = copy.deepcopy(rows)
    if diff["nodes"] is not None:
        d = diff["nodes"]
        nxt = [x for x in n["nodes"] if x["id"] not in d["removed"]]
        assert all(any(x["id"] == r for x in n["nodes"]) for r in d["removed"])
        for item in d["added"]:
            assert not any(x["id"] == item["id"] for x in nxt)
            nxt.append(copy.deepcopy(item))
        for e in d["patched"]:
            node = next(x for x in nxt if x["id"] == e["id"])
            if e["patch"]["label"] is not None:
                node["label"] = e["patch"]["label"]
        assert d["reordered"] is None
        n["nodes"] = nxt
    if diff["activeModelDefinitionId"] is not None:
        n["activeModelDefinitionId"] = diff["activeModelDefinitionId"]
    return n


def run(root, applier, label):
    leaves = {re.sub(r"^[^a-z]*", "", e): e for e in sorted(os.listdir(root)) if (root / e).is_dir()}
    ok = bad = 0
    for slug, entry in sorted(leaves.items()):
        tests = root / entry / "🧪️tests"
        if not tests.is_dir():
            continue
        for case in sorted(c for c in os.listdir(tests) if (tests / c).is_dir()):
            d = tests / case
            j = lambda *p: json.loads((d.joinpath(*p)).read_text(encoding="utf-8"))
            before = j("📸️snapshot", "⬅️before", "🔣️component.json")
            after = j("📸️snapshot", "➡️after", "🔣️component.json")
            diff = j("🔺️diff", "🔣️component.json")
            kwargs = {"vacate_field": VACATES[slug]} if (applier is apply_cad and slug in VACATES) else {}
            try:
                produced = applier(before, diff, **kwargs)
            except Exception as e:
                print(f"  ✗ {label}/{slug}/{case}: apply raised {e!r}")
                bad += 1
                continue
            if produced == after:
                ok += 1
            else:
                bad += 1
                print(f"  ✗ {label}/{slug}/{case}: diff∘before != after")
                for k in set(produced) | set(after):
                    if produced.get(k, "<absent>") != after.get(k, "<absent>"):
                        print(f"      field {k}:\n        produced={json.dumps(produced.get(k,'<absent>'))[:400]}\n        after   ={json.dumps(after.get(k,'<absent>'))[:400]}")
    return ok, bad


lo, lb = run(LMUT, apply_layout, "layout")
co, cb = run(CMUT, apply_cad, "cad")
print(f"layout: {lo} diffs carry before→after, {lb} mismatched")
print(f"cad:    {co} diffs carry before→after, {cb} mismatched  "
      f"({len(VACATES)} of them via the in-memory Some(None) arm — JSON null is ambiguous)")
sys.exit(1 if (lb or cb) else 0)
