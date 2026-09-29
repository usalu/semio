#!/usr/bin/env python3
"""🩺️ EX1: per failing law test in a cargo test capture — test name and its first panic message (one line each)."""
import re, sys
text = open(sys.argv[1], encoding="utf-8", errors="replace").read()
blocks = re.split(r"\n---- ", text)
for block in blocks[1:]:
    name = block.split(" stdout ----")[0].strip()
    m = re.search(r"panicked at ([^\n]*):\n(.*?)(?:\nnote:|\n\n|$)", block, re.S)
    if not m:
        continue
    where = m.group(1).split("/")[-1]
    msg = " ".join(m.group(2).split())
    msg = re.sub(r"\"[0-9a-f]{40,}[^\"]*\"", "\"<hex>\"", msg)
    print(f"{name} | {where} | {msg[:400]}")
