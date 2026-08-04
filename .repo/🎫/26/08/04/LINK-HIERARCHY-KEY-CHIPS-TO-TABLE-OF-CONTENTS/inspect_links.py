#!/usr/bin/env python3
"""🔎 Verify hierarchy key chips link back to their Inhaltsverzeichnis line.

`\\label{semio-tocline-<number>}` records the *auto* hyperref anchor of the TOC
row (e.g. `section*.7`), not a destination literally named after the label — so
this reads `<job>.aux` to map tocline label -> anchor, then walks every /GoTo
link annotation in the PDF and reports which ones target those anchors, on
which page they sit, and which page the destination lands on.
"""
import re
import sys
from pathlib import Path

from pypdf import PdfReader
from pypdf.generic import ArrayObject, IndirectObject

NEWLABEL = re.compile(r"\\newlabel\{(semio-tocline-[^}]*)\}\{\{[^}]*\}\{([^}]*)\}\{[^}]*\}\{([^}]*)\}")


def resolve(obj):
    while isinstance(obj, IndirectObject):
        obj = obj.get_object()
    return obj


def tocline_anchors(aux):
    return {match.group(3): (match.group(1), match.group(2)) for match in NEWLABEL.finditer(aux.read_text("utf8", errors="replace"))}


def dest_pages(reader):
    pages = {}
    for name, dest in (reader.named_destinations or {}).items():
        raw = getattr(dest, "dest_array", None)
        target = resolve(raw[0]) if isinstance(raw, ArrayObject) and raw else None
        for index, page in enumerate(reader.pages):
            if target is not None and resolve(page.indirect_reference) == target:
                pages[str(name)] = index + 1
                break
    return pages


def main(pdf):
    pdf = Path(pdf)
    reader = PdfReader(pdf)
    anchors = tocline_anchors(pdf.with_suffix(".aux"))
    pages = dest_pages(reader)

    print(f"== {pdf}")
    print(f"tocline anchors in .aux: {len(anchors)}")
    for anchor, (label, page) in sorted(anchors.items()):
        print(f"  {label:<28} -> anchor {anchor:<16} (aux page {page}, pdf page {pages.get(anchor)})")

    hits = []
    for index, page in enumerate(reader.pages):
        for annot in resolve(page.get("/Annots") or []):
            annot = resolve(annot)
            action = resolve(annot.get("/A") or {})
            target = resolve(action.get("/D"))
            if isinstance(target, str) and target in anchors:
                hits.append((index + 1, target))

    print(f"chip -> tocline link annotations: {len(hits)}")
    for page_no, anchor in hits:
        label = anchors[anchor][0]
        print(f"  page {page_no:>3}: chip {label.removeprefix('semio-tocline-'):<12} -> {anchor} on toc page {pages.get(anchor)}")

    unlinked = sorted({label for label, _ in anchors.values()} - {anchors[a][0] for _, a in hits})
    print(f"toc lines with no chip pointing at them: {unlinked}")


if __name__ == "__main__":
    for arg in sys.argv[1:]:
        main(arg)
