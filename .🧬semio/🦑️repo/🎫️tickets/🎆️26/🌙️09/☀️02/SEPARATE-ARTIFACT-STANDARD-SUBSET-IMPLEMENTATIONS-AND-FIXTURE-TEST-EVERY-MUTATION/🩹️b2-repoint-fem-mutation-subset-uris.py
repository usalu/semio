#!/usr/bin/env python3
"""🩹️ Repoints the fem2d/fem3d spec-vector `asset://` URIs at the subset that now owns each mutation.

An earlier shard split `s.fem.2d` and `s.fem.3d`'s 25 mutations out of the wildcard `✳️any` subset
into `mesh`/`material`/`load`/`boundary`/`analysis` and moved every mutation directory, but the
`🥒️.feature` files' `id-spec-vector` scenario still hardcodes `✳️any` in its `Given`/`And` steps, so
all 75+75 fixture rows point at a directory that no longer exists. This adds a `subset` column to
each Examples table (measured from what is actually on disk under each artifact's
`🏅️standards/🔖️1/🪆️subsets/*/🧬️schema/🧬️mutations/`) and swaps the hardcoded `✳️any` segment for
`✳️<subset>` in the three Given/And lines.

Run with `--check` to report without writing.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]

# 🗺️ measured on disk: `find …/🪆️subsets/✳️<subset>/🧬️schema/🧬️mutations -maxdepth 1 -type d`
SUBSET_OF_2D = {
    "create-node": "mesh", "delete-node": "mesh", "create-element": "mesh",
    "delete-element": "mesh", "replace-element": "mesh",
    "create-material": "material", "delete-material": "material", "replace-material": "material",
    "create-section": "mesh", "delete-section": "mesh", "replace-section": "mesh",
    "create-support": "boundary", "delete-support": "boundary", "replace-support": "boundary",
    "create-region": "mesh", "delete-region": "mesh", "replace-region": "mesh",
    "create-load-case": "load", "delete-load-case": "load",
    "add-load": "load", "remove-load": "load", "change-load-case-self-weight": "load",
    "create-combination": "load", "delete-combination": "load",
    "update-analysis-settings": "analysis",
}

SUBSET_OF_3D = dict(SUBSET_OF_2D)
del SUBSET_OF_3D["create-region"], SUBSET_OF_3D["delete-region"], SUBSET_OF_3D["replace-region"]
SUBSET_OF_3D.update({"create-solid": "mesh", "delete-solid": "mesh", "replace-solid": "mesh"})

TARGETS = [
    (ROOT / "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🧪️tests/mutate-fem2d-1/🥒️.feature", SUBSET_OF_2D),
    (ROOT / "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🧪️tests/mutate-fem3d-1/🥒️.feature", SUBSET_OF_3D),
]

GIVEN_AND_RE = re.compile(r"(asset://🏅️standards/🔖️1/🪆️subsets/)✳️any(/🧬️schema/🧬️mutations/<dir>)")


def rewrite_table(lines: list[str], id_to_subset: dict[str, str]) -> list[str]:
    """🔨️ Inserts a `subset` column into the `id-spec-vector` Examples table, right after `id`."""
    header_i = next(
        i for i, line in enumerate(lines)
        if line.lstrip().startswith("| id") and "dir" in line and "fixture" in line
    )
    header_cells = [c.strip() for c in lines[header_i].strip().strip("|").split("|")]
    assert header_cells == ["id", "dir", "fixture"], header_cells

    row_indices = []
    i = header_i + 1
    while i < len(lines) and lines[i].lstrip().startswith("|"):
        row_indices.append(i)
        i += 1

    rows = []
    for ri in row_indices:
        cells = [c.strip() for c in lines[ri].strip().strip("|").split("|")]
        mutation_id, dir_, fixture = cells
        subset = id_to_subset[mutation_id]
        rows.append((mutation_id, subset, dir_, fixture))

    columns = ["id", "subset", "dir", "fixture"]
    widths = [max(len(columns[c]), *(len(r[c]) for r in rows)) for c in range(4)]

    def fmt_row(cells: tuple[str, ...]) -> str:
        return "      | " + " | ".join(cell.ljust(widths[c]) for c, cell in enumerate(cells)) + " |"

    new_lines = list(lines)
    new_lines[header_i] = fmt_row(tuple(columns))
    for ri, row in zip(row_indices, rows):
        new_lines[ri] = fmt_row(row)
    return new_lines


def main() -> int:
    check = "--check" in sys.argv
    total = 0
    for path, id_to_subset in TARGETS:
        text = path.read_text(encoding="utf-8")
        lines = text.split("\n")

        given_hits = len(GIVEN_AND_RE.findall(text))
        text = GIVEN_AND_RE.sub(r"\1✳️<subset>\2", text)
        lines = text.split("\n")

        lines = rewrite_table(lines, id_to_subset)
        updated = "\n".join(lines)

        changed = updated != text if given_hits == 0 else True
        print(f"{'would fix' if check else 'fixed'} {given_hits:2d} Given/And segment(s) + table  {path.relative_to(ROOT)}")
        total += given_hits
        if not check:
            path.write_text(updated, encoding="utf-8")
    print(f"total Given/And segments repointed: {total}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
