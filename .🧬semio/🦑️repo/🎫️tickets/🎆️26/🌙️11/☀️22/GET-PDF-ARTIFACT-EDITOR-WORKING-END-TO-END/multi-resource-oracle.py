#!/usr/bin/env python3
"""pymupdf oracle: per-page font names, image names and text of the multi-resource fixtures must equal the schema-first spec.

usage: uv run --offline --with pymupdf==1.26.7 python multi-resource-oracle.py <fixture directory>
"""
import json
import sys
from pathlib import Path

import fitz

directory = Path(sys.argv[1])
spec = json.loads((directory / "spec.json").read_text())
failures = 0
for name, case in spec.items():
    document = fitz.open(directory / f"{name}.pdf")
    assert document.page_count == len(case["pages"]), (name, document.page_count)
    fonts, images = set(), set()
    for index, (page, expected) in enumerate(zip(document, case["pages"])):
        page_fonts = sorted(font[4] for font in page.get_fonts())
        page_images = [image[7] for image in page.get_images(full=True)]
        text = [line.strip() for line in page.get_text().split("\n") if line.strip()]
        want_text = [line for line in expected["text"]]
        ok = (
            sorted(page_fonts) == sorted(expected["fonts"])
            and sorted(page_images) == sorted(expected["images"])
            and all(line in text for line in want_text)
        )
        print(name, index + 1, "fonts", page_fonts, "images", page_images, "text", len(text), "OK" if ok else "MISMATCH")
        failures += 0 if ok else 1
        fonts.update(font[0] for font in page.get_fonts())
        images.update(image[0] for image in page.get_images(full=True))
    assert len(fonts) == case["fontCount"], (name, "fonts", len(fonts))
    assert len(images) == case["imageCount"], (name, "images", len(images))
sys.exit(1 if failures else 0)
