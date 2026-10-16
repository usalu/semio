#!/usr/bin/env python3
# 🧪️ Hub test hosts for the window-config / window-transient test clusters that name facade-only fixtures.
# A host is a FILE module of hub 1 (a `#[path]` inside an inline module would resolve through a directory named after it): it sits at the nesting level of the original module, replays that module's pre-carve `use` statements, globs the carved crate's public API and mounts the relocated tests, so `use super::*` and `super::sibling` keep their meaning.
import os, re, json
from carvelib import *
from rs import blank

SNAPP = SNAP + "/" + PLUG_REL
HUBDIR = PLUG + "/🪝️mounts/🌱️wp1-foundation"
HOSTS = HUBDIR + "/🧪️test-hosts"


def top_uses(text):
    clean = blank(text)
    out = []
    for m in re.finditer(r"(?m)^((?:#\[[^\]\n]*\]\s*\n)*)((?:pub(?:\([^)]*\))?\s+)?use\s[^;]*;)", clean):
        out.append((text[m.start(1):m.end(1)] + text[m.start(2):m.end(2)]).strip())
    return out


def make_host(dirname, host, parent_rel, alias, children, doc, extra=(), unmount=()):
    """dirname: folder under 🧪️test-hosts; children: (mod name, repo-relative PLUG path or ('host', dirname)); unmount: (file rel, text) pairs removed from the carved sources."""
    pt = read(SNAPP + "/" + parent_rel)
    lines = top_uses(pt)
    path = HOSTS + "/" + dirname + "/🦀️.rs"
    assert not os.path.exists(path), path
    base = os.path.dirname(path)
    body = f"//! 🧪️ {doc}\n"
    body += "\n".join(lines) + "\n"
    body += f"pub use {alias}::*;\n"
    for l in extra:
        body += l + "\n"
    for name, target, vis in children:
        if isinstance(target, tuple):
            p = f"{target[1]}/🦀️.rs"
        else:
            p = os.path.relpath(PLUG + "/" + target, base)
        body += f'#[path = "{p}"]\n{vis}mod {name};\n'
    write(path, body)
    for rel, text in unmount:
        edit(PLUG + "/" + rel, text, "")
    return path


def mount_in_hub(host, dirname):
    h = read(HUB)
    region, end = "//#region 🧪️Wp1TestHosts\n", "//#endregion 🧪️Wp1TestHosts\n"
    block = f'#[cfg(test)]\n#[path = "🧪️test-hosts/{dirname}/🦀️.rs"]\nmod {host};\n'
    assert f"mod {host};" not in h
    if region in h:
        h = h.replace(end, block + end)
    else:
        h = h.rstrip("\n") + "\n" + region + block + end
    write(HUB, h)
