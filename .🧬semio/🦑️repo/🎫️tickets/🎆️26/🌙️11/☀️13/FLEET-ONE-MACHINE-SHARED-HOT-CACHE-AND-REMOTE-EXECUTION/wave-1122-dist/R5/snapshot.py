"""📸️ R5: read-only snapshot of the live fleet cache server store (index + sidecars + output names) and fresh sha256 of every declared source in the coordinator tree and in the play clone.

Opens one file at a time, writes only into the ticket's generated folder. Never unlinks, renames or touches anything in the store.
"""
import hashlib, json, os, re, sys, time

STORE = "/Users/ueli/Documents/semio/.cache/b/p/m/fleet/cache-server/v1"
TREE = "/Users/ueli/Documents/semio"
PLAY = "/Users/ueli/.sinc/play"
OUT = sys.argv[1]
OUTPUT = re.compile(rb"out/([A-Za-z0-9_\-.@]+)")


def sha(path):
    try:
        if not os.path.isfile(path) or os.path.islink(path) and not os.path.isfile(os.path.realpath(path)):
            return None
        h = hashlib.sha256()
        with open(path, "rb") as f:
            for chunk in iter(lambda: f.read(1 << 20), b""):
                h.update(chunk)
        return h.hexdigest()
    except OSError:
        return None


started = time.time()
entries = []
for ns in sorted(os.listdir(STORE)):
    if ns.startswith(".") or not os.path.isdir(os.path.join(STORE, ns)):
        continue
    for kind in sorted(os.listdir(os.path.join(STORE, ns))):
        base = os.path.join(STORE, ns, kind)
        for shard in sorted(os.listdir(base)):
            for name in sorted(os.listdir(os.path.join(base, shard))):
                path = os.path.join(base, shard, name)
                try:
                    info = os.stat(path)
                    with open(path, "rb") as f:
                        head = f.read(65536)
                except OSError:
                    continue
                key = f"{ns}/{kind}/{shard}/{name}"
                meta = None
                try:
                    with open(os.path.join(STORE, ".meta", ns, kind, shard, name + ".json")) as f:
                        meta = json.load(f)
                except (OSError, ValueError):
                    pass
                names = sorted({m.decode("utf8", "replace") for m in OUTPUT.findall(head)})
                manifests = [m for m in ("unit.json", "stage.json", "inputs.json") if m.encode() in head]
                entries.append({"key": key, "size": info.st_size, "mtime": info.st_mtime * 1000, "meta": meta, "outputs": names[:8], "manifests": manifests})
paths = sorted({row["path"] for e in entries if e["meta"] and e["meta"].get("declaration") for row in e["meta"]["declaration"].get("sources", [])})
tree = {p: sha(os.path.join(TREE, *p.split("/"))) for p in paths}
play = {p: sha(os.path.join(PLAY, *p.split("/"))) for p in paths}
with open(OUT, "w") as f:
    json.dump({"takenAt": started * 1000, "finishedAt": time.time() * 1000, "entries": entries, "tree": tree, "play": play}, f)
print(json.dumps({"entries": len(entries), "paths": len(paths), "seconds": round(time.time() - started, 1)}))
