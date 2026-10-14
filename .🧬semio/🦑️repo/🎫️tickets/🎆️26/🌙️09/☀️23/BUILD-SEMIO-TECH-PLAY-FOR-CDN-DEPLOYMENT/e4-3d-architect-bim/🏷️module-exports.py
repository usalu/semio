#!/usr/bin/env python3
"""🏷️ Lists the names a module file exports through `module::*`: its top-level `pub` items, its `pub mod`s, explicit
`pub use` leaves and, recursively, the exports of every `#[path]`-mounted child module it glob re-exports.
usage: 🏷️module-exports.py <module.rs>   (prints one name per line)"""
import os, re, sys
def strip(s):
    s = re.sub(r"//[^\n]*", "", s); return re.sub(r"/\*.*?\*/", "", s, flags=re.S)
def exports(path, seen=None):
    seen = seen or set()
    if path in seen: return set()
    seen.add(path)
    s = strip(open(path, encoding="utf-8").read()); out = set(); mods = {}
    for m in re.finditer(r'(?:#\[path\s*=\s*"([^"]+)"\]\s*)?^pub\s+mod\s+(\w+)\s*;', s, re.M):
        mods[m.group(2)] = os.path.join(os.path.dirname(path), m.group(1)) if m.group(1) else None; out.add(m.group(2))
    depth = 0; lines = []
    for line in s.split("\n"):
        if depth == 0: lines.append(line)
        depth += line.count("{") - line.count("}")
    top = "\n".join(lines)
    for m in re.finditer(r"^pub\s+(?:async\s+)?(?:unsafe\s+)?(?:struct|enum|type|fn|const|static|trait|union)\s+(\w+)", top, re.M): out.add(m.group(1))
    for m in re.finditer(r"^macro_rules!\s*(\w+)", top, re.M): out.add(m.group(1))
    for m in re.finditer(r"^pub\s+use\s+([^;]+);", top, re.M):
        body = m.group(1).strip()
        g = re.match(r"(\w+)::\*$", body)
        if g and mods.get(g.group(1)): out |= exports(mods[g.group(1)], seen); continue
        for leaf in re.split(r"[{},]", body):
            leaf = leaf.strip()
            if not leaf or leaf.endswith("*"): continue
            a = re.search(r"\bas\s+(\w+)$", leaf)
            out.add(a.group(1) if a else leaf.split("::")[-1])
    return out
for n in sorted(exports(sys.argv[1])): print(n)
