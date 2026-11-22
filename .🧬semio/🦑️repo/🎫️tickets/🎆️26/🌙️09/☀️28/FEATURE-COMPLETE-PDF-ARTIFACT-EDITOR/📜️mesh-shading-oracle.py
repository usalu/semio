"""🧪️ Builds mesh shading streams (types 4 to 7), renders them with MuPDF and writes probe colors."""

import sys

import fitz


class Bits:
    def __init__(self):
        self.value, self.count = 0, 0

    def put(self, value, bits):
        self.value = self.value << bits | int(value)
        self.count += bits

    def align(self):
        pad = -self.count % 8
        self.value <<= pad
        self.count += pad

    def bytes(self):
        self.align()
        return self.value.to_bytes(self.count // 8, "big")


def q(value, bits, low=0.0, high=100.0):
    return round((value - low) / (high - low) * ((1 << bits) - 1))


def rgb(bits, color):
    return [round(c * ((1 << bits) - 1)) for c in color]


def vertex(b, flag, x, y, color, fb, cb, pb):
    if fb:
        b.put(flag, fb)
    b.put(q(x, cb), cb)
    b.put(q(y, cb), cb)
    for c in rgb(pb, color):
        b.put(c, pb)
    b.align()


RED, GREEN, BLUE, YELLOW = (1, 0, 0), (0, 1, 0), (0, 0, 1), (1, 1, 0)
CASES = []


def add(name, shading_type, cb, pb, fb, vpr, data):
    CASES.append(dict(name=name, type=shading_type, cb=cb, pb=pb, fb=fb, vpr=vpr, data=data))


b = Bits()
for flag, x, y, color in [(0, 10, 10, RED), (0, 90, 10, GREEN), (0, 50, 60, BLUE), (1, 90, 90, YELLOW), (2, 10, 90, RED)]:
    vertex(b, flag, x, y, color, 8, 16, 8)
add("free-form", 4, 16, 8, 8, 0, b.bytes())

b = Bits()
for row, y in enumerate([10, 50, 90]):
    for col, x in enumerate([10, 50, 90]):
        vertex(b, 0, x, y, [RED, GREEN, BLUE, YELLOW][(row + col) % 4], 0, 16, 8)
add("lattice", 5, 16, 8, 0, 3, b.bytes())


def patch(b, flag, points, colors, tensor, cb=12, pb=8, fb=8):
    b.put(flag, fb)
    for x, y in points:
        b.put(q(x, cb), cb)
        b.put(q(y, cb), cb)
    for color in colors:
        for c in rgb(pb, color):
            b.put(c, pb)
    b.align()


boundary = [(10, 10), (5, 35), (15, 65), (10, 90), (35, 95), (65, 85), (90, 90), (95, 65), (85, 35), (90, 10), (65, 5), (35, 15)]
b = Bits()
patch(b, 0, boundary, [RED, GREEN, BLUE, YELLOW], False)
add("coons", 6, 12, 8, 8, 0, b.bytes())

b = Bits()
patch(b, 0, boundary + [(35, 35), (35, 65), (65, 65), (65, 35)], [RED, GREEN, BLUE, YELLOW], True)
add("tensor", 7, 12, 8, 8, 0, b.bytes())

# a first Coons patch on the left half, then a flag 2 patch sharing its edge
left = [(5, 10), (5, 35), (5, 65), (5, 90), (15, 92), (30, 88), (45, 90), (45, 65), (45, 35), (45, 10), (30, 8), (15, 12)]
b = Bits()
patch(b, 0, left, [RED, GREEN, BLUE, YELLOW], False)
# flag 2: new edge = previous p33 p32 p31 p30 = (45,90) (30,88) (15,92) (5,90): choose continuing patch below
rest = [(45, 90)]
shared = [(45, 90), (45, 65), (45, 35), (45, 10)]
patch_b = Bits()
b.put(2, 8)
for x, y in [(80, 15), (95, 15), (95, 85), (80, 90), (70, 95), (60, 92), (50, 88)]:
    pass
add("coons-flag-chain", 6, 12, 8, 8, 0, b.bytes())
CASES.pop()

DECODE = [0, 100, 0, 100, 0, 1, 0, 1, 0, 1]


def build(case):
    stream = case["data"]
    dictionary = f"<< /ShadingType {case['type']} /ColorSpace /DeviceRGB /BitsPerCoordinate {case['cb']} /BitsPerComponent {case['pb']} /Decode [0 100 0 100 0 1 0 1 0 1]"
    if case["fb"]:
        dictionary += f" /BitsPerFlag {case['fb']}"
    if case["vpr"]:
        dictionary += f" /VerticesPerRow {case['vpr']}"
    dictionary += f" /Filter /ASCIIHexDecode /Length {len(stream) * 2 + 1} >>"
    hexed = stream.hex() + ">"
    objects = [
        "<< /Type /Catalog /Pages 2 0 R >>",
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 100 100] /Contents 4 0 R /Resources << /Shading << /Sh1 5 0 R >> >> >>",
        "<< /Length 7 >>\nstream\n/Sh1 sh\nendstream",
        dictionary + "\nstream\n" + hexed + "\nendstream",
    ]
    pdf = "%PDF-1.7\n"
    xref = ""
    for index, body in enumerate(objects):
        xref += f"{len(pdf):010} 00000 n \n"
        pdf += f"{index + 1} 0 obj\n{body}\nendobj\n"
    pdf += f"xref\n0 {len(objects) + 1}\n0000000000 65535 f \n{xref}trailer\n<< /Root 1 0 R /Size {len(objects) + 1} >>\nstartxref\n{len(pdf)}\n%%EOF\n"
    return pdf.encode("ascii")


def main(cases_path, expected_path):
    cases, expected = [], []
    for case in CASES:
        with fitz.open(stream=build(case), filetype="pdf") as document:
            pixmap = document[0].get_pixmap(colorspace=fitz.csRGB)
            for y in range(5, 100, 10):
                for x in range(5, 100, 10):
                    expected.append("\t".join(map(str, [case["name"], x, y, *pixmap.pixel(x, y)])))
        cases.append("\t".join(map(str, [case["name"], case["type"], case["cb"], case["pb"], case["fb"], case["vpr"], 100, 100, 3, ",".join(map(str, DECODE)), case["data"].hex()])))
    open(cases_path, "w").write("\n".join(cases))
    open(expected_path, "w").write("\n".join(expected))
    print(len(cases), "mesh cases")


main(sys.argv[1], sys.argv[2])
