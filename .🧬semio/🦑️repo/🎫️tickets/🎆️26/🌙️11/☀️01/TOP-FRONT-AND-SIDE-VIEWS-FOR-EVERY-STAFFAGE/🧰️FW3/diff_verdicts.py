import json
import sys

directory = sys.argv[1]
whole = json.load(open(f"{directory}/whole.json", encoding="utf8"))
visible = json.load(open(f"{directory}/visible.json", encoding="utf8"))
changed = []
for key in sorted(whole):
    a, b = whole[key], visible.get(key)
    if b is None or a.get("error") or b.get("error"):
        changed.append((key, "error", a, b))
        continue
    if a["suitable"] != b["suitable"] or a["feature"] != b["feature"] or abs(a["minFeature"] - b["minFeature"]) > 1e-3:
        changed.append((key, "verdict" if a["suitable"] != b["suitable"] else "detail", a, b))
print(f"representations {len(whole)}, changed {len(changed)}, unsuitable whole {sum(1 for v in whole.values() if not v.get('suitable', True))}, unsuitable visible {sum(1 for v in visible.values() if not v.get('suitable', True))}")
for key, kind, a, b in changed:
    print(kind, key, "whole:", a.get("suitable"), a.get("minFeature"), a.get("feature"), "| visible:", None if b is None else b.get("suitable"), None if b is None else b.get("minFeature"), None if b is None else b.get("feature"))
