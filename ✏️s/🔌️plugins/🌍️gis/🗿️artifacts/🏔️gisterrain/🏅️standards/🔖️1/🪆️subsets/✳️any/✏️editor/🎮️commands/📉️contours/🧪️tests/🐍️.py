# /// script
# requires-python = ">=3.11"
# dependencies = ["matplotlib", "shapely"]
# ///
"""🐍 Verify WGS84 route geometry against Matplotlib's triangulated isolines."""

import json
import math
from pathlib import Path

import matplotlib
import matplotlib.tri as tri
from shapely.geometry import LineString

matplotlib.use("Agg")
import matplotlib.pyplot as plt

fixture = json.loads(Path(__file__).with_name("🔣️.json").read_text())
west, south, east, north = fixture["bounds"]
width = 33
coordinates = [
    (west + (east - west) * column / (width - 1), north + (south - north) * row / (width - 1))
    for row in range(width)
    for column in range(width)
]
triangles = []
for row in range(width - 1):
    for column in range(width - 1):
        a = row * width + column
        b, c, d = a + 1, a + width, a + width + 1
        triangles.extend(((a, c, b), (b, c, d)))
mesh = tri.Triangulation(*zip(*coordinates), triangles)
for scenario in fixture["scenarios"]:
    surface = scenario["surface"]
    interval = scenario.get("intervalMetres", fixture["intervalMetres"])
    heights = [
        index % width - 12 if surface == "eastwardRamp"
        else 32 if surface == "isolatedHill" and index == 16 * (width + 1)
        else 0
        for index in range(width * width)
    ]
    levels = [
        index * interval
        for index in range(math.floor(min(heights) / interval) + 1, math.ceil(max(heights) / interval))
    ]
    assert levels == scenario["levels"], scenario["name"]
    if not levels:
        assert not scenario["routes"], scenario["name"]
        continue
    runs = plt.tricontour(mesh, heights, levels=levels).allsegs
    actual = [(level, LineString(route)) for level, routes in zip(levels, runs) for route in routes]
    assert len(actual) == len(scenario["routes"]), scenario["name"]
    for (level, route), wanted_level, expected in zip(actual, scenario["levels"], scenario["routes"]):
        assert level == wanted_level and route.equals(LineString(expected)), (scenario["name"], route)
    plt.close()
