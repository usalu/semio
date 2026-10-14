#!/usr/bin/env python3
"""🔎️ Lists (and with --write removes) `use crate::<name>` leaves in crate-root files that import an item the same
root file defines at top level (E0255 after items moved into the root).

usage: 🔎️self-imports.py <root.rs>... [--write]
"""
import re, sys, importlib.util, os
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("relocate", os.path.join(HERE, "🧭️relocate.py")); r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
for path in [a for a in sys.argv[1:] if not a.startswith("--")]:
    raw = open(path, encoding="utf-8").read(); text = r.strip(raw)
    defs = set(m.group("name") for m in re.finditer(r"(?m)^" + r.ITEM_HEAD.pattern, text))
    edits = []
    for m in re.finditer(r"(?m)^use\s+crate::(\{[^;]*\}|\w+);\n", text):
        body = m.group(1)
        names = [n.strip() for n in body.strip("{}").split(",") if n.strip()]
        keep = [n for n in names if not (re.fullmatch(r"\w+", n) and n in defs)]
        if len(keep) != len(names):
            print(f"{path}: drop {sorted(set(names) - set(keep))}")
            edits.append((m.start(), m.end(), "" if not keep else f"use crate::{{{', '.join(keep)}}};\n"))
    if "--write" in sys.argv and edits:
        for a, b, rep in sorted(edits, reverse=True):
            raw = raw[:a] + rep + raw[b:]
        open(path, "w", encoding="utf-8").write(raw)
