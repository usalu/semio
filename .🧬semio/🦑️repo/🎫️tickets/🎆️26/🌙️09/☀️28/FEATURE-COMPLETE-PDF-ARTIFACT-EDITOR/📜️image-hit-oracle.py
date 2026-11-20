"""🖱️ Compare pointer hit probes with pixels from an independently rasterized PDF image."""

import json
import sys
from pathlib import Path
import fitz

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
with fitz.open() as doc:
    page = doc.new_page(width=100, height=100)
    image = doc.get_new_xref()
    doc.update_object(image, "<< /Type /XObject /Subtype /Image /Width 1 /Height 1 /ColorSpace /DeviceRGB /BitsPerComponent 8 >>")
    doc.update_stream(image, bytes((255, 0, 0)))
    content = doc.get_new_xref()
    doc.update_object(content, "<<>>")
    doc.update_stream(content, b"q 20 10 8 16 20 20 cm /Im1 Do Q")
    doc.xref_set_key(page.xref, "Resources", f"<< /XObject << /Im1 {image} 0 R >> >>")
    page.set_contents(content)
    bitmap = page.get_pixmap(colorspace=fitz.csRGB)
    for probe in fixture["probes"]:
        assert list(bitmap.pixel(*probe["at"])) == probe["rgb"], probe
    print(f"Matched {len(fixture['probes'])} skewed image probes")
