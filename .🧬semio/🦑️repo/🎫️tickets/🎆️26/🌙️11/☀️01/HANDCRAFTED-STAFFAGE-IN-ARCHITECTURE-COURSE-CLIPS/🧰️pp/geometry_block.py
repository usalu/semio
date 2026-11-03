bones = [
    {"id": "case", "parent": None, "title": t("Case with feet and components", "Gehäuse mit Füßen und Komponenten"), "x": 0, "y": 0, "angleDeg": 0,
     "lengthM": 0.45, "joint": {"type": "fixed"},
     "mass": {"kg": 11.0, "comX": -0.02, "comY": 0.2, "source": "authored-estimate", "distribution": "point"}},
]
anchors = [
    {"id": "feet-contact", "bone": "case", "x": 0, "y": 0, "kind": "contact", "title": t("Feet on the floor", "Füße auf dem Boden"), "support": "contact",
     "footprintLengthM": 0.4, "footprintWidthM": 0.17, "frictionCoefficient": 0.6},
    {"id": "rear-outlet", "bone": "case", "x": -0.225, "y": 0.3, "kind": "emitter", "title": t("Rear fan exhaust", "Abluft des Rücklüfters")},
    {"id": "case-top", "bone": "case", "x": -0.1, "y": 0.45, "kind": "emitter", "title": t("Warm air above the case", "Warme Luft über dem Gehäuse")},
]
