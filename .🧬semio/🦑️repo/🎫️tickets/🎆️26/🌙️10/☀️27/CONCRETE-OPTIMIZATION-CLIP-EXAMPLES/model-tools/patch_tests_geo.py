from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧪️tests/🐍️.py")
s = p.read_text(encoding="utf-8")
def sub(old, new):
    global s
    assert s.count(old) == 1, old
    s = s.replace(old, new)
sub('    room, climate, envelope, shading, daylight = (p[name] for name in ("room", "climate", "envelope", "shading", "daylight"))\n', '    room, climate, envelope, daylight = (p[name] for name in ("room", "climate", "envelope", "daylight"))\n')
a = s.index("    reach = shading["); b = s.index("    def utilization(ratio, exponent):")
b = s.rindex("\n\n", a, b)
s = s[:a] + '''    gap = room["height"] - room["head"]
    height = np.minimum(wwr * room["height"], room["head"] - 0.05)
    sun = study["sun"]

    def shade(season):
        delta = np.radians(sun["declination"] if season == "summer" else -sun["declination"])
        angle = np.radians(15.0 * np.arange(-sun["hours"], sun["hours"] + 1))
        latitude = np.radians(sun["latitude"])
        altitude = np.degrees(np.arcsin(np.sin(latitude) * np.sin(delta) + np.cos(latitude) * np.cos(delta) * np.cos(angle)))
        bearing = (180 + np.degrees(np.arctan2(np.sin(angle), np.cos(angle) * np.sin(latitude) - np.tan(delta) * np.cos(latitude)))) % 360
        across = np.abs((bearing[None, :] - theta[:, None] + 180) % 360 - 180)
        valid = (altitude[None, :] >= sun["minimum_altitude"]) & (across < 89)
        profile = np.minimum(85.0, np.degrees(np.arctan(np.tan(np.radians(altitude))[None, :] / np.cos(np.radians(across)))))
        share = np.clip((depth[:, None] * np.tan(np.radians(profile)) - gap) / height[:, None], 0, 1)
        weight = np.where(valid, np.cos(np.radians(altitude))[None, :] * np.cos(np.radians(across)), 0.0)
        total = weight.sum(axis=1)
        return np.where((depth > 0) & (total > 0), (weight * share).sum(axis=1) / np.where(total > 0, total, 1), 0.0)

    direct = lambda season: climate[f"direct_share_{season}"]
    solar = {season: window * envelope["g_value"] * irradiance(season) * (1 - direct(season) * shade(season)) for season in ("winter", "summer")}
''' + s[b:]
s = s.replace("    height = wwr * room[\"height\"]\n", "")
p.write_text(s, encoding="utf-8", newline="\n")
