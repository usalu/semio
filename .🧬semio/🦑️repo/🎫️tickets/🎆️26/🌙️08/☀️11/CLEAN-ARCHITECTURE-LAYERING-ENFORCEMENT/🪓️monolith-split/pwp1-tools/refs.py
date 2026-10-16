#!/usr/bin/env python3
# 🔗️ Reference census of the WP1 files: every crate::/super::/self:: path, resolved to a monolith module and its owner crate.
import os, re, sys, collections, json
from lib import *
from rs import blank
import modmap, wp1files
from crates_wp1 import OWNER, NESTED

REF = re.compile(r"(?<![A-Za-z0-9_$:])((?:crate|super|self)(?:::super)*)::(\*|\{|[A-Za-z_][A-Za-z0-9_]*)")


def module_paths(unit_name, root_file):
    """file -> module path list (names from the monolith crate root) for every file of the tree of a hub unit."""
    out = {}

    def walk(path, mpath):
        if path in out:
            return
        out[path] = mpath
        text = read(path)
        clean = blank(text)
        here = os.path.dirname(path)
        for m in wp1files.MOUNT.finditer(clean):
            if m.group(3) == "{":
                continue
            attrs = text[m.start(1):m.end(1)]
            pm = wp1files.PATH_ATTR.search(attrs)
            if pm:
                child = os.path.normpath(os.path.join(here, pm.group(1)))
            else:
                stem = os.path.splitext(os.path.basename(path))[0]
                base = os.path.join(here, stem)
                child = None
                for cand in (os.path.join(base, m.group(2) + ".rs"), os.path.join(base, m.group(2), "mod.rs")):
                    if os.path.exists(cand):
                        child = cand
                        break
            if child and os.path.exists(child):
                walk(child, mpath + [m.group(2)])
        for m in wp1files.INCLUDE.finditer(text):
            if clean[m.start():m.start() + 8] == "include!":
                inc = os.path.normpath(os.path.join(here, m.group(1)))
                if os.path.exists(inc):
                    walk(inc, mpath)
    walk(root_file, [unit_name])
    return out


def resolve(chain, mpath):
    parts = chain.split("::")
    p = list(mpath)
    out = None
    for i, seg in enumerate(parts):
        if seg == "crate":
            p = []
        elif seg == "super":
            p = p[:-1]
        elif seg == "self":
            pass
    return p


def analyze(unit):
    mounts = modmap.snapshot_mounts()
    if unit in NESTED:
        f = PLUG + "/" + NESTED[unit][2]
        paths = module_paths(unit, f)
        paths = {k: NESTED[unit][1] + v for k, v in paths.items()}
    else:
        f = mounts[unit][0]
        paths = module_paths(unit, f)
    rows = []
    for path, mpath in paths.items():
        text = read(path)
        clean = blank(text)
        for m in REF.finditer(clean):
            base = resolve(m.group(1), mpath)
            nxt = m.group(2)
            line = clean.count("\n", 0, m.start()) + 1
            rows.append((path, mpath, m.group(1), base, nxt, line))
    return rows

if __name__ == "__main__":
    units = sys.argv[1:] or sorted(list(OWNER) + list(NESTED))
    summary = collections.defaultdict(collections.Counter)
    for u in units:
        for path, mpath, chain, base, nxt, line in analyze(u):
            full = base + ([nxt] if nxt not in ("*", "{") else [nxt])
            target = full[0] if full else "<root>"
            crate = OWNER.get(target, "-")
            summary[(u, OWNER.get(u) or NESTED[u][0])][(target, crate if crate != "-" else "OTHER")] += 1
    for (u, c), cnt in summary.items():
        print(f"== {u} [{c}]")
        for (target, crate), n in sorted(cnt.items(), key=lambda kv: (-kv[1], kv[0])):
            if target == u or (OWNER.get(target) == c): continue
            print(f"   {n:4d} -> {target} [{crate}]")
