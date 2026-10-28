from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🔣️.json")
s = p.read_text(encoding="utf-8")
a = s.index('  "sun": {'); b = s.index('  "baseline":')
s = s[:a] + '''  "sun": {
    "latitude": 47.4,
    "winter_altitude": 19,
    "summer_altitude": 66,
    "low_summer_altitude": 30,
    "winter_azimuth": [128, 232],
    "summer_azimuth": [53, 307],
    "note": {"en": "Noon sun altitudes at the solstices and typical low summer sun in the morning and evening (compass bearing, 180 degrees is south).", "de": "Mittagshoehen der Sonne zu den Sonnenwenden und typische tiefe Sommersonne morgens und abends (Kompasswinkel, 180 Grad ist Sued)."}
  },
''' + s[b:]
a = s.index('  "sampling": {'); b = s.index('  "factorial": {')
s = s[:a] + '''  "sampling": {
    "levels": {"wwr": [0.1, 0.45, 0.8], "overhang": [0, 0.6, 1.2], "orientation": [90, 180, 270, 360], "insulation": [0.1, 0.2, 0.3]},
    "random": [[0.057, 0.792], [0.186, 0.25], [0.329, 0.6], [0.357, 0.65], [0.529, 0.1], [0.686, 0.125], [0.857, 0.517], [0.957, 0.25], [0.114, 0.067]],
    "latin_permutation": [5, 1, 8, 6, 2, 3, 0, 7, 4]
  },
''' + s[b:]
a = s.index('  "factorial": {'); b = s.index('  "criteria": [')
s = s[:a] + '''  "factorial": {
    "note": {"en": "Two factors at two levels on the south facade; response is the cooling electricity. The third factor is only used by the half fraction.", "de": "Zwei Faktoren auf zwei Stufen an der Suedfassade; Zielgroesse ist der Kuehlstrom. Der dritte Faktor wird nur im halben Versuchsplan genutzt."},
    "design": {"orientation": 180, "wwr": 0.45, "overhang": 0.6, "insulation": 0.16},
    "response": "cooling",
    "factors": [
      {"id": "wwr", "levels": [0.3, 0.7], "unit": "%", "title": {"en": "Window-to-wall ratio", "de": "Fensteranteil"}},
      {"id": "overhang", "levels": [0, 1], "unit": "m", "title": {"en": "Overhang depth", "de": "Vordachtiefe"}}
    ],
    "third": {"id": "insulation", "levels": [0.1, 0.3], "unit": "m", "title": {"en": "Insulation thickness", "de": "Daemmstaerke"}}
  },
  "weights": {
    "balanced": {"electricity": 0.4, "cost": 0.3, "daylight": 0.2, "carbon": 0.1},
    "cost_heavy": {"electricity": 0.2, "cost": 0.6, "daylight": 0.13333333333333333, "carbon": 0.06666666666666667}
  },
''' + s[b:]
s = s.replace('''    {"id": "glass_bare", "title": {"en": "Bare glass wall", "de": "Unverschattete Glaswand"}, "design": {"orientation": 180, "wwr": 0.8, "overhang": 0, "insulation": 0.16}}
''', '''    {"id": "glass_bare", "title": {"en": "Bare glass wall", "de": "Unverschattete Glaswand"}, "design": {"orientation": 180, "wwr": 0.8, "overhang": 0, "insulation": 0.16}},
    {"id": "glass_premium", "extra": true, "title": {"en": "Premium glass wall", "de": "Premium-Glaswand"}, "design": {"orientation": 180, "wwr": 0.8, "overhang": 1.2, "insulation": 0.3}}
''')
a = s.index('  "pareto": {'); b = s.index('  "algorithms": {')
s = s[:a] + '''  "pareto": {
    "note": {"en": "South facade grid: 5 window ratios, 3 insulation thicknesses, 2 overhang depths.", "de": "Suedfassaden-Raster: 5 Fensteranteile, 3 Daemmstaerken, 2 Vordachtiefen."},
    "design": {"orientation": 180, "wwr": 0.45, "overhang": 0.6, "insulation": 0.16},
    "wwr": [0.2, 0.35, 0.5, 0.65, 0.8],
    "insulation": [0.12, 0.2, 0.3],
    "overhang": [0, 0.6],
    "weights": [0, 0.25, 0.5, 0.75, 1],
    "dominated_example": {"winner": "w35-t12-d0", "loser": "w65-t12-d0"},
    "step": {"from": "w20-t12-d0", "to": "w35-t12-d0"},
    "refinement": {"member": "w50-t20-d60", "design": {"wwr": 0.5, "overhang": 0.45, "insulation": 0.2}}
  },
''' + s[b:]
s = s.replace('"gradient": {"difference": 0.01, "learning_rate": 0.0005, "iterations": 12, "tolerance": 2}', '"gradient": {"difference": 0.01, "learning_rate": 0.0005, "iterations": 10}')
s = s.replace('"population": 8, "generations": 6,', '"population": 8, "generations": 7,')
s = s.replace('"algorithms": {\n', '"algorithms": {\n    "budget": 50,\n')
s = s.replace('"pattern": {"step": {"wwr": 0.2, "overhang": 0.3}, "contraction": 0.5, "minimum_step": {"wwr": 0.02, "overhang": 0.03}, "tolerance": 0.001, "iterations": 20}', '"pattern": {"step": {"wwr": 0.2, "overhang": 0.3}, "contraction": 0.5, "tolerance": 0.001, "iterations": 12}')
p.write_text(s, encoding="utf-8")
