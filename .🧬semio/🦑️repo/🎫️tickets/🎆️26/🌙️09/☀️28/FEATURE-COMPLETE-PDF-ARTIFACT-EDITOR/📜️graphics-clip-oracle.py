"""🎨️ Verify graphics state clipping and restore on the shared PDF raster points."""
from pathlib import Path
import sys
import json
import fitz

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
with fitz.open() as doc:
    page = doc.new_page(width=100, height=100)
    stream = doc.get_new_xref()
    doc.update_object(stream, "<<>>")
    doc.update_stream(stream, b"q 1 0.5 0.5 1 20 20 cm 0 0 20 16 re W n 1 0 0 rg -30 -30 90 90 re f Q 1 0 0 rg 72 12 12 12 re f")
    page.set_contents(stream)
    raster = page.get_pixmap(colorspace=fitz.csRGB)
    for probe in fixture["probes"]:
        assert list(raster.pixel(*probe["at"])) == probe["rgb"], probe
    assert raster.pixel(78, 82) == (255, 0, 0)
print("MuPDF matched six clip probes and the post-restore vector")
