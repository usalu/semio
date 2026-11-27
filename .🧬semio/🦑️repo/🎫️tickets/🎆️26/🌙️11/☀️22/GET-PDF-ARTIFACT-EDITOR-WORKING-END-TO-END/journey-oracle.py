"""🧾️ Third-party oracle for the PDFs the pane journey saved (pymupdf and pypdf must both read them).

Usage: `uv run --offline --with pymupdf==1.26.7 --with pypdf python -I journey-oracle.py ops <pdf> <title> <marker>`
or `... journey-oracle.py all <journey out dir>` to validate every saved `*.pdf` of a run.
See ./🌐️pdf-browser-journey.md
"""
import pathlib
import re
import sys

import pymupdf
import pypdf


def xref_offsets(path):
    data = pathlib.Path(path).read_bytes()
    assert data.startswith(b"%PDF-") and data.rstrip().endswith(b"%%EOF"), path
    sections = 0
    for table in re.finditer(rb"xref\r?\n((?:\d+ \d+\r?\n(?:\d{10} \d{5} [nf][ \r\n]{2})+)+)trailer", data):
        sections += 1
        for sub in re.finditer(rb"(\d+) (\d+)\r?\n((?:\d{10} \d{5} [nf][ \r\n]{2})+)", table.group(1)):
            for index, entry in enumerate(re.findall(rb"(\d{10}) \d{5} ([nf])", sub.group(3))):
                if entry[1] == b"n":
                    number = int(sub.group(1)) + index
                    assert re.match(rb"%d \d+ obj" % number, data[int(entry[0]):]), (path, number, int(entry[0]))
    return sections


def read(path):
    mu = pymupdf.open(path)
    py = pypdf.PdfReader(path)
    assert mu.page_count == len(py.pages) > 0, (mu.page_count, len(py.pages))
    for index, page in enumerate(py.pages):
        page.extract_text()
        assert mu[index].get_text() is not None
    return mu, py


def ops(path, title, marker):
    mu, py = read(path)
    xref_offsets(path)
    assert mu.page_count == 2, mu.page_count
    assert mu.metadata["title"] == title == py.metadata.title, (mu.metadata, py.metadata)
    assert mu[1].rotation == 90 == py.pages[1].rotation, (mu[1].rotation, py.pages[1].rotation)
    assert [round(v) for v in mu[1].mediabox] == [0, 0, 595, 842], mu[1].mediabox
    assert [round(v) for v in mu[0].mediabox] == [0, 0, 612, 792], mu[0].mediabox
    assert marker in mu[1].get_text(), repr(mu[1].get_text())
    assert marker in py.pages[1].extract_text(), repr(py.pages[1].extract_text())
    assert any(drawing.get("fill") == (1.0, 0.0, 0.0) for drawing in mu[1].get_drawings()), mu[1].get_drawings()
    print(f"ok ops {path}: 2 pages, rotation 90, A4, text, red rectangle, title")


def every(folder):
    files = sorted(pathlib.Path(folder).glob("*.pdf"))
    assert files, f"no saved PDFs in {folder}"
    for file in files:
        mu, py = read(str(file))
        print(f"ok {file.name}: {xref_offsets(str(file))} xref sections with exact offsets, {mu.page_count} pages, pdf {py.pdf_header}, title {mu.metadata.get('title')!r}, text {mu[0].get_text().strip()[:60]!r}")


if sys.argv[1] == "ops":
    ops(*sys.argv[2:5])
else:
    every(sys.argv[2])
