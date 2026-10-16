"""🔎 Counts `crate::` path heads and `super` globs per mounted file (analysis only)."""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import tokenize
SP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/rep4"
BASE = SP + "/base/"
R = "🧰️framework/🔨️modules/📡️replication/"
PATH_RX = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]|include!\(\s*"([^"]*\.rs)"\s*\)')

def crawl():
    seen = {}
    stack = [R + "📦️packages/🦀️rust/🦀️.rs"]
    while stack:
        rel = stack.pop()
        if rel in seen: continue
        t = open(BASE + rel, encoding="utf8").read()
        seen[rel] = t
        for m in PATH_RX.finditer(t):
            raw = m.group(1) if m.group(1) is not None else m.group(2)
            if not raw: continue
            p = os.path.normpath(os.path.join(os.path.dirname(rel), raw))
            if p.endswith(".rs") and os.path.exists(BASE + p): stack.append(p)
    return seen

if __name__ == "__main__":
    files = crawl()
    heads = collections.Counter()
    perfile = collections.defaultdict(collections.Counter)
    for rel, t in sorted(files.items()):
        toks = [x for x in tokenize(t) if x[0] not in ("ws", "comment")]
        for k, (kd, tx, a, b) in enumerate(toks):
            if kd == "id" and tx == "crate" and k + 2 < len(toks) and toks[k + 1][1] == ":" and toks[k + 2][1] == ":":
                seg = toks[k + 3][1] if k + 3 < len(toks) else "?"
                if seg == "{":
                    seg = "{...}"
                heads[seg] += 1
                perfile[rel.replace(R, "")][seg] += 1
    for s, n in heads.most_common(): print(n, s)
    print("---")
    for f, c in perfile.items(): print(f, dict(c))
