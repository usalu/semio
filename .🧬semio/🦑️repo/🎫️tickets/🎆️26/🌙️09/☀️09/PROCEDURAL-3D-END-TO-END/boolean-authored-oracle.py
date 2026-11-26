#!/usr/bin/env python3
"""📐️ Independently derive the authored sphere/box and sphere/torus preview volumes and bounds."""
import json
from math import asin, pi, sqrt
from pathlib import Path
import re

fixtures = Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples")


def example(directory, name):
    home = fixtures / directory
    text = (home / "🖼️assets" / directory / "🗣️.dsl.semio").read_text()
    reference = json.loads((home / "🧪️tests/🧫️fixtures/🧩️example/🔣️.json").read_text())
    assert reference["example"] == name
    return text, reference["expect"]


def slider(text, identifier):
    match = re.search(fr'input-slider id="{re.escape(identifier)}"[^\n]+?value=([0-9.]+)', text)
    return float(match.group(1))


def torus_defaults():
    path = Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧾️register-operators/🧊️schemas-primitives-and-curves/🦀️.rs")
    text = path.read_text()
    return tuple(float(re.search(fr'number_channel\("{port}", "brep\.prim3d\.torus", ([0-9.]+)\)', text).group(1)) for port in ("major", "minor"))


def corner_union(radius, side):
    assert side >= radius > 0.0
    volume = 4.0 * pi * radius**3 / 3.0 * 7.0 / 8.0 + side**3
    return volume, [-radius] * 3, [side] * 3


def coaxial_difference(radius, major, minor):
    assert 0.0 < minor < major and major - minor < radius < major + minor
    meeting = (radius**2 + major**2 - minor**2) / (2.0 * major)
    offset = meeting - major
    half_height = sqrt(minor**2 - offset**2)
    torus_primitive = -half_height**3 / 3.0 + major * (offset * half_height + minor**2 * asin(offset / minor)) / 2.0
    sphere_primitive = half_height**3 / 3.0
    overlap = 4.0 * pi * (torus_primitive + major * minor**2 * pi / 4.0 + sphere_primitive)
    volume = 4.0 * pi * radius**3 / 3.0 - overlap
    return volume, [-meeting, -meeting, -radius], [meeting, meeting, radius]


def compare(reference, result):
    volume, lower, upper = result
    assert abs(volume - reference["volume"]) < 1e-9, (volume, reference["volume"])
    for expected, measured in ((reference["boundingBoxMin"], lower), (reference["boundingBoxMax"], upper)):
        assert all(abs(a - b) < 1e-9 for a, b in zip(expected, measured)), (expected, measured)
    return {"volume": volume, "bounds": [lower, upper]}


def main():
    cube_graph, cube_reference = example("🧲️sphere-box-fuse", "sphere-box-fuse")
    torus_graph, torus_reference = example("🍩️sphere-cut-with-torus", "sphere-cut-with-torus")
    cube = compare(cube_reference, corner_union(slider(cube_graph, "radius"), slider(cube_graph, "size")))
    torus = compare(torus_reference, coaxial_difference(slider(torus_graph, "slider_2"), *torus_defaults()))
    print(json.dumps({"sphere-box-fuse": cube, "sphere-cut-with-torus": torus}, indent=2))


if __name__ == "__main__":
    main()
