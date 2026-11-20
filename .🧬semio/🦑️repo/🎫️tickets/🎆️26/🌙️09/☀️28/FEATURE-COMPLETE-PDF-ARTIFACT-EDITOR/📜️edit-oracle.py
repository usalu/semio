"""🎨 Compare PDF paint edit fixture semantics with MuPDF."""
import json
import sys
from pathlib import Path

import fitz


def drawing_pair(directory, case):
    fixture = json.loads((directory / f"🧪️{case}.json").read_text(encoding="utf-8"))
    pair = []
    for key in ("before", "after"):
        with fitz.open() as document:
            page = document.new_page(width=240, height=240)
            stream = document.get_new_xref()
            document.update_object(stream, "<<>>")
            document.update_stream(stream, fixture[key].encode("ascii"))
            page.set_contents(stream)
            pair.append(page.get_drawings())
    return pair


def verify(directory):
    before, after = drawing_pair(directory, "edit-paint")
    assert [drawing["fill"] for drawing in before] == [(1, 0, 0)] * 3
    assert [drawing["fill"] for drawing in after] == [(1, 0, 0), (0, 1, 0), (1, 0, 0)]
    assert [drawing["rect"] for drawing in before] == [drawing["rect"] for drawing in after]
    before, after = drawing_pair(directory, "delete-path")
    assert len(before) == 2 and len(after) == 1
    assert (before[1]["rect"], before[1]["fill"]) == (after[0]["rect"], after[0]["fill"])
    before, after = drawing_pair(directory, "resize-curve")
    assert tuple(before[0]["rect"]) == (10, 180, 50, 220)
    assert tuple(after[0]["rect"]) == (30, 180, 110, 200)
    assert after[0]["items"][0][0] == "c"
    print("MuPDF verified isolated fill, path deletion, and transformed curve fixtures")


def verify_font(directory):
    fixture = json.loads((directory / "🧪️font-edit.json").read_text(encoding="utf-8"))
    with fitz.open() as document:
        page = document.new_page()
        resources = []
        for name, base in (("F1", fixture["base"]), ("F2", fixture["replacement"]), (fixture["occupied"], fixture["occupiedFont"])):
            font = document.get_new_xref()
            document.update_object(font, f"<< /Type /Font /Subtype /Type1 /BaseFont /{base} >>")
            resources.append(f"/{name} {font} 0 R")
        document.xref_set_key(page.xref, "Resources", "<< /Font << " + " ".join(resources) + " >> >>")
        stream = document.get_new_xref()
        document.update_object(stream, "<<>>")
        document.update_stream(stream, b"BT /F1 12 Tf 15 250 Td /F2 12 Tf (Hello) Tj /F1 12 Tf (Following) Tj ET")
        page.set_contents(stream)
        spans = page.get_text("dict")["blocks"][0]["lines"][0]["spans"]
        assert [span["font"] for span in spans] == fixture["expected"]
        assert [span["text"] for span in spans] == ["Hello", "Following"]
    print("MuPDF verified the selected text font and following text font")


def verify_text_move(directory):
    fixture = json.loads((directory / "🧪️move-text-run.json").read_text(encoding="utf-8"))
    points = []
    for stream_text in (fixture["before"], fixture["after"], fixture["resize"]["reference"], fixture["deleted"]):
        with fitz.open() as document:
            page = document.new_page(width=fixture["page"][0], height=fixture["page"][1])
            font = document.get_new_xref()
            document.update_object(font, f"<< /Type /Font /Subtype /Type1 /BaseFont /{fixture['font']} >>")
            document.xref_set_key(page.xref, "Resources", f"<< /Font << /F1 {font} 0 R >> >>")
            stream = document.get_new_xref()
            document.update_object(stream, "<<>>")
            document.update_stream(stream, stream_text.encode("ascii"))
            page.set_contents(stream)
            origins = {char["c"]: char["origin"] for block in page.get_text("rawdict")["blocks"]
                       for line in block["lines"] for span in line["spans"] for char in span["chars"]}
            points.append(origins)
    before, after, resized, deleted = points
    assert abs(before["B"][0] - fixture["followingX"]) < 1e-5
    x, y = before["A"]
    dx, dy = fixture["offset"]
    assert after["A"] == (x + dx, y - dy)
    assert all(abs(a - b) < 1e-5 for a, b in zip(after["B"], before["B"]))
    assert set(deleted) == {"B"}
    assert all(abs(a - b) < 1e-5 for a, b in zip(deleted["B"], before["B"]))
    assert resized["A"] == (fixture["resize"]["x"], fixture["page"][1] - fixture["resize"]["y"])
    assert all(abs(a - b) < 1e-5 for a, b in zip(resized["B"], before["B"]))
    print("MuPDF verified isolated text displacement and resizing under the original font metrics")


def verify_text_render_modes(directory):
    fixture = json.loads((directory / "🧪️text-rendering.json").read_text(encoding="utf-8"))
    for mode in fixture["modes"]:
        with fitz.open() as document:
            page = document.new_page(width=100, height=100)
            font = document.get_new_xref()
            document.update_object(font, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>")
            document.xref_set_key(page.xref, "Resources", f"<< /Font << /F1 {font} 0 R >> >>")
            stream = document.get_new_xref()
            document.update_object(stream, "<<>>")
            content = f"BT /F1 12 Tf 10 50 Td {mode['value']} Tr ({mode['label']}) Tj ET"
            document.update_stream(stream, content.encode("ascii"))
            page.set_contents(stream)
            events = {kind for kind, _ in page.get_bboxlog()}
            assert ("fill-text" in events) == mode["fill"], (mode, events)
            assert ("stroke-text" in events) == mode["stroke"], (mode, events)
    print("MuPDF verified fill and stroke visibility across the eight PDF text rendering modes")


if __name__ == "__main__":
    verify(Path(sys.argv[1]))
    verify_font(Path(sys.argv[1]))
    verify_text_move(Path(sys.argv[1]))
    verify_text_render_modes(Path(sys.argv[1]))
