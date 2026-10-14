#!/usr/bin/env python3
"""🧪️ Turns unit-test modules mounted at a family crate's root into `[[test]]` targets of that crate (split lane P7), for
a crate whose test unit would otherwise exceed the Source ceiling: the root's `#[cfg(test)] #[path] mod` mount goes, the
test file's `crate::`/`super::` paths name the crate, and the manifest declares the target.

usage: 🧪️to-integration.py <crate root 🦀️.rs> <crate ident> <Cargo.toml> <test name>=<test file relative to the root dir>...
"""
import os, re, sys
root, ident, manifest = sys.argv[1:4]
text = open(root, encoding="utf-8").read()
cargo = open(manifest, encoding="utf-8").read()
for spec in sys.argv[4:]:
    name, rel = spec.split("=", 1)
    mount = re.compile(r'(?m)^[ \t]*#\[cfg\(test\)\]\n[ \t]*#\[path = "' + re.escape(rel) + r'"\]\n[ \t]*(?:pub )?mod \w+;\n')
    text, n = mount.subn("", text)
    assert n == 1, (rel, n)
    path = os.path.join(os.path.dirname(root), rel)
    test = open(path, encoding="utf-8").read()
    test = re.sub(r"(?<![\w:$])crate::", ident + "::", test)
    test = re.sub(r"(?<![\w:$])super::", ident + "::", test)
    open(path, "w", encoding="utf-8").write(test)
    target = f'\n[[test]]\nname = "{name}"\npath = "{os.path.relpath(path, os.path.dirname(manifest))}"\n'
    cargo = cargo.replace("\n[dependencies]", target + "\n[dependencies]", 1)
    print(f"{name}: {rel}")
open(root, "w", encoding="utf-8").write(text)
open(manifest, "w", encoding="utf-8").write(cargo)
