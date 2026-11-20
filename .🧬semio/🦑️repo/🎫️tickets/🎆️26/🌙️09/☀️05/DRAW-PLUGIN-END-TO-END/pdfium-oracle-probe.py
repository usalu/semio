"""🔮️ Samples the two Draw PDF pixel fixtures using the repository's pinned PDFium wheel."""
from pathlib import Path
import sys

import pypdfium2 as pdfium
from pypdfium2 import raw


def sample(data: bytes) -> bytes:
    pixels = bytearray()
    with pdfium.PdfDocument(data) as document:
        if len(document) != 1:
            raise ValueError("expected one Draw PDF page")
        page = document[0]
        try:
            bitmap = page.render(scale=256, force_bitmap_format=raw.FPDFBitmap_BGRA)
            try:
                if (bitmap.width, bitmap.height) != (512, 256):
                    raise ValueError("unexpected Draw PDF raster extent")
                view = memoryview(bitmap.buffer).cast("B")
                for x in (128, 384):
                    offset = 128 * bitmap.stride + x * 4
                    blue, green, red, alpha = view[offset : offset + 4]
                    pixels.extend((red, green, blue, alpha))
            finally:
                bitmap.close()
        finally:
            page.close()
    return bytes(pixels)


if len(sys.argv) == 1:
    sys.stdout.buffer.write(sample(sys.stdin.buffer.read()))
else:
    for path in map(Path, sys.argv[1:]):
        print(path.name, *sample(path.read_bytes()))
