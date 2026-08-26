#!/usr/bin/env python3
"""🔍️ One-shot probe of a mutation case: its kinds, its committed vectors' payload signatures, the
snapshot members each kind moves, and the committed outcome each declares. Authoring aid only."""
import json, os, sys, collections

ROOT = "/Users/ueli/Documents/semio"
case = json.load(open(sys.argv[1], encoding="utf-8"))
caseDir = os.path.join(ROOT, case["caseDir"])
art = os.path.dirname(os.path.dirname(caseDir))
subset = None
for base, dirs, files in os.walk(art):
    if base.endswith("🧬️mutations") and "🧪️tests" not in base:
        subset = base
        break
print("artifact:", art.replace(ROOT + "/", ""))
print("mutations dir:", subset.replace(ROOT + "/", "") if subset else None)
rows = []
if subset:
    for slug in sorted(os.listdir(subset)):
        tests = os.path.join(subset, slug, "🧪️tests")
        if not os.path.isdir(tests):
            continue
        for fx in sorted(os.listdir(tests)):
            p = os.path.join(tests, fx)
            def rd(*parts):
                q = os.path.join(p, *parts)
                return json.load(open(q, encoding="utf-8")) if os.path.exists(q) else None
            b = rd("📸️snapshot/⬅️before/🔣️component.json"); a = rd("📸️snapshot/➡️after/🔣️component.json")
            m = rd("🦠️mutation/🔣️component.json"); o = rd("🎯️outcome/🔣️component.json"); df = rd("🔺️diff/🔣️component.json")
            changed = [k for k in (b or {}) if (a or {}).get(k) != b[k]] if b and a else None
            rows.append({"slug": slug, "fixture": fx, "tag": (m or {}).get("mutation"), "payload": sorted(set(m or {}) - {"mutation"}), "changed": changed, "outcome": o, "hasDiff": df is not None})
print("vectors:", len(rows))
for r in rows:
    print("  %-46s tag=%-38s payload=%-40s moved=%s outcome=%s" % (r["slug"][:46], r["tag"], ",".join(r["payload"])[:40], r["changed"], json.dumps(r["outcome"], ensure_ascii=False) if r["outcome"] else None))
if rows and rows[0]["changed"] is not None:
    print("members:", end=" ")
    b = None
