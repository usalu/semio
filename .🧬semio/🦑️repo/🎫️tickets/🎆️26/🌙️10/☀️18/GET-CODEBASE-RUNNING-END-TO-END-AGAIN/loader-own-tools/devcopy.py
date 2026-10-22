#!/usr/bin/env python3
"""devcopy.py <live-file> <dev-dir>: copy with relative import specifiers made absolute (static import/export-from, import(), createRequire literal, new URL(..., import.meta.url) left alone)."""
import os, re, sys
src = os.path.abspath(sys.argv[1]); dev = sys.argv[2]
text = open(src, encoding="utf8").read()
base = os.path.dirname(src)
def fix(m):
    pre, q, spec, post = m.group(1), m.group(2), m.group(3), m.group(4)
    return pre + q + os.path.normpath(os.path.join(base, spec)) + q + post
text = re.sub(r'(\bfrom\s*|\bimport\s*\(\s*|\bimport\s+)(["\'])(\.{1,2}/[^"\']+)\2(?=)', lambda m: m.group(1)+m.group(2)+os.path.normpath(os.path.join(base,m.group(3)))+m.group(2), text)
os.makedirs(dev, exist_ok=True)
out = os.path.join(dev, os.path.basename(src))
open(out, "w", encoding="utf8").write(text)
print(out)
