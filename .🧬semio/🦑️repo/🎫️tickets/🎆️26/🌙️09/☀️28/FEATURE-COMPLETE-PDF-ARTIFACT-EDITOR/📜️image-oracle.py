"""🌇 Validate the PDF editor image sample fixture with PyMuPDF 1.24.14."""

import json
from pathlib import Path
import sys

import fitz


def pdf_array(values):
    return "[" + " ".join(map(str, values)) + "]"


def set_reference_mask(document, image, case):
    mask = document.get_new_xref()
    stencil = case["kind"] == "stencil"
    color = "/ImageMask true" if stencil else "/ColorSpace /DeviceGray"
    document.update_object(
        mask,
        f"<< /Type /XObject /Subtype /Image /Width {case['width']} /Height {case['height']}"
        f" {color} /BitsPerComponent {case['bits']} /Decode {pdf_array(case['decode'])} >>",
    )
    if "matte" in case:
        document.xref_set_key(mask, "Matte", pdf_array(case["matte"]))
    if stencil:
        width = case["width"]
        assert width <= 8
        payload = bytes(
            sum(value << (7 - x) for x, value in enumerate(case["samples"][start:start + width]))
            for start in range(0, len(case["samples"]), width)
        )
    else:
        payload = bytes(case["samples"])
    document.update_stream(mask, payload)
    if case.get("overridesColorKey"):
        document.xref_set_key(image, "Mask", "[0 255 0 255 0 255]")
    document.xref_set_key(image, "Mask" if stencil else "SMask", f"{mask} 0 R")


def verify(path):
    data = json.loads(path.read_text(encoding="utf-8"))
    states = [(state, None) for state in ("before", "after", "masked")]
    states += [(case["kind"], case) for case in data["referencedMasks"]]
    for state, case in states:
        with fitz.open() as document:
            page = document.new_page(width=data["page"][0], height=data["page"][1])
            image = document.get_new_xref()
            document.update_object(
                image,
                f"<< /Type /XObject /Subtype /Image /Width {data['width']} /Height {data['height']}"
                " /ColorSpace /DeviceRGB /BitsPerComponent 8 >>",
            )
            if state == "masked":
                for key, name in (("Mask", "mask"), ("Decode", "decode")):
                    document.xref_set_key(image, key, pdf_array(data[name]))
            if case is not None:
                set_reference_mask(document, image, case)
                source = case.get("source", data["before"])
            else:
                source = data["before" if state == "masked" else state]
            document.update_stream(image, bytes(source))
            resources = f"<< /XObject << /Im1 {image} 0 R >> >>"
            document.xref_set_key(page.xref, "Resources", resources)
            stream = document.get_new_xref()
            document.update_object(stream, "<<>>")
            matrix = " ".join(str(value) for value in data["matrix"])
            document.update_stream(stream, f"q {matrix} cm /Im1 Do Q".encode("ascii"))
            page.set_contents(stream)
            bounds, = page.get_image_rects(image)
            assert [bounds.x0, bounds.y0, bounds.width, bounds.height] == data["bounds"]
            alpha = state not in ("before", "after")
            pixmap = page.get_pixmap(colorspace=fitz.csRGB, alpha=alpha)
            channels = 4 if alpha else 3
            samples = case["expectedRgba"] if case is not None else data["maskedRgba" if alpha else state]
            for index, probe in enumerate(data["probes"]):
                expected = samples[index * channels:(index + 1) * channels]
                if alpha:
                    expected[:3] = [(component * expected[3] + 127) // 255 for component in expected[:3]]
                actual = list(pixmap.pixel(*probe))
                tolerance = int(case is not None and state == "soft" and 0 < expected[3] < 255)
                assert all(abs(a - b) <= tolerance for a, b in zip(actual, expected)), (state, probe, expected, actual)
    print("Image bounds and four pixel probes match across all six image and mask cases")


if __name__ == "__main__":
    verify(Path(sys.argv[1]))
