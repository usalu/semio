import importlib

M = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🌬️ventilation.🎬️clip.🧪️architecture.🐍️")
V = M.VISUALS
api = M.helpers("🪟️natural")
cast = "shared-person-pathlines"
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
folder = S.ensure_casts([cast])[cast]
index = S._index(folder)
print({key: index.get(key) for key in ("heightM", "boundsM", "figureBoundsM", "mmPerMetre", "originMm")})
person = api["_stand"]([0.4, 0.2, 0], 2.5, 0.4, t=0.16)
unit = 0.7 * 2.34 / V.staffage_height(cast)
print("unit", unit, "group height", person.height, "probe height", (person.probes[2].get_center()[1] - person.probes[0].get_center()[1]))
ys = [p[1] for s in person.slots for p in s.get_all_points() if s.get_fill_opacity() > 0 or s.get_stroke_opacity() > 0]
print("slot extent", min(ys), max(ys), (max(ys) - min(ys)))
print("bottom", person.get_bottom()[1], "floor", 0.2 - 1.25 + 0.08)
