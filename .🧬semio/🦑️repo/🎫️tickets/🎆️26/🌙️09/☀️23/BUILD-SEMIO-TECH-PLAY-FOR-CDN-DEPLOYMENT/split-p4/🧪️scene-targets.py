#!/usr/bin/env python3
"""🎞️ Moves a mutation group's per-leaf scene tests out of the group crate into one integration test root of the union crate.

The group root mounts every leaf as `#[path = "."] pub mod <leaf> { … #[cfg(test)] #[path = "<leaf dir>/🧪️tests/<case>/🦀️.rs"]
mod <name>; }`. Scene tests exercise the dispatch union (`apply_*_mutation`, `inverse_*_mutation`), which sits above the
group crate, so they run as `[[test]]` targets of the union crate instead. This removes the `#[cfg(test)]` mounts from the
group root and writes `<group>/🧪️tests/🦀️.rs`, which re-mounts the same files under the same leaf modules with the crate
aliases they name.

usage: 🧪️scene-targets.py <group root 🦀️.rs> <doc line> [--apply]
"""
import os, re, sys

root, doc = os.path.abspath(sys.argv[1]), sys.argv[2]
apply = "--apply" in sys.argv
text = open(root, encoding="utf-8").read()
BLOCK = re.compile(r'(?ms)^#\[path = "\."\]\npub mod (?P<leaf>\w+) \{\n(?P<body>.*?)^\}\n')
MOUNT = re.compile(r'(?m)^    #\[cfg\(test\)\]\n    #\[path = "(?P<path>[^"]+)"\]\n    mod (?P<name>\w+);\n')
ALIASES = re.findall(r'(?m)^extern crate (\w+) as (\w+);$', text)
leaves, edits = [], []
for block in BLOCK.finditer(text):
    mounts = [(m["path"], m["name"]) for m in MOUNT.finditer(block["body"])]
    if mounts:
        leaves.append((block["leaf"], mounts))
        start = block.start("body")
        for m in MOUNT.finditer(block["body"]):
            edits.append((start + m.start(), start + m.end()))
if not leaves:
    raise SystemExit("no scene mounts in " + root)
for start, end in sorted(edits, reverse=True):
    text = text[:start] + text[end:]
files = [os.path.normpath(os.path.join(os.path.dirname(root), path)) for _, mounts in leaves for path, _ in mounts]
used = set()
for path in files:
    body = open(path, encoding="utf-8").read()
    used |= {alias for _, alias in ALIASES if re.search(r'(?<![\w:])' + alias + r'::', body)}
lines = [f"//! 🎞️ {doc}", "", "#![allow(clippy::result_large_err)]", ""]
lines += [f"extern crate {crate} as {alias};" for crate, alias in ALIASES if alias in used]
for leaf, mounts in leaves:
    lines += ["", '#[path = ".."]', f"mod {leaf} {{"]
    for path, name in mounts:
        lines += [f'    #[path = "{path}"]', f"    mod {name};"]
    lines.append("}")
target = os.path.join(os.path.dirname(root), "🧪️tests", "🦀️.rs")
print(f"{os.path.relpath(target)}: {sum(len(m) for _, m in leaves)} scenes over {len(leaves)} leaves, aliases {sorted(used)}")
for path in files:
    print("  " + os.path.relpath(path))
if apply:
    os.makedirs(os.path.dirname(target), exist_ok=True)
    if os.path.exists(target):
        raise SystemExit("exists: " + target)
    open(target, "w", encoding="utf-8").write("\n".join(lines) + "\n")
    open(root, "w", encoding="utf-8").write(text)
