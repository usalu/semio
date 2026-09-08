#!/usr/bin/env python3
"""🐫 Ticket row 47a impact scan.

Finds every `#[derive(… ToValue|FromValue …)]` enum whose container attributes declare
`#[value(rename_all = "…")]` WITHOUT `rename_all_fields`, and that has at least one named variant
field whose Rust identifier is multi-word (contains `_`). Those are exactly the containers whose
wire field names change when `ContainerAttrs::field_rename_all()` stops falling back from
`rename_all_fields` to `rename_all`.

    python3 wp4c-rename-all-fields-scan.py [root ...]

Prints one `path:line enum rename_all=<case> fields=<a,b,…>` row per hit, then a count per top
partition. Text-level scan (no rustc): the repo has ~2.4M lines of Rust and no crate here can be
expanded cheaply, so the parser tracks `#[…]`/`#[value(…)]` blocks and brace depth directly.
"""
from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

VALUE_ATTR = re.compile(r"#\[value\((.*)\)\]", re.DOTALL)
DERIVE_ATTR = re.compile(r"#\[derive\((.*)\)\]", re.DOTALL)
RENAME_ALL = re.compile(r"\brename_all\s*=\s*\"([^\"]+)\"")
RENAME_ALL_FIELDS = re.compile(r"\brename_all_fields\s*=\s*\"([^\"]+)\"")
ENUM_HEAD = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?enum\s+([A-Za-z_][A-Za-z0-9_]*)")
FIELD = re.compile(r"(?<![:\w])([a-z_][a-z0-9_]*)\s*:(?!:)")


def scan_file(path: Path) -> list[tuple[int, str, str, list[str]]]:
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except (UnicodeDecodeError, OSError):
        return []
    hits: list[tuple[int, str, str, list[str]]] = []
    attrs: list[str] = []
    pending = ""
    index = 0
    while index < len(lines):
        line = lines[index]
        stripped = line.strip()
        if pending or stripped.startswith("#["):
            pending = f"{pending} {stripped}".strip()
            if pending.count("[") <= pending.count("]") and pending.count("(") <= pending.count(")"):
                attrs.append(pending)
                pending = ""
            index += 1
            continue
        head = ENUM_HEAD.match(line)
        if head is None:
            if stripped and not stripped.startswith("//"):
                attrs = []
            index += 1
            continue
        block = " ".join(attrs)
        attrs = []
        derives = " ".join(DERIVE_ATTR.findall(block))
        value_attrs = " ".join(VALUE_ATTR.findall(block))
        if "ToValue" not in derives and "FromValue" not in derives:
            index += 1
            continue
        case = RENAME_ALL.search(value_attrs)
        if case is None or RENAME_ALL_FIELDS.search(value_attrs) is not None:
            index += 1
            continue
        depth = 0
        cursor = index
        multiword: list[str] = []
        while cursor < len(lines):
            body = lines[cursor]
            depth += body.count("{") - body.count("}")
            if cursor > index and not body.strip().startswith(("#[", "//")):
                multiword.extend(name for name in FIELD.findall(body) if "_" in name)
            if depth == 0 and cursor > index:
                break
            cursor += 1
        if multiword:
            hits.append((index + 1, head.group(1), case.group(1), sorted(set(multiword))))
        index = cursor + 1
    return hits


def main() -> int:
    roots = sys.argv[1:] or ["."]
    listed = subprocess.run(["git", "ls-files", *roots], capture_output=True, text=True, check=True).stdout.splitlines()
    total = 0
    buckets: dict[str, int] = {}
    for name in listed:
        if not name.endswith(".rs"):
            continue
        path = Path(name)
        for line, enum, case, fields in scan_file(path):
            total += 1
            bucket = "/".join(path.parts[:2])
            buckets[bucket] = buckets.get(bucket, 0) + 1
            print(f"{name}:{line} {enum} rename_all={case} fields={','.join(fields)}")
    print(f"\naffected-enums={total}")
    for bucket, count in sorted(buckets.items(), key=lambda row: -row[1]):
        print(f"  {bucket} {count}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
