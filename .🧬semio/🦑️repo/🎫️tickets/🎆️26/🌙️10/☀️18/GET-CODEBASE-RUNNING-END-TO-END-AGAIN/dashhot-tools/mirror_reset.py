"""🔁 Puts the base bytes of batch `fresh` back into a mirror (new files removed) so `stage.py apply <mirror>` starts from the live base again. usage: python3 -I mirror_reset.py <mirror-root>"""
import json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); S = os.path.join(HERE, "staged", "fresh"); M = os.path.abspath(sys.argv[1])
for rel, row in json.load(open(os.path.join(S, "manifest.json"), encoding="utf8")).items():
    target = os.path.join(M, rel)
    if row["base"] is None:
        if os.path.exists(target): os.remove(target)
    else: shutil.copyfile(os.path.join(S, "base", row["file"]), target)
print("mirror reset to base")
