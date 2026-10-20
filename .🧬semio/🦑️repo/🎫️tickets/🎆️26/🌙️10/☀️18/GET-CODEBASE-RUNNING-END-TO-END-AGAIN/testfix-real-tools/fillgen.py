#!/usr/bin/env python3
"""Copies git-ignored generated inputs that esbuild could not resolve from the live checkout into BOTH overlays (never staged). usage: fillgen.py LOG"""
import os, re, shutil, sys
R = "/Users/ueli/Documents/semio/"
n = 0
for m in re.finditer(r'^(\S[^\n]*?):\d+:\d+: ERROR: Could not resolve "([^"]+)"', open(sys.argv[1], encoding="utf8").read(), re.M):
    imp, spec = m.group(1), m.group(2)
    if not spec.startswith("."): continue
    tgt = os.path.normpath(os.path.join(os.path.dirname(imp), spec))
    if os.path.isfile(R + tgt):
        for o in ("r", "r0"):
            d = "/private/tmp/testfix-real/%s/%s" % (o, tgt)
            if not os.path.exists(d):
                os.makedirs(os.path.dirname(d), exist_ok=True); shutil.copyfile(R + tgt, d); n += 1
    else: print("not in live:", tgt[-110:])
print("copied", n)
