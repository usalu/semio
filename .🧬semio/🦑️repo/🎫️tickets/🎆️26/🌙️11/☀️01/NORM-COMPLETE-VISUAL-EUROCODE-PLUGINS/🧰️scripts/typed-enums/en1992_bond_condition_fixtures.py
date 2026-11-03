#!/usr/bin/env python3
"""One-shot: en1992 fixtures for BondCondition. Wire tokens `good`/`poor` become `Good`/`Poor`; the apply fixture flips before=Poor, mutation/after/diff=Good so the check-outcome numbers (computed with a non-poor layer) stay valid; the sqlite roundtrip value `arbitrary` becomes `Poor`."""
import pathlib, re
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any")
APPLY = ROOT / "🧪️tests/🧫️fixtures/🧬️mutations/⭕️change-bar-layer-bond-condition/✅apply"
SQLITE = ROOT / "🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json"
n = 0
for p in ROOT.rglob("*.json"):
    if "target" in p.parts or "node_modules" in p.parts or "🧫️fixtures" not in str(p): continue
    t = p.read_text(encoding="utf-8")
    o = t
    if p == SQLITE:
        t = t.replace('"bondCondition": "arbitrary"', '"bondCondition": "Poor"')
    elif p == APPLY / "🔣️.before.json":
        i = t.index('"id": "vert"'); j = t.index('"bondCondition": "good"', i)
        t = t[:j] + '"bondCondition": "Poor"' + t[j + len('"bondCondition": "good"'):]
    t = t.replace('"good-alt"', '"Good"') if p.is_relative_to(APPLY) else t
    t = re.sub(r'("bondCondition":\s*)"good"', r'\1"Good"', t)
    if t != o:
        p.write_text(t, encoding="utf-8"); n += 1
print("files", n)
