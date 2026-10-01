#!/usr/bin/env python3
"""🩹️ W3-T-LAYOUT: the layout `PagePatch` carries every field-patched frame of a page (`frames_patched`, a list in page
order) instead of at most one (`frame_patched`), so a frame-selection leaf (`drag-frames`/`rotate-frames`/`scale-frames`)
can move several frames of one page in ONE sparse diff. Rewrites every committed `🔺️diff/🔣️.json` in place: `frame_patched:
null` → `frames_patched: []`, `frame_patched: {…}` → `frames_patched: [{…}]`, key position and formatting kept. Idempotent.

Usage: python3 🧪️w3-t-layout-frames-patched.py [--apply]
"""
import json
import os
import sys

FIXTURES = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations"


def rewrite(node):
    if isinstance(node, dict):
        return {("frames_patched" if key == "frame_patched" else key): ([] if value is None else [rewrite(value)]) if key == "frame_patched" else rewrite(value) for key, value in node.items()}
    if isinstance(node, list):
        return [rewrite(item) for item in node]
    return node


def main():
    apply = "--apply" in sys.argv
    for directory, _dirs, files in os.walk(FIXTURES):
        if not directory.endswith("🔺️diff") or "🔣️.json" not in files:
            continue
        path = os.path.join(directory, "🔣️.json")
        before = open(path, encoding="utf-8").read()
        after = json.dumps(rewrite(json.loads(before)), indent=2, ensure_ascii=False) + ("\n" if before.endswith("\n") else "")
        if json.dumps(json.loads(before), indent=2, ensure_ascii=False) + ("\n" if before.endswith("\n") else "") != before:
            raise SystemExit(f"[w3-t-layout] {path} is not in canonical indent-2 form; edit it by hand")
        if after == before:
            continue
        if apply:
            open(path, "w", encoding="utf-8").write(after)
        print(f"[w3-t-layout] {'wrote' if apply else 'would write'} {os.path.relpath(path, FIXTURES)}")


if __name__ == "__main__":
    main()
