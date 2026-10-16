"""↩️ Reverts one split of pfb: removes each child's 🦀️.rs named by the spec (and the folder when it is then empty) and restores the original file."""
import json, os, shutil, sys
spec = json.load(open(sys.argv[1], encoding="utf-8"))
directory = os.path.dirname(spec["file"])
for chunk in spec["chunks"]:
    folder = os.path.join(directory, chunk["folder"])
    target = os.path.join(folder, "🦀️.rs")
    if os.path.isfile(target):
        os.remove(target)
    if os.path.isdir(folder) and not os.listdir(folder):
        os.rmdir(folder)
shutil.copyfile(spec["original"], spec["file"])
print("restored", spec["file"])
