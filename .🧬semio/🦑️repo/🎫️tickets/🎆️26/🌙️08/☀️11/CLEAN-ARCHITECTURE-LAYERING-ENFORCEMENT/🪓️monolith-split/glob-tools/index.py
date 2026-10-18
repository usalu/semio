import sys, os, re, pickle
from multiprocessing import Pool
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *
from roots import *
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
C = set()
for k, v in roots.items():
    d = find(v); t = tree(os.path.join(d, "🦀️.rs"))
    root = strip(t[()][1])
    for g in re.finditer(r'^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+(\w+)::\*;', root, re.M):
        if (g.group(1),) in t:
            for i in top_items(t[(g.group(1),)][1]):
                if i["vis"] and i["name"] != "*": C.add(i["name"])
IDENT = re.compile(r"[A-Za-z_]\w*")
def work(p):
    try:
        t = open(os.path.join(R, p), encoding="utf-8", errors="ignore").read()
    except Exception:
        return p, set()
    return p, set(IDENT.findall(t)) & C
if __name__ == "__main__":
    files = [l.strip() for l in open(S + "/rsfiles2.txt") if l.strip()]
    with Pool(8) as pool:
        res = dict(pool.imap_unordered(work, files, chunksize=200))
    pickle.dump((sorted(C), res), open(S + "/index.pkl", "wb"))
    print(len(C), len(res))
