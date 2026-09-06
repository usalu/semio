#!/usr/bin/env python3
"""🐍️ Read-only audit: does every asset:// fixture path the mutate-remodeling-1 feature file
declares in its Scenario Outline Examples tables resolve to a real file on disk?

Usage: python3 🐍️fixture-audit.py

Mirrors (in miniature) `fixtureUrisIn` + `resolveFixtures` from
🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts: it extracts every
literal `asset://` URI produced by substituting a Scenario Outline's `<placeholder>` tokens with each
Examples row, then resolves it against the subset owner root exactly as `resolveFixtures` does for
scheme `asset` (`baseRel = discovered.owner`).
"""
import os
import re
import sys

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
OWNER_REL = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
FEATURE_REL = f"{OWNER_REL}/🧪️tests/📸️mutate-remodeling-1/🥒️.feature"


def parse_outlines(text):
    """🔍️ Very small Gherkin slice: pairs each `Scenario Outline:` block's step lines (which carry
    `<placeholder>` tokens) with its own `Examples:` table, returning [(steps, header, rows)]."""
    lines = text.splitlines()
    outlines = []
    i = 0
    while i < len(lines):
        if lines[i].strip().startswith("Scenario Outline:"):
            steps = []
            i += 1
            while i < len(lines) and "Examples:" not in lines[i]:
                stripped = lines[i].strip()
                if stripped.startswith(("Given ", "And ", "When ", "Then ")):
                    steps.append(stripped)
                i += 1
            # now at Examples:
            i += 1
            table = []
            while i < len(lines) and lines[i].strip().startswith("|"):
                cells = [c.strip() for c in lines[i].strip().strip("|").split("|")]
                table.append(cells)
                i += 1
            if table:
                header, rows = table[0], table[1:]
                outlines.append((steps, header, rows))
        else:
            i += 1
    return outlines


def substitute(step, header, row):
    out = step
    for col, val in zip(header, row):
        out = out.replace(f"<{col}>", val)
    return out


ASSET_RE = re.compile(r"asset://([^\s\"'`,;)\]]+)")


def main():
    feature_path = os.path.join(REPO_ROOT, FEATURE_REL)
    owner_abs = os.path.join(REPO_ROOT, OWNER_REL)
    if not os.path.isfile(feature_path):
        print(f"[fixture-audit] FEATURE NOT FOUND: {feature_path}")
        return 1
    text = open(feature_path, encoding="utf-8").read()
    outlines = parse_outlines(text)
    total = 0
    missing = 0
    checked_uris = set()
    for steps, header, rows in outlines:
        for row in rows:
            for step in steps:
                expanded = substitute(step, header, row)
                for m in ASSET_RE.finditer(expanded):
                    uri = m.group(1)
                    if uri in checked_uris:
                        continue
                    checked_uris.add(uri)
                    total += 1
                    abs_path = os.path.join(owner_abs, uri)
                    ok = os.path.isfile(abs_path)
                    if not ok:
                        missing += 1
                        row_label = dict(zip(header, row))
                        print(f"[MISSING] {uri}")
                        print(f"          row={row_label}")
                        print(f"          resolved={abs_path}")
    print(f"[fixture-audit] {total} distinct asset:// uri(s) referenced, {missing} missing, {total - missing} resolved")
    return 1 if missing else 0


if __name__ == "__main__":
    sys.exit(main())
