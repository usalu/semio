#!/usr/bin/env python3
# 📚️ provide.json: every public column-0 item name of a carved crate -> its library name (what `use super::*` of the old root namespace used to supply).
import json, os, sys
from common import *
import modtree, items
from rs import blank, uses

SKIP_KINDS = {"impl", "use", "macro_rules!"}


def pub_items(text):
    res = {}
    for it in items.table(text):
        if it["kind"] in SKIP_KINDS or it.get("thread_local"):
            continue
        if not it["vis"].startswith("pub") or (it["vis"].startswith("pub(") and it["vis"] != "pub(crate)"):
            continue
        if it["name"] == "tests" or it["name"].endswith("_tests"):
            continue
        res[it["name"]] = it
    return res


def crate_names(suffix):
    root = T + "/" + CRATES[suffix] + "/🦀️.rs"
    names = pub_items(read(root))
    text = read(root)
    here = os.path.dirname(root)
    mount_files = {}
    for m in modtree.MOUNT.finditer(blank(text)):
        attrs = text[m.start(1):m.end(1)]
        pm = modtree.PATH_ATTR.search(attrs)
        if pm:
            mount_files[m.group(2)] = os.path.normpath(os.path.join(here, pm.group(1)))
    for cfg, vis, path, local, glob in uses(text):
        if not vis.startswith("pub"):
            continue
        if glob and len(path) == 1 and path[0] in mount_files:
            for n in pub_items(read(mount_files[path[0]])):
                names.setdefault(n, None)
        elif glob and path and path[0] in ("crate",):
            continue
        elif not glob and len(path) >= 2 and path[0] in mount_files:
            names.setdefault(local, None)
        elif not glob and local:
            names.setdefault(local, None)
    return names


def cfg_names(suffix):
    root = T + "/" + CRATES[suffix] + "/🦀️.rs"
    res = {}
    for f in modtree.tree(root):
        for it in items.table(read(f)):
            if "#[cfg(test)]" in it["attrs"] and it["vis"].startswith("pub"):
                res[it["name"]] = "#[cfg(test)]"
    return res


def build(only=None):
    carved = json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/carved.json"))
    out, seen = {}, {}
    for c in carved:
        for n in crate_names(c):
            if n in out and out[n] != lib_name(c):
                seen.setdefault(n, [out[n]]).append(lib_name(c))
                continue
            out[n] = lib_name(c)
    cfgs = {}
    for c in carved:
        for n, v in cfg_names(c).items():
            if out.get(n) == lib_name(c):
                cfgs[n] = v
    json.dump(cfgs, open(os.path.dirname(os.path.abspath(__file__)) + "/provide_cfg.json", "w"), ensure_ascii=False, indent=0, sort_keys=True)
    json.dump(out, open(os.path.dirname(os.path.abspath(__file__)) + "/provide.json", "w"), ensure_ascii=False, indent=0, sort_keys=True)
    print(len(out), "names; collisions", {k: v for k, v in seen.items()})


if __name__ == "__main__":
    build()
