#!/usr/bin/env python3
"""📏️ Per-module production size of `semio-framework-plugin` against the global source admission ceiling.

For every top-level module (its own files, mounted children and `include!`d inputs, test-only mounts excluded) it
reports files, full bytes and active lines (non-blank lines once comments and `#[cfg(test)]` items are blanked) and
flags modules over 4,000 lines / 262,144 bytes / 32 files. This is an authored estimate for planning; the canonical
cfg-selected admission helper (`📦️artifacts/🏗️native-build/📥️source/🟦️.ts`) is the verdict. It compiles nothing.
Usage: unit-size.py [--facets]
"""
import importlib.util, os, re, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("facet_imports", os.path.join(HERE, "🧭️facet-imports.py"))
fi = importlib.util.module_from_spec(spec); spec.loader.exec_module(fi)
LINES, BYTES, FILES = 4000, 262144, 32
TEST = re.compile(r"#\[\s*cfg\s*\(\s*test\s*\)\s*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:(?:const|async|unsafe)\s+)*(?:use|mod|fn|impl|struct|enum|const|static|type|trait)\b[^;{]*(?:;|\{)")


def active(cl):
    out = cl
    for m in TEST.finditer(cl):
        end = m.end() if m.group(0).endswith(";") else fi.ri.skip_group(cl, m.end() - 1)
        out = out[:m.start()] + re.sub(r"[^\n]", " ", out[m.start():end]) + out[end:]
    return sum(1 for l in out.split("\n") if l.strip())


def main():
    fi.walk(f"{fi.PLUGIN}/📦️packages/🦀️rust/🦀️.rs", "crate", False, f"{fi.PLUGIN}/📦️packages/🦀️rust")
    files = defaultdict(set)
    for m in fi.modules:
        s = m.path.split("::")
        if m.test or len(s) < 3 or m.rng[0] != 0: continue
        files[s[2]].add(m.file)
    top = {}
    for m in fi.modules:
        s = m.path.split("::")
        if len(s) == 3 and s[2] not in top: top[s[2]] = m
    rows, fac = [], defaultdict(lambda: [0, 0, 0])
    for k, fs in files.items():
        lines = sum(active(fi.load(f)[1]) for f in fs)
        size = sum(os.path.getsize(f) for f in fs)
        f_ = fi.facet_of(top[k]) if k in top else "root"
        rows.append((f_, k, len(fs), lines, size))
        fac[f_][0] += len(fs); fac[f_][1] += lines; fac[f_][2] += size
    for f_, k, n, l, b in sorted(rows, key=lambda r: (fi.ORDER.index(r[0]) if r[0] in fi.ORDER else 9, -r[3])):
        over = " OVER" if l > LINES or b > BYTES or n > FILES else ""
        print(f"{f_:12} {k:34} files {n:3} lines {l:6} bytes {b:8}{over}")
    print()
    for f_, (n, l, b) in fac.items(): print(f"{f_:12} total files {n:4} lines {l:7} bytes {b:9}")


if __name__ == "__main__":
    main()
