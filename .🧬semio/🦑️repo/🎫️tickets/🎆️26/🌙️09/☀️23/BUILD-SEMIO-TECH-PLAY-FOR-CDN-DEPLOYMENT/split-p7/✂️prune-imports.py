#!/usr/bin/env python3
"""✂️ Drops the top-level `use` leaves of the given files whose bound name is a module, function or constant (lowercase or
SCREAMING_CASE) that the file no longer names, iterating to a fixpoint so imports that only fed other dropped imports go
too. Only leaves rooted in `crate`/`super`/`self` or in a name the file itself imports are considered; type and trait
leaves stay (a trait can be used for method resolution only) and external crate leaves stay.

usage: ✂️prune-imports.py <file.rs>...
"""
import importlib.util, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("family_cut", os.path.join(HERE, "✂️family-cut.py"))
fc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fc)
USE = re.compile(r"(?m)^(?P<vis>pub(?:\([^)]*\))?\s+)?use\s+(?P<body>[^;]*);[ \t]*\n")
for path in sys.argv[1:]:
    raw = open(path, encoding="utf-8").read()
    changed = False
    while True:
        text = fc.strip(raw)
        stmts = [m for m in USE.finditer(text) if not m.group("vis")]
        bound = {alias or segs[-1] for m in stmts for segs, alias in fc.leaves([], m.group("body"))}
        outside = text
        for m in reversed(list(USE.finditer(text))):
            outside = outside[:m.start()] + outside[m.end():]
        heads = [re.match(fc.IDENT, m.group("body").strip()).group(0) for m in USE.finditer(text) if re.match(fc.IDENT, m.group("body").strip())]
        edit = None
        for m in stmts:
            head = re.match(fc.IDENT, m.group("body").strip())
            if not head or (head.group(0) not in ("crate", "super", "self") and head.group(0) not in bound):
                continue
            rows = fc.leaves([], m.group("body"))
            keep = []
            for segs, alias in rows:
                name = alias or segs[-1]
                prunable = name not in ("*", "self", "_") and (name[:1].islower() or re.fullmatch(r"[A-Z][A-Z0-9_]+", name))
                used = re.search(r"(?<![\w.])" + re.escape(name) + r"\b", outside) or heads.count(name) > (1 if name == head.group(0) else 0)
                if prunable and not used:
                    continue
                keep.append((segs, alias))
            if len(keep) != len(rows):
                groups = {}
                for segs, alias in keep:
                    groups.setdefault(segs[0], []).append("::".join(segs[1:]) + (f" as {alias}" if alias else ""))
                rep = "".join(f"use {h}::{p[0] if len(p) == 1 else '{' + ', '.join(p) + '}'};\n" for h, p in groups.items())
                edit = (m.start(), m.end(), rep)
                break
        if not edit:
            break
        raw = raw[:edit[0]] + edit[2] + raw[edit[1]:]
        changed = True
    if changed:
        open(path, "w", encoding="utf-8").write(raw)
        print(f"pruned {os.path.relpath(path)}")
