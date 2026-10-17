import json, glob, os, re, sys
G = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated")
rows = []
for ex in sorted(glob.glob(os.path.join(G, "pgr-*.exit"))):
    tag = os.path.basename(ex)[4:-5]
    e = open(ex).read().strip()
    log = os.path.join(G, "pgr-%s.log" % tag)
    try:
        d = json.load(open(log, encoding="utf8"))
    except Exception:
        rows.append((tag, e, "UNREADABLE", "", "", "", "", ""))
        continue
    lay = d.get("layout", {})
    n = lay.get("native", {})
    t = lay.get("--test", {})
    rows.append((tag, e, d.get("base"), d.get("crates"), "%s/%s/%s" % (n.get("base"), n.get("combined"), n.get("onlyCombined")), "%s/%s/%s" % (t.get("base"), t.get("combined"), t.get("onlyCombined")), "heal=%d sib=%d adm=%d shr=%d" % (len(d.get("healedBase", [])), len(d.get("siblings", [])), len(d.get("admission", [])), len(d.get("shared", []))), ""))
for r in rows:
    print(" | ".join(str(x) for x in r))
