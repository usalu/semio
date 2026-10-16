#!/usr/bin/env python3
# 🪚️ Helpers shared by the per-crate carve scripts of RENDERER-WP1 (anchored edits, crate roots, hub lines).
import os
import re
import json
from common import *

HUB = T + "/🧊️renderer/📥️mounts/🌱️wp1-foundation/🦀️.rs"


def edit(path, old, new, count=1):
    text = read(path)
    assert text.count(old) == count, (path, old[:80], text.count(old))
    write(path, text.replace(old, new))


def widen(path, pairs=(("pub(crate)", "pub"),)):
    text = read(path)
    n = 0
    for a, b in pairs:
        n += text.count(a)
        text = text.replace(a, b)
    write(path, text)
    return n


def relpath(frm_dir, to):
    return os.path.relpath(to, frm_dir)


def extern_line(suffix, module, cfg=None):
    prefix = f"#[cfg({cfg})]\n" if cfg else ""
    return f"{prefix}extern crate {lib_name(suffix)} as {module};\n"


def crate_root_text(suffix, doc, mounts, imports="", tail=""):
    """mounts: list of (module, absolute source path, visibility, cfg or None); the crate root re-exports module items (glob) unless vis == 'pub mod'."""
    folder = T + "/" + CRATES[suffix]
    out = doc.rstrip("\n") + "\n\n"
    if imports:
        out += imports.rstrip("\n") + "\n\n"
    for module, src, vis, cfg in mounts:
        if cfg:
            out += f"#[cfg({cfg})]\n"
        out += f'#[path = "{relpath(folder, src)}"]\n'
        if vis == "pub mod":
            out += f"pub mod {module};\n\n"
        else:
            out += f"mod {module};\n"
            if cfg:
                out += f"#[cfg({cfg})]\n"
            out += f"pub use {module}::*;\n\n"
    if tail:
        out += tail.rstrip("\n") + "\n\n"
    out += '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n'
    return out


def write_crate_root(suffix, *args, **kw):
    write(T + "/" + CRATES[suffix] + "/🦀️.rs", crate_root_text(suffix, *args, **kw))


def mark_carved(suffix):
    p = os.path.dirname(os.path.abspath(__file__)) + "/carved.json"
    data = json.load(open(p))
    if suffix not in data:
        data.append(suffix)
    json.dump(data, open(p, "w"))
