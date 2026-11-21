"""🔍 Independently reopen the PDF streamed into the framework segmented output queue."""

import hashlib
import json
from pathlib import Path

import pymupdf

TICKET = Path(__file__).resolve().parent
assets = TICKET / "🗑️generated" / "paged-jpeg-assets"
witness = json.loads((assets / "fixture.json").read_text(encoding="utf8"))
expected = next(row for row in witness["valid"] if row["name"] == "large")
source = assets / "large.stream.segmented.pdf"
with pymupdf.open(source) as pdf:
    assert pdf.page_count == 1 and not pdf.is_repaired
    assert pdf.metadata["producer"] == "Bounded PDF writer probe"
    assert pdf.xref_get_key(5, "Filter") == ("name", "/DCTDecode")
    assert pdf.xref_get_key(5, "DecodeParms/ColorTransform") == ("int", "1")
    assert hashlib.sha256(pdf.xref_stream_raw(5)).hexdigest() == expected["encodedSha256"]
    clip = pymupdf.Rect(witness["crop"])
    rendered = pdf[0].get_pixmap(clip=clip, colorspace=pymupdf.csRGB, alpha=False)
    assert rendered.samples.hex() == expected["muPdfCropRgb"]
print(f"[DEBUG] Reopened all {source.stat().st_size} Save queue bytes with the same compressed image and crop")
