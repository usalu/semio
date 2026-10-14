#!/usr/bin/env python3
"""🩹️ Re-imports names a file only uses inside format strings (`format!("{NAME}")`) when no `use` or local definition binds
them; the defining module is looked up in the crate's definition index (unique names only).

usage: 🩹️restore-format-idents.py <crate root> <file.rs>...
"""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ns = {"__file__": os.path.join(HERE, "✂️owner-cut.py")}
exec(open(os.path.join(HERE, "✂️owner-cut.py"), encoding="utf-8").read().split("\nif __name__")[0], ns)
Old, strip, use_leaves, ITEM = ns["Old"], ns["strip"], ns["use_leaves"], ns["ITEM"]
REPO = ns["REPO"]
old = Old(os.path.relpath(sys.argv[1], REPO))
where = {}
for module, names in old.defs.items():
    for name, entries in names.items():
        if any(e[0] == "def" for e in entries):
            where.setdefault(name, set()).add(module)
for path in sys.argv[2:]:
    raw = open(path, encoding="utf-8").read()
    s = strip(raw)
    bound = set()
    for m in re.finditer(r"\buse\s+([^;]*);", s):
        for segs, alias, glob in use_leaves([], m.group(1)):
            if segs:
                bound.add(alias or segs[-1])
    bound |= {m.group("name") for m in ITEM.finditer(s)}
    need = []
    for fmt in re.finditer(r"\{([A-Z_][A-Z0-9_]*)(?::[^{}]*)?\}", raw):
        name = fmt.group(1)
        if name in bound or name in need or len(where.get(name, ())) != 1:
            continue
        need.append(name)
    if not need:
        continue
    lines = "".join(f"use crate::{'::'.join(next(iter(where[n])))}::{n};\n" for n in need)
    first = re.search(r"(?m)^use ", raw)
    at = first.start() if first else 0
    open(path, "w", encoding="utf-8").write(raw[:at] + lines + raw[at:])
    print(f"restored {need} in {path.split('✳️any/')[-1]}")
