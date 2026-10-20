"""📦 Builds the staged set from the write-isolated overlay: every regular file below the given overlay directories that differs from the live tree (or is new) becomes one op with the base sha256 of the live file. usage: stage.py <overlay> <repoRoot> <outDir> <relDir>...   (scratch files named __orig_* are ignored)"""
import hashlib, json, os, shutil, sys

ov, root, out = sys.argv[1:4]
dirs = [d.rstrip("/") for d in sys.argv[4:]] or [""]
sha = lambda b: hashlib.sha256(b).hexdigest()
ops, seen, stale = [], set(), []

def walk(rel):
    d = os.path.join(ov, rel)
    for name in sorted(os.listdir(d)):
        p = os.path.join(d, name)
        r = os.path.join(rel, name)
        if os.path.islink(p):
            continue
        if os.path.isdir(p):
            walk(r)
        elif os.path.isfile(p) and not name.startswith("__orig_"):
            if r in seen:
                continue
            seen.add(r)
            new = open(p, "rb").read()
            live = os.path.join(root, r)
            if os.path.isfile(live):
                old = open(live, "rb").read()
                if old == new:
                    continue
                if os.stat(live).st_mtime > os.stat(p).st_birthtime + 1:
                    stale.append(r)
                ops.append({"op": "modify", "path": r, "baseSha256": sha(old), "sha256": sha(new), "bytes": len(new)})
            else:
                ops.append({"op": "create", "path": r, "baseSha256": None, "sha256": sha(new), "bytes": len(new)})
            blobs[sha(new)] = new

blobs = {}
for d in dirs:
    if os.path.isdir(os.path.join(ov, d)):  # noqa
        walk(d)
if os.path.isdir(os.path.join(out, "blobs")):
    shutil.rmtree(os.path.join(out, "blobs"))
os.makedirs(os.path.join(out, "blobs"), exist_ok=True)
for h, b in blobs.items():
    open(os.path.join(out, "blobs", h[:24]), "wb").write(b)
if stale:
    print("STALE (live file changed after the overlay copy; re-derive):")
    for r in stale:
        print("  " + r)
    sys.exit(2)
longest = max((len(o["path"]) for o in ops), default=0)
json.dump({"schema": "fnsplit4a-finish/1", "ops": ops}, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(f"staged ops={len(ops)} modify={sum(o['op']=='modify' for o in ops)} create={sum(o['op']=='create' for o in ops)} blobs={len(blobs)} longestPath={longest}")
