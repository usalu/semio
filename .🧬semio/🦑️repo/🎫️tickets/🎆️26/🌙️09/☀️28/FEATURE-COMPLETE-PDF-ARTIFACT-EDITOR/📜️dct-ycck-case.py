"""🧪️ Adds an Adobe YCCK JPEG (APP14 transform 2, made from a CMYK JPEG by patching the marker) to the paint-fidelity fixture."""

import io
import json
import sys

from PIL import Image

path = sys.argv[1]
data = json.load(open(path, encoding="utf-8"))
data["cases"] = [case for case in data["cases"] if not case["name"].startswith("codec-dct-ycck")]
width, height, scale = 16, 12, 4
planes = Image.new("CMYK", (width, height))
planes.putdata([(100 + x * 6, 128 + (y - 6) * 5, 128 + (x - 8) * 6, (x * 5 + y * 7) % 90) for y in range(height) for x in range(width)])
buffer = io.BytesIO()
planes.save(buffer, "JPEG", quality=100, subsampling=0)
jpeg = bytearray(buffer.getvalue())
at = jpeg.find(b"Adobe")
assert at > 0, "no Adobe marker"
jpeg[at + 11] = 2
probes = [[int((x + 0.5) * scale), int((y + 0.5) * scale)] for x, y in [(1, 1), (5, 2), (9, 4), (14, 6), (3, 9), (12, 10), (7, 7), (15, 11)]]
for name, decode in (("codec-dct-ycck", ""), ("codec-dct-ycck-inverted", "/Decode [1 0 1 0 1 0 1 0]")):
    data["cases"].append({"kind": "image", "name": name, "page": [width * scale, height * scale], "resources": "/XObject << /Im1 5 0 R >>",
                          "content": f"q {width * scale} 0 0 {height * scale} 0 0 cm /Im1 Do Q",
                          "objects": [{"dict": f"<< /Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /DeviceCMYK /BitsPerComponent 8 {decode} /Filter [/ASCIIHexDecode /DCTDecode] >>", "stream": bytes(jpeg).hex() + ">"}],
                          "probes": [{"at": p, "rgb": None} for p in probes], "tolerance": 6})
compact = lambda value: json.dumps(value, separators=(",", ":"), ensure_ascii=False)
open(path, "w", encoding="utf-8").write('{"cases":[\n' + ",\n".join(compact(case) for case in data["cases"]) + "\n]}\n")
