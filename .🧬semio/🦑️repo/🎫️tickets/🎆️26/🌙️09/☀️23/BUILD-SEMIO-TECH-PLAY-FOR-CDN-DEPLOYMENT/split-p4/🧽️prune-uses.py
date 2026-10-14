#!/usr/bin/env python3
"""🧽️ Drops `use` leaves whose binding name the file never mentions outside its `use` statements (module level and inline
module level alike); `pub use` re-exports, globs, `self` leaves and trait imports named `_` are kept.

usage: 🧽️prune-uses.py <file.rs>...
"""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ns = {"__file__": os.path.join(HERE, "✂️owner-cut.py")}
exec(open(os.path.join(HERE, "✂️owner-cut.py"), encoding="utf-8").read().split("\nif __name__")[0], ns)
strip, use_leaves = ns["strip"], ns["use_leaves"]
TRAITISH = re.compile(r"^(?:[A-Z][A-Za-z0-9]*)$")
for path in sys.argv[1:]:
    raw = open(path, encoding="utf-8").read()
    s = strip(raw)
    stmts = list(re.finditer(r"(?m)^(?P<indent>[ \t]*)(?P<vis>pub(?:\([^)]*\))?\s+)?use\s+(?P<body>[^;]*);[ \t]*\n", s))
    masked = list(s)
    for m in stmts:
        for i in range(m.start(), m.end()):
            if masked[i] != "\n":
                masked[i] = " "
    body = "".join(masked)
    for fmt in re.finditer(r"\{([A-Za-z_][A-Za-z0-9_]*)(?::[^{}]*)?\}", raw):
        body += " " + fmt.group(1)
    idents = set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", body))
    macros = set(re.findall(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*!", body))
    edits = []
    for m in stmts:
        if m.group("vis"):
            continue
        prev = s[:m.start()].rstrip("\n").split("\n")[-1].strip()
        leaves = use_leaves([], m.group("body"))
        keep = []
        for segs, alias, glob in leaves:
            name = alias or (segs[-1] if segs else "")
            if glob or name in ("self", "_") or not segs:
                keep.append((segs, alias, glob)); continue
            if name in macros or re.search(r"(?<![.\w])" + re.escape(name) + r"\b(?!\s*:(?!:))", body):
                keep.append((segs, alias, glob)); continue
            if TRAITISH.match(name) and re.search(r"\." + r"|::", body) and name.endswith(("Ext", "Trait")):
                keep.append((segs, alias, glob)); continue
            trait_methods = False
            if TRAITISH.match(name):
                trait_methods = bool(re.search(r"\btrait\b|ToValue|FromValue|Mutation|Retire|Clone|Algebra|Hash|Write|Read|Field|Tree|Dsl|Binary|Text|Schema|Codec|Owner|Job|App|Editor|Viewer|Factory|Preparation|Snapshot", name))
            if trait_methods:
                keep.append((segs, alias, glob)); continue
        if len(keep) == len(leaves):
            continue
        if not keep:
            edits.append((m.start(), m.end(), ""))
            continue
        parts = ["::".join(segs) + ("::*" if glob else "") + (f" as {alias}" if alias else "") for segs, alias, glob in keep]
        tree = parts[0] if len(parts) == 1 else "{" + ", ".join(parts) + "}"
        edits.append((m.start(), m.end(), f"{m.group('indent')}use {tree};\n"))
    if edits:
        out, last = [], 0
        for a, b, rep in edits:
            out.append(raw[last:a]); out.append(rep); last = b
        out.append(raw[last:])
        open(path, "w", encoding="utf-8").write("".join(out))
        print(f"pruned {len(edits)} use statements in {path.split('✳️any/')[-1]}")
