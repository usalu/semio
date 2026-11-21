import json
import math
import struct
from pathlib import Path


def f32(value):
    return struct.unpack("<f", struct.pack("<f", value))[0]


law = json.loads(Path(__file__).with_name("gis-terrain-scalars.json").read_text())
rows = law["rows"]
size = len(rows)
heights = [sample for row in reversed(rows) for sample in row]
minimum, maximum = min(heights), max(heights)
span = (maximum - minimum) or 1
left, bottom, right, top = law["bounds"]
dx, dy = law["cellMeters"]
scale = law["exaggeration"]


def node(row, col):
    at = row * size + col
    longitude = left + (right - left) * (col / (size - 1))
    latitude = top + (bottom - top) * (row / (size - 1))
    west, east = max(col - 1, 0), min(col + 1, size - 1)
    north, south = max(row - 1, 0), min(row + 1, size - 1)
    nx = -(heights[row * size + east] - heights[row * size + west]) * scale / ((east - west) * dx)
    ny = -(heights[north * size + col] - heights[south * size + col]) * scale / ((south - north) * dy)
    length = math.sqrt(nx * nx + ny * ny + 1)
    return {
        "geographic": [longitude, latitude],
        "elevation": f32(heights[at] * scale),
        "normal": [f32(nx / length), f32(ny / length), f32(1 / length)],
        "uv": [0.5, f32((heights[at] - minimum) / span)],
    }


def cell(row, col):
    a = row * size + col
    return [a, a + size, a + 1, a + 1, a + size, a + size + 1]


actual = {
    "vertices": [node(*coords) for coords in law["vertexAddresses"]],
    "triangles": [cell(*coords) for coords in law["cellAddresses"]],
}
assert actual == law["expected"], actual
print(json.dumps(actual))
