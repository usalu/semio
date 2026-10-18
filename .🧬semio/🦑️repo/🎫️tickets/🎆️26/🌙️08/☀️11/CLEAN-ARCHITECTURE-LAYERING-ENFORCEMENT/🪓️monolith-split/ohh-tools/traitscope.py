"""🧭 OHH trait-scope digest (read only): lists E0599 `trait ... implemented but not in scope` rows of the family's own crates with the compiler's suggested import and the use-site file (repo path).
usage: traitscope.py <final.err> <family-json>"""
import json, os, re, sys, collections
err, fam = sys.argv[1:3]
spec = json.load(open(fam, encoding="utf-8"))
own = {c.replace("-", "_") for c in spec.get("own", spec["crates"])}
rows = collections.OrderedDict()
for l in open(err, encoding="utf-8"):
    if not l.startswith("{"):
        continue
    try:
        d = json.loads(l)
    except Exception:
        continue
    if (d.get("code") or {}).get("code") != "E0599":
        continue
    prim = [s for s in d["spans"] if s["is_primary"]]
    if not prim:
        continue
    m = re.match(r"__m_([A-Za-z0-9_]+)/(.*)", prim[0]["file_name"])
    if not m or m.group(1) not in own:
        continue
    sugg = [s["suggested_replacement"].strip() for c in d.get("children", []) for s in c.get("spans", []) if (s.get("suggested_replacement") or "").strip().startswith("use ")]
    if not sugg:
        continue
    key = (m.group(1), os.path.normpath(m.group(2)), re.sub(r"crate::__m_[A-Za-z0-9_]+?::", "", sugg[0]) if False else sugg[0])
    rows.setdefault(key, []).append(prim[0]["line_start"])
for (c, f, s), ls in rows.items():
    print(f"{c} | {f} | {s} | lines {sorted(set(ls))[:5]}")
