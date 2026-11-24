import sys
import pymupdf

kind, path, original = sys.argv[1], sys.argv[2], sys.argv[3]
saved = pymupdf.open(path)
base = pymupdf.open(original)
assert saved.page_count == base.page_count == 2, (saved.page_count, base.page_count)
if kind == "rotation":
    assert saved[1].rotation == 90, saved[1].rotation
    assert saved[0].rotation == base[0].rotation
    for index in range(2):
        assert saved[index].mediabox == base[index].mediabox
        assert saved[index].get_text() == base[index].get_text()
    assert saved.metadata["title"] == base.metadata["title"], saved.metadata
    assert saved.metadata["author"] == base.metadata["author"], saved.metadata
elif kind == "text":
    assert saved.metadata["title"] == "Edited Title", saved.metadata
    assert saved.metadata["author"] == base.metadata["author"], saved.metadata
    assert "Hello" in saved[0].get_text(), repr(saved[0].get_text())
    assert saved[1].get_text() == base[1].get_text() == ""
    assert saved[0].mediabox == base[0].mediabox and saved[1].mediabox == base[1].mediabox
    assert saved[1].rotation == base[1].rotation
else:
    raise SystemExit("unknown kind")
print("pymupdf", pymupdf.__version__, "accepts", kind, "pages", saved.page_count)
