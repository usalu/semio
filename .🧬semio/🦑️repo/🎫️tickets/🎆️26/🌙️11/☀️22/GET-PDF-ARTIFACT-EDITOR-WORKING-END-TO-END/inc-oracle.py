import sys, fitz
for path in sys.argv[1:]:
    doc = fitz.open(path)
    print(path.split("/")[-1], "pages", doc.page_count, "title", doc.metadata.get("title"), "p1", repr(doc[0].get_text().strip()[:40]), "p2", repr(doc[1].get_text().strip()[:40]) if doc.page_count > 1 else None)
