#!/usr/bin/env python3
# 💥️ Replaces `use super::*;` of a child file by the explicit imports it needs from its parent's namespace.
# usage: explode_glob.py <child> <parent> [--apply] [--crate K]   (paths absolute or relative to PLUG)
import os, re, sys
from lib import *
from rs import blank, uses, item_names
import rewrite_paths


def tokens(text):
    clean = blank(text)
    out = set(re.findall(r"(?<![A-Za-z0-9_$])([A-Za-z_][A-Za-z0-9_]*)", clean))
    return out


def parent_namespace(parent_text):
    """local name -> list of (cfg, vis, path) from top-level `use`; plus names of items the parent defines."""
    entries = {}
    for cfg, vis, path, local, glob in uses(parent_text, top_only=True):
        if glob or not local:
            continue
        entries.setdefault(local, []).append((cfg, vis, path))
    import defs
    items = {d.name for d in defs.scan(parent_text) if d.kind == "item"}
    items |= set(re.findall(r"\bmacro_rules!\s+([A-Za-z_0-9]+)", blank(parent_text)))
    return entries, items


def explode(child_text, parent_text, this_crate=None, parent_is_unit_root=True, parent_alias=None):
    clean = blank(child_text)
    ctoks = tokens(child_text)
    import defs as _defs
    local = {d.name for d in _defs.scan(child_text) if d.kind == "item"}
    for cfg, vis, path, loc, glob in uses(child_text, top_only=True):
        if loc:
            local.add(loc)
    entries, pitems = parent_namespace(parent_text)
    lines = {}
    need_items = []
    for name, ents in entries.items():
        if name in ctoks and name not in local:
            for cfg, vis, path in ents:
                lines.setdefault((cfg, tuple(path[:-1])), set()).add((path[-1], name))
    for name in pitems:
        if name in ctoks and name not in local and name not in entries:
            need_items.append(name)
    out = []
    for (cfg, parent_path), names in sorted(lines.items()):
        names = sorted(names)
        body = ", ".join(a if a == b else f"{a} as {b}" for a, b in names)
        body = body if len(names) == 1 else "{" + body + "}"
        out.append((cfg + "\n" if cfg else "") + f"use {'::'.join(parent_path)}::{body};")
    if parent_alias:
        out.append(f"use {parent_alias}::*;")
    text = "\n".join(out)
    if this_crate:
        text, un = rewrite_paths.rewrite_text(text, this_crate, True)
    else:
        un = []
    return text, need_items, un


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    child, parent = [a if os.path.isabs(a) else PLUG + "/" + a for a in args[:2]]
    crate = None
    for i, a in enumerate(sys.argv):
        if a == "--crate":
            crate = sys.argv[i + 1]
    alias = None
    for i, a in enumerate(sys.argv):
        if a == "--alias":
            alias = sys.argv[i + 1]
    ct, pt = read(child), read(parent)
    text, items, un = explode(ct, pt, crate, parent_alias=alias)
    print(text)
    print("-- parent items needed:", items)
    print("-- unresolved:", un)
    if "--apply" in sys.argv:
        assert ct.count("use super::*;") == 1
        write(child, ct.replace("use super::*;", text if text else "", 1))
