"""🔎 Audits an old semio tree against the canonical fleet worktree and the fleet store.

Every file of the old tree is classified as `same` (identical bytes at the same path in the canonical tree),
`fleet-history` (other bytes, but the store holds a blob with this sha256, so it is an older fleet version),
or `unique` (bytes the fleet never held: a local change that must be merged or consciously dropped).
Usage: python3 -I audit-old-tree.py <old-tree> <canonical-tree> <store-root> <report.json>
"""
import filecmp, hashlib, json, os, sys

old, canonical, store, report = sys.argv[1:5]
SKIP = {".git", ".cache", "node_modules"}


def sha256(path):
    digest = hashlib.sha256()
    with open(path, "rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


counts = {"same": 0, "fleet-history": 0, "unique": 0, "links": 0}
history, unique = [], []
for root, directories, files in os.walk(old):
    if root == old:
        directories[:] = [name for name in directories if name not in SKIP]
    else:
        directories[:] = [name for name in directories if name != "node_modules"]
    for name in files:
        path = os.path.join(root, name)
        relative = os.path.relpath(path, old)
        if os.path.islink(path):
            counts["links"] += 1
            continue
        other = os.path.join(canonical, relative)
        if os.path.isfile(other) and not os.path.islink(other) and filecmp.cmp(path, other, shallow=False):
            counts["same"] += 1
            continue
        digest = sha256(path)
        entry = {"path": relative, "sha256": digest, "bytes": os.path.getsize(path), "modified": int(os.path.getmtime(path)), "inCanonical": os.path.lexists(other)}
        if os.path.exists(os.path.join(store, "v1", "blobs", digest[:2], digest)):
            counts["fleet-history"] += 1
            history.append(entry)
        else:
            counts["unique"] += 1
            unique.append(entry)
        total = sum(counts.values())
        if total % 5000 == 0:
            print(json.dumps({"event": "progress", "done": total}), flush=True)
with open(report, "w") as stream:
    json.dump({"old": old, "canonical": canonical, "counts": counts, "unique": unique, "fleetHistory": history}, stream, ensure_ascii=False, indent=1)
print(json.dumps({"event": "done", **counts}), flush=True)
