"""🔗️ R5: who holds the present blobs that only retire-set entries reference (live pin, play unit cache, main unit cache, nobody). Read-only stats.

Usage: held.py <snapshot.json> <rows.json> <blobs.json>
"""
import collections, json, os, sys
snap, rows, size = json.load(open(sys.argv[1])), json.load(open(sys.argv[2])), json.load(open(sys.argv[3]))["size"]
V1 = os.path.expanduser("~/.cache/semio/cas/v1")
pinned = set(json.load(open(os.path.join(V1, "p", [p for p in os.listdir(os.path.join(V1, "p")) if p.startswith("remote-cache-live-67402")][0])))["digests"])
holders = {json.load(open(os.path.join(V1, "h", f)))["root"]: f[:-5] for f in os.listdir(os.path.join(V1, "h"))}
owners = collections.defaultdict(set)
for e in snap["entries"]:
    for h in (e["meta"] or {}).get("refs", []):
        owners[h].add(e["key"])
for name, test in [("stale+unreached", lambda r: r["verdict"].startswith(("stale", "unreached"))), ("undeclared", lambda r: r["verdict"] == "undeclared")]:
    gone = {k for k, r in rows.items() if test(r)}
    tally = collections.Counter()
    for h, s in size.items():
        if not s or not owners[h] <= gone:
            continue
        who = [label for root, label in holders.items() if os.path.exists(os.path.join(V1, "k", label, h[:2], h[2:]))]
        cls = "pinned" if h in pinned else ("held:" + ",".join(sorted("play" if "15344" in w else "main" for w in who))) if who else "nobody"
        tally[cls] += s
    print(name, {k: round(v / 2**30, 3) for k, v in tally.items()})
