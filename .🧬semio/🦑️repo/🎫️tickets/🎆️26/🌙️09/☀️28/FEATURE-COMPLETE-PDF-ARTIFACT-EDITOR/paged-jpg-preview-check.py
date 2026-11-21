"""🧪 Verify exact JPEG sample centres with Pillow and independently rendered PDF pixels."""

import json
from pathlib import Path
import os
import subprocess
import sys

sys.dont_write_bytecode = True

import pymupdf
from PIL import Image

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
ASSETS = GENERATED / "paged-jpeg-assets"
STAGE = GENERATED / "paged-jpeg-stage"
CASES = json.loads((TICKET / "paged-jpg-preview-cases.json").read_text(encoding="utf8"))


def samples(image, source, display):
    """📍 Extract each output cell's centre sample without applying image filtering."""
    width, height = source
    cols, rows = display
    assert image.size == (width, height)
    pixels = image.load()
    expected = bytearray()
    for row in range(rows):
        y = (2 * row + 1) * height // (2 * rows)
        for col in range(cols):
            x = (2 * col + 1) * width // (2 * cols)
            expected.extend(pixels[x, y])
    return expected


def generate_subsampled():
    """🎨 Generate odd-size baseline 4:2:0 color on smooth independent chroma ramps."""
    import importlib.util
    from PIL.JpegImagePlugin import get_sampling

    spec = importlib.util.spec_from_file_location("fixture_pdf", TICKET / "paged-jpeg-oracle.py")
    reference = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(reference)
    local = STAGE / "4-2-0"
    local.mkdir(exist_ok=True)
    width, height = 113, 79
    rgb = bytes(value for y in range(height) for x in range(width)
                for value in (20 + x, 40 + y * 2, 60 + (x + y) // 2))
    image = Image.frombytes("RGB", (width, height), rgb)
    jpg = local / "subsampled.jpg"
    image.save(jpg, format="JPEG", quality=93, subsampling=2)
    with Image.open(jpg) as decoded:
        assert get_sampling(decoded) == 2
    (local / "subsampled.pdf").write_bytes(reference.pdf_image(jpg.read_bytes(), width, height, 3, 1))
    return local


def run():
    """📏 Exercise genuine JPEG bodies and compare the staged decoder against two libraries."""
    subprocess.run([sys.executable, str(TICKET / "paged-jpeg-stage.py")], check=True)
    for name in ("paged-jpg-preview.rs", "paged-jpg-preview-probe.rs"):
        (STAGE / name.removeprefix("paged-")).write_bytes((TICKET / name).read_bytes())
    exe = STAGE / ("preview.exe" if os.name == "nt" else "preview")
    result = STAGE / "preview.rgb"
    subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                    "--edition=2021", "--crate-name=jpg_preview_probe", "-O", str(STAGE / "jpg-preview-probe.rs"), "-o", str(exe)], check=True)

    subsampled = generate_subsampled()

    def library(case):
        return subsampled if case.get('subsampled') else ASSETS

    def invoke(case, *extra):
        result.unlink(missing_ok=True)
        jpg = library(case) / (case["name"] + ".jpg")
        args = [*case["source"], *case["display"], case["channels"], *extra]
        return subprocess.run([str(exe), str(jpg), str(result), *map(str, args)], capture_output=True, text=True)

    for case in CASES["valid"]:
        outcome = invoke(case)
        assert outcome.returncode == 0, outcome.stderr
        info = outcome.stdout.strip()
        transform = "None" if case["transform"] is None else f"Some({case['transform']})"
        assert info.endswith("transform " + transform)
        assert result.is_file()
        actual = result.read_bytes()
        name = case["name"]
        with Image.open(library(case) / (name + ".jpg")) as original:
            expected = samples(original.convert("RGB"), case["source"], case["display"])
        with pymupdf.open(library(case) / (name + ".pdf")) as doc:
            rendered = doc[0].get_pixmap(colorspace=pymupdf.csRGB, alpha=False)
            expected_pdf = samples(Image.frombytes("RGB", (rendered.width, rendered.height), rendered.samples), case["source"], case["display"])
        delta = [max(abs(a - b) for a, b in zip(actual, other, strict=True)) for other in (expected, expected_pdf)]
        assert max(delta) <= 4, (name, delta)
        print(info, f"({name}, Pillow/MuPDF max channel error {delta})")
    for case in CASES["refusal"]:
        outcome = invoke(case)
        assert outcome.returncode == 2 and case["reason"] in outcome.stderr and not result.exists(), (case, outcome)
    cancelled = invoke(CASES["valid"][0], 1000)
    assert cancelled.returncode == 0 and "1000 unpublished" in cancelled.stdout and not result.exists()
    print("[DEBUG] All invalid previews stayed unpublished, and cancellation closed the private preview")


if __name__ == "__main__":
    run()
