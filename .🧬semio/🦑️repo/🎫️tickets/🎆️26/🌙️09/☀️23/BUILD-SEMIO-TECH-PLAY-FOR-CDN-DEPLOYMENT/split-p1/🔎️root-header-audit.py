#!/usr/bin/env python3
"""🔎️ Checks every crate root (lib and `[[test]]`) under a root (lane P1): inner attributes (`#![…]`) only after module docs,
and `extern crate` items only at brace depth 0. usage: 🔎️root-header-audit.py [root]
"""
import importlib.util, os, re, sys
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
ROOT = sys.argv[1] if len(sys.argv) > 1 else L.PLUGIN
bad = 0
for r, dirs, fs in os.walk(ROOT):
    dirs[:] = [x for x in dirs if x not in ("target", "node_modules")]
    if "Cargo.toml" not in fs or "📦️packages" not in r: continue
    t = L.rd(os.path.join(r, "Cargo.toml"))
    roots = [os.path.normpath(os.path.join(r, p)) for p in re.findall(r'(?m)^path = "([^"]+\.rs)"', t)]
    for root in roots:
        if not os.path.exists(root): continue
        raw = L.rd(root); s = L.strip(raw); problems = []
        seen_code = False
        for line in s.splitlines():
            st = line.strip()
            if not st: continue
            if st.startswith("#!["):
                if seen_code: problems.append("inner attribute after code"); break
            else: seen_code = True
        depth = 0
        for m in re.finditer(r"[{}]|\bextern\s+crate\b", s):
            if m.group(0) == "{": depth += 1
            elif m.group(0) == "}": depth -= 1
            elif depth: problems.append(f"extern crate at depth {depth} (line {s[:m.start()].count(chr(10)) + 1})")
        if problems: bad += 1; print(os.path.relpath(root, L.PLUGIN), "|", "; ".join(problems))
print(bad, "roots with header problems")
