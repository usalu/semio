from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py")
s = p.read_text(encoding="utf-8")
old_solar = '''    solar = {season: window * envelope["g_value"] * _irradiance(season, phi) * (1 - climate[f"direct_share_{season}"] * shading[season]) for season in shading}
'''
assert old_solar in s
s = s.replace(old_solar, "")
old_angle = '''    angle = daylight["sky_angle"] * (1 - daylight["sky_angle_loss"] * overhang / (overhang + daylight["overhang_half_depth"]))
'''
assert old_angle in s
s = s.replace(old_angle, '''    sky = 1 - daylight["sky_angle_loss"] * overhang / (overhang + daylight["overhang_half_depth"])
    angle = daylight["sky_angle"] * sky
    direct = {season: climate[f"direct_share_{season}"] for season in shading}
    solar = {season: window * envelope["g_value"] * _irradiance(season, phi) * (direct[season] * (1 - shading[season]) + (1 - direct[season]) * sky) for season in shading}
''')
# losses must come after solar; move ordering check
p.write_text(s, encoding="utf-8", newline="\n")
