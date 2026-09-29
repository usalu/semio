#!/usr/bin/env python3
"""🦀️ R10 T5: per-file rustfmt state of the Rust files in a list — `clean` (rustfmt's stdin output equals the file),
`dirty` (formats differently) or `parse-error` — so a codemod is proven not to break formatting: every file clean before
must stay clean after. Only the file's own text is formatted (stdin: no child modules followed).
usage: python3 rustfmt-state.py <list-file> <out.json>"""
import json
import subprocess
import sys

ROOT = "/Users/ueli/Documents/semio"
state = {}
for path in [line for line in open(sys.argv[1], encoding="utf-8").read().splitlines() if line.endswith(".rs")]:
    text = open(f"{ROOT}/{path}", encoding="utf-8").read()
    run = subprocess.run(["rustfmt", "--config-path", f"{ROOT}/rustfmt.toml", "--emit", "stdout"], input=text, capture_output=True, text=True)
    state[path] = "parse-error" if run.returncode != 0 else ("clean" if run.stdout == text else "dirty")
json.dump(state, open(sys.argv[2], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print({kind: sum(1 for value in state.values() if value == kind) for kind in ("clean", "dirty", "parse-error")})
