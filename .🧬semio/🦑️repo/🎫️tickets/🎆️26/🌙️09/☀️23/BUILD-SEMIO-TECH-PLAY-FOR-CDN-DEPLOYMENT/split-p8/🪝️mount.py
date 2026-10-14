#!/usr/bin/env python3
"""🪝️ Mounts one module file as `pub mod <name>` inside the crate root's inline tree at the given module path.

usage: 🪝️mount.py <crate root.rs> <module path> <name> <file.rs>
"""
import os, re, sys
root, module, name, target = sys.argv[1:5]
src = open(root, encoding="utf-8").read()
start, end, indent = 0, len(src), ""
for seg in module.split("::"):
    found = None
    for m in re.finditer(r'(?m)^(?P<indent>[ \t]*)pub mod ' + re.escape(seg) + r' \{\n', src[start:end]):
        if len(m.group("indent")) == (len(indent) + 4 if start else 0):
            found = m; break
    if not found:
        raise SystemExit(f"no tree {seg} of {module}")
    open_at = start + found.end() - 2
    depth, j = 0, open_at
    while True:
        if src[j] == "{": depth += 1
        elif src[j] == "}":
            depth -= 1
            if depth == 0: break
        j += 1
    indent = found.group("indent"); start, end = start + found.end(), j
rel = os.path.relpath(os.path.abspath(target), os.path.dirname(os.path.abspath(root)))
src = src[:start] + f'{indent}    #[path = "{rel}"]\n{indent}    pub mod {name};\n' + src[start:]
open(root, "w", encoding="utf-8").write(src)
print("mounted", module + "::" + name)
