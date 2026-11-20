"""📄 Builds and checks the large natural-file fixture using an independent PDF implementation."""

import sys
from pathlib import Path

import fitz

PAGE_COUNT = 48
ONE_TURN_INPUT = 4096


def write_fixture(destination):
    with fitz.open() as document:
        for index in range(PAGE_COUNT):
            page = document.new_page(width=360, height=480)
            page.insert_text((48, 64), f"Page {index + 1:02d} of {PAGE_COUNT}")
            drawing = page.new_shape()
            for line in range(48):
                horizontal = 45 + line * 5
                drawing.draw_line((horizontal, 85), (horizontal, 410))
            drawing.finish(width=0.5, color=(0.2, 0.25, 0.3), closePath=False)
            drawing.commit()
        document.set_metadata({"title": "Large PDF editor fixture"})
        document.save(destination, deflate=False)


def verify(destination, decoded=None):
    with fitz.open(destination) as document:
        assert not document.is_repaired
        assert document.page_count == PAGE_COUNT
        assert document.metadata["title"] == "Large PDF editor fixture"
        if decoded is not None:
            decoded.mkdir(exist_ok=True)
        for index, page in enumerate(document):
            assert page.get_text().strip() == f"Page {index + 1:02d} of {PAGE_COUNT}"
            assert [font[3:6] for font in page.get_fonts()] == [("Helvetica", "helv", "WinAnsiEncoding")]
            if decoded is not None:
                for stream in page.get_contents():
                    if document.xref_get_key(stream, "Filter")[0] != "null":
                        (decoded / str(stream)).write_bytes(document.xref_stream(stream))
        assert len(document[-1].get_drawings()[0]["items"]) == 48
        assert destination.stat().st_size > ONE_TURN_INPUT
        print(f"{destination.stat().st_size} bytes, {document.page_count} pages, xref size {document.xref_length()}")


if __name__ == "__main__":
    target = Path(sys.argv[1])
    write_fixture(target)
    verify(target, Path(sys.argv[2]) if len(sys.argv) > 2 else None)
