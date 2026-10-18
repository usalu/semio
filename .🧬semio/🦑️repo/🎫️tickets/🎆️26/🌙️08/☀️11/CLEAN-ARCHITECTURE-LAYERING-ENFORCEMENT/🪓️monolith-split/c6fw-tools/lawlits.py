"""📜 c6fw law-literal impact: string literals of the given TS files that occur in the ORIGINAL text of a split root, not in its new root, and where they live now. usage: lawlits.py <tag> <ts file>..."""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from treelib import R, tree
import importlib.util
spec = importlib.util.spec_from_file_location("r3", HERE + "/readers3.py")
src = open(HERE + "/readers3.py", encoding="utf-8").read()
ns = {}
exec(src[src.index("def js_strings"):src.index("def unesc")], ns)
js_strings = ns["js_strings"]
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
tag = sys.argv[1]
orig = open(f"{HERE}/orig/{tag}.rs", encoding="utf-8").read()
files = tree(os.path.join(R, ft[tag]))
root = files[0][1]
for p in sys.argv[2:]:
    t = open(p, encoding="utf-8", errors="ignore").read()
    seen = set()
    n = 0
    for s in js_strings(t):
        if len(s) < 8 or s in seen:
            continue
        seen.add(s)
        if s in orig and s not in root:
            where = [os.path.basename(os.path.dirname(f)) for f, x in files[1:] if s in x]
            print("  ", os.path.basename(os.path.dirname(p))[:20], repr(s[:90]), "->", where[:2])
            n += 1
    print(p[-70:], "moved:", n)
