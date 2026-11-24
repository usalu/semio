#!/usr/bin/env python3
"""Writes the multi-resource PDF fixtures and their schema-first spec; deterministic, standard library only.

usage: multi-resource-fixture.py <fixture directory>
"""
import json
import sys
from pathlib import Path


def pdf(objects):
    out = bytearray(b"%PDF-1.7\n")
    offsets = []
    for number, body in enumerate(objects, 1):
        offsets.append(len(out))
        out += f"{number} 0 obj\n".encode() + body + b"\nendobj\n"
    start = len(out)
    out += f"xref\n0 {len(objects) + 1}\n0000000000 65535 f \n".encode()
    for offset in offsets:
        out += f"{offset:010} 00000 n \n".encode()
    out += f"trailer\n<< /Size {len(objects) + 1} /Root 1 0 R >>\nstartxref\n{start}\n%%EOF\n".encode()
    return bytes(out)


def stream(content, fields=""):
    body = content if isinstance(content, bytes) else content.encode("latin-1")
    return f"<< /Length {len(body)}{fields} >>\nstream\n".encode() + body + b"\nendstream"


def image(width, height, space, samples):
    fields = f" /Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /{space} /BitsPerComponent 8"
    return stream(bytes(samples), fields)


def generated():
    fonts = {
        "F1": b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>",
        "F2": b"<< /Type /Font /Subtype /Type1 /BaseFont /Times-Roman /Encoding /MacRomanEncoding >>",
        "F3": b"<< /Type /Font /Subtype /Type1 /BaseFont /Courier >>",
        "F4": b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold /Encoding << /Type /Encoding /BaseEncoding /WinAnsiEncoding /Differences [233 /eacute 252 /udieresis] >> >>",
    }
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R 4 0 R 5 0 R] /Count 3 /Resources << /Font << /F1 9 0 R /F2 10 0 R /F3 11 0 R >> /XObject << /Im1 12 0 R /Im2 13 0 R >> /ExtGState << /GS1 16 0 R >> /Properties << /MC0 17 0 R >> >> >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 180] /Contents 6 0 R >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 180] /Contents 7 0 R >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 200 180] /Resources 14 0 R /Contents 8 0 R >>",
        stream("/GS1 gs BT /F1 12 Tf 20 160 Td (Alpha one) Tj /F2 12 Tf 0 -16 Td (Beta two) Tj /F3 10 Tf 0 -16 Td (Gamma three) Tj ET\nq 40 0 0 40 20 20 cm /Im1 Do Q q 40 0 0 40 80 20 cm /Im2 Do Q\n"),
        stream("BT /F3 10 Tf 20 160 Td (Delta four) Tj /F2 12 Tf 0 -16 Td (Epsilon five) Tj ET\nq 40 0 0 40 20 20 cm /Im2 Do Q\n"),
        stream("BT /F4 12 Tf 20 160 Td (Caf\\351 \\374ber) Tj /F1 12 Tf 0 -16 Td (Zeta six) Tj ET\nq 40 0 0 40 20 20 cm /Im2 Do Q\n"),
        fonts["F1"], fonts["F2"], fonts["F3"],
        image(2, 2, "DeviceGray", [0, 85, 170, 255]),
        image(2, 2, "DeviceRGB", [255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0]),
        b"<< /Font << /F1 9 0 R /F4 15 0 R >> /XObject << /Im2 13 0 R >> /ProcSet [/PDF /Text /ImageB /ImageC] >>",
        fonts["F4"],
        b"<< /Type /ExtGState /LW 2 /LC 1 /BM /Multiply /CA 0.5 /ca 0.25 /AIS false >>",
        b"<< /MCID 0 /Lang (en) >>",
    ]
    spec = {
        "pages": [
            {"fonts": ["F1", "F2", "F3"], "images": ["Im1", "Im2"], "usedFonts": ["F1", "F2", "F3"], "paintedImages": ["Im1", "Im2"], "text": ["Alpha one", "Beta two", "Gamma three"], "independentText": True},
            {"fonts": ["F1", "F2", "F3"], "images": ["Im1", "Im2"], "usedFonts": ["F3", "F2"], "paintedImages": ["Im2"], "text": ["Delta four", "Epsilon five"], "independentText": True},
            {"fonts": ["F1", "F4"], "images": ["Im2"], "usedFonts": ["F4", "F1"], "paintedImages": ["Im2"], "text": ["Café über", "Zeta six"], "independentText": False},
        ],
        "fontCount": 4, "imageCount": 2, "extGStateCount": 1, "propertiesCount": 1,
    }
    return pdf(objects), spec


def report():
    fonts = [
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Times-Roman /Encoding /WinAnsiEncoding >>",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Times-Italic /Encoding /WinAnsiEncoding >>",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Courier-Bold /Encoding /WinAnsiEncoding >>",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Oblique /Encoding /WinAnsiEncoding >>",
    ]
    pages = 12
    first_page = 3
    first_content = first_page + pages
    first_font = first_content + pages
    first_image = first_font + len(fonts)
    first_resources = first_image + 2
    groups = [
        ({"F1": 0, "F2": 1, "F3": 2}, {}),
        ({"F1": 0, "F4": 3, "F5": 4}, {"Im1": 0}),
        ({"F2": 1, "F5": 4}, {"Im1": 0, "Im2": 1}),
    ]
    resources = []
    for group_fonts, group_images in groups:
        entries = "/Font << " + " ".join(f"/{name} {first_font + index} 0 R" for name, index in group_fonts.items()) + " >>"
        if group_images:
            entries += " /XObject << " + " ".join(f"/{name} {first_image + index} 0 R" for name, index in group_images.items()) + " >>"
        resources.append(f"<< {entries} /ProcSet [ /PDF /Text /ImageB /ImageC ] >>".encode())
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        f"<< /Type /Pages /Kids [{' '.join(f'{first_page + i} 0 R' for i in range(pages))}] /Count {pages} >>".encode(),
    ]
    contents, specs = [], []
    for page in range(pages):
        group = page // 4
        group_fonts, group_images = groups[group]
        names = list(group_fonts)
        lines = []
        text = []
        body = ["BT"]
        for line in range(4):
            font = names[(page + line) % len(names)]
            sentence = f"Report page {page + 1} line {line + 1} in {font}"
            text.append(sentence)
            body.append(f"/{font} 11 Tf 1 0 0 1 56.7 {760 - 18 * line} Tm ({sentence}) Tj")
            if line == 1:
                body.append(f"/{font} 11 Tf 1 0 0 1 56.7 700 Tm [(Kern)-120(ed)40(text)] TJ")
        body.append("ET")
        painted = list(group_images)
        for index, name in enumerate(painted):
            body.append(f"q 60 0 0 60 {56 + 80 * index} 500 cm /{name} Do Q")
        contents.append(stream("\n".join(body) + "\n"))
        specs.append({"fonts": names, "images": painted, "usedFonts": sorted(set(names[(page + line) % len(names)] for line in range(4))), "paintedImages": painted, "text": text, "independentText": True})
    for page in range(pages):
        objects.append(f"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595.276 841.89] /Resources {first_resources + page // 4} 0 R /Contents {first_content + page} 0 R >>".encode())
    objects.extend(contents)
    objects.extend(fonts)
    objects.append(image(2, 2, "DeviceGray", [10, 90, 170, 250]))
    objects.append(image(3, 1, "DeviceRGB", [255, 0, 0, 0, 255, 0, 0, 0, 255]))
    objects.extend(resources)
    return pdf(objects), {"pages": specs, "fontCount": len(fonts), "imageCount": 2, "extGStateCount": 0, "propertiesCount": 0}


def main():
    directory = Path(sys.argv[1])
    directory.mkdir(parents=True, exist_ok=True)
    spec = {}
    for name, build in (("generated", generated), ("report", report)):
        data, expected = build()
        (directory / f"{name}.pdf").write_bytes(data)
        spec[name] = expected
    (directory / "spec.json").write_text(json.dumps(spec, indent=2, ensure_ascii=False) + "\n")


main()
