#!/usr/bin/env python3
"""📂️ Replaces `include_bytes!`/`include_str!` of committed test inputs in the given test files with runtime reads through
the crate's `committed`/`committed_text` helpers, so test fixtures stay out of the compilation unit's Source closure.

Paths are rewritten relative to the crate's manifest directory (`CARGO_MANIFEST_DIR`). `include_str!(…)` used as a
call argument becomes `&crate::committed_text(…)`; a `let` binding takes the owned `String`.

usage: 📂️runtime-fixtures.py <manifest dir> [--write] <test file>...
"""
import os, re, sys

INCLUDE = re.compile(r'include_(bytes|str)!\("([^"]+)"\)(\.to_vec\(\))?')


def rewrite(path, manifest_dir):
    text = open(path, encoding="utf-8").read()
    def replace(match):
        kind, relative, to_vec = match.groups()
        target = os.path.relpath(os.path.normpath(os.path.join(os.path.dirname(path), relative)), manifest_dir)
        if kind == "bytes":
            return f'crate::committed("{target}")'
        before = text[:match.start()].rstrip()
        owned = before.endswith("=")
        return f'{"" if owned else "&"}crate::committed_text("{target}")'
    return INCLUDE.sub(replace, text)


def main():
    manifest_dir = os.path.abspath(sys.argv[1])
    write = "--write" in sys.argv
    for path in [a for a in sys.argv[2:] if a != "--write"]:
        path = os.path.abspath(path)
        new = rewrite(path, manifest_dir)
        old = open(path, encoding="utf-8").read()
        print(f"{'write' if write else 'plan'} {path} {len(INCLUDE.findall(old))} includes")
        if write and new != old:
            open(path, "w", encoding="utf-8").write(new)


main()
