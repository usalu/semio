"""📸 Byte backup of the files a staged batch touches: `snap.py save <batch> <list>` / `snap.py restore <batch>`; a listed path that is absent is recorded and removed again on restore."""
import json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
STORE = os.path.join(HERE, "run", "backups")


def save(batch, listing):
    base = os.path.join(STORE, batch)
    os.makedirs(base, exist_ok=True)
    paths = [line.rstrip("\n") for line in open(listing, encoding="utf8") if line.strip()]
    manifest = {}
    for path in paths:
        source = os.path.join(ROOT, path)
        manifest[path] = os.path.isfile(source)
        if manifest[path]:
            target = os.path.join(base, "files", path)
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copy2(source, target)
    json.dump(manifest, open(os.path.join(base, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print(f"saved {sum(manifest.values())} files, {len(manifest) - sum(manifest.values())} absent")


def restore(batch):
    base = os.path.join(STORE, batch)
    manifest = json.load(open(os.path.join(base, "manifest.json"), encoding="utf8"))
    for path, existed in manifest.items():
        target = os.path.join(ROOT, path)
        if existed:
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copy2(os.path.join(base, "files", path), target)
        elif os.path.isfile(target):
            os.remove(target)
    print(f"restored {len(manifest)} paths")


save(sys.argv[2], sys.argv[3]) if sys.argv[1] == "save" else restore(sys.argv[2])
