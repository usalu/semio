"""🕸 OSD: cross-file definition/reference graph of the os-directory schema tree and its text/binary io."""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/pos")
from posstub import mask
D = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory"
FILES = {
 "root": "🦀️.rs",
 "s-root": "🧬️schema/🦀️.rs",
 "s-session": "🧬️schema/🪪️session-authority-v1/🦀️.rs",
 "s-create": "🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs",
 "s-index": "🧬️schema/📇️document-index-v1/🦀️.rs",
 "s-checkin": "🧬️schema/📌️document-check-in-v1/🦀️.rs",
 "s-pair": "🧬️schema/🪢️canonical-checkpoint-pair-v1/🦀️.rs",
 "s-browser": "🧬️schema/🌐️browser-actor/🦀️.rs",
 "s-events": "🧬️schema/📖️events-commands-and-admin/🦀️.rs",
 "s-views": "🧬️schema/🪟️views-and-document-open/🦀️.rs",
 "s-exec": "🧬️schema/🛰️execution-target-artifacts-and-stream/🦀️.rs",
 "t-text": "🚪️io/📝️text/🦀️.rs",
 "t-session": "🚪️io/📝️text/🪪️session-authority-v1/🦀️.rs",
 "t-create": "🚪️io/📝️text/🌱️space-artifact-creation-v1/🦀️.rs",
 "t-checkin": "🚪️io/📝️text/📌️document-check-in-v1/🦀️.rs",
 "b-hash": "🚪️io/🧱️binary/🪪️artifact-hash/🦀️.rs",
 "b-digest": "🚪️io/🧱️binary/🔐️descriptor-digest/🦀️.rs",
 "b-pair": "🚪️io/🧱️binary/🪢️checkpoint-pair/🦀️.rs",
}
DEF = re.compile(r"^\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:struct|enum|type|const|trait|static)\s+([A-Z]\w*)", re.M)
text = {k: mask(open(os.path.join(D, v), encoding="utf-8").read()) for k, v in FILES.items()}
defs = collections.defaultdict(set)
for k, t in text.items():
    if k.startswith("s-") or k == "root":
        for m in DEF.finditer(t):
            defs[k].add(m.group(1))
owner = {}
for k, names in defs.items():
    for n in names:
        owner.setdefault(n, set()).add(k)
edges = collections.defaultdict(lambda: collections.defaultdict(set))
for k, t in text.items():
    words = set(re.findall(r"\b\w+\b", t))
    for n, ow in owner.items():
        if n in words:
            for o in ow:
                if o != k:
                    edges[k][o].add(n)
for k in FILES:
    print(f"{k:10s} defs={len(defs.get(k,()))} uses ->", {o: len(ns) for o, ns in sorted(edges[k].items())})
if len(sys.argv) > 1:
    a, b = sys.argv[1], sys.argv[2]
    print(a, "uses from", b, sorted(edges[a][b]))
