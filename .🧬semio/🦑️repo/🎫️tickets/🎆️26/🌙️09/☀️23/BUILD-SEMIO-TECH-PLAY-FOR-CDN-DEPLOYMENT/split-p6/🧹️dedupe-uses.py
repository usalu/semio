#!/usr/bin/env python3
"""🧹️ Removes repeated identical `use` statements inside the same brace scope (E0252 after path rewrites mapped two
leaves onto one path) and collapses runs of blank lines, for every `.rs` file under the given roots.

usage: 🧹️dedupe-uses.py <root>... [--write]
"""
import os, re, sys, importlib.util
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("relocate", os.path.join(HERE, "🧭️relocate.py")); r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
write = "--write" in sys.argv
USE = re.compile(r"(?m)^([ \t]*(?:#\s*\[[^\]\n]*\][ \t]*)*(?:pub(?:\s*\([^)]*\))?\s+)?use\s+[^;{}]*(?:\{[^;]*\})?\s*;)[ \t]*\n")
changed = 0
for root in [a for a in sys.argv[1:] if not a.startswith("--")]:
    for dirpath, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in ("target", "🗑️generated", "node_modules")]
        for name in files:
            if not name.endswith(".rs"):
                continue
            path = os.path.join(dirpath, name)
            raw = open(path, encoding="utf-8").read()
            text = r.strip(raw)
            seen, drops = set(), []
            for m in USE.finditer(text):
                before = text[:m.start()]
                scope = (before.count("{") - before.count("}"), before.rfind("{"))
                key = (scope, re.sub(r"\s+", " ", raw[m.start(1):m.end(1)]).strip())
                if key in seen:
                    drops.append((m.start(), m.end()))
                seen.add(key)
            new = raw
            for a, b in reversed(drops):
                new = new[:a] + new[b:]
            new = re.sub(r"\n{4,}", "\n\n\n", new)
            if new != raw:
                changed += 1
                print(f"{os.path.relpath(path, root)}: {len(drops)} duplicate uses")
                if write:
                    open(path, "w", encoding="utf-8").write(new)
print(f"files changed: {changed}")
