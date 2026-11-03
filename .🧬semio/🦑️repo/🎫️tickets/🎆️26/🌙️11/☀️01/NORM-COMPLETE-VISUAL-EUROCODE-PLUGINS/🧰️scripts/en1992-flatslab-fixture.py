"""Writes the EN 1992 flat-slab punching fixture (the JSON twin of En1992Snapshot::flat_slab_punching) next to the check-outcome fixtures."""
import json
import pathlib
import sys

ROOT = pathlib.Path(sys.argv[1])
COLUMNS = [
    ("slab-FS-interior", "Flat slab interior column", "Flachdecke Innenstütze", "Interior", 300e3, 110e3, [(0.10, 12), (0.24, 12), (0.38, 12), (0.52, 12)]),
    ("slab-FS-edge", "Flat slab edge column", "Flachdecke Randstütze", "Edge", 160e3, 60e3, [(0.10, 8), (0.24, 8), (0.38, 8), (0.52, 8)]),
    ("slab-FS-corner", "Flat slab corner column", "Flachdecke Eckstütze", "Corner", 85e3, 30e3, [(0.10, 5), (0.24, 5), (0.38, 5), (0.62, 5)]),
]


def action(id, kind, category, m_k, v_punch):
    return {"id": id, "kind": kind, "category": category, "source": "External", "gKLine": 0.0, "qKLine": 0.0, "pointForce": 0.0, "mK": m_k, "nK": 0.0, "vK": 0.0, "tK": 0.0, "vKPunch": v_punch}


def member(id, en, de, position, g, q, rings):
    return {
        "id": id, "labelEn": en, "labelDe": de, "kind": "FlatSlab", "concreteGradeId": "c30", "reinforcementGradeId": "b500", "prestressSteelId": "", "exposure": "Xc1",
        "width": 1.0, "height": 0.3, "effectiveDepth": 0.255, "cover": 0.025, "span": 6.0, "support": "Continuous", "bucklingLength": 0.0,
        "longitudinal": [{"id": "bot", "diameter": 0.014, "count": 8, "position": "Bottom", "anchorageLength": 0.65, "lapLength": 0.85, "bondCondition": "Good", "aggregateSize": 0.016, "lapShare": 1.0}],
        "stirrups": None, "punching": {"columnWidth": 0.3, "columnDepth": 0.3, "columnPosition": position, "asw": 0.0, "betaMethod": "Simplified", "momentC1": 0.0, "momentC2": 0.0}, "prestress": None,
        "fire": {"rating": "R60", "axisDistance": 0.035, "slabSystem": "OneWay"},
        "actions": [action("G", "Permanent", "SelfWeight", 30e3, g), action("Q-office", "Imposed", "Office", 15e3, q)],
        "useFem": True, "udl": 0.0, "deflectionSensitive": False, "tightness": None, "hdOverH": 0.0, "liquidSigmaS": 0.0, "liquidRhoPEff": 0.0, "liquidFCtEff": 0.0, "liquidSRMax": 0.0,
        "bridgeSigmaC": 0.0, "bridgeDeltaSigmaS": 0.0, "relativeHumidity": 0.5, "sectionPoints": [], "bars": [], "tendonPoints": [],
        "studRings": [{"id": f"r{k + 1}", "radius": r, "count": n, "diameter": 0.012} for k, (r, n) in enumerate(rings)],
    }


document = {
    "annex": "De", "reliabilityClass": "RC2", "title": "Flat slab with punching reinforcement", "designWorkingLifeYears": 50.0, "deltaCDev": 0.01, "cementType": "N",
    "concreteGrades": [{"id": "c30", "name": "C30/37", "fCk": 30.0e6}],
    "reinforcementGrades": [{"id": "b500", "name": "B500B", "fYk": 500.0e6, "eS": 200.0e9, "ductility": "b", "k": 1.08, "epsUk": 0.05}],
    "prestressSteels": [], "members": [], "anchors": [], "placements": [], "cases": [],
}
members = [member(*c) for c in COLUMNS]
small = {**members[0], "id": "slab-FS-small", "labelEn": "Flat slab small interior column", "labelDe": "Flachdecke kleine Innenstütze", "punching": {**members[0]["punching"], "columnWidth": 0.2, "columnDepth": 0.2}}
for name, bare, annex, only in (("flat-slab-punching", False, "De", None), ("flat-slab-punching-bare", True, "De", None), ("flat-slab-punching-en", False, "En", None), ("flat-slab-punching-small-column-de", False, "De", small), ("flat-slab-punching-small-column-en", False, "En", small), ("flat-slab-punching-small-column-de-bare", True, "De", small), ("flat-slab-punching-small-column-en-bare", True, "En", small)):
    document["annex"] = annex
    document["members"] = [{**m, "studRings": [] if bare else m["studRings"]} for m in ([only] if only else members)]
    target = ROOT / name / "🔣️.json"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
