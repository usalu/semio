"""📂 C5B new-folder lister: absolute directories and folder names of every child of the given tags (module specs and fragment tables). usage: alldirs.py <tag>... -> newdirs.txt, newnames.txt"""
import importlib, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
dirs, names = [], []
for tag in sys.argv[1:]:
    base = os.path.join(REPO, os.path.dirname(ft[tag]))
    try:
        folders = [c["folder"] for c in importlib.import_module("specs." + tag).SPEC["children"]]
    except ModuleNotFoundError:
        folders = json.load(open(f"{HERE}/staged-{tag}.json", encoding="utf-8"))["folders"]
    for f in folders:
        dirs.append(os.path.join(base, f)); names.append(f)
open(HERE + "/newdirs.txt", "a", encoding="utf-8").write("\n".join(dirs) + "\n")
open(HERE + "/newnames.txt", "a", encoding="utf-8").write("\n".join(names) + "\n")
print(len(dirs), "dirs")
