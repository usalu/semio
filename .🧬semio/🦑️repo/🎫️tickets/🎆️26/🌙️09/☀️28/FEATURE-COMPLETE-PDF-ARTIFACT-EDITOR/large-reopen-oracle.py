"""🔬 Independently check the portable law and lossless source image with JSON Schema and MuPDF."""
import hashlib
import json
from pathlib import Path

import jsonschema
import pymupdf

ticket = Path(__file__).resolve().parent
law = json.loads((ticket / "large-reopen-law.json").read_text())
schema = json.loads((ticket / "large-reopen-law.schema.json").read_text())
jsonschema.validate(law, schema)

source = ticket / "🗑️generated/large-reopen/entropy.pdf"
with pymupdf.open(source) as pdf:
    assert not pdf.is_repaired and len(pdf) == 1
    image, = pdf[0].get_images()
    pixels = pymupdf.Pixmap(pdf, image[0])
    assert (pixels.width, pixels.height, pixels.n) == (1536, 1152, 3)
    expected = hashlib.shake_256(b"semio.document.archive.reopen.rgb/v1").digest(1536 * 1152 * 3)
    assert hashlib.sha256(pixels.samples).digest() == hashlib.sha256(expected).digest()
print("[DEBUG] JSON Schema and MuPDF lossless RGB independent check passed")
