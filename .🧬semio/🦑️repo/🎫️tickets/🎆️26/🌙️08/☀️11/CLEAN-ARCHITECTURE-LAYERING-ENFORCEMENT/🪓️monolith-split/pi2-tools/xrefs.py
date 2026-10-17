#!/usr/bin/env python3
"""🕸️ Cross-top references: facade-level re-exports of test host items (`pub(crate) use super::<host>::...`) and which test roots use the re-exported names."""
import json, re, sys, collections
from lib import *
from rs import blank
import tree as T
tag = sys.argv[1]
tops = json.load(open(GEN + f"/pi2w7-{tag}-tops.json"))
hubs = [PLUG + "/🦀️.rs"] + [PLUG + "/🪝️mounts/" + h + "/🦀️.rs" for h in sorted(os.listdir(PLUG + "/🪝️mounts"))] if (os := __import__("os")) else []
mods = {t["mod"] for t in tops}
top_of_file = {}
for t in tops:
    for f in t["files"]:
        top_of_file[f] = t["mod"]
texts = {f: read(PLUG + "/" + f) for t in tops for f in t["files"]}
RX = re.compile(r"((?:#\[[^\]]*\]\s*)*)pub(?:\([a-z]+\))?\s+use\s+(?:super|crate)::([A-Za-z_0-9]+)::(.+?);", re.S)
edges = collections.defaultdict(set)
exports = {}
for h in hubs:
    tx = read(h)
    for m in RX.finditer(blank(tx)):
        host = m.group(2)
        if host not in mods:
            continue
        body = m.group(3)
        names = re.findall(r"[A-Za-z_][A-Za-z0-9_]*", body.split("::")[-1] if "{" not in body else body[body.index("{"):])
        names = [n for n in names if n not in ("as", "self")]
        exports[(rel(h), host)] = names
        for f, tt in texts.items():
            cl = blank(tt)
            used = [n for n in names if len(n) > 8 and n != "fixture" and re.search(r"\b" + re.escape(n) + r"\b", cl)]
            if used and top_of_file[f] != host:
                edges[(top_of_file[f], host)].add(tuple(sorted(used))[:3])
for (u, h), s in sorted(edges.items()):
    print(f"{u:40s} -> {h:36s} {sorted(s)[:2]}")
print("exports", {k: len(v) for k, v in exports.items()})
