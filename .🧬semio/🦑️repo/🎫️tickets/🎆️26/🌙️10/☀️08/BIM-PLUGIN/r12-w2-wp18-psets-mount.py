"""🪢️ WP-18: mounts the leaves and cases of `r12-w2-wp18-psets-leaves.ts` in the artifact root `🦀️.rs`.

Run from the repo root after `bun r12-w2-wp18-psets-leaves.ts`: `python r12-w2-wp18-psets-mount.py`. Idempotent: a leaf module that is mounted already only gets the case modules it lacks; the new leaves go before
`//#endregion 🔖️Leaves`; the extra cases of the existing leaves in `case-mounts.json` go before the closing brace of their `pub mod`.
"""
import json
import os
import re
import sys

sys.stdout.reconfigure(encoding="utf-8")
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.join("✏️s", "🔌️plugins", "🏙️bim", "🗿️artifacts", "🏢️model", "🦀️.rs")
OUT = os.path.join(HERE, "🗑️generated", "w2-wp18-psets")


def put(path, text):
    try:
        with open(path, "w", encoding="utf8", newline="") as handle:
            handle.write(text)
    except OSError:
        tmp = path + ".wp18tmp"
        with open(tmp, "w", encoding="utf8", newline="") as handle:
            handle.write(text)
        os.replace(tmp, path)


source = open(ROOT, encoding="utf8", newline="").read()
crlf = "\r\n" in source
if crlf:
    source = source.replace("\r\n", "\n")


def add_cases(source, module, text):
    start = source.index(f"pub mod {module} {{")
    end = source.index("\n                        }\n", start)
    present = source[start:end]
    lines = [line for line in text.split("\n") if line.strip()]
    additions = []
    for index in range(0, len(lines) - 2, 3):
        group = "\n".join(lines[index : index + 3])
        name = re.search(r"mod (tests_\w+);", group).group(1)
        if f"mod {name};" not in present:
            additions.append(group + "\n")
    if not additions:
        return source, 0
    return source[:end] + "\n" + "".join(additions).rstrip("\n") + source[end:], len(additions)


mounts = open(os.path.join(OUT, "mounts.txt"), encoding="utf8").read()
blocks = re.split(r"(?=^ {24}#\[path = \"\.\"\]\n)", mounts, flags=re.M)
fresh = []
added = 0
for block in blocks:
    found = re.search(r"pub mod (\w+) \{", block)
    if not block.strip() or not found:
        continue
    module = found.group(1)
    if f"pub mod {module} {{" in source:
        tests = "\n".join(re.findall(r"^ {28}#\[cfg\(test\)\]\n {28}#\[path = \"[^\"]+\"\]\n {28}mod tests_\w+;$", block, flags=re.M))
        source, count = add_cases(source, module, tests)
        added += count
    else:
        fresh.append(block)
anchor = "                        //#endregion 🔖️Leaves"
assert source.count(anchor) == 1, "the leaves anchor"
source = source.replace(anchor, "".join(fresh) + anchor)

cases = json.load(open(os.path.join(OUT, "case-mounts.json"), encoding="utf8"))
for module, text in cases.items():
    source, count = add_cases(source, module, text)
    added += count

put(ROOT, source.replace("\n", "\r\n") if crlf else source)
print(f"mounted {len(fresh)} leaf modules and {added} case modules")
