# 🧷️ Symbol usage: for every free item defined in a WP5 staging crate, the crates (and non-staging files) that mention it.
import os, re, sys, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import stg, items, shellsrc as S
from blank import blank

def corpus():
    out = {}
    for dp, dn, fn in os.walk(S.SH):
        dn[:] = [x for x in dn if x != "node_modules"]
        for f in fn:
            if f.endswith(".rs"):
                p = os.path.join(dp, f); out[p] = open(p, encoding="utf-8").read()
    return out

def owner(p):
    rel = os.path.relpath(p, S.SH)
    if rel.startswith("📥️staging/"):
        d = rel.split("/")[1]
        return stg.DIR.get(d, "t1:" + d)
    if rel.startswith("🧪️tests/"): return "tests"
    return "residual:" + rel.split("/")[0]

def defs():
    out = []
    for d, c in stg.CRATES:
        for it in items.free_items(d):
            if it["kind"] in ("impl", "mod", "use") or not it["name"]: continue
            nm = re.match(r"[A-Za-z_][A-Za-z_0-9]*", it["name"])
            if not nm: continue
            it["ident"] = nm.group(0); it["crate"] = c; out.append(it)
    return out

if __name__ == "__main__":
    cor = corpus(); blanked = {p: blank(t) for p, t in cor.items()}
    D = defs()
    use = {}
    for it in D:
        rx = re.compile(r"(?<![A-Za-z_0-9])" + re.escape(it["ident"]) + r"(?![A-Za-z_0-9])")
        users = collections.Counter()
        for p, b in blanked.items():
            n = len(rx.findall(b))
            if p == it["path"]:
                raw = cor[p].split("\n"); own = blank("\n".join(raw[it["start"] - 1:it["end"]]))
                n -= len(rx.findall(own))
            if n > 0: users[owner(p)] += n
        it2 = dict(crate=it["crate"], slug=it["slug"], ident=it["ident"], kind=it["kind"], lines=it["end"] - it["start"] + 1, users=dict(users))
        use[it["ident"] + "@" + it["slug"] + "@" + it["crate"]] = it2
    json.dump(use, open(S.MONO + "/🗑️generated/rwp5-symuse.json", "w"), ensure_ascii=False, indent=1)
    # sinkable: min rank of WP5 users
    rows = []
    for k, v in use.items():
        wp = [u for u in v["users"] if u in stg.RANK]
        other = [u for u in v["users"] if u not in stg.RANK]
        lo = min((stg.RANK[u] for u in wp), default=99)
        rows.append((v["crate"], v["lines"], v["ident"], v["slug"], sorted(wp, key=lambda u: stg.RANK[u]), other))
    tot = collections.Counter(); sink = collections.Counter()
    for c, n, i, s, wp, other in rows:
        tot[c] += n
        if not other and wp and stg.RANK[wp[0]] > stg.RANK[c]: sink[c] += n
    print("free lines per crate", dict(tot)); print("lines whose users are all WP5 and all strictly above the defining crate:", dict(sink))
    for c, n, i, s, wp, other in sorted(rows, key=lambda r: (stg.RANK[r[0]], -r[1])):
        if n >= 15: print("%-22s %4d %-40s %-28s users=%s other=%s" % (c, n, i, s[:14], [w.replace("shell-", "") for w in wp], [o[:20] for o in other]))
