#!/usr/bin/env python3
"""🥒️ W2-W media: rewrites one `🥒️.feature` in place to the F10 wire form.

Every Examples table's `params` cell is replaced by the wire payload named for its row id (rows file: one
`| id | params |` line per kind, as `🧪️w2w-media-wire-rows.py` prints them or as written by hand), rows whose id is
listed with `--drop-row` are removed, and every scenario tagged `@id-<prefix>…` for a `--drop-scenario` prefix is
deleted with its tags. Other columns (`code`, `setup`) are kept cell for cell.

Usage: python3 🧪️w2w-media-rewrite-feature.py <feature> <rows-file> [--drop-row id]… [--drop-scenario prefix]…
"""
import re
import sys
from pathlib import Path


def cells(line):
    return [cell.strip() for cell in line.strip().strip("|").split("|")]


def render(values):
    return "      | " + " | ".join(values) + " |"


def main():
    feature, rows_file, *flags = sys.argv[1:]
    drop_rows = {flags[i + 1] for i, flag in enumerate(flags) if flag == "--drop-row"}
    drop_scenarios = [flags[i + 1] for i, flag in enumerate(flags) if flag == "--drop-scenario"]
    wire = {}
    for line in Path(rows_file).read_text().splitlines():
        if line.strip().startswith("|"):
            kind, params = line.strip().strip("|").split("|", 1)
            wire[kind.strip()] = params.strip()
    lines = Path(feature).read_text().split("\n")
    out, header = [], None
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("|"):
            row = cells(line)
            if header is None or row[0] == "id":
                header = row
                out.append(render(row))
                continue
            kind = row[header.index("id")]
            if kind in drop_rows:
                continue
            if kind not in wire:
                raise SystemExit(f"{feature}: no wire payload for row {kind!r}")
            row[header.index("params")] = wire[kind]
            out.append(render(row))
            continue
        header = None
        out.append(line)
    text = "\n".join(out)
    for prefix in drop_scenarios:
        pattern = re.compile(r"  @id-" + re.escape(prefix) + r"[^\n]*\n(?:  @[^\n]*\n)*  Scenario[^\n]*\n(?:    [^\n]*\n|      [^\n]*\n)*\n?")
        text, count = pattern.subn("", text)
        if count == 0:
            raise SystemExit(f"{feature}: no scenario tagged @id-{prefix}")
    Path(feature).write_text(text)


if __name__ == "__main__":
    main()
