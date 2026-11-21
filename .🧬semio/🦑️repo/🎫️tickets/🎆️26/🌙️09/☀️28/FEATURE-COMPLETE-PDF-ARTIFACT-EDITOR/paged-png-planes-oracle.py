"""🔬 Independently generate PNG frames and reopen the sampled PDF color and alpha planes."""

import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import random
import struct
import subprocess
import sys
import zlib

sys.dont_write_bytecode = True

import pymupdf
from PIL import Image

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "paged-png"
EXE = OUT / ("png-probe.exe" if os.name == "nt" else "png-probe")
LAW = json.loads((TICKET / "paged-png-planes-cases.json").read_text(encoding="utf8"))
SIGNATURE = b"\x89PNG\r\n\x1a\n"


def chunk(kind, body):
    """🏺 Emit PNG bytes independently using the platform CRC32 algorithm."""
    return struct.pack(">I", len(body)) + kind + body + struct.pack(">I", zlib.crc32(kind + body))


def prediction(algorithm, a, b, c):
    """📐 Give the independent filter prediction used only to author fixture rows."""
    if algorithm < 4:
        return (0, a, b, (a + b) // 2)[algorithm]
    estimate = a + b - c
    return min((a, b, c), key=lambda value: abs(estimate - value))


def png(case):
    """🎨 Create samples encoded with each of the five real predictive filters."""
    width, height, mode = case["width"], case["height"], case["mode"]
    channels = len(mode)
    raw = random.Random(case["seed"]).randbytes(width * height * channels)
    previous = bytes(width * channels)
    filtered = bytearray()
    for y in range(height):
        current = raw[y * len(previous):(y + 1) * len(previous)]
        method = y % 5
        filtered.append(method)
        for x, value in enumerate(current):
            a, b, c = (current[x - channels] if x >= channels else 0), previous[x], (previous[x - channels] if x >= channels else 0)
            filtered.append((value - prediction(method, a, b, c)) & 255)
        previous = current
    color = {"L": 0, "RGB": 2, "LA": 4, "RGBA": 6}[mode]
    header = struct.pack(">IIBBBBB", width, height, 8, color, 0, 0, 0)
    compressed = zlib.compress(filtered, 7)
    parts = [compressed[:1], b"", compressed[1:3]]
    parts.extend(compressed[index:index + 7111] for index in range(3, len(compressed), 7111))
    return raw, header, parts


def encode(header, parts, end=b""):
    return SIGNATURE + chunk(b"IHDR", header) + b"".join(chunk(b"IDAT", part) for part in parts) + chunk(b"IEND", b"") + end


def run_data(name, encoded, *args):
    file = OUT / (name + ".png")
    file.write_bytes(encoded)
    result = OUT / (name + ".image")
    for ext in ("color", "alpha"):
        result.with_suffix("." + ext).unlink(missing_ok=True)
    run = subprocess.run([str(EXE), str(file), str(result), *map(str, args)], capture_output=True, text=True)
    return run, result


def verify_pdf(case, color, alpha):
    """🖼️ Reopen a PDF with the exact decoded raster and its separate soft mask."""
    width, height = case["width"], case["height"]
    spec = importlib.util.spec_from_file_location("png_pdf_reference", TICKET / "controlled-dct-oracle.py")
    shared = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(shared)
    image_head = f"/Type /XObject /Subtype /Image /Width {width} /Height {height} /BitsPerComponent 8".encode()
    space = b"/DeviceRGB" if case["mode"].startswith("RGB") else b"/DeviceGray"
    pdf = shared.document([
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        (f"<< /Type /Page /Parent 2 0 R /Resources << /XObject << /Color 5 0 R >> >> "
         f"/MediaBox [0 0 {width} {height}] /Contents 4 0 R >>").encode(),
        shared.stream(b"", f"q {width} 0 0 {height} 0 0 cm /Color Do Q".encode()),
        shared.stream(image_head + b" /ColorSpace " + space + (b" /SMask 6 0 R" if alpha else b""), color),
        shared.stream(image_head + b" /ColorSpace /DeviceGray", alpha),
    ])
    with pymupdf.open(stream=pdf, filetype="pdf") as doc:
        assert doc.page_count == 1 and not doc.is_repaired
        assert doc.xref_stream(5) == color
        if alpha:
            assert doc.xref_get_key(5, "SMask") == ("xref", "6 0 R")
            assert doc.xref_stream(6) == alpha
        plane = Image.frombytes("RGB" if space == b"/DeviceRGB" else "L", (width, height), color).convert("RGB")
        if alpha:
            plane = Image.composite(plane, Image.new("RGB", (width, height), "white"), Image.frombytes("L", (width, height), alpha))
        pixels = doc[0].get_pixmap(colorspace=pymupdf.csRGB, alpha=False).samples
        delta = max(abs(a - b) for a, b in zip(plane.tobytes(), pixels, strict=True))
        assert delta <= 2, delta
    return delta


def check():
    for case in LAW["valid"]:
        raw, head, parts = png(case)
        encoded = encode(head, parts)
        run, out = run_data(case["name"], encoded)
        assert run.returncode == 0, run.stderr
        with Image.open(io.BytesIO(encoded)) as decoded:
            assert decoded.tobytes() == raw
            color = decoded.convert("RGB" if case["mode"].startswith("RGB") else "L").tobytes()
            alpha = decoded.getchannel("A").tobytes() if "A" in case["mode"] else b""
        assert out.with_suffix(".color").read_bytes() == color
        assert out.with_suffix(".alpha").read_bytes() == alpha
        delta = verify_pdf(case, color, alpha)
        print(run.stdout.strip(), "output RGB/gray and alpha exact, PDF delta", delta)
    _, head, parts = png(LAW["valid"][1])
    data = encode(head, parts)
    broken_crc = bytearray(data)
    broken_crc[29] ^= 1
    wrong_adler = [*parts[:-1], parts[-1][:-1] + bytes([parts[-1][-1] ^ 1])]
    invalid = {
        "chunk-crc": bytes(broken_crc),
        "adler": encode(head, wrong_adler),
        "nonconsecutive-idat": SIGNATURE + chunk(b"IHDR", head) + chunk(b"IDAT", parts[0])
            + chunk(b"tEXt", b"author\x00test") + chunk(b"IDAT", b"".join(parts[1:])) + chunk(b"IEND", b""),
        "unexpected-alpha-key": SIGNATURE + chunk(b"IHDR", head) + chunk(b"tRNS", b"\0" * 6)
            + chunk(b"IDAT", b"".join(parts)) + chunk(b"IEND", b""),
        "trailing": encode(head, parts, b"y"),
        "duplicate-palette": SIGNATURE + chunk(b"IHDR", head) + chunk(b"PLTE", b"abc") + chunk(b"PLTE", b"def")
            + chunk(b"IDAT", b"".join(parts)) + chunk(b"IEND", b""),
    }
    for case in LAW["refusals"]:
        run, output = run_data(case["name"], invalid[case["name"]])
        assert run.returncode == 2 and case["kind"] in run.stderr and not output.with_suffix(".color").exists(), (case, run)
    _, head, parts = png(LAW["valid"][0])
    cancelled, output = run_data("cancelled", encode(head, parts), 1000)
    assert cancelled.returncode == 0 and "Cancelled" in cancelled.stdout and not output.with_suffix(".color").exists()
    print("[DEBUG] Invalid and cancelled PNG controls stayed private")
    case = LAW["valid"][0]
    _, header, parts = png(case)
    paid, delivered = run_data("paid-rgba", encode(header, parts), "paid")
    assert paid.returncode == 0, paid.stderr
    assert delivered.with_suffix(".color").read_bytes() == (OUT / "rgba-large.color").read_bytes()
    assert delivered.with_suffix(".alpha").read_bytes() == (OUT / "rgba-large.alpha").read_bytes()
    print(paid.stdout.strip())
    paid_cancel, delivered = run_data("paid-cancelled", encode(header, parts), "paid", 10_000)
    assert paid_cancel.returncode == 0 and "rendered false" in paid_cancel.stdout, paid_cancel.stderr
    assert not delivered.with_suffix(".color").exists()
    print(paid_cancel.stdout.strip())



if __name__ == "__main__":
    check()
