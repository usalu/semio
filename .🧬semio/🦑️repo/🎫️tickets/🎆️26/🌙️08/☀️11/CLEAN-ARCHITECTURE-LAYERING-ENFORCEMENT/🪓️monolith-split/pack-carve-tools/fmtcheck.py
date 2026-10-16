"""🎨 rustfmt parse check of every staged .rs file, plus #[path]/include! target resolution on the overlay."""
import os, re, subprocess, sys
from lib import *
from crates import *
import overlay
overlay.build(SP + "/S3")
bad = 0; n = 0
for dp, dn, fn in os.walk(STAGE):
    for f in fn:
        if not f.endswith(".rs"): continue
        p = os.path.join(dp, f); n += 1
        r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true,unstable_features=true", p], capture_output=True, text=True)
        if r.returncode != 0:
            bad += 1; print("FMT-PARSE-FAIL", os.path.relpath(p, STAGE), r.stderr[:300])
print("rustfmt parsed", n, "files, failures", bad)
# path resolution on the overlay
miss = 0
for key, *_ in CRATES:
    root = libroot(key)
    seen = []; stack = [root]
    while stack:
        rel = stack.pop()
        if rel in seen: continue
        seen.append(rel)
        t = open(SP + "/S3/" + rel, encoding="utf8").read()
        for m in list(PATH_RX.finditer(t)) + list(INCLUDE_RX.finditer(t)):
            p = os.path.normpath(os.path.join(os.path.dirname(rel), m.group(1)))
            if not os.path.exists(SP + "/S3/" + p): miss += 1; print("MISSING", rel, "->", p)
            elif p.endswith(".rs"): stack.append(p)
        for m in re.finditer(r'include_(?:str|bytes)!\(\s*"([^"]*)"\s*\)', t):
            p = os.path.normpath(os.path.join(os.path.dirname(rel), m.group(1)))
            if not os.path.exists(SP + "/S3/" + p): miss += 1; print("MISSING-EMBED", rel, "->", p)
print("missing path targets:", miss)
