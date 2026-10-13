#!/usr/bin/env python3
"""🔍️ Lists every definition of the shared codec helpers across the semio subsets with a normalized body hash."""
import os, re, sys, hashlib, collections
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
NAMES = sys.argv[1].split(",")
SKIP = ("🏭️generator", "🔬️probes", "🏭️bridge", "🔮️oracles")

def fn_body(src, start):
    i = src.index("{", start)
    depth = 0
    for j in range(i, len(src)):
        c = src[j]
        if c == "{": depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0: return src[start:j + 1]
    return src[start:]

out = collections.defaultdict(list)
for root, _, files in os.walk(ROOT):
    if any(s in root for s in SKIP): continue
    for f in files:
        if not f.endswith(".rs"): continue
        p = os.path.join(root, f); s = open(p).read()
        for m in re.finditer(r"(?m)^[ \t]*(pub(?:\([a-z]+\))? )?fn (" + "|".join(map(re.escape, NAMES)) + r")\b", s):
            body = fn_body(s, m.start())
            norm = re.sub(r"\s+", "", re.sub(r"//[^\n]*", "", body))
            out[m.group(2)].append((hashlib.sha1(norm.encode()).hexdigest()[:8], os.path.relpath(p, ROOT), s[:m.start()].count("\n") + 1, (m.group(1) or "").strip()))
for n in NAMES:
    for h, p, l, vis in out.get(n, []): print(f"{n:22} {h} {vis:10} {p}:{l}")
