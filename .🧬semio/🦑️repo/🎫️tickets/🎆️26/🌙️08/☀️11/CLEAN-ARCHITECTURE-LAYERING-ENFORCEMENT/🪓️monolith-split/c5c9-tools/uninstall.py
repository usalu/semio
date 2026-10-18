"""↩️ C5C roll back of my OWN installed batch: removes the child folders of the given tags (each must hold exactly one 🦀️.rs that I installed) and restores the root from its pre-edit snapshot. usage: uninstall.py <tag>..."""
import importlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
REPO = "/Users/ueli/Documents/semio/"
files_tsv = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
for tag in sys.argv[1:]:
    rel = files_tsv[tag]
    bdir = REPO + os.path.dirname(rel)
    try:
        from specs.lib import folders_of
        folders = folders_of(importlib.import_module("specs." + tag).SPEC)
    except ModuleNotFoundError:
        folders = json.load(open(f"{HERE}/staged-{tag}.json", encoding="utf-8"))["folders"]
    for f in folders:
        d = bdir + "/" + f
        if not os.path.isdir(d):
            continue
        assert os.listdir(d) == ["🦀️.rs"], (d, os.listdir(d))
        shutil.rmtree(d)
    shutil.copyfile(f"{HERE}/orig/{tag}.rs", REPO + rel)
    print("rolled back", tag, len(folders), "folders")
