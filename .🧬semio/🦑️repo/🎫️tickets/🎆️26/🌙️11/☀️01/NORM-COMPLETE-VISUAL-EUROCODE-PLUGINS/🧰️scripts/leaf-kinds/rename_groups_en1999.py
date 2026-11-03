"""Renames the numbered en1999 group crates (`-2`) to family names across dirs, Cargo, script.ts and build-owners. Usage: python -I rename_groups_en1999.py"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *

ART = NORM + "/🗿️artifacts/🪶️en1999"
MAP = [("connection-action-2", "connection-action-components"), ("cold-formed-action-2", "cold-formed-action-components"), ("member-action-2", "member-action-components"), ("shell-action-2", "shell-action-components"), ("section-element-2", "section-element-stiffeners"), ("connection-2", "connection-fasteners"), ("member-2", "member-web-details")]
files = [OWNERS]
for d, _, fs in os.walk(ART):
    files += [os.path.join(d, f) for f in fs if f.endswith((".rs", ".toml", ".ts", ".json", ".lock"))]
n = 0
for f in files:
    t = rd(f)
    u = t
    for old, new in MAP:
        for a, b in ((old, new), (old.replace("-", "_"), new.replace("-", "_"))):
            u = re.sub(r"(?<![A-Za-z0-9])%s(?![A-Za-z0-9])" % re.escape(a), b, u)
    if u != t:
        open(f, "w", encoding="utf-8").write(u)
        n += 1
print("files", n)
base = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
for x in sorted(os.listdir(base)):
    for old, new in MAP:
        if x.endswith(old) and re.sub(r"^[^a-z]+", "", x) == old:
            os.rename(base + "/" + x, base + "/" + x[: -len(old)] + new)
            print(x, "->", new)
