#!/usr/bin/env python3
"""🧭️ EX1: which editors the prepared set resolves through `editor_example_snapshot::<E>` publish their examples ONLY through
their subset declaration (no `ArtifactEditor::examples()`): their examples resolve after plugin assembly registered them, so
every unit test that loads one of their examples without assembling the plugin first needs `assemble_once`.

usage: python3 ex1-registry-bound.py <tree> <diff-file>"""
import re, subprocess, sys

tree, diff = sys.argv[1], sys.argv[2]
names = sorted({name.split("::")[-1] for name in re.findall(r"editor_example_snapshot::<([\w:]+)>", open(diff, encoding="utf-8").read())})
root = f"{tree}/✏️s/🔌️plugins"
files = subprocess.run(["/usr/bin/grep", "-rln", "--include=*.rs", "impl ArtifactEditor for", root], capture_output=True, text=True).stdout.split("\n")
impls = {}
for file in filter(None, files):
    text = open(file, encoding="utf-8").read()
    for match in re.finditer(r"impl ArtifactEditor for (\w+) \{", text):
        body = text[match.end():]
        end = re.search(r"\n\}\n", body)
        impls[match.group(1)] = ("fn examples(" in body[: end.start() if end else len(body)], file.replace(root + "/", "").split("/")[0])
for name in names:
    own, plugin = impls.get(name, (None, "?"))
    print(f"{'editor' if own else 'REGISTRY' if own is False else 'MISSING'}\t{plugin}\t{name}")
