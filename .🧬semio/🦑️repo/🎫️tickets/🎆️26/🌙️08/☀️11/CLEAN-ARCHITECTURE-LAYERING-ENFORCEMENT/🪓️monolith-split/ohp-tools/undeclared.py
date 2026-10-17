"""🧷 Finds crates whose sources name an indexed workspace crate that is not declared in their Cargo.toml (oracle marker: E0433 `cannot find module or crate X in this scope`).
usage: undeclared.py <final.err>...   prints <using crate> -> <missing crate> (count) with a sample file:line"""
import json, re, sys, collections
idx = json.load(open("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/ohp-manifest-index.json", encoding="utf-8"))
byu = {n.replace("-", "_"): n for n in idx}
rows = collections.OrderedDict()
for p in sys.argv[1:]:
    for line in open(p, encoding="utf-8", errors="replace"):
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("level") != "error":
            continue
        m = re.search(r"cannot find (?:module or crate|crate) `([A-Za-z0-9_]+)` in this scope|use of unresolved module or unlinked crate `([A-Za-z0-9_]+)`|use of undeclared crate or module `([A-Za-z0-9_]+)`|^unresolved import `([A-Za-z0-9_]+)`$|^unresolved import `([A-Za-z0-9_]+)::", d["message"])
        if not m:
            continue
        x = next(g for g in m.groups() if g)
        if x not in byu:
            continue
        pr = [s for s in d["spans"] if s["is_primary"]]
        if not pr:
            continue
        fm = re.match(r"^__m_([A-Za-z0-9_]+)/", pr[0]["file_name"])
        user = fm.group(1) if fm else "?"
        k = (user, x)
        r = rows.setdefault(k, [0, pr[0]["file_name"].split("artifacts/")[-1][-70:], pr[0]["line_start"]])
        r[0] += 1
for (u, x), (n, f, ln) in rows.items():
    print(f"{u} -> {x} x{n}  {f}:{ln}")
