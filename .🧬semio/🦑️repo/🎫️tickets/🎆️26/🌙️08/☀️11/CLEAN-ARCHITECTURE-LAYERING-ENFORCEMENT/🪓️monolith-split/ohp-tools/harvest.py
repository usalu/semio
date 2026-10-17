"""📋 Harvest non-stub rows of the wanted error codes from a pos.py result json.
usage: harvest.py <result.json> [--codes E0425,...] [--cfg lib|test] [--all-codes]"""
import json, re, sys, collections
WANT = "E0425 E0432 E0433 E0412 E0599 E0061 E0308 E0603 E0616 E0624 E0107 E0282".split()
a = sys.argv[1:]
r = json.load(open(a[0], encoding="utf-8"))
codes = set(a[a.index("--codes") + 1].split(",")) if "--codes" in a else set(WANT)
if "--all-codes" in a:
    codes = None
only = a[a.index("--cfg") + 1] if "--cfg" in a else None
for cfg, e in r["summary"].items():
    if only and cfg != only:
        continue
    rows = collections.OrderedDict()
    for d in e["new_list"]:
        if d["stub"] or d.get("inherited"):
            continue
        if codes is not None and d["code"] not in codes:
            continue
        m = re.match(r"^__m_([A-Za-z0-9_]+)/(.*)$", d["file"])
        pkg, rel = (m.group(1), m.group(2)) if m else ("", d["file"])
        k = (d["code"], pkg, rel, re.sub(r"__m_[A-Za-z0-9_]+", "__m_X", d["msg"])[:200])
        rows.setdefault(k, []).append(d["line"])
    print(f"== cfg {cfg}: {len(rows)} grouped rows / {sum(len(v) for v in rows.values())} errors")
    for (code, pkg, rel, msg), lines in rows.items():
        print(f"{code} [{pkg.replace('semio_s_artifact_','').replace('semio_s_','')}] {rel} x{len(lines)} L{sorted(set(lines))[:8]}: {msg}")
