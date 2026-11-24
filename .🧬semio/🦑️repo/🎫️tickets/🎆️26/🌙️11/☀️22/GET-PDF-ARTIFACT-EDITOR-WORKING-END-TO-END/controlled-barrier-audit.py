#!/usr/bin/env python3
"""Lists which structures of a real-world PDF the controlled lifter does not qualify, in the order it meets them.

usage: uv run --offline --with pymupdf==1.26.7 python controlled-barrier-audit.py <pdf>
"""
import collections
import sys

import fitz

CATALOG_OK = {"Type", "Pages", "PageLayout", "PageMode", "Version", "Lang", "MarkInfo"}
INFO_OK = {"Title", "Author", "Subject", "Keywords", "Creator", "Producer", "Trapped", "CreationDate", "ModDate"}
PAGE_OK = {"Type", "Parent", "Contents", "MediaBox", "CropBox", "Rotate", "Resources", "BleedBox", "TrimBox", "ArtBox", "UserUnit", "StructParents", "Dur", "Annots", "AA", "Group"}
RESOURCE_OK = {"Font", "XObject", "ProcSet"}

document = fitz.open(sys.argv[1])
catalog = document.pdf_catalog()
keys = document.xref_get_keys(catalog)
print("catalog keys not qualified:", sorted(set(keys) - CATALOG_OK))
trailer = document.pdf_trailer()
print("trailer:", trailer.replace("\n", " ")[:160])
info = document.xref_get_key(-1, "Info")
if info[0] == "xref":
    print("info keys not qualified:", sorted(set(document.xref_get_keys(int(info[1].split()[0]))) - INFO_OK))
pages = collections.Counter()
resources = collections.Counter()
fonts = collections.Counter()
xobjects = collections.Counter()
for page in document:
    for key in document.xref_get_keys(page.xref):
        if key not in PAGE_OK:
            pages[key] += 1
    resource = document.xref_get_key(page.xref, "Resources")
    node = page.xref
    while resource[0] == "null" and node:
        parent = document.xref_get_key(node, "Parent")
        node = int(parent[1].split()[0]) if parent[0] == "xref" else 0
        resource = document.xref_get_key(node, "Resources") if node else ("null", "")
    rx = int(resource[1].split()[0]) if resource[0] == "xref" else None
    keys = set(document.xref_get_keys(rx)) if rx else set()
    for key in keys:
        resources[key] += 1
    for font in page.get_fonts(full=True):
        fonts[(font[2], "embedded" if font[0] else "standard")] += 1
    for xobject in page.get_xobjects():
        xobjects["form"] += 1
    for image in page.get_images(full=True):
        xobjects["image"] += 1
print("page keys not qualified:", dict(pages))
print("resource categories per page:", dict(resources), "qualified:", sorted(RESOURCE_OK))
print("font kinds:", dict(fonts))
print("xobjects:", dict(xobjects))
print("annotations:", dict(collections.Counter(a.type[1] for page in document for a in page.annots() or [])))
print("pages:", document.page_count, "bytes:", sys.argv[1])
