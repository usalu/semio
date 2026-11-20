"""📐 Interprets the split page-content fixture as genuine PDF Contents streams in MuPDF."""
from pathlib import Path
import sys

import fitz


def verify(fixture, output):
    streams = fixture.read_bytes().split(b"\n%% STREAM\n")
    assert len(streams) == 2
    with fitz.open() as document:
        page = document.new_page(width=100, height=90)
        contents = []
        for stream in streams:
            ref = document.get_new_xref()
            document.update_object(ref, "<< >>")
            document.update_stream(ref, stream)
            contents.append(f"{ref} 0 R")
        document.xref_set_key(page.xref, "Contents", "[" + " ".join(contents) + "]")
        encoded = document.tobytes()
    output.write_bytes(encoded)
    with fitz.open(stream=encoded, filetype="pdf") as document:
        page = document[0]
        assert len(page.get_contents()) == 2
        assert [document.xref_stream(ref) for ref in page.get_contents()] == streams
        stroke, fill = page.get_drawings()
        assert stroke["type"] == "s" and stroke["color"] == (0.0, 0.0, 1.0)
        assert stroke["items"][0][1].x == 40.0 and stroke["items"][0][2].x == 80.0
        assert fill["type"] == "f" and fill["fill"] == (1.0, 0.0, 0.0)
        assert tuple(fill["rect"]) == (20.0, 55.0, 60.0, 70.0)
        pixels = page.get_pixmap()
        assert pixels.pixel(44, 80) == (0, 0, 255)
        assert pixels.pixel(52, 80) == (255, 255, 255)
        assert pixels.pixel(30, 62) == (255, 0, 0)
    print("MuPDF preserved both authored streams, placed the cross-stream line, and matched the dash and fill samples.")


if __name__ == "__main__":
    verify(*map(Path, sys.argv[1:]))
