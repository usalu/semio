"""🧪️ Adds odd-width 1, 2 and 4 bit image and mask cases (row padding, stride) to the paint-fidelity fixture."""

import json
import sys

path = sys.argv[1]
data = json.load(open(path, encoding="utf-8"))
data["cases"] = [case for case in data["cases"] if not case["name"].startswith("odd-width-")]
scale = 8


def image(dictionary, hexdata):
    return {"dict": f"<< /Type /XObject /Subtype /Image {dictionary} /Filter /ASCIIHexDecode >>", "stream": hexdata + ">"}


def case(name, width, height, objects, content_prefix=""):
    probes = [[int((x + 0.5) * scale), int((y + 0.5) * scale)] for y in range(height) for x in range(width)]
    return {"kind": "image", "name": name, "page": [width * scale, height * scale], "resources": "/XObject << /Im1 5 0 R >>",
            "content": f"{content_prefix}0.5 g 0 0 {width * scale} {height * scale} re f q {width * scale} 0 0 {height * scale} 0 0 cm /Im1 Do Q" if content_prefix == "" else f"{content_prefix}q {width * scale} 0 0 {height * scale} 0 0 cm /Im1 Do Q",
            "objects": objects, "probes": [{"at": p, "rgb": None} for p in probes], "tolerance": 3}


data["cases"].append(case("odd-width-4bit-gray", 5, 3, [image("/Width 5 /Height 3 /ColorSpace /DeviceGray /BitsPerComponent 4", "0123 4F 8A7B 5C 0 F00 F0 3A")]))
data["cases"][-1]["objects"] = [image("/Width 5 /Height 3 /ColorSpace /DeviceGray /BitsPerComponent 4", "0123F0 48A7B0 5C0F00")]
data["cases"].append(case("odd-width-4bit-with-2bit-smask", 5, 3, [
    image("/Width 5 /Height 3 /ColorSpace /DeviceGray /BitsPerComponent 4 /SMask 6 0 R", "0123F0 48A7B0 5C0F00"),
    image("/Type /XObject /Width 5 /Height 3 /ColorSpace /DeviceGray /BitsPerComponent 2", "1B00 E440 0AFC")]))
data["cases"].append(case("odd-width-1bit-stencil", 7, 3, [image("/Width 7 /Height 3 /ImageMask true /BitsPerComponent 1", "A6 5C 3E")], "1 0 0 rg "))
data["cases"].append(case("odd-width-1bit-gray", 9, 3, [image("/Width 9 /Height 3 /ColorSpace /DeviceGray /BitsPerComponent 1", "A580 5A00 FF80")]))
compact = lambda value: json.dumps(value, separators=(",", ":"), ensure_ascii=False)
open(path, "w", encoding="utf-8").write('{"cases":[\n' + ",\n".join(compact(c) for c in data["cases"]) + "\n]}\n")
