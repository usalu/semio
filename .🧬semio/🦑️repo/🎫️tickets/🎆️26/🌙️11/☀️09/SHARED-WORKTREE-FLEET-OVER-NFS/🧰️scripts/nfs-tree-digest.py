#!/usr/bin/env python3
"""usage: nfs-tree-digest.py DIR [--json] -- "COUNT MD5" of the sorted {relative name: sha256} of the regular files below DIR (with --json the map itself)."""
import hashlib, json, os, sys

root = sys.argv[1]; out = {}
for base, _, names in os.walk(root):
    for n in names:
        p = os.path.join(base, n)
        try: out[os.path.relpath(p, root)] = hashlib.sha256(open(p, "rb").read()).hexdigest()
        except OSError: pass
text = json.dumps(out, sort_keys=True)
print(text if "--json" in sys.argv else "%d %s" % (len(out), hashlib.md5(text.encode()).hexdigest()))
