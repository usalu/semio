import sys; sys.path.insert(0, ".")
from silhouette_edit import *
PLAN = {
    "cooking-pot": {"50": {"elevation": 0.0}, "100": {"elevation": 0.0}},
    "retractable-bollard": {"50": {"elevation": 0.0, "front": 0.0}, "100": {"elevation": 0.0, "front": 0.0}},
    "panel-radiator": {"50": {"elevation": 0.0}},
    "sectional-garage-door": {"50": {"elevation": 0.0}, "100": {"elevation": 0.0}},
    "solar-thermal-collector": {"50": {"elevation": 0.0}, "100": {"elevation": 0.0}, "200": {"elevation": 0.0}, "500": {"elevation": 0.0}},
}
LABEL = {"elevation": "A-A", "front": "B-B"}
for slug, scales in PLAN.items():
    p = find(slug)
    raw, doc, idx = load(p)
    ops = []
    for sc, views in scales.items():
        r = doc["asset"]["representations"][sc]
        assert "sections" not in r, (slug, sc)
        sec = {v: {"plane": pl, "label": {"en": LABEL[v], "de": LABEL[v]}} for v, pl in views.items()}
        ops.append(("insert_member_after", ("asset", "representations", sc, "geometry"), ("sections", sec)))
    print(slug, apply(p, ops, expect=raw))
