#!/usr/bin/env python3
# 🗺️ Monolith module name -> source file, from the root and the seven mount hubs (live tree).
import os, re, glob
from lib import *
from rs import blank

MOUNT = re.compile(r"((?:#\[[^\]]*\]\s*)*)(?:(pub(?:\([^)]*\))?)\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;")
PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')


def mounts_of(path):
    text = read(path)
    clean = blank(text)
    here = os.path.dirname(path)
    out = []
    for m in MOUNT.finditer(clean):
        attrs = text[m.start(1):m.end(1)]
        pm = PATH_ATTR.search(attrs)
        if not pm:
            continue
        out.append((m.group(3), os.path.normpath(os.path.join(here, pm.group(1))), (m.group(2) or ""), " ".join(attrs.split())))
    return out


def all_mounts(base=None):
    """module name -> (live file, vis, attrs, hub). base=snapshot plugin dir reads the pre-WP1 hubs; paths are translated to the live tree."""
    src = base or PLUG
    res = {}
    for hub in sorted(glob.glob(src + "/🪝️mounts/*/🦀️.rs")):
        for name, f, vis, attrs in mounts_of(hub):
            res[name] = (f.replace(src, PLUG), vis, attrs, os.path.basename(os.path.dirname(hub)))
    for name, f, vis, attrs in mounts_of(src + "/🦀️.rs"):
        res[name] = (f.replace(src, PLUG), vis, attrs, "root")
    return res


def snapshot_mounts():
    return all_mounts(SNAP + "/" + PLUG_REL)

if __name__ == "__main__":
    for k, (f, vis, attrs, hub) in sorted(all_mounts().items()):
        print(f"{k:42s} {hub:22s} {f.replace(PLUG + '/', '')}")
