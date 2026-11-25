#!/usr/bin/env python3
"""pymupdf oracle for the font fixture: page count, per-page font names and subtypes, embedded program kinds and per-page text must equal the schema-first spec, for the original and (optionally) the saved file, which may only add fonts and text.

usage: uv run --offline --with pymupdf==1.26.7 python font-oracle.py <fixture directory> [saved.pdf]
"""
import json
import sys
from pathlib import Path

import fitz

directory = Path(sys.argv[1])
spec = json.loads((directory / "spec.json").read_text())
failures = 0
for path in [directory / "fonts.pdf", *map(Path, sys.argv[2:])]:
    document = fitz.open(path)
    assert document.page_count == len(spec["pages"]), (path, document.page_count)
    for index, (page, expected) in enumerate(zip(document, spec["pages"])):
        fonts = {font[4]: font for font in page.get_fonts(full=True)}
        lines = [line.strip() for line in page.get_text().split("\n") if line.strip()]
        checks = {
            "fonts": all(name in fonts for name in expected["fonts"]),
            "kinds": all(fonts[name][2] == kind for name, kind in zip(expected["fonts"], expected["kinds"]) if name in fonts),
            "programs": all((fonts[name][1] != "n/a") == (spec["programs"][name] is not None) for name in expected["fonts"] if name in fonts),
            "text": all(any(text in line for line in lines) for text in expected["text"]),
        }
        ok = all(checks.values())
        failures += 0 if ok else 1
        print(path.name, index + 1, {key: "OK" if value else "MISMATCH" for key, value in checks.items()}, sorted(fonts))
sys.exit(1 if failures else 0)
