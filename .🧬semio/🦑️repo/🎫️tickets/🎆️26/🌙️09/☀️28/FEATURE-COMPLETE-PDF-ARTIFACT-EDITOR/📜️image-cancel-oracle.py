"""🛑 Independently check that a cancelled PNG decode left the saved PDF byte-identical and structurally intact."""
import argparse
from pathlib import Path

import fitz

cli = argparse.ArgumentParser()
cli.add_argument("directory", type=Path)
directory = cli.parse_args().directory
names = ["decode-cancellation-before.pdf", "decode-cancellation-after.pdf", "decode-cancellation-undone.pdf"]
saved = [(directory / name).read_bytes() for name in names]
assert saved[0] == saved[1] == saved[2], "a cancelled or undone decode changed the saved bytes"
with fitz.open(directory / names[0]) as book:
    assert not book.is_repaired and len(book) == 2
    images = sum(len(page.get_images()) for page in book)
assert images == 2, images
with fitz.open(directory / "image-source.pdf") as source:
    assert sum(len(page.get_images()) for page in source) == images
print("[DEBUG] MuPDF verified before, cancelled and undone saves are identical with the two original images")
