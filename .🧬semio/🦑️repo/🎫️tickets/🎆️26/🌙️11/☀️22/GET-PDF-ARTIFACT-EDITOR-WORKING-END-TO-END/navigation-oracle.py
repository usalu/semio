#!/usr/bin/env python3
"""pymupdf oracle: the navigation fixture and every file saved from it must keep the schema-first spec's outline,
named destinations, page labels, open action, Info, catalog extras, link annotations and page text.

usage: uv run --offline --with pymupdf==1.26.7 python navigation-oracle.py <spec.json> <pdf> [<pdf> ...]
       a saved file may carry an inserted text and an edited title; pass `--edited <title> <text>` before it
"""
import json
import re
import sys

import fitz

spec = json.loads(open(sys.argv[1], encoding="utf-8").read())
arguments = sys.argv[2:]
failures = 0


def check(name, condition, detail):
    global failures
    print(f"{name}: {'OK' if condition else 'MISMATCH'} {detail}")
    failures += 0 if condition else 1


edited = None
while arguments:
    if arguments[0] == "--edited":
        edited = (arguments[1], arguments[2])
        arguments = arguments[3:]
        continue
    path, arguments = arguments[0], arguments[1:]
    document = fitz.open(path)
    label = path.rsplit("/", 1)[-1]
    check(label, document.page_count == spec["pageCount"], f"pages {document.page_count}")
    texts = [page.get_text().strip() for page in document]
    for index, (text, expected) in enumerate(zip(texts, spec["text"])):
        wanted = expected in text and (edited is None or index != 0 or f"({edited[1]}) Tj".encode() in document[index].read_contents())
        check(label, wanted, f"page {index + 1} text {text!r}")
    toc = [[level, title, page if explicit[2] > 0 else explicit[2]] for (level, title, page), explicit in zip(document.get_toc(), spec["toc"])]
    check(label, [entry[:2] for entry in document.get_toc()] == [entry[:2] for entry in spec["toc"]], f"outline titles {document.get_toc()}")
    check(label, all(entry[2] == expected[2] for entry, expected in zip(toc, spec["toc"]) if expected[2] in (1, 4)), "outline explicit destinations")
    names = {name: target["page"] + 1 for name, target in document.resolve_names().items()}
    check(label, names == spec["namedDestinations"], f"named destinations {names}")
    check(label, [page.get_label() for page in document] == spec["pageLabels"], "page labels")
    opened = document.xref_get_key(document.pdf_catalog(), "OpenAction")
    first = document.page_xref(spec["openActionPage"] - 1)
    check(label, re.match(rf"\[\s*{first} 0 R", opened[1]) is not None, f"open action {opened}")
    title = edited[0] if edited else spec["info"]["title"]
    check(label, document.metadata["title"] == title and document.metadata["author"] == spec["info"]["author"], f"info {document.metadata['title']!r} {document.metadata['author']!r}")
    info = int(document.xref_get_key(-1, "Info")[1].split()[0])
    keys = document.xref_get_keys(info)
    check(label, all(key in keys for key in spec["info"]["extra"]), f"info extras {keys}")
    catalog = document.xref_get_keys(document.pdf_catalog())
    check(label, all(key in catalog for key in spec["catalogExtra"] + ["Outlines", "Names", "Dests", "PageLabels", "Metadata", "ViewerPreferences"]), f"catalog {catalog}")
    check(label, "Navigation fixture" in (document.get_xml_metadata() or ""), "xmp metadata")
    for index, expected in enumerate(spec["links"]):
        annots = document.xref_get_key(document.page_xref(index), "Annots")
        count = len(re.findall(r"\d+ 0 R", annots[1])) if annots[0] == "array" else 0
        check(label, count == len(expected), f"page {index + 1} link annotations {count}")
    group = document.xref_get_key(document.page_xref(spec["groupPage"] - 1), "Group")
    check(label, group[0] == "xref", f"indirect page group {group}")
sys.exit(1 if failures else 0)
