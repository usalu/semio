#!/usr/bin/env python3
"""🔑 Finds declared l3keys whose target variable is never read.

Every occurrence of the variable across the package set is classified: an occurrence directly
behind a `\\..._new:N`, `\\..._set…:N…`, `\\..._clear:N`, `\\..._zero:N` or an l3keys `.…_set:N =`
is a write, everything else is a read. A key with zero reads is declared and ignored.
Usage: python 🔧️polish-unread-keys.py <latex-dir> [file.sty ...]
"""
import re
import sys
from pathlib import Path

KEY = re.compile(
    r"^\s*([A-Za-z0-9_-]+)\s*\.(?:tl|clist|int|fp|bool|str|dim)_set:N\s*=\s*(\\[A-Za-z_@:]+)"
)
WRITE = re.compile(
    r"(?:\\[a-z]+_(?:new|g?set|g?set_eq|g?clear|g?zero|g?set_true|g?set_false|g?set_from_clist|"
    r"g?set_split|g?set_eq|g?incr|g?decr|g?add|g?sub|g?put_right|g?put_left|g?gset)[a-z_]*"
    r":[A-Za-z]*N[A-Za-z]*\s*|\.[a-z_]+_set:N\s*=\s*)$"
)


def main() -> None:
    root = Path(sys.argv[1])
    names = sys.argv[2:] or sorted(p.name for p in root.glob("semio-viz*.sty"))
    corpus = {p.name: p.read_text(encoding="utf-8") for p in root.glob("*.sty")}
    whole = "\n".join(corpus.values())
    for name in names:
        rows = []
        for line in corpus[name].split("\n"):
            hit = KEY.match(line)
            if hit is None:
                continue
            key, var = hit.groups()
            reads = 0
            for match in re.finditer(re.escape(var) + r"(?![A-Za-z_@:])", whole):
                if WRITE.search(whole[max(0, match.start() - 60):match.start()]) is None:
                    reads += 1
            if reads == 0:
                rows.append((key, var))
        if rows:
            print("### " + name)
            for key, var in rows:
                print(f"  {key:<18} {var}")


main()
