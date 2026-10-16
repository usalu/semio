#!/usr/bin/env python3
# 🧪️ Rebuilds the four hub test hosts with the replayed `use` statements pruned to the names the hosted tests actually use (an unresolved unused import is still an error).
import re, os
from carvelib import *
from rs import blank, uses
import host_clusters as H
import explode_glob

CFG = "🪟️window/🎚️config"
TR = "🪟️window/🫧️transient"
SPECS = [
    dict(dirname="📬️window-mutation", parent=("🪟️window/📬️mutation/🦀️.rs"), alias="crate::window_mutation", doc="📬️ Test host of `window_mutation`: the erased-payload and refresh laws and the close helpers the window tests share.",
         children=[("tests", "🪟️window/📬️mutation/🧪️tests/🦀️.rs", "")], extra=["pub use crate::window_mutation::refresh::{refresh_demands, refresh_transfer, WindowRefreshSnapshot};", "pub(crate) use tests::{close_original as close_test_original, close_with as close_test_original_with_pump};"]),
    dict(dirname="🎚️window-config/🧬️retained", parent=CFG + "/📥️retained/🦀️.rs", alias="crate::window_config::retained", doc="🧱️ Test host of the window-config `retained` module.",
         children=[("physical_ownership_tests", CFG + "/🧪️tests/🧬️physical-ownership/🦀️.rs", "")], extra=[]),
    dict(dirname="🎚️window-config", parent=CFG + "/🦀️.rs", alias="crate::window_config", doc="🎚️ Test host of `window_config`: the laws that name the facade test fixtures run in the facade test build.",
         children=[("original_preparation_tests", CFG + "/🧬️preparation/🧪️tests/🦀️.rs", ""), ("tests", CFG + "/🧪️tests/🪟️retained-window-config/🦀️.rs", ""), ("pack_identity_tests", CFG + "/🧪️tests/🪪️pack-identity/🦀️.rs", ""),
                   ("retained_pack_load_tests", CFG + "/🧪️tests/📥️retained-pack-load/🦀️.rs", ""), ("retained_tests", ("host", "🧬️retained"), "")], extra=["pub use crate::window_config::retained::*;"]),
    dict(dirname="🫧️window-transient", parent=TR + "/🦀️.rs", alias="crate::window_transient", doc="🫧️ Test host of `window_transient`: the laws that name the facade publication fixtures and the exported transient macros.",
         children=[("retained_window_input_tests", TR + "/🧪️tests/🪟️retained-window-input/🦀️.rs", ""), ("transient_root_tests", TR + "/🧪️tests/🧪️transient-root/🦀️.rs", "")], extra=[]),
]


def tokens_of(files):
    out = set()
    for f in files:
        out |= explode_glob.tokens(read(f))
    return out


def prune_uses(parent_text, toks):
    clean = blank(parent_text)
    out = []
    for m in re.finditer(r"(?m)^((?:#\[[^\]\n]*\]\s*\n)*)((?:pub(?:\([^)]*\))?\s+)?use\s([^;]*);)", clean):
        stmt = (parent_text[m.start(2):m.end(2)]).strip()
        attrs = parent_text[m.start(1):m.end(1)].strip()
        if stmt.startswith("pub"):
            continue
        entries = uses(parent_text[m.start(1):m.end(2)])
        keep = []
        for cfg, vis, path, local, glob in entries:
            if glob:
                keep.append((path, None, True))
            elif local in toks:
                keep.append((path, local, False))
        if not keep:
            continue
        parts = []
        for path, local, glob in keep:
            if glob:
                parts.append("::".join(path) + "::*")
            else:
                base = "::".join(path)
                parts.append(base if path[-1] == local else f"{base} as {local}")
        for p in parts:
            out.append((attrs + "\n" if attrs else "") + f"use {p};")
    return out


def child_files(spec):
    files = []
    for name, target, vis in spec["children"]:
        if isinstance(target, tuple):
            continue
        files.append(PLUG + "/" + target)
    return files


for spec in SPECS:
    path = H.HOSTS + "/" + spec["dirname"] + "/🦀️.rs"
    assert os.path.exists(path), path
    toks = tokens_of(child_files(spec))
    if spec["dirname"] == "🎚️window-config":
        toks |= tokens_of([PLUG + "/" + CFG + "/🧪️tests/🧬️physical-ownership/🦀️.rs"])
        toks |= {"WindowConfigOwner", "WindowConfigPack", "WindowConfigPartition", "WindowRegistry", "PluginLifecycleStep"}
    pt = read(H.SNAPP + "/" + spec["parent"])
    lines = prune_uses(pt, toks)
    body = f"//! {spec['doc'].lstrip('📬️🧱️🎚️🫧️ ')}\n" if False else f"//! {spec['doc']}\n"
    body += "\n".join(lines) + "\n"
    body += f"pub use {spec['alias']}::*;\n"
    for l in spec["extra"]:
        body += l + "\n"
    base = os.path.dirname(path)
    for name, target, vis in spec["children"]:
        p = (target[1] + "/🦀️.rs") if isinstance(target, tuple) else os.path.relpath(PLUG + "/" + target, base)
        body += f'#[path = "{p}"]\n{vis}mod {name};\n'
    write(path, body)
    print(spec["dirname"], len(lines), "use statements")
