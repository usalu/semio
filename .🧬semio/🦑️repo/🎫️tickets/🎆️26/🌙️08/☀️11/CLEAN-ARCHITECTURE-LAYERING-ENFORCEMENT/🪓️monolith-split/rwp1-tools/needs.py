#!/usr/bin/env python3
# 🧲️ Which names of the OLD renderer root namespace (123-name import block, extern aliases, root items) a set of files mentions;
# emits the explicit `extern crate` / `use` lines a carved crate root needs instead of `use super::*`.
import json
import os
import re
import sys
from common import *
from rs import blank, uses, item_names

ROOT_SNAPSHOT = SNAP + "/engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"
PROVIDE = os.path.dirname(os.path.abspath(__file__)) + "/provide.json"
TRAITS = {"Future": ["poll", "Future"]}


def tokens(text):
    clean = blank(text)
    out = set()
    for m in re.finditer(r"(?<![A-Za-z0-9_$])(?:(?<=\.\.)|(?<!\.))([A-Za-z_][A-Za-z0-9_]*)", clean):
        out.add(m.group(1))
    # method-call style tokens for traits
    for m in re.finditer(r"\.([A-Za-z_][A-Za-z0-9_]*)\s*\(", clean):
        out.add("." + m.group(1))
    return out


def root_block():
    text = read(ROOT_SNAPSHOT)
    # the three leading use lines and the import block live in the root file proper
    entries = [e for e in uses(text) if e[1] == ""]
    externs = []
    for m in re.finditer(r"((?:#\[[^\]]*\]\s*)*)extern\s+crate\s+([A-Za-z0-9_]+)\s+as\s+([A-Za-z0-9_]+)\s*;", blank(text)):
        externs.append((text[m.start(1):m.end(1)].strip(), m.group(2), m.group(3)))
    return entries, externs


def provided():
    if os.path.exists(PROVIDE):
        return json.load(open(PROVIDE))
    return {}


def wanted(files, exclude_local=True, skip=(), extra_entries=(), self_lib=None):
    toks = set()
    local = set()
    for f in files:
        t = read(f)
        toks |= tokens(t)
        if exclude_local:
            import items as _items
            local |= {it["name"] for it in _items.table(t) if not it["name"].startswith("impl")}
            for e in uses(t, top_only=True):
                if e[3]:
                    local.add(e[3])
    entries, externs = root_block()
    entries = list(entries) + list(extra_entries)
    use_lines = {}
    for cfg, vis, path, local_name, glob in entries:
        if glob or not local_name:
            continue
        if local_name in skip:
            continue
        hit = local_name in toks
        if local_name in TRAITS and any(x in toks for x in TRAITS[local_name]) or (local_name in TRAITS and ("." + "poll") in toks):
            hit = True
        if hit and local_name not in local:
            use_lines.setdefault((cfg, "::".join(path[:-1])), set()).add(path[-1] + ("" if path[-1] == local_name else f" as {local_name}"))
    extern_lines = []
    for cfg, crate, alias in externs:
        if alias in toks and alias not in local and re.search(r"(?<![A-Za-z0-9_])" + re.escape(alias) + r"::", "\n".join(blank(read(f)) for f in files)):
            extern_lines.append((cfg, crate, alias))
    prov = provided()
    pcfg = json.load(open(PROVIDE.replace("provide.json", "provide_cfg.json"))) if os.path.exists(PROVIDE.replace("provide.json", "provide_cfg.json")) else {}
    prov_lines = {}
    for name, lib in prov.items():
        if lib == self_lib:
            continue
        if name in toks and name not in local and name not in skip:
            prov_lines.setdefault((lib, pcfg.get(name, "")), set()).add(name)
    return use_lines, extern_lines, prov_lines


def render(use_lines, extern_lines, prov_lines=None):
    out = []
    for cfg, crate, alias in extern_lines:
        out.append((cfg + "\n" if cfg else "") + f"extern crate {crate} as {alias};")
    for (cfg, parent), names in sorted(use_lines.items(), key=lambda kv: (kv[0][1], kv[0][0])):
        names = sorted(names)
        body = names[0] if len(names) == 1 else "{" + ", ".join(names) + "}"
        out.append((cfg + "\n" if cfg else "") + f"use {parent}::{body};")
    for (lib, cfg), names in sorted((prov_lines or {}).items()):
        names = sorted(names)
        body = names[0] if len(names) == 1 else "{" + ", ".join(names) + "}"
        out.append((cfg + "\n" if cfg else "") + f"use {lib}::{body};")
    return "\n".join(out)


if __name__ == "__main__":
    files = sys.argv[1:]
    print(render(*wanted(files)))
