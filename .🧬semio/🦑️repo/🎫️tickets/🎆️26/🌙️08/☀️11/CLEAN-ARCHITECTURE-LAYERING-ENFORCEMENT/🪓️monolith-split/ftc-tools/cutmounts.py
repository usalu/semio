"""✂ Remove `#[cfg(test)]` + `#[path = "…"]` + `[pub[(…)]] mod <name>;` mount blocks from lib files.
usage: cutmounts.py <names,comma> <file>...   (expects exactly one block per file per listed name unless --any)
prints each removed block; refuses when the count differs."""
import os, re, sys
names = sys.argv[1].split(",")
any_ = "--any" in sys.argv
files = [f for f in sys.argv[2:] if not f.startswith("--")]
for f in files:
    s = open(f, encoding="utf-8").read()
    total = 0
    for n in names:
        rx = re.compile(r'(?:\n)?[ \t]*#\[cfg\(test\)\]\n[ \t]*#\[path\s*=\s*"[^"]+"\]\n[ \t]*(?:pub(?:\([^)]*\))?\s+)?mod\s+' + re.escape(n) + r';(?:\n|$)')
        found = rx.findall(s)
        if len(found) != 1 and not any_:
            raise SystemExit(f"{f}: expected exactly one mount block for {n}, found {len(found)}")
        total += len(found)
        for b in found:
            print(f"cut {os.path.basename(os.path.dirname(os.path.abspath(f)))}: {b.strip()!r}")
        s = rx.sub("\n", s, count=1) if found else s
    open(f, "w", encoding="utf-8").write(s)
