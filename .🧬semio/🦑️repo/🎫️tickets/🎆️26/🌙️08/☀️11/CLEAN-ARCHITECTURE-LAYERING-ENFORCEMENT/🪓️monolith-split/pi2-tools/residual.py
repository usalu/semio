#!/usr/bin/env python3
"""🔍️ Residual scan of the gt overlay: every `crate::<name>` of a file that belongs to a bin must name an item of that bin's root; prints the ones that do not."""
import json, re, sys, os, collections
from lib import *
from rs import blank
import tree as T
ov = json.load(open(SCRATCH + "/pi2/overlay-gt.json"))
bins = json.load(open(os.environ["PI2_MAP"] + ".bins.json"))
cm = json.load(open(os.environ["PI2_MAP"]))
res = collections.defaultdict(lambda: collections.defaultdict(list))
for K in sorted(bins):
    dirp = [k for k in ov if k.endswith("🧷️companions/") is False and f"tests-{K[-2:]}/🦀️.rs" in k]
    root = next((k for k in ov if k.endswith(f"tests-{K[-2:]}/🦀️.rs")), None)
    rtxt = ov[root]
    names = set(re.findall(r"\bmod\s+([A-Za-z_]\w*)\s*;", blank(rtxt))) | set(re.findall(r"extern\s+crate\s+\w+\s+as\s+(\w+)\s*;", rtxt)) | set(re.findall(r"\buse\s+[^;]*\bas\s+(\w+)\s*;", rtxt))
    # files of this bin: overlay texts whose path maps to K, plus included/mounted children
    files = [rp for rp in ov if ov[rp] and cm.get(rp.replace("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/", "")) == K]
    for rp in files:
        t = ov[rp]; c = blank(t)
        for m in re.finditer(r"(?<![A-Za-z0-9_:$])crate::([A-Za-z_]\w*)", c):
            n = m.group(1)
            if n not in names:
                ln = t.count("\n", 0, m.start()) + 1
                res[K][n].append((rp.replace("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/", "")[-60:], ln))
for K, d in res.items():
    print("==", K, "root names:", len(d))
    for n, locs in sorted(d.items(), key=lambda kv: -len(kv[1]))[:25]:
        print("   ", n, len(locs), locs[0])
