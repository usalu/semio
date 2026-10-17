"""Rewrites real.json: every renderer crate whose root is more than a skeleton (the union oracle mounts these as modules)."""
import json
from lib import *
crates = crate_table()
FORCED = {"fixtures"}
real = []
for suffix, folder in crates.items():
    text = read(T + "/" + folder + "/🦀️.rs")
    body = [l for l in text.split("\n") if l.strip() and not l.lstrip().startswith("//")]
    if len(body) > 3 or suffix in FORCED:
        real.append(suffix)
json.dump(sorted(real), open(HERE + "/real.json", "w"), ensure_ascii=False)
print(len(real), "real crates;", len(crates), "total")
