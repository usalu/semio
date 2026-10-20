#!/usr/bin/env python3
"""Turns every node_modules symlink of the overlay into a real directory whose workspace links point into the overlay."""
import os, sys
R = "/Users/ueli/Documents/semio/"
O = "/private/tmp/testfix-law/"
def retarget(path):
    if path.startswith(R) and not path.startswith(R + ".cache/") and os.path.lexists(O + path[len(R):]): return O + path[len(R):]
    return path
def link_entries(src, dst):
    os.makedirs(dst, exist_ok=True)
    for name in os.listdir(src):
        s = os.path.join(src, name); d = os.path.join(dst, name)
        if os.path.lexists(d): continue
        if os.path.islink(s):
            t = os.readlink(s)
            if not os.path.isabs(t): t = os.path.normpath(os.path.join(src, t))
            os.symlink(retarget(t), d)
        elif os.path.isdir(s) and name.startswith("@"):
            link_entries(s, d)
        else:
            os.symlink(s, d)
def convert(p):
    if not os.path.islink(p): return False
    t = os.readlink(p)
    os.remove(p)
    if os.path.isdir(t): link_entries(t, p)
    else: os.makedirs(p)
    return True
n = 0
if convert(O + "node_modules"): n += 1
for dp, dns, fns in os.walk(O, followlinks=False):
    dns[:] = [d for d in dns if d not in ("node_modules", ".orig", ".logs", ".art", ".git")]
    for d in list(os.listdir(dp)):
        if d == "node_modules" and os.path.islink(os.path.join(dp, d)):
            n += convert(os.path.join(dp, d))
print("converted", n)
