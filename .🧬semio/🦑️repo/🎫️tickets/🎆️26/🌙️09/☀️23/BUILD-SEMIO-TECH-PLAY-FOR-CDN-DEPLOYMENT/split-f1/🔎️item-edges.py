#!/usr/bin/env python3
"""🔎️ Item-level edge census between line-range groups of one Rust file (comments and strings blanked).

usage: 🔎️item-edges.py <file.rs> <groups.json>
groups.json: {"<group>": [[start, end], ...], ...} with 1-based inclusive line ranges, groups in dependency order.
Prints, per group, the items it defines and every reference to an item another group defines (upward edges flagged).
"""
import json, re, sys

def blank(src):
    out, i, n = [], 0, len(src)
    while i < n:
        if src.startswith("//", i):
            j = src.find("\n", i); j = n if j < 0 else j; out.append(" " * (j - i)); i = j
        elif src.startswith("/*", i):
            j = src.find("*/", i + 2); j = n if j < 0 else j + 2; out.append(re.sub(r"[^\n]", " ", src[i:j])); i = j
        elif src[i] == '"':
            j = i + 1
            while j < n and src[j] != '"': j += 2 if src[j] == "\\" else 1
            out.append('"' + re.sub(r"[^\n]", " ", src[i + 1:j]) + '"'); i = j + 1
        elif src[i] == "'" and re.match(r"'(\\.|[^\\'])'", src[i:i + 4]):
            m = re.match(r"'(\\.|[^\\'])'", src[i:]); out.append(" " * len(m.group(0))); i += len(m.group(0))
        else:
            out.append(src[i]); i += 1
    return "".join(out)

DEF = re.compile(r"^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn|struct|enum|trait|const|type|static)\s+([A-Za-z_][A-Za-z0-9_]*)|^\s+pub enum ([A-Za-z_][A-Za-z0-9_]*)\s*:", re.M)
lines = blank(open(sys.argv[1], encoding="utf-8").read()).split("\n")
groups = json.load(open(sys.argv[2]))
order = list(groups)
text = {g: "\n".join("\n".join(lines[a - 1:b]) for a, b in r) for g, r in groups.items()}
defs = {}
for g in order:
    for m in DEF.finditer(text[g]):
        defs.setdefault(m.group(1) or m.group(2), g)
for g in order:
    refs = {}
    for name, owner in defs.items():
        if owner != g and re.search(r"(?<![A-Za-z0-9_])" + re.escape(name) + r"(?![A-Za-z0-9_])", text[g]):
            refs.setdefault(owner, []).append(name)
    up = {o: v for o, v in refs.items() if order.index(o) > order.index(g)}
    print(f"== {g}: {sum(1 for d in defs.values() if d == g)} items")
    for o, v in refs.items():
        print(f"   {'UP ' if o in up else '   '}{o}: {', '.join(sorted(v))}")
