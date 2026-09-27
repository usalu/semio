"""🧫️ W4: complete layout's mutation fixtures for the `FramePatch.locked`/`.visible` fields and the two new leaves.
- every committed `🔺️diff` carrying a frame patch gains the explicit `"locked": null, "visible": null` (LayoutDiff has no
  skip-serializing, so every field is on the wire),
- `update-grid/📐️sets-an-18-point-baseline` and `set-frame-flags/🔒️locks-frame-1` gain their `🔺️diff` + `🎯️outcome`,
- the stray `📸️snapshot/🔤️splits-the-text-frame-into-two-columns/🔣️.json` inside update-grid's snapshot facet (a byte copy of its
  `➡️after`) is moved aside into `wp-w4/moved-aside/` (never deleted).
The `diff`/`fixture-test` shapes mirror `🔄️rotate-frame/🌀️rotates-the-rect-frame`. Idempotent; `--dry-run` lists the changes.
usage: python3 w4-layout-fixtures.py [--dry-run]"""
import json, os, shutil, sys

ANY = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/"
FIX = ANY + "🧫️fixtures/🧬️mutations/"
ASIDE = "/Users/ueli/Documents/semio/.tmp-ticket/wp-w4/moved-aside/"
DRY = "--dry-run" in sys.argv
changes = []


def dump(value):
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"


def write(path, text):
    if os.path.exists(path) and open(path, encoding="utf-8").read() == text:
        return
    changes.append(path.replace(FIX, ""))
    if not DRY:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf-8").write(text)


def frame_patch_with_flags(patch):
    return {**patch, "locked": patch.get("locked"), "visible": patch.get("visible")}


for leaf in sorted(os.listdir(FIX)):
    for fixture in sorted(os.listdir(FIX + leaf)):
        path = f"{FIX}{leaf}/{fixture}/🔺️diff/🔣️.json"
        if not os.path.exists(path):
            continue
        diff = json.load(open(path, encoding="utf-8"))
        for entry in ((diff.get("pages") or {}).get("patched") or []):
            patched = entry["patch"].get("frame_patched")
            if patched:
                patched["patch"] = frame_patch_with_flags(patched["patch"])
        write(path, dump(diff))

template = json.load(open(FIX + "🔄️rotate-frame/🌀️rotates-the-rect-frame/🔺️diff/🔣️.json", encoding="utf-8"))
empty = {key: None for key in template}

grid = FIX + "📐update-grid/📐️sets-an-18-point-baseline/"
write(grid + "🔺️diff/🔣️.json", dump({**empty, "grid": {"baselineGrid": 18.0, "baselineOffset": 4.0, "snapToBaseline": False}}))
write(grid + "🎯️outcome/🔣️.json", dump({"status": "applied"}))
stray = grid + "📸️snapshot/🔤️splits-the-text-frame-into-two-columns/🔣️.json"
if os.path.exists(stray):
    assert open(stray, "rb").read() == open(grid + "📸️snapshot/➡️after/🔣️.json", "rb").read(), "stray snapshot is no longer a copy of ➡️after"
    changes.append("move aside " + stray.replace(FIX, ""))
    if not DRY:
        target = ASIDE + "📐update-grid-📸️snapshot-🔤️splits-the-text-frame-into-two-columns-🔣️.json"
        os.makedirs(ASIDE, exist_ok=True)
        shutil.move(stray, target)
        os.rmdir(os.path.dirname(stray))

page_patch = template["pages"]["patched"][0]["patch"]
flags = FIX + "🔒set-frame-flags/🔒️locks-frame-1/"
frame_patch = {key: None for key in frame_patch_with_flags(page_patch["frame_patched"]["patch"])}
write(flags + "🔺️diff/🔣️.json", dump({**empty, "pages": {"added": [], "removed": [], "patched": [{"id": "page-1", "patch": {**{key: None for key in page_patch}, "frame_patched": {"frame_id": "frame-rect", "patch": {**frame_patch, "locked": True, "visible": False}}}}], "reordered": None}}))
write(flags + "🎯️outcome/🔣️.json", dump({"status": "applied"}))

print(("would change: " if DRY else "changed: ") + ("\n  ".join([""] + changes) if changes else "nothing"))
