"""🪢️ Wave W07: mounts the leaves and cases of `r10-w07-zones-leaves.ts` in the artifact root `🦀️.rs`.

Run from the repo root after `bun r10-w07-zones-leaves.ts`: `python r10-w07-zones-mount.py`. Idempotent: a leaf module or a case module that is already mounted is skipped.
The new leaves go before `//#endregion 🔖️Leaves`; the new cases of an existing leaf go before the closing brace of its `pub mod`.
"""
import json
import os
import re
import sys

sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.join("✏️s", "🔌️plugins", "🏙️bim", "🗿️artifacts", "🏢️model", "🦀️.rs")
OUT = os.path.join(HERE, "🗑️generated", "w07-zones")


def put(path, text):
    try:
        with open(path, "w", encoding="utf8", newline="") as handle:
            handle.write(text)
    except OSError:
        tmp = path + ".w07tmp"
        with open(tmp, "w", encoding="utf8", newline="") as handle:
            handle.write(text)
        os.replace(tmp, path)


source = open(ROOT, encoding="utf8", newline="").read()
crlf = "\r\n" in source
if crlf:
    source = source.replace("\r\n", "\n")

mounts = open(os.path.join(OUT, "mounts.txt"), encoding="utf8").read()
blocks = re.split(r"(?=^ {24}#\[path = \"\.\"\]\n)", mounts, flags=re.M)
fresh = [block for block in blocks if block.strip() and re.search(r"pub mod (\w+) \{", block) and f"pub mod {re.search(r'pub mod (\w+) {', block).group(1)} {{" not in source]
anchor = "                        //#endregion 🔖️Leaves"
assert source.count(anchor) == 1, "the leaves anchor"
source = source.replace(anchor, "".join(fresh) + anchor)

cases = json.load(open(os.path.join(OUT, "case-mounts.json"), encoding="utf8"))
for module, text in cases.items():
    start = source.index(f"pub mod {module} {{")
    end = source.index("\n                        }\n", start)
    present = source[start:end]
    lines = text.split("\n")
    additions = []
    for index in range(0, len(lines) - 2, 3):
        group = "\n".join(lines[index : index + 3])
        name = re.search(r"mod (tests_\w+);", group).group(1)
        if f"mod {name};" not in present:
            additions.append(group + "\n")
    source = source[:end] + "\n" + "".join(additions).rstrip("\n") + source[end:] if additions else source

put(ROOT, source.replace("\n", "\r\n") if crlf else source)
print(f"mounted {len(fresh)} leaf modules and the new cases of {', '.join(cases)}")
