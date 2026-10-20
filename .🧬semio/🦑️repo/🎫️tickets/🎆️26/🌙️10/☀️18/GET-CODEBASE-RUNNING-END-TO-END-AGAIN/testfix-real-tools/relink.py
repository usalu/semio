#!/usr/bin/env python3
"""Creates overlay node_modules: a real directory of symlinks to the installed store entries (read-only use) whose workspace links are retargeted into the overlay when the overlay holds that path."""
import os, shutil
R = "/Users/ueli/Documents/semio/"
O = os.environ.get("TFR_ROOT", "/private/tmp/testfix-real/r").rstrip("/") + "/"
SRC = os.path.realpath(R + "node_modules")
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
        elif os.path.isdir(s) and name.startswith("@"): link_entries(s, d)
        else: os.symlink(s, d)
shutil.rmtree(O + "node_modules", ignore_errors=True)
link_entries(SRC, O + "node_modules")
print("ok", len(os.listdir(O + "node_modules")))
