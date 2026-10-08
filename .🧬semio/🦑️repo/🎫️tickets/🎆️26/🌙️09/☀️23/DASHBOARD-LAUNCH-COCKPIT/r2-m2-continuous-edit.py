#!/usr/bin/env python3
"""M-2 round 2b: adds `"continuous": true` to the named Nx targets by a textual insert after the target's own `"cache"` key.
Re-reads each manifest right before writing and retries when the file refuses the write. Usage (repository root): python r2-m2-continuous-edit.py"""
import json
import re
import sys
import time

sys.stdout.reconfigure(encoding="utf-8")
EDITS = [
    ("📋️project.json", ["dev", "dev-mcp-engine"]),
    ("🌎️hub/📦️packages/🦀️rust/📋️project.json", ["scoped-presence-browser-serve"]),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json", ["serve-hold"]),
]


def insert(text, target):
    head = re.search(r'^(?P<indent> +)"' + re.escape(target) + r'": \{\n', text, re.M)
    assert head, target
    indent = head.group("indent")
    inner = indent + "  "
    end = re.compile(r"^" + indent + r"\}", re.M).search(text, head.end())
    body = text[head.end():end.start()]
    assert '"continuous"' not in re.sub(r'^' + inner + r'"options": \{.*?^' + inner + r'\}', "", body, flags=re.M | re.S), target
    cache = re.search(r"^" + inner + r'"cache": (true|false)(,?)\n', body, re.M)
    assert cache, target
    line = cache.group(0)
    new = line if cache.group(2) else line.rstrip("\n") + ",\n"
    new += inner + '"continuous": true' + ("," if cache.group(2) else "") + "\n"
    return text[: head.end() + cache.start()] + new + text[head.end() + cache.end():]


for file, targets in EDITS:
    for attempt in range(15):
        text = open(file, encoding="utf8", newline="").read()
        for target in targets:
            text = insert(text, target)
        json.loads(text)
        try:
            open(file, "w", encoding="utf8", newline="").write(text)
            break
        except OSError as error:
            print("retry", file[-30:], error)
            time.sleep(2)
    else:
        raise SystemExit(f"could not write {file}")
    print("edited", file[-40:], targets)
