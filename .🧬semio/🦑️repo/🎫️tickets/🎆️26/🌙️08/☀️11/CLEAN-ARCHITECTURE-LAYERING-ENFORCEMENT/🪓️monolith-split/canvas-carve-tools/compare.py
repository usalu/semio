#!/usr/bin/env python3
# 🔬️ Compares the baseline error multiset with the sum over the split crates, by message line.
import collections
import os
import re
import sys

SCRATCH = os.environ["SCRATCH"]
out = SCRATCH + "/probe"
configs = sys.argv[1].split(",") if len(sys.argv) > 1 else ["native", "native-test", "wasm", "wasm-test", "wasip2", "wasip2-test"]


def heads(path):
    text = open(path, encoding="utf8").read()
    rows = collections.Counter()
    for block in re.split(r"\n(?=error)", text):
        line = block.split("\n", 1)[0]
        if not line.startswith("error") or line.startswith("error: aborting") or line.startswith("error: could not compile"):
            continue
        rows[line] += 1
    return rows


for config in configs:
    base = heads(f"{out}/baseline-{config}.err")
    split = collections.Counter()
    for key in ["scene", "camera", "icons", "assets", "facade"]:
        split += heads(f"{out}/{key}-{config}.err")
    print(f"== {config}: baseline {sum(base.values())} split {sum(split.values())}")
    for message in sorted(set(base) | set(split)):
        if base[message] != split[message]:
            print(f"   {base[message]:3d} -> {split[message]:3d}  {message}")
