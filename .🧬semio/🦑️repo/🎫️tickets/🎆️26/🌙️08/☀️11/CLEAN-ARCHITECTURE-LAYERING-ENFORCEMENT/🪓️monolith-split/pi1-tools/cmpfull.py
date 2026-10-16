#!/usr/bin/env python3
"""📊️ Re-reads the rustc outputs of a full oracle run and prints the A/B difference: cmpfull.py <tag> <config> [--extra-real a,b] ."""
import collections, json, sys
from lib import *
import uo

tag, cfg = sys.argv[1], sys.argv[2]
extra = []
for i, a in enumerate(sys.argv):
    if a == "--extra-real":
        extra = sys.argv[i + 1].split(",")
real = uo.real_crates()
for k in extra:
    real[k] = ""
rxs = uo.noise_rx(set(real))
def load(side):
    err = open(f"{GEN}/pi1-oracle-{tag}-{side}-{cfg}.jsonl").read()
    sb = f"{uo.SBROOT}/sb-{tag}/{side}"
    return uo.parse(err, sb)
a, b = load("A"), load("B")
fa, fb = collections.Counter(), collections.Counter()
where = {}
for k, v in a.items():
    fa[uo.norm(k)] += v
    where.setdefault(uo.norm(k), collections.Counter())[uo.short(k[4])] += v
for k, v in b.items():
    fb[uo.norm(k)] += v
fa = collections.Counter({k: v for k, v in fa.items() if not uo.is_noise(k, rxs)})
fb = collections.Counter({k: v for k, v in fb.items() if not uo.is_noise(k, rxs)})
new, old = fa - fb, fb - fa
print(f"== {cfg}: signal A={sum(fa.values())} B={sum(fb.values())} onlyNew={sum(new.values())} onlyOld={sum(old.values())}")
for k, v in sorted(new.items(), key=lambda kv: (-kv[1], kv[0])):
    print("  +new", v, k[0], k[1][:200], "|", k[2][:50], "|", dict(where[k].most_common(3)))
for k, v in sorted(old.items(), key=lambda kv: (-kv[1], kv[0])):
    print("  -old", v, k[0], k[1][:200], "|", k[2][:50])
