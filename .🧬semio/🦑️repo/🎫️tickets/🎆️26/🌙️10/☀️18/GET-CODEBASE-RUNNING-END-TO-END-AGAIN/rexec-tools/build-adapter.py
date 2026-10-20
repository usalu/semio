#!/usr/bin/env python3
"""Builds the staged Bazel adapter copy: the live 📜️script.ts plus the anchored hermetic-execution edits. Usage: build-adapter.py [--live PATH] [--out PATH]."""
import hashlib, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from adapter_edits import patch

ROOT = "/Users/ueli/Documents/semio"
C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
live = os.path.join(ROOT, B, "📜️script.ts")
out = os.path.join(ROOT, C, "rexec-tools/staged", B, "📜️script.ts")
args = sys.argv[1:]
if "--live" in args: live = args[args.index("--live") + 1]
if "--out" in args: out = args[args.index("--out") + 1]
text = open(live, encoding="utf-8").read()
base = hashlib.sha256(text.encode()).hexdigest()
patched = patch(text)
os.makedirs(os.path.dirname(out), exist_ok=True)
open(out, "w", encoding="utf-8").write(patched)
print(f'{{"base":"{base}","out":"{out}","bytes":{len(patched.encode())}}}')
