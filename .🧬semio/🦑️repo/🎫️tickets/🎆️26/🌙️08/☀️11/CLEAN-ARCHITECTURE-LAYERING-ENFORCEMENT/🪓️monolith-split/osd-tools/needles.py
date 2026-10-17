"""🔍 OSD: per-file occurrence matrix of crate needles across the os-directory closure."""
import os, sys
from depscan import closure
D = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory"
files = closure(D + "/🦀️.rs")
needles = sys.argv[1:]
for f in files:
    t = open(f, encoding="utf-8").read()
    hits = {n: t.count(n) for n in needles if n in t}
    print(os.path.relpath(f, D), hits)
