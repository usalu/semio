from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py")
s = p.read_text(encoding="utf-8")
a = s.index("def _shading("); b = s.index("@functools.lru_cache(maxsize=None)\ndef _evaluate(")
s = s[:a] + '''@functools.lru_cache(maxsize=None)
def _sun_samples(orientation, season):
    """🕘 Hourly sun positions of the solstice that shine on the façade, with their share of the beam (cos of incidence)."""
    study, _ = _frozen()
    sun = study["sun"]
    phi, delta = math.radians(sun["latitude"]), math.radians(sun["declination"] if season == "summer" else -sun["declination"])
    samples = []
    for hour in range(-sun["hours"], sun["hours"] + 1):
        angle = math.radians(15 * hour)
        altitude = math.degrees(math.asin(math.sin(phi) * math.sin(delta) + math.cos(phi) * math.cos(delta) * math.cos(angle)))
        bearing = (180 + math.degrees(math.atan2(math.sin(angle), math.cos(angle) * math.sin(phi) - math.tan(delta) * math.cos(phi)))) % 360
        across = abs((bearing - orientation + 180) % 360 - 180)
        if altitude >= sun["minimum_altitude"] and across < 89:
            profile = min(85.0, math.degrees(math.atan(math.tan(math.radians(altitude)) / math.cos(math.radians(across)))))
            samples.append({"hour": hour, "bearing": bearing, "altitude": altitude, "across": across, "profile": profile,
                            "weight": math.cos(math.radians(altitude)) * math.cos(math.radians(across))})
    return tuple(samples)


def sun_samples(orientation, season):
    """🕘 Hourly solstice sun positions on the façade: bearing, altitude, angle across the façade normal, profile angle in section and beam weight."""
    return [dict(item) for item in _sun_samples(float(orientation), season)]


def _shading(season, orientation, overhang, window_height, gap):
    """⛱️ Beam-weighted share of the window height in the overhang's shadow: depth × tan(profile angle) minus the wall gap above the window."""
    samples = _sun_samples(orientation, season)
    if overhang <= 0 or not samples:
        return 0.0
    shaded = (min(1.0, max(0.0, (overhang * math.tan(math.radians(item["profile"])) - gap) / window_height)) for item in samples)
    return sum(item["weight"] * share for item, share in zip(samples, shaded, strict=True)) / sum(item["weight"] for item in samples)


''' + s[b:]
p.write_text(s, encoding="utf-8", newline="\n")
j = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🔣️.json")
t = j.read_text(encoding="utf-8")
t = t.replace('    "minimum_altitude": 3,\n', '    "minimum_altitude": 3,\n    "hours": 4,\n')
j.write_text(t, encoding="utf-8", newline="\n")
b = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/model-tools/build_schema.py")
u = b.read_text(encoding="utf-8")
u = u.replace('"minimum_altitude": number, "winter_altitude"', '"minimum_altitude": number, "hours": {"type": "integer", "minimum": 0}, "winter_altitude"')
b.write_text(u, encoding="utf-8", newline="\n")
