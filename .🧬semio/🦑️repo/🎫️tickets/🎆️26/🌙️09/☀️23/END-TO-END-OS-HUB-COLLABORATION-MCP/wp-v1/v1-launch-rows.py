#!/usr/bin/env python3
"""V1 one-off: insert launch rows into .vscode/🧩️launch.seed.jsonc AND .vscode/launch.json identically.

Usage: python3 v1-launch-rows.py <spec.json> [--dry-run]
spec = [{"after": "<existing row name>", "row": {<launch configuration>}}, ...]
Rows whose name already exists in a file are skipped (idempotent). The anchor row's object is located by its exact
`"name": "<anchor>"` line and closed by the next line equal to `    },` or `    }` (4-space object indent).
"""
import json
import sys

ROOT = "/Users/ueli/Documents/semio/.vscode/"
FILES = [ROOT + "🧩️launch.seed.jsonc", ROOT + "launch.json"]


def render(row):
    body = json.dumps(row, indent=2, ensure_ascii=False).split("\n")
    return "\n".join(("    " + line) for line in body)


def insert(text, anchor, row):
    if f'"name": {json.dumps(row["name"], ensure_ascii=False)},' in text:
        return text, "exists"
    marker = f'      "name": {json.dumps(anchor, ensure_ascii=False)},\n'
    count = text.count(marker)
    if count != 1:
        raise SystemExit(f"anchor {anchor!r} found {count} times")
    start = text.index(marker)
    lines = text[start:].split("\n")
    offset = start
    for line in lines:
        offset += len(line) + 1
        if line in ("    },", "    }"):
            break
    else:
        raise SystemExit(f"no close for {anchor!r}")
    close_line_end = offset
    closed = text[:close_line_end]
    rest = text[close_line_end:]
    if closed.endswith("    }\n"):
        closed = closed[:-2] + ",\n"
        return closed + render(row) + "\n" + rest, "inserted-last"
    return closed + render(row) + ",\n" + rest, "inserted"


def insert_input(text, anchor, item):
    if f'"id": {json.dumps(item["id"], ensure_ascii=False)},' in text:
        return text, "exists"
    marker = f'      "id": {json.dumps(anchor, ensure_ascii=False)},\n'
    if text.count(marker) != 1:
        raise SystemExit(f"input anchor {anchor!r} found {text.count(marker)} times")
    start = text.index(marker)
    close = text.index("\n    }", start) + len("\n    }")
    return text[:close] + ",\n" + render(item) + text[close:], "inserted"


def main():
    spec = json.load(open(sys.argv[1]))
    dry = "--dry-run" in sys.argv
    for path in FILES:
        text = open(path, encoding="utf-8").read()
        for item in spec:
            if "afterInput" in item:
                text, outcome = insert_input(text, item["afterInput"], item["input"])
                print(f"{path.rsplit('/', 1)[1]}: input {item['input']['id']} after {item['afterInput']}: {outcome}")
                continue
            text, outcome = insert(text, item["after"], item["row"])
            print(f"{path.rsplit('/', 1)[1]}: {item['row']['name']} after {item['after']}: {outcome}")
        if not dry:
            open(path, "w", encoding="utf-8").write(text)
    if not dry:
        import re
        for path in FILES:
            body = re.sub(r"(?m)^\s*//.*$", "", open(path, encoding="utf-8").read())
            body = body[: body.rindex("}") + 1]
            try:
                json.loads(body)
                print(f"{path.rsplit('/', 1)[1]} parses")
            except json.JSONDecodeError as error:
                print(f"{path.rsplit('/', 1)[1]} strict-parse note: {error}")


main()
