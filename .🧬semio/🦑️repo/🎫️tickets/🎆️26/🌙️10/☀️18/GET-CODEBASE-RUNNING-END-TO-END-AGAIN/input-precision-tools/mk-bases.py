#!/usr/bin/env python3
# 🧷 Records the content hashes of the live files the apply edits (informational drift baseline; `apply.py --strict` refuses when one moved).
import hashlib, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from layout import L
FILES = ["🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs", "⚖️laws/🔒️cache-correctness/🟦️.ts", "⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🧬️schema/🔣️.json", "⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🔣️.json"]
json.dump({f: hashlib.sha256(open(L + "/" + f, "rb").read()).hexdigest() for f in FILES}, open(HERE + "/bases.json", "w"), ensure_ascii=False, indent=1)
print("bases written")
