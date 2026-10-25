"""Ticket input script: weighs the chunks of a built release site (raw and gzip bytes) and marks the ones the document runs.

  measure.py <site directory>        e.g. .cache/b/p/m/vite/release/<sha256(owner)[:16]>/out
"""
import glob
import gzip
import os
import re
import sys

root = sys.argv[1]
html = open(os.path.join(root, "index.html"), encoding="utf8").read()
tags = [(m.group(1), 'fetchpriority="low"' in m.group(0)) for m in re.finditer(r'<(?:script|link)\b[^>]*\s(?:src|href)="/(assets/[^"]+)"[^>]*>', html)]
charged = {p for p, hinted in tags if not hinted}
hinted = {p for p, pre in tags if pre}
total = 0
for path in sorted(glob.glob(os.path.join(root, "assets", "*"))):
    data = open(path, "rb").read()
    rel = os.path.relpath(path, root)
    gz = len(gzip.compress(data))
    mark = "CHARGED" if rel in charged else "low-prio" if rel in hinted else "lazy"
    print(f"{mark:9} {len(data):9} {gz:8}  {rel}")
    if rel in charged and rel.endswith(".js"):
        total += gz
print(f"scripts charged (gzip): {total}")
