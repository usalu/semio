#!/usr/bin/env python3
# 🧮️ Markdown census table from the TSV of census-wp1.ts.
import collections, sys
from lib import *
rows = [l.split("\t") for l in open(GEN + "/pwp1-census-rows.tsv", encoding="utf8").read().strip().split("\n")[1:] if l.strip()]
d = collections.OrderedDict()
for crate, target, variant, files, bytes_, lines, emb, verdict in rows:
    d.setdefault(crate, {})[(target, variant)] = (files, bytes_, lines, emb, verdict)
order = ["lib", "lib+features", "test", "test+features"]
print("| crate | variant | native | wasm32-wasip2 | wasm32-unknown-unknown | worst verdict |")
print("|---|---|---|---|---|---|")
for crate, m in d.items():
    for v in order:
        cells, verdicts = [], []
        for t in ("native", "wasip2", "wasm"):
            r = m.get((t, v))
            if not r:
                cells.append("-")
                continue
            cells.append(f"{r[0]} / {int(r[1]):,} / {int(r[2]):,}" + (f" ({r[3]} emb)" if r[3] not in ("0",) else ""))
            verdicts.append(r[4])
        worst = "OVER-GATE" if "OVER-GATE" in verdicts else "over-target" if "over-target" in verdicts else "ok"
        print(f"| `{crate}` | {v} | " + " | ".join(cells) + f" | {worst} |")
