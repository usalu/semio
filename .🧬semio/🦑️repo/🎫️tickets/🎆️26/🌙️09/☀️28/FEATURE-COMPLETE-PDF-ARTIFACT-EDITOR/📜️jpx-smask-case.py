"""🧪️ Adds a JPEG 2000 image with in-data alpha (SMaskInData 1) to the paint-fidelity fixture."""

import io
import json
import sys

from PIL import Image

path = sys.argv[1]
data = json.load(open(path, encoding="utf-8"))
data["cases"] = [case for case in data["cases"] if case["name"] != "codec-jpx-smask-in-data"]
width, height, scale = 16, 12, 4
image = Image.new("RGBA", (width, height))
image.putdata([(x * 16, y * 20, 128, (x * 17 + y * 3) % 256) for y in range(height) for x in range(width)])
buffer = io.BytesIO()
image.save(buffer, "JPEG2000", irreversible=False)
probes = [[int((x + 0.5) * scale), int((y + 0.5) * scale)] for x, y in [(1, 1), (5, 2), (9, 4), (14, 6), (3, 9), (12, 10), (7, 7), (15, 11)]]
data["cases"].append({"kind": "image", "name": "codec-jpx-smask-in-data", "page": [width * scale, height * scale], "resources": "/XObject << /Im1 5 0 R >>",
                      "content": f"0.5 g 0 0 {width * scale} {height * scale} re f q {width * scale} 0 0 {height * scale} 0 0 cm /Im1 Do Q",
                      "objects": [{"dict": f"<< /Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /DeviceRGB /BitsPerComponent 8 /SMaskInData 1 /Filter [/ASCIIHexDecode /JPXDecode] >>", "stream": buffer.getvalue().hex() + ">"}],
                      "probes": [{"at": p, "rgb": None} for p in probes], "tolerance": 4})
compact = lambda value: json.dumps(value, separators=(",", ":"), ensure_ascii=False)
open(path, "w", encoding="utf-8").write('{"cases":[\n' + ",\n".join(compact(case) for case in data["cases"]) + "\n]}\n")
