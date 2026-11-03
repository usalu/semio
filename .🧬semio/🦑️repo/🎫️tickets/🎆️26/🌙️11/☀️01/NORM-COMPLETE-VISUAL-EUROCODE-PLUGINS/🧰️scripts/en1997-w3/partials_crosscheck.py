"""Third-party check of the EN 1997 partials (GI-01 spacing and depth, GI-03 N_kt): scipy cKDTree nearest neighbours against the independent oracle and the hand values of the Rust tests. Run: /home/deck/semio/.venv/bin/python -I partials_crosscheck.py"""
import importlib.util, json, math, pathlib, sys
import numpy as np
from scipy.spatial import cKDTree

ART = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any")
spec = importlib.util.spec_from_file_location("oracle", ART / "🔮️oracles/⚖️compliance/🐍️.py")
oracle = importlib.util.module_from_spec(spec)
sys.modules["oracle"] = oracle
spec.loader.exec_module(oracle)

def nearest(points):
    tree = cKDTree(np.array(points, dtype=float))
    distance, _ = tree.query(np.array(points, dtype=float), k=2)
    return distance[:, 1]

hand = [(0.0, 0.0), (30.0, 0.0), (30.0, 40.0)]
assert list(nearest(hand)) == [30.0, 30.0, 40.0], nearest(hand)
for limit, gk, expected in ((40.0, 2, 1.0), (25.0, 3, 1.6)):
    doc = {"geotechnicalCategory": gk, "investigationDepth": 20.0, "investigations": [{"positionX": x, "positionY": y, "depth": 20.0, "id": f"p{i}"} for i, (x, y) in enumerate(hand)]}
    util = {c["id"]: c["utilization"] for c in oracle.investigation_extent_checks(doc, "De")}["en1997.gi-01.spacing"]
    assert abs(util - nearest(hand).max() / limit) < 1e-12 and abs(util - expected) < 1e-12, (gk, util)
fixture = json.load(open(ART / "🧪️tests/🧫️fixtures/🚦️check-outcome/ground-showcase-pass/🔣️.json"))
pts = [(p["positionX"], p["positionY"]) for p in fixture["investigations"]]
util = {c["id"]: c["utilization"] for c in oracle.investigation_extent_checks(fixture, "De")}
assert abs(util["en1997.gi-01.spacing"] - nearest(pts).max() / 40.0) < 1e-12, util
for pile, required in (("pile-P1", 14.0 + max(5.0, 3 * 0.6)), ("pile-P2", None)):
    row = next(p for p in fixture["piles"] if p["id"] == pile)
    need = row["length"] + max(5.0, 3.0 * row["diameter"])
    assert abs(util[f"en1997.gi-01.pile.{pile}"] - need / 20.0) < 1e-12
    if required is not None:
        assert abs(need - required) < 1e-12 and abs(need - 19.0) < 1e-12
print("partials cross-check ok: nearest", list(nearest(pts)), "spacing", util["en1997.gi-01.spacing"])
