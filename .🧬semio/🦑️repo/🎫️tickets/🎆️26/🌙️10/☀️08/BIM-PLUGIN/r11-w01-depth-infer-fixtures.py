#!/usr/bin/env python3
"""Adds the r11 w01 depth-parameter cases to the committed inference fixtures (stairs, stair runs, railings, roofs, plans), then the oracles rewrite the expected tables.

    python r11-w01-depth-infer-fixtures.py <repo root>
"""
import json
import math
import sys
from pathlib import Path

SUBSET = "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
FIXTURES = "🧫️fixtures/💡️inferences"


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


def save(path, document):
    path.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def stair(name, start, direction, width, flight, top, **more):
    row = {
        "storey": "st-ground", "start": {"x": start[0], "y": start[1]}, "direction": direction, "width": width, "flight": flight, "top": top,
        "max_riser": 0.18, "min_tread": 0.25, "stringer": {"kind": "None", "width": 0.05, "depth": 0.25}, "nosing": 0, "tread_thickness": 0.04,
        "riser": "Closed", "landing_depth": width, "phase": "New", "name": name,
    }
    row.update(more)
    return row


def stairs_flights(root):
    path = root / SUBSET / FIXTURES / "🧊️element-solids/🪜️stairs-flights/🔣️.json"
    document = load(path)
    top = {"StoreyTop": {"offset": 0.0}}
    closed = {"kind": "Closed", "width": 0.06, "depth": 0.24}
    added = {
        "st-closed-stringer": stair("Closed stringers and nosing", (0.0, 12.0), 0.0, 1.0, "Straight", top, stringer=closed, nosing=0.03),
        "st-open-stringer": stair("Open stringers, open risers", (0.0, 15.0), 0.0, 1.0, "Straight", top, stringer={"kind": "Open", "width": 0.05, "depth": 0.2}, nosing=0.02, tread_thickness=0.05, riser="Open"),
        "st-open-stringer-closed-risers": stair("Open stringers, closed risers", (0.0, 18.0), 0.0, 1.0, "Straight", top, stringer={"kind": "Open", "width": 0.05, "depth": 0.2}, tread_thickness=0.05),
        "st-mono-stringer": stair("Mono stringer, open risers", (0.0, 21.0), 0.0, 1.2, "Straight", top, stringer={"kind": "Mono", "width": 0.12, "depth": 0.22}, tread_thickness=0.05, riser="Open"),
        "st-l-closed-deep": stair("L turn, deep landing, closed stringers", (10.0, 12.0), 0.0, 1.0, {"LTurn": {"split": 0.5, "turn": "Left"}}, top, stringer=closed, nosing=0.02, landing_depth=1.4),
        "st-u-open-deep": stair("U turn, deep landing, open stringers", (20.0, 12.0), 1.5707963267948966, 1.0, {"UTurn": {"gap": 0.1}}, top, stringer={"kind": "Open", "width": 0.05, "depth": 0.2}, landing_depth=1.3, tread_thickness=0.05),
        "st-spiral-stringer": stair("Spiral with a stringer", (30.0, 12.0), 0.0, 0.9, {"Spiral": {"radius": 1.5, "sweep": 4.5}}, top, stringer=closed, max_riser=0.2, min_tread=0.2, landing_depth=0.9),
    }
    document["snapshot"]["stairs"].update(added)
    save(path, document)


def stair_runs(root):
    path = root / SUBSET / FIXTURES / "🪜️stair-runs/🪜️flights/📸️snapshot/🔣️.json"
    document = load(path)
    base = document["stairs"]["s-lturn"]
    deep_l = dict(base, name="L turn with a deep landing", landing_depth=1.6, start={"x": 20.0, "y": 0.0})
    deep_u = dict(document["stairs"]["s-uturn"], name="U turn with a deep landing", landing_depth=1.5, start={"x": 30.0, "y": 0.0})
    document["stairs"].update({"s-lturn-deep": deep_l, "s-uturn-deep": deep_u})
    save(path, document)


def railing(name, path, height, spacing, **more):
    row = {
        "storey": "st-ground", "path": [{"x": x, "y": y} for x, y in path], "height": height, "post_spacing": spacing, "profile": {"Rectangle": {"width": 0.06, "depth": 0.04}},
        "post_profile": {"Rectangle": {"width": 0.05, "depth": 0.05}}, "infill": "None", "material": "m-steel", "base_offset": 0.0, "phase": "New", "name": name,
    }
    row.update(more)
    return row


def railings_posts(root):
    path = root / SUBSET / FIXTURES / "🧊️element-solids/🛤️railings-posts/🔣️.json"
    document = load(path)
    document["snapshot"]["materials"]["m-glass"] = {"name": "Glass", "category": "Glass", "color": {"r": 0.7, "g": 0.85, "b": 0.9}, "density": 2500.0, "conductivity": 1.0, "specific_heat": 840.0}
    balusters = {"profile": {"Rectangle": {"width": 0.02, "depth": 0.02}}, "spacing": 0.13}
    added = {
        "r-balusters": railing("Balusters between posts", [(10.0, 0.0), (13.0, 0.0)], 1.0, 1.5, baluster=balusters),
        "r-glass": railing("Glass infill on an L", [(10.0, 3.0), (12.0, 3.0), (12.0, 5.0)], 1.0, 2.0, infill={"Glass": {"thickness": 0.012}}, profile={"Circle": {"diameter": 0.05}}),
        "r-panel": railing("Panel infill, round posts, I rail", [(10.0, 6.0), (13.0, 6.0)], 1.1, 1.5, infill={"Panel": {"thickness": 0.03}}, post_profile={"Circle": {"diameter": 0.06}}, profile={"IShape": {"width": 0.08, "depth": 0.06, "web": 0.02, "flange": 0.01}}),
        "r-balusters-glass": railing("Balusters in front of glass", [(10.0, 9.0), (12.4, 9.0), (12.4, 10.2)], 0.95, 1.2, baluster={"profile": {"Circle": {"diameter": 0.016}}, "spacing": 0.11}, infill={"Glass": {"thickness": 0.01}}, base_offset=0.05),
        "r-balusters-short": railing("Baluster spacing wider than the bay", [(10.0, 12.0), (11.0, 12.0)], 1.0, 0.0, baluster={"profile": {"Rectangle": {"width": 0.02, "depth": 0.02}}, "spacing": 2.0}, infill={"Panel": {"thickness": 0.02}}),
    }
    document["snapshot"]["railings"].update(added)
    save(path, document)


def footprint(*points):
    return [{"point": {"x": x, "y": y}, "bulge": 0.0} for x, y in points]


def roof(name, points, shape, **more):
    row = {"storey": "st-first", "roof_type": "rt-tiles", "footprint": footprint(*points), "shape": shape, "overhang": 0.0, "base_offset": 0.0, "phase": "New", "name": name}
    row.update(more)
    return row


def roofs_shapes(root):
    path = root / SUBSET / FIXTURES / "🧊️element-solids/🏔️roofs-shapes/🔣️.json"
    document = load(path)
    roofs = document["snapshot"]["roofs"]
    roofs["r-l-hip"]["name"] = "Hip on an L footprint"
    sin, cos = math.sin(0.3), math.cos(0.3)
    turned = [(round(x * cos - y * sin, 12), round(x * sin + y * cos, 12)) for x, y in [(0, 0), (8, 0), (8, 6), (0, 6)]]
    roofs["r-gable-rotated"].update(footprint=footprint(*turned), name="Gable on a rotated rectangle")
    ell = [(0, 0), (8, 0), (8, 3), (4, 3), (4, 6), (0, 6)]
    tee = [(0, 0), (9, 0), (9, 3), (6, 3), (6, 7), (3, 7), (3, 3), (0, 3)]
    ush = [(0, 0), (9, 0), (9, 6), (6, 6), (6, 3), (3, 3), (3, 6), (0, 6)]
    added = {
        "r-l-hip-overhang": roof("Hip on an L with overhang", ell, {"Hip": {"pitch": 0.5}}, overhang=0.4),
        "r-t-hip": roof("Hip on a T footprint", tee, {"Hip": {"pitch": 0.4}}),
        "r-u-hip": roof("Hip on a U footprint", ush, {"Hip": {"pitch": 0.5}}),
        "r-l-gable": roof("Gable on an L becomes a hip", ell, {"Gable": {"pitch": 0.35, "ridge_direction": 0.0}}),
        "r-l-mansard": roof("Mansard on an L footprint", ell, {"Mansard": {"lower_pitch": 1.1, "upper_pitch": 0.3, "break_height": 0.8}}),
        "r-hip-zero-pitch": roof("Hip of pitch zero is flat", [(0, 0), (8, 0), (8, 6), (0, 6)], {"Hip": {"pitch": 0.0}}),
        "r-mansard-no-break": roof("Mansard without a break height", [(0, 0), (8, 0), (8, 6), (0, 6)], {"Mansard": {"lower_pitch": 1.1, "upper_pitch": 0.3, "break_height": 0.0}}),
        "r-bowtie": roof("Footprint crossing itself", [(0, 0), (5, 4), (5, 0), (0, 3)], {"Hip": {"pitch": 0.4}}),
    }
    roofs.update(added)
    save(path, document)


def plan_cut_heights(root):
    base = root / SUBSET / FIXTURES / "🗺️plan-linework"
    house = load(next(base.glob("*house")) / "📸️snapshot/🔣️.json")
    house["storeys"]["st-ground"]["cut_height"] = 0.5
    house["storeys"]["st-first"]["cut_height"] = 2.5
    target = base / "🔪️cut-heights" / "📸️snapshot" / "🔣️.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    save(target, house)


def main(root):
    root = Path(root)
    stair_runs(root)
    stairs_flights(root)
    railings_posts(root)
    roofs_shapes(root)
    plan_cut_heights(root)


if __name__ == "__main__":
    main(sys.argv[1])
