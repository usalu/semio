#!/usr/bin/env python3
# 🧪 Writes the live project-assembly text with the receipt-backed program analysis applied to /private/tmp/ip/assembly2.mjs (syntax-checked by the caller with `node --check`).
import importlib.util, os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from layout import L
spec = importlib.util.spec_from_file_location("hook_patch2", HERE + "/hook-patch2.py"); module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
text = open(L + "/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs", encoding="utf8").read()
patched, changed = module.assembly(text)
os.makedirs("/private/tmp/ip", exist_ok=True)
open("/private/tmp/ip/assembly2.mjs", "w", encoding="utf8").write(patched)
print("changed" if changed else "already applied", len(patched) - len(text))
