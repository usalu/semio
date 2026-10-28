"""🔁 resync.py <mirror root> <batch...>: copies the LIVE base bytes of every file of the batches into a mirror (clonefile via cp -c), so a changed batch can be applied again."""
import json, os, subprocess, sys
here = os.path.dirname(os.path.abspath(__file__)); live = os.path.abspath(os.path.join(here, *[".."] * 8)); root = sys.argv[1]
for batch in sys.argv[2:]:
    for entry in json.load(open(os.path.join(here, "staged", batch, "edits.json"), encoding="utf8"))["files"]:
        target = os.path.join(root, entry["file"])
        if entry.get("created"):
            if os.path.exists(target): os.remove(target)
            continue
        os.makedirs(os.path.dirname(target), exist_ok=True)
        if os.path.exists(target): os.remove(target)
        subprocess.check_call(["cp", "-c", os.path.join(live, entry["file"]), target])
print("resynced", sys.argv[2:])
