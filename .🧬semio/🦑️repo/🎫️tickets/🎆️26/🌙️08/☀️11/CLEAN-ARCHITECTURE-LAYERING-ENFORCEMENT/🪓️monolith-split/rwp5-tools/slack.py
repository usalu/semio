# 🛝️ Feasible rank interval of every node (callees at or below, callers at or above) and the lines that could move per direction.
import os, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, deps
def intervals(nodes):
    callers = collections.defaultdict(set)
    for n in nodes:
        for t in n.refs:
            if t is not n: callers[id(t)].add(n)
    out = {}
    for n in nodes:
        lo = max([cfg.RANK[t.crate] for t in n.refs if t is not n] + [0])
        hi = min([cfg.RANK[c.crate] for c in callers[id(n)]] + [len(cfg.CRATES) - 1])
        out[id(n)] = (lo, hi)
    return out
if __name__ == "__main__":
    root = sys.argv[1] if len(sys.argv) > 1 else cfg.NEW
    nodes = deps.load(root); deps.link(nodes); iv = intervals(nodes)
    names = [c for _, c, _ in cfg.CRATES]
    for n in nodes:
        pass
    for c in names:
        mine = [n for n in nodes if n.crate == c]
        down = collections.Counter(); up = collections.Counter()
        for n in mine:
            lo, hi = iv[id(n)]; sz = n.e - n.s + 1
            for r in range(lo, cfg.RANK[c]): down[names[r]] += sz
            for r in range(cfg.RANK[c] + 1, hi + 1): up[names[r]] += sz
        print(c, "nodes", len(mine), "lines", sum(n.e - n.s + 1 for n in mine), "\n   can move down to:", dict(down), "\n   can move up to:", dict(up))
