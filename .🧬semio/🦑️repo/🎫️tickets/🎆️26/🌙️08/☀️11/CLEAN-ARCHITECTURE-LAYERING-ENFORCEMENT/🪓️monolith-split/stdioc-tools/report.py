#!/usr/bin/env python3
# 🧾️ Prints the internal-error multiset of the baseline and of every split crate per configuration.
import os, sys, collections
sys.path.insert(0, os.path.dirname(__file__))
import probe
from lib import *
cfgs = sys.argv[1].split(",")
for cfg in cfgs:
    base = probe.internal_heads(open(f"{probe.SCRATCH}/probe/baseline-{cfg}.err").read())
    print(f"== {cfg}: baseline internal {sum(base.values())}")
    for k, v in sorted(base.items()): print("   B", v, k)
    total = collections.Counter()
    for key in [c["key"] for c in CRATES] + ["facade"]:
        h = probe.internal_heads(open(f"{probe.SCRATCH}/probe/{key}-{cfg}.err").read())
        total += h
        for k, v in sorted(h.items()): print(f"   {key}:", v, k)
