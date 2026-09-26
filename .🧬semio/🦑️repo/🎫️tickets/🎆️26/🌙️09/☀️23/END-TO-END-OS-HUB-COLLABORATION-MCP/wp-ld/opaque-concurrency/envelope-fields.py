#!/usr/bin/env python3
"""✉️ LD item 2: adds `observed: None, target: Vec::new()` right after `dependencies` in every Rust
`MutationEnvelope { … }` struct-literal EXPRESSION of the tree (patterns and `..base` spreads are left
alone). One-off codemod for the envelope wire change; lives in the ticket folder, never in the repo.

usage: envelope-fields.py [--dry-run]
"""
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
SKIP = ("🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs",)


def literal_sites(text: str):
    for match in re.finditer(r"MutationEnvelope\s*\{", text):
        start = match.end()
        prefix = text[max(0, match.start() - 40):match.start()]
        if re.search(r"(struct|for|impl)\s+$", prefix) or re.search(r"(struct|impl .* for)\s*$", prefix):
            continue
        depth, index = 1, start
        while depth and index < len(text):
            char = text[index]
            if char in "{([":
                depth += 1
            elif char in "})]":
                depth -= 1
            index += 1
        body = text[start:index - 1]
        yield start, index - 1, body


def top_level_fields(body: str):
    depth, current, fields, begin = 0, [], [], 0
    for position, char in enumerate(body):
        if char in "{([":
            depth += 1
        elif char in "})]":
            depth -= 1
        elif char == "," and depth == 0:
            fields.append((begin, position))
            begin = position + 1
    if body[begin:].strip():
        fields.append((begin, len(body)))
    return fields


def rewrite(text: str):
    edits = []
    for start, end, body in literal_sites(text):
        fields = top_level_fields(body)
        names = [body[a:b].strip().split(":")[0].strip() for a, b in fields]
        if any(name.startswith("..") for name in names) or "dependencies" not in names or "observed" in names:
            continue
        a, b = fields[names.index("dependencies")]
        multiline = "\n" in body[a:b]
        if multiline:
            indent = re.match(r"\s*", body[a:b].lstrip("\n")).group(0) if "\n" in body[a:b] else ""
            line_start = body.rfind("\n", 0, b)
            indent = re.match(r"[ \t]*", body[line_start + 1:]).group(0)
            insertion = f"\n{indent}observed: None,\n{indent}target: Vec::new(),"
            edits.append((start + b + 1, insertion))
        else:
            edits.append((start + b + 1, " observed: None, target: Vec::new(),"))
    for position, insertion in sorted(edits, reverse=True):
        text = text[:position] + insertion + text[position:]
    return text, len(edits)


def main() -> int:
    dry = "--dry-run" in sys.argv
    files = subprocess.run(["git", "grep", "-l", "MutationEnvelope {", "--", "*.rs", ":!.tmp-ticket", ":!.🧬semio"], cwd=ROOT, capture_output=True, text=True, check=True).stdout.split("\n")
    total = 0
    for name in filter(None, files):
        if name in SKIP:
            continue
        path = ROOT / name
        text = path.read_text(encoding="utf-8")
        rewritten, count = rewrite(text)
        if count:
            total += count
            print(f"{count:3} {name}")
            if not dry:
                path.write_text(rewritten, encoding="utf-8")
    print(f"total {total}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
