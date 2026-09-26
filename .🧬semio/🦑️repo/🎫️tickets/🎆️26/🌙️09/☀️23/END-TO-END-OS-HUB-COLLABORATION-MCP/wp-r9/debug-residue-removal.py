#!/usr/bin/env python3
"""🧹️ R9 item 2: `[DEBUG]` is reserved for temporary logs (AGENTS.md). In the hub and semio-MCP trees this
(1) deletes every whole-line `println!`/`eprintln!`/`console.log` statement whose message starts with `[DEBUG]`
inside test sources (temporary evidence prints; nothing consumes them), and (2) drops the `[DEBUG] ` tag from the
hub `📜️script.ts` status lines (permanent check-verb output) plus the one machine channel it parses
(`native-catalog-payload=`, producer in the stdio provider law).

Usage: `python3 debug-residue-removal.py [--apply]` (default dry run; exit 1 on any unexpected shape)."""
import os
import re
import sys

ROOT = "/Users/ueli/Documents/semio/"
HUB_SCRIPT = "🌎️hub/📦️packages/🦀️rust/📜️script.ts"
PAYLOAD_PRODUCER = "✏️s/🔌️plugins/🗄️stdio/🧪️tests/📇️native-openable-provider/🦀️.rs"
TREES = ("🌎️hub", "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp")
SKIP = {"node_modules", "dist", "target", "🗑️generated", "🤖️generated"}
START = re.compile(r"^([ \t]*)(println!|eprintln!|console\.log)\(\s*", re.M)


def statement_end(text: str, open_at: int) -> int:
    depth, index, quote = 0, open_at, None
    while index < len(text):
        char = text[index]
        if quote:
            if char == "\\":
                index += 2
                continue
            if char == quote:
                quote = None
        elif char in "\"'`":
            quote = char
        elif char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                return index
        index += 1
    return -1


def remove_debug_statements(text: str) -> tuple[str, list[str]]:
    removed, out, cursor = [], [], 0
    for match in START.finditer(text):
        if match.start() < cursor:
            continue
        literal = text[match.end():match.end() + 9]
        if not re.match(r"""["`]\[DEBUG\]""", literal):
            continue
        close = statement_end(text, text.index("(", match.start()))
        tail = text[close + 1:]
        terminator = re.match(r"[ \t]*;?[ \t]*\n", tail)
        if close < 0 or not terminator:
            raise SystemExit(f"unexpected statement shape at offset {match.start()}: {text[match.start():match.start() + 120]!r}")
        end = close + 1 + terminator.end()
        out.append(text[cursor:match.start()])
        removed.append(text[match.start():end].strip().splitlines()[0][:140])
        cursor = end
    out.append(text[cursor:])
    return "".join(out), removed


def test_source(path: str) -> bool:
    return "/🧪️tests/" in "/" + path and path.endswith(("🦀️.rs", "🟦️.ts", "🟦️.tsx"))


def main() -> int:
    apply = "--apply" in sys.argv
    edits: dict[str, str] = {}
    total = 0
    for tree in TREES:
        for directory, names, files in os.walk(ROOT + tree):
            names[:] = [name for name in names if name not in SKIP]
            for name in files:
                path = os.path.relpath(os.path.join(directory, name), ROOT)
                if not test_source(path):
                    continue
                text = open(ROOT + path, encoding="utf-8").read()
                if "[DEBUG]" not in text:
                    continue
                new, removed = remove_debug_statements(text)
                for line in removed:
                    print(f"delete {path} :: {line}")
                total += len(removed)
                if new != text:
                    edits[path] = new
    script = open(ROOT + HUB_SCRIPT, encoding="utf-8").read()
    tagged = script.count("[DEBUG] ")
    print(f"retag {HUB_SCRIPT}: {tagged} `[DEBUG] ` tags")
    edits[HUB_SCRIPT] = script.replace("[DEBUG] ", "")
    producer = open(ROOT + PAYLOAD_PRODUCER, encoding="utf-8").read()
    old = 'println!("[DEBUG] native-catalog-payload={}"'
    if producer.count(old) != 1:
        print("BAD payload producer shape")
        return 1
    edits[PAYLOAD_PRODUCER] = producer.replace(old, 'println!("native-catalog-payload={}"')
    print(f"test statements deleted: {total}; files edited: {len(edits)}")
    for path, text in edits.items():
        left = [line.strip()[:120] for line in text.splitlines() if "[DEBUG]" in line]
        for line in left:
            print(f"left {path} :: {line}")
        if apply:
            open(ROOT + path, "w", encoding="utf-8").write(text)
    print("applied" if apply else "dry run")
    return 0


if __name__ == "__main__":
    sys.exit(main())
