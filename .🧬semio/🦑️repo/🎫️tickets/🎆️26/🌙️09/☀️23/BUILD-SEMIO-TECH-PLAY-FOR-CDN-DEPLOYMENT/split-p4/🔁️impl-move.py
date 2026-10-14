#!/usr/bin/env python3
"""🔁️ Moves every `impl … for <Type>` block (at any nesting, inline modules included) out of source files into a target region.

The orphan rule puts a foreign trait's impl for a type in the type's own crate. When a type lands in a crate above the
codec modules that implemented traits for it, this lifts those impls next to the type: each block, with its leading doc
comments and attributes, is cut from its file and appended inside `//#region <region>` of the target, followed by the
spec's extra `use` lines. Bare names the moved code takes from its old module have to be listed as `use` lines (and made
`pub` there) by the spec.

usage: 🔁️impl-move.py <spec.json> plan|apply
spec: {"type": "Puzzle2dPlaySnapshot", "target": "<repo-rel file>", "region": "🚪️Codecs", "sources": ["<repo-rel file>", …],
       "uses": ["use …;", …], "widen": {"<repo-rel file>": ["fn restore_authority", "mod semantic_cells", …]}}
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
census = {}
CENSUS = os.path.join(HERE, "🕸️module-graph.py")
exec(compile(open(CENSUS, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CENSUS, "exec"), census)
strip = census["strip"]
spec, mode = json.load(open(sys.argv[1], encoding="utf-8")), sys.argv[2]
absolute = lambda p: os.path.normpath(os.path.join(REPO, p))
HEAD = re.compile(r'(?m)^[ \t]*impl\b[^{;]*?\bfor\s+(?:[\w:]+::)?' + re.escape(spec["type"]) + r'\s*\{')
writes, moved = {}, []
for source in spec["sources"]:
    path = absolute(source)
    text = open(path, encoding="utf-8").read()
    while True:
        s = strip(text)
        m = HEAD.search(s)
        if not m:
            break
        depth, i = 0, s.index("{", m.start())
        while True:
            depth += s[i] == "{"
            depth -= s[i] == "}"
            i += 1
            if depth == 0:
                break
        start = m.start()
        while start > 0:
            previous = text.rfind("\n", 0, start - 1) + 1
            if re.match(r'[ \t]*(?:///|#\[)', text[previous:start]):
                start = previous
            else:
                break
        end = i + 1 if text[i:i + 1] == "\n" else i
        block = text[start:end].rstrip("\n")
        indent = re.match(r'[ \t]*', block).group(0)
        moved.append("\n".join(line[len(indent):] if line.startswith(indent) else line for line in block.split("\n")))
        text = text[:start] + text[end:]
    writes[path] = text
for path, items in spec.get("widen", {}).items():
    path = absolute(path)
    text = writes.get(path) or open(path, encoding="utf-8").read()
    for item in items:
        kind, name = item.split(" ", 1)
        pattern = re.compile(r'(?m)^((?:pub\([^)]*\)\s+)?)' + re.escape(kind) + r'\s+' + re.escape(name) + r'\b')
        if not pattern.search(text):
            raise SystemExit(f"no {item} in {path}")
        text = pattern.sub(lambda m: "pub " + kind + " " + name, text, count=1)
    writes[path] = text
target = absolute(spec["target"])
body = writes.get(target) or open(target, encoding="utf-8").read()
region = f"//#region {spec['region']}\n" + "\n".join(spec.get("uses", [])) + ("\n\n" if spec.get("uses") else "") + "\n\n".join(moved) + f"\n//#endregion {spec['region']}\n"
writes[target] = body.rstrip("\n") + "\n\n" + region
print(f"moved {len(moved)} impls for {spec['type']} into {spec['target']}")
for block in moved:
    print("  " + block.split("{", 1)[0].strip().split("\n")[-1][:120])
if mode == "apply":
    for path, text in writes.items():
        open(path, "w", encoding="utf-8").write(text)
