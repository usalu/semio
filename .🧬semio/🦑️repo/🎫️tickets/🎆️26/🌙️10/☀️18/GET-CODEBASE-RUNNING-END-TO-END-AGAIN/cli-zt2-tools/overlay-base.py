"""Records the content hash of every text file of an overlay right after it was copied from the live tree, or lists what changed since; usage: overlay-base.py record|changed <overlay> <base.json>"""
import hashlib, json, os, sys
mode, overlay, base = sys.argv[1:4]
skip = {"node_modules"}
def walk():
    for directory, names, files in os.walk(overlay):
        names[:] = [name for name in names if name not in skip]
        for file in files:
            path = os.path.join(directory, file)
            yield os.path.relpath(path, overlay), path
digest = lambda path: hashlib.sha256(open(path, "rb").read()).hexdigest()
if mode == "record":
    json.dump({rel: digest(path) for rel, path in walk()}, open(base, "w", encoding="utf-8"), ensure_ascii=False)
else:
    before = json.load(open(base, encoding="utf-8"))
    now = {rel: path for rel, path in walk()}
    print(json.dumps({"changed": sorted(rel for rel, path in now.items() if rel in before and digest(path) != before[rel]), "created": sorted(rel for rel in now if rel not in before), "removed": sorted(rel for rel in before if rel not in now)}, ensure_ascii=False, indent=1))
