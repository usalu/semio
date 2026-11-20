"""🌱 Root values of the round 3 delivery van besides rig and drawings: overall width over the mirrors, the cited class reference, thermal areas measured on the honest 1:50 model, the true-extent symbol and the texts that state these numbers."""
MEMBERS = ["summary", "provenance", "dimensions", "symbols", "poses", "thermal"]
REFERENCE = {"id": "medium-panel-vans", "citation": "Manufacturer technical data of medium panel vans of about 5.3 m: Ford Transit Custom L2 H1 (2012): length 5340 mm, width 1986 mm without and 2272 mm with mirrors, height 1972 to 2000 mm, wheelbase 3300 mm, front overhang 1011 mm, rear overhang 1029 mm, track 1736 / 1720 mm; Volkswagen Transporter T6 long wheelbase (2015): 5304 x 1904 x 1990 mm, 2297 mm with mirrors, wheelbase 3400 mm, track 1628 mm; Mercedes-Benz Vito long (2014): 5140 x 1928 x 1910 mm, 2244 mm with mirrors, wheelbase 3200 mm, track 1666 / 1646 mm; Renault Trafic L2 (2014): 5399 x 1956 x 1971 mm, wheelbase 3498 mm. The asset is a generic van of this class: 5.34 m long, 1.97 m body width, 2.27 m over the mirrors, 1.99 m high, wheelbase 3.30 m, front overhang 0.97 m, rear overhang 1.07 m, track 1.68 m, static front axle share of the empty van 57 %."}
AREAS = {"body-panels": 27.0, "roof": 7.4, "cab-glazing": 3.3}
SYMBOL = {
    "elevation": {"elevation-wheel-f": {"rx": 0.356, "ry": 0.356, "cy": 0.356}, "elevation-wheel-r": {"rx": 0.356, "ry": 0.356, "cy": 0.356}, "elevation-body": {"points": [[-2.67, 0.34], [-2.67, 1.99], [1.12, 1.99], [1.75, 1.2], [2.55, 1.0], [2.67, 0.62], [2.67, 0.34]]}, "elevation-windscreen": {"points": [[1.125, 1.83], [1.745, 1.205], [1.6, 1.205], [0.98, 1.83]]}, "elevation-door-window": {"x": 0.37, "y": 1.17, "w": 0.83, "h": 0.63}, "elevation-slider": {"x": -0.8, "y": 0.4, "w": 1.05, "h": 1.44}},
    "front": {"front-body": {"x": -0.985, "y": 0.34, "w": 1.97, "h": 1.65}, "front-tyre-l": {"x": 0.7225, "y": 0, "w": 0.235, "h": 0.712}, "front-tyre-r": {"x": -0.9575, "y": 0, "w": 0.235, "h": 0.712}, "front-windscreen": {"x": -0.9, "y": 1.205, "w": 1.8, "h": 0.625}, "front-mirror-l": {"x": 0.945, "y": 1.25, "w": 0.19, "h": 0.27}, "front-mirror-r": {"x": -1.135, "y": 1.25, "w": 0.19, "h": 0.27}},
    "plan": {"plan-body": {"points": [[-2.67, -0.985], [2.32, -0.985], [2.67, -0.66], [2.67, 0.66], [2.32, 0.985], [-2.67, 0.985]]}, "plan-tyre-fl": {"x": 1.344, "y": 0.7225, "w": 0.712, "h": 0.235}, "plan-tyre-fr": {"x": 1.344, "y": -0.9575, "w": 0.712, "h": 0.235}, "plan-tyre-rl": {"x": -1.956, "y": 0.7225, "w": 0.712, "h": 0.235}, "plan-tyre-rr": {"x": -1.956, "y": -0.9575, "w": 0.712, "h": 0.235}, "plan-windscreen": {"x": 1.1, "y": -0.9, "w": 0.64, "h": 1.8}, "plan-slider": {"x": -0.8, "y": -0.985, "w": 1.05, "h": 0.045}, "plan-rear-doors": {"points": [[-2.61, -0.86], [-2.61, 0.86]]}, "plan-mirror-l": {"x": 1.355, "y": 0.945, "w": 0.12, "h": 0.19}, "plan-mirror-r": {"x": 1.355, "y": -1.135, "w": 0.12, "h": 0.19}},
}


def apply(asset):
    references = asset["provenance"]["references"]
    if all(reference["id"] != REFERENCE["id"] for reference in references):
        references.insert(0, REFERENCE)
    asset["provenance"]["notes"] = {
        "en": "Typical medium panel van (wheelbase 3.30 m, body width 1.97 m, 2.27 m over the mirrors, kerb mass 2.0 t, 235/65 R16); dimensions follow the cited class data, masses and heat data are authored class values (authored-estimate). Wheels roll with theta = -x / r (r 0.356 m).",
        "de": "Typischer mittlerer Kastenwagen (Radstand 3,30 m, Aufbaubreite 1,97 m, 2,27 m über die Spiegel, Leermasse 2,0 t, 235/65 R16); die Abmessungen folgen den zitierten Klassendaten, Massen und Wärmedaten sind selbst gesetzte Klassenwerte (authored-estimate). Die Räder rollen mit theta = -x / r (r 0,356 m).",
    }
    asset["dimensions"]["depthM"] = 2.27
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
    for pose in asset["poses"]:
        if pose["id"] == "rear-doors-open":
            pose["description"] = {"en": "Both rear leaves swung 90 degrees (0.86 m behind the rear panel), engine off, 350 kg of cargo.", "de": "Beide Hecktürflügel um 90 Grad geschwenkt (0,86 m hinter der Heckwand), Motor aus, 350 kg Ladung."}
    for symbol in asset["symbols"]:
        for view, shapes in SYMBOL.items():
            for shape in symbol["geometry"][view]:
                shape.update(SYMBOL[view].get(shape["id"], {}))
        symbol["dimensionalNote"] = {"en": "Model metres: length 5.34 m, width over the mirrors 2.27 m (body 1.97 m), height 1.99 m; side, front and top view.", "de": "Modellmeter: Länge 5,34 m, Breite über die Spiegel 2,27 m (Aufbau 1,97 m), Höhe 1,99 m; Seiten-, Vorder- und Draufsicht."}
    return asset
