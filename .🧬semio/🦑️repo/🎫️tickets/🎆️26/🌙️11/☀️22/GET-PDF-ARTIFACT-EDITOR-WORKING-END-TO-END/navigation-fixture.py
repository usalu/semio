#!/usr/bin/env python3
"""Writes the real-world-style navigation PDF fixture (outlines, name tree, catalog dests, page labels, open action,
viewer preferences, XMP metadata, structure root, Info extras, an indirect page group and more than eight link
annotations) and its schema-first spec; deterministic, standard library only.

usage: navigation-fixture.py <fixture directory>
"""
import json
import sys
from pathlib import Path


def pdf(objects, trailer):
    out = bytearray(b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n")
    offsets = []
    for number, body in enumerate(objects, 1):
        offsets.append(len(out))
        out += f"{number} 0 obj\n".encode() + body + b"\nendobj\n"
    start = len(out)
    out += f"xref\n0 {len(objects) + 1}\n0000000000 65535 f \n".encode()
    for offset in offsets:
        out += f"{offset:010} 00000 n \n".encode()
    out += f"trailer\n<< /Size {len(objects) + 1} {trailer} >>\nstartxref\n{start}\n%%EOF\n".encode()
    return bytes(out)


def stream(content, fields=""):
    body = content if isinstance(content, bytes) else content.encode("latin-1")
    return f"<< /Length {len(body)}{fields} >>\nstream\n".encode() + body + b"\nendstream"


def utf16(text):
    return "<FEFF" + text.encode("utf-16-be").hex().upper() + ">"


XMP = """<?xpacket begin="﻿" id="W5M0MpCehiHzreSzNTczkc9d"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/"><rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
<rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title><rdf:Alt><rdf:li xml:lang="x-default">Navigation fixture</rdf:li></rdf:Alt></dc:title></rdf:Description>
</rdf:RDF></x:xmpmeta>
<?xpacket end="w"?>"""


def navigation():
    page = {1: 3, 2: 4, 3: 5, 4: 6}
    links = [
        ("/A << /S /GoTo /D (chapter.3) >>", 3),
        (f"/Dest [{page[4]} 0 R /Fit]", 4),
        ("/A << /Type /Action /S /URI /URI (https://example.org/spec) >>", None),
    ] + [(f"/Dest [{page[1]} 0 R /XYZ 0 {280 - 10 * index} null]", 1) for index in range(7)]
    first_link = 23
    annots = " ".join(f"{first_link + index} 0 R" for index in range(len(links)))
    objects = {
        1: f"<< /Type /Catalog /Pages 2 0 R /Outlines 7 0 R /Names 8 0 R /Dests 9 0 R /PageLabels << /Nums [0 << /S /r >> 2 << /S /D /P (A-) /St 5 >>] >> /OpenAction [{page[1]} 0 R /Fit] /PageMode /UseOutlines /ViewerPreferences << /DisplayDocTitle true /Direction /L2R >> /Metadata 10 0 R /StructTreeRoot 11 0 R /MarkInfo << /Marked true >> /PieceInfo << /Semio << /LastModified (D:20260101000000Z) >> >> >>",
        2: f"<< /Type /Pages /Kids [{page[1]} 0 R {page[2]} 0 R {page[3]} 0 R {page[4]} 0 R] /Count 4 /MediaBox [0 0 300 300] /Resources << /Font << /F1 12 0 R >> /ProcSet [/PDF /Text] >> >>",
        3: "<< /Type /Page /Parent 2 0 R /Contents 40 0 R >>",
        4: f"<< /Type /Page /Parent 2 0 R /Contents 41 0 R /Annots [{annots}] >>",
        5: "<< /Type /Page /Parent 2 0 R /Contents 42 0 R /Group 20 0 R >>",
        6: "<< /Type /Page /Parent 2 0 R /Contents 43 0 R >>",
        7: "<< /Type /Outlines /First 13 0 R /Last 15 0 R /Count 3 >>",
        8: "<< /Dests 18 0 R >>",
        9: f"<< /appendix [{page[4]} 0 R /Fit] >>",
        10: stream(XMP.encode("utf-8"), " /Type /Metadata /Subtype /XML"),
        11: "<< /Type /StructTreeRoot >>",
        12: "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>",
        13: f"<< /Title (Introduction) /Parent 7 0 R /Next 14 0 R /Dest [{page[1]} 0 R /XYZ 0 300 0] >>",
        14: f"<< /Title {utf16('Über')} /Parent 7 0 R /Prev 13 0 R /Next 15 0 R /First 16 0 R /Last 17 0 R /Count 2 /A << /S /GoTo /D (chapter.2) >> >>",
        15: "<< /Title (Appendix) /Parent 7 0 R /Prev 14 0 R /Dest /appendix >>",
        16: "<< /Title (Part A) /Parent 14 0 R /Next 17 0 R /Dest (section.2a) >>",
        17: "<< /Title (Part B \\351) /Parent 14 0 R /Prev 16 0 R /A << /S /URI /URI (https://example.org/) >> /C [1 0 0] /F 2 >>",
        18: "<< /Kids [19 0 R 21 0 R] >>",
        19: f"<< /Limits [(chapter.2) (chapter.3)] /Names [(chapter.2) [{page[2]} 0 R /Fit] (chapter.3) 22 0 R] >>",
        20: "<< /Type /Group /S /Transparency /CS /DeviceRGB /I true >>",
        21: f"<< /Limits [(section.2a) (section.2a)] /Names [(section.2a) [{page[2]} 0 R /XYZ 10 250 null]] >>",
        22: f"<< /D [{page[3]} 0 R /FitH 280] >>",
    }
    for index, (target, _) in enumerate(links):
        objects[first_link + index] = f"<< /Type /Annot /Subtype /Link /Rect [20 {200 - 15 * index} 120 {212 - 15 * index}] /Border [0 0 0] /H /I /C [0 1 1] {target} >>"
    for number, label in zip((40, 41, 42, 43), ("Page one", "Page two", "Page three", "Page four")):
        objects[number] = stream(f"BT /F1 12 Tf 20 260 Td ({label}) Tj ET\n")
    objects[44] = f"<< /Title {utf16('Prüfbericht')} /Author (J\\366rg) /Producer (navigation-fixture) /PTEX.Fullbanner (This is a fixture) /Custom /Value >>"
    numbered = [objects.get(number, "null") for number in range(1, max(objects) + 1)]
    body = [value if isinstance(value, bytes) else value.encode("latin-1") for value in numbered]
    data = pdf(body, "/Root 1 0 R /Info 44 0 R /ID [<0123456789ABCDEF0123456789ABCDEF> <0123456789ABCDEF0123456789ABCDEF>]")
    spec = {
        "pageCount": 4,
        "text": ["Page one", "Page two", "Page three", "Page four"],
        "toc": [[1, "Introduction", 1], [1, "Über", 2], [2, "Part A", 2], [2, "Part B é", -1], [1, "Appendix", 4]],
        "namedDestinations": {"chapter.2": 2, "chapter.3": 3, "section.2a": 2, "appendix": 4},
        "pageLabels": ["i", "ii", "A-5", "A-6"],
        "openActionPage": 1,
        "links": [[], [[3, "goto"], [4, "goto"], [None, "uri"]] + [[1, "goto"]] * 7, [], []],
        "info": {"title": "Prüfbericht", "author": "Jörg", "extra": ["PTEX.Fullbanner", "Custom"]},
        "catalogExtra": ["StructTreeRoot", "PieceInfo"],
        "groupPage": 3,
    }
    return data, spec


def main():
    directory = Path(sys.argv[1])
    directory.mkdir(parents=True, exist_ok=True)
    data, spec = navigation()
    (directory / "navigation.pdf").write_bytes(data)
    (directory / "spec.json").write_text(json.dumps(spec, indent=2, ensure_ascii=False) + "\n")


main()
