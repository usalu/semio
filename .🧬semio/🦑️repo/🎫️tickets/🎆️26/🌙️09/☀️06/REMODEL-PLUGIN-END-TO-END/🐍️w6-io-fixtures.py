#!/usr/bin/env python3
"""🧫️ W6 generator — writes remodel's language-agnostic io round-trip fixtures.

Every byte below is produced by Python's standard library alone (`struct`, `zlib`) or hand-authored
text — never by this repo's own encoders — so a Rust test that reads one of these files and recovers
the documented geometry is a genuine cross-implementation check, not a self-round-trip.
"""

import struct
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
FIXTURES = ROOT / "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧫️fixtures"

# ☁️ 4 coloured points, no faces — the smallest honest sparse-cloud input.
PLY = """ply
format ascii 1.0
comment remodel io fixture: four coloured points, no faces
element vertex 4
property float x
property float y
property float z
property uchar red
property uchar green
property uchar blue
end_header
0 0 0 255 0 0
1 0 0 0 255 0
0 1 0 0 0 255
0 0 1 255 255 255
"""

# 🧊️ Unit cube, 8 shared vertices, 12 triangles.
OBJ = """# remodel io fixture: unit cube, 8 vertices, 12 triangles
v 0 0 0
v 1 0 0
v 1 1 0
v 0 1 0
v 0 0 1
v 1 0 1
v 1 1 1
v 0 1 1
f 1 3 2
f 1 4 3
f 5 6 7
f 5 7 8
f 1 2 6
f 1 6 5
f 2 3 7
f 2 7 6
f 3 4 8
f 3 8 7
f 4 1 5
f 4 5 8
"""

# 🔺️ Unit tetrahedron, 4 facets, explicit face normals.
STL = """solid remodel-io-fixture
  facet normal 0 0 -1
    outer loop
      vertex 0 0 0
      vertex 0 1 0
      vertex 1 0 0
    endloop
  endfacet
  facet normal 0 -1 0
    outer loop
      vertex 0 0 0
      vertex 1 0 0
      vertex 0 0 1
    endloop
  endfacet
  facet normal -1 0 0
    outer loop
      vertex 0 0 0
      vertex 0 0 1
      vertex 0 1 0
    endloop
  endfacet
  facet normal 1 1 1
    outer loop
      vertex 1 0 0
      vertex 0 1 0
      vertex 0 0 1
    endloop
  endfacet
endsolid remodel-io-fixture
"""

# 🖼️ 2x2 RGBA8: red, green / blue, white.
PIXELS = [
    (255, 0, 0, 255),
    (0, 255, 0, 255),
    (0, 0, 255, 255),
    (255, 255, 255, 255),
]


def png_chunk(kind: bytes, payload: bytes) -> bytes:
    return struct.pack(">I", len(payload)) + kind + payload + struct.pack(">I", zlib.crc32(kind + payload) & 0xFFFFFFFF)


def two_by_two_png() -> bytes:
    raw = b""
    for row in range(2):
        raw += b"\x00"
        for column in range(2):
            raw += bytes(PIXELS[row * 2 + column])
    header = struct.pack(">IIBBBBB", 2, 2, 8, 6, 0, 0, 0)
    return b"\x89PNG\r\n\x1a\n" + png_chunk(b"IHDR", header) + png_chunk(b"IDAT", zlib.compress(raw, 9)) + png_chunk(b"IEND", b"")


def main() -> None:
    FIXTURES.mkdir(parents=True, exist_ok=True)
    written = {
        "🧱️four-points.ply": PLY.encode("utf-8"),
        "🗿️unit-cube.obj": OBJ.encode("utf-8"),
        "🔺️unit-tetra.stl": STL.encode("utf-8"),
        "📷️two-by-two.png": two_by_two_png(),
    }
    for name, payload in written.items():
        (FIXTURES / name).write_bytes(payload)
        print(f"wrote {name} ({len(payload)} bytes)")


if __name__ == "__main__":
    main()
