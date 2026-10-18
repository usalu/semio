import sys,os,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from plan import *
def pubnames(key, mods=None):
    rootp = os.path.join(find(roots[key]), "🦀️.rs")
    t = tree(rootp)
    res = {}
    text = read(rootp)
    order = [m.group(1) for m in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+);', text, re.M)]
    for m in order:
        if (m,) not in t or (mods and m not in mods): continue
        for it in top_items(t[(m,)][1]):
            if it["vis"] == "pub" and it["kind"] != "use" or it["vis"] == "pub" and it["name"] != "*":
                res.setdefault(m, {}).setdefault(tuple(it["cfg"]), set()).add(it["name"])
    return res
if __name__ == "__main__":
    r = pubnames(sys.argv[1])
    for m, g in r.items():
        for cfg, ns in g.items(): print(m, cfg, len(ns))
