#!/usr/bin/env python3
# 🔍️ Re-reads a finished oracle run (A jsonl of <tag>/<config> against the cached baseline) and prints the new / vanished error rows.
import collections, sys, os
import uo
from lib import *

tag, config = sys.argv[1], sys.argv[2]
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 60
A = open(f"{GEN}/pwp1-oracle-{tag}-A-{config}.jsonl").read()
B = open(f"{GEN}/pwp1-oracle-Bcache-{config}.jsonl").read()
a, b = uo.parse(A, f"{SB}/sb-{tag}/A"), uo.parse(B, f"{SB}/sb-B")
fa, fb, where = collections.Counter(), collections.Counter(), {}
for k, v in a.items():
    fa[uo.norm(k)] += v
    where.setdefault(uo.norm(k), collections.Counter())[uo.short(k[4])] += v
for k, v in b.items():
    fb[uo.norm(k)] += v
fa = collections.Counter({k: v for k, v in fa.items() if not uo.is_noise(k)})
fb = collections.Counter({k: v for k, v in fb.items() if not uo.is_noise(k)})
new, old = fa - fb, fb - fa
print(f"{tag}/{config}: A={sum(a.values())} B={sum(b.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())}")
for k, v in sorted(new.items(), key=lambda kv: (-kv[1], kv[0]))[:limit]:
    print("  +new", v, k[0], k[1][:170], "|", k[2][:50], "|", dict(where[k].most_common(3)))
for k, v in sorted(old.items(), key=lambda kv: (-kv[1], kv[0]))[:limit]:
    print("  -old", v, k[0], k[1][:170], "|", k[2][:50])
