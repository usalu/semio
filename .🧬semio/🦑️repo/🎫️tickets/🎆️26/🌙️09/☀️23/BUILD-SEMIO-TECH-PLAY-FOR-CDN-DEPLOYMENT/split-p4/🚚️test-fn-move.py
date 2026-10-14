#!/usr/bin/env python3
"""🚚️ Moves named test functions (with their doc comments and attributes) out of one file into another.

A test belongs to the lowest crate that can name everything it exercises. When one file mixes tests of its own crate
with tests that need an upper crate, this lifts the upper ones out: each named `fn` (sync or async, `#[test]` or helper)
is cut with its leading `///` lines and attributes, the `//#region`/`//#endregion` pair left empty around it is dropped,
and the functions are appended to the destination, which is created with the spec's header when missing. With `"drop"`
instead of `"to"` the functions are deleted (a duplicate of a law kept elsewhere).

usage: 🚚️test-fn-move.py <spec.json> plan|apply
spec: [{"from": "<repo-rel file>", "fns": ["name", ...], "to": "<repo-rel file>" | "drop": true, "header": "<text>"}]
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
files = {}


def text(path):
    if path not in files:
        files[path] = open(path, encoding="utf-8").read() if os.path.exists(path) else None
    return files[path]


for row in spec:
    source = absolute(row["from"])
    src = text(source)
    moved = []
    for name in row["fns"]:
        s = strip(src)
        m = re.search(r'(?m)^((?:[ \t]*(?:///[^\n]*|#\[[^\n]*\])\n)*)[ \t]*(?:pub )?(?:async )?fn ' + re.escape(name) + r'\b', s)
        if not m:
            raise SystemExit(f"fn {name} not in {row['from']}")
        depth, i = 0, s.index("{", m.end())
        while True:
            depth += s[i] == "{"
            depth -= s[i] == "}"
            i += 1
            if depth == 0:
                break
        end = i + 1 if src[i:i + 1] == "\n" else i
        start = m.start()
        while start > 0:
            previous = src.rfind("\n", 0, start - 1) + 1
            if re.match(r'[ \t]*(?:///|#\[)', src[previous:start]):
                start = previous
            else:
                break
        moved.append(src[start:end].rstrip("\n"))
        src = src[:start] + src[end:]
    src = re.sub(r'(?m)^//#region [^\n]*\n\s*//#endregion [^\n]*\n', "", src)
    src = re.sub(r'\n{3,}', "\n\n", src)
    files[source] = src
    if row.get("drop"):
        print(f"dropped {row['fns']} from {row['from']}")
        continue
    target = absolute(row["to"])
    existing = text(target)
    files[target] = (existing.rstrip("\n") if existing is not None else row["header"].rstrip("\n")) + "\n\n" + "\n\n".join(moved) + "\n"
    print(f"moved {row['fns']} from {row['from']} to {row['to']}")
if mode == "apply":
    for path, content in files.items():
        if content is not None:
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, "w", encoding="utf-8").write(content)
