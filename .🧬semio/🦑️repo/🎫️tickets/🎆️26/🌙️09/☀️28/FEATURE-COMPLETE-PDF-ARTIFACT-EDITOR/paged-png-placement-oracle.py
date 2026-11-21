"""🔬 Compare native PDF plane writer output to the original PNG through Pillow and MuPDF."""
from pathlib import Path
import hashlib
import struct

from PIL import Image
import pymupdf

generated = Path(__file__).resolve().parent / "🗑️generated"
with Image.open(generated / "paged-png" / "paid-rgba.png") as original:
    assert original.mode == "RGBA"
    width, height = original.size
    color = original.convert("RGB")
    alpha = original.getchannel("A")
    expected = Image.composite(color, Image.new("RGB", original.size, "white"), alpha)
    assert (width, height) == (512, 512)
    hash_prefix = b"s.stdio.pdf.png-placement.v1\n" + struct.pack("<IIBB", width, height, 3, 1)
    digest = hashlib.sha256(hash_prefix + color.tobytes() + alpha.tobytes()).hexdigest()
    assert (generated / "png-placement" / "pair.keys").read_text(encoding="utf8").splitlines() == [
        f"pdf:s.stdio.png-plane:sha256:{digest}:color",
        f"pdf:s.stdio.png-plane:sha256:{digest}:alpha",
    ]
    path = generated / "png-placement" / "planes.pdf"
    with path.open("rb") as data:
        assert b"/SMask 6 9 R" in data.read(640)
        data.seek(-450, 2)
        assert b" 00009 n " in data.read()
    archive_color = generated / "png-placement" / "reopened.color"
    archive_alpha = generated / "png-placement" / "reopened.alpha"
    if archive_color.exists() and archive_alpha.exists():
        assert archive_color.read_bytes() == color.tobytes()
        assert archive_alpha.read_bytes() == alpha.tobytes()
    with pymupdf.open(path) as pdf:
        assert pdf.page_count == 1 and not pdf.is_repaired
        kind, target = pdf.xref_get_key(5, "SMask")
        assert kind == "xref" and target.split()[0] == "6", (kind, target)
        assert pdf.xref_stream_raw(5) == color.tobytes()
        assert pdf.xref_stream_raw(6) == alpha.tobytes()
        actual = pdf[0].get_pixmap(colorspace=pymupdf.csRGB, alpha=False)
        assert (actual.width, actual.height) == original.size
        maximum = max(abs(left - right) for left, right in zip(actual.samples, expected.tobytes(), strict=True))
        assert maximum <= 2, maximum
    print(f"[DEBUG] Independent exact paired digest and PDF stream octets; largest composited pixel difference {maximum}/255")
