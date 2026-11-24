import sys
import pymupdf

directory = sys.argv[1]
base = pymupdf.open(f"{directory}/large-input.pdf")
for name, text in (("large-rotation.pdf", False), ("large-text.pdf", True)):
    saved = pymupdf.open(f"{directory}/{name}")
    assert saved.page_count == base.page_count == 1600, (saved.page_count, base.page_count)
    assert saved[1000].rotation == 90 and saved[999].rotation == 0 and saved[0].rotation == 0
    assert saved.metadata["title"] == "Generated" and saved.metadata["author"] == "Fixture"
    for index in (0, 1, 999, 1000, 1599):
        assert saved[index].mediabox == base[index].mediabox
        expected = base[index].get_text()
        got = saved[index].get_text()
        assert got.startswith(expected.rstrip("\n")), (index, got, expected)
        assert ("Hello" in got) == (text and index == 0), (index, got)
    print("pymupdf", pymupdf.__version__, "accepts", name, saved.page_count)
