#!/usr/bin/env python3
"""🪪️ Renames every `mutate-remodeling-1` Examples row to the registered scenario id of the vector it
runs (the vector column's scenario directory without its emoji), so the contract's completeness gate
counts each row against the catalog's own `vectors[]` instead of reporting `<kind>-<suffix>` rows as
undeclared kinds. Rewrites the feature's id column (re-aligning each table), the `Given` wording, and
the MUTATE_SCENARIOS / INVERSE_SCENARIOS lists of the Rust and Python adapters.
Usage: remodel-row-ids.py [--write]"""
import re
import sys
from pathlib import Path

case = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📸️mutate-remodeling-1")
write = "--write" in sys.argv


def scenario_id(vector):
    return re.sub(r"^[^a-z0-9]+", "", vector.split("/")[1])


def cells(line):
    return [cell.strip() for cell in line.strip().strip("|").split("|")]


def render(block, indent):
    widths = [max(len(row[column]) for row in block) for column in range(len(block[0]))]
    return [indent + "| " + " | ".join(value.ljust(width) for value, width in zip(row, widths)) + " |" for row in block]


feature = (case / "🥒️.feature").read_text(encoding="utf-8").split("\n")
renamed = {"mutate": {}, "inverse": {}}
out, block, indent, tag = [], [], "", None
for line in feature + [""]:
    stripped = line.strip()
    if stripped.startswith("@id-"):
        tag = stripped[4:]
    if stripped.startswith("|"):
        indent = line[: len(line) - len(line.lstrip())]
        row = cells(line)
        if block:
            new = scenario_id(row[2]) if "/" in row[2] else row[0]
            if renamed[tag].setdefault(row[0], new) != new:
                raise SystemExit(f"{tag} row {row[0]} maps to two vectors")
            row[0] = new
        block.append(row)
        continue
    if block:
        out.extend(render(block, indent))
        block = []
    out.append(line)
text = "\n".join(out[:-1])
text = re.sub(r"(Given the (?:committed specification|committed refusal|committed no-op|case-local refusal) vector) for the <id> kind", r"\1 <id>", text)
for role, mapping in renamed.items():
    if len(set(mapping.values())) != len(mapping):
        raise SystemExit(f"{role}: two rows would share one scenario id")

edits = {case / "🥒️.feature": text}
for adapter, pattern in [(case / "🦀️.rs", r'(const {name}: &\[&str\] = &\[\n)(.*?)(\n\];)'), (case / "🐍️.py", r"({name}: list\[str\] = \[\n)(.*?)(\n\])")]:
    source = adapter.read_text(encoding="utf-8")
    for name, role in [("MUTATE_SCENARIOS", "mutate"), ("INVERSE_SCENARIOS", "inverse")]:
        match = re.search(pattern.format(name=name), source, re.S)
        if match is None:
            raise SystemExit(f"{adapter.name}: no {name} list")
        old = re.findall(r'"([^"]+)"', match.group(2))
        if sorted(old) != sorted(renamed[role]):
            raise SystemExit(f"{adapter.name}: {name} drifts from the feature's {role} rows")
        entries = "\n".join(f'    "{value}",' for value in sorted(renamed[role][item] for item in old))
        source = source[: match.start(2)] + entries + source[match.end(2) :]
    edits[adapter] = source

for path, content in edits.items():
    print(("write " if write else "would write ") + path.name)
    if write:
        path.write_text(content, encoding="utf-8")
print({role: sum(1 for old, new in mapping.items() if old != new) for role, mapping in renamed.items()}, "rows renamed")
