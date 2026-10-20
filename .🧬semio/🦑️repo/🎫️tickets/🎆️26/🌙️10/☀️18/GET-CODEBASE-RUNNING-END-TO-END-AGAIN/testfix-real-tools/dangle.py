#!/usr/bin/env python3
"""Reports every quoted path token that mentions 🧫️fixtures and resolves to nothing (against the file dir, each ancestor and each Cargo.toml / package.json dir of the same module) in an overlay root. usage: dangle.py ROOT OUTJSON ; diff two outputs to find references broken by a move."""
import json, os, re, sys
root = sys.argv[1].rstrip("/") + "/"
files = set()
for line in open("/private/tmp/testfix-real/gg-fixtures.txt", encoding="utf8"):
    m = re.match(r"^(.*?):\d+:", line)
    if m: files.add(m.group(1))
extra = [l.strip() for l in open("/private/tmp/testfix-real/extra-readers.txt", encoding="utf8")] if os.path.exists("/private/tmp/testfix-real/extra-readers.txt") else []
files |= set(extra)
tok = re.compile(r'"([^"\n]{0,400}?🧫️fixtures[^"\n]{0,300}?)"|\'([^\'\n]{0,400}?🧫️fixtures[^\'\n]{0,300}?)\'|`([^`\n]{0,400}?🧫️fixtures[^`\n]{0,300}?)`')
bases_cache = {}
def manifests(f):
    parts = f.split("/")
    out = []
    for k in range(3, len(parts)):
        d = "/".join(parts[:k])
        if d not in bases_cache:
            c = []
            try:
                for e in os.listdir(root + d):
                    p = root + d + "/" + e
                    if os.path.isdir(p) and e in ("📦️packages",):
                        for sub in os.listdir(p):
                            if os.path.exists(p + "/" + sub + "/Cargo.toml") or os.path.exists(p + "/" + sub + "/package.json"): c.append(d + "/📦️packages/" + sub)
                if os.path.exists(root + d + "/Cargo.toml") or os.path.exists(root + d + "/package.json"): c.append(d)
            except OSError: pass
            bases_cache[d] = c
        out += bases_cache[d]
    return out
bad = []
for f in sorted(files):
    p = root + f
    if not os.path.isfile(p): continue
    try: text = open(p, encoding="utf8").read()
    except Exception: continue
    if len(text) > 3_000_000: continue
    for m in tok.finditer(text):
        t = next(x for x in m.groups() if x)
        if any(c in t for c in ("${", "{}", "{path}", "<", "*")): continue
        cands = set()
        for b in [os.path.dirname(f)] + ["/".join(f.split("/")[:k]) for k in range(1, len(f.split("/")))] + manifests(f):
            for s in (t, t.lstrip("/")):
                cands.add(os.path.normpath(os.path.join(b, s)))
        for s in (t, t.lstrip("/")): cands.add(os.path.normpath(s))
        if not any(os.path.exists(root + c) for c in cands):
            bad.append([f, t])
json.dump(bad, open(sys.argv[2], "w"), ensure_ascii=False)
print(len(bad), "unresolved tokens in", len(files), "files")
