from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py")
s = p.read_text(encoding="utf-8")
old = '''    solar = {season: window * envelope["g_value"] * _irradiance(season, phi) * (direct[season] * (1 - shading[season]) + (1 - direct[season]) * sky) for season in shading}'''
assert old in s
s = s.replace(old, '''    solar = {season: window * envelope["g_value"] * _irradiance(season, phi) * (1 - direct[season] * shading[season]) for season in shading}''')
p.write_text(s, encoding="utf-8", newline="\n")
