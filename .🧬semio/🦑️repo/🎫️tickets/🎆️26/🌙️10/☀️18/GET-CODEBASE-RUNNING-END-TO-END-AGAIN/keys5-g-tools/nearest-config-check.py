# keys5-g: is the module-resolution rule (resolutionConfigsV1) effective on a key? For every module of a key list (one repository-relative path per line) the nearest package.json above it must be in the list.
# usage: python3 -I nearest-config-check.py <repo root> <key list>      prints "<modules> <missing configs>" and the first 20 misses
import os, re, sys
root, listing = sys.argv[1], sys.argv[2]
key = set(l for l in open(listing, encoding="utf-8").read().split("\n") if l)
missing = {}
for f in key:
    if not re.search(r"\.(ts|tsx|mts|cts|js|jsx|mjs|cjs)$", f): continue
    d = os.path.dirname(f)
    while d:
        c = d + "/package.json"
        if os.path.exists(os.path.join(root, c)):
            if c not in key: missing.setdefault(c, []).append(f)
            break
        d = os.path.dirname(d)
print(sum(1 for f in key if re.search(r"\.(ts|tsx|mts|cts|js|jsx|mjs|cjs)$", f)), len(missing))
for k, v in list(missing.items())[:20]: print(k, len(v), v[0])
