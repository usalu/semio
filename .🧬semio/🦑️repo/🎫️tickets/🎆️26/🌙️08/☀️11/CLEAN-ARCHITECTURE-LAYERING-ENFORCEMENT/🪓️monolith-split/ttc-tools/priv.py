"""🔐 Unique privacy and internal (non-stub) NEW diagnostics of an oracle result, grouped by what has to be exposed.
usage: priv.py <result.json> [cfg=test] [--all]"""
import json, re, sys, collections
r = json.load(open(sys.argv[1], encoding="utf-8"))
cfg = next((a for a in sys.argv[2:] if not a.startswith("--")), "test")
show_all = "--all" in sys.argv
e = r["summary"][cfg]
grp = collections.defaultdict(set)
other = collections.Counter()
for d in e["new_list"]:
    m = d["msg"]
    if d["code"] in ("E0616", "E0624", "E0603", "E0451"):
        t = re.sub(r"__m_[a-z_0-9]+::component::", "", m)
        t = re.sub(r"__m_[a-z_0-9]+::", "", t)
        grp[d["code"]].add(t[:170])
    elif show_all or "__ext" not in m + d["label"]:
        mm = re.search(r"__m_([A-Za-z0-9_]+)/(.*)$", d["file"])
        other[f'{d["code"]} {m[:140]} @ {(mm.group(2) if mm else d["file"])[-48:]}:{d["line"]}'] += 1
for k, v in sorted(grp.items()):
    print("==", k, len(v))
    for x in sorted(v):
        print("  ", x)
print("== other internal NEW", sum(other.values()))
for k, n in sorted(other.items()):
    print("  ", n, k)
