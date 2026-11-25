#!/usr/bin/env python3
"""Writes the nested page-tree fixture: 5 pages under two intermediate page-tree nodes (the first carries the inherited MediaBox), an outline pointing into the second node, text on every page; spec in spec.json.

usage: uv run --offline --with pymupdf==1.26.7 python nested-fixture.py <fixture directory>
"""
import json
import sys
from pathlib import Path

import fitz

target = Path(sys.argv[1])
document = fitz.open()
texts = [f"Nested page {index}" for index in range(1, 6)]
for text in texts:
    page = document.new_page(width=300, height=300)
    page.insert_text((40, 150), text, fontname="helv", fontsize=14)
document.set_toc([[1, "Second node", 4]])
document.set_metadata({"title": "Nested fixture", "producer": "nested-fixture.py"})
root = int(document.xref_get_key(document.pdf_catalog(), "Pages")[1].split()[0])
pages = [document[index].xref for index in range(5)]
nodes = []
for members, inherited in ((pages[:3], "/MediaBox [0 0 300 300]"), (pages[3:], "")):
    node = document.get_new_xref()
    kids = " ".join(f"{page} 0 R" for page in members)
    document.update_object(node, f"<</Type/Pages/Parent {root} 0 R/Kids[{kids}]/Count {len(members)}{inherited}>>")
    for page in members:
        document.xref_set_key(page, "Parent", f"{node} 0 R")
        if inherited:
            document.update_object(page, document.xref_object(page, compressed=True).replace("/MediaBox[0 0 300 300]", ""))
    nodes.append(node)
document.xref_set_key(root, "Kids", "[" + " ".join(f"{node} 0 R" for node in nodes) + "]")
document.xref_set_key(root, "Count", "5")
target.mkdir(parents=True, exist_ok=True)
document.save(target / "nested.pdf", garbage=0, deflate=True, no_new_id=True)

reopened = fitz.open(target / "nested.pdf")
assert reopened.page_count == 5
for page, text in zip(reopened, texts):
    assert text in page.get_text(), (page.number, page.get_text())
    assert page.rect == fitz.Rect(0, 0, 300, 300)
assert reopened.get_toc() == [[1, "Second node", 4]]
(target / "spec.json").write_text(json.dumps({"pageCount": 5, "texts": texts, "nodes": [3, 2], "outline": [["Second node", 4]]}, indent=2) + "\n")
print("wrote", target / "nested.pdf")
