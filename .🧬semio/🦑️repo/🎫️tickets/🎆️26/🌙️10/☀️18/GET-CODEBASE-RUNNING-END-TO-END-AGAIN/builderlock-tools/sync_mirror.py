"""👯 Copies the files of a staged batch into a MIRROR root as they are staged (new bytes), refusing any root that is the live checkout.
usage: python3 -I sync_mirror.py <stage dir> <mirror root>"""
import json, os, shutil, sys
stage, root = (os.path.abspath(a) for a in sys.argv[1:3])
if os.path.isdir(os.path.join(root, ".git")): sys.exit("refusing a checkout with .git (live tree)")
for rel, row in json.load(open(os.path.join(stage, "manifest.json"), encoding="utf8")).items():
    target = os.path.join(root, rel); os.makedirs(os.path.dirname(target), exist_ok=True)
    shutil.copyfile(os.path.join(stage, "files", row["file"]), target)
print("synced", len(json.load(open(os.path.join(stage, "manifest.json"), encoding="utf8"))), "files into", root)
