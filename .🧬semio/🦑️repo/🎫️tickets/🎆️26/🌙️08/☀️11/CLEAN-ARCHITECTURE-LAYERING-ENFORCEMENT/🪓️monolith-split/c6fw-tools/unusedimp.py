"""🧹 c6fw private-import usage audit: every non-pub `use child::{..}` name of a split root must occur as a path-position identifier (not after `.`, not a `fn`/`struct` name token) in the root, a sibling child or a descendant of the root. usage: unusedimp.py live <tag>..."""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, HERE + "/../rf8-tools")
from rsitems import lex
from treelib import tree, R
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
USE = re.compile(r"^((?:#\[[^\n]*\]\n)*)(pub(?:\([^)]*\))? )?use (\w+)::(\{[^}]*\}|\w+);$", re.M)
def pos_idents(text):
    toks = [t for t in lex(text) if t[0] != "comment"]
    out = set()
    for i, (k, x, sl, el, si) in enumerate(toks):
        if k != "id":
            continue
        prev = toks[i - 1][1] if i else ""
        if prev == "." or prev in ("fn", "struct", "enum", "trait", "type", "mod", "let", "const", "static", "union"):
            continue
        out.add(x)
    return out
bad = 0
for tag in sys.argv[2:]:
    root = os.path.join(R, ft[tag])
    files = tree(root)
    rt = files[0][1]
    wiring = {}
    for m in USE.finditer(rt):
        vis, mod, names = m.group(2), m.group(3), m.group(4)
        if vis and vis.strip() == "pub":
            continue
        ns = [n.strip() for n in names.strip("{}").split(",")]
        wiring.setdefault(mod, []).extend(ns)
    clean_root = USE.sub("", rt)
    ids = {"root": pos_idents(clean_root)}
    for f, t in files[1:]:
        ids[f] = pos_idents(t)
    unused = []
    for mod, ns in wiring.items():
        child = [f for f in ids if f != "root" and os.path.basename(os.path.dirname(f)) and re.search(mod, f.replace("-", "_").replace("/", "_")) is not None]
        for n in ns:
            used = n in ids["root"]
            for f in ids:
                if f == "root":
                    continue
                if n in ids[f] and not (os.path.basename(os.path.dirname(f)).replace("-", "_") .endswith(mod) or mod in os.path.dirname(f).replace("-", "_")):
                    used = True
            if not used:
                unused.append((mod, n))
    print(f"[{tag}] private imports {sum(len(v) for v in wiring.values())}; possibly unused {len(unused)}")
    for u in unused:
        print("    ", u)
        bad += 1
print("UNUSEDIMP", "OK" if not bad else "REVIEW")
