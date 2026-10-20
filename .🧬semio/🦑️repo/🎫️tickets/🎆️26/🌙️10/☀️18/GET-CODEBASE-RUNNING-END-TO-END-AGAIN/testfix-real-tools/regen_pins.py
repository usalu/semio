#!/usr/bin/env python3
"""Refreshes feature-locks.json thirdpartyPinsSha256 for locks whose registry pin set changed, only when no surviving package changed version or checksum (pure additions/removals)."""
from uregen import *
import json, hashlib, subprocess, re
R = "/Users/ueli/Documents/semio"
fl = load("📋️feature-locks.json")
def pins(text):
    out = []
    for blk in text.split("[[package]]\n")[1:]:
        n = re.search(r'^name = "([^"]+)"', blk, re.M); v = re.search(r'^version = "([^"]+)"', blk, re.M); s = re.search(r'^source = "([^"]+)"', blk, re.M); c = re.search(r'^checksum = "([^"]+)"', blk, re.M)
        if s: out.append([n.group(1), v.group(1), s.group(1), c.group(1) if c else None])
    return out
def sh(l): return hashlib.sha256(json.dumps(l, separators=(",", ":"), ensure_ascii=False).encode("utf8")).hexdigest()
changed = 0
for row in fl:
    cur = pins(open(O + "/" + row["path"], encoding="utf8").read())
    if sh(cur) == row["thirdpartyPinsSha256"]: continue
    log = subprocess.run(["git", "log", "--format=%h", "-n", "40", "--", row["path"]], cwd=R, capture_output=True, text=True).stdout.split()
    old = None
    for c in log:
        t = subprocess.run(["git", "show", c + ":" + row["path"]], cwd=R, capture_output=True, text=True).stdout
        if t and sh(pins(t)) == row["thirdpartyPinsSha256"]: old = pins(t); break
    if old is None: raise SystemExit("baseline pins not found in history: " + row["path"])
    o = {(a[0], a[1], a[3]) for a in old}; n = {(a[0], a[1], a[3]) for a in cur}
    ch = {a[0] for a in (o - n)} & {a[0] for a in (n - o)}
    if ch: raise SystemExit("version/checksum change for %s in %s" % (sorted(ch)[:5], row["path"]))
    row["thirdpartyPinsSha256"] = sh(cur); changed += 1
save("📋️feature-locks.json", fl)
print("refreshed", changed)
