from pathlib import Path
clip = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
p = clip / "🧮️model/🔣️.json"
s = p.read_text(encoding="utf-8")
old = '      "internal_gains_winter":'
assert s.count(old) == 1
s = s.replace(old, '''      "direct_share_winter": {"value": 0.5, "unit": "1", "note": {"en": "Share of the winter irradiation that arrives as direct beam; only this share can be shaded, the diffuse rest always enters.", "de": "Anteil der Winter-Einstrahlung, der als direkte Strahlung ankommt; nur dieser Anteil lässt sich verschatten, der diffuse Rest tritt immer ein."}},
      "direct_share_summer": {"value": 0.6, "unit": "1", "note": {"en": "Same for summer: the clear summer sky is more direct.", "de": "Dasselbe für den Sommer: Der klare Sommerhimmel ist direkter."}},
''' + old)
p.write_text(s, encoding="utf-8", newline="\n")
p = clip / "🧮️model/🐍️.py"
s = p.read_text(encoding="utf-8")
old = '''    solar = {season: window * envelope["g_value"] * _irradiance(season, phi) * (1 - shading[season]) for season in shading}'''
assert old in s
s = s.replace(old, '''    solar = {season: window * envelope["g_value"] * _irradiance(season, phi) * (1 - climate[f"direct_share_{season}"] * shading[season]) for season in shading}''')
p.write_text(s, encoding="utf-8", newline="\n")
b = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/model-tools/build_schema.py")
s = b.read_text(encoding="utf-8")
s = s.replace('"summer_irradiance_cos1", "summer_irradiance_cos2", "internal_gains_winter"', '"summer_irradiance_cos1", "summer_irradiance_cos2", "direct_share_winter", "direct_share_summer", "internal_gains_winter"')
b.write_text(s, encoding="utf-8", newline="\n")
