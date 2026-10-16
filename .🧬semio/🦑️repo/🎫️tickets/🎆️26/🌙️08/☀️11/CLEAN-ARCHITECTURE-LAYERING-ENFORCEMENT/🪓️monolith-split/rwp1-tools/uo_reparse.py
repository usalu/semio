#!/usr/bin/env python3
# 🔁️ Re-reads the saved jsonl of a full union-oracle run and prints the signal rows (usage: uo_reparse.py <tag> <config>).
import collections, re, sys
from common import *
import uo

tag, name = sys.argv[1], sys.argv[2]
base = f"{RS}/sb-{tag}-full"
a = uo.parse(open(f"{GEN}/rwp1-oracle-{tag}-A-{name}.jsonl").read(), base + "/A")
b = uo.parse(open(f"{GEN}/rwp1-oracle-{tag}-B-{name}.jsonl").read(), base + "/B")
fa, fb, where = collections.Counter(), collections.Counter(), {}
for k, v in a.items():
    fa[k[:4]] += v
    where.setdefault(k[:4], collections.Counter())[uo.short(k[4])] += v
for k, v in b.items():
    fb[k[:4]] += v
new, old = fa - fb, fb - fa
noise_new = {k: v for k, v in new.items() if uo.noise(k + ("",)) or re.search(r"unresolved import `[a-z_0-9]+`$", k[1]) and "renderer_wgpu" not in k[1]}
sig = {k: v for k, v in new.items() if k not in noise_new}
print(f"{name}: A={sum(a.values())} B={sum(b.values())} new={sum(new.values())} (noise {sum(noise_new.values())}, signal rows {len(sig)}) old={sum(old.values())}")
for k, v in sorted(sig.items(), key=lambda kv: -kv[1])[:30]:
    print("  +SIGNAL", v, k[0], k[1][:120], "|", k[2][:40], "|", dict(where[k].most_common(2)))
oldsig = {k: v for k, v in old.items() if not uo.noise(k + ("",))}
for k, v in sorted(oldsig.items(), key=lambda kv: -kv[1])[:10]:
    print("  -old", v, k[0], k[1][:120], "|", k[2][:40])
