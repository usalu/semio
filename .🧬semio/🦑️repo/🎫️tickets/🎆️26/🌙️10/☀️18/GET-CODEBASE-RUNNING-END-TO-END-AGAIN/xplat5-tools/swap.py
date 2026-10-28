"""🔁 Swaps the files of a staged batch in a MIRROR root between `base` and `new` (negative controls: files that did not exist before the batch, the new laws, stay): python3 -I swap.py <batch-dir> <root> base|new. Refuses a root that holds a .git directory (the live checkout)."""
import json, os, shutil, sys
batch, root, side = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2]), sys.argv[3]
if os.path.exists(os.path.join(root, ".git")): sys.exit("refusing to write into a checkout with .git: " + root)
rows = json.load(open(os.path.join(batch, "manifest.json"), encoding="utf8"))
for rel, row in rows.items():
    target = os.path.join(root, rel)
    if side == "base":
        if row["base"] is not None: shutil.copyfile(os.path.join(batch, "base", row["file"]), target)
    else:
        os.makedirs(os.path.dirname(target), exist_ok=True); shutil.copyfile(os.path.join(batch, "files", row["file"]), target)
