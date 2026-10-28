"""🛂️ Writes every file of the staged batch into a MIRROR regardless of its state (the mirror is scratch), refusing any root that is not below a `/sm/` mirror folder: `python3 -I forceapply.py MIRROR_ROOT`."""
import json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
root = os.path.abspath(sys.argv[1])
if "/sm/" not in root + "/":
    sys.exit("refusing: not a mirror")
staged = os.path.join(HERE, "staged", "acceptance")
for rel, row in json.load(open(os.path.join(staged, "manifest.json"), encoding="utf8")).items():
    target = os.path.join(root, rel)
    os.makedirs(os.path.dirname(target), exist_ok=True)
    shutil.copyfile(os.path.join(staged, "files", row["file"]), target)
print("forced", root)
