#!/usr/bin/env python3
"""🐍️ Read-only audit: does every fixture URI the mutate-remodeling-1 feature file declares resolve
to a real file on disk?

Usage: python3 🐍️fixture-audit.py

Mirrors (in miniature) `fixtureUrisIn` + `resolveFixtures` from
🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts. Since the harness
moved to puzzle's runtime pattern the URIs no longer live only in `Examples` cells: each Scenario
Outline carries a doc-string JSON spec whose `<placeholder>` tokens the runner substitutes per row,
and the feature's own description names the case-local refusal fixtures in prose. All three
carriers — description, step text and doc string — are scanned here, exactly as `fixtureUrisIn` does.

Resolution follows `resolveFixtures`: `asset://` against the subset owner root (`discovered.owner`),
`local://` against the case's own `🧫️fixtures/` directory, `shared://` against the owner's shared
fixture directory.
"""
import os
import re
import sys

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
OWNER_REL = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
CASE_REL = f"{OWNER_REL}/🧪️tests/📸️mutate-remodeling-1"
FEATURE_REL = f"{CASE_REL}/🥒️.feature"
FIXTURE_URI_RE = re.compile(r"\b(shared|local|asset)://([^\s\"'`,;)\]]+)")

BASE_REL = {
    "asset": OWNER_REL,
    "local": f"{CASE_REL}/🧫️fixtures",
    "shared": f"{OWNER_REL}/🧫️fixtures",
}


def blocks(text):
    """🔍️ Very small Gherkin slice: every `Scenario`/`Scenario Outline` block's raw text (step lines
    and doc-string bodies together) paired with its own `Examples:` table, plus the feature
    description as one block with no table."""
    lines = text.splitlines()
    found = []
    description, index = [], 0
    while index < len(lines) and not lines[index].strip().startswith("Feature:"):
        index += 1
    index += 1
    while index < len(lines) and "Scenario" not in lines[index]:
        description.append(lines[index])
        index += 1
    found.append(("\n".join(description), [], []))
    while index < len(lines):
        if "Scenario" not in lines[index]:
            index += 1
            continue
        body = []
        index += 1
        while index < len(lines) and "Examples:" not in lines[index] and "Scenario" not in lines[index]:
            body.append(lines[index])
            index += 1
        table = []
        if index < len(lines) and "Examples:" in lines[index]:
            index += 1
            while index < len(lines) and lines[index].strip().startswith("|"):
                table.append([cell.strip() for cell in lines[index].strip().strip("|").split("|")])
                index += 1
        header, rows = (table[0], table[1:]) if table else ([], [])
        found.append(("\n".join(body), header, rows))
    return found


def substitute(text, header, row):
    for column, value in zip(header, row):
        text = text.replace(f"<{column}>", value)
    return text


def main():
    feature_path = os.path.join(REPO_ROOT, FEATURE_REL)
    if not os.path.isfile(feature_path):
        print(f"[fixture-audit] FEATURE NOT FOUND: {feature_path}")
        return 1
    text = open(feature_path, encoding="utf-8").read()
    seen, missing, unexpanded = {}, 0, 0
    for body, header, rows in blocks(text):
        for row in rows or [None]:
            expanded = body if row is None else substitute(body, header, row)
            for scheme, path in FIXTURE_URI_RE.findall(expanded):
                uri = f"{scheme}://{path}"
                if uri in seen:
                    continue
                if "<" in path:
                    unexpanded += 1
                    print(f"[UNEXPANDED] {uri} — a placeholder survived row {dict(zip(header, row or []))}")
                    continue
                resolved = os.path.join(REPO_ROOT, BASE_REL[scheme], path)
                seen[uri] = os.path.isfile(resolved)
                if not seen[uri]:
                    missing += 1
                    print(f"[MISSING] {uri}\n          resolved={resolved}")
    print(f"[fixture-audit] {len(seen)} distinct fixture uri(s) referenced, {missing} missing, {len(seen) - missing} resolved, {unexpanded} unexpanded")
    return 1 if missing or unexpanded else 0


if __name__ == "__main__":
    sys.exit(main())
