"""🔎 C5D pre-install sibling check: every new child folder of a tag has an emoji that no existing sibling folder (other than the new ones) uses, new emojis are unique among themselves, no new folder exists yet, and names are valid. usage: sibcheck.py <tag>... (module specs or frag tables)"""
import importlib, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
REPO = "/Users/ueli/Documents/semio"
files_tsv = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
bad = 0
for tag in sys.argv[1:]:
    rel = files_tsv[tag]
    bdir = os.path.join(REPO, os.path.dirname(rel))
    try:
        from specs.lib import folders_of
        folders = folders_of(importlib.import_module("specs." + tag).SPEC)
    except ModuleNotFoundError:
        from specs5 import FRAG
        folders = [r[1] + "️" + r[2] for r in FRAG[tag]["rows"]]
    emos = {}
    for f in folders:
        e = f[: f.index("️") + 1]
        if e in emos:
            print(tag, "DUP emoji among new", e); bad += 1
        emos[e] = f
        if os.path.exists(os.path.join(bdir, f)):
            print(tag, "EXISTS", f); bad += 1
    for e in os.listdir(bdir):
        if not os.path.isdir(os.path.join(bdir, e)) or e in folders:
            continue
        if "️" in e and e[: e.index("️") + 1] in emos:
            print(tag, "SIBLING CLASH", e, "vs", emos[e[: e.index("️") + 1]]); bad += 1
    print(tag, "folders", len(folders), "longest", max(len((os.path.relpath(os.path.join(bdir, f), REPO) + "/🦀️.rs").encode()) for f in folders), "bytes")
print("SIBCHECK", "OK" if not bad else "FAILED")
sys.exit(1 if bad else 0)
