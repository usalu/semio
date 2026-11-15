import sys, json
from author_08_lib import *
for slug in sys.argv[1:]:
    r = check("humans", slug)
    print("##", slug, "fails", sum(1 for m in r["measurements"] if not m["ok"]))
    by = {}
    for m in r["measurements"]:
        if m["ok"] or m["name"].startswith("info"):
            continue
        by.setdefault((m["rule"].replace("volumes-", ""), m.get("scale")), []).append(f"{m['name']}={round(m['value'],3)}/{round(m['limit'],3) if m['limit']<1e6 else ''}" + (f"[{m['note'][:40]}]" if m.get("note") else ""))
    for k, v in sorted(by.items(), key=lambda x: (str(x[0][1]), x[0][0])):
        print("  ", k[0], k[1], "; ".join(v)[:420])
