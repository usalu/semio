"""Prints the Cargo package owning each given source path (nearest 📦️packages/🦀️rust/Cargo.toml whose lib reaches it)."""
import os, re, sys
root = "/Users/ueli/Documents/semio/"
def crate(path):
    d = os.path.dirname(os.path.abspath(root + path if not path.startswith("/") else path))
    while d.startswith(root):
        m = d + "/📦️packages/🦀️rust/Cargo.toml"
        if os.path.exists(m):
            return re.search(r'(?m)^name\s*=\s*"([^"]+)"', open(m, encoding="utf-8").read()).group(1)
        d = os.path.dirname(d)
    return None
seen = []
for line in sys.stdin:
    c = crate(line.strip())
    if c and c not in seen: seen.append(c)
print("\n".join(seen))
