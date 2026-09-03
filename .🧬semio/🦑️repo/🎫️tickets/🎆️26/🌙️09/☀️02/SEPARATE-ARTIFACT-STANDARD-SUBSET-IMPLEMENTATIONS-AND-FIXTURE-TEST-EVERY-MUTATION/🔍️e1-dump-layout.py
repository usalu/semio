import json, os

BASE = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

VECTORS = {
 "rename-layout": ("✏️rename-layout","renames-the-document"),
 "change-print-target": ("🖨️change-print-target","sets-a-cmyk-print-target"),
 "change-data-fields": ("🧾change-data-fields","attaches-a-data-fields-payload"),
 "create-page": ("🌱create-page","appends-page-3"),
 "delete-page": ("🗑️delete-page","removes-page-2"),
 "rename-page": ("🏷️rename-page","renames-page-1"),
 "change-page-width": ("↔️change-page-width","widens-page-1"),
 "change-page-height": ("↕️change-page-height","lengthens-page-1"),
 "update-page-margins": ("📐update-page-margins","sets-asymmetric-margins-on-page-1"),
 "update-page-columns": ("🏛️update-page-columns","splits-page-1-into-three-columns"),
 "reorder-pages": ("🔀reorder-pages","moves-page-1-behind-page-2"),
 "create-story": ("📖create-story","appends-story-3"),
 "delete-story": ("📕delete-story","removes-story-2"),
 "edit-story": ("📝edit-story","rewrites-story-1-body"),
 "create-link": ("🖇️create-link","appends-link-3"),
 "delete-link": ("✂️delete-link","removes-link-2"),
 "change-link-path": ("🔗change-link-path","relinks-link-1-to-a-new-file"),
 "create-frame": ("➕create-frame","inserts-a-rect-frame-at-index-1"),
 "delete-frame": ("➖delete-frame","removes-the-text-frame-and-its-layer-membership"),
 "move-frame": ("🕹️move-frame","moves-the-rect-frame"),
 "resize-frame": ("📏resize-frame","resizes-the-rect-frame"),
 "change-frame-fill": ("🎨change-frame-fill","repaints-the-rect-frame-fill"),
 "change-frame-stroke": ("🖊️change-frame-stroke","adds-a-stroke-to-the-rect-frame"),
 "change-frame-wrap-mode": ("🔤change-frame-wrap-mode","switches-the-text-frame-to-column-wrap"),
 "change-frame-columns": ("🔢change-frame-columns","splits-the-text-frame-into-two-columns"),
}

def flat(obj, prefix=""):
    out = {}
    if isinstance(obj, dict):
        for k, v in obj.items():
            out.update(flat(v, f"{prefix}.{k}" if prefix else k))
    elif isinstance(obj, list):
        for i, v in enumerate(obj):
            out.update(flat(v, f"{prefix}[{i}]"))
    else:
        out[prefix] = obj
    return out

for kind, (dirname, fixture) in VECTORS.items():
    root = f"{BASE}/{dirname}/🧪️tests/{fixture}"
    with open(f"{root}/🦠️mutation/🔣️.json", encoding="utf-8") as f:
        mutation = json.load(f)
    with open(f"{root}/📸️snapshot/⬅️before/🔣️.json", encoding="utf-8") as f:
        before = json.load(f)
    with open(f"{root}/📸️snapshot/➡️after/🔣️.json", encoding="utf-8") as f:
        after = json.load(f)
    fb, fa = flat(before), flat(after)
    keys = sorted(set(fb) | set(fa))
    diffs = []
    for k in keys:
        if fb.get(k, "∅") != fa.get(k, "∅"):
            diffs.append(f"    {k}: {fb.get(k,'∅')!r} -> {fa.get(k,'∅')!r}")
    print(f"### {kind}  [{dirname}/🧪️tests/{fixture}]")
    print("  mutation:", json.dumps(mutation, ensure_ascii=False))
    print(f"  diffs ({len(diffs)}):")
    for d in diffs[:40]:
        print(d)
    if len(diffs) > 40:
        print(f"    ... and {len(diffs)-40} more")
    print()
