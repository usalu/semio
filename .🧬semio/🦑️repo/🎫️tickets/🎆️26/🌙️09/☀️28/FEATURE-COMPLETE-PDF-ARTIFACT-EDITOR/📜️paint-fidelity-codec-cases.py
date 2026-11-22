"""🧪️ Appends mesh shading, PostScript function and CCITT, JBIG2, JPEG 2000 image cases to the neutral paint-fidelity fixture."""

import json
import sys

FIXTURE, MESH, CCITT, JBIG2, JPX = sys.argv[1:6]
data = json.load(open(FIXTURE, encoding="utf-8"))
data["cases"] = [case for case in data["cases"] if not case["name"].startswith(("mesh-", "postscript-", "codec-"))]
compact = lambda value: json.dumps(value, separators=(",", ":"), ensure_ascii=False)


def rows(path):
    return [line.split("\t") for line in open(path, encoding="utf-8").read().splitlines() if line]


def image_case(name, width, height, filters, dictionary, hexdata, tolerance, scale):
    probes = [[int((x + 0.5) * scale), int((y + 0.5) * scale)] for x, y in [(1, 1), (width // 2, height // 3), (width - 2, 2), (3, height - 3), (width // 3, height // 2), (width - 3, height - 2), (width // 2, height - 2), (5, height // 2)]]
    return {"kind": "image", "name": name, "page": [width * scale, height * scale], "resources": "/XObject << /Im1 5 0 R >>", "content": f"q {width * scale} 0 0 {height * scale} 0 0 cm /Im1 Do Q",
            "objects": [{"dict": f"<< /Type /XObject /Subtype /Image /Width {width} /Height {height} {dictionary} /Filter [/ASCIIHexDecode {filters}] >>", "stream": hexdata + ">"}], "probes": [{"at": p, "rgb": None} for p in probes], "tolerance": tolerance}


for line in rows(MESH):
    name, kind, cb, pb, fb, vpr, _, _, comps, decode, hexdata = line
    extra = (f" /BitsPerFlag {fb}" if fb != "0" else "") + (f" /VerticesPerRow {vpr}" if vpr != "0" else "")
    probes = [[x, y] for x in (12, 33, 52, 77, 93) for y in (13, 37, 58, 83)]
    data["cases"].append({"kind": "shading", "name": f"mesh-type-{kind}", "page": [100, 100], "resources": "/Shading << /Sh1 5 0 R >>", "content": "/Sh1 sh",
                          "objects": [{"dict": f"<< /ShadingType {kind} /ColorSpace /DeviceRGB /BitsPerCoordinate {cb} /BitsPerComponent {pb}{extra} /Decode [{decode.replace(',', ' ')}] /Filter /ASCIIHexDecode >>", "stream": hexdata + ">"}],
                          "probes": [{"at": p, "rgb": None} for p in probes], "tolerance": 12})
data["cases"].append({"kind": "shading", "name": "postscript-function", "page": [100, 60], "resources": "/Shading << /Sh1 5 0 R >>", "content": "/Sh1 sh",
                      "objects": [{"dict": "<< /ShadingType 2 /ColorSpace /DeviceRGB /BBox [0 0 100 60] /Coords [0 30 100 30] /Extend [false false] /Function 6 0 R >>"},
                                  {"dict": "<< /FunctionType 4 /Domain [0 1] /Range [0 1 0 1 0 1] >>", "stream": "{ dup 2 mul 1 exch sub exch 0.5 mul 0.25 }"}],
                      "probes": [{"at": [x, 30], "rgb": None} for x in (5, 20, 40, 60, 80, 95)], "tolerance": 6})
for line in rows(CCITT):
    if line[0] == "small-group4":
        width, height = int(line[2]), int(line[3])
        data["cases"].append(image_case("codec-ccitt-g4", width, height, "/CCITTFaxDecode", f"/ColorSpace /DeviceGray /BitsPerComponent 1 /DecodeParms [null << /K -1 /Columns {width} /Rows {height} /BlackIs1 true >>]", line[4], 2, 4))
for line in rows(JBIG2):
    if line[0] == "symbols-and-text":
        data["cases"].append(image_case("codec-jbig2-text", int(line[1]), int(line[2]), "/JBIG2Decode", "/ColorSpace /DeviceGray /BitsPerComponent 1", line[3], 2, 4))
for line in rows(JPX):
    if line[0] == "rgb-lossless-mct":
        data["cases"].append(image_case("codec-jpx-rgb", int(line[1]), int(line[2]), "/JPXDecode", "", line[5], 4, 2))
open(FIXTURE, "w", encoding="utf-8").write('{"cases":[\n' + ",\n".join(compact(case) for case in data["cases"]) + "\n]}\n")
print(len(data["cases"]), "cases")
