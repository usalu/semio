from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py")
s = p.read_text(encoding="utf-8")
old = '''def sun_state(orientation, season):
    """🌞 The sun-path point that meets the façade best: bearing, altitude, angle across the façade normal and profile angle in section."""
    bearing, altitude = min(_sun_track(season), key=lambda point: abs((point[0] - orientation + 180) % 360 - 180))
    across = abs((bearing - orientation + 180) % 360 - 180)
    front = across < 89
    profile = min(85.0, math.degrees(math.atan(math.tan(math.radians(altitude)) / max(math.cos(math.radians(across)), 1e-3)))) if front else altitude
    return {"bearing": bearing, "altitude": altitude, "profile": profile, "front": front, "across": across}
'''
assert old in s
s = s.replace(old, '''@functools.lru_cache(maxsize=None)
def _sun_point(orientation, season):
    """📌 Cached best-matching sun-path point of one façade bearing."""
    bearing, altitude = min(_sun_track(season), key=lambda point: abs((point[0] - orientation + 180) % 360 - 180))
    across = abs((bearing - orientation + 180) % 360 - 180)
    front = across < 89
    profile = min(85.0, math.degrees(math.atan(math.tan(math.radians(altitude)) / max(math.cos(math.radians(across)), 1e-3)))) if front else altitude
    return {"bearing": bearing, "altitude": altitude, "profile": profile, "front": front, "across": across}


def sun_state(orientation, season):
    """🌞 The sun-path point that meets the façade best: bearing, altitude, angle across the façade normal and profile angle in section."""
    return dict(_sun_point(float(orientation), season))
''')
s = s.replace("    state = sun_state(orientation, season)\n    if overhang <= 0", "    state = _sun_point(orientation, season)\n    if overhang <= 0")
p.write_text(s, encoding="utf-8", newline="\n")
