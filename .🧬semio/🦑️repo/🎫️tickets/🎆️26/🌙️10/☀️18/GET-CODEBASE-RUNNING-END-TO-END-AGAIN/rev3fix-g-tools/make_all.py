import sys, hashlib, json, os
import make_lib
for name in sys.argv[1:]:
    __import__(name)
make_lib.write("rev3fix")
bases = {}
for e in make_lib.spec["edits"]:
    bases[e["file"]] = hashlib.sha256(open(os.path.join(make_lib.ROOT, e["file"]), "rb").read()).hexdigest()
json.dump(bases, open(os.path.join(make_lib.HERE, "staged", "rev3fix", "bases.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(make_lib.spec["edits"]), "edits", len(make_lib.spec["newFiles"]), "new files", len(bases), "base hashes")
