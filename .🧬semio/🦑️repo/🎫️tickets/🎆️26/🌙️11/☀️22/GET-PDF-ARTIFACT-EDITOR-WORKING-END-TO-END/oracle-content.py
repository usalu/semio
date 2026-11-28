"""🧾️ Third-party oracle (pymupdf and pypdf) for the PDFs saved by the content and page action sweep.

Usage: `uv run --offline --with pymupdf==1.26.7 --with pypdf python -I oracle-content.py <check> <pdf> [args]` where check is `rail`, `inline`, `pages`, `extra` or `dump`.
"""
import json
import pathlib
import re
import sys

import pymupdf
import pypdf


def xref_offsets(path):
    data = pathlib.Path(path).read_bytes()
    assert data.startswith(b"%PDF-") and data.rstrip().endswith(b"%%EOF"), path
    for table in re.finditer(rb"xref\r?\n((?:\d+ \d+\r?\n(?:\d{10} \d{5} [nf][ \r\n]{2})+)+)trailer", data):
        for sub in re.finditer(rb"(\d+) (\d+)\r?\n((?:\d{10} \d{5} [nf][ \r\n]{2})+)", table.group(1)):
            for index, entry in enumerate(re.findall(rb"(\d{10}) \d{5} ([nf])", sub.group(3))):
                if entry[1] == b"n":
                    number = int(sub.group(1)) + index
                    assert re.match(rb"%d \d+ obj" % number, data[int(entry[0]):]), (path, number, int(entry[0]))


def read(path):
    xref_offsets(path)
    mu = pymupdf.open(path)
    py = pypdf.PdfReader(path)
    assert mu.page_count == len(py.pages) > 0, (mu.page_count, len(py.pages))
    for index, page in enumerate(py.pages):
        page.extract_text()
        pixmap = mu[index].get_pixmap()
        assert pixmap.width > 0 and pixmap.height > 0
    return mu, py


def dump(path):
    mu, py = read(path)
    for page in mu:
        print("page", page.number, page.rect, "rot", page.rotation, "crop", page.cropbox)
        print(" text", repr(page.get_text()))
        for drawing in page.get_drawings():
            print(" drawing", drawing.get("type"), drawing.get("fill"), drawing.get("color"), drawing.get("width"), drawing["rect"])
        print(" fonts", page.get_fonts())
        print(" images", page.get_images(), page.get_image_info())
        for block in page.get_text("dict")["blocks"]:
            for line in block.get("lines", []):
                for span in line["spans"]:
                    print(" span", span["text"], span["font"], round(span["size"], 2), [round(v) for v in span["bbox"]])
    print("labels", mu.get_page_labels())


def near(a, b, tolerance=1.5):
    return all(abs(x - y) <= tolerance for x, y in zip(a, b))


def spans(page):
    return [span for block in page.get_text("dict")["blocks"] for line in block.get("lines", []) for span in line["spans"]]


def rail(path):
    mu, py = read(path)
    assert mu.page_count == 2
    one = mu[0]
    by_text = {span["text"]: span for span in spans(one)}
    assert set(by_text) == {"Rich Edited", "Second font Times", "Alpha Marker"}, set(by_text)
    assert near(by_text["Alpha Marker"]["origin"], (300, 792 - 600)), by_text["Alpha Marker"]["origin"]
    assert by_text["Alpha Marker"]["size"] == 14
    assert by_text["Rich Edited"]["size"] == 30, by_text["Rich Edited"]["size"]
    assert by_text["Second font Times"]["font"] == "Courier", by_text["Second font Times"]["font"]
    assert near(by_text["Second font Times"]["origin"], (200, 792 - 400)), by_text["Second font Times"]["origin"]
    drawings = one.get_drawings()
    cyan = [d for d in drawings if d.get("fill") == (0.0, 1.0, 1.0)]
    assert len(cyan) == 1 and near(tuple(cyan[0]["rect"]), (72, 792 - 550, 372, 792 - 500)), cyan
    green_line = [d for d in drawings if d.get("color") == (0.0, 0.5, 0.0)]
    assert len(green_line) == 1 and green_line[0]["width"] == 5, green_line
    green = [d for d in drawings if d.get("fill") == (0.0, 1.0, 0.0)]
    assert len(green) == 1 and near(tuple(green[0]["rect"]), (300, 792 - 150, 400, 792 - 100)), green
    assert not [d for d in drawings if d.get("color") == (1.0, 0.0, 1.0)], "deleted magenta line is still drawn"
    assert not [d for d in drawings if d.get("fill") == (1.0, 0.0, 0.0)], "recoloured rectangle is still red"
    info = one.get_image_info()
    assert len(info) == 1 and (info[0]["width"], info[0]["height"]) == (2, 2), info
    pix = pymupdf.Pixmap(mu, one.get_images()[0][0])
    samples = list(pix.samples)
    assert samples[:12] == [255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0], samples[:12]
    assert "Page two body" in mu[1].get_text()
    print("ok rail: 2 pages, text/size/font/position, fill, stroke, rectangle, deleted line, 2x2 image samples")


CHECKS = {"dump": dump, "rail": rail}
if __name__ == "__main__":
    CHECKS[sys.argv[1]](*sys.argv[2:])
