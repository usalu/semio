"""List imports of the staged split whose every occurrence in the child looks like a binding (param, let, field), i.e. a name collision."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from osmlib import *
from rstok import *
import osmeng, plans, tempfile

def run(names):
    out = []
    for g in names:
        files = plans.GROUPS[g]()
        with tempfile.TemporaryDirectory(dir=W) as td:
            osmeng.run_group(files, td, srcroot=f"{W}/snap-{'services' if g == 'services' else 'mcp'}")
        for f in files:
            for c in f.ctxs:
                toks = tokenize(c.text)
                allnames = [(b, "leaf") for (_, b, _) in c.leaves] + [(n, "sib") for (_, n, _) in c.sibs]
                for nm, kind in allnames:
                    occ = [i for i, t in enumerate(toks) if t[0] == "id" and t[1] == nm]
                    use_like = 0
                    for i in occ:
                        prev = toks[i - 1] if i else ("", "", 0, 0)
                        prev2 = toks[i - 2] if i > 1 else ("", "", 0, 0)
                        nxt = toks[i + 1] if i + 1 < len(toks) else ("", "", 0, 0)
                        nxt2 = toks[i + 2] if i + 2 < len(toks) else ("", "", 0, 0)
                        if prev[1] == "." or (prev[1] == ":" and prev2[1] == ":"):
                            continue
                        binding = (prev[1] in ("let", "mut", "ref", "|") or (nxt[1] == ":" and nxt2[1] != ":") and prev[1] not in ("{",) or (nxt[1] == ":" and nxt2[1] != ":"))
                        if prev[1] in ("let", "mut", "ref") or (nxt[1] == ":" and nxt2[1] != ":"):
                            continue
                        use_like += 1
                    if use_like == 0:
                        out.append((f.label, c.mod, nm, kind, len(occ)))
    return out

if __name__ == "__main__":
    for r in run(sys.argv[1].split(",")):
        print(r)
