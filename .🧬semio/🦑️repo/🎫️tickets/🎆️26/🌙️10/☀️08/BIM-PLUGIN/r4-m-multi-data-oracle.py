"""Third-party cross-check of the m-multi-data placement leaves (shapely 2.x).

Reads the committed fixture quintets of `move-elements`, `rotate-elements` and `place-elements`, recomputes the expected placements of
every selected element with `shapely.affinity.translate` / `rotate`, and compares them with the committed after snapshot within 1e-9.
Run: `.venv/Scripts/python.exe .🧬semio/…/BIM-PLUGIN/r4-m-multi-data-oracle.py` (POSIX: `.venv/bin/python`). Exit code 0 means every case agrees.
"""

import json
import math
import sys
from pathlib import Path

from shapely.affinity import rotate, translate
from shapely.geometry import Point

ROOT = Path(__file__).resolve().parents[7]
FIXTURES = next(ROOT.glob("*s/*plugins/*bim/*artifacts/*model/*standards/*1/*subsets/*any/*fixtures/*mutations"))
TOLERANCE = 1e-9


def leaf_dir(kind):
    return next(path for path in FIXTURES.iterdir() if path.name.endswith(kind) and len(path.name) > len(kind))


def load(case, part):
    pattern = "*snapshot/*" + part + "/*.json" if part in ("before", "after") else "*" + part + "/*.json"
    return json.loads(next(case.glob(pattern)).read_text(encoding="utf-8"))


def status(case):
    return json.loads(next(case.glob("*outcome/*.json")).read_text(encoding="utf-8"))["status"]


def point(raw, transform):
    moved = transform(Point(raw["x"], raw["y"]))
    return {"x": moved.x, "y": moved.y}


def vertices(raw, transform):
    return [{"point": point(vertex["point"], transform), "bulge": vertex["bulge"]} for vertex in raw]


def axis(raw, transform):
    (name, body), = raw.items()
    return {name: {**body, "start": point(body["start"], transform), "end": point(body["end"], transform)}}


def expected(collection, row, transform, turn):
    if collection in ("walls", "curtain_walls"):
        return {"axis": axis(row["axis"], transform)}
    if collection == "columns":
        return {"position": point(row["position"], transform), "rotation": row["rotation"] + turn}
    if collection in ("beams", "grids"):
        return {"start": point(row["start"], transform), "end": point(row["end"], transform)}
    if collection == "slabs":
        slope = row.get("slope")
        return {"boundary": vertices(row["boundary"], transform), "holes": [vertices(hole, transform) for hole in row["holes"]], "slope": None if slope is None else {**slope, "direction": slope["direction"] + turn}}
    if collection == "roofs":
        shape = row["shape"]
        if isinstance(shape, dict):
            (name, body), = shape.items()
            body = dict(body)
            for key in ("direction", "ridge_direction"):
                if key in body:
                    body[key] += turn
            shape = {name: body}
        return {"footprint": vertices(row["footprint"], transform), "shape": shape}
    if collection == "stairs":
        return {"start": point(row["start"], transform), "direction": row["direction"] + turn}
    if collection == "railings":
        return {"path": [point(p, transform) for p in row["path"]]}
    if collection == "spaces":
        (name, body), = row["boundary"].items()
        return {"boundary": {name: {"seed": point(body["seed"], transform)} if name == "Bounded" else {"outline": vertices(body["outline"], transform)}}}
    raise KeyError(collection)


def close(left, right):
    if isinstance(left, dict):
        return left.keys() == right.keys() and all(close(left[key], right[key]) for key in left)
    if isinstance(left, list):
        return len(left) == len(right) and all(close(a, b) for a, b in zip(left, right))
    if isinstance(left, float) or isinstance(right, float):
        return left is not None and right is not None and abs(left - right) <= TOLERANCE
    return left == right


def check(kind):
    failures, checked = [], 0
    for case in sorted(leaf_dir(kind).iterdir()):
        if not case.is_dir():
            continue
        if status(case) != "applied":
            continue
        before, after, mutation = load(case, "before"), load(case, "after"), load(case, "mutation")
        if kind == "move-elements":
            transform, turn = (lambda shape: translate(shape, mutation["vector"]["x"], mutation["vector"]["y"])), 0.0
        elif kind == "rotate-elements":
            pivot, turn = Point(mutation["pivot"]["x"], mutation["pivot"]["y"]), mutation["angle"]
            transform = lambda shape: rotate(shape, turn, origin=pivot, use_radians=True)
        else:
            transform, turn = None, 0.0
        for collection in ("walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "stairs", "railings", "spaces", "grids"):
            for element, row in before[collection].items():
                if kind == "place-elements":
                    placed = mutation["placements"].get(element)
                    want = None if placed is None else next(iter(placed.values()))
                    got = {key: after[collection][element][key] for key in want} if want is not None else None
                elif element in mutation["ids"]:
                    want = expected(collection, row, transform, turn)
                    got = {key: after[collection][element].get(key) for key in want}
                else:
                    want, got = None, None
                    if after[collection][element] != row:
                        failures.append(f"{case.name}: {element} changed without being selected")
                if want is not None:
                    checked += 1
                    if not close(want, got):
                        failures.append(f"{case.name}: {element} differs from shapely\n  want {want}\n  got  {got}")
    return checked, failures


def main():
    bad = 0
    for kind in ("move-elements", "rotate-elements", "place-elements"):
        checked, failures = check(kind)
        print(f"{kind}: {checked} element placements checked, {len(failures)} differ")
        for failure in failures:
            print("  " + failure)
        bad += len(failures)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
