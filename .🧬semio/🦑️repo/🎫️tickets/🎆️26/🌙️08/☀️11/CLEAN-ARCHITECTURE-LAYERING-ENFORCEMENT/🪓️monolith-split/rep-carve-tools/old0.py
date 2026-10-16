"""🕸️ Resolves every `crate::` path of the old crate in the old module world (analysis only)."""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *

def crate_paths(text):
    toks = [x for x in tokenize(text) if x[0] not in ("ws", "comment")]
    out = []
    for k, (kd, tx, a, b) in enumerate(toks):
        if kd == "id" and tx == "crate" and k + 2 < len(toks) and toks[k + 1][1] == ":" and toks[k + 2][1] == ":":
            if k >= 1 and toks[k - 1][1] == "$": continue
            segs = []; j = k + 3
            if j < len(toks) and toks[j][1] == "{":
                out.append(("group", a)); continue
            while j < len(toks) and toks[j][0] == "id":
                segs.append(toks[j][1])
                if j + 3 < len(toks) and toks[j + 1][1] == ":" and toks[j + 2][1] == ":" and toks[j + 3][0] == "id":
                    j += 3
                else:
                    break
            out.append((tuple(segs), a))
    return out

if __name__ == "__main__":
    W = World(FACADE_ROOT, staged=False)
    print("nodes", len(W.nodes))
    res = collections.Counter(); unres = collections.Counter()
    for key, node in sorted(W.nodes.items()):
        if node.file != key: continue
        for segs, pos in crate_paths(node.text):
            if segs == "group": continue
            r = W.resolve_path(node, ("crate",) + segs)
            if r is None:
                unres[(key.replace(R, ""), "::".join(segs))] += 1
            elif r[0] == "ext":
                res[("ext", r[1][0])] += 1
            elif r[0] == "def":
                res[("def", r[1].key.replace(R, ""), r[2])] += 1
            else:
                res[("mod", r[1].key.replace(R, ""))] += 1
    for k, n in sorted(res.items(), key=lambda x: str(x[0])): print(n, k)
    print("UNRESOLVED")
    for k, n in sorted(unres.items()): print(n, k)
