"""🔮️ Third-party oracle of the views-and-history sweep: reads a saved PDF with pymupdf and pypdf and checks facts.

Usage: `uv run --offline --with pymupdf==1.26.7 --with pypdf python -I sweep-views-history-oracle.py <pdf> [title=X] [has=X] [lacks=X] [pages=N] [rotation=P:D] [box=P:l,b,r,t] [info=Key:Value]`
Prints the facts it read as one JSON line and exits 1 on the first mismatch.
"""
import json
import sys

import pymupdf
import pypdf

path = sys.argv[1]
mu = pymupdf.open(path)
py = pypdf.PdfReader(path)
assert mu.page_count == len(py.pages), (mu.page_count, len(py.pages))
text = "\n".join(page.get_text() for page in mu)
pytext = "\n".join(page.extract_text() for page in py.pages)
facts = {"pages": mu.page_count, "title": mu.metadata.get("title"), "pypdf_title": py.metadata.title if py.metadata else None, "text": text.strip()[:200], "rotations": [page.rotation for page in mu], "mediaboxes": [[round(v, 2) for v in page.mediabox] for page in mu], "cropboxes": [[round(v, 2) for v in page.cropbox] for page in mu], "info": {key: str(value) for key, value in (py.metadata or {}).items()}, "lang": str(py.trailer["/Root"].get("/Lang"))}
failures = []
for check in sys.argv[2:]:
    key, _, value = check.partition("=")
    if key == "title" and not (facts["title"] == value == facts["pypdf_title"]):
        failures.append(f"title {facts['title']!r}/{facts['pypdf_title']!r} != {value!r}")
    elif key == "has" and not (value in text and value in pytext):
        failures.append(f"text lacks {value!r}: {text.strip()[:120]!r}")
    elif key == "lacks" and (value in text or value in pytext):
        failures.append(f"text still has {value!r}")
    elif key == "pages" and mu.page_count != int(value):
        failures.append(f"pages {mu.page_count} != {value}")
    elif key == "rotation":
        page, _, degrees = value.partition(":")
        if mu[int(page)].rotation != int(degrees):
            failures.append(f"rotation {mu[int(page)].rotation} != {degrees}")
    elif key == "box":
        page, _, box = value.partition(":")
        if [round(v) for v in mu[int(page)].mediabox] != [int(v) for v in box.split(",")]:
            failures.append(f"mediabox {mu[int(page)].mediabox} != {box}")
    elif key == "info":
        name, _, expected = value.partition(":")
        if facts["info"].get("/" + name) != expected:
            failures.append(f"info {name} {facts['info'].get('/' + name)!r} != {expected!r}")
    elif key in ("pagehas", "pagelacks"):
        page, _, expected = value.partition(":")
        present = expected in mu[int(page)].get_text() and expected in py.pages[int(page)].extract_text()
        if present != (key == "pagehas"):
            failures.append(f"{key} {page} {expected!r}: page text {mu[int(page)].get_text().strip()[:80]!r}")
    elif key == "lang" and facts["lang"] != value:
        failures.append(f"lang {facts['lang']!r} != {value!r}")
print(json.dumps({"path": path, "facts": facts, "failures": failures}, ensure_ascii=False))
sys.exit(1 if failures else 0)
