"""📥 C5D TypeScript installer: copies the staged tree of a tssplit tag into the repo. Preconditions: the live root equals its snapshot byte for byte and no child folder exists yet. Children first, then the root.
usage: tsinstall.py check|apply <tag>"""
import os, shutil, sys, json
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
STAGE = "/Users/ueli/.cache/c5d/stage-ts"
mode, tag = sys.argv[1], sys.argv[2]
files = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
root = files[tag]
live = open(os.path.join(REPO, root), "rb").read()
snap = open(f"{HERE}/orig/{tag}.ts", "rb").read()
problems = 0
if live != snap:
    print("PROBLEM live root differs from snapshot"); problems += 1
staged = []
for dp, dn, fn in os.walk(os.path.join(STAGE, tag)):
    for f in fn:
        if f.endswith(".ts"):
            staged.append(os.path.relpath(os.path.join(dp, f), os.path.join(STAGE, tag)))
kids = [s for s in staged if s != root]
for k in kids:
    if os.path.exists(os.path.join(REPO, k)):
        print("PROBLEM child exists", k); problems += 1
    if len(k.encode()) > 240 or len(os.path.join(REPO, k)) > 256:
        print("PROBLEM path too long", k); problems += 1
print("problems", problems, "children", len(kids))
if mode == "apply" and not problems:
    for k in sorted(kids):
        dst = os.path.join(REPO, k)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copyfile(os.path.join(STAGE, tag, k), dst)
    shutil.copyfile(os.path.join(STAGE, tag, root), os.path.join(REPO, root))
    print("installed", len(kids), "children + root")
sys.exit(1 if problems else 0)
