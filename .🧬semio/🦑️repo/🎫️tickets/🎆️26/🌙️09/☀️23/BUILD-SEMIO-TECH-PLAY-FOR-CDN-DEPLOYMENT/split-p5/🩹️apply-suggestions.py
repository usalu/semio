#!/usr/bin/env python3
"""🩹️ Applies rustc suggestions from a `--message-format=json` cargo log to files under the given roots, for the
error codes named (default: every error with a suggestion). One suggestion per primary span; overlapping edits skipped.

usage: 🩹️apply-suggestions.py <cargo.json.log> <codes,comma|all> <root>...   (lint names such as unused_imports select warnings)
"""
import json, os, sys, collections
REPO = "/Users/ueli/Documents/semio"
log, codes = sys.argv[1], sys.argv[2]
roots = [os.path.join(REPO, r) for r in sys.argv[3:]]
want = None if codes == "all" else set(codes.split(","))
edits = collections.defaultdict(list)
seen = set()
for line in open(log, encoding="utf-8", errors="replace"):
    if not line.startswith("{"): continue
    try: msg = json.loads(line)
    except Exception: continue
    if msg.get("reason") != "compiler-message": continue
    d = msg["message"]
    level = "warning" if (want and any(c.startswith("unused") or c.startswith("clippy") for c in want)) else "error"
    if d["level"] != level: continue
    code = (d.get("code") or {}).get("code")
    if want is not None and code not in want: continue
    def walk(diag):
        for c in diag.get("children", []):
            for sp in c.get("spans", []):
                if sp.get("suggested_replacement") is not None:
                    yield sp
            yield from walk(c)
    for sp in walk(d):
        fname = sp["file_name"]
        path = fname if os.path.isabs(fname) else os.path.normpath(os.path.join(REPO, "✏️s", fname))
        if not os.path.exists(path):
            for base in [REPO, os.path.join(REPO, "✏️s/🔌️plugins/🏗️fem"), os.path.join(REPO, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d"), os.path.join(REPO, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"), os.path.join(REPO, "🌎️hub")]:
                q = os.path.normpath(os.path.join(base, fname))
                if os.path.exists(q): path = q; break
        path = os.path.realpath(path)
        if not any(path.startswith(os.path.realpath(r)) for r in roots): continue
        key = (path, sp["byte_start"], sp["byte_end"], sp["suggested_replacement"])
        if key in seen: continue
        seen.add(key)
        edits[path].append((sp["byte_start"], sp["byte_end"], sp["suggested_replacement"]))
total = 0
for path, es in edits.items():
    data = open(path, "rb").read()
    es.sort()
    out, last = [], 0
    for a, b, r in es:
        if a < last: continue
        out.append(data[last:a]); out.append(r.encode("utf-8")); last = b; total += 1
    out.append(data[last:])
    open(path, "wb").write(b"".join(out))
    print(len(es), os.path.relpath(path, REPO))
print("applied", total)
