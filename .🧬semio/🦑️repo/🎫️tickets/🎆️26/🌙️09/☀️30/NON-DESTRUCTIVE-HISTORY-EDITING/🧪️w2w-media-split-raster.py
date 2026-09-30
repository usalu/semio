#!/usr/bin/env python3
"""🧩️ W2-W media: moves whole-raster rows into their own outlines on a small committed document.

A whole-raster leaf's wire payload is the full replacement raster, which no Examples cell can hold for a real
multi-megapixel document. For each `@id-mutate` / `@id-inverse` Scenario Outline of `<feature>` this inserts a
sibling outline `@id-<base>-raster` whose `Given` names `<uri>` and whose table holds exactly the `<kind>` rows of
`<rows-file>`; the original outlines lose those rows. Titles take `<subject>` in place of their document phrase.

Usage: python3 🧪️w2w-media-split-raster.py <feature> <rows-file> <uri> <given-phrase> <title-phrase> <kind>…
"""
import re
import sys
from pathlib import Path


def main():
    feature, rows_file, uri, given_phrase, title_phrase, *kinds = sys.argv[1:]
    rows = [line for line in Path(rows_file).read_text().splitlines() if any(f"| {kind} |" in line for kind in kinds)]
    assert len(rows) == len(kinds), (rows, kinds)
    text = Path(feature).read_text()
    for base in ("mutate", "inverse"):
        block = re.search(r"(  @id-" + base + r"\n(?:  @[^\n]*\n)*  Scenario Outline: ([^\n]*)\n(?:    [^\n]*\n)*?    Examples:\n      \| id \|[^\n]*\n)((?:      \|[^\n]*\n)*)\n", text)
        assert block, (feature, base)
        head, title, table = block.group(1), block.group(2), block.group(3)
        kept = "".join(line + "\n" for line in table.splitlines() if not any(f"| {kind} |" in line for kind in kinds))
        clone = head.replace(f"  @id-{base}\n", f"  @id-{base}-raster\n", 1)
        clone = re.sub(r"Scenario Outline: [^\n]*", "Scenario Outline: " + re.sub(r"\b(the|a) [^<]*$", title_phrase, title), clone, count=1)
        clone = re.sub(r"    Given [^\n]*", f"    Given {given_phrase} {uri}", clone, count=1)
        clone += "".join(row + "\n" for row in rows)
        text = text[: block.start()] + head + kept + "\n" + clone + "\n" + text[block.end():]
    Path(feature).write_text(text)


if __name__ == "__main__":
    main()
