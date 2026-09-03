#!/usr/bin/env python3
"""🔍️ D3 self-check: for each given feature file, finds every `asset://` URI inside every
`Scenario Outline`'s step text AFTER substituting each Examples row (mirroring the framework's own
`substitute()`/`fixtureUrisIn` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`), and reports any
URI that does not resolve to a real file under the case's owner root. Catches the exact class of bug
`missing-fixture` would — an emoji variation-selector dropped from one column of a table but not the
prose — without waiting on the full, slow `bun test contract` gate.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

STEP_RE = re.compile(r"^\s*(Given|When|Then|And|But|\*)\s+(.*)$")
URI_RE = re.compile(r"\b(shared|local|asset)://([^\s\"'`,;)\]]+)")


def substitute(text: str, row: dict) -> str:
    return re.sub(r"<([^<>]+)>", lambda m: row.get(m.group(1), m.group(0)), text)


def parse_blocks(text: str):
    lines = text.split("\n")
    blocks = []
    current = None
    in_examples = False
    header = None
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("Scenario Outline") or stripped.startswith("Scenario Template"):
            if current:
                blocks.append(current)
            current = {"steps": [], "examples": []}
            in_examples = False
            continue
        if stripped.startswith("Scenario:") or stripped.startswith("Feature:"):
            if current:
                blocks.append(current)
            current = None
            in_examples = False
            continue
        if current is None:
            continue
        if stripped.startswith("Examples:") or stripped.startswith("Scenarios:"):
            in_examples = True
            header = None
            continue
        if in_examples and stripped.startswith("|"):
            cells = [c.strip() for c in stripped.strip("|").split("|")]
            if header is None:
                header = cells
            else:
                current["examples"].append(dict(zip(header, cells)))
            continue
        m = STEP_RE.match(line)
        if m and not in_examples:
            current["steps"].append(m.group(2))
    if current:
        blocks.append(current)
    return blocks


def main() -> int:
    bad = False
    for feature_path in sys.argv[1:]:
        path = Path(feature_path)
        owner = path.parent.parent.parent  # 🧪️tests/<case>/🥒️.feature -> owner is 2 up from 🧪️tests
        text = path.read_text(encoding="utf-8")
        blocks = parse_blocks(text)
        seen = set()
        for block in blocks:
            rows = block["examples"] or [{}]
            for row in rows:
                for step in block["steps"]:
                    substituted = substitute(step, row)
                    for scheme, name in URI_RE.findall(substituted):
                        if scheme != "asset":
                            continue
                        uri = f"{scheme}://{name}"
                        if uri in seen:
                            continue
                        seen.add(uri)
                        target = owner / name
                        ok = target.exists()
                        if not ok:
                            bad = True
                            print(f"MISSING  {feature_path}  {uri}")
        print(f"checked {feature_path}: {len(seen)} distinct asset:// uris")
    return 1 if bad else 0


if __name__ == "__main__":
    raise SystemExit(main())
