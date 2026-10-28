#!/usr/bin/env python3
"""🔎 [sealprep] Lists every staged batch in the ticket whose manifest names a file of the given repo-relative path prefixes, with its state against a root (base / applied / drifted / missing). usage: scan_manifests.py <root> <prefix>... (stdlib only, streaming, no recursion into 🗑️generated)"""
import hashlib, json, os, sys
root = os.path.abspath(sys.argv[1]); prefixes = sys.argv[2:]
here = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
def sha(path):
    if not os.path.isfile(path): return None
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""): h.update(chunk)
    return h.hexdigest()
live = {}
for dirpath, dirs, files in os.walk(here):
    dirs[:] = [d for d in dirs if d != "🗑️generated"]
    if "manifest.json" not in files: continue
    path = os.path.join(dirpath, "manifest.json")
    try: rows = json.load(open(path, encoding="utf8"))
    except Exception: continue
    if not isinstance(rows, dict): continue
    for rel, row in rows.items():
        if not isinstance(row, dict) or not any(rel.startswith(p) or p in rel for p in prefixes): continue
        digest = sha(os.path.join(root, rel)); new = row.get("new"); base = row.get("base")
        state = "missing" if digest is None else "applied" if digest == new else "base" if digest == base else "drifted"
        print("%-8s %s :: %s" % (state, os.path.relpath(dirpath, here), rel[-70:]))
