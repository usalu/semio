#!/usr/bin/env python3
"""usage: error-files.py <log…>  (prints the distinct absolute source files named by compiler errors in the logs, one per line, normalized, existing only)"""
import os, re, sys
root = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 7))
rx = re.compile(r'^\s*(?:-->\s*)?(\S+?\.(?:rs|ts|toml)):\d+:\d+(?::\s*error|$)|^\s*-->\s*(\S+?\.(?:rs|toml)):\d+:\d+')
out = set()
for log in sys.argv[1:]:
    ctx = False
    for line in open(log, encoding="utf8", errors="replace"):
        if line.startswith("error[") or line.startswith("error:"):
            ctx = True; continue
        m = re.match(r'^\s*-->\s*(\S+?)\.?(?::\d+:\d+)?$', line.rstrip()) if ctx else None
        m2 = re.match(r'^(\S+?\.rs):\d+:\d+: error', line) 
        path = None
        if m: path = re.sub(r':\d+:\d+$', '', m.group(1))
        elif m2: path = m2.group(1)
        if path:
            ctx = False
            p = os.path.normpath(path if os.path.isabs(path) else os.path.join(root, path))
            if os.path.exists(p): out.add(p)
for p in sorted(out): print(p)
