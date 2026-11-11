#!/usr/bin/env python3
"""usage: [CONVERGE_SAMPLE=K] fleet-converge.py ORIGINS.json [--quick] -- with CONVERGE_SAMPLE only work/ and 1 in K other files are hashed; compares the trees of all origins (skip .git and .cache/f): path sets, sizes, exec bits and (without --quick) sha256 of every file against the first tree; prints one JSON summary. Hash results are cached per (path, size, mtime_ns, inode) in $R/converge-cache.* between calls."""
import hashlib, json, os, pickle, sys
from concurrent.futures import ProcessPoolExecutor
import fleet

def listing(o):
    root = fleet.tree(o); out = {}
    stack = [""]
    while stack:
        rel = stack.pop()
        try: it = list(os.scandir(os.path.join(root, rel) if rel else root))
        except OSError: continue
        for e in it:
            q = f"{rel}/{e.name}" if rel else e.name
            if rel == ".cache" and e.name == "f" or q == ".git": continue
            try:
                if e.is_dir(follow_symlinks=False): stack.append(q)
                else:
                    st = e.stat(follow_symlinks=False); out[q] = (st.st_size, st.st_mtime_ns, st.st_ino, st.st_mode & 0o111 != 0, e.is_symlink())
            except OSError: pass
    return out

SAMPLE = int(os.environ.get("CONVERGE_SAMPLE", "0"))

def wanted(q):
    return not SAMPLE or q.startswith("work/") or int(hashlib.md5(q.encode()).hexdigest()[:6], 16) % SAMPLE == 0

def digest(args):
    o, quick = args
    cache_file = f"{fleet.R}/converge-cache.{o}"
    try: cache = pickle.load(open(cache_file, "rb"))
    except Exception: cache = {}
    cur = listing(o); res = {}
    for q, (size, mt, ino, ex, link) in cur.items():
        if quick or link or not wanted(q): res[q] = (size, ex, None); continue
        old = cache.get(q)
        if old and old[0] == (size, mt, ino): res[q] = (size, ex, old[1]); continue
        h = hashlib.sha256()
        try:
            with open(f"{fleet.tree(o)}/{q}", "rb") as f:
                for c in iter(lambda: f.read(1 << 20), b""): h.update(c)
        except OSError: continue
        res[q] = (size, ex, h.hexdigest())
    if not quick: pickle.dump({q: ((v[0], cur[q][1], cur[q][2]), v[2]) for q, v in res.items() if v[2]}, open(cache_file, "wb"))
    return o, res

if __name__ == "__main__":
    names = json.load(open(sys.argv[1])); quick = "--quick" in sys.argv
    with ProcessPoolExecutor(8) as ex: results = dict(ex.map(digest, [(o, quick) for o in names]))
    ref_name = names[0]; ref = results[ref_name]; bad = {}
    for o in names[1:]:
        r = results[o]; only_a = [q for q in ref if q not in r]; only_b = [q for q in r if q not in ref]
        diff = [q for q in ref if q in r and ref[q] != r[q]]
        if only_a or only_b or diff: bad[o] = {"missing": len(only_a), "extra": len(only_b), "different": len(diff), "sample": (only_a[:3], only_b[:3], diff[:3])}
    print(json.dumps({"trees": len(names), "files": len(ref), "identical": not bad, "bad_trees": len(bad), "bad": dict(list(bad.items())[:8])}, ensure_ascii=False))
    sys.exit(0 if not bad else 1)
