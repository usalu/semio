#!/usr/bin/env python3
"""🧳️ Moves anchored text blocks between files of one crate (pre-cut item relocation, compile-atomic with 🔀️repoint.py).

Each step cuts the text between a start anchor (inclusive) and an end anchor (inclusive, first occurrence after the
start) from `from` and appends it to `to` (before `before` when given, else at the end); `insert` steps add literal text
after an anchor (or at the start of the file after the inner doc comment when the anchor is null); `delete` steps drop
literal text. Every anchor must match exactly once, or nothing is written.

usage: 🧳️relocate.py <steps.json> plan|apply
steps: [{"op": "move", "from": file, "to": file, "start": text, "end": text, "before": text|null},
        {"op": "insert", "file": file, "after": text|null, "text": text}, {"op": "delete", "file": file, "text": text}]
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))


def main():
    steps = json.load(open(sys.argv[1], encoding="utf-8"))
    base = os.path.join(REPO, steps["base"])
    files = {}

    def get(name):
        path = os.path.join(base, name)
        if path not in files:
            files[path] = open(path, encoding="utf-8").read() if os.path.exists(path) else ""
        return path

    def once(text, needle, what):
        count = text.count(needle)
        if count != 1:
            raise SystemExit(f"anchor {what!r} matched {count} times: {needle[:80]!r}")
        return text.index(needle)

    for step in steps["steps"]:
        if step["op"] == "move":
            src, dst = get(step["from"]), get(step["to"])
            text = files[src]
            a = once(text, step["start"], "start")
            b = text.index(step["end"], a) + len(step["end"])
            if b < len(text) and text[b] == "\n":
                b += 1
            block = text[a:b]
            files[src] = text[:a] + text[b:]
            target = files[dst]
            if step.get("carry_uses"):
                idents = set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", block))
                carried = []
                for m in re.finditer(r"(?ms)^use [^;]*;", text):
                    stmt = m.group(0)
                    leaves = set(re.findall(r"([A-Za-z_][A-Za-z0-9_]*)\s*(?:[,}]|$|;)", stmt.replace("\n", " "))) | set(re.findall(r"\bas\s+([A-Za-z_][A-Za-z0-9_]*)", stmt))
                    if leaves & idents and stmt not in target:
                        carried.append(stmt)
                if carried:
                    um = list(re.finditer(r"(?ms)^use [^;]*;\n", target))
                    at = um[-1].end() if um else re.match(r"((?:[ \t]*//![^\n]*\n)*)", target).end()
                    target = target[:at] + "\n".join(carried) + "\n" + target[at:]
                    print(f"  carried {len(carried)} use lines")
            if step.get("before"):
                i = once(target, step["before"], "before")
                files[dst] = target[:i] + block + ("\n" if not block.endswith("\n\n") else "") + target[i:]
            else:
                files[dst] = target.rstrip("\n") + "\n\n" + block
            print(f"move {len(block.splitlines())} lines {step['from']} -> {step['to']}")
        elif step["op"] == "insert":
            path = get(step["file"])
            text = files[path]
            if step.get("after") is None:
                m = re.match(r"((?:[ \t]*//![^\n]*\n)*)", text)
                i = m.end()
            else:
                i = once(text, step["after"], "after") + len(step["after"])
            files[path] = text[:i] + step["text"] + text[i:]
        elif step["op"] == "delete":
            path = get(step["file"])
            text = files[path]
            i = once(text, step["text"], "delete")
            files[path] = text[:i] + text[i + len(step["text"]):]
    if sys.argv[2] == "apply":
        for path, text in files.items():
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, "w", encoding="utf-8").write(text)
        print(f"wrote {len(files)} files")


main()
