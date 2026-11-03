"""Writes the language-agnostic grip pick fixture. The line law expectations come from numpy's least squares (third-party reference), never from the implementation. Run: /home/deck/semio/.venv/bin/python -I grip-pick-fixture.py"""
import json, pathlib
import numpy as np

OUT = pathlib.Path("/home/deck/semio/🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🧪️tests/🧫️fixtures/🎯️grip-pick.json")


def numpy_line_parameter(ray_origin, ray_dir, origin, axis):
    a = np.array(axis, dtype=float)
    d = np.array(ray_dir, dtype=float)
    if np.linalg.matrix_rank(np.stack([a, d])) < 2:
        return None
    rhs = np.array(ray_origin, dtype=float) - np.array(origin, dtype=float)
    solution, *_ = np.linalg.lstsq(np.stack([a, -d], axis=1), rhs, rcond=None)
    return float(solution[0])


lines = [
    ("a ray straight across the line", [0, -10, 0], [0, 1, 0], [0, 0, 0], [1, 0, 0]),
    ("a ray above the line along the way", [3, 0, 5], [0, 0, -1], [0, 0, 0], [1, 0, 0]),
    ("a skew ray in a perspective view", [-4.0, -8.0, 6.0], [0.31, 0.62, -0.48], [1.0, 2.0, 0.5], [0.0, 0.0, 1.0]),
    ("a line with an offset origin and a diagonal axis", [10.0, -3.0, 4.0], [-0.4, 0.8, -0.2], [2.0, 1.0, 0.0], [0.6, 0.8, 0.0]),
    ("a negative parameter behind the origin", [-7.0, 0.0, 2.0], [0.0, 0.0, -1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
    ("a ray parallel to the line has no parameter", [0.0, 5.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
    ("a ray anti-parallel to the line has no parameter", [0.0, 5.0, 0.0], [-2.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
]
fixture = {
    "why": "The grip pick law every viewport host obeys: a hit edge per device (44 px for a finger, 24 px for a pointer, a device profile only raises it), the nearest grip inside half that edge (never less than its drawn radius), and the cursor ray turned into the parameter along the grip's line. `line` expectations are numpy least squares (np.linalg.lstsq of [axis, -dir] [t, u] = rayOrigin - origin), not the implementation. Language-agnostic: Rust and TypeScript replay the same rows.",
    "hitEdge": [
        {"device": "touch", "profileMin": 0, "expect": 44},
        {"device": "touch", "profileMin": 48, "expect": 48},
        {"device": "touch", "profileMin": 24, "expect": 44},
        {"device": "pointer", "profileMin": 0, "expect": 24},
        {"device": "pointer", "profileMin": 44, "expect": 44},
    ],
    "pick": [
        {"name": "a finger 21 px from a grip hits it", "device": "touch", "profileMin": 0, "drawn": 4, "points": [[100, 100]], "at": [121, 100], "expect": 0},
        {"name": "a finger exactly on the 22 px radius hits it", "device": "touch", "profileMin": 0, "drawn": 4, "points": [[100, 100]], "at": [122, 100], "expect": 0},
        {"name": "a finger 23 px away misses", "device": "touch", "profileMin": 0, "drawn": 4, "points": [[100, 100]], "at": [123, 100], "expect": None},
        {"name": "a pointer 11 px from a grip hits it", "device": "pointer", "profileMin": 0, "drawn": 4, "points": [[100, 100]], "at": [111, 100], "expect": 0},
        {"name": "a pointer 13 px away misses what a finger would hit", "device": "pointer", "profileMin": 0, "drawn": 4, "points": [[100, 100]], "at": [113, 100], "expect": None},
        {"name": "a big drawn grip is never harder to hit than it looks", "device": "pointer", "profileMin": 0, "drawn": 30, "points": [[100, 100]], "at": [129, 100], "expect": 0},
        {"name": "the nearer of two grips wins", "device": "touch", "profileMin": 0, "drawn": 4, "points": [[100, 100], [112, 100]], "at": [109, 100], "expect": 1},
        {"name": "equally near grips pick the first", "device": "touch", "profileMin": 0, "drawn": 4, "points": [[90, 100], [110, 100]], "at": [100, 100], "expect": 0},
        {"name": "a device profile with a larger minimum widens the reach", "device": "pointer", "profileMin": 60, "drawn": 4, "points": [[100, 100]], "at": [129, 100], "expect": 0},
        {"name": "no grips pick nothing", "device": "touch", "profileMin": 0, "drawn": 4, "points": [], "at": [0, 0], "expect": None},
    ],
    "line": [
        {"name": name, "rayOrigin": ro, "rayDir": rd, "origin": o, "axis": ax, "expect": numpy_line_parameter(ro, rd, o, ax)}
        for name, ro, rd, o, ax in lines
    ],
    "drag": [
        {"anchorS": 2.0, "pressT": 3.0, "cursorT": 3.0, "expect": 2.0},
        {"anchorS": 2.0, "pressT": 3.0, "cursorT": 4.5, "expect": 3.5},
        {"anchorS": -1.0, "pressT": 0.0, "cursorT": -0.25, "expect": -1.25},
    ],
}
OUT.write_text(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print({row["name"]: row["expect"] for row in fixture["line"]})
