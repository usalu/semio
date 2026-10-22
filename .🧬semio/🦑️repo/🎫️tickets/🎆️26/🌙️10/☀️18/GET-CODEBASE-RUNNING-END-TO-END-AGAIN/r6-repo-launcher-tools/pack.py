"""Packs the authored launcher tree (default /private/tmp/repo-launcher/src) into digest-named blobs next to this script, so no ticket path exceeds 256 bytes, and writes manifest.json (repository path -> digest, mode). Usage: pack.py [--src DIR]   |   pack.py --unpack [--src DIR] restores the tree from the blobs."""
import hashlib, json, os, shutil, sys
here = os.path.dirname(os.path.abspath(__file__))
src = sys.argv[sys.argv.index("--src") + 1] if "--src" in sys.argv else "/private/tmp/repo-launcher/src"
blobs = os.path.join(here, "blobs"); manifest_path = os.path.join(here, "manifest.json")
if "--unpack" in sys.argv:
    manifest = json.load(open(manifest_path, encoding="utf-8"))
    for row in manifest["files"]:
        target = os.path.join(src, row["path"]); os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copyfile(os.path.join(blobs, row["sha256"]), target); os.chmod(target, row["mode"])
    print(len(manifest["files"]), "files restored into", src); sys.exit(0)
os.makedirs(blobs, exist_ok=True); rows = []
for dp, dn, fn in os.walk(src):
    dn[:] = [d for d in dn if d != "__pycache__"]
    for f in sorted(fn):
        full = os.path.join(dp, f); data = open(full, "rb").read(); digest = hashlib.sha256(data).hexdigest()
        if not os.path.exists(os.path.join(blobs, digest)): open(os.path.join(blobs, digest), "wb").write(data)
        rows.append({"path": os.path.relpath(full, src), "sha256": digest, "mode": os.stat(full).st_mode & 0o777})
rows.sort(key=lambda row: row["path"])
used = {row["sha256"] for row in rows}
for name in os.listdir(blobs):
    if name not in used: os.remove(os.path.join(blobs, name))
json.dump({"version": 1, "files": rows}, open(manifest_path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(rows), "files,", len(used), "blobs")
