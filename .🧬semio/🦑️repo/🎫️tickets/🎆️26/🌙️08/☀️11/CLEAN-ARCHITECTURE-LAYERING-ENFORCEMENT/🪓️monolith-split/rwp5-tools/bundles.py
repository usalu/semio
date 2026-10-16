# 🧺️ Bundle derivation: per WP5 crate the components its methods touch (fields) and the tier-1 / cross-crate methods they call.
import os, re, json, collections, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import model as M
home, counts = M.appendix_b(); comp, fields = M.components(); ms = M.scan()
order = [c["struct"] for c in comp]
WP5 = ["shell-session", "shell-window-actions", "shell-actions", "shell-pointer", "shell-view-paint", "shell-view-chrome", "shell-view-overlays", "shell-view-panels"]
names = {m["name"] for m in ms}
res = collections.defaultdict(lambda: dict(structs=collections.Counter(), callees=collections.Counter(), methods=0, lines=0))
perm = []
for m in ms:
    h = home.get(m["name"], ["?"])[0]
    if h not in WP5: continue
    r = res[h]; r["methods"] += 1; r["lines"] += m["end"] - m["start"] + 1
    touched = collections.Counter(fields[f] for f in re.findall(r"\bself\.([a-z0-9_]+)\b(?!\s*\()", m["body"]) if f in fields)
    for s in touched: r["structs"][s] += 1
    calls = set(re.findall(r"\bself\.([a-z0-9_]+)\s*(?:::<[^>]*>)?\(", m["body"])) | set(re.findall(r"\bSelf::([a-z0-9_]+)\s*(?:::<[^>]*>)?\(", m["body"]))
    for c in calls:
        if c in names:
            ch = home.get(c, ["?"])[0]
            if ch != h: r["callees"][ch] += 1
    perm.append((h, m["name"], sorted(touched), sorted({home[c][0] for c in calls if c in home and home[c][0] != h})))
for c in WP5:
    r = res[c]
    print("==", c, r["methods"], "methods", r["lines"], "lines")
    print("  structs:", sorted(r["structs"].items(), key=lambda kv: order.index(kv[0])))
    print("  callee crates:", dict(r["callees"]))
json.dump({k: dict(structs=dict(v["structs"]), callees=dict(v["callees"])) for k, v in res.items()}, open(os.path.dirname(os.path.abspath(__file__)) + "/../🗑️generated/rwp5-bundles.json", "w"), indent=1, ensure_ascii=False)
