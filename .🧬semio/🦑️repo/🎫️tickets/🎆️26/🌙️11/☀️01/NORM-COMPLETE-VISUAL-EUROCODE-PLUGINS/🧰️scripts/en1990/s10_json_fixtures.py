from spec import *
import json, os
EMPTY = lambda: {"removed": [], "inserted": [], "moved": [], "modified": []}
def fix(node):
    changed = False
    if isinstance(node, dict):
        if {"id", "category", "qk"} <= node.keys() and "group" not in node:
            node["group"] = ""; changed = True
        if isinstance(node.get("bridgeSls"), list) and "effects" in node and "fatigueSpectra" not in node:
            node["fatigueSpectra"] = []; node["reliabilityAnalyses"] = []; node["testSeries"] = []; changed = True
        elif isinstance(node.get("bridgeSls"), dict) and "removed" in node["bridgeSls"] and "fatigueSpectra" not in node:
            node["fatigueSpectra"] = EMPTY(); node["reliabilityAnalyses"] = EMPTY(); node["testSeries"] = EMPTY(); changed = True
        for v in list(node.values()):
            changed |= fix(v)
    elif isinstance(node, list):
        for v in node: changed |= fix(v)
    return changed
n = skipped = 0
for dp, dn, fn in os.walk(ROOT):
    for f in fn:
        if not f.endswith(".json"): continue
        p = os.path.join(dp, f)
        t = open(p, encoding="utf8").read()
        if '"bridgeSls"' not in t and '"qk"' not in t: continue
        try: d = json.loads(t)
        except Exception: skipped += 1; print("unparsable", p[-80:]); continue
        if json.dumps(d, indent=2, ensure_ascii=False) + "\n" != t:
            skipped += 1; print("format differs", p[-90:]); continue
        if fix(d):
            open(p, "w", encoding="utf8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n"); n += 1
print("updated", n, "skipped", skipped)
