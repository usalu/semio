#!/usr/bin/env python3
"""🧩️ Re-exports every inline `component` wrapper of a cut family's crate roots (split lane P7): when a module's own file
lives in another family crate, the cut mounts that module's children inside an inline `component` block, and the
spelled cross-crate paths (`…::io::text::…`) need the parent to glob it like a file component.

usage: 🧩️component-globs.py <cut config> plan|apply
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
config_path, mode = sys.argv[1:3]
config = json.load(open(config_path, encoding="utf-8"))
plan = json.load(open(os.path.join(os.path.dirname(os.path.abspath(config_path)), "🗺️plan.json"), encoding="utf-8"))["family"]
artifact = os.path.join(REPO, config["artifact"])
total = 0
for name, info in plan.items():
    path = os.path.join(artifact, info["root"])
    text = open(path, encoding="utf-8").read()
    out, last, count = [], 0, 0
    for m in re.finditer(r"(?m)^(?P<pad>[ \t]*)(?:pub\s+)?mod component \{\n", text):
        depth, j = 0, m.end() - 2
        while True:
            depth += text[j] == "{"
            depth -= text[j] == "}"
            if depth == 0:
                break
            j += 1
        end = text.index("\n", j) + 1
        if text[end:].lstrip(" \t").startswith("pub use component::*;"):
            continue
        out.append(text[last:end] + f"{m.group('pad')}pub use component::*;\n")
        last, count = end, count + 1
    out.append(text[last:])
    if count:
        total += count
        print(f"{name}: +{count}")
        if mode == "apply":
            open(path, "w", encoding="utf-8").write("".join(out))
print(f"globs={total}")
