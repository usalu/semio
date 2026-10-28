from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py")
s = p.read_text(encoding="utf-8")
block = '''    sky = 1 - daylight["sky_angle_loss"] * overhang / (overhang + daylight["overhang_half_depth"])
    angle = daylight["sky_angle"] * sky
    direct = {season: climate[f"direct_share_{season}"] for season in shading}
    solar = {season: window * envelope["g_value"] * _irradiance(season, phi) * (direct[season] * (1 - shading[season]) + (1 - direct[season]) * sky) for season in shading}
'''
assert block in s
s = s.replace(block, "")
anchor = '''    losses = conductance * climate["heating_degree_hours"] / 1000
'''
s = s.replace(anchor, block + anchor)
p.write_text(s, encoding="utf-8", newline="\n")
