#!/usr/bin/env python3
# 📝️ Fills the placeholders of RESULT-RENDERER-WP1.md from the census, the oracle logs and the static texts below.
import re, subprocess, sys, json
from common import *

RES = SPLIT + "/RESULT-RENDERER-WP1.md"
text = read(RES)


def run(cmd):
    return subprocess.run(cmd, capture_output=True, text=True, cwd=os.path.dirname(os.path.abspath(__file__))).stdout


import os
census = run(["python3", "report_tables.py"]).rstrip("\n")
oracle_lines = []
for cfg in ["lib", "wasm", "test", "wasm-test"]:
    out = run(["python3", "uo_reparse.py", sys.argv[1], cfg]).strip().split("\n")
    oracle_lines.append(out)

rows = []
for out in oracle_lines:
    head = out[0]
    m = re.match(r"([\w-]+): A=(\d+) B=(\d+) new=(\d+) \(noise (\d+), signal rows (\d+)\) old=(\d+)", head)
    if m:
        rows.append(f"| `{m.group(1)}` | {m.group(2)} | {m.group(3)} | {m.group(4)} ({m.group(5)} external-crate or extern-alias count noise) | **{m.group(6)}** | {m.group(7)} |")
    else:
        rows.append(f"| ? | {head[:100]} |")
signal_detail = "\n".join(line for out in oracle_lines for line in out[1:] if line.startswith("  +SIGNAL"))
text = text.replace("TABLE_CENSUS", census)
text = text.replace("ORACLE_ROWS", "\n".join(rows))
text = text.replace("ORACLE_SIGNALS", signal_detail or "(none)")
write(RES, text)
print("\n".join(rows))
print(signal_detail)
