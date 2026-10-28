import importlib.util, json
CLIP = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
spec = importlib.util.spec_from_file_location("m", CLIP + "/🧮️model/🐍️.py"); m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
b = m.evaluate()
new = ["conductance", "heating_losses", "heating_gains", "heating_eta", "heating_used", "cop", "cooling_gains", "cooling_removable", "cooling_eta", "cooling_removed", "eer",
       "internal_winter", "internal_summer", "beam_winter", "beam_summer", "direct_winter", "direct_summer", "lighting_power", "lighting_hours", "lamps_on"]
base = {k: round(b[k], 6) for k in new}
print(json.dumps(base))
design = {"wwr": 0.45, "overhang": 0.6, "insulation": 0.16}
probe = {"design": design, "bearings": [], "shading_summer": [], "cooling": [], "electricity": []}
for t in (354.0, 354.5, 355.0, 356.0, 360.0):
    r = m.evaluate({**design, "orientation": t})
    probe["bearings"].append(t)
    for k in ("shading_summer", "cooling", "electricity"):
        probe[k].append(round(r[k], 6))
print(json.dumps(probe))
r = m.evaluate({**design, "orientation": 354.0}); print({k: r[k] for k in ("beam_summer", "direct_summer")}, m.sun_samples(354, "summer"))
