#!/usr/bin/env python3
"""🧮️ Audits every `🔬️structural-correspondence` law's hardcoded `outcomes` per leaf against the leaf's own
`🔣️.json` `outcomeClasses` (the declaration the MutationLeaf derive compiles from). The 00:41 outcome-class
normalization rewrote the declarations to applied/no-op/rejected/fatal but left these literal copies behind.
Usage: structural-outcomes.py [--write]"""
import json
import re
import subprocess
import sys
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
write = "--write" in sys.argv
tests = subprocess.run(["git", "ls-files", "*structural-correspondence/🦀️.rs"], cwd=root, capture_output=True, text=True, check=True).stdout.split()
block = re.compile(r'let directory = "([^"]+)";(.*?)let outcomes = &\[([^\]]*)\]\[\.\.\];', re.S)
for relative in tests:
    path = root / relative
    mutations = path.parent.parent.parent
    text = path.read_text(encoding="utf-8")
    changes = 0

    def settle(match):
        global changes
        declared = json.loads((mutations / match.group(1) / "🔣️.json").read_text(encoding="utf-8"))["outcomeClasses"]
        literal = [item.strip().strip('"') for item in match.group(3).split(",") if item.strip()]
        if literal == declared:
            return match.group(0)
        changes += 1
        print(f"{relative.split('/')[2]}/{match.group(1)}: {literal} -> {declared}")
        return f'let directory = "{match.group(1)}";{match.group(2)}let outcomes = &[{", ".join(json.dumps(item) for item in declared)}][..];'

    settled = block.sub(settle, text)
    if changes and write:
        path.write_text(settled, encoding="utf-8")
    print(f"{relative.split('/')[2]}: {changes} drifted")
