"""💾️ R5: stats (never opens for write, never touches atime-relevant state beyond a stat) every blob the snapshot's entries reference in the host CAS, and computes what enforce would free: blobs referenced only by entries enforce retires (stale, unreached, optionally undeclared older than N days).

Usage: blobs.py <snapshot.json> <rows.json> <out.json>
"""
import collections, json, os, sys

snap, rows = json.load(open(sys.argv[1])), json.load(open(sys.argv[2]))
CAS = os.path.expanduser("~/.cache/semio/cas/v1/o")
size = {}
for e in snap["entries"]:
    for h in (e["meta"] or {}).get("refs", []):
        if h not in size:
            try:
                size[h] = os.stat(os.path.join(CAS, h[:2], h[2:])).st_blocks * 512
            except OSError:
                size[h] = None
holders = collections.defaultdict(set)
for e in snap["entries"]:
    for h in (e["meta"] or {}).get("refs", []):
        holders[h].add(e["key"])
complete = collections.Counter()
for e in snap["entries"]:
    refs = (e["meta"] or {}).get("refs", [])
    v = rows[e["key"]]["verdict"]
    complete[(v, "no-refs" if not refs else "complete" if all(size[h] is not None for h in refs) else "blob-missing")] += 1


def freed(retire):
    gone = {k for k, r in rows.items() if retire(r)}
    bytes_ = sum(s for h, s in size.items() if s and holders[h] and holders[h] <= gone)
    return {"entries": len(gone), "bytes": bytes_, "gib": round(bytes_ / 2**30, 2)}


now = snap["takenAt"]
out = {
    "referencedBlobs": len(size), "presentBlobs": sum(1 for s in size.values() if s is not None),
    "referencedGiB": round(sum(s for s in size.values() if s) / 2**30, 2),
    "completeness": {f"{v}|{c}": n for (v, c), n in sorted(complete.items())},
    "enforceFrees": freed(lambda r: r["verdict"].startswith("stale") or r["verdict"].startswith("unreached")),
    "staleOnlyFrees": freed(lambda r: r["verdict"].startswith("stale")),
    "unreachedOnlyFrees": freed(lambda r: r["verdict"].startswith("unreached")),
    "undeclaredFrees": freed(lambda r: r["verdict"] == "undeclared"),
}
json.dump({**out, "size": size}, open(sys.argv[3], "w"))
print(json.dumps(out, indent=1))
