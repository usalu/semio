"""🔭️ R5: independent judge of the live snapshot, written from the user's rule (an entry is current when every file it read equals the coordinator's tree and every entry whose output it consumed is current) and compared entry by entry with the server judge's verdicts.

Usage: oracle.py <snapshot.json> <verdicts.json> <report.json> [tree|play]
"""
import collections, json, sys

snap = json.load(open(sys.argv[1]))
server = json.load(open(sys.argv[2]))
which = sys.argv[4] if len(sys.argv) > 4 else "tree"
digest = snap[which]
now, GRACE = snap["takenAt"], 600_000
entries = {e["key"]: e for e in snap["entries"] if e["key"].split("/")[1] != "cas"}


def decl(e):
    d = (e["meta"] or {}).get("declaration")
    return d if d and d.get("version") == 2 else None


producers, by_self = collections.defaultdict(set), collections.defaultdict(set)
for k, e in entries.items():
    for h in (e["meta"] or {}).get("refs", []):
        producers[h].add(k)
    d = decl(e)
    if d and d.get("self"):
        by_self[d["self"]].add(k)

own = {}
for k, e in entries.items():
    d = decl(e)
    if not d:
        continue
    for row in d["sources"]:
        now_sha = digest.get(row["path"])
        if now_sha is None:
            own[k] = ("deleted", row["path"])
            break
        if d["kind"] != "input-manifest" and now_sha != row["sha256"]:
            own[k] = ("edited", row["path"])
            break

stale = dict(own)
changed = True
while changed:
    changed = False
    for k, e in entries.items():
        d = decl(e)
        if not d or k in stale or d["kind"] == "graph":
            continue
        edges = [producers[h] - {k} for h in d["externs"]] + [by_self[u] - {k} for u in d.get("units", [])]
        for holders in edges:
            if holders and all(o in stale for o in holders):
                stale[k] = ("extern", sorted(holders)[0])
                changed = True
                break


def young(e):
    return now - ((e["meta"] or {}).get("storedAt") or 0) < GRACE


def rooted(d):
    return bool(d["sources"]) or d["kind"] in ("stage-output", "graph", "input-manifest")


verdict, live_roots = {}, []
for k, e in entries.items():
    d = decl(e)
    if not d:
        verdict[k] = "undeclared"
    elif d["kind"] == "graph":
        verdict[k] = "live"; live_roots.append(k)
    elif d["kind"] == "input-manifest" and not d["sources"] and k not in own:
        verdict[k] = "live"
    elif k in stale and not young(e):
        verdict[k] = "stale:" + stale[k][0]
    elif rooted(d):
        verdict[k] = "grace" if k in stale else "live"; live_roots.append(k)
reached, todo = set(), list(live_roots)
while todo:
    k = todo.pop()
    d = decl(entries[k])
    for o in set().union(*[producers[h] for h in d["externs"]], *[by_self[u] for u in d.get("units", [])]) if (d["externs"] or d.get("units")) else set():
        if o == k or o in reached or o in verdict:
            continue
        od = decl(entries[o])
        if not od or rooted(od):
            continue
        reached.add(o); todo.append(o)
for k, e in entries.items():
    if k in verdict:
        continue
    verdict[k] = "live" if k in reached else "grace" if young(e) else "unreached:unreached"

srv = {k: v["state"] + (":" + v["reason"] if v.get("reason") else "") for k, v in server.items()}
diff = [(k, srv.get(k), verdict[k]) for k in entries if srv.get(k) != verdict[k]]
counts = collections.Counter(verdict.values())
json.dump({"counts": counts, "server": collections.Counter(srv.values()), "disagree": diff[:200], "disagreeCount": len(diff), "stale": {k: v for k, v in stale.items()}, "verdict": verdict}, open(sys.argv[3], "w"))
print(json.dumps({"oracle": counts, "disagree": len(diff), "first": diff[:5]}))
