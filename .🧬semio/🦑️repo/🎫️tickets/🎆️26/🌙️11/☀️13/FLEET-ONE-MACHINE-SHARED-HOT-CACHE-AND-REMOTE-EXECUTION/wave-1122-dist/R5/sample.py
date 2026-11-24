"""🎯️ R5: draws a stratified random sample of 30 stale and 30 live entries and re-proves each verdict from first principles: re-hashes every declared source in the coordinator tree NOW, follows the stale chain to the edited file, and for every stale unit looks for the current-tree rebuild of the same slot (a live entry with the same slot whose key differs and whose externs no longer name the stale blob: proof the current tree computes another key, so the stale entry can never be hit). Read-only.

Usage: sample.py <snapshot.json> <rows.json> <blobs.json> <out.md-table> [seed]
"""
import collections, hashlib, json, os, random, sys

snap, rows, size = json.load(open(sys.argv[1])), json.load(open(sys.argv[2])), json.load(open(sys.argv[3]))["size"]
rng = random.Random(int(sys.argv[5]) if len(sys.argv) > 5 else 1124)
TREE, CAS = "/Users/ueli/Documents/semio", os.path.expanduser("~/.cache/semio/cas/v1/o")
E = {e["key"]: e for e in snap["entries"]}
TARGET = {"std-1f566e83c93908dd": "native", "core-c9f2b40513075628": "native", "std-8b49f48fd752ae51": "wasm32-wasip2", "core-f9be7ff82408cd93": "wasm32-wasip2", "std-52aec09eb0a29f6c": "wasm32-unknown", "core-27935203a0d853ed": "wasm32-unknown"}


def decl(e):
    d = (e["meta"] or {}).get("declaration")
    return d if d and d.get("version") == 2 else None


def sha(rel):
    try:
        return hashlib.sha256(open(os.path.join(TREE, *rel.split("/")), "rb").read()).hexdigest()
    except OSError:
        return None


def target(e):
    for h in (e["meta"] or {}).get("refs", []):
        p = os.path.join(CAS, h[:2], h[2:])
        try:
            if os.path.getsize(p) > 65536:
                continue
            j = json.loads(open(p, "rb").read())
        except Exception:
            continue
        rows_ = j if isinstance(j, list) else j.get("dependencies", []) if isinstance(j, dict) else []
        if rows_:
            for row in rows_:
                if isinstance(row, list) and row and row[0] in TARGET:
                    return TARGET[row[0]]
    return "?"


producers = collections.defaultdict(set)
by_slot = collections.defaultdict(list)
for k, e in E.items():
    for h in (e["meta"] or {}).get("refs", []):
        producers[h].add(k)
    if e["meta"] and e["meta"].get("slot"):
        by_slot[e["meta"]["slot"]].append(k)


def pick(pred, n):
    pool = sorted(k for k, r in rows.items() if pred(k, r))
    return rng.sample(pool, min(n, len(pool)))


derive_blobs = {h for k, e in E.items() if (rows[k]["crate"] or "").endswith("dsl_derive") for h in (e["meta"] or {}).get("refs", [])}
derive_users = {k for k, e in E.items() if decl(e) and set(decl(e)["externs"]) & derive_blobs}
stale = (pick(lambda k, r: r["verdict"] == "stale:extern" and r["root"] and "value/📋️list" in r["root"][2], 8)
         + pick(lambda k, r: r["verdict"] == "stale:extern" and r["root"] and "value/📋️list" not in r["root"][2] and r["emit"] == "link" and k not in derive_users, 10)
         + pick(lambda k, r: r["verdict"] == "stale:extern" and k in derive_users, 2)
         + pick(lambda k, r: r["verdict"] == "stale:extern" and r["emit"] != "link", 3)
         + pick(lambda k, r: r["verdict"] == "stale:edited" and r["origin"] == "play", 3)
         + pick(lambda k, r: r["verdict"] == "stale:edited" and r["origin"] != "play", 4))
live = (pick(lambda k, r: r["verdict"] == "live" and r["kind"] == "rust-unit" and r["sources"] > 0 and r["emit"] == "link", 8)
        + pick(lambda k, r: r["verdict"] == "live" and r["kind"] == "rust-unit" and r["emit"] == "metadata", 3)
        + pick(lambda k, r: r["verdict"] == "live" and k in derive_users, 2)
        + pick(lambda k, r: r["verdict"] == "live" and r["kind"] == "rust-unit" and r["sources"] == 0, 5)
        + pick(lambda k, r: r["verdict"] == "live" and r["kind"] == "build-script-run", 3)
        + pick(lambda k, r: r["verdict"] == "live" and r["kind"] == "input-manifest", 5)
        + pick(lambda k, r: r["verdict"] == "live" and r["kind"] == "graph", 4))


STALE = json.load(open(sys.argv[2].replace("rows", "oracle")))["stale"]


def chain(k):
    out = [k]
    while STALE.get(k, [None])[0] == "extern" and STALE[k][1] not in out:
        k = STALE[k][1]
        out.append(k)
    return out


def prove_stale(k):
    e, d, r = E[k], decl(E[k]), rows[k]
    fresh = [(s["path"], s["sha256"], sha(s["path"])) for s in d["sources"]]
    own_bad = [(p, a[:10], (b or "gone")[:10]) for p, a, b in fresh if a != b]
    if own_bad and d["kind"] != "input-manifest":
        return "edited now: %s declared %s tree %s" % (own_bad[0][0].split("/")[-2], own_bad[0][1], own_bad[0][2]), own_bad
    c = chain(k)
    root = c[-1]
    rd = decl(E[root])
    rb = [(s["path"], s["sha256"][:10], (sha(s["path"]) or "gone")[:10]) for s in rd["sources"] if sha(s["path"]) != s["sha256"]]
    stale_blobs = {h for h in d["externs"] if producers[h] - {k} and all(not rows[o]["verdict"].startswith("live") for o in producers[h] - {k})}
    rebuilt = [o for o in by_slot.get(e["meta"]["slot"], []) if o != k and rows[o]["verdict"] in ("live", "grace")]
    proof = "no current rebuild in store"
    for o in rebuilt:
        od = decl(E[o])
        if od and od["sources"] == d["sources"] and not (stale_blobs & set(od["externs"])):
            proof = "current rebuild %s: same slot+sources, key differs, externs no longer name the stale blob" % o.split("/")[-1][:12]
            break
        if od and not (stale_blobs & set(od["externs"])):
            proof = "current rebuild %s: same slot, key differs" % o.split("/")[-1][:12]
    return "chain %d deep to %s (%s); root file %s declared %s tree %s; %s" % (len(c), rows[root]["crate"], root.split("/")[-1][:12], rb[0][0].split("/")[-2] if rb else "?", rb[0][1] if rb else "?", rb[0][2] if rb else "?", proof), rb


def prove_live(k):
    e, d, r = E[k], decl(E[k]), rows[k]
    bad = [s["path"] for s in d["sources"] if sha(s["path"]) is None or (d["kind"] != "input-manifest" and sha(s["path"]) != s["sha256"])]
    if bad:
        return "LIVE BUT SOURCE DIFFERS NOW: " + bad[0]
    ext = [h for h in d["externs"] if producers[h] - {k}]
    stale_ext = [h for h in ext if all(rows[o]["verdict"].startswith(("stale",)) for o in producers[h] - {k})]
    if stale_ext:
        return "LIVE BUT EXTERN ALL-STALE: " + stale_ext[0][:12]
    note = "%d sources re-hashed equal; %d/%d externs held by a live/grace producer" % (len(d["sources"]), len(ext), len(d["externs"]))
    if not d["sources"] and d["kind"] not in ("graph", "input-manifest"):
        note += "; reached from a live root (no sources)"
    return note


lines = ["| # | verdict | entry | crate | kind/emit | target | blobs | evidence |", "|---|---|---|---|---|---|---|---|"]
problems = []
for i, k in enumerate(stale + live, 1):
    e, r = E[k], rows[k]
    refs = (e["meta"] or {}).get("refs", [])
    blobs = "complete" if refs and all(size.get(h) for h in refs) else "missing %d/%d" % (sum(1 for h in refs if not size.get(h)), len(refs))
    ev = prove_stale(k)[0] if r["verdict"].startswith("stale") else prove_live(k)
    if ev.startswith(("LIVE BUT",)) or (r["verdict"].startswith("stale") and "declared" in ev and ev.split("tree ")[-1].startswith("?")):
        problems.append((k, ev))
    lines.append("| %d | %s | `%s` | %s | %s/%s | %s | %s | %s |" % (i, r["verdict"], k, r["crate"] or "-", r["kind"], r["emit"], target(e), blobs, ev))
open(sys.argv[4], "w").write("\n".join(lines) + "\n")
print("\n".join(lines))
print("PROBLEMS", problems)
