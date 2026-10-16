#!/usr/bin/env python3
# 🪚️ Helpers of the per-crate carve scripts: anchored edits, hub edits, manifest edits, stub removal.
import json, os, re, sys
from lib import *

HUB = PLUG + "/🪝️mounts/🌱️wp1-foundation/🦀️.rs"
ROOTRS = PLUG + "/🦀️.rs"


def edit(path, old, new, count=1):
    text = read(path)
    assert text.count(old) == count, (path, old[:100], text.count(old))
    write(path, text.replace(old, new))


def hub_edit(old, new, count=1):
    edit(HUB, old, new, count)


def pkg_dir(rootdir):
    return PLUG + "/" + rootdir + "/" + PKGDIR


def set_lib_path(rootdir, path="../../🦀️.rs"):
    p = pkg_dir(rootdir) + "/Cargo.toml"
    t = read(p)
    assert t.count('[lib]\npath = "🦀️.rs"') == 1, p
    write(p, t.replace('[lib]\npath = "🦀️.rs"', f'[lib]\npath = "{path}"'))


def remove_stub(rootdir):
    p = pkg_dir(rootdir) + "/🦀️.rs"
    if os.path.exists(p):
        os.remove(p)


def mark_carved(key):
    p = TOOLS + "/carved.json"
    data = json.load(open(p))
    if key not in data:
        data.append(key)
    json.dump(data, open(p, "w"))


def aliases_used(files):
    """kernel aliases (dsl/protocol/store/vcs/pack) a set of files names as a path root."""
    from rs import blank
    text = "\n".join(blank(read(f)) for f in files)
    found = []
    for a in ("dsl", "protocol", "store", "vcs"):
        if re.search(r"(?<![A-Za-z0-9_$.:])" + a + r"::", text) or re.search(r"(?<![A-Za-z0-9_$.:])" + a + r"\b\s*(?:as\b|;|,|\})", text) and False:
            found.append(a)
    return found


def ensure_externs(rootfile, aliases):
    t = read(rootfile)
    add = []
    for a in aliases:
        line = f"extern crate semio_framework_os_kernel as {a};"
        if line not in t:
            add.append(line)
    if not add:
        return []
    head_end = 0
    lines = t.split("\n")
    i = 0
    while i < len(lines) and lines[i].startswith("//!"):
        i += 1
    t = "\n".join(lines[:i]) + ("\n" if i else "") + "\n".join(add) + "\n" + "\n".join(lines[i:])
    write(rootfile, t)
    return add
