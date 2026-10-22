#!/usr/bin/env python3
"""dev_rewrite.py <live-path-of-origin> <in> <out>: rewrite ./ and ../ module specifiers of <in> to absolute paths relative to the directory of <live-path-of-origin>."""
import os, re, sys
origin, src, dst = sys.argv[1:4]
base = os.path.dirname(os.path.abspath(origin))
t = open(src, encoding="utf8").read()
t = re.sub(r'(\bfrom\s*|\bimport\s*\(\s*|\bimport\s+)(["\'])(\.{1,2}/[^"\']+)\2', lambda m: m.group(1)+m.group(2)+os.path.normpath(os.path.join(base, m.group(3)))+m.group(2), t)
open(dst, "w", encoding="utf8").write(t)
