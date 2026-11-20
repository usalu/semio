"""🦜 Compare the PDF editor vector fixture against PyMuPDF 1.24.14."""

import json
from pathlib import Path
import sys

import fitz


def verify(fixture_path):
    fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
    with fitz.open() as document:
        page = document.new_page(width=fixture["width"], height=fixture["height"])
        stream_ref = document.get_new_xref()
        document.update_object(stream_ref, "<<>>")
        document.update_stream(stream_ref, "\n".join(fixture["content"]).encode("ascii"))
        page.set_contents(stream_ref)
        drawings = page.get_drawings()
        assert len(drawings) == len(fixture["paths"]), drawings
        for drawing, expected in zip(drawings, fixture["paths"]):
            assert drawing["type"] == expected["type"], drawing
            if "rule" in expected:
                assert drawing["even_odd"] == (expected["rule"] == "evenodd"), drawing
            if "curves" in expected:
                curves = [
                    {"ctrl1": list(item[2]), "ctrl2": list(item[3]), "to": list(item[4])}
                    for item in drawing["items"]
                    if item[0] == "c"
                ]
                assert curves == expected["curves"], drawing
            else:
                rectangle = drawing["items"][0][1]
                assert [rectangle.x0, rectangle.y1] == expected["start"], drawing
                assert [rectangle.x1, rectangle.y0] == expected["end"], drawing
        assert drawings[0]["width"] == fixture["scaledStrokeWidth"]
        pixels = page.get_pixmap(colorspace=fitz.csRGB)
        for example in fixture["pixels"]:
            assert list(pixels.pixel(*example["at"])) == example["rgb"], example
    print(f"{len(drawings)} painted paths and {len(fixture['pixels'])} pixels match")


if __name__ == "__main__":
    verify(Path(sys.argv[1]))
