#!/usr/bin/env python3
"""usage: compare-trees.py A B [SAMPLE] -- walk both trees (skip .git and .cache/f), compare path sets, then sha256 + exec bit of SAMPLE random common files; prints a JSON summary."""
import hashlib, json, os, random, sys, unicodedata

def walk(root):
    files = {}
    for base, dirs, names in os.walk(root):
        rel = os.path.relpath(base, root)
        rel = "" if rel == "." else rel
        dirs[:] = [d for d in dirs if not (rel == "" and d == ".git") and not (rel == ".cache" and d == "f")]
        for n in names:
            p = os.path.join(rel, n) if rel else n
            files[unicodedata.normalize("NFC", p)] = os.path.join(base, n)
    return files

def digest(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for c in iter(lambda: f.read(1 << 20), b""):
            h.update(c)
    return h.hexdigest()

a, b = sys.argv[1], sys.argv[2]
sample = int(sys.argv[3]) if len(sys.argv) > 3 else 5000
fa, fb = walk(a), walk(b)
only_a = sorted(set(fa) - set(fb)); only_b = sorted(set(fb) - set(fa))
common = sorted(set(fa) & set(fb))
random.seed(7)
picked = random.sample(common, min(sample, len(common)))
bad = []
for p in picked:
    sa, sb = os.lstat(fa[p]), os.lstat(fb[p])
    if os.path.islink(fa[p]) != os.path.islink(fb[p]) or (not os.path.islink(fa[p]) and (digest(fa[p]) != digest(fb[p]) or (sa.st_mode & 0o111 != 0) != (sb.st_mode & 0o111 != 0))):
        bad.append(p)
print(json.dumps({"filesA": len(fa), "filesB": len(fb), "onlyA": len(only_a), "onlyB": len(only_b), "onlyA_sample": only_a[:10], "onlyB_sample": only_b[:10], "common": len(common), "sampled": len(picked), "mismatch": len(bad), "mismatch_sample": bad[:10]}, ensure_ascii=False))
