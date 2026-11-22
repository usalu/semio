"""📄 Create or verify the mounted invalid PDF fixture through a real PDF parser that refuses it."""
import argparse
import json
from pathlib import Path

import pymupdf

ticket = Path(__file__).resolve().parent
law = json.loads((ticket / "🔣️mounted-invalid-pdf.json").read_text(encoding="utf8"))


def generate(source):
    """🧱 Write the malformed bytes once and never replace an existing file."""
    with source.open("xb") as target:
        target.write((law["header"] + "\n" + law["body"]).encode("ascii"))


def verify(source):
    """🔎 Assert the exact malformed bytes exist and MuPDF yields no pages."""
    assert source.read_bytes() == (law["header"] + "\n" + law["body"]).encode("ascii")
    try:
        with pymupdf.open(source) as document:
            assert document.page_count == 0
    except pymupdf.FileDataError:
        pass
    print("[DEBUG] MuPDF could not open any pages from the malformed PDF")


cli = argparse.ArgumentParser()
cli.add_argument("mode", choices=["generate", "verify"])
cli.add_argument("file", type=Path)
args = cli.parse_args()
if args.mode == "generate":
    generate(args.file)
verify(args.file)
