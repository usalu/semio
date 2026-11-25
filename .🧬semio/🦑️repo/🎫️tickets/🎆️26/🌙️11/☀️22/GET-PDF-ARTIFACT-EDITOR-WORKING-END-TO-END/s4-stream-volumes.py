"""📊️ Independent (pymupdf) census of a PDF's streams: encoded and decoded bytes per stream kind, so the volume an import holds when every stream is decoded can be compared with the volume when only interpreted streams are.

usage: uv run --offline --with pymupdf==1.26.7 python -I s4-stream-volumes.py <file.pdf>..
"""
import sys

import pymupdf


def kind(document, xref):
    subtype = document.xref_get_key(xref, "Subtype")[1]
    kind_ = document.xref_get_key(xref, "Type")[1]
    if subtype == "/Image":
        return "image"
    if subtype == "/Form":
        return "form"
    if kind_ in ("/ObjStm", "/XRef"):
        return "structure"
    if kind_ == "/Metadata":
        return "metadata"
    if document.xref_get_key(xref, "Length1")[0] != "null" or subtype in ("/Type1C", "/CIDFontType0C", "/OpenType"):
        return "font program"
    return "other"


def census(path):
    document = pymupdf.open(path)
    contents = {xref for page in document for xref in page.get_contents()}
    totals = {}
    for xref in range(1, document.xref_length()):
        if not document.xref_is_stream(xref):
            continue
        name = "page content" if xref in contents else kind(document, xref)
        raw = len(document.xref_stream_raw(xref) or b"")
        try:
            decoded = len(document.xref_stream(xref) or b"")
        except Exception:
            decoded = raw
        entry = totals.setdefault(name, [0, 0, 0])
        entry[0] += 1
        entry[1] += raw
        entry[2] += decoded
    print(path)
    print(f"  {'kind':<14}{'streams':>8}{'encoded':>14}{'decoded':>14}")
    for name, (count, raw, decoded) in sorted(totals.items(), key=lambda item: -item[1][2]):
        print(f"  {name:<14}{count:>8}{raw:>14,}{decoded:>14,}")
    print(f"  {'total':<14}{sum(v[0] for v in totals.values()):>8}{sum(v[1] for v in totals.values()):>14,}{sum(v[2] for v in totals.values()):>14,}")


if __name__ == "__main__":
    for argument in sys.argv[1:]:
        census(argument)
