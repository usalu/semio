#!/usr/bin/env python3
# 🧭️ `crate::X` first segments of a carved crate against the names its crate root defines (items, modules, `use` aliases, extern crates, macros).
import re, sys
from lib import *
import wp1files, defs
from rs import blank, uses
from crates_wp1 import ROOT, ORDER


def root_names(crate):
    root = PLUG + "/" + ROOT[crate] + "/🦀️.rs"
    names = set()
    files = [root]
    # included files share the root namespace
    for f, parent in wp1files.tree(root):
        if parent == root and f != root:
            text = read(root)
            if re.search(r'include!\(\s*"[^"]*' + re.escape(os.path.basename(os.path.dirname(f))), text):
                files.append(f)
    for f in files:
        t = read(f)
        names |= {d.name for d in defs.scan(t) if d.kind == "item"}
        for cfg, vis, path, local, glob in uses(t, top_only=True):
            if local:
                names.add(local)
            if glob:
                names.add("*")
        names |= set(re.findall(r"\bextern\s+crate\s+[A-Za-z0-9_]+\s+as\s+([A-Za-z0-9_]+)", blank(t)))
        names |= set(re.findall(r"\bmacro_rules!\s+([A-Za-z_0-9]+)", blank(t)))
        names |= set(re.findall(r"\bthread_local!\s*\{[^}]*?static\s+([A-Za-z_0-9]+)", blank(t)))
    return names


bad = 0
for crate in (sys.argv[1:] or ORDER):
    root = PLUG + "/" + ROOT[crate] + "/🦀️.rs"
    names = root_names(crate)
    seen = {}
    for f, _ in wp1files.tree(root):
        clean = blank(read(f))
        for m in re.finditer(r"(?<![A-Za-z0-9_$:])crate::\{?\s*([A-Za-z_][A-Za-z0-9_]*)", clean):
            seen.setdefault(m.group(1), set()).add(f.replace(PLUG + "/", ""))
        for m in re.finditer(r"(?<![A-Za-z0-9_$:])crate::\{([^}]*)\}", clean):
            for part in m.group(1).split(","):
                n = part.strip().split("::")[0].strip()
                if n:
                    seen.setdefault(n, set()).add(f.replace(PLUG + "/", ""))
    missing = {n: fs for n, fs in seen.items() if n not in names}
    print(f"== {crate}: crate:: heads {len(seen)}, undefined at root: {sorted(missing)}")
    for n, fs in missing.items():
        print("    ", n, sorted(fs)[:3])
        bad += 1
print("problems", bad)
