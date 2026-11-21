"""🖼️ Generates native JPEG controls and validates their PDF embedding and crop independently."""
import hashlib
import importlib.util
import io
import json
import random
import sys
from pathlib import Path

import pymupdf
from PIL import Image

TICKET = Path(__file__).resolve().parent
ASSETS = TICKET / "🗑️generated" / "paged-jpeg-assets"
CROP = (13, 12, 24, 22)

spec = importlib.util.spec_from_file_location("reference_pdf", TICKET / "controlled-dct-oracle.py")
reference_pdf = importlib.util.module_from_spec(spec)
spec.loader.exec_module(reference_pdf)


def jpeg(mode, size, seed, **options):
    source = random.Random(seed).randbytes(size[0] * size[1] * len(mode))
    image = Image.frombytes(mode, size, source)
    encoded = io.BytesIO()
    image.save(encoded, format="JPEG", quality=92, subsampling=0, **options)
    return encoded.getvalue()


def sha(data):
    return hashlib.sha256(data).hexdigest()


def pdf_image(original, width, height, components, transform):
    space = b"DeviceGray" if components == 1 else b"DeviceRGB"
    attributes = (f"/Type /XObject /Subtype /Image /Width {width} /Height {height} /BitsPerComponent 8".encode()
                  + b" /ColorSpace /" + space + b" /Filter /DCTDecode")
    if transform is not None:
        attributes += f" /DecodeParms << /ColorTransform {transform} >>".encode()
    return reference_pdf.document([
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        (f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {width} {height}] "
         "/Resources << /XObject << /Im 5 0 R >> >> /Contents 4 0 R >>").encode(),
        reference_pdf.stream(b"", f"q {width} 0 0 {height} 0 0 cm /Im Do Q".encode()),
        reference_pdf.stream(attributes, original),
    ])


def assert_near(first, second, tolerance, label):
    assert len(first) == len(second), (label, len(first), len(second))
    delta = max(abs(a - b) for a, b in zip(first, second, strict=True))
    assert delta <= tolerance, (label, delta, tolerance)
    return delta


def valid(name, data, transform):
    (ASSETS / f"{name}.jpg").write_bytes(data)
    with Image.open(io.BytesIO(data)) as image:
        image.load()
        width, height = image.size
        components = len(image.getbands())
        pillow_crop = image.convert("RGB").crop(CROP).tobytes()

    pdf = pdf_image(data, width, height, components, transform)
    with pymupdf.open(stream=pdf, filetype="pdf") as document:
        assert document.page_count == 1 and not document.is_repaired
        assert document.xref_stream_raw(5) == data
        assert document.xref_get_key(5, "Filter") == ("name", "/DCTDecode")
        if transform is not None:
            assert document.xref_get_key(5, "DecodeParms/ColorTransform") == ("int", str(transform))
        mu_crop = document[0].get_pixmap(clip=pymupdf.Rect(CROP), colorspace=pymupdf.csRGB, alpha=False).samples
    assert_near(mu_crop, pillow_crop, 3, f"{name} Pillow/MuPDF")
    (ASSETS / f"{name}.pdf").write_bytes(pdf)
    filter_wire = json.dumps([{"kind": "dct", "colorTransform": transform}], separators=(",", ":")).encode()
    return {
        "name": name, "width": width, "height": height, "components": components,
        "colorTransform": transform, "byteLength": len(data),
        "reference": "pdf:s.stdio.jpeg:sha256:" + sha(b"s.stdio.jpeg" + data + filter_wire),
        "encodedSha256": sha(data), "pillowCropRgb": pillow_crop.hex(), "muPdfCropRgb": mu_crop.hex(),
    }


def invalid(name, data, failure, pillow_decodes=None):
    (ASSETS / f"{name}.jpg").write_bytes(data)
    try:
        with Image.open(io.BytesIO(data)) as image:
            image.load()
            decoded = True
    except OSError:
        decoded = False
    if pillow_decodes is not None:
        assert decoded == pillow_decodes, name
    return {"name": name, "expectedRefusal": failure, "pillowDecodes": decoded, "encodedSha256": sha(data), "byteLength": len(data)}


def generate():
    ASSETS.mkdir(parents=True, exist_ok=True)
    large = jpeg("RGB", (1024, 1024), 127)
    assert 1024 * 1024 <= len(large) <= 4 * 1024 * 1024
    gray = jpeg("L", (29, 24), 17, restart_marker_blocks=2)
    rgb = jpeg("RGB", (29, 24), 715, keep_rgb=True)
    progressive = jpeg("RGB", (29, 24), 192, progressive=True)
    first_dqt = gray.index(b"\xff\xdb")
    stuff = b"\xff\xef\xff\xff" + bytes(65533)
    fixtures = {
        "crop": CROP,
        "valid": [
            valid("large", large, 1),
            valid("gray-restart", gray, None),
            valid("adobe-rgb", rgb, 0),
            valid("unknown-metadata", gray[:2] + stuff * 20 + gray[2:], None),
        ],
        "invalid": [
            invalid("progressive", progressive, "baseline header marker", True),
            invalid("truncated-scan", gray[:3 * len(gray) // 4], "entropy-coded segment", False),
            invalid("trailing-bytes", gray + b"x", "terminal EOI", True),
            invalid("zero-quantizer", gray[:first_dqt + 5] + b"\0" + gray[first_dqt + 6:], "zero JPEG quantization"),
            invalid("bad-restart", gray.replace(b"\xff\xd0", b"\xff\xd1", 1), "unexpected JPEG restart marker"),
        ],
    }
    (ASSETS / "fixture.json").write_text(json.dumps(fixtures, indent=2) + "\n")
    print(f"[DEBUG] {len(fixtures['valid'])} valid controls, {len(fixtures['invalid'])} invalid controls, large file {len(large)} bytes")


def compare():
    fixture = json.loads((ASSETS / "fixture.json").read_text())
    for case in fixture["valid"]:
        actual = (ASSETS / f"{case['name']}.actual").read_text().splitlines()
        dimensions = [case["width"], case["height"], case["components"]]
        assert actual[0] == f"{dimensions[0]} {dimensions[1]} {dimensions[2]} {case['colorTransform']}", actual[0]
        assert actual[1] == case["reference"], actual[1]
        crop = bytes.fromhex(actual[2])
        delta = assert_near(crop, bytes.fromhex(case["pillowCropRgb"]), 3, f"{case['name']} Rust/Pillow")
        mu_delta = assert_near(crop, bytes.fromhex(case["muPdfCropRgb"]), 3, f"{case['name']} Rust/MuPDF")
        print(f"[DEBUG] {case['name']} exact DCT reference, crop maximum Pillow delta {delta}, MuPDF delta {mu_delta}")
    for case in fixture["invalid"]:
        result = (ASSETS / f"{case['name']}.actual").read_text()
        assert case["expectedRefusal"] in result, result
    print(f"[DEBUG] {len(fixture['invalid'])} invalid controls refused")


def verify_writer(graph=False):
    fixture = json.loads((ASSETS / "fixture.json").read_text())
    cases = fixture["valid"][:1] if graph else fixture["valid"]
    for case in cases:
        name = "native-graph" if graph else case["name"]
        resource = (ASSETS / f"{name}.stream-ref").read_text()
        assert resource == case["reference"], (name, resource)
        with pymupdf.open(ASSETS / f"{name}.stream.pdf") as document:
            assert document.page_count == 1 and not document.is_repaired, name
            assert document.metadata["producer"] == "Bounded PDF writer probe", name
            page = document[0]
            assert (int(page.rect.width), int(page.rect.height)) == (case["width"], case["height"]), name
            assert document.xref_get_key(5, "Filter") == ("name", "/DCTDecode"), name
            assert document.xref_get_key(5, "Length") == ("int", str(case["byteLength"])), name
            transform = case["colorTransform"]
            expected = ("null", "null") if transform is None else ("int", str(transform))
            assert document.xref_get_key(5, "DecodeParms/ColorTransform") == expected, name
            assert sha(document.xref_stream_raw(5)) == case["encodedSha256"], name
            pixels = page.get_pixmap(clip=pymupdf.Rect(fixture["crop"]), colorspace=pymupdf.csRGB, alpha=False).samples
            assert pixels.hex() == case["muPdfCropRgb"], name
        print(f"[DEBUG] {name} reopened without repair, exact original DCT bytes, crop and reference")


def verify_install_pack(kind="dct-install"):
    fixture = json.loads((ASSETS / "fixture.json").read_text())
    case = fixture["valid"][0]
    name = case["name"]
    installed = (ASSETS / f"{name}.{kind}.jpg").read_bytes()
    archive_bytes = (ASSETS / f"{name}.{kind}.pack").stat().st_size
    assert len(installed) == case["byteLength"] and sha(installed) == case["encodedSha256"]
    assert len(installed) < archive_bytes < len(installed) + 32768
    assert (ASSETS / f"{name}.{kind}.ref").read_text() == case["reference"]
    with Image.open(io.BytesIO(installed)) as image:
        assert image.size == (case["width"], case["height"])
        assert image.convert("RGB").crop(tuple(fixture["crop"])).tobytes().hex() == case["pillowCropRgb"]
    pdf = pdf_image(installed, case["width"], case["height"], case["components"], case["colorTransform"])
    with pymupdf.open(stream=pdf, filetype="pdf") as document:
        assert document.page_count == 1 and not document.is_repaired
        assert sha(document.xref_stream_raw(5)) == case["encodedSha256"]
        crop = document[0].get_pixmap(clip=pymupdf.Rect(fixture["crop"]), colorspace=pymupdf.csRGB, alpha=False).samples
        assert crop.hex() == case["muPdfCropRgb"]
    print(f"[DEBUG] {name} exact archived JPEG and DCT reference, {archive_bytes - len(installed)} Pack overhead bytes, exact Pillow and MuPDF crops")


if __name__ == "__main__":
    {"generate": generate, "compare": compare, "verify-writer": verify_writer, "verify-graph-writer": lambda: verify_writer(graph=True), "verify-install-pack": verify_install_pack, "verify-placement-pack": lambda: verify_install_pack("dct-placement")}[sys.argv[1]]()
