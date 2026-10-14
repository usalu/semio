#!/usr/bin/env python3
"""🧹️ Removes duplicate `use` leaves at module level (same binding, same path) and leaves that import an item the same module
defines; reports same-name conflicts. Only top-level (column 0) `use` statements without attributes are touched.

usage: 🧹️dedupe-uses.py <file.rs>...
"""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ns = {"__file__": os.path.join(HERE, "✂️owner-cut.py")}
exec(open(os.path.join(HERE, "✂️owner-cut.py"), encoding="utf-8").read().split("\nif __name__")[0], ns)
strip, use_leaves, ITEM = ns["strip"], ns["use_leaves"], ns["ITEM"]
for path in sys.argv[1:]:
    raw = open(path, encoding="utf-8").read()
    s = strip(raw)
    defined = {m.group("name") for m in ITEM.finditer(s) if not s[:m.start()].rstrip().endswith("]") or True}
    top_defs = set()
    for m in ITEM.finditer(s):
        line_start = s.rfind("\n", 0, m.start()) + 1
        if s[line_start:m.start("kind")].strip().startswith(("pub", "")) and (m.start() == line_start or not s[line_start:m.start()].strip()):
            if not s[line_start:line_start + 1].isspace():
                top_defs.add(m.group("name"))
    seen, edits, conflicts = {}, [], []
    for m in re.finditer(r"(?m)^(?P<vis>pub(?:\([^)]*\))?\s+)?use\s+(?P<body>[^;]*);[ \t]*\n", s):
        prev = s[:m.start()].rstrip("\n").split("\n")[-1].strip()
        if prev.startswith("#["):
            continue
        leaves = use_leaves([], m.group("body"))
        keep = []
        for segs, alias, glob in leaves:
            if glob:
                keep.append((segs, alias, glob)); continue
            name = alias or (segs[-1] if segs else "")
            if name == "self" and len(segs) > 1:
                name = alias or segs[-2]
            path_key = "::".join(segs)
            if name in top_defs and not m.group("vis"):
                continue
            if name in seen:
                if seen[name] != path_key:
                    conflicts.append((name, seen[name], path_key))
                continue
            seen[name] = path_key
            keep.append((segs, alias, glob))
        if len(keep) == len(leaves):
            continue
        if not keep:
            edits.append((m.start(), m.end(), ""))
            continue
        parts = ["::".join(segs) + ("::*" if glob else "") + (f" as {alias}" if alias else "") for segs, alias, glob in keep]
        tree = parts[0] if len(parts) == 1 else "{" + ", ".join(parts) + "}"
        edits.append((m.start(), m.end(), f"{m.group('vis') or ''}use {tree};\n"))
    if edits:
        out, last = [], 0
        for a, b, rep in edits:
            out.append(raw[last:a]); out.append(rep); last = b
        out.append(raw[last:])
        open(path, "w", encoding="utf-8").write("".join(out))
        print(f"deduped {len(edits)} use statements in {os.path.relpath(path)}")
    for c in conflicts:
        print(f"CONFLICT {os.path.relpath(path)}: {c}")
