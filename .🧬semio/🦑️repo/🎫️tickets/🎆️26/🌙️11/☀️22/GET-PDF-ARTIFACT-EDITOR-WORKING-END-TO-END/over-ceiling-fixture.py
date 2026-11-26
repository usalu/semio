#!/usr/bin/env python3
"""Writes the over-ceiling fixture (slice 13): a 30-page PDF whose pages each hold 14,000 save and restore operators (7,000 pairs, a few hundred bytes of Flate stream each), so the typed pages (about 24 MB of operators and as much again for the roles that mirror them) pass the 32 MiB natural-file output ceiling midway through the document while the file stays at a few KB; an unreferenced 8 KB stream lifts the file over the 4,096-byte limit under which the route decodes synchronously. A page program above about 18,000 operators cannot be typed under the browser's 1 MiB empty-backing grant at all (the operator vector doubles past it), which is why the operators are spread over pages.

usage: python3 -I over-ceiling-fixture.py <fixtures directory>
"""
import json
import sys
import zlib
from pathlib import Path

PAGES = 30
OPERATORS_PER_PAGE = 14_000
CEILING = 33_554_432
PADDING = bytes((index * 7919 + 13) % 256 for index in range(8192))


def build() -> tuple[bytes, int]:
    content = ("q Q " * (OPERATORS_PER_PAGE // 2) + "\n").encode("ascii")
    packed = zlib.compress(content, 9)
    first_page = 3
    kids = " ".join(f"{first_page + index} 0 R" for index in range(PAGES))
    bodies = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        f"<< /Type /Pages /Kids [{kids}] /Count {PAGES} >>".encode(),
    ]
    contents_first = first_page + PAGES
    for index in range(PAGES):
        bodies.append(f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << >> /Contents {contents_first + index} 0 R >>".encode())
    for _ in range(PAGES):
        bodies.append(b"<< /Filter /FlateDecode /Length " + str(len(packed)).encode() + b" >>\nstream\n" + packed + b"\nendstream")
    bodies.append(b"<< /Length " + str(len(PADDING)).encode() + b" >>\nstream\n" + PADDING + b"\nendstream")
    out = bytearray(b"%PDF-1.7\n")
    offsets = []
    for number, body in enumerate(bodies, start=1):
        offsets.append(len(out))
        out += f"{number} 0 obj\n".encode() + body + b"\nendobj\n"
    start = len(out)
    out += f"xref\n0 {len(bodies) + 1}\n".encode() + b"0000000000 65535 f \n"
    for offset in offsets:
        out += f"{offset:010d} 00000 n \n".encode()
    out += f"trailer\n<< /Size {len(bodies) + 1} /Root 1 0 R >>\nstartxref\n{start}\n%%EOF\n".encode()
    return bytes(out), len(content)


def main() -> None:
    directory = Path(sys.argv[1]) / "🛑️over-ceiling"
    directory.mkdir(parents=True, exist_ok=True)
    data, decoded = build()
    (directory / "over-ceiling.pdf").write_bytes(data)
    spec = {"pageCount": PAGES, "operatorsPerPage": OPERATORS_PER_PAGE, "decodedBytesPerPage": decoded, "paddingBytes": len(PADDING), "fileBytes": len(data), "ceiling": CEILING}
    (directory / "spec.json").write_text(json.dumps(spec, indent=2) + "\n")
    print(spec)


main()
