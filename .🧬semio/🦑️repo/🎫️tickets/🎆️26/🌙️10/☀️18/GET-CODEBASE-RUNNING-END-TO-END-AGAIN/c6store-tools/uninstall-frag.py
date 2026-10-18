"""↩️ C6STORE uninstall of an include-fragment split: restores the snapshot root and removes the fragment folders (live files must equal the staged ones). usage: uninstall-frag.py <tag>"""
import json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
STAGE = "/Users/ueli/.cache/c6store/stage-frag"
tag = sys.argv[1]
st = json.load(open(f"{HERE}/staged-{tag}.json", encoding="utf-8"))
rel = st["root"]
bdir = os.path.dirname(rel)
for f in st["folders"]:
    live = os.path.join(REPO, bdir, f, "🦀️.rs")
    assert open(live, "rb").read() == open(os.path.join(STAGE, bdir, f, "🦀️.rs"), "rb").read(), ("live fragment differs from staged", f)
assert open(os.path.join(REPO, rel), "rb").read() == open(os.path.join(STAGE, rel), "rb").read(), "live root differs from staged"
for f in st["folders"]:
    shutil.rmtree(os.path.join(REPO, bdir, f))
    shutil.rmtree(os.path.join(STAGE, bdir, f))
shutil.copyfile(f"{HERE}/orig/{tag}.rs", os.path.join(REPO, rel))
print("uninstalled", tag, len(st["folders"]))
