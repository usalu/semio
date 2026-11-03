"""🔎 Every `semio_*` / known workspace crate named at a path root or in a `use` must be a [dependencies]/[dev-dependencies] entry (or extern-crate alias) of the crate whose source tree contains it."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import scan_resolve as S
from rs import *
S.load_crates()
code = sys.argv[1]
adir = ART + "/" + CODE[code]
ROOTS = re.compile(r'(?<![\w:$])(semio_\w+)\s*::')
USEROOT = re.compile(r'\buse\s+(?:\{[^}]*\}|(\w+))')
for lib, c in sorted(S.CRATES.items()):
    if not c["dir"].startswith(adir): continue
    files, parent = S.tree(c["root"])
    deps = set(c["deps"]) | {lib}
    al = S.aliases_of(files)
    for f, src in files.items():
        seen = set()
        for m in ROOTS.finditer(src):
            k = m.group(1)
            if k in deps or k in al or k in seen: continue
            seen.add(k)
            ok = k in S.CRATES
            print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: `{k}::` not a dependency of {c['name']}" + ("" if ok else " (crate unknown)"))
