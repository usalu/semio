"""📊️ Progress table of all catalog assets x 4 scales x 3 lists: seeded, authored, check-pass, spliced.

Usage: python3 volumes-status.py [--category humans] [--open] [--json]
Reads the sources under volumes/, the cached check results under 🗑️generated/volumes-tools/check/ (written by volumes-check.ts) and the asset files (spliced = the asset has `volumes`). Fast: no geometry is computed.
Cell per scale = three letters for body, thermal, structure: . not seeded, s seeded but empty, a has solids, u unavailable; followed by + (cached check of that scale passes), ! (fails), ? (never checked or the source changed since).
"""
import glob
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 7))
CATALOG = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog")
SCALES = ("50", "100", "200", "500")
LISTS = ("body", "thermal", "structure")


def category_of(folder):
    return "".join(ch for ch in folder if ch.isascii() and (ch.isalnum() or ch == "-"))



def spliced_state(path):
    with open(path, encoding="utf-8") as f:
        text = f.read()
    return '"volumes"' in text, '\n  "volume"' in text or '\n    "volume"' in text


def main():
    only = sys.argv[sys.argv.index("--category") + 1] if "--category" in sys.argv else None
    rows = []
    for path in sorted(glob.glob(os.path.join(CATALOG, "*", "*", "🔣️.json"))):
        slug = os.path.basename(os.path.dirname(path))
        category = category_of(os.path.basename(os.path.dirname(os.path.dirname(path))))
        if only and category != only:
            continue
        source_path = os.path.join(HERE, "volumes", category, slug + ".json")
        source = json.load(open(source_path, encoding="utf-8"))["volumes"] if os.path.exists(source_path) else None
        cache_path = os.path.join(HERE, "🗑️generated", "volumes-tools", "check", category, slug + ".json")
        cache = json.load(open(cache_path, encoding="utf-8")) if os.path.exists(cache_path) else None
        fresh = cache is not None and source is not None and cache.get("sourceMtimeMs", 0) >= os.path.getmtime(source_path) * 1000 - 5
        has_new, has_old = spliced_state(path)
        cells = []
        for scale in SCALES:
            v = (source or {}).get(scale)
            if v is None:
                letters = "..."
            elif v.get("status") == "unavailable":
                letters = "uuu"
            else:
                letters = "".join("a" if v.get(l) else "s" for l in LISTS)
            mark = "?"
            if fresh and v is not None and v.get("status") == "authored":
                per = cache["perScale"].get(scale)
                mark = "?" if per is None else ("+" if per["ok"] else "!")
            elif v is not None and v.get("status") == "unavailable" and fresh:
                mark = "+"
            cells.append(letters + mark)
        rows.append({"asset": category + "/" + slug, "cells": cells, "check": "pass" if fresh and cache["ok"] else ("fail" if fresh else ("stale" if cache else "-")), "spliced": has_new and not has_old, "seeded": source is not None})
    if "--json" in sys.argv:
        print(json.dumps(rows, ensure_ascii=False, indent=1))
        return
    width = max([len(r["asset"]) for r in rows] + [5])
    print("asset".ljust(width), *[("1:" + s).ljust(5) for s in SCALES], "check ", "spliced")
    for r in rows:
        print(r["asset"].ljust(width), *[c.ljust(5) for c in r["cells"]], r["check"].ljust(6), "yes" if r["spliced"] else "no")
    n = len(rows)
    print()
    print("assets %d | seeded %d | check pass %d | check fail %d | stale %d | spliced %d" % (n, sum(r["seeded"] for r in rows), sum(r["check"] == "pass" for r in rows), sum(r["check"] == "fail" for r in rows), sum(r["check"] == "stale" for r in rows), sum(r["spliced"] for r in rows)))


main()
