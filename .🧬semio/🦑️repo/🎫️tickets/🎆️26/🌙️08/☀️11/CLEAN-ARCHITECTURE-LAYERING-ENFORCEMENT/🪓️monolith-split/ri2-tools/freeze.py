#!/usr/bin/env python3
# 🧊 Updates the frozen crate table (🗑️generated/rwp0-crates.json) to the first-party edges the live manifests declare, and checks the rank order.
# usage: freeze.py [--apply]   (prints every crate whose dependency set differs; with --apply writes the JSON and the Appendix F rows of the plan)
import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
import depscheck

table = crate_table()
J = GEN + "/rwp0-crates.json"
frozen = json.load(open(J))
changes = {}
for k, c in frozen.items():
    if k not in table:
        continue
    man = T + "/" + table[k] + "/📦️packages/🦀️rust/Cargo.toml"
    d = depscheck.toml(man)
    tabs = [d.get("dependencies", {})] + [v.get("dependencies", {}) for v in d.get("target", {}).values()]
    live = set()
    for tb in tabs:
        for name in tb:
            if name.startswith(PKG_PREFIX):
                live.add(name[len(PKG_PREFIX):])
    old = set(c["deps"])
    if live != old:
        changes[k] = (sorted(old - live), sorted(live - old))
bad = []
for k, (rm, add) in changes.items():
    for a in add:
        if frozen[a]["rank"] >= frozen[k]["rank"]:
            bad.append((k, a))
print("crates with changed edges:", len(changes))
for k, (rm, add) in sorted(changes.items(), key=lambda kv: frozen[kv[0]]["rank"]):
    print(f"  {k}: -{rm} +{add}")
print("rank violations:", bad)
if "--apply" in sys.argv and not bad:
    for k, (rm, add) in changes.items():
        keep = [x for x in frozen[k]["deps"] if x not in rm]
        keep += [x for x in add]
        keep.sort(key=lambda x: frozen[x]["rank"])
        frozen[k]["deps"] = keep
    json.dump(frozen, open(J, "w"), ensure_ascii=False, indent=1)
    plan = SPLIT + "/PLAN-RENDERER-WGPU-CRATES.md"
    text = read(plan)
    for k in changes:
        pat = re.compile(r"^(\| \d+ \| `" + re.escape(k) + r"` \| [^\n]*? \| \d+ \| \S+ \| [^|]+ \| \d+ \| )([^\n]*) \|$", re.M)
        m = pat.search(text)
        assert m, k
        deps = ", ".join("`" + x + "`" for x in frozen[k]["deps"]) or "-"
        text = text[:m.start(2)] + deps + text[m.end(2):]
    write(plan, text)
    print("applied")
