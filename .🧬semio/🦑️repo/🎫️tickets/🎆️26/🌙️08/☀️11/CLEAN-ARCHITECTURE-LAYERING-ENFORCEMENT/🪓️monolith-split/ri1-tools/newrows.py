#!/usr/bin/env python3
# 🔍️ Signal rows of one oracle tag/config, optionally minus the rows of a baseline tag: newrows.py <tag> <config> [<baseline-tag>]
import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import uo
from common import *
tag, cfg = sys.argv[1], sys.argv[2]
base = sys.argv[3] if len(sys.argv) > 3 else None
def rows(t):
    t, _, c2 = t.partition(":")
    sb = f"{RS}/sb-{t}-crates/A"
    err = open(f"{GEN}/ri1-oracle-{t}-A-{c2 or cfg}.jsonl").read()
    c = uo.parse(err, sb)
    out = collections.Counter()
    for k, v in c.items():
        if uo.noise(k): continue
        out[k[:4]] += v
    return out
a = rows(tag)
b = rows(base) if base else collections.Counter()
new = a - b
print(f"{tag}/{cfg}: signal={sum(a.values())} baseline={sum(b.values())} new={sum(new.values())} gone={sum((b-a).values())}")
for k, v in sorted(new.items(), key=lambda kv: (kv[0][1], kv[0][2])):
    print("  +", v, k[0], k[1][:170], "|", k[2][:40])
for k, v in sorted((b - a).items()):
    print("  -", v, k[0], k[1][:150], "|", k[2][:40])
