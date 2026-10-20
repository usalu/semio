#!/usr/bin/env python3
"""Independent oracle for the stock-Nx fixture: recomputes every out.txt (sha256 of the project's files except out.txt) and counts mismatches. usage: verify-fixture-outputs.py DIR [GLOB]"""
import glob, hashlib, os, sys
root = sys.argv[1]
pattern = sys.argv[2] if len(sys.argv) > 2 else "p*"
bad = total = 0
for d in sorted(glob.glob(os.path.join(root, pattern))):
    if not os.path.exists(os.path.join(d, "out.txt")):
        continue
    names = sorted(f for f in os.listdir(d) if f != "out.txt")
    expected = hashlib.sha256(b"".join(open(os.path.join(d, f), "rb").read() for f in names)).hexdigest() + "\n"
    total += 1
    bad += expected != open(os.path.join(d, "out.txt")).read()
print("outputs checked %d, mismatches %d" % (total, bad))
