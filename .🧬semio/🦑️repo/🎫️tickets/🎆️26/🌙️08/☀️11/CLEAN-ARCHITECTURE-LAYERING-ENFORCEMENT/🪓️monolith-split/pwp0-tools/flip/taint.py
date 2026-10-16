# 🧪️ Upward / undeclared reference census per crate, gated (test or testing feature) versus ungated.
import re, collections, sys
from common import *
import model
from modules import module_map
from index import Tree
from rs import blank
pt = model.pt
mm = module_map()
rank = {k: v["rank"] for k, v in CRATES.items()}
deps = {k: set(v["deps"]) for k, v in CRATES.items()}
def mod_crate(name):
    m = mm.get(name)
    if not m: return None
    if m["kind"] == "file": return pt.file_crate(m["file"])
    return m["lib"].replace("semio_framework_plugin_", "").replace("_", "-")
o = model.owners()
want = sys.argv[1:] or [k for k in CRATES if CRATES[k]["kind"] == "lower"]
GATE = re.compile(r"cfg\((?:any\()?[^)]*(?:test|artifact-app-testing|media-turn-testing|reserved-producer-testing|original-actor-context-testing)")
for k in want:
    if k not in o: continue
    res = collections.defaultdict(lambda: [0, 0])
    for f in o[k]:
        text = read(PLUG + "/" + f); clean = blank(text); lines = text.split("\n")
        testfile = "/🧪️tests/" in f
        for m in re.finditer(r"(?<![A-Za-z0-9_$:])crate::([A-Za-z_]\w*)", clean):
            c = mod_crate(m.group(1))
            if c is None: c = "ROOTNAME:" + m.group(1)
            if c == k: continue
            ln = clean.count("\n", 0, m.start())
            prev = "\n".join(lines[max(0, ln - 2):ln + 1])
            gated = testfile or bool(GATE.search(prev))
            res[c][0 if not gated else 1] += 1
    bad = {c: v for c, v in res.items() if c.startswith(("TOP", "UNMAPPED", "ROOTNAME")) or c not in rank or rank[c] >= rank[k] or c not in deps[k]}
    print(f"== {k}: " + ", ".join(f"{c}={v[0]}/{v[1]}" for c, v in sorted(bad.items())))
